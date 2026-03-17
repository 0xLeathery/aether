# S02: Sovereign Network

**Goal:** Build the libp2p network core: behaviour composition, swarm initialization, transport configuration, and background event loop.
**Demo:** Build the libp2p network core: behaviour composition, swarm initialization, transport configuration, and background event loop.

## Must-Haves


## Tasks

- [x] **T01: 02-sovereign-network 01**
  - Build the libp2p network core: behaviour composition, swarm initialization, transport configuration, and background event loop. This establishes the P2P networking foundation that all subsequent plans depend on.

Purpose: Validate that libp2p integrates cleanly with Tauri's async runtime and that the existing Ed25519 identity maps to a stable PeerId. This is the riskiest technical work in the phase — if libp2p + Tauri don't play well together, we need to know now.

Output: A `network` Rust module that creates and runs a libp2p swarm with mDNS discovery, NAT traversal protocols, and peer state tracking, all running in a background tokio task.
- [x] **T02: 02-sovereign-network 02**
  - Bridge the Rust network module to the Svelte frontend: create Tauri commands for network control, add event listeners for real-time peer status, and build UI components showing peer connection state. This completes the Phase 2 user-facing experience.

Purpose: Users need to SEE that networking works. Without a frontend bridge, the libp2p swarm runs invisibly. This plan makes peer discovery and connection status observable, which is essential for validating the P2P stack works.

Output: Tauri network commands, a reactive Svelte network store listening for peer-status events, and a PeerList component in the sidebar showing discovered peers with online/offline/connecting indicators.

## Files Likely Touched

- `src-tauri/Cargo.toml`
- `src-tauri/src/error.rs`
- `src-tauri/src/identity/keypair.rs`
- `src-tauri/src/network/mod.rs`
- `src-tauri/src/network/behaviour.rs`
- `src-tauri/src/network/swarm.rs`
- `src-tauri/src/network/peer_state.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/network.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/network.svelte.ts`
- `src/lib/components/layout/Sidebar.svelte`
- `src/lib/components/peers/PeerList.svelte`
- `src/App.svelte`
