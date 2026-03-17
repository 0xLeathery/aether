# S05: Real Time Voice

**Goal:** Build the audio capture, codec, mixing, and playback pipeline for real-time voice.
**Demo:** Build the audio capture, codec, mixing, and playback pipeline for real-time voice.

## Must-Haves


## Tasks

- [x] **T01: 04-real-time-voice 01**
  - Build the audio capture, codec, mixing, and playback pipeline for real-time voice.

Purpose: Establish the core audio infrastructure that the voice streaming protocol (Plan 02) will use to send/receive audio between peers. This plan creates all audio processing components without network integration.

Output: A `voice/` Rust module with capture, codec, mixer, jitter buffer, and playback components that compile and are structurally ready for network integration.
- [x] **T02: 04-real-time-voice 02**
  - Wire the audio pipeline to the libp2p network for real-time P2P voice streaming.

Purpose: Connect the audio capture/encode pipeline (Plan 01) to the libp2p mesh network, enabling peers to send and receive voice data. This plan creates the voice streaming protocol, session manager, and integrates the stream behaviour into the existing network stack.

Output: A working VoiceSession that can join/leave voice, capture mic audio, encode it, stream to peers, receive peer audio, decode, mix, and play back -- all over the existing libp2p connections.
- [x] **T03: 04-real-time-voice 03** `est:52min`
  - Build Tauri voice commands and frontend UI for joining/leaving voice sessions.

Purpose: Connect the voice session manager (Plan 02) to the frontend via Tauri commands and create the voice session UI. Users can join voice in a channel, see participants, and leave. This completes the Phase 4 walking skeleton.

Output: Working voice UI with join/leave buttons, participant list, and mic activity indicator. Human verification confirms end-to-end voice between two instances.

## Files Likely Touched

- `src-tauri/Cargo.toml`
- `src-tauri/src/error.rs`
- `src-tauri/src/voice/mod.rs`
- `src-tauri/src/voice/capture.rs`
- `src-tauri/src/voice/playback.rs`
- `src-tauri/src/voice/codec.rs`
- `src-tauri/src/voice/mixer.rs`
- `src-tauri/src/voice/jitter_buffer.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/tauri.conf.json`
- `src-tauri/src/voice/protocol.rs`
- `src-tauri/src/voice/session.rs`
- `src-tauri/src/voice/mod.rs`
- `src-tauri/src/network/behaviour.rs`
- `src-tauri/src/network/mod.rs`
- `src-tauri/src/commands/voice.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/voice.svelte.ts`
- `src/lib/components/voice/VoicePanel.svelte`
- `src/lib/components/layout/MainContent.svelte`
