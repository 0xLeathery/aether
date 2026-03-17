# T03: 06-foundation 03

**Slice:** S07 — **Milestone:** M001

## Description

Add swarm management operations: rename locally, leave with full data cleanup, and copy invite link.

Purpose: Users need to manage their swarm list -- renaming for organization, leaving swarms they no longer want, and sharing invite links with others. The leave operation is the most complex because it must clean up across four data domains (voice, network, chat documents, metadata) without leaving orphaned state.

Output: Three new Tauri commands (rename_swarm, leave_swarm, get_invite_uri), ChatService cleanup methods, SwarmSettings UI component, and swarm store methods for frontend operations.

## Must-Haves

- [ ] "User can rename a swarm locally and see the new name reflected in the sidebar"
- [ ] "User can leave a swarm with full data cleanup (voice, network, chat documents, store entry)"
- [ ] "User can copy the invite link for a swarm to share with others"
- [ ] "After leaving a swarm, all associated data is removed and the swarm disappears from the list"

## Files

- `src-tauri/src/swarm/storage.rs`
- `src-tauri/src/commands/swarm.rs`
- `src-tauri/src/chat/mod.rs`
- `src-tauri/src/chat/sync.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/swarm.svelte.ts`
- `src/lib/components/swarm/SwarmSettings.svelte`
- `src/lib/components/layout/Sidebar.svelte`
