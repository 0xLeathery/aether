# T04: 09-peer-moderation 04

**Slice:** S11 — **Milestone:** M001

## Description

Wire VoiceSession's mute_peer/unmute_peer methods to actually call AudioMixer's mute/unmute methods, closing the voice mute enforcement gap identified in 09-VERIFICATION.md.

Purpose: MOD-02 requires blocking to "refuse voice audio." The AudioMixer has working mute logic (drains buffer, discards audio) but VoiceSession never calls it. This plan connects the two.
Output: VoiceSession.mute_peer() that resolves hex keys to PeerIds and calls mixer.mute_peer() for active participants, plus join-time mute application for new participants.

## Must-Haves

- [ ] "VoiceSession::mute_peer() resolves hex key to PeerId and calls mixer.mute_peer() for active participants"
- [ ] "VoiceSession::unmute_peer() resolves hex key to PeerId and calls mixer.unmute_peer() for active participants"
- [ ] "When a new participant joins via session.join(), muted_peer_keys are checked and applied to the mixer"
- [ ] "Misleading implementation comment on mute_peer() is removed"

## Files

- `src-tauri/src/voice/session.rs`
