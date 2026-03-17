# T01: 06-foundation 01

**Slice:** S07 — **Milestone:** M001

## Description

Add microphone mute/unmute functionality to voice sessions with visual feedback.

Purpose: Users need the ability to mute their microphone during voice chat -- this is the most fundamental voice control. Without mute, users must leave the session entirely to stop transmitting audio.

Output: Working mute toggle button in VoicePanel that controls an AtomicBool flag in VoiceSession, preventing audio encoding and transmission when muted.

## Must-Haves

- [ ] "User can mute/unmute their microphone during a voice session"
- [ ] "Mute state is visually reflected in the voice panel (MIC MUTED vs MIC LIVE)"
- [ ] "When muted, no audio is encoded or sent to peers"
- [ ] "Mute button only appears when user is in a voice session"

## Files

- `src-tauri/src/voice/session.rs`
- `src-tauri/src/commands/voice.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/voice.svelte.ts`
- `src/lib/components/voice/VoicePanel.svelte`
