# Phase 2: Sovereign Network - Research

**Researched:** 2026-02-13
**Domain:** P2P networking with libp2p in Rust
**Confidence:** HIGH

## Summary

Phase 2 implements peer-to-peer networking using libp2p 0.56, the mature Rust implementation of the modular P2P networking framework. The architecture combines multiple libp2p protocols to achieve sovereign peer discovery and NAT traversal: mDNS for zero-config LAN discovery, Kademlia DHT for internet-wide peer routing, AutoNAT for reachability detection, Circuit Relay v2 for proxied connections, and DCUtR (Direct Connection Upgrade through Relay) for UDP hole-punching through NATs.

The critical path is NAT traversal. Approximately 90-95% of networks support UDP (QUIC transport), but symmetric NATs remain a fundamental limitation where hole-punching cannot succeed. For these cases, relay fallback is essential. The libp2p ecosystem provides proven solutions: QUIC should be preferred over TCP when possible (faster, higher holepunch success rate), and the dcutr protocol coordinates simultaneous dials to establish direct connections after initial relay.

**Primary recommendation:** Use libp2p 0.56 with QUIC primary transport + TCP fallback, compose NetworkBehaviour with kad + mdns + relay + dcutr + autonat + identify + ping, run swarm event loop in Tauri background task via tokio::spawn, derive libp2p PeerId from existing Ed25519 keypair bytes.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| libp2p | 0.56.0 | P2P networking framework | De facto standard for sovereign P2P in Rust, proven in IPFS/Filecoin production |
| tokio | 1.x (already in Cargo.toml) | Async runtime | Required by libp2p, already integrated with Tauri v2 |
| libp2p-identity | 0.43.1+ | Ed25519 keypair wrapper | Provides PeerId derivation from existing ed25519-dalek keys |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| libp2p-kad | (via libp2p) | Kademlia DHT | Always - required for internet-wide peer discovery |
| libp2p-mdns | (via libp2p) | Local network discovery | Always - enables LAN connectivity without internet |
| libp2p-relay | (via libp2p) | Circuit Relay v2 | Always - required for NAT traversal setup |
| libp2p-dcutr | (via libp2p) | Direct connection upgrade | Always - implements hole-punching coordination |
| libp2p-autonat | (via libp2p) | Reachability detection | Always - determines if node is behind NAT |
| libp2p-identify | (via libp2p) | Peer protocol exchange | Always - standard handshake for peer info |
| libp2p-ping | (via libp2p) | Connection keepalive | Always - maintains connection state |
| libp2p-quic | (via libp2p) | QUIC transport | Primary transport - faster, better NAT traversal |
| serde | (already in deps) | Event serialization | For sending connection events to frontend |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| libp2p | Hyperswarm (JavaScript) | Would require 50MB+ Node.js sidecar, IPC latency harmful to real-time voice (per prior decision) |
| QUIC primary | TCP primary | TCP slower, lower holepunch success rate, but UDP blocked in ~5-10% of corporate networks |
| Circuit Relay v2 | Public IPFS bootstraps | Relay v2 more efficient (bounded connections), but requires running own relay or finding community relays |

**Installation:**
```toml
# Add to src-tauri/Cargo.toml
[dependencies]
libp2p = { version = "0.56", features = ["kad", "mdns", "relay", "dcutr", "autonat", "identify", "ping", "quic", "tcp", "noise", "yamux"] }
libp2p-identity = "0.43"
```

## Architecture Patterns

### Recommended Project Structure
```
src-tauri/src/
├── network/
│   ├── mod.rs              # Public API, network state management
│   ├── swarm.rs            # Swarm setup, NetworkBehaviour composition
│   ├── events.rs           # Event loop, SwarmEvent → frontend bridge
│   ├── behaviour.rs        # Custom NetworkBehaviour derivation
│   ├── config.rs           # Transport, protocol configurations
│   └── peer_state.rs       # Peer connection tracking (online/offline/connecting)
├── commands/
│   └── network.rs          # Tauri commands for network control
└── identity/               # Existing from Phase 1
    └── keypair.rs          # Ed25519 key → libp2p PeerId conversion
```

