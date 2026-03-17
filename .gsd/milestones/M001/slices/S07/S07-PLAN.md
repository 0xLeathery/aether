# S07: Foundation

**Goal:** Add microphone mute/unmute functionality to voice sessions with visual feedback.
**Demo:** Add microphone mute/unmute functionality to voice sessions with visual feedback.

## Must-Haves


## Tasks

- [x] **T01: 06-foundation 01** `est:4min`
  - Add microphone mute/unmute functionality to voice sessions with visual feedback.

Purpose: Users need the ability to mute their microphone during voice chat -- this is the most fundamental voice control. Without mute, users must leave the session entirely to stop transmitting audio.

Output: Working mute toggle button in VoicePanel that controls an AtomicBool flag in VoiceSession, preventing audio encoding and transmission when muted.
- [x] **T02: 06-foundation 02** `est:6min`
  - Add local petname/contacts system so users can assign friendly names to peers and manage a contacts list.

Purpose: In a decentralized system where peers self-assert their display names, users need the ability to assign their own local labels (petnames) that override the self-asserted name everywhere in the UI. This follows the Spritely Institute petname model and is the foundation for identity management in Aether.

Output: New contacts module with storage, Tauri commands, reactive frontend store with resolveName helper, ContactsList view, ContactEditor popup, and petname resolution integrated into MessageList, PeerList, and VoicePanel.
- [x] **T03: 06-foundation 03** `est:4min`
  - Add swarm management operations: rename locally, leave with full data cleanup, and copy invite link.

Purpose: Users need to manage their swarm list -- renaming for organization, leaving swarms they no longer want, and sharing invite links with others. The leave operation is the most complex because it must clean up across four data domains (voice, network, chat documents, metadata) without leaving orphaned state.

Output: Three new Tauri commands (rename_swarm, leave_swarm, get_invite_uri), ChatService cleanup methods, SwarmSettings UI component, and swarm store methods for frontend operations.

## Files Likely Touched

- `src-tauri/src/voice/session.rs`
- `src-tauri/src/commands/voice.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/voice.svelte.ts`
- `src/lib/components/voice/VoicePanel.svelte`
- `src-tauri/src/contacts/mod.rs`
- `src-tauri/src/contacts/storage.rs`
- `src-tauri/src/commands/contacts.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/error.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/contacts.svelte.ts`
- `src/lib/components/chat/MessageList.svelte`
- `src/lib/components/peers/PeerList.svelte`
- `src/lib/components/voice/VoicePanel.svelte`
- `src/lib/components/contacts/ContactEditor.svelte`
- `src/lib/components/contacts/ContactsList.svelte`
- `src/lib/components/layout/Sidebar.svelte`
- `src-tauri/src/swarm/storage.rs`
- `src-tauri/src/commands/swarm.rs`
- `src-tauri/src/chat/mod.rs`
- `src-tauri/src/chat/sync.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/swarm.svelte.ts`
- `src/lib/components/swarm/SwarmSettings.svelte`
- `src/lib/components/layout/Sidebar.svelte`
