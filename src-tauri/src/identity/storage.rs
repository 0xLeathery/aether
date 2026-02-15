use base64::Engine;

#[cfg(not(target_os = "macos"))]
use keyring::Entry;

#[cfg(target_os = "macos")]
use security_framework::{
    access_control::{ProtectionMode, SecAccessControl},
    passwords::{self, PasswordOptions},
};

use crate::error::IdentityError;

const SERVICE_NAME: &str = "com.aether.identity";
const SECRET_KEY_USERNAME: &str = "secret_key";

/// Store secret key bytes in platform keychain
#[cfg(target_os = "macos")]
pub fn store_secret_key(secret_bytes: &[u8]) -> Result<(), IdentityError> {
    let encoded = base64::engine::general_purpose::STANDARD.encode(secret_bytes);

    // Create access control that allows access when device is unlocked, without password prompts
    let access_control = SecAccessControl::create_with_protection(
        Some(ProtectionMode::AccessibleWhenUnlocked),
        0, // No additional flags
    )
    .map_err(|e| {
        IdentityError::KeychainDenied(format!("Failed to create access control: {}", e))
    })?;

    // Create password options with access control
    let mut options = PasswordOptions::new_generic_password(SERVICE_NAME, SECRET_KEY_USERNAME);
    options.set_access_control(access_control);

    // Store the password with the configured options
    passwords::set_generic_password_options(encoded.as_bytes(), options).map_err(|e| {
        IdentityError::KeychainDenied(format!("Failed to store key: {}", e))
    })?;

    Ok(())
}

/// Store secret key bytes in platform keychain
#[cfg(not(target_os = "macos"))]
pub fn store_secret_key(secret_bytes: &[u8]) -> Result<(), IdentityError> {
    let entry = Entry::new(SERVICE_NAME, SECRET_KEY_USERNAME).map_err(|e| {
        IdentityError::KeychainDenied(format!("Failed to access keychain: {}", e))
    })?;

    let encoded = base64::engine::general_purpose::STANDARD.encode(secret_bytes);
    entry.set_password(&encoded).map_err(|e| {
        IdentityError::KeychainDenied(format!("Failed to store key: {}", e))
    })?;

    Ok(())
}

/// Load secret key from platform keychain
#[cfg(target_os = "macos")]
pub fn load_secret_key() -> Result<Vec<u8>, IdentityError> {
    let options = PasswordOptions::new_generic_password(SERVICE_NAME, SECRET_KEY_USERNAME);

    let encoded = passwords::generic_password(options).map_err(|e| {
        // Check if it's a "not found" error
        if e.to_string().contains("NoSuchKeychain")
            || e.to_string().contains("ItemNotFound")
            || e.to_string().contains("-25300")
        {
            IdentityError::NotFound
        } else {
            IdentityError::KeychainDenied(format!("Keychain error: {}", e))
        }
    })?;

    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| {
            IdentityError::KeychainDenied(format!("Failed to decode stored key: {}", e))
        })?;

    Ok(decoded)
}

/// Load secret key from platform keychain
#[cfg(not(target_os = "macos"))]
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
#[cfg(target_os = "macos")]
pub fn has_secret_key() -> bool {
    let options = PasswordOptions::new_generic_password(SERVICE_NAME, SECRET_KEY_USERNAME);
    passwords::generic_password(options).is_ok()
}

/// Check if secret key exists in keychain
#[cfg(not(target_os = "macos"))]
pub fn has_secret_key() -> bool {
    match Entry::new(SERVICE_NAME, SECRET_KEY_USERNAME) {
        Ok(entry) => entry.get_password().is_ok(),
        Err(_) => false,
    }
}

/// Delete secret key from keychain
#[cfg(target_os = "macos")]
pub fn delete_secret_key() -> Result<(), IdentityError> {
    passwords::delete_generic_password(SERVICE_NAME, SECRET_KEY_USERNAME).map_err(|e| {
        if e.to_string().contains("NoSuchKeychain")
            || e.to_string().contains("ItemNotFound")
            || e.to_string().contains("-25300")
        {
            IdentityError::NotFound
        } else {
            IdentityError::KeychainDenied(format!("Failed to delete key: {}", e))
        }
    })?;

    Ok(())
}

/// Delete secret key from keychain
#[cfg(not(target_os = "macos"))]
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