### Pattern 1: Derive libp2p PeerId from Existing Ed25519 Keys
**What:** Convert Phase 1's ed25519-dalek keypair to libp2p identity
**When to use:** At network initialization, reuse sovereign identity for P2P
**Example:**
```rust
// Source: https://docs.rs/libp2p-identity/latest/libp2p_identity/ed25519/struct.Keypair.html
// In src-tauri/src/identity/keypair.rs

use libp2p_identity::ed25519;
use ed25519_dalek::SigningKey;

impl AetherKeypair {
    pub fn to_libp2p_keypair(&self) -> Result<ed25519::Keypair, Error> {
        // Extract 32-byte secret scalar from ed25519-dalek SigningKey
        let secret_bytes = self.signing_key.to_bytes();

        // libp2p ed25519::Keypair uses same RFC 8032 binary format
        // Try from secret bytes (libp2p will derive public key)
        ed25519::Keypair::try_from_bytes(&mut secret_bytes.to_vec())
            .map_err(|e| Error::Libp2pKeyConversion(e.to_string()))
    }
}
```

### Pattern 2: Compose NetworkBehaviour with All Required Protocols
**What:** Use `#[derive(NetworkBehaviour)]` to compose multiple protocols
**When to use:** Always - required for NAT traversal + discovery stack
**Example:**
```rust
// Source: https://context7.com/libp2p/rust-libp2p/llms.txt
use libp2p::{kad, mdns, relay, dcutr, autonat, identify, ping, swarm::NetworkBehaviour};

#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "AetherEvent")]
struct AetherBehaviour {
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
    mdns: mdns::tokio::Behaviour,
    relay_client: relay::client::Behaviour,
    dcutr: dcutr::Behaviour,
    autonat: autonat::Behaviour,
    identify: identify::Behaviour,
    ping: ping::Behaviour,
}

#[derive(Debug)]
enum AetherEvent {
    Kademlia(kad::Event),
    Mdns(mdns::Event),
    Relay(relay::client::Event),
    Dcutr(dcutr::Event),
    AutoNat(autonat::Event),
    Identify(identify::Event),
    Ping(ping::Event),
}

// Implement From<T> for AetherEvent for each protocol event type
impl From<kad::Event> for AetherEvent {
    fn from(event: kad::Event) -> Self { AetherEvent::Kademlia(event) }
}
// ... repeat for all protocols
```

### Pattern 3: Track Connection State via SwarmEvent
**What:** Monitor ConnectionEstablished/ConnectionClosed for peer status
**When to use:** Always - required for success criterion #4 (real-time status)
**Example:**
```rust
// Source: https://docs.rs/libp2p/latest/libp2p/swarm/enum.SwarmEvent.html
use libp2p::swarm::SwarmEvent;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PeerStatus {
    Connecting,
    Online,
    Offline,
}

pub struct PeerStateTracker {
    states: HashMap<PeerId, PeerStatus>,
}

impl PeerStateTracker {
    pub fn handle_swarm_event(&mut self, event: &SwarmEvent<AetherEvent>) {
        match event {
            SwarmEvent::ConnectionEstablished { peer_id, num_established, .. } => {
                if *num_established == 1 {
                    self.states.insert(*peer_id, PeerStatus::Online);
                    // Emit to frontend via Tauri event
                }
            }
            SwarmEvent::ConnectionClosed { peer_id, num_established, .. } => {
                if *num_established == 0 {
                    self.states.insert(*peer_id, PeerStatus::Offline);
                }
            }
            SwarmEvent::Dialing { peer_id: Some(peer_id), .. } => {
                self.states.insert(*peer_id, PeerStatus::Connecting);
            }
            SwarmEvent::OutgoingConnectionError { peer_id: Some(peer_id), .. } => {
                self.states.insert(*peer_id, PeerStatus::Offline);
            }
            _ => {}
        }
    }
}
```

