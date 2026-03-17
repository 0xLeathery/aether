# T03: 04-real-time-voice 03

**Slice:** S05 — **Milestone:** M001

## Description

Build Tauri voice commands and frontend UI for joining/leaving voice sessions.

Purpose: Connect the voice session manager (Plan 02) to the frontend via Tauri commands and create the voice session UI. Users can join voice in a channel, see participants, and leave. This completes the Phase 4 walking skeleton.

Output: Working voice UI with join/leave buttons, participant list, and mic activity indicator. Human verification confirms end-to-end voice between two instances.

## Must-Haves

- [ ] "User can click a button to join voice in the active channel"
- [ ] "User can see which peers are in the voice session"
- [ ] "User can click a button to leave the voice session"
- [ ] "Voice session shows participant count with 8-person limit visible"
- [ ] "User can see a visual indicator of their own microphone activity"

## Files

- `src-tauri/src/commands/voice.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/voice.svelte.ts`
- `src/lib/components/voice/VoicePanel.svelte`
- `src/lib/components/layout/MainContent.svelte`
