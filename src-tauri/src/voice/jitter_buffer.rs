use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// A single audio frame with sequence number and receive timestamp
#[derive(Debug, Clone)]
pub struct AudioFrame {
    pub sequence: u32,
    pub data: Vec<f32>,
    pub received_at: Instant,
}

/// Adaptive jitter buffer for reordering out-of-sequence audio frames
///
/// Maintains frames in sequence order and adapts buffering delay based on
/// network jitter to balance latency vs. packet loss.
pub struct JitterBuffer {
    buffer: VecDeque<AudioFrame>,
    target_delay: Duration,
    min_delay: Duration,
    max_delay: Duration,
    next_expected_seq: u32,
}

impl JitterBuffer {
    /// Create a new jitter buffer with adaptive delay
    ///
    /// - Initial target delay: 40ms
    /// - Min delay: 15ms (aggressive, low latency)
    /// - Max delay: 120ms (conservative, high jitter tolerance)
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
            target_delay: Duration::from_millis(40),
            min_delay: Duration::from_millis(15),
            max_delay: Duration::from_millis(120),
            next_expected_seq: 0,
        }
    }

    /// Add a frame to the jitter buffer
    ///
    /// Frames are inserted in sequence order. The buffer adapts its target
    /// delay based on fill level to handle varying network jitter.
    pub fn add_frame(&mut self, frame: AudioFrame) {
        // Find insertion position (maintain sorted order by sequence)
        let insert_pos = self
            .buffer
            .iter()
            .position(|f| f.sequence > frame.sequence)
            .unwrap_or(self.buffer.len());

        self.buffer.insert(insert_pos, frame);
        self.adapt_delay();
    }

    /// Get the next frame if it's ready to play
    ///
    /// Returns `Some(data)` if the oldest frame has waited >= target_delay.
    /// Returns `None` if buffer is empty or frames aren't ready yet.
    pub fn get_frame(&mut self) -> Option<Vec<f32>> {
        if let Some(frame) = self.buffer.front() {
            let wait_time = frame.received_at.elapsed();
            if wait_time >= self.target_delay {
                let frame = self.buffer.pop_front().unwrap();
                self.next_expected_seq = frame.sequence.wrapping_add(1);
                return Some(frame.data);
            }
        }
        None
    }

    /// Check if there's a gap in the sequence (for PLC triggering)
    ///
    /// Returns true if the next frame in buffer has a sequence number
    /// higher than expected (indicating packet loss).
    pub fn has_gap(&self) -> bool {
        if let Some(frame) = self.buffer.front() {
            // Check if there's a gap between expected and actual sequence
            frame.sequence != self.next_expected_seq
        } else {
            false
        }
    }

    /// Adapt target delay based on buffer fill level
    ///
    /// - If buffer is deep (>6 frames): increase delay (tolerate more jitter)
    /// - If buffer is shallow (<2 frames): decrease delay (reduce latency)
    ///
    /// This adapts to network conditions dynamically.
    fn adapt_delay(&mut self) {
        let len = self.buffer.len();

        if len > 6 {
            // Buffer filling up - increase delay to tolerate jitter
            self.target_delay = (self.target_delay + Duration::from_millis(5)).min(self.max_delay);
        } else if len < 2 {
            // Buffer draining - decrease delay to reduce latency
            self.target_delay = (self.target_delay.saturating_sub(Duration::from_millis(5)))
                .max(self.min_delay);
        }
    }

    /// Get current buffer length (for monitoring)
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
