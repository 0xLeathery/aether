# T01: 04-real-time-voice 01

**Slice:** S05 — **Milestone:** M001

## Description

Build the audio capture, codec, mixing, and playback pipeline for real-time voice.

Purpose: Establish the core audio infrastructure that the voice streaming protocol (Plan 02) will use to send/receive audio between peers. This plan creates all audio processing components without network integration.

Output: A `voice/` Rust module with capture, codec, mixer, jitter buffer, and playback components that compile and are structurally ready for network integration.

## Must-Haves

- [ ] "Microphone audio can be captured via cpal and encoded to Opus"
- [ ] "Opus-encoded audio can be decoded back to PCM samples"
- [ ] "Multiple decoded audio streams can be mixed into a single output"
- [ ] "Jitter buffer reorders out-of-sequence frames and handles timing"
- [ ] "Audio pipeline uses lock-free channels between capture thread and consumer"

## Files

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