### Pattern 4: Run Swarm Event Loop in Tauri Background Task
**What:** Use tauri::async_runtime::spawn for non-blocking network service
**When to use:** Always - prevents blocking main Tauri thread
**Example:**
```rust
// Source: https://docs.rs/tauri/latest/tauri/async_runtime/index.html
use tauri::{async_runtime, AppHandle};
use tokio::sync::mpsc;

pub fn start_network_service(app: AppHandle, swarm: Swarm<AetherBehaviour>) {
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Spawn swarm event loop
    async_runtime::spawn(async move {
        loop {
            tokio::select! {
                event = swarm.select_next_some() => {
                    handle_swarm_event(event, &app);
                }
                Some(cmd) = rx.recv() => {
                    handle_network_command(cmd, &mut swarm);
                }
            }
        }
    });

    // Store tx in Tauri state for commands to send to swarm
}
```

### Pattern 5: Configure QUIC with TCP Fallback
**What:** Dual transport for maximum reachability
**When to use:** Always - QUIC blocked in 5-10% of corporate networks
**Example:**
```rust
// Source: https://docs.rs/libp2p/latest/libp2p/
use libp2p::{SwarmBuilder, tcp, quic, noise, yamux};

let swarm = SwarmBuilder::with_existing_identity(keypair)
    .with_tokio()
    // QUIC primary (UDP-based, faster, better holepunch)
    .with_quic()
    // TCP fallback (for networks blocking UDP)
    .with_tcp(
        tcp::Config::default().port_reuse(true),  // Required for hole-punching
        noise::Config::new,
        yamux::Config::default,
    )?
    .with_relay_client(noise::Config::new, yamux::Config::default)?
    .with_behaviour(|key, relay_client| {
        // Behavior setup
    })?
    .with_swarm_config(|cfg| {
        cfg.with_idle_connection_timeout(Duration::from_secs(60))
    })
    .build();
```

### Pattern 6: Bootstrap Kademlia DHT and Connect mDNS Peers
**What:** Wire mDNS discovered peers into Kademlia routing table
**When to use:** Always - enables DHT queries after local discovery
**Example:**
```rust
// Source: https://context7.com/libp2p/rust-libp2p/llms.txt
match swarm.select_next_some().await {
    SwarmEvent::Behaviour(AetherEvent::Mdns(mdns::Event::Discovered(peers))) => {
        for (peer_id, multiaddr) in peers {
            // Add to Kademlia routing table for DHT queries
            swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr);
            println!("mDNS discovered: {}", peer_id);
        }
    }
    // ... other events
}
```

### Anti-Patterns to Avoid
- **Symmetric NAT assumption**: Never assume hole-punching will always work. ~10-20% of NATs are symmetric and require relay fallback.
- **Missing port reuse**: TCP transport MUST have `.port_reuse(true)` or hole-punching fails. QUIC enables this by default.
- **Blocking Tauri main thread**: Never run `swarm.select_next_some()` in Tauri command handlers. Always use `async_runtime::spawn`.
- **PeerId generation without identity**: Don't use `with_new_identity()`. Always derive PeerId from Phase 1's Ed25519 key for stable identity.
- **No relay fallback**: Don't rely on direct connections only. Many networks have restrictive NATs that require relay.
- **Assuming UDP is always available**: ~5-10% of corporate networks block UDP. Always provide TCP fallback.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| NAT type detection | Custom STUN/TURN client | libp2p-autonat | Handles all NAT types, integrates with libp2p's hole-punching |
| Hole-punching coordination | Custom simultaneous dial logic | libp2p-dcutr | Proven synchronization, RTT calculation, relay coordination |
| DHT implementation | Custom peer routing | libp2p-kad (Kademlia) | Battle-tested in IPFS, handles churn, XOR distance metric |
| LAN discovery | Custom UDP broadcast | libp2p-mdns | Standard mDNS protocol, zero-config, handles multiple interfaces |
| Circuit relay | Custom proxy protocol | libp2p-relay (v2) | Resource-bounded, prevents abuse, standardized |
| Connection pooling | Custom peer connection manager | libp2p Swarm | Handles concurrent dials, connection limits, keepalive |
| Multiaddr parsing | Custom address parser | libp2p::Multiaddr | Self-describing, future-proof, composable protocol stack |

