pub mod capture;
pub mod codec;

// Re-export for use in other modules (will be used in Plan 02)
pub use codec::{VoiceEncoder, VoiceDecoder, SAMPLE_RATE, CHANNELS, FRAME_SIZE};
