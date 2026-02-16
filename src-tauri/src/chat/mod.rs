pub mod document;
pub mod message;
pub mod protocol;
pub mod storage;
pub mod sync;

pub use document::ChatDocument;
pub use message::ChatMessage;

use std::collections::HashMap;
use std::sync::Arc;

use futures::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use libp2p::PeerId;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::error::ChatError;
use protocol::CHAT_PROTOCOL;
use sync::PeerSyncStates;

/// Event payload emitted when chat messages are updated via sync
#[derive(Clone, Serialize)]
struct ChatMessagesUpdated {
    swarm_id: String,
    channel_id: String,
}

/// Chat service manages per-channel Automerge documents and sync orchestration
///
/// Provides a high-level API for sending messages, retrieving history,
/// and syncing with peers. Each channel has its own CRDT document
/// that is persisted to disk and synced over libp2p-stream.
pub struct ChatService {
    /// In-memory cache of channel documents: "swarm_id/channel_id" -> ChatDocument
    documents: HashMap<String, ChatDocument>,
    /// Per-peer sync states for efficient delta sync
    sync_states: PeerSyncStates,
    /// Hex-encoded Ed25519 public key for Automerge ActorId
    actor_hex: Option<String>,
}

impl ChatService {
    /// Create a new empty chat service
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            sync_states: PeerSyncStates::new(),
            actor_hex: None,
        }
    }

    /// Set the actor identity for new documents
    pub fn set_actor(&mut self, pub_key_hex: &str) {
        self.actor_hex = Some(pub_key_hex.to_string());
    }

    /// Get or load a channel document, creating a new one if no file exists
    pub fn get_or_load_doc(
        &mut self,
        app: &AppHandle,
        swarm_id: &str,
        channel_id: &str,
    ) -> Result<&mut ChatDocument, ChatError> {
        let key = format!("{}/{}", swarm_id, channel_id);

        if !self.documents.contains_key(&key) {
            // Try to load from disk
            let doc = match storage::load_doc(app, swarm_id, channel_id)? {
                Some(doc) => doc,
                None => {
                    // Create new document
                    let mut doc = ChatDocument::new()?;
                    if let Some(ref actor_hex) = self.actor_hex {
                        doc.set_actor(actor_hex)?;
                    }
                    doc
                }
            };
            self.documents.insert(key.clone(), doc);
        }

        Ok(self.documents.get_mut(&key).unwrap())
    }

    /// Send a message to a channel
    ///
    /// Validates content, creates a ChatMessage, adds it to the document,
    /// and persists the document to disk.
    pub fn send_message(
        &mut self,
        app: &AppHandle,
        swarm_id: &str,
        channel_id: &str,
        sender_key: &str,
        sender_name: &str,
        content: &str,
    ) -> Result<ChatMessage, ChatError> {
        // Validate content is not empty
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Err(ChatError::MessageError(
                "Message content cannot be empty".to_string(),
            ));
        }

        let msg = ChatMessage::new(
            sender_key.to_string(),
            sender_name.to_string(),
            trimmed.to_string(),
        );

        let doc = self.get_or_load_doc(app, swarm_id, channel_id)?;
        let msg = doc.add_message(msg)?;

        // Persist to disk
        let doc = self.documents.get_mut(&format!("{}/{}", swarm_id, channel_id)).unwrap();
        storage::save_doc(app, swarm_id, channel_id, doc)?;

        Ok(msg)
    }

    /// Get all messages for a channel sorted by timestamp
    pub fn get_messages(
        &mut self,
        app: &AppHandle,
        swarm_id: &str,
        channel_id: &str,
    ) -> Result<Vec<ChatMessage>, ChatError> {
        let doc = self.get_or_load_doc(app, swarm_id, channel_id)?;
        doc.get_messages()
    }

    /// Sync a channel document with a peer over an existing stream
    ///
    /// Returns true if changes were received from the peer.
    pub async fn sync_channel_with_peer(
        &mut self,
        app: &AppHandle,
        swarm_id: &str,
        channel_id: &str,
        peer_id: PeerId,
        stream: &mut libp2p::Stream,
    ) -> Result<bool, ChatError> {
        // Get or load the document
        let _ = self.get_or_load_doc(app, swarm_id, channel_id)?;

        let channel_key = format!("{}/{}", swarm_id, channel_id);

        // Get sync state for this peer-channel pair
        let sync_state = self.sync_states.get_or_create(peer_id, channel_key.clone());

        // Get the document for sync
        let doc = self.documents.get_mut(&channel_key).unwrap();

        // Run the sync loop
        let received_changes = sync::sync_document(doc, sync_state, stream).await?;

        // Save to disk if changes were received
        if received_changes {
            let doc = self.documents.get_mut(&channel_key).unwrap();
            storage::save_doc(app, swarm_id, channel_id, doc)?;
        }

        Ok(received_changes)
    }

    /// Start listening for incoming chat sync streams
    ///
    /// Spawns a background tokio task that accepts incoming `/aether/chat/1.0.0`
    /// streams. For each stream, reads a channel identifier header, runs sync,
    /// and emits `chat-messages-updated` event on changes.
    pub fn start_sync_listener(
        app: AppHandle,
        chat_service: Arc<tokio::sync::Mutex<ChatService>>,
        mut control: libp2p_stream::Control,
    ) {
        tokio::spawn(async move {
            let mut incoming = match control.accept(CHAT_PROTOCOL) {
                Ok(incoming) => incoming,
                Err(e) => {
                    eprintln!("Failed to accept chat protocol streams: {}", e);
                    return;
                }
            };

            use futures::StreamExt;
            while let Some((peer_id, mut stream)) = incoming.next().await {
                let app = app.clone();
                let chat_service = Arc::clone(&chat_service);

                tokio::spawn(async move {
                    // Read channel identifier header: "swarm_id\nchannel_id\n"
                    // Use futures BufReader since libp2p::Stream implements futures::io::AsyncRead
                    let mut reader = BufReader::new(&mut stream);
                    let mut swarm_id = String::new();
                    let mut channel_id = String::new();

                    if reader.read_line(&mut swarm_id).await.is_err() {
                        eprintln!("Failed to read swarm_id from chat sync stream");
                        return;
                    }
                    if reader.read_line(&mut channel_id).await.is_err() {
                        eprintln!("Failed to read channel_id from chat sync stream");
                        return;
                    }

                    let swarm_id = swarm_id.trim().to_string();
                    let channel_id = channel_id.trim().to_string();

                    if swarm_id.is_empty() || channel_id.is_empty() {
                        eprintln!("Empty swarm_id or channel_id in chat sync header");
                        return;
                    }

                    // Drop the BufReader to get the stream back
                    drop(reader);

                    // Run sync
                    let mut service = chat_service.lock().await;
                    match service
                        .sync_channel_with_peer(&app, &swarm_id, &channel_id, peer_id, &mut stream)
                        .await
                    {
                        Ok(true) => {
                            // Emit event to frontend
                            let _ = app.emit(
                                "chat-messages-updated",
                                ChatMessagesUpdated {
                                    swarm_id,
                                    channel_id,
                                },
                            );
                        }
                        Ok(false) => {
                            // No changes — nothing to notify
                        }
                        Err(e) => {
                            eprintln!("Chat sync error with peer {}: {}", peer_id, e);
                        }
                    }
                });
            }
        });
    }

    /// Trigger a sync to a specific peer for a channel
    ///
    /// Spawns a background task that opens an outgoing stream, writes
    /// the channel identifier header, and runs the sync loop.
    pub fn trigger_sync_to_peer(
        app: AppHandle,
        chat_service: Arc<tokio::sync::Mutex<ChatService>>,
        mut control: libp2p_stream::Control,
        peer_id: PeerId,
        swarm_id: String,
        channel_id: String,
    ) {
        tokio::spawn(async move {
            // Open outgoing stream to peer
            let mut stream = match control.open_stream(peer_id, CHAT_PROTOCOL).await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!(
                        "Failed to open chat sync stream to {}: {}",
                        peer_id, e
                    );
                    return;
                }
            };

            // Write channel identifier header
            let header = format!("{}\n{}\n", swarm_id, channel_id);
            if let Err(e) = stream.write_all(header.as_bytes()).await {
                eprintln!("Failed to write chat sync header: {}", e);
                return;
            }
            if let Err(e) = stream.flush().await {
                eprintln!("Failed to flush chat sync header: {}", e);
                return;
            }

            // Run sync
            let mut service = chat_service.lock().await;
            match service
                .sync_channel_with_peer(&app, &swarm_id, &channel_id, peer_id, &mut stream)
                .await
            {
                Ok(true) => {
                    let _ = app.emit(
                        "chat-messages-updated",
                        ChatMessagesUpdated {
                            swarm_id,
                            channel_id,
                        },
                    );
                }
                Ok(false) => {}
                Err(e) => {
                    eprintln!("Chat sync error with peer {}: {}", peer_id, e);
                }
            }
        });
    }
}
