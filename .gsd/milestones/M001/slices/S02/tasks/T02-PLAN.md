# T02: 02-sovereign-network 02

**Slice:** S02 — **Milestone:** M001

## Description

Bridge the Rust network module to the Svelte frontend: create Tauri commands for network control, add event listeners for real-time peer status, and build UI components showing peer connection state. This completes the Phase 2 user-facing experience.

Purpose: Users need to SEE that networking works. Without a frontend bridge, the libp2p swarm runs invisibly. This plan makes peer discovery and connection status observable, which is essential for validating the P2P stack works.

Output: Tauri network commands, a reactive Svelte network store listening for peer-status events, and a PeerList component in the sidebar showing discovered peers with online/offline/connecting indicators.

## Must-Haves

- [ ] "User can see a list of discovered peers with their PeerId and connection status"
- [ ] "Peer status updates in real-time as peers connect and disconnect (no page refresh needed)"
- [ ] "User can see their own PeerId and listening addresses in the UI"
- [ ] "Network starts automatically after identity creation (not just on app launch)"
- [ ] "User can discover peers on the same LAN via mDNS without internet"

## Files

- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/network.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/network.svelte.ts`
- `src/lib/components/layout/Sidebar.svelte`
- `src/lib/components/peers/PeerList.svelte`
- `src/App.svelte`
