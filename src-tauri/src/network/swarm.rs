use libp2p::{
    autonat, dcutr, identify, kad, mdns, noise, ping,
    swarm::Swarm,
    tcp, yamux, PeerId, SwarmBuilder,
};
use std::time::Duration;

use crate::error::NetworkError;

use super::behaviour::AetherBehaviour;

/// Build a libp2p swarm with Aether network behaviour
///
/// Creates a swarm with:
/// - QUIC transport (primary, better NAT traversal)
/// - TCP transport (fallback, with port reuse for hole-punching)
/// - 7 composed behaviours (kad, mdns, relay, dcutr, autonat, identify, ping)
/// - 60s idle connection timeout
pub fn build_swarm(keypair: libp2p::identity::Keypair) -> Result<Swarm<AetherBehaviour>, NetworkError> {
    let peer_id = PeerId::from_public_key(&keypair.public());

    let swarm = SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )
        .map_err(|e| NetworkError::TransportInit(format!("TCP transport failed: {}", e)))?
        .with_quic()
        .with_relay_client(noise::Config::new, yamux::Config::default)
        .map_err(|e| NetworkError::TransportInit(format!("Relay client init failed: {}", e)))?
        .with_behaviour(|key: &libp2p::identity::Keypair, relay_client| {
            // Create Kademlia DHT with in-memory store
            let kademlia = kad::Behaviour::new(
                peer_id,
                kad::store::MemoryStore::new(peer_id),
            );

            // Create mDNS for LAN discovery
            let mdns = mdns::tokio::Behaviour::new(
                mdns::Config::default(),
                peer_id,
            )
            .map_err(|e| format!("mDNS init failed: {}", e))?;

            // Create autonat for NAT detection
            let autonat = autonat::Behaviour::new(
                peer_id,
                Default::default(),
            );

            // Create identify for protocol identification
            let identify = identify::Behaviour::new(
                identify::Config::new("/aether/1.0.0".to_string(), key.public()),
            );

            // Create DCUTR for hole-punching
            let dcutr = dcutr::Behaviour::new(peer_id);

            // Create ping for keepalive
            let ping = ping::Behaviour::default();

            Ok(AetherBehaviour {
                kademlia,
                mdns,
                relay_client,
                dcutr,
                autonat,
                identify,
                ping,
            })
        })
        .map_err(|e| NetworkError::SwarmStart(format!("Behaviour creation failed: {}", e)))?
        .with_swarm_config(|cfg: libp2p::swarm::Config| {
            cfg.with_idle_connection_timeout(Duration::from_secs(60))
        })
        .build();

    Ok(swarm)
}

/// Start listening on QUIC and TCP addresses
///
/// Listens on:
/// - /ip4/0.0.0.0/udp/0/quic-v1 (QUIC on random port)
/// - /ip4/0.0.0.0/tcp/0 (TCP on random port)
pub fn start_listening(swarm: &mut Swarm<AetherBehaviour>) -> Result<(), NetworkError> {
    swarm
        .listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse().unwrap())
        .map_err(|e| NetworkError::ListenFailed(format!("QUIC listen failed: {}", e)))?;

    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .map_err(|e| NetworkError::ListenFailed(format!("TCP listen failed: {}", e)))?;

    Ok(())
}
