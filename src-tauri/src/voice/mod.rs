pub mod capture;
pub mod codec;
pub mod jitter_buffer;
pub mod mixer;
pub mod playback;
pub mod protocol;
pub mod session;

// Re-export key types for external use
pub use capture::start_capture;
pub use codec::{VoiceEncoder, VoiceDecoder, SAMPLE_RATE, CHANNELS, FRAME_SIZE};
pub use jitter_buffer::{AudioFrame, JitterBuffer};
pub use mixer::AudioMixer;
pub use playback::start_playback;
pub use protocol::{VoicePacket, VOICE_PROTOCOL, encode_packet, decode_packet, send_frame, recv_frame};
pub use session::VoiceSession;
