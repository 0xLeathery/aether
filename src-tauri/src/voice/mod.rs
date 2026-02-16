pub mod capture;
pub mod codec;
pub mod jitter_buffer;
pub mod mixer;
pub mod playback;

// Re-export key types for external use
pub use capture::start_capture;
pub use codec::{VoiceEncoder, VoiceDecoder, SAMPLE_RATE, CHANNELS, FRAME_SIZE};
pub use jitter_buffer::{AudioFrame, JitterBuffer};
pub use mixer::AudioMixer;
pub use playback::start_playback;
