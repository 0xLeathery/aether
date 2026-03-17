# S08: Channel Management

**Goal:** Build the backend CRDT infrastructure for channel metadata: a SwarmMetadataDocument that stores channel list + creator key in an Automerge document, file-based persistence, and a dedicated metadata sync protocol for peer-to-peer propagation.
**Demo:** Build the backend CRDT infrastructure for channel metadata: a SwarmMetadataDocument that stores channel list + creator key in an Automerge document, file-based persistence, and a dedicated metadata sync protocol for peer-to-peer propagation.

## Must-Haves


## Tasks

- [x] **T01: 07-channel-management 01** `est:4min`
  - Build the backend CRDT infrastructure for channel metadata: a SwarmMetadataDocument that stores channel list + creator key in an Automerge document, file-based persistence, and a dedicated metadata sync protocol for peer-to-peer propagation.

Purpose: This is the data foundation for all channel operations. Without the CRDT metadata document and sync protocol, channels cannot be created, synced, or managed across peers.

Output: Three new Rust modules (metadata_doc, metadata_sync, metadata_storage) in the swarm crate, updated error types, and SwarmMetadata with creator_key field.
- [x] **T02: 07-channel-management 02** `est:4min`
  - Create channel CRUD Tauri commands with creator permission checks and full deletion cleanup, plus TypeScript bindings and swarm store integration for the frontend data layer.

Purpose: Bridges the CRDT metadata infrastructure (Plan 01) to the frontend, enabling channel management operations. This is the command + data layer that the UI (Plan 03) consumes.

Output: New channel.rs commands module, updated create_swarm/join_swarm for default channels, TypeScript bindings in tauri.ts, and swarm store with activeChannelId and channel CRUD methods.
- [x] **T03: 07-channel-management 03** `est:3min`
  - Build the channel management UI: interactive channel list with active state, create dialog, right-click context menu with rename/delete, type-to-confirm deletion, and wire MainContent to use activeChannelId from the swarm store.

Purpose: This is the user-facing layer that makes channel management tangible. Without the UI, the backend infrastructure from Plans 01-02 is invisible. The user specifically requested Discord-familiar interaction patterns.

Output: Rewritten ChannelList.svelte with full interactivity, three new channel/ components (CreateChannelDialog, DeleteChannelDialog, ChannelContextMenu), and updated MainContent.svelte.

## Files Likely Touched

- `src-tauri/src/swarm/metadata_doc.rs`
- `src-tauri/src/swarm/metadata_sync.rs`
- `src-tauri/src/swarm/metadata_storage.rs`
- `src-tauri/src/swarm/mod.rs`
- `src-tauri/src/swarm/storage.rs`
- `src-tauri/src/error.rs`
- `src-tauri/src/commands/channel.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/swarm.rs`
- `src-tauri/src/chat/mod.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/swarm.svelte.ts`
- `src/lib/components/layout/ChannelList.svelte`
- `src/lib/components/layout/MainContent.svelte`
- `src/lib/components/channel/CreateChannelDialog.svelte`
- `src/lib/components/channel/DeleteChannelDialog.svelte`
- `src/lib/components/channel/ChannelContextMenu.svelte`
