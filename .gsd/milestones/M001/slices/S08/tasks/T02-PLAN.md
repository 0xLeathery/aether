# T02: 07-channel-management 02

**Slice:** S08 — **Milestone:** M001

## Description

Create channel CRUD Tauri commands with creator permission checks and full deletion cleanup, plus TypeScript bindings and swarm store integration for the frontend data layer.

Purpose: Bridges the CRDT metadata infrastructure (Plan 01) to the frontend, enabling channel management operations. This is the command + data layer that the UI (Plan 03) consumes.

Output: New channel.rs commands module, updated create_swarm/join_swarm for default channels, TypeScript bindings in tauri.ts, and swarm store with activeChannelId and channel CRUD methods.

## Must-Haves

- [ ] "Swarm creator can create a new named channel via Tauri command and it persists in CRDT + local store"
- [ ] "Swarm creator can rename a channel via Tauri command and it updates CRDT + local store"
- [ ] "Swarm creator can delete a channel via Tauri command with full cleanup (CRDT + message doc + disk + memory)"
- [ ] "Non-creator peers receive permission denied error when attempting channel mutations"
- [ ] "Default channels (general, voice) cannot be deleted or renamed"
- [ ] "Channel name validation enforces lowercase alphanumeric + hyphens, 1-32 chars"
- [ ] "Frontend swarm store exposes channel CRUD methods and activeChannelId state"
- [ ] "create_swarm and join_swarm produce default general + voice channels via metadata document"

## Files

- `src-tauri/src/commands/channel.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/swarm.rs`
- `src-tauri/src/chat/mod.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/swarm.svelte.ts`
