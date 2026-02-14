use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};

use crate::error::SwarmError;

/// SwarmKey wraps a 32-byte pre-shared key for swarm isolation
#[derive(Clone)]
pub struct SwarmKey([u8; 32]);

impl SwarmKey {
    /// Generate a new random swarm key using cryptographically secure RNG
    pub fn generate() -> Self {
        let mut key = [0u8; 32];
        OsRng.fill_bytes(&mut key);
        Self(key)
    }

    /// Create SwarmKey from raw bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Convert to libp2p PreSharedKey
    pub fn to_psk(&self) -> libp2p::pnet::PreSharedKey {
        libp2p::pnet::PreSharedKey::new(self.0)
    }

    /// Get reference to raw bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Generate deterministic swarm ID from key hash
    pub fn swarm_id(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.0);
        let result = hasher.finalize();
        hex::encode(result)
    }
}