**Key insight:** P2P networking has deceptive edge cases (symmetric NATs, connection races, DHT attacks, relay abuse). libp2p has been hardened in production (IPFS, Filecoin, Polkadot) for years. Use it as-is.

## Common Pitfalls

### Pitfall 1: Relay Dependency Without Fallback Plan
**What goes wrong:** Hole-punching fails on symmetric NATs, app becomes unusable
**Why it happens:** Assuming all NATs support hole-punching, not testing relay-only mode
**How to avoid:**
- Always configure Circuit Relay v2 client behavior
- Test with `LIBP2P_HOLE_PUNCHING=false` environment variable
- Show clear UI when operating in relay-only mode (higher latency)
**Warning signs:** Users report "can't connect" from corporate/university networks

### Pitfall 2: Not Setting `port_reuse` on TCP Transport
**What goes wrong:** TCP hole-punching never succeeds, only QUIC works
**Why it happens:** Default TCP config doesn't enable socket address reuse
**How to avoid:** Always use `tcp::Config::default().port_reuse(true)` in SwarmBuilder
**Warning signs:** QUIC connections work, TCP connections always time out behind NAT

### Pitfall 3: Blocking Tauri Thread with Swarm Event Loop
**What goes wrong:** UI freezes when handling network events, app becomes unresponsive
**Why it happens:** Running `loop { swarm.select_next_some().await }` in command handler
**How to avoid:** Always spawn network service with `tauri::async_runtime::spawn` at app startup
**Warning signs:** Frontend commands time out, UI hangs during peer discovery

### Pitfall 4: Using `with_new_identity()` Instead of Deriving from Ed25519
**What goes wrong:** New PeerId every launch, peers can't find each other across restarts
**Why it happens:** Convenience of `with_new_identity()` vs manual keypair conversion
**How to avoid:** Always derive libp2p identity from Phase 1's persistent Ed25519 key
**Warning signs:** Peers show as different identities after app restart

### Pitfall 5: Not Wiring mDNS Peers into Kademlia
**What goes wrong:** LAN peers discovered but DHT queries don't route to them
**Why it happens:** Separate protocols, no automatic integration
**How to avoid:** In `mdns::Event::Discovered` handler, call `kademlia.add_address(peer_id, addr)`
**Warning signs:** `mdns` shows peers, `kad::QueryResult` returns empty

### Pitfall 6: Forgetting AutoNAT for Reachability Detection
**What goes wrong:** Node doesn't know it's behind NAT, never attempts hole-punching
**Why it happens:** AutoNAT is optional, not included in minimal examples
**How to avoid:** Always include `autonat::Behaviour` in NetworkBehaviour composition
**Warning signs:** Debug logs show no hole-punch attempts, always using relay

### Pitfall 7: No Connection State Tracking
**What goes wrong:** UI shows stale "online" status after peer disconnects
**Why it happens:** Not monitoring `ConnectionClosed` events with `num_established == 0`
**How to avoid:** Track per-peer connection count, only mark offline when last connection closes
**Warning signs:** Peers show online but communication fails

## Code Examples

Verified patterns from official sources:

### Swarm Initialization with Existing Identity
```rust
// Source: https://docs.rs/libp2p/latest/libp2p/
use libp2p::{SwarmBuilder, identity::Keypair};

// Derive from Phase 1 Ed25519 key
let libp2p_keypair = aether_identity.to_libp2p_keypair()?;
let keypair = Keypair::from(libp2p_keypair);

let swarm = SwarmBuilder::with_existing_identity(keypair)
    .with_tokio()
    .with_quic()
    .with_tcp(
        tcp::Config::default().port_reuse(true),
        noise::Config::new,
        yamux::Config::default,
    )?
    .with_relay_client(noise::Config::new, yamux::Config::default)?
    .with_behaviour(|key, relay_client| {
        let peer_id = key.public().to_peer_id();

        Ok(AetherBehaviour {
            kademlia: kad::Behaviour::new(peer_id, kad::store::MemoryStore::new(peer_id)),
            mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id)?,
            relay_client,
            dcutr: dcutr::Behaviour::new(peer_id),
            autonat: autonat::Behaviour::new(peer_id, Default::default()),
            identify: identify::Behaviour::new(identify::Config::new(
                "/aether/1.0.0".to_string(),
                key.public(),
            )),
            ping: ping::Behaviour::default(),
        })
    })?
    .build();

// Listen on all interfaces (OS assigns port)
swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
```

