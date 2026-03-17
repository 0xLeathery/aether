# T01: 09.1-fix-iscreator-integration-bug 01

**Slice:** S12 — **Milestone:** M001

## Description

Fix the orphaned `setLocalIdentity()` call so `isCreator` evaluates correctly, add CRDT sync-layer creator validation to prevent unauthorized channel mutations from peers, and auto-migrate channels with missing creator metadata on app startup.

Purpose: This closes the isCreator integration gap discovered in the v1.1 milestone audit. The [+] create channel button, rename, and delete menu items are permanently hidden because `localPublicKey` is never set. Additionally, hardening the sync layer and migrating legacy data ensures the creator permission model is robust end-to-end.

Output: Working channel create/rename/delete UI for swarm creators, post-sync unauthorized channel rejection, and silent auto-migration of legacy channel metadata.

## Must-Haves

- [ ] "Swarm creator sees [+] create channel button, rename menu item, and delete menu item"
- [ ] "Non-creator peers do NOT see creator-only UI elements (isCreator returns false)"
- [ ] "setLocalIdentity is called during app startup before swarm store initializes"
- [ ] "setLocalIdentity is called after new identity creation in setup flow path"
- [ ] "Post-sync validation removes channels created by non-creator peers"
- [ ] "Auto-migration fills missing created_by fields in existing channels on app startup"

## Files

- `src/App.svelte`
- `src-tauri/src/swarm/metadata_doc.rs`
- `src-tauri/src/swarm/metadata_sync.rs`
- `src-tauri/src/commands/channel.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
