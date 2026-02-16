use futures::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use libp2p::StreamProtocol;

use crate::error::VoiceError;

/// Voice streaming protocol identifier
pub const VOICE_PROTOCOL: StreamProtocol = StreamProtocol::new("/aether/voice/1.0.0");

/// Voice packet for network transmission
///
/// Wire format (big-endian):
/// - 4 bytes: sequence number (u32)
/// - 2 bytes: opus data length (u16)
/// - N bytes: opus data
#[derive(Debug, Clone)]
pub struct VoicePacket {
    /// Monotonically increasing sequence number per sender
    /// Used by jitter buffer for frame ordering
    pub sequence: u32,
    /// Opus-encoded audio frame (typically 20-100 bytes)
    pub opus_data: Vec<u8>,
}

impl VoicePacket {
    /// Create a new voice packet
    pub fn new(sequence: u32, opus_data: Vec<u8>) -> Self {
        Self { sequence, opus_data }
    }
}

/// Encode a voice packet to wire format
///
/// Format: [sequence:4][length:2][opus_data:N]
pub fn encode_packet(packet: &VoicePacket) -> Vec<u8> {
    let mut buf = Vec::with_capacity(6 + packet.opus_data.len());

    // 4 bytes: sequence number (big-endian)
    buf.extend_from_slice(&packet.sequence.to_be_bytes());

    // 2 bytes: opus data length (big-endian)
    let len = packet.opus_data.len() as u16;
    buf.extend_from_slice(&len.to_be_bytes());

    // N bytes: opus data
    buf.extend_from_slice(&packet.opus_data);

    buf
}

/// Decode a voice packet from wire format
///
/// Validates lengths and returns error on malformed packets
pub fn decode_packet(data: &[u8]) -> Result<VoicePacket, VoiceError> {
    if data.len() < 6 {
        return Err(VoiceError::CodecError(
            "Packet too short (minimum 6 bytes)".to_string()
        ));
    }

    // Read sequence number (4 bytes, big-endian)
    let sequence = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);

    // Read opus data length (2 bytes, big-endian)
    let opus_len = u16::from_be_bytes([data[4], data[5]]) as usize;

    // Validate opus data length matches remaining bytes
    if data.len() != 6 + opus_len {
        return Err(VoiceError::CodecError(
            format!("Packet length mismatch: header says {} bytes, got {} bytes",
                opus_len, data.len() - 6)
        ));
    }

    // Extract opus data
    let opus_data = data[6..].to_vec();

    Ok(VoicePacket { sequence, opus_data })
}

/// Send a voice frame over a libp2p stream
///
/// Writes length-prefixed packet to stream
pub async fn send_frame(
    stream: &mut (impl AsyncWrite + Unpin),
    packet: &VoicePacket,
) -> Result<(), VoiceError> {
    let encoded = encode_packet(packet);

    stream
        .write_all(&encoded)
        .await
        .map_err(|e| VoiceError::AudioStreamFailed(format!("Stream write failed: {}", e)))?;

    stream
        .flush()
        .await
        .map_err(|e| VoiceError::AudioStreamFailed(format!("Stream flush failed: {}", e)))?;

    Ok(())
}

/// Receive a voice frame from a libp2p stream
///
/// Reads length-prefixed packet from stream
/// Returns error on stream close or malformed packet
pub async fn recv_frame(
    stream: &mut (impl AsyncRead + Unpin),
) -> Result<VoicePacket, VoiceError> {
    // Read header (6 bytes: sequence + length)
    let mut header = [0u8; 6];
    stream
        .read_exact(&mut header)
        .await
        .map_err(|e| VoiceError::AudioStreamFailed(format!("Stream read failed: {}", e)))?;

    let sequence = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
    let opus_len = u16::from_be_bytes([header[4], header[5]]) as usize;

    // Validate reasonable opus packet size (max 1500 bytes)
    if opus_len > 1500 {
        return Err(VoiceError::CodecError(
            format!("Opus packet too large: {} bytes", opus_len)
        ));
    }

    // Read opus data
    let mut opus_data = vec![0u8; opus_len];
    stream
        .read_exact(&mut opus_data)
        .await
        .map_err(|e| VoiceError::AudioStreamFailed(format!("Stream read failed: {}", e)))?;

    Ok(VoicePacket { sequence, opus_data })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let packet = VoicePacket::new(42, vec![1, 2, 3, 4, 5]);
        let encoded = encode_packet(&packet);
        let decoded = decode_packet(&encoded).unwrap();

        assert_eq!(decoded.sequence, 42);
        assert_eq!(decoded.opus_data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_decode_invalid_short() {
        let data = vec![0, 0, 0, 1]; // Only 4 bytes
        assert!(decode_packet(&data).is_err());
    }

    #[test]
    fn test_decode_length_mismatch() {
        let data = vec![0, 0, 0, 1, 0, 10, 1, 2]; // Says 10 bytes, has 2
        assert!(decode_packet(&data).is_err());
    }
}