### Private Network with Swarm Key (Future Phase 3)
```rust
// Source: https://github.com/libp2p/specs/blob/master/pnet/Private-Networks-PSK-V1.md
// Note: Not implemented in Phase 2, shown for completeness

// swarm.key format:
// /key/swarm/psk/1.0.0/
// /base16/
// <64 hex chars = 32 bytes>

use libp2p_pnet::PreSharedKey;

let swarm_key = PreSharedKey::from_bytes(&swarm_key_bytes)?;

// Apply to transport layer before building swarm
// This isolates the network - only peers with same key can connect
```

### Connection Event → Tauri Frontend Bridge
```rust
// Source: Integration of https://docs.rs/libp2p/latest/libp2p/swarm/enum.SwarmEvent.html
// and https://v2.tauri.app/develop/calling-rust/

use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Serialize, Clone)]
struct PeerStatusUpdate {
    peer_id: String,
    status: PeerStatus,
    multiaddr: Option<String>,
}

fn handle_swarm_event(event: SwarmEvent<AetherEvent>, app: &AppHandle) {
    match event {
        SwarmEvent::ConnectionEstablished { peer_id, endpoint, num_established, .. } => {
            if num_established == 1 {
                let _ = app.emit("peer-status", PeerStatusUpdate {
                    peer_id: peer_id.to_string(),
                    status: PeerStatus::Online,
                    multiaddr: Some(endpoint.get_remote_address().to_string()),
                });
            }
        }
        SwarmEvent::ConnectionClosed { peer_id, num_established, cause, .. } => {
            if num_established == 0 {
                let _ = app.emit("peer-status", PeerStatusUpdate {
                    peer_id: peer_id.to_string(),
                    status: PeerStatus::Offline,
                    multiaddr: None,
                });
                if let Some(err) = cause {
                    eprintln!("Connection closed with error: {}", err);
                }
            }
        }
        SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
            if let Some(peer_id) = peer_id {
                let _ = app.emit("peer-status", PeerStatusUpdate {
                    peer_id: peer_id.to_string(),
                    status: PeerStatus::Offline,
                    multiaddr: None,
                });
                eprintln!("Connection error: {}", error);
            }
        }
        _ => {}
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Circuit Relay v1 | Circuit Relay v2 | libp2p 0.42+ | v2 enforces resource limits, prevents relay abuse, more efficient |
| SECIO security | Noise protocol | libp2p 0.30+ | Noise is faster, formally verified, TLS 1.3 compatible |
| TCP-only transport | QUIC primary + TCP fallback | libp2p 0.50+ | QUIC better for NAT traversal, multiplexing, lower latency |
| Global DHT | Per-protocol Kademlia | libp2p 0.45+ | Isolate routing per network, prevent cross-contamination |
| `Swarm::new` | `SwarmBuilder` | libp2p 0.53+ | Type-safe, compile-time protocol composition checks |
| Manual hole-punch | dcutr protocol | libp2p 0.49+ | Standardized, automated, better success rates |

**Deprecated/outdated:**
- **SECIO**: Removed in libp2p 0.47. Use Noise or TLS.
- **Circuit Relay v1**: Deprecated in libp2p 0.42. Use v2 for production.
- **Floodsub**: Use GossipSub for pub/sub (better DoS resistance, message signing).
- **`Swarm::new()`**: Use `SwarmBuilder` for type-safe construction (0.53+).

## Open Questions

1. **Relay Node Strategy**
   - What we know: libp2p-relay server exists, can self-host or use community relays
   - What's unclear: Should Phase 2 include relay server setup, or defer to Phase 3? How to discover relay nodes?
   - Recommendation: Phase 2 should support connecting TO relay nodes (client-only). Phase 3 can add relay server capability if needed. For testing, run local relay server. For production, investigate public IPFS relay nodes or deploy dedicated relay.

2. **Swarm Key Distribution**
   - What we know: Phase 3's "Secret Code" should derive from swarm key (per requirements)
   - What's unclear: Does Phase 2 implement swarm key (PSK) support now, or just plain public network?
   - Recommendation: Phase 2 should focus on PUBLIC network connectivity to validate NAT traversal. Phase 3 adds swarm key isolation when implementing invitation system. This keeps Phase 2 testable without invitation flow.

3. **Bootstrap Node Strategy**
   - What we know: Kademlia DHT needs bootstrap nodes to join network
   - What's unclear: Phase 2 has no invitation system yet, so what DHT are we joining?
   - Recommendation: Phase 2 should implement mDNS (LAN) and relay (NAT) but defer DHT bootstrap until Phase 3 when swarm keys provide network isolation. Otherwise, Phase 2 would join global public DHT (not desirable for sovereign network).

4. **Connection Limits**
   - What we know: libp2p Swarm supports connection limits via config
   - What's unclear: What's reasonable max peer count for desktop app?
   - Recommendation: Start with conservative limits (max 50 connections, 10 per peer) in Phase 2. Phase 4 (voice) will reveal bandwidth constraints for final tuning.

## Sources

### Primary (HIGH confidence)
- [libp2p 0.56.0 on docs.rs](https://docs.rs/libp2p/latest/libp2p/) - Current version, module overview
- [Context7 /libp2p/rust-libp2p](https://context7.com/libp2p/rust-libp2p/llms.txt) - Code examples for kad, mdns, relay, behaviour composition
- [libp2p Private Networks PSK Spec](https://github.com/libp2p/specs/blob/master/pnet/Private-Networks-PSK-V1.md) - Swarm key format, XSalsa20 encryption
- [libp2p SwarmEvent enum](https://docs.rs/libp2p/latest/libp2p/swarm/enum.SwarmEvent.html) - Connection lifecycle events
- [libp2p-identity ed25519 Keypair](https://docs.rs/libp2p-identity/latest/libp2p_identity/ed25519/struct.Keypair.html) - Key import/export, binary format
- [Tauri async_runtime docs](https://docs.rs/tauri/latest/tauri/async_runtime/index.html) - tokio::spawn integration

### Secondary (MEDIUM confidence)
- [Hole Punching in libp2p (IPFS Blog, 2022-01-20)](https://blog.ipfs.tech/2022-01-20-libp2p-hole-punching/) - DCUtR mechanism, AutoNAT, relay phases
- [libp2p Addressing Concepts](https://docs.libp2p.io/concepts/fundamentals/addressing/) - Multiaddr format, /p2p protocol
- [libp2p Hole Punching Tutorial](https://docs.rs/libp2p/latest/libp2p/tutorials/hole_punching/index.html) - relay-server-example, dcutr-example
- [libp2p Connectivity Tester](https://connectivity.libp2p.io/) - Real-world NAT traversal stats
- [GitHub rust-libp2p/pull/3964](https://github.com/libp2p/rust-libp2p/pull/3964) - QUIC hole-punching implementation (merged)
- [libp2p NAT Traversal Discussion](https://discuss.libp2p.io/t/nat-traversal-rust-libp2p/1316) - Community best practices
- [Tauri + Async Rust Process](https://rfdonnelly.github.io/posts/tauri-async-rust-process/) - Background task patterns

### Tertiary (LOW confidence)
- WebSearch: "rust-libp2p common pitfalls" - General discussions, needs validation in practice
- WebSearch: "QUIC UDP blocked corporate networks" - 5-10% blocking claim from discussions, not official

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - libp2p 0.56 is current stable, Context7 + official docs confirm API patterns
- Architecture: HIGH - NetworkBehaviour derivation, SwarmBuilder patterns verified in docs/examples
- Pitfalls: MEDIUM - Some from official issues/discussions, others from general P2P knowledge (need validation in implementation)

**Research date:** 2026-02-13
**Valid until:** ~60 days (libp2p stable, slow-moving API changes)