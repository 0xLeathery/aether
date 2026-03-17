# Phase 4: Real-Time Voice - Research

**Researched:** 2026-02-16
**Domain:** Real-time P2P audio streaming with mesh networking
**Confidence:** HIGH

## Summary

Phase 4 implements real-time voice communication over the existing libp2p mesh network established in Phase 2. The standard approach uses **cpal** for cross-platform audio I/O, **opus** codec for efficient low-latency encoding/decoding, and **libp2p-stream** for custom audio streaming protocol implementation. The 8-participant hard limit is well within the theoretical bounds for audio-only P2P mesh (estimated ~10 participants), though bandwidth management and jitter buffering are critical for quality.

The architecture follows a producer-consumer pattern: cpal captures microphone audio → encode with Opus → send via libp2p streams to all connected peers → receive from peers → decode → mix → output via cpal. Lock-free channels (crossbeam or custom SPSC queues) should be used between audio callback threads and network threads to achieve sub-50ms latency targets.

**Primary recommendation:** Build custom libp2p stream protocol for audio distribution, use cpal 0.17 for audio I/O, opus 1.5.x for encoding/decoding, implement adaptive jitter buffers (40-120ms), and use voice activity detection (VAD) to reduce bandwidth via discontinuous transmission (DTX).

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| cpal | 0.17.x | Cross-platform audio I/O | De facto standard for Rust audio capture/playback with 2943 code examples in Context7, supports all desktop platforms |
| opus/opusic-c | 1.5.x | Voice codec | Industry standard for real-time voice (26.5ms latency), patent-free, used by Discord/WebRTC/VoIP systems |
| libp2p-stream | 0.56.x | Custom streaming protocol | Built-in libp2p behaviour for stream-oriented protocols, enables bidirectional audio streams over existing mesh |
| crossbeam-channel | Latest | Lock-free audio pipeline | Eliminates mutex contention for sub-millisecond latency between audio callback and network threads |

### Supporting

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| webrtc-audio-processing | Latest | Audio processing pipeline | Optional: echo cancellation, noise removal, auto-gain control, built-in VAD |
| ringbuf | Latest | Lock-free ring buffer | Alternative to crossbeam for SPSC audio buffering between threads |
| dasp | Latest | DSP utilities | Audio sample format conversion, resampling if needed |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Custom libp2p stream | WebRTC (webrtc-rs) | WebRTC brings full ICE/STUN/TURN stack (redundant with libp2p dcutr), more complex API, monolithic codec configs. Custom protocol gives control over latency/quality. |
| P2P mesh | SFU (Selective Forwarding Unit) | SFU scales better beyond 8 participants but requires relay servers, contradicts sovereign architecture. Mesh is correct for small groups. |
| Opus | Other codecs (Speex, AAC) | Opus specifically designed for real-time voice with better latency/quality than alternatives. Speex deprecated, AAC has licensing issues. |

**Installation:**
```bash
# Add to src-tauri/Cargo.toml
cpal = "0.17"
opus = "0.3"  # or opusic-c = "1.5" for higher-level API
libp2p = { version = "0.56", features = ["stream"] }
crossbeam-channel = "0.5"
```

## Architecture Patterns

### Recommended Project Structure
```
src-tauri/src/
├── voice/
│   ├── mod.rs              # Public API, voice session management
│   ├── capture.rs          # Microphone capture via cpal
│   ├── playback.rs         # Audio output via cpal
│   ├── codec.rs            # Opus encoder/decoder
│   ├── mixer.rs            # Mix multiple peer audio streams
│   ├── jitter_buffer.rs    # Adaptive jitter buffer per peer
│   └── protocol.rs         # libp2p stream protocol for audio
└── network/
    └── behaviours/
        └── voice_stream.rs # Voice streaming behaviour
```

### Pattern 1: Audio Capture Pipeline

