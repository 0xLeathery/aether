use base64::Engine;
use keyring::Entry;

use super::keychain_acl;
use crate::error::IdentityError;

const SERVICE_NAME: &str = "com.aether.identity";
const SECRET_KEY_USERNAME: &str = "secret_key";

/// Store secret key bytes in platform keychain
pub fn store_secret_key(secret_bytes: &[u8]) -> Result<(), IdentityError> {
    let entry = Entry::new(SERVICE_NAME, SECRET_KEY_USERNAME).map_err(|e| {
        IdentityError::KeychainDenied(format!("Failed to access keychain: {}", e))
    })?;

    let encoded = base64::engine::general_purpose::STANDARD.encode(secret_bytes);
    entry.set_password(&encoded).map_err(|e| {
        IdentityError::KeychainDenied(format!("Failed to store key: {}", e))
    })?;

    keychain_acl::add_app_to_keychain_acl(SERVICE_NAME, SECRET_KEY_USERNAME);

    Ok(())
}

/// Load secret key from platform keychain
pub fn load_secret_key() -> Result<Vec<u8>, IdentityError> {
    let entry = Entry::new(SERVICE_NAME, SECRET_KEY_USERNAME).map_err(|e| {
        IdentityError::KeychainDenied(format!("Failed to access keychain: {}", e))
    })?;

    let encoded = entry.get_password().map_err(|e| match e {
        keyring::Error::NoEntry => IdentityError::NotFound,
        keyring::Error::Ambiguous(_) => {
            IdentityError::KeychainDenied("Multiple keychain entries found".to_string())
        }
        other => IdentityError::KeychainDenied(format!("Keychain error: {}", other)),
    })?;

    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| {
            IdentityError::KeychainDenied(format!("Failed to decode stored key: {}", e))
        })?;

    Ok(decoded)
}

/// Check if secret key exists in keychain
pub fn has_secret_key() -> bool {
    match Entry::new(SERVICE_NAME, SECRET_KEY_USERNAME) {
        Ok(entry) => entry.get_password().is_ok(),
        Err(_) => false,
    }
}

/// Delete secret key from keychain
pub fn delete_secret_key() -> Result<(), IdentityError> {
    let entry = Entry::new(SERVICE_NAME, SECRET_KEY_USERNAME).map_err(|e| {
        IdentityError::KeychainDenied(format!("Failed to access keychain: {}", e))
    })?;

    entry.delete_credential().map_err(|e| match e {
        keyring::Error::NoEntry => IdentityError::NotFound,
        other => IdentityError::KeychainDenied(format!("Failed to delete key: {}", other)),
    })?;

    Ok(())
}
