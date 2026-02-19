use crate::error::VoiceError;
use crate::voice::jitter_buffer::{AudioFrame, JitterBuffer};
use libp2p::PeerId;
use std::collections::{HashMap, HashSet};

/// Multi-peer audio mixer with participant limit
///
/// Maintains one jitter buffer per peer, mixes all available audio,
/// and enforces an 8-participant hard limit for mesh scalability.
pub struct AudioMixer {
    peer_buffers: HashMap<PeerId, JitterBuffer>,
    max_participants: usize,
    /// Peers whose audio is muted (silenced but buffer still drained)
    muted_peers: HashSet<PeerId>,
}

impl AudioMixer {
    /// Create a new audio mixer with 8-participant limit
    pub fn new() -> Self {
        Self {
            peer_buffers: HashMap::new(),
            max_participants: 8,
            muted_peers: HashSet::new(),
        }
    }

    /// Add a new peer to the voice session
    ///
    /// Returns `SessionFull` error if already at max capacity (8 peers).
    pub fn add_peer(&mut self, peer_id: PeerId) -> Result<(), VoiceError> {
        if self.peer_buffers.len() >= self.max_participants {
            return Err(VoiceError::SessionFull);
        }

        self.peer_buffers.insert(peer_id, JitterBuffer::new());
        Ok(())
    }

    /// Remove a peer from the voice session
    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        self.peer_buffers.remove(peer_id);
        self.muted_peers.remove(peer_id);
    }

    /// Mute a peer - their audio will be silenced but buffer still drained
    pub fn mute_peer(&mut self, peer_id: PeerId) {
        self.muted_peers.insert(peer_id);
    }

    /// Unmute a peer - their audio will be included in the mix again
    pub fn unmute_peer(&mut self, peer_id: &PeerId) {
        self.muted_peers.remove(peer_id);
    }

    /// Check if a peer is muted
    pub fn is_muted(&self, peer_id: &PeerId) -> bool {
        self.muted_peers.contains(peer_id)
    }

    /// Feed an audio frame from a specific peer
    ///
    /// Silently ignores frames from unknown peers (handles race conditions
    /// where peer leaves during frame transmission).
    pub fn feed_frame(&mut self, peer_id: &PeerId, frame: AudioFrame) {
        if let Some(buffer) = self.peer_buffers.get_mut(peer_id) {
            buffer.add_frame(frame);
        }
        // Silently drop frames from unknown peers (race condition safety)
    }

    /// Mix the next frame from all peers
    ///
    /// Retrieves one frame from each peer's jitter buffer (if ready),
    /// sums samples, normalizes by active peer count, and applies hard limiting.
    ///
    /// Muted peers have their buffer drained but audio is discarded (prevents buffer buildup).
    ///
    /// Returns a vector of mixed samples (typically FRAME_SIZE length).
    /// Returns silence (zeros) if no peers have ready frames.
    pub fn mix_next_frame(&mut self, frame_size: usize) -> Vec<f32> {
        let mut mixed = vec![0.0f32; frame_size];
        let mut active_count = 0;

        // Collect frames from all peers
        for (peer_id, buffer) in self.peer_buffers.iter_mut() {
            // Drain buffer for muted peers but discard the audio
            if self.muted_peers.contains(peer_id) {
                let _ = buffer.get_frame();
                continue;
            }

            if let Some(data) = buffer.get_frame() {
                active_count += 1;

                // Sum samples (handle variable frame sizes)
                let len = data.len().min(frame_size);
                for (i, &sample) in data.iter().take(len).enumerate() {
                    mixed[i] += sample;
                }
            }
        }

        if active_count > 0 {
            // Normalize by dividing by active peer count
            let scale = 1.0 / active_count as f32;
            for sample in mixed.iter_mut() {
                *sample *= scale;

                // Hard limiter: clamp to [-1.0, 1.0] to prevent clipping
                *sample = sample.clamp(-1.0, 1.0);
            }
        }

        mixed
    }

    /// Get current participant count
    pub fn participant_count(&self) -> usize {
        self.peer_buffers.len()
    }
}
