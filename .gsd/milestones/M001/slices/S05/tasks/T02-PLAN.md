# T02: 04-real-time-voice 02

**Slice:** S05 — **Milestone:** M001

## Description

Wire the audio pipeline to the libp2p network for real-time P2P voice streaming.

Purpose: Connect the audio capture/encode pipeline (Plan 01) to the libp2p mesh network, enabling peers to send and receive voice data. This plan creates the voice streaming protocol, session manager, and integrates the stream behaviour into the existing network stack.

Output: A working VoiceSession that can join/leave voice, capture mic audio, encode it, stream to peers, receive peer audio, decode, mix, and play back -- all over the existing libp2p connections.

## Must-Haves

- [ ] "Encoded audio frames stream between peers via libp2p stream protocol"
- [ ] "Voice session tracks participants and enforces 8-peer limit"
- [ ] "Incoming audio is decoded, buffered, mixed, and output to speakers"
- [ ] "Join/leave events update session state without race condition crashes"
- [ ] "Packet loss triggers Opus PLC for smooth audio continuity"

## Files

- `src-tauri/src/voice/protocol.rs`
- `src-tauri/src/voice/session.rs`
- `src-tauri/src/voice/mod.rs`
- `src-tauri/src/network/behaviour.rs`
- `src-tauri/src/network/mod.rs`
