use keyring::Entry;

use crate::error::IdentityError;

const SERVICE_NAME: &str = "com.aether.identity";
const DISPLAY_NAME_USERNAME: &str = "display_name";

/// Store display name in keychain
pub fn store_display_name(name: &str) -> Result<(), IdentityError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(IdentityError::DisplayNameRequired);
    }

    let entry = Entry::new(SERVICE_NAME, DISPLAY_NAME_USERNAME).map_err(|e| {
        IdentityError::KeychainDenied(format!("Failed to access keychain: {}", e))
    })?;

    entry.set_password(trimmed).map_err(|e| {
        IdentityError::KeychainDenied(format!("Failed to store display name: {}", e))
    })?;

    Ok(())
}

/// Load display name from keychain
pub fn load_display_name() -> Result<String, IdentityError> {
    let entry = Entry::new(SERVICE_NAME, DISPLAY_NAME_USERNAME).map_err(|e| {
        IdentityError::KeychainDenied(format!("Failed to access keychain: {}", e))
    })?;

    entry.get_password().map_err(|e| match e {
        keyring::Error::NoEntry => IdentityError::NotFound,
        other => IdentityError::KeychainDenied(format!("Failed to load display name: {}", other)),
    })
}
