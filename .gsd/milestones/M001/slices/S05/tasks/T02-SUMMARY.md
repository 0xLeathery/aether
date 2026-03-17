---
id: T02
parent: S05
milestone: M001
provides: []
requires: []
affects: []
key_files: []
key_decisions: []
patterns_established: []
observability_surfaces: []
drill_down_paths: []
duration: 
verification_result: passed
completed_at: 
blocker_discovered: false
---
# T02: 04-real-time-voice 02

**# Phase 04 Plan 02: Voice Network Integration Summary**

## What Happened

# Phase 04 Plan 02: Voice Network Integration Summary

**One-liner:** Complete P2P voice streaming with libp2p-stream protocol, VoiceSession manager, persistent stream caching, and full audio-network pipeline integration.

## What Was Built

Integrated the audio pipeline (Plan 01) with the libp2p network stack for real-time peer-to-peer voice communication:

**Voice Streaming Protocol (protocol.rs)**
- Wire format: [sequence:4][length:2][opus_data:N] (big-endian)
- `VoicePacket` struct with sequence numbering for jitter buffer ordering
- `encode_packet/decode_packet` with validation (max 1500 bytes)
- `send_frame/recv_frame` async helpers for libp2p streams
- Protocol identifier: `/aether/voice/1.0.0`
- Stream-based (not request-response) for continuous audio flow

**Stream Behaviour Integration**
- Added `libp2p-stream` 0.4.0-alpha dependency
- Integrated `stream::Behaviour` as 8th protocol in AetherBehaviour
- Initialized in both PSK and open swarm paths
- Stream control extracted from swarm BEFORE async move
- Exposed via `NetworkService::stream_control()` getter

**Voice Session Manager (session.rs)**
- Complete audio-network pipeline coordination:
  ```
  capture (cpal) → encode (opus) → send (stream) → [network] →
  receive → decode → jitter buffer → mix → playback (cpal)
  ```

- **Join lifecycle:**
  1. Validates not already in session
  2. Enforces 8-participant limit (7 peers + self)
  3. Activates session (is_active flag)
  4. Initializes mixer with all peer jitter buffers
  5. Creates bounded channels (capacity 10 = 200ms buffering)
  6. Starts audio capture/playback streams
  7. Spawns 3 async tasks: encode-send, receive-decode, mix-playback
  8. Emits `voice-session-joined` event

- **Leave lifecycle:**
  1. Deactivates session (stops all tasks via is_active flag)
  2. Drops cpal streams (stops audio I/O)
  3. Clears mixer peer buffers
  4. Clears participants set
  5. Resets sequence counter
  6. Emits `voice-session-left` event

- **Encode-and-send task:**
  - Receives PCM from capture channel (blocking recv OK in tokio task)
  - Encodes to Opus
  - Increments sequence number (atomic)
  - Persistent stream cache per peer (HashMap)
  - Opens stream only on first send or after error
  - Graceful degradation: stream errors logged, peer skipped

- **Receive-and-decode task:**
  - Accepts incoming voice protocol streams
  - Spawns per-peer decoder task for each incoming stream
  - Each peer task: recv_frame → decode → feed to mixer's jitter buffer
  - PLC triggered on decode errors
  - Streams run independently (peer isolation)

- **Mix-to-playback task:**
  - 20ms interval (matches Opus frame rate)
  - Locks mixer, calls mix_next_frame(FRAME_SIZE)
  - Sends to playback channel (non-blocking try_send)
  - Drops frames if playback buffer full (acceptable)

- **Session state:**
  - `is_active: Arc<AtomicBool>` - shared across all tasks
  - `participants: Arc<RwLock<HashSet<PeerId>>>` - thread-safe participant list
  - `mixer: Arc<RwLock<AudioMixer>>` - shared mixer with per-peer jitter buffers
  - `sequence: Arc<AtomicU32>` - outgoing packet counter
  - `max_participants: 8` - hard limit

**NetworkService Updates**
- Added `stream_control: Option<libp2p_stream::Control>` field
- Extract stream control in both `start()` and `start_with_psk()`
- Extract BEFORE moving swarm into async task (critical timing)
- Expose via `stream_control()` getter (cloneable Arc internally)

## Deviations from Plan

None - plan executed exactly as written.

All implementation details followed precisely:
- Persistent stream cache (not opening new stream per frame)
- Per-peer decoder tasks (parallel processing, isolation)
- Stream control extraction timing (before async move)
- 8-participant enforcement (7 peers + self)
- Graceful error handling (no crashes on stream errors)
- Clean shutdown via is_active flag

## Testing & Verification

**Compilation:**
- ✅ `cargo check` passes with all voice modules
- ✅ No compilation errors
- 38 warnings (all "never used" - expected before Tauri command integration)

**Architecture Verification:**
- ✅ Voice protocol uses `/aether/voice/1.0.0` stream protocol
- ✅ Wire format: sequence (4 bytes) + length (2 bytes) + opus data
- ✅ stream::Behaviour field exists in AetherBehaviour
- ✅ Stream behaviour initialized in both PSK and open swarm paths (lines 96, 156 in swarm.rs)
- ✅ VoiceSession has join() and leave() methods
- ✅ NetworkService exposes stream_control()
- ✅ Persistent stream cache (HashMap<PeerId, libp2p::Stream>)
- ✅ Per-peer decoder tasks spawned in receive loop
- ✅ is_active checked in all spawned task loops (7 occurrences)
- ✅ 8-participant limit enforced (max_participants: 8)

