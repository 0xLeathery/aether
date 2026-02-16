use std::collections::HashMap;

use automerge::sync::{Message as SyncMessage, State as SyncState, SyncDoc};
use libp2p::PeerId;

use crate::error::ChatError;
use super::document::ChatDocument;
use super::protocol::{recv_sync_msg, send_sync_msg};

/// Manages per-peer Automerge sync state
///
/// Each (peer_id, channel_key) pair maintains its own sync state,
/// allowing efficient delta sync without full document transfer.
pub struct PeerSyncStates {
    states: HashMap<(PeerId, String), SyncState>,
}

impl PeerSyncStates {
    /// Create a new empty sync state tracker
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    /// Remove all sync states for channels belonging to a swarm
    pub fn remove_swarm(&mut self, swarm_id: &str) {
        let prefix = format!("{}/", swarm_id);
        self.states
            .retain(|(_, channel_key), _| !channel_key.starts_with(&prefix));
    }

    /// Remove sync states for a specific channel across all peers
    pub fn remove_channel(&mut self, swarm_id: &str, channel_id: &str) {
        let channel_key = format!("{}/{}", swarm_id, channel_id);
        self.states
            .retain(|(_, key), _| *key != channel_key);
    }

    /// Get or create sync state for a peer-channel pair
    pub fn get_or_create(&mut self, peer_id: PeerId, channel_key: String) -> &mut SyncState {
        self.states
            .entry((peer_id, channel_key))
            .or_insert_with(SyncState::new)
    }
}

/// Run the Automerge sync loop with a peer over a libp2p stream
///
/// Returns true if any changes were received from the peer (indicating
/// the local document was updated and the UI should refresh).
///
/// The sync loop exchanges Automerge sync messages until both sides
/// have converged (no more messages to send or receive).
pub async fn sync_document(
    doc: &mut ChatDocument,
    sync_state: &mut SyncState,
    stream: &mut libp2p::Stream,
) -> Result<bool, ChatError> {
    let mut received_changes = false;

    loop {
        // Generate outgoing sync message
        let outgoing = doc.doc_mut().sync().generate_sync_message(sync_state);

        if let Some(msg) = outgoing {
            // Send our sync message to the peer
            send_sync_msg(stream, &msg.encode()).await?;
        }

        // Try to receive peer's sync message (2 second timeout)
        match recv_sync_msg(stream, 2000).await? {
            Some(data) => {
                // Decode and apply the peer's sync message
                let msg = SyncMessage::decode(&data).map_err(|e| {
                    ChatError::SyncFailed(format!("Failed to decode sync message: {}", e))
                })?;

                doc.doc_mut()
                    .sync()
                    .receive_sync_message(sync_state, msg)
                    .map_err(|e| {
                        ChatError::SyncFailed(format!("Failed to apply sync message: {}", e))
                    })?;

                received_changes = true;
            }
            None => {
                // No more messages from peer — check if we're done
                let next = doc.doc_mut().sync().generate_sync_message(sync_state);
                if next.is_none() {
                    // Both sides have nothing to send — converged
                    break;
                }
                // We still have something to send, continue loop
                if let Some(msg) = next {
                    send_sync_msg(stream, &msg.encode()).await?;
                }
                // Give peer another chance to respond
                match recv_sync_msg(stream, 2000).await? {
                    Some(data) => {
                        let msg = SyncMessage::decode(&data).map_err(|e| {
                            ChatError::SyncFailed(format!(
                                "Failed to decode sync message: {}",
                                e
                            ))
                        })?;
                        doc.doc_mut()
                            .sync()
                            .receive_sync_message(sync_state, msg)
                            .map_err(|e| {
                                ChatError::SyncFailed(format!(
                                    "Failed to apply sync message: {}",
                                    e
                                ))
                            })?;
                        received_changes = true;
                    }
                    None => break, // Peer timed out again — we're done
                }
            }
        }
    }

    Ok(received_changes)
}
