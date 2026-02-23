use std::collections::HashMap;

use automerge::sync::{Message as SyncMessage, State as SyncState, SyncDoc};
use libp2p::PeerId;
use libp2p::StreamProtocol;

use crate::error::ChannelError;
use super::metadata_doc::SwarmMetadataDocument;
use crate::chat::protocol::{recv_sync_msg, send_sync_msg};

/// Swarm metadata sync protocol identifier
///
/// Uses Automerge sync messages over length-prefixed framing,
/// reusing the same wire format as the chat protocol but with
/// a distinct protocol identifier for stream multiplexing.
pub const SWARM_META_PROTOCOL: StreamProtocol =
    StreamProtocol::new("/aether/swarm-meta/1.0.0");

/// Manages per-peer Automerge sync state for swarm metadata
///
/// Each (peer_id, swarm_id) pair maintains its own sync state,
/// allowing efficient delta sync without full document transfer.
pub struct MetadataSyncStates {
    states: HashMap<(PeerId, String), SyncState>,
}

impl MetadataSyncStates {
    /// Create a new empty sync state tracker
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    /// Get or create sync state for a peer-swarm pair
    pub fn get_or_create(&mut self, peer_id: PeerId, swarm_id: String) -> &mut SyncState {
        self.states
            .entry((peer_id, swarm_id))
            .or_insert_with(SyncState::new)
    }

    /// Remove all sync states for a swarm
    pub fn remove_swarm(&mut self, swarm_id: &str) {
        self.states
            .retain(|(_, sid), _| sid != swarm_id);
    }
}

/// Run the Automerge sync loop for swarm metadata with a peer over a libp2p stream
///
/// Returns true if any changes were received from the peer (indicating
/// the local document was updated and the UI should refresh the channel list).
///
/// Reuses the same length-prefixed sync message framing from the chat protocol.
/// The sync loop exchanges Automerge sync messages until both sides
/// have converged (no more messages to send or receive).
pub async fn sync_metadata_document(
    doc: &mut SwarmMetadataDocument,
    sync_state: &mut SyncState,
    stream: &mut libp2p::Stream,
) -> Result<bool, ChannelError> {
    let mut received_changes = false;

    loop {
        // Generate outgoing sync message
        let outgoing = doc.doc_mut().sync().generate_sync_message(sync_state);

        if let Some(msg) = outgoing {
            // Send our sync message to the peer
            send_sync_msg(stream, &msg.encode())
                .await
                .map_err(|e| ChannelError::SyncError(e.to_string()))?;
        }

        // Try to receive peer's sync message (2 second timeout)
        match recv_sync_msg(stream, 2000)
            .await
            .map_err(|e| ChannelError::SyncError(e.to_string()))?
        {
            Some(data) => {
                // Decode and apply the peer's sync message
                let msg = SyncMessage::decode(&data).map_err(|e| {
                    ChannelError::SyncError(format!("Failed to decode sync message: {}", e))
                })?;

                doc.doc_mut()
                    .sync()
                    .receive_sync_message(sync_state, msg)
                    .map_err(|e| {
                        ChannelError::SyncError(format!("Failed to apply sync message: {}", e))
                    })?;

                received_changes = true;
            }
            None => {
                // No more messages from peer -- check if we're done
                let next = doc.doc_mut().sync().generate_sync_message(sync_state);
                if next.is_none() {
                    // Both sides have nothing to send -- converged
                    break;
                }
                // We still have something to send, continue loop
                if let Some(msg) = next {
                    send_sync_msg(stream, &msg.encode())
                        .await
                        .map_err(|e| ChannelError::SyncError(e.to_string()))?;
                }
                // Give peer another chance to respond
                match recv_sync_msg(stream, 2000)
                    .await
                    .map_err(|e| ChannelError::SyncError(e.to_string()))?
                {
                    Some(data) => {
                        let msg = SyncMessage::decode(&data).map_err(|e| {
                            ChannelError::SyncError(format!(
                                "Failed to decode sync message: {}",
                                e
                            ))
                        })?;
                        doc.doc_mut()
                            .sync()
                            .receive_sync_message(sync_state, msg)
                            .map_err(|e| {
                                ChannelError::SyncError(format!(
                                    "Failed to apply sync message: {}",
                                    e
                                ))
                            })?;
                        received_changes = true;
                    }
                    None => break, // Peer timed out again -- we're done
                }
            }
        }
    }

    // Post-sync validation: remove unauthorized channels from non-creator peers
    if received_changes {
        let _ = doc.validate_channels_creator();
    }

    Ok(received_changes)
}
