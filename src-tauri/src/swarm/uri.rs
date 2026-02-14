use crate::error::SwarmError;
use super::key::SwarmKey;

/// Encode a SwarmKey as an aether:// URI
pub fn encode_secret_code(key: &SwarmKey) -> String {
    let hex = hex::encode(key.as_bytes());
    format!("aether://{}", hex)
}

/// Decode an aether:// URI to a SwarmKey
pub fn decode_secret_code(uri: &str) -> Result<SwarmKey, SwarmError> {
    // Case-insensitive scheme check
    let lower = uri.to_lowercase();
    if !lower.starts_with("aether://") {
        return Err(SwarmError::InvalidUri(
            "URI must start with aether://".to_string(),
        ));
    }

    // Extract hex portion (after scheme)
    let hex_str = &uri[9..]; // Skip "aether://"

    // Validate hex length (64 chars = 32 bytes)
    if hex_str.len() != 64 {
        return Err(SwarmError::InvalidUri(format!(
            "Expected 64 hex characters, got {}",
            hex_str.len()
        )));
    }

    // Decode hex
    let bytes = hex::decode(hex_str)
        .map_err(|e| SwarmError::InvalidHex(e.to_string()))?;

    // Verify decoded length
    if bytes.len() != 32 {
        return Err(SwarmError::InvalidKeyLength);
    }

    // Convert to fixed-size array
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&bytes);

    Ok(SwarmKey::from_bytes(key_bytes))
}
