# S11: Peer Moderation

**Goal:** Build the moderation data layer end-to-end: Rust backend types, persistent storage, Tauri IPC commands, TypeScript bindings, reactive Svelte 5 store, and voice mixer peer-mute integration.
**Demo:** Build the moderation data layer end-to-end: Rust backend types, persistent storage, Tauri IPC commands, TypeScript bindings, reactive Svelte 5 store, and voice mixer peer-mute integration.

## Must-Haves


## Tasks

- [x] **T01: 09-peer-moderation 01** `est:5 min`
  - Build the moderation data layer end-to-end: Rust backend types, persistent storage, Tauri IPC commands, TypeScript bindings, reactive Svelte 5 store, and voice mixer peer-mute integration.

Purpose: Establish the moderation infrastructure that Plan 02 (UI) and Plan 03 (unread suppression + management) build on top of.
Output: Working moderation store with CRUD operations, voice mute skip, and reactive frontend state.
- [x] **T02: 09-peer-moderation 02** `est:2min`
  - Build all moderation UI components: peer context menu (right-click), block confirmation dialog, message view filtering (hide placeholders + block removal), moderation status icons in peer list, and context menu triggers on both peer list and message author names.

Purpose: Users can see and interact with moderation controls, and moderated content is filtered appropriately in the chat view.
Output: Complete moderation UI layer with context menus, dialogs, message filtering, and status indicators.
- [x] **T03: 09-peer-moderation 03** `est:4 min`
  - Integrate moderation with the unread/mention tracking system and build the moderation management UI accessible from the sidebar.

Purpose: Ensure moderated peers don't generate noise (unreads/mentions) and give users a central place to view and manage all their moderation actions.
Output: Moderation-filtered unread tracking + management panel in sidebar.
- [x] **T04: 09-peer-moderation 04** `est:2min`
  - Wire VoiceSession's mute_peer/unmute_peer methods to actually call AudioMixer's mute/unmute methods, closing the voice mute enforcement gap identified in 09-VERIFICATION.md.

Purpose: MOD-02 requires blocking to "refuse voice audio." The AudioMixer has working mute logic (drains buffer, discards audio) but VoiceSession never calls it. This plan connects the two.
Output: VoiceSession.mute_peer() that resolves hex keys to PeerIds and calls mixer.mute_peer() for active participants, plus join-time mute application for new participants.

## Files Likely Touched

- `src-tauri/src/moderation/mod.rs`
- `src-tauri/src/moderation/storage.rs`
- `src-tauri/src/commands/moderation.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/error.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/voice/mixer.rs`
- `src-tauri/src/voice/session.rs`
- `src-tauri/src/voice/mod.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/moderation.svelte.ts`
- `src/lib/components/moderation/PeerContextMenu.svelte`
- `src/lib/components/moderation/BlockConfirmDialog.svelte`
- `src/lib/components/peers/PeerList.svelte`
- `src/lib/components/chat/MessageList.svelte`
- `src/lib/components/layout/Sidebar.svelte`
- `src/lib/stores/unread.svelte.ts`
- `src/lib/components/moderation/ModerationList.svelte`
- `src/lib/components/layout/Sidebar.svelte`
- `src-tauri/src/voice/session.rs`