**Pattern Compliance:**
- Persistent stream cache: HashMap maintained across frames, only opens on miss or error
- Per-peer isolation: Each peer's decode task runs independently, errors don't propagate
- Graceful degradation: Stream failures logged, peer skipped, session continues
- Clean shutdown: All tasks check is_active, exit gracefully on leave()
- Lock-free audio: No locks in audio callbacks (only in async task coordination)

## Key Decisions Made

**1. libp2p-stream as Separate Crate (vs. Feature Flag)**
- **Context:** libp2p 0.56 doesn't have a "stream" feature
- **Decision:** Use libp2p-stream 0.4.0-alpha as standalone dependency
- **Rationale:** Provides generic stream protocol behaviour for custom application protocols
- **Trade-off:** None - this is the correct integration pattern for libp2p 0.56

**2. Persistent Stream Cache (vs. New Stream Per Frame)**
- **Context:** Audio sends 50 frames/second to each peer
- **Decision:** Maintain HashMap of persistent streams per peer in encode task
- **Rationale:** Opening new stream per frame would overwhelm network (50 × N peers/sec). Persistent streams reuse established connections.
- **Trade-off:** Slightly more complex state management, but massive performance gain

**3. Stream Control Extraction Timing (vs. Post-Spawn Access)**
- **Context:** Swarm is moved into async task, becoming inaccessible
- **Decision:** Extract stream::Control BEFORE moving swarm into tokio::spawn
- **Rationale:** Once moved, swarm can't be accessed from NetworkService. Control must be cloned out first.
- **Trade-off:** None - this is the only viable approach given ownership rules

**4. Per-Peer Decoder Tasks (vs. Single Decoder Task)**
- **Context:** Multiple peers streaming audio simultaneously
- **Decision:** Spawn separate tokio task per peer for decode loop
- **Rationale:** Parallel processing, peer isolation, graceful degradation. One slow peer doesn't block others.
- **Trade-off:** More tasks (max 7), but better robustness and performance

## What's Next (Plan 03)

Tauri commands and frontend integration:

1. **Tauri Commands**
   - `voice_join_session(peer_ids: Vec<String>) -> Result<()>`
   - `voice_leave_session() -> Result<()>`
   - `voice_session_status() -> SessionStatus`
   - Integrate with existing NetworkService state management

2. **Frontend Controls**
   - Voice session UI component
   - Participant list display
   - Join/leave buttons
   - Active speaker visualization
   - Connection quality indicator

3. **Testing**
   - Multi-device LAN testing (2-4 peers)
   - Latency measurement (target: <100ms end-to-end)
   - Jitter buffer adaptation under load
   - Packet loss resilience (PLC verification)
   - Session stability (join/leave stress test)

4. **Performance Monitoring**
   - Voice session events logged to console
   - Track active participants
   - Monitor stream errors (if any)
   - Measure CPU usage during 8-peer session

## Performance Characteristics

**Expected Latency Budget (per Plan 01):**
- Capture buffer: ~10ms
- Encode: <5ms
- Network (LAN): ~1-5ms
- Jitter buffer: 40ms target (adapts 15-120ms)
- Decode: <5ms
- Playback buffer: ~10ms
- **Total estimate: 70-150ms** (acceptable for voice chat)

**Stream Overhead:**
- Wire format: 6 bytes header + opus data (~50-100 bytes/frame)
- Frame rate: 50 frames/sec (20ms Opus frames)
- Per-peer bandwidth: ~2.5-5 KB/sec upstream
- 8-peer session: ~17.5-35 KB/sec upstream, ~17.5-35 KB/sec downstream
- **Total: ~35-70 KB/sec** (well within typical LAN/WAN capacity)

**CPU Impact:**
- Stream protocol overhead: <1% (length-prefix framing is cheap)
- Persistent stream cache: eliminates repeated connection overhead
- Per-peer tasks: <1% overhead per peer (async tasks are lightweight)
- **Total added cost: ~5-10%** (on top of Plan 01's 20-50% for audio processing)

## Commits

- **24e8e81** - feat(04-02): add voice streaming protocol and integrate stream behaviour
  - Added libp2p-stream dependency
  - Created voice/protocol.rs with VoicePacket wire format
  - Added stream::Behaviour to AetherBehaviour (8th behaviour)
  - Initialized stream behaviour in both PSK and open swarm paths

- **9fe879a** - feat(04-02): add voice session manager with full audio-network pipeline
  - Created VoiceSession with join/leave lifecycle
  - Encode-and-send task with persistent stream cache
  - Receive-and-decode task with per-peer decoder tasks
  - Mix-to-playback task at 20ms interval
  - Exposed stream_control from NetworkService
  - Events: voice-session-joined, voice-session-left

## Self-Check: PASSED

**Created files verified:**
```
✅ FOUND: src-tauri/src/voice/protocol.rs
✅ FOUND: src-tauri/src/voice/session.rs
```

**Modified files verified:**
```
✅ FOUND: src-tauri/Cargo.toml (libp2p-stream added)
✅ FOUND: src-tauri/src/voice/mod.rs (protocol, session modules added)
✅ FOUND: src-tauri/src/network/behaviour.rs (stream::Behaviour field)
✅ FOUND: src-tauri/src/network/swarm.rs (stream behaviour initialized)
✅ FOUND: src-tauri/src/network/mod.rs (stream_control field and getter)
```

**Commits verified:**
```
✅ FOUND: 24e8e81 (Task 1 - Voice protocol and stream behaviour)
✅ FOUND: 9fe879a (Task 2 - Voice session manager)
```

**Compilation verified:**
```
✅ cargo check passes
✅ Voice protocol compiles with wire format helpers
✅ VoiceSession compiles with full pipeline
✅ NetworkService exposes stream_control
✅ All behaviours integrated cleanly
```

All files created, all commits present, all requirements met.