**What:** Non-blocking audio capture with lock-free channel to network thread
**When to use:** All audio input scenarios
**Example:**
```rust
// Source: https://docs.rs/cpal/0.17.0/cpal/
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{bounded, Sender};

pub fn start_capture(tx: Sender<Vec<f32>>) -> Result<cpal::Stream, Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or("No input device available")?;

    let config = device.default_input_config()?;

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // Audio callback - must not block!
            // Use try_send to drop frames under pressure rather than blocking
            let _ = tx.try_send(data.to_vec());
        },
        move |err| {
            eprintln!("Audio input error: {}", err);
        },
        None // Blocking
    )?;

    stream.play()?;
    Ok(stream)
}
```

### Pattern 2: Opus Encoding for Real-Time Voice

**What:** Configure Opus for VoIP with low latency
**When to use:** Before sending audio over network
**Example:**
```rust
// Source: https://wiki.xiph.org/Opus_Recommended_Settings
// VoIP settings: 48kHz, 20ms frames, 24kbps for fullband speech

use opus::{Encoder, Application, Channels};

pub fn create_encoder() -> Result<Encoder, opus::Error> {
    let mut encoder = Encoder::new(
        48000,                      // 48kHz sample rate
        Channels::Mono,             // Mono for bandwidth efficiency
        Application::Voip           // VoIP mode (not Audio or LowDelay)
    )?;

    encoder.set_bitrate(opus::Bitrate::Bits(24000))?;  // 24kbps for fullband
    encoder.set_complexity(10)?;     // Max quality (reduce if CPU-bound)
    encoder.set_vbr(true)?;          // Variable bitrate
    encoder.set_dtx(true)?;          // Discontinuous transmission (silence suppression)

    Ok(encoder)
}

// Frame size: 960 samples at 48kHz = 20ms
const FRAME_SIZE: usize = 960;

pub fn encode_frame(encoder: &mut Encoder, pcm: &[f32]) -> Vec<u8> {
    let mut output = vec![0u8; 4000];  // Max Opus frame size
    let len = encoder.encode_float(pcm, &mut output).unwrap();
    output.truncate(len);
    output
}
```

### Pattern 3: libp2p Custom Stream Protocol

**What:** Bidirectional audio streaming over libp2p
**When to use:** Sending/receiving audio packets between peers
**Example:**
```rust
// Source: https://github.com/libp2p/rust-libp2p/blob/master/protocols/stream/README.md
use libp2p_stream as stream;
use libp2p_swarm::{Swarm, StreamProtocol};
use futures::StreamExt;

const VOICE_PROTOCOL: StreamProtocol = StreamProtocol::new("/aether/voice/1.0.0");

// Sending audio to a peer
pub async fn send_audio(control: &mut stream::Control, peer_id: PeerId, audio_data: Vec<u8>) {
    let mut stream = control.open_stream(peer_id, VOICE_PROTOCOL)
        .await
        .expect("Failed to open stream");

    // Send frame length prefix + audio data
    let len_bytes = (audio_data.len() as u16).to_be_bytes();
    stream.write_all(&len_bytes).await.unwrap();
    stream.write_all(&audio_data).await.unwrap();
}

// Receiving audio from peers
pub async fn receive_audio(control: &mut stream::Control) {
    let mut incoming = control.accept(VOICE_PROTOCOL).unwrap();

    while let Some((peer, mut stream)) = incoming.next().await {
        tokio::spawn(async move {
            loop {
                // Read frame length
                let mut len_bytes = [0u8; 2];
                if stream.read_exact(&mut len_bytes).await.is_err() {
                    break;
                }
                let len = u16::from_be_bytes(len_bytes) as usize;

                // Read audio frame
                let mut audio_data = vec![0u8; len];
                if stream.read_exact(&mut audio_data).await.is_err() {
                    break;
                }

                // Process audio (decode, buffer, mix)
                handle_audio_frame(peer, audio_data).await;
            }
        });
    }
}
```

### Pattern 4: Adaptive Jitter Buffer

