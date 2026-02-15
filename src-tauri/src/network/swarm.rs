use libp2p::{
    autonat, dcutr, identify, kad, mdns, noise, ping,
    core::transport::upgrade::Version,
    pnet::PnetConfig,
    swarm::{Swarm, behaviour::toggle::Toggle},
    tcp, yamux, PeerId, SwarmBuilder, Transport,
};
use std::time::Duration;

use crate::error::NetworkError;

use super::behaviour::AetherBehaviour;

/// Build a libp2p swarm with Aether network behaviour
///
/// Creates a swarm with:
/// - TCP transport (always included)
/// - QUIC transport (only when PSK is None - QUIC's TLS conflicts with PSK encryption)
/// - Optional PSK for swarm isolation (applied via PnetConfig on TCP transport)
/// - Up to 7 composed behaviours (kad, mdns, relay [optional], dcutr, autonat, identify, ping)
/// - 60s idle connection timeout
///
/// PSK Implementation: Uses .with_other_transport() to manually build TCP transport with
/// conditional PSK wrapping (pattern from libp2p ipfs-private example). PSK swarms are
/// TCP-only because QUIC has built-in TLS that conflicts with XSalsa20 PSK encryption.
/// relay_client is disabled for PSK swarms via Toggle.
pub fn build_swarm(
    keypair: libp2p::identity::Keypair,
    psk: Option<libp2p::pnet::PreSharedKey>,
) -> Result<Swarm<AetherBehaviour>, NetworkError> {
    let peer_id = PeerId::from_public_key(&keypair.public());

    let swarm = if let Some(psk_key) = psk {
        // PSK swarm: TCP-only with PnetConfig encryption layer
        // QUIC is incompatible with PSK (QUIC has TLS, PSK adds XSalsa20 layer)
        SwarmBuilder::with_existing_identity(keypair)
            .with_tokio()
            .with_other_transport(|key| {
                let noise_config = noise::Config::new(key).unwrap();
                let yamux_config = yamux::Config::default();

                // Build base TCP transport
                let base_transport = tcp::tokio::Transport::new(tcp::Config::default());

                // Wrap with PSK encryption using PnetConfig
                let psk_transport = base_transport
                    .and_then(move |socket, _| PnetConfig::new(psk_key).handshake(socket));

                // Chain authentication and multiplexing
                psk_transport
                    .upgrade(Version::V1Lazy)
                    .authenticate(noise_config)
                    .multiplex(yamux_config)
            })
            .map_err(|e| NetworkError::TransportInit(format!("PSK TCP transport failed: {}", e)))?
            .with_behaviour(|key: &libp2p::identity::Keypair| {
                // Create Kademlia DHT with in-memory store
                let kademlia = kad::Behaviour::new(
                    peer_id,
                    kad::store::MemoryStore::new(peer_id),
                );

                // Create mDNS for LAN discovery (works with PSK - same L2 network)
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

                // No relay_client for PSK swarms (QUIC not available)
                Ok(AetherBehaviour {
                    kademlia,
                    mdns,
                    relay_client: Toggle::from(None),
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
            .build()
    } else {
        // Open swarm: TCP + QUIC + relay for maximum connectivity
        SwarmBuilder::with_existing_identity(keypair)
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
                    relay_client: Toggle::from(Some(relay_client)),
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
            .build()
    };

    Ok(swarm)
}

/// Start listening on network addresses
///
/// PSK swarms (TCP-only):
/// - /ip4/0.0.0.0/tcp/0 (TCP on random port)
///
/// Open swarms (TCP + QUIC):
/// - /ip4/0.0.0.0/udp/0/quic-v1 (QUIC on random port)
/// - /ip4/0.0.0.0/tcp/0 (TCP on random port)
pub fn start_listening(swarm: &mut Swarm<AetherBehaviour>, use_quic: bool) -> Result<(), NetworkError> {
    // QUIC only for open swarms (PSK swarms are TCP-only)
    if use_quic {
        swarm
            .listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse().unwrap())
            .map_err(|e| NetworkError::ListenFailed(format!("QUIC listen failed: {}", e)))?;
    }

    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .map_err(|e| NetworkError::ListenFailed(format!("TCP listen failed: {}", e)))?;

    Ok(())
}
