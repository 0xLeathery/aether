---
title: Voice Chat
description: Real-time peer-to-peer voice sessions using Opus codec and WebRTC mesh networking.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# Voice Chat

Aether voice chat is direct peer-to-peer audio using the `Opus` codec. There is no voice server -- audio streams directly between connected peers via `WebRTC`.

## Joining a Voice Session

Click the voice button in a channel to join. Your microphone activates and you connect to other peers already in the session. Audio flows immediately.

## Mute and Unmute

Toggle your microphone with the mute button. Your mute state is visible to other peers so they know when you are not transmitting.

## Peer Limit

Voice sessions have a hard limit of **8 simultaneous participants**. This is a mesh network constraint -- each peer sends audio to every other peer directly. At 8 peers, that is 56 audio streams, which approaches the practical bandwidth ceiling.

## Audio Quality

Aether uses the `Opus` codec at approximately 48kbps per stream. On a local network, expect latency under 50ms. Over the internet, latency varies depending on the distance between peers.

## Leaving Voice

Click the voice button again or switch channels. Audio stops immediately and other peers see you leave the session.

## Troubleshooting

If voice does not connect, check these common issues:

- **Microphone permissions** -- Your OS may need to grant Aether access to the microphone.
- **Firewall or NAT** -- Aether uses `STUN` to traverse NAT, but symmetric NAT (common on corporate networks) may prevent direct connections. Approximately 10-20% of users behind symmetric NAT may experience connection issues.
- **No relay fallback** -- Aether does not use `TURN` relay servers. If a direct peer-to-peer path cannot be established, voice will not connect. This is a trade-off of the zero-server architecture.
