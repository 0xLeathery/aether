---
title: Voice
description: How Aether handles real-time voice communication with Opus codec and P2P mesh topology.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# Voice Protocol

> This is a simplified overview of Aether's voice system, not a real-time audio engineering specification.

Aether supports real-time voice chat between peers, with no mixing server or relay infrastructure. Audio is captured, encoded, transmitted, decoded, and played back entirely through direct peer-to-peer connections.

## Opus Codec

Aether uses the **Opus** audio codec at approximately **48kbps**. Opus is an open standard designed specifically for real-time speech, offering excellent voice quality at low bitrate. It is the same codec used by Discord, WebRTC, and most modern VoIP applications.

The target end-to-end latency is under **50ms on LAN** -- fast enough that conversation feels natural with no perceptible delay.

## Mesh Topology

Voice sessions use a **full mesh** topology: each peer in a voice channel sends their audio stream directly to every other participant.

With N peers in a voice session:
- Each peer sends **N-1** outgoing audio streams
- Each peer receives **N-1** incoming audio streams
- Total streams across all peers: **N x (N-1)**

This scales quadratically, which is why Aether enforces an **8-peer hard limit** for voice sessions. At 8 peers, each participant handles 7 streams -- manageable for modern hardware and home network connections. Beyond 8, bandwidth and CPU requirements become impractical without a mixing server (which Aether intentionally avoids).

| Peers | Streams per peer | Total streams |
|-------|-----------------|---------------|
| 2 | 1 | 2 |
| 4 | 3 | 12 |
| 6 | 5 | 30 |
| 8 | 7 | 56 |

## Audio Pipeline

The full audio path, all processed in the Rust backend:

1. **Capture** -- Microphone audio captured via `cpal` (cross-platform audio library) or system audio APIs
2. **Encode** -- Raw PCM audio compressed with the Opus encoder at 48kbps
3. **Transmit** -- Encoded packets sent over `libp2p` streams to each connected peer
4. **Receive** -- Incoming packets from each peer arrive over separate `libp2p` streams
5. **Decode** -- Each peer's audio decoded from Opus back to PCM
6. **Mix** -- Decoded audio from all peers mixed into a single output stream
7. **Playback** -- Mixed audio sent to the system speaker/headphone output

All encoding, decoding, and mixing happens in the Rust backend. The Svelte frontend only handles the UI (mute button, voice channel membership display).

## Mute

Muting **stops sending audio packets** -- the microphone is not captured and no data is transmitted. Other peers are notified of your mute state via a control message, so the UI can display who is muted. Unmuting resumes audio capture and transmission.

## Connection Model

Voice sessions are per-channel. When you join a voice channel:

1. Aether initiates connections to all other participants currently in the channel
2. Audio streams begin flowing in both directions
3. When a new peer joins, existing participants establish new streams with them
4. When you leave, all your streams stop

## Limitations

The current voice implementation has known limitations:

- **No echo cancellation** -- Use headphones to avoid feedback loops. This is the most important practical consideration.
- **No noise suppression** -- Background noise is transmitted as-is.
- **No automatic gain control** -- Peers with different microphone volumes may sound louder or quieter.
- **8-peer limit** -- The full mesh topology does not scale beyond 8 participants.

These are targeted for future improvement. Echo cancellation and noise suppression are the highest-priority audio enhancements.

## Related Pages

- [Architecture Overview](/docs/architecture) -- Where voice fits in the system stack
- [Networking](/docs/architecture/networking) -- Transport layer that carries audio streams
- [Encryption](/docs/architecture/encryption) -- How voice data is protected in transit
