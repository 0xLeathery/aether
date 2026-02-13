use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;

use crate::error::IdentityError;
use crate::identity::storage;

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

/// Convert ed25519-dalek SigningKey to libp2p Keypair
///
/// This ensures PeerId is deterministically derived from the existing keychain identity.
/// NEVER use libp2p's with_new_identity() - always derive from our Ed25519 key.
pub fn to_libp2p_keypair(signing_key: &SigningKey) -> Result<libp2p::identity::Keypair, IdentityError> {
    // Extract the 32-byte secret scalar
    let secret_bytes = signing_key.to_bytes();

    // Create libp2p SecretKey from the 32-byte secret (not the 64-byte concatenated format)
    let libp2p_secret = libp2p::identity::ed25519::SecretKey::try_from_bytes(secret_bytes)
        .map_err(|e| IdentityError::KeyGeneration(format!("libp2p key conversion failed: {}", e)))?;

    // Generate the keypair from the secret key (libp2p will derive the public key)
    let libp2p_ed25519 = libp2p::identity::ed25519::Keypair::from(libp2p_secret);

    // Wrap in libp2p::identity::Keypair
    Ok(libp2p::identity::Keypair::from(libp2p_ed25519))
}

/// Load libp2p keypair from keychain storage
///
/// Convenience function that loads the secret key from keychain and converts to libp2p format.
pub fn load_libp2p_keypair() -> Result<libp2p::identity::Keypair, IdentityError> {
    let secret_key_bytes = storage::load_secret_key()?;
    let signing_key = signing_key_from_bytes(&secret_key_bytes)?;
    to_libp2p_keypair(&signing_key)
}
