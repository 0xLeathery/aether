use libp2p::{
    autonat, dcutr, identify, kad, mdns, ping, relay,
    swarm::NetworkBehaviour,
};

/// Aether network behaviour composition
///
/// Combines 7 libp2p protocols:
/// - kad: Kademlia DHT for peer routing
/// - mdns: LAN peer discovery
/// - relay_client: Circuit relay v2 for NAT traversal
/// - dcutr: Direct Connection Upgrade through Relay (hole-punching)
/// - autonat: Automatic NAT detection
/// - identify: Protocol identification and peer info exchange
/// - ping: Connection keepalive
#[derive(NetworkBehaviour)]
pub struct AetherBehaviour {
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub mdns: mdns::tokio::Behaviour,
    pub relay_client: relay::client::Behaviour,
    pub dcutr: dcutr::Behaviour,
    pub autonat: autonat::Behaviour,
    pub identify: identify::Behaviour,
    pub ping: ping::Behaviour,
}