**What:** Per-peer buffer to handle network jitter
**When to use:** For each peer's incoming audio stream
**Example:**
```rust
// Source: https://webrtchacks.com/how-webrtcs-neteq-jitter-buffer-provides-smooth-audio/
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct JitterBuffer {
    buffer: VecDeque<AudioFrame>,
    target_delay: Duration,    // Start at 40ms
    min_delay: Duration,       // 15ms
    max_delay: Duration,       // 120ms
}

struct AudioFrame {
    sequence: u32,
    data: Vec<f32>,
    received_at: Instant,
}

impl JitterBuffer {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
            target_delay: Duration::from_millis(40),
            min_delay: Duration::from_millis(15),
            max_delay: Duration::from_millis(120),
        }
    }

    pub fn add_frame(&mut self, frame: AudioFrame) {
        // Insert sorted by sequence number
        let pos = self.buffer.iter()
            .position(|f| f.sequence > frame.sequence)
            .unwrap_or(self.buffer.len());
        self.buffer.insert(pos, frame);

        // Adapt buffer size based on jitter
        self.adapt_delay();
    }

    pub fn get_frame(&mut self) -> Option<Vec<f32>> {
        if self.buffer.is_empty() {
            return None;
        }

        let oldest = &self.buffer[0];
        let age = oldest.received_at.elapsed();

        // Wait until frame is old enough (target delay reached)
        if age >= self.target_delay {
            self.buffer.pop_front().map(|f| f.data)
        } else {
            None  // Not ready yet
        }
    }

    fn adapt_delay(&mut self) {
        // Increase delay if experiencing drops, decrease if buffer growing
        // Simplified adaptation logic
        if self.buffer.len() > 6 {
            self.target_delay = (self.target_delay + Duration::from_millis(5))
                .min(self.max_delay);
        } else if self.buffer.len() < 2 {
            self.target_delay = (self.target_delay.saturating_sub(Duration::from_millis(5)))
                .max(self.min_delay);
        }
    }
}
```

### Pattern 5: Audio Mixer for Multiple Participants

**What:** Mix decoded audio from all peers for output
**When to use:** Before playback
**Example:**
```rust
use std::collections::HashMap;
use libp2p_identity::PeerId;

pub struct AudioMixer {
    peer_buffers: HashMap<PeerId, JitterBuffer>,
    max_participants: usize,
}

impl AudioMixer {
    pub fn new(max_participants: usize) -> Self {
        Self {
            peer_buffers: HashMap::new(),
            max_participants,
        }
    }

    pub fn mix_next_frame(&mut self, frame_size: usize) -> Vec<f32> {
        let mut mixed = vec![0.0f32; frame_size];
        let mut active_peers = 0;

        // Get audio from each peer's jitter buffer
        for (_peer_id, buffer) in &mut self.peer_buffers {
            if let Some(peer_audio) = buffer.get_frame() {
                active_peers += 1;
                // Simple additive mixing
                for (i, sample) in peer_audio.iter().take(frame_size).enumerate() {
                    mixed[i] += sample;
                }
            }
        }

        // Normalize to prevent clipping
        if active_peers > 0 {
            let scale = 1.0 / (active_peers as f32);
            for sample in &mut mixed {
                *sample *= scale;
                *sample = sample.clamp(-1.0, 1.0);  // Hard limiter
            }
        }

        mixed
    }

    pub fn add_peer(&mut self, peer_id: PeerId) -> Result<(), &'static str> {
        if self.peer_buffers.len() >= self.max_participants {
            return Err("Maximum participants reached");
        }
        self.peer_buffers.insert(peer_id, JitterBuffer::new());
        Ok(())
    }

    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        self.peer_buffers.remove(peer_id);
    }
}
```

### Anti-Patterns to Avoid

