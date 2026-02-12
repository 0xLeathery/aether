use crate::error::IdentityError;
use crate::identity::{display, keypair, storage};

#[derive(serde::Serialize)]
pub struct IdentityInfo {
    pub public_key_hex: String,
    pub short_id: String,
    pub display_name: String,
}

/// Check if identity exists in keychain
#[tauri::command]
pub async fn has_identity() -> Result<bool, IdentityError> {
    Ok(storage::has_secret_key())
}

/// Create new identity with Ed25519 keypair and display name
#[tauri::command]
pub async fn create_identity(display_name: String) -> Result<IdentityInfo, IdentityError> {
    // Check if identity already exists
    if storage::has_secret_key() {
        return Err(IdentityError::AlreadyExists);
    }

    // Validate display name
    if display_name.trim().is_empty() {
        return Err(IdentityError::DisplayNameRequired);
    }

    // Generate Ed25519 keypair
    let (signing_key, verifying_key) = keypair::generate_keypair();

    // Store secret key in keychain
    let secret_bytes = keypair::signing_key_to_bytes(&signing_key);
    storage::store_secret_key(&secret_bytes)?;

    // Store display name in keychain
    display::store_display_name(&display_name)?;

    // Return identity info
    Ok(IdentityInfo {
        public_key_hex: keypair::public_key_to_hex(&verifying_key),
        short_id: keypair::public_key_short_id(&verifying_key),
        display_name: display_name.trim().to_string(),
    })
}

/// Load existing identity from keychain
#[tauri::command]
pub async fn get_identity() -> Result<IdentityInfo, IdentityError> {
    // Load secret key from keychain
    let secret_bytes = storage::load_secret_key()?;
    let signing_key = keypair::signing_key_from_bytes(&secret_bytes)?;
    let verifying_key = signing_key.verifying_key();

    // Load display name
    let display_name = display::load_display_name()?;

    Ok(IdentityInfo {
        public_key_hex: keypair::public_key_to_hex(&verifying_key),
        short_id: keypair::public_key_short_id(&verifying_key),
        display_name,
    })
}

/// Update display name in keychain
#[tauri::command]
pub async fn update_display_name(new_name: String) -> Result<(), IdentityError> {
    if new_name.trim().is_empty() {
        return Err(IdentityError::DisplayNameRequired);
    }

    display::store_display_name(&new_name)
}
