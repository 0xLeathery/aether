# T01: 02-sovereign-network 01

**Slice:** S02 — **Milestone:** M001

## Description

Build the libp2p network core: behaviour composition, swarm initialization, transport configuration, and background event loop. This establishes the P2P networking foundation that all subsequent plans depend on.

Purpose: Validate that libp2p integrates cleanly with Tauri's async runtime and that the existing Ed25519 identity maps to a stable PeerId. This is the riskiest technical work in the phase — if libp2p + Tauri don't play well together, we need to know now.

Output: A `network` Rust module that creates and runs a libp2p swarm with mDNS discovery, NAT traversal protocols, and peer state tracking, all running in a background tokio task.

## Must-Haves

- [ ] "libp2p swarm starts with QUIC + TCP transports listening on all interfaces"
- [ ] "PeerId is deterministically derived from existing Ed25519 keypair in keychain"
- [ ] "mDNS discovers peers on the same LAN and adds them to Kademlia routing table"
- [ ] "Swarm event loop runs in background tokio task without blocking Tauri main thread"
- [ ] "Peer connection state tracks connecting/online/offline transitions"

## Files

- `src-tauri/Cargo.toml`
- `src-tauri/src/error.rs`
- `src-tauri/src/identity/keypair.rs`
- `src-tauri/src/network/mod.rs`
- `src-tauri/src/network/behaviour.rs`
- `src-tauri/src/network/swarm.rs`
- `src-tauri/src/network/peer_state.rs`
- `src-tauri/src/lib.rs`
