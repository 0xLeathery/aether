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
