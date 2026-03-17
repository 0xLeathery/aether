# T02: 06-foundation 02

**Slice:** S07 — **Milestone:** M001

## Description

Add local petname/contacts system so users can assign friendly names to peers and manage a contacts list.

Purpose: In a decentralized system where peers self-assert their display names, users need the ability to assign their own local labels (petnames) that override the self-asserted name everywhere in the UI. This follows the Spritely Institute petname model and is the foundation for identity management in Aether.

Output: New contacts module with storage, Tauri commands, reactive frontend store with resolveName helper, ContactsList view, ContactEditor popup, and petname resolution integrated into MessageList, PeerList, and VoicePanel.

## Must-Haves

- [ ] "User can assign a local petname to any peer, and that petname appears instead of the self-asserted name everywhere in the UI"
- [ ] "User can view a contacts list showing all known peers with their petnames, public keys, and last-seen info"
- [ ] "Petnames are stored locally only (never synced via CRDT)"
- [ ] "Petname resolution chain: petname > sender_name > truncated public key"

## Files

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
