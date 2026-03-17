# T01: 09-peer-moderation 01

**Slice:** S11 — **Milestone:** M001

## Description

Build the moderation data layer end-to-end: Rust backend types, persistent storage, Tauri IPC commands, TypeScript bindings, reactive Svelte 5 store, and voice mixer peer-mute integration.

Purpose: Establish the moderation infrastructure that Plan 02 (UI) and Plan 03 (unread suppression + management) build on top of.
Output: Working moderation store with CRUD operations, voice mute skip, and reactive frontend state.

## Must-Haves

- [ ] "Moderation state persists in moderation.json via tauri-plugin-store"
- [ ] "Three cumulative tiers (mute/hide/block) stored as single enum value per peer"
- [ ] "Per-swarm overrides can override global tier for a specific swarm"
- [ ] "Voice mute for peers is enforced at the AudioMixer level in Rust"
- [ ] "Frontend moderation store provides reactive isMuted/isHidden/isBlocked with swarm-aware tier lookup"

## Files

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
