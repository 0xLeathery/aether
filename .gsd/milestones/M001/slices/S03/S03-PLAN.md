# S03: Invitation System

**Goal:** Build the Rust backend for swarm management: PSK key generation, aether:// URI encoding, persistent swarm storage via Tauri Store plugin, and Tauri IPC commands for create/join/list/switch operations.
**Demo:** Build the Rust backend for swarm management: PSK key generation, aether:// URI encoding, persistent swarm storage via Tauri Store plugin, and Tauri IPC commands for create/join/list/switch operations.

## Must-Haves


## Tasks

- [x] **T01: 03-invitation-system 01** `est:5min`
  - Build the Rust backend for swarm management: PSK key generation, aether:// URI encoding, persistent swarm storage via Tauri Store plugin, and Tauri IPC commands for create/join/list/switch operations.

Purpose: Provides the cryptographic foundation and data persistence layer that the frontend (Plan 03-02) will invoke to create and join swarms.
Output: Complete Rust swarm module with Tauri commands, ready for frontend integration.
- [x] **T02: 03-invitation-system 02** `est:7min`
  - Build the frontend for swarm management: TypeScript invoke wrappers, reactive Svelte 5 swarm store, Create/Join dialogs with clipboard support, swarm selector in sidebar, and channel list driven by active swarm selection.

Purpose: Completes the invitation system user experience - users can create swarms, share Secret Codes, join swarms, and navigate between them.
Output: Fully functional swarm UI integrated into the existing three-column layout.

## Files Likely Touched

- `src-tauri/Cargo.toml`
- `src-tauri/src/error.rs`
- `src-tauri/src/swarm/mod.rs`
- `src-tauri/src/swarm/key.rs`
- `src-tauri/src/swarm/storage.rs`
- `src-tauri/src/swarm/uri.rs`
- `src-tauri/src/commands/swarm.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/network/swarm.rs`
- `src-tauri/src/network/mod.rs`
- `src-tauri/src/lib.rs`
- `package.json`
- `src/lib/tauri.ts`
- `src/lib/stores/swarm.svelte.ts`
- `src/lib/components/swarm/SwarmSelector.svelte`
- `src/lib/components/swarm/InviteDialog.svelte`
- `src/lib/components/swarm/JoinDialog.svelte`
- `src/lib/components/layout/Sidebar.svelte`
- `src/lib/components/layout/ChannelList.svelte`
- `src/lib/components/layout/AppShell.svelte`
- `src/App.svelte`