- **Blocking in audio callbacks:** cpal audio callbacks run on real-time threads. Any blocking (mutex locks, I/O, allocation) causes audio glitches. Use lock-free channels with `try_send()`/`try_recv()`.
- **std::sync::mpsc in audio path:** Standard mpsc has mutex contention. Use crossbeam-channel or ringbuf for lock-free audio pipelines.
- **No jitter buffering:** Direct playback of received packets causes choppy audio. Always buffer to handle network jitter (40-120ms adaptive).
- **Synchronous encoding on audio thread:** Opus encoding should happen on separate thread, not in cpal callback. Use channels to pass PCM between threads.
- **Forgetting sequence numbers:** Audio packets can arrive out of order. Always include sequence numbers for proper jitter buffer ordering.
- **No packet loss concealment:** Missing packets create audible gaps. Implement PLC (Opus has built-in PLC when decoding with missing frames).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Audio codec | Custom PCM/ADPCM encoder | Opus codec | Opus specifically designed for real-time voice with excellent quality at 24kbps, built-in VAD/DTX, PLC, complexity scaling. Hand-rolled codecs lack these features. |
| Echo cancellation | Manual acoustic echo removal | webrtc-audio-processing | Echo cancellation requires adaptive filters tracking speaker-to-mic delay, room acoustics. WebRTC's AEC is battle-tested across billions of calls. |
| NAT traversal for voice | Custom UDP hole-punching | libp2p dcutr (already integrated) | Phase 2 already handles NAT traversal. Voice streams use existing libp2p connections. |
| Sample rate conversion | Manual interpolation/decimation | dasp or libsamplerate | Resampling requires anti-aliasing filters to avoid artifacts. Existing libraries are optimized and correct. |
| Audio device enumeration | Direct OS APIs | cpal | cpal abstracts platform differences (CoreAudio/WASAPI/ALSA), handles device hotplug, format negotiation. Platform-specific code is error-prone. |

**Key insight:** Real-time audio has extreme timing sensitivity. Production-grade codecs, jitter buffers, and audio I/O libraries represent years of optimization. Focus implementation effort on mesh distribution logic and integration, not reinventing audio fundamentals.

## Common Pitfalls

### Pitfall 1: Underestimating Mesh Bandwidth Requirements

**What goes wrong:** With 8 participants, each peer uploads 7 streams simultaneously. At 24kbps per stream, that's 168kbps upload required. Typical home upload is 5-10 Mbps, but congestion/other apps can saturate it, causing packet loss and quality degradation.

**Why it happens:** Developers focus on download bandwidth (abundant) and forget upload bandwidth (constrained). P2P mesh scales as O(N) upload per participant.

**How to avoid:**
- Enforce 8-participant hard limit in code, not just docs
- Implement bandwidth estimation and adaptive bitrate (drop to 16kbps or 12kbps if network degrades)
- Use Opus DTX (discontinuous transmission) so silence doesn't consume bandwidth
- Consider implementing "active speaker detection" to only send audio from speaking participants

**Warning signs:** Choppy audio when 6+ participants join, reports of "works fine with 3 people but breaks at 7", increased jitter buffer underruns.

### Pitfall 2: Audio/Network Thread Synchronization Issues

**What goes wrong:** Audio callback runs on high-priority real-time thread (cpal), network I/O on tokio thread pool. Improper synchronization causes deadlocks, priority inversion, or dropped audio frames. Symptoms: periodic audio glitches, frozen UI, tokio tasks starving.

**Why it happens:** Real-time audio and async runtimes have conflicting requirements. Audio threads must never block, but async code naturally yields/blocks.

**How to avoid:**
- Never await/block in cpal callbacks
- Use bounded lock-free channels (crossbeam) with `try_send` in audio thread
- Set channel capacity to hold ~200ms of frames (10 frames at 20ms each) - enough to absorb scheduling jitter but not excessive latency
- If channel full, drop oldest frame (underrun) rather than blocking
- Network thread continuously drains channel and sends over libp2p

**Warning signs:** Audio stuttering every few seconds, "channel send blocked" logs, audio thread taking >5ms per callback.

### Pitfall 3: Ignoring Platform-Specific Audio Permissions

**What goes wrong:** macOS requires microphone permission in Info.plist, but Tauri doesn't add this by default. App silently fails to capture audio, or system denies access. Users see "microphone not working" but no error messages.

**Why it happens:** Desktop apps (unlike browsers) need OS-level permissions. Tauri doesn't automatically configure platform-specific settings.

**How to avoid:**
- Add to `src-tauri/tauri.conf.json`:
  ```json
  "bundle": {
    "macOS": {
      "infoPlist": {
        "NSMicrophoneUsageDescription": "Aether needs microphone access for voice chat"
      }
    }
  }
  ```
