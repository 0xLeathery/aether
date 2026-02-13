use libp2p::PeerId;
use serde::Serialize;
use std::collections::HashMap;
use std::num::NonZero;

use super::behaviour::AetherBehaviourEvent;

/// Peer connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PeerStatus {
    Connecting,
    Online,
    Offline,
}

/// Tracks peer connection states
pub struct PeerStateTracker {
    states: HashMap<PeerId, PeerStatus>,
}

impl PeerStateTracker {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    /// Handle swarm events and update peer states
    pub fn handle_swarm_event(&mut self, event: &libp2p::swarm::SwarmEvent<AetherBehaviourEvent>) {
        use libp2p::swarm::SwarmEvent;

        match event {
            SwarmEvent::ConnectionEstablished {
                peer_id,
                num_established,
                ..
            } => {
                // First connection to this peer
                if num_established.get() == 1 {
                    self.states.insert(*peer_id, PeerStatus::Online);
                }
            }
            SwarmEvent::ConnectionClosed {
                peer_id,
                num_established,
                ..
            } => {
                // Last connection to this peer closed
                if *num_established == 0 {
                    self.states.insert(*peer_id, PeerStatus::Offline);
                }
            }
            SwarmEvent::Dialing {
                peer_id: Some(peer_id),
                ..
            } => {
                self.states.insert(*peer_id, PeerStatus::Connecting);
            }
            SwarmEvent::OutgoingConnectionError {
                peer_id: Some(peer_id),
                ..
            } => {
                self.states.insert(*peer_id, PeerStatus::Offline);
            }
            _ => {}
        }
    }

    /// Get all peers and their statuses
    pub fn get_peers(&self) -> Vec<(String, PeerStatus)> {
        self.states
            .iter()
            .map(|(peer_id, status)| (peer_id.to_string(), *status))
            .collect()
    }

    /// Get status of a specific peer
    pub fn get_status(&self, peer_id: &PeerId) -> PeerStatus {
        self.states.get(peer_id).copied().unwrap_or(PeerStatus::Offline)
    }
}
