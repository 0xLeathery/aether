use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("Identity already exists")]
    AlreadyExists,
    #[error("Identity not found in keychain")]
    NotFound,
    #[error("Keychain access denied: {0}")]
    KeychainDenied(String),
    #[error("Key generation failed: {0}")]
    KeyGeneration(String),
    #[error("Display name is required")]
    DisplayNameRequired,
    #[error(transparent)]
    Keyring(#[from] keyring::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl serde::Serialize for IdentityError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Failed to convert ed25519 key: {0}")]
    Libp2pKeyConversion(String),
    #[error("Failed to create transport: {0}")]
    TransportInit(String),
    #[error("Failed to start swarm: {0}")]
    SwarmStart(String),
    #[error("Failed to listen on address: {0}")]
    ListenFailed(String),
    #[error("Network service not running")]
    NotRunning,
    #[error("Network service already running")]
    AlreadyRunning,
    #[error(transparent)]
    Identity(#[from] IdentityError),
}

impl serde::Serialize for NetworkError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Error)]
pub enum SwarmError {
    #[error("Invalid aether:// URI: {0}")]
    InvalidUri(String),
    #[error("Invalid key length (expected 32 bytes)")]
    InvalidKeyLength,
    #[error("Invalid hex encoding: {0}")]
    InvalidHex(String),
    #[error("Already joined this swarm")]
    AlreadyJoined,
    #[error("Swarm not found: {0}")]
    NotFound(String),
    #[error("Storage error: {0}")]
    StorageError(String),
}

impl serde::Serialize for SwarmError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Error)]
pub enum VoiceError {
    #[error("Audio device not found: {0}")]
    AudioDeviceNotFound(String),
    #[error("Audio stream failed: {0}")]
    AudioStreamFailed(String),
    #[error("Codec error: {0}")]
    CodecError(String),
    #[error("Voice session is full (maximum 8 participants)")]
    SessionFull,
    #[error("Not currently in a voice session")]
    NotInSession,
    #[error("Already in a voice session")]
    AlreadyInSession,
}

impl serde::Serialize for VoiceError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Error)]
pub enum ContactError {
    #[error("Contact not found: {0}")]
    NotFound(String),
    #[error("Storage error: {0}")]
    StorageError(String),
}

impl serde::Serialize for ContactError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Debug, Error)]
pub enum ChatError {
    #[error("Document not found: {0}")]
    DocumentNotFound(String),
    #[error("Document corrupted: {0}")]
    DocumentCorrupted(String),
    #[error("Sync failed: {0}")]
    SyncFailed(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Message error: {0}")]
    MessageError(String),
}

impl serde::Serialize for ChatError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