- Linux: Document PulseAudio/PipeWire requirements
- Windows: No special permissions, but test with Windows Defender

**Warning signs:** "No input device available" error on macOS only, works in dev but not production build, user reports "microphone blocked".

### Pitfall 4: No Graceful Degradation for Packet Loss

**What goes wrong:** Real-world networks lose packets (1-5% typical, up to 20% on bad WiFi). Without packet loss concealment (PLC), every lost packet creates 20ms audio gap, making speech unintelligible.

**Why it happens:** Testing on localhost or good networks hides packet loss. Opus has PLC but only if decoder is invoked with explicit packet loss indication.

**How to avoid:**
- Track sequence numbers on received packets
- When gap detected, call Opus decoder with PLC flag:
  ```rust
  decoder.decode_float(None, &mut output, true)  // true = use PLC
  ```
- Opus generates plausible audio for up to 120ms loss
- For longer gaps, insert comfort noise or mute
- Log packet loss rate per peer for monitoring

**Warning signs:** "Works great on LAN but terrible over internet", choppy audio on WiFi, user reports "every word cuts out".

### Pitfall 5: Sub-Optimal Opus Configuration for Voice

**What goes wrong:** Using Opus defaults (Audio mode, no DTX, high bitrate) instead of VoIP-specific settings wastes bandwidth and increases latency. Audio mode prioritizes music quality over latency, using 40-60ms frames instead of 20ms.

**Why it happens:** Opus documentation shows generic examples, not VoIP-tuned configs. Developers copy defaults without understanding mode differences.

**How to avoid:**
- Use `Application::Voip` mode (not Audio or LowDelay)
- Enable DTX: `encoder.set_dtx(true)` - reduces bandwidth during silence
- Set bitrate 24kbps for fullband speech, 16kbps for wideband
- Use 20ms frames (960 samples at 48kHz) - balance of latency and efficiency
- Enable FEC if packet loss >2%: `encoder.set_inband_fec(true)`

**Warning signs:** Latency >100ms reported by users, excessive bandwidth usage (>50kbps per stream), "audio lags behind video" if adding video later.

### Pitfall 6: Race Condition in Voice Session Join/Leave

**What goes wrong:** Peer joins voice session while you're iterating peer list to send audio, or peer disconnects mid-stream send. Causes panics on send to closed stream or missing peer in mixer.

**Why it happens:** Voice session membership changes asynchronously (network events) while audio thread sends continuously (20ms intervals).

**How to avoid:**
- Maintain peer list with RwLock: read lock for sending, write lock for membership changes
- Use `try_send` on streams and handle closed stream error gracefully
- Audio mixer should handle missing peers: if jitter buffer doesn't exist, skip that peer
- Send session membership events to audio thread via channel before network changes

**Warning signs:** "Broken pipe" errors during voice sessions, panics on peer disconnect, "peer not found" when mixing audio.

## Code Examples

Verified patterns from official sources:

### Listing Audio Devices with cpal

```rust
// Source: https://docs.rs/cpal/0.17.0/cpal/
use cpal::traits::{HostTrait, DeviceTrait};

fn list_audio_devices() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();

    println!("Input devices:");
    for device in host.input_devices()? {
        println!("  {}", device.name()?);
        let config = device.default_input_config()?;
        println!("    Default: {:?}", config);
    }

    println!("Output devices:");
    for device in host.output_devices()? {
        println!("  {}", device.name()?);
        let config = device.default_output_config()?;
        println!("    Default: {:?}", config);
    }

    Ok(())
}
```

### Complete Voice Session Manager Structure

