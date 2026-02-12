use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

use crate::error::IdentityError;

/// Generate a new Ed25519 keypair using OS random source
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

/// Serialize signing key to bytes for storage (32 bytes secret key)
pub fn signing_key_to_bytes(key: &SigningKey) -> Vec<u8> {
    key.to_bytes().to_vec()
}

/// Deserialize signing key from stored bytes
pub fn signing_key_from_bytes(bytes: &[u8]) -> Result<SigningKey, IdentityError> {
    if bytes.len() != 32 {
        return Err(IdentityError::KeyGeneration(format!(
            "Invalid key length: expected 32 bytes, got {}",
            bytes.len()
        )));
    }

    let key_bytes: [u8; 32] = bytes.try_into().map_err(|_| {
        IdentityError::KeyGeneration("Failed to convert bytes to key".to_string())
    })?;

    Ok(SigningKey::from_bytes(&key_bytes))
}

/// Convert public key to hex string (identity fingerprint)
pub fn public_key_to_hex(key: &VerifyingKey) -> String {
    hex::encode(key.to_bytes())
}

/// Generate short ID from public key (first 16 hex chars in xxxx:xxxx:xxxx:xxxx format)
pub fn public_key_short_id(key: &VerifyingKey) -> String {
    let hex = public_key_to_hex(key);
    let short = &hex[..16];
    format!(
        "{}:{}:{}:{}",
        &short[0..4],
        &short[4..8],
        &short[8..12],
        &short[12..16]
    )
}
