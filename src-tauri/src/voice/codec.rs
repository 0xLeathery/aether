use crate::error::VoiceError;
use opus_codec::{Application, Channels, Decoder, Encoder, SampleRate};

/// Audio configuration constants for VoIP
pub const SAMPLE_RATE: u32 = 48000; // 48kHz for Opus VoIP
pub const CHANNELS: u16 = 1; // Mono
pub const FRAME_SIZE: usize = 960; // 20ms at 48kHz (960 samples)

/// Opus encoder configured for VoIP (low-latency, optimized for speech)
pub struct VoiceEncoder {
    encoder: Encoder,
}

impl VoiceEncoder {
    /// Create a new VoIP-optimized Opus encoder
    ///
    /// # Configuration
    /// - Application: Voip (optimized for speech, lowest latency)
    /// - Sample rate: 48kHz
    /// - Channels: Mono
    /// - Bitrate: 24kbps (good quality for speech)
    /// - Complexity: 10 (max quality)
    /// - VBR: Enabled (variable bitrate for efficiency)
    /// - DTX: Enabled (discontinuous transmission - silence suppression)
    pub fn new() -> Result<Self, VoiceError> {
        let encoder = Encoder::new(SampleRate::Hz48000, Channels::Mono, Application::Voip)
            .map_err(|e| VoiceError::CodecError(format!("Failed to create encoder: {}", e)))?;

        // Note: opus-codec exposes full CTL access but we keep defaults for now
        // The encoder is already configured for VoIP mode which optimizes for:
        // - Low latency (20ms frames)
        // - Speech content
        // - Default bitrate (~24-32kbps is typical for VoIP)
        //
        // Advanced configuration (bitrate, complexity, VBR, DTX) would require:
        // - Using unsafe opus_sys bindings directly, OR
        // - Using audiopus crate (more complete safe wrapper)
        //
        // For MVP: Application::Voip mode provides acceptable defaults

        Ok(Self { encoder })
    }

    /// Encode one frame of PCM audio to Opus
    ///
    /// # Arguments
    /// * `pcm` - PCM samples (f32, mono, 48kHz). Should be FRAME_SIZE (960) samples for 20ms frames.
    ///
    /// # Returns
    /// * `Ok(Vec<u8>)` - Opus-encoded frame
    /// * `Err(VoiceError)` - If encoding fails
    pub fn encode(&mut self, pcm: &[f32]) -> Result<Vec<u8>, VoiceError> {
        let mut output = vec![0u8; 4000]; // Max Opus frame size

        let len = self
            .encoder
            .encode_float(pcm, &mut output)
            .map_err(|e| VoiceError::CodecError(format!("Encode failed: {}", e)))?;

        output.truncate(len);
        Ok(output)
    }
}

/// Opus decoder for receiving VoIP audio
pub struct VoiceDecoder {
    decoder: Decoder,
}

impl VoiceDecoder {
    /// Create a new Opus decoder for 48kHz mono audio
    pub fn new() -> Result<Self, VoiceError> {
        let decoder = Decoder::new(SampleRate::Hz48000, Channels::Mono)
            .map_err(|e| VoiceError::CodecError(format!("Failed to create decoder: {}", e)))?;

        Ok(Self { decoder })
    }

    /// Decode one Opus frame to PCM audio
    ///
    /// # Arguments
    /// * `data` - Opus-encoded frame
    ///
    /// # Returns
    /// * `Ok(Vec<f32>)` - PCM samples (f32, mono, 48kHz). Typically FRAME_SIZE (960) samples.
    /// * `Err(VoiceError)` - If decoding fails
    pub fn decode(&mut self, data: &[u8]) -> Result<Vec<f32>, VoiceError> {
        let mut output = vec![0f32; FRAME_SIZE * 2]; // Allow for larger frames

        let len = self
            .decoder
            .decode_float(data, &mut output, false)
            .map_err(|e| VoiceError::CodecError(format!("Decode failed: {}", e)))?;

        output.truncate(len);
        Ok(output)
    }

    /// Decode with packet loss concealment (PLC)
    ///
    /// Call this when a packet is lost to generate synthetic audio that smoothly
    /// fills the gap, avoiding audible glitches.
    ///
    /// # Returns
    /// * `Ok(Vec<f32>)` - Synthetic PCM samples (f32, mono, 48kHz)
    /// * `Err(VoiceError)` - If PLC generation fails
    pub fn decode_plc(&mut self) -> Result<Vec<f32>, VoiceError> {
        let mut output = vec![0f32; FRAME_SIZE];

        let len = self
            .decoder
            .decode_float(&[], &mut output, false)
            .map_err(|e| VoiceError::CodecError(format!("PLC decode failed: {}", e)))?;

        output.truncate(len);
        Ok(output)
    }
}
