mod behaviour;
mod peer_state;
mod swarm;

use futures::StreamExt;
use libp2p::mdns;
use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, RwLock};

use crate::error::NetworkError;
use crate::identity::keypair;

use behaviour::AetherBehaviourEvent;
use peer_state::{PeerStateTracker, PeerStatus};

/// Network command channel message
pub enum NetworkCommand {
    Shutdown,
}

/// Peer status update event
#[derive(Clone, Serialize)]
struct PeerStatusUpdate {
    peer_id: String,
    status: PeerStatus,
    multiaddr: Option<String>,
}

/// Network service manages the libp2p swarm lifecycle
pub struct NetworkService {
    cmd_tx: Option<mpsc::UnboundedSender<NetworkCommand>>,
    peer_state: Arc<RwLock<PeerStateTracker>>,
}

impl NetworkService {
    /// Create a new network service (not started)
    pub fn new() -> Self {
        Self {
            cmd_tx: None,
            peer_state: Arc::new(RwLock::new(PeerStateTracker::new())),
        }
    }

    /// Start the network service
    ///
    /// Loads keypair from keychain, builds swarm, starts listening, and spawns event loop
    pub fn start(&mut self, app: AppHandle) -> Result<(), NetworkError> {
        if self.is_running() {
            return Err(NetworkError::AlreadyRunning);
        }

        // Load libp2p keypair from keychain
        let keypair = keypair::load_libp2p_keypair()?;

        // Build swarm
        let mut swarm = swarm::build_swarm(keypair)?;

        // Start listening
        swarm::start_listening(&mut swarm)?;

        // Create command channel
        let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel();
        self.cmd_tx = Some(cmd_tx);

        // Clone peer state for event loop
        let peer_state = Arc::clone(&self.peer_state);

        // Spawn event loop in background
        tauri::async_runtime::spawn(async move {
            loop {
                tokio::select! {
                    // Handle swarm events
                    event = swarm.select_next_some() => {
                        // Update peer state
                        peer_state.write().await.handle_swarm_event(&event);

                        // Handle behaviour-specific events
                        if let libp2p::swarm::SwarmEvent::Behaviour(behaviour_event) = &event {
                            match behaviour_event {
                                AetherBehaviourEvent::Mdns(mdns::Event::Discovered(peers)) => {
                                    // CRITICAL: Wire mDNS discovered peers into Kademlia
                                    for (peer_id, multiaddr) in peers {
                                        swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr.clone());

                                        // Emit peer discovered event
                                        let _ = app.emit(
                                            "peer-status",
                                            PeerStatusUpdate {
                                                peer_id: peer_id.to_string(),
                                                status: PeerStatus::Connecting,
                                                multiaddr: Some(multiaddr.to_string()),
                                            },
                                        );
                                    }
                                }
                                AetherBehaviourEvent::Mdns(mdns::Event::Expired(peers)) => {
                                    for (peer_id, _multiaddr) in peers {
                                        // Log peer expiration
                                        println!("mDNS peer expired: {}", peer_id);
                                    }
                                }
                                _ => {}
                            }
                        }

                        // Emit status updates for connection events
                        match &event {
                            libp2p::swarm::SwarmEvent::ConnectionEstablished {
                                peer_id,
                                endpoint,
                                num_established,
                                ..
                            } => {
                                if num_established.get() == 1 {
                                    let _ = app.emit(
                                        "peer-status",
                                        PeerStatusUpdate {
                                            peer_id: peer_id.to_string(),
                                            status: PeerStatus::Online,
                                            multiaddr: Some(endpoint.get_remote_address().to_string()),
                                        },
                                    );
                                }
                            }
                            libp2p::swarm::SwarmEvent::ConnectionClosed {
                                peer_id,
                                num_established,
                                ..
                            } => {
                                if *num_established == 0 {
                                    let _ = app.emit(
                                        "peer-status",
                                        PeerStatusUpdate {
                                            peer_id: peer_id.to_string(),
                                            status: PeerStatus::Offline,
                                            multiaddr: None,
                                        },
                                    );
                                }
                            }
                            _ => {}
                        }
                    }

                    // Handle commands
                    cmd = cmd_rx.recv() => {
                        match cmd {
                            Some(NetworkCommand::Shutdown) | None => {
                                println!("Network service shutting down");
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// Stop the network service
    pub fn stop(&mut self) {
        if let Some(tx) = self.cmd_tx.take() {
            let _ = tx.send(NetworkCommand::Shutdown);
        }
    }

    /// Get all peers and their statuses
    pub fn get_peers(&self) -> Vec<(String, PeerStatus)> {
        // This is sync, so we use try_read (non-blocking)
        if let Ok(state) = self.peer_state.try_read() {
            state.get_peers()
        } else {
            Vec::new()
        }
    }

    /// Check if service is running
    pub fn is_running(&self) -> bool {
        self.cmd_tx.is_some()
    }
}