```rust
// Integration pattern for voice session
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

pub struct VoiceSession {
    session_id: String,
    mixer: Arc<RwLock<AudioMixer>>,
    capture_stream: Option<cpal::Stream>,
    playback_stream: Option<cpal::Stream>,
    encoder: opus::Encoder,
    decoders: Arc<RwLock<HashMap<PeerId, opus::Decoder>>>,
    stream_control: stream::Control,
    is_active: Arc<AtomicBool>,
}

impl VoiceSession {
    pub async fn join(&mut self, peer_ids: Vec<PeerId>) -> Result<(), VoiceError> {
        // 1. Check participant limit
        if peer_ids.len() > 8 {
            return Err(VoiceError::TooManyParticipants);
        }

        // 2. Initialize mixer and decoders for each peer
        let mut mixer = self.mixer.write().await;
        let mut decoders = self.decoders.write().await;
        for peer_id in peer_ids {
            mixer.add_peer(peer_id)?;
            decoders.insert(peer_id, opus::Decoder::new(48000, opus::Channels::Mono)?);
        }

        // 3. Start audio capture
        self.capture_stream = Some(self.start_capture()?);

        // 4. Start audio playback
        self.playback_stream = Some(self.start_playback()?);

        // 5. Mark session active
        self.is_active.store(true, Ordering::Relaxed);

        Ok(())
    }

    pub async fn leave(&mut self) {
        self.is_active.store(false, Ordering::Relaxed);
        self.capture_stream = None;  // Drop stops stream
        self.playback_stream = None;
        self.mixer.write().await.peer_buffers.clear();
        self.decoders.write().await.clear();
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Speex codec | Opus codec | ~2013 | Opus provides better quality at lower bitrates with lower latency. Speex is deprecated. |
| Static jitter buffers | Adaptive jitter buffers | ~2015 | Adaptive buffers adjust to network conditions (15-120ms) vs fixed 60ms+. Reduces latency on good networks. |
| Server-based mixing (MCU) | SFU or P2P mesh | ~2016 | Trend away from server-side mixing. SFU for scale, P2P for privacy/cost. For 8 users, mesh is viable. |
| std::sync::mpsc | Lock-free queues (crossbeam, ringbuf) | ~2020 | Lock-free channels eliminate contention for real-time audio. Critical for sub-50ms latency. |
| Manual WebRTC stack | High-level libraries (webrtc-rs, mediasoup) | ~2021 | WebRTC complexity abstracted. But for this project, custom libp2p protocol simpler than full WebRTC. |

**Deprecated/outdated:**
- **Speex codec:** Obsoleted by Opus. Use Opus for all new projects.
- **G.711/G.729:** Legacy telecom codecs with worse quality/latency than Opus.
- **Fixed 20ms frame playback:** Modern systems use adaptive jitter buffers, not fixed playback schedules.
- **WebRTC for everything:** WebRTC is overbuilt for simple use cases. Custom protocols over libp2p are lighter for established P2P meshes.

## Open Questions

1. **Echo Cancellation Necessity**
   - What we know: webrtc-audio-processing provides AEC, but adds complexity and CPU load
   - What's unclear: Whether echo cancellation is essential for desktop app with headphones (most users), or only needed for speakers
   - Recommendation: Start without AEC (assume headphones), add opt-in AEC in Phase 5 if user feedback demands it. Desktop users typically use headphones for voice chat.

2. **Voice Session Discovery**
   - What we know: libp2p mesh already handles peer discovery (Phase 2), swarm formation (Phase 3)
   - What's unclear: How voice session membership is communicated - separate libp2p protocol, or embedded in existing channel messages
   - Recommendation: Use libp2p request-response protocol for voice session join/leave signaling. Keep voice data streaming separate from control plane.

3. **Opus Frame Size Trade-off**
   - What we know: 20ms frames are standard, but 10ms frames reduce latency at cost of overhead
   - What's unclear: Whether 10ms frames provide perceptible improvement vs 20ms for target sub-50ms end-to-end latency
   - Recommendation: Start with 20ms (industry standard), measure end-to-end latency, experiment with 10ms only if measurements show bottleneck in encoding latency. Network jitter (20-40ms) likely dominates vs frame size.

4. **Bandwidth Adaptation Strategy**
   - What we know: Opus supports dynamic bitrate adjustment (12-48kbps), DTX helps with silence
   - What's unclear: Best strategy for detecting congestion and coordinating bitrate drops across peers in mesh
   - Recommendation: Implement per-peer send bitrate control based on observed packet loss rate (track at receiver, send feedback). Drop bitrate to 16kbps if loss >5%, 12kbps if >10%. Simpler than distributed congestion control.

## Sources

### Primary (HIGH confidence)

- [libp2p/rust-libp2p Context7](https://context7.com/libp2p/rust-libp2p/llms.txt) - Stream protocol implementation patterns
- [cpal 0.17.0 Context7](https://docs.rs/cpal/0.17.0/cpal/) - Audio I/O API, stream configuration, callback patterns
- [Opus Recommended Settings - Xiph.org](https://wiki.xiph.org/Opus_Recommended_Settings) - VoIP codec configuration, bitrate/complexity/frame size
- [opusic-c 1.5.2 Context7](https://docs.rs/opusic-c/1.5.2/opusic_c/) - Rust Opus bindings, encoder/decoder API

### Secondary (MEDIUM confidence)

- [WebRTC P2P Mesh - BlogGeek.me](https://bloggeek.me/webrtc-p2p-mesh/) - Mesh scalability limits, 8-10 participant ceiling for audio
- [Jitter Buffer - Digital Samba](https://www.digitalsamba.com/blog/jitter-buffers-optimising-real-time-communications) - Adaptive jitter buffer algorithms, 15-120ms range
- [WebRTC NetEQ Jitter Buffer - webrtcHacks](https://webrtchacks.com/how-webrtcs-neteq-jitter-buffer-provides-smooth-audio/) - Packet loss concealment, comfort noise generation
- [Opus Discontinuous Transmission - GetStream](https://getstream.io/resources/projects/webrtc/advanced/dtx/) - DTX bandwidth savings, silence detection
- [WebRTC Audio Processing - GitHub](https://github.com/tonarino/webrtc-audio-processing) - Rust bindings for echo cancellation, noise suppression, VAD
- [WebRTC Latency Comparison - Nanocosmos](https://www.nanocosmos.net/webrtc-latency/) - Real-world latency measurements: 120-250ms internet, sub-50ms LAN
- [Voice Activity Detection Guide 2026 - Picovoice](https://picovoice.ai/blog/complete-guide-voice-activity-detection-vad/) - VAD algorithms, RNN-VAD in Opus
- [Sub-Millisecond Latency Rust - OneUpTime](https://oneuptime.com/blog/post/2026-01-25-streaming-data-sub-millisecond-latency-rust/view) - Lock-free channel patterns for real-time streaming
- [WebRTC vs Custom Protocols - Red5](https://www.red5.net/blog/moq-vs-webrtc/) - Protocol comparison for 2026, WebRTC vs custom approaches

### Tertiary (LOW confidence)

- Various Rust audio forum discussions on cpal/opus integration
- GitHub issues on tauri-plugin-audio-recorder for permission handling patterns

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - cpal, Opus, libp2p-stream are established standards with extensive documentation
- Architecture: HIGH - Patterns verified from official docs and production implementations (WebRTC, Discord-style architecture)
- Pitfalls: MEDIUM - Derived from WebRTC/VoIP domain knowledge and Rust audio forum discussions, not all Aether-specific

**Research date:** 2026-02-16
**Valid until:** ~60 days (2026-04-17) - Audio standards are stable, cpal/opus unlikely to change significantly. libp2p 0.56 is current.

## Notes for Planning

- Phase 2's libp2p mesh provides connectivity layer - voice builds on existing connections
- 8-participant limit aligns with P2P mesh bandwidth constraints (168kbps upload per user)
- Sub-50ms latency target is achievable on LAN (<50ms measured), challenging over internet (120-250ms typical) - target should be "<100ms on typical home networks" as more realistic success criteria
- Voice session is per-channel (from Phase 3 swarm channels) - multiple users can join voice in same channel
- No relay servers needed - all audio flows P2P via libp2p streams (consistent with sovereign architecture)
- Audio permissions handled via Tauri platform-specific config, not runtime prompts
- Consider adding audio-level indicators in UI (visualize who's speaking) - useful for 8-person mesh