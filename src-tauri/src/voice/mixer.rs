use crate::error::VoiceError;
use crate::voice::jitter_buffer::{AudioFrame, JitterBuffer};
use libp2p::PeerId;
use std::collections::HashMap;

/// Multi-peer audio mixer with participant limit
///
/// Maintains one jitter buffer per peer, mixes all available audio,
/// and enforces an 8-participant hard limit for mesh scalability.
pub struct AudioMixer {
    peer_buffers: HashMap<PeerId, JitterBuffer>,
    max_participants: usize,
}

impl AudioMixer {
    /// Create a new audio mixer with 8-participant limit
    pub fn new() -> Self {
        Self {
            peer_buffers: HashMap::new(),
            max_participants: 8,
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
    /// Returns a vector of mixed samples (typically FRAME_SIZE length).
    /// Returns silence (zeros) if no peers have ready frames.
    pub fn mix_next_frame(&mut self, frame_size: usize) -> Vec<f32> {
        let mut mixed = vec![0.0f32; frame_size];
        let mut active_count = 0;

        // Collect frames from all peers
        for buffer in self.peer_buffers.values_mut() {
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
