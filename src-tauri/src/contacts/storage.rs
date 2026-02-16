use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::contacts::Contact;
use crate::error::ContactError;

/// Set a petname for a peer. Creates contact if it doesn't exist.
pub fn set_petname(app: &AppHandle, public_key: &str, petname: &str) -> Result<(), ContactError> {
    let store = app
        .store("contacts.json")
        .map_err(|e| ContactError::StorageError(format!("Failed to access store: {}", e)))?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Load existing contact or create new
    let contact = if let Some(value) = store.get(public_key) {
        let mut existing: Contact = serde_json::from_value(value.clone())
            .map_err(|e| ContactError::StorageError(format!("Failed to deserialize contact: {}", e)))?;
        existing.petname = Some(petname.to_string());
        existing
    } else {
        Contact {
            public_key_hex: public_key.to_string(),
            petname: Some(petname.to_string()),
            notes: None,
            added_at: now,
        }
    };

    store.set(public_key.to_string(), serde_json::to_value(&contact).unwrap());

    store
        .save()
        .map_err(|e| ContactError::StorageError(format!("Failed to save store: {}", e)))?;

    Ok(())
}

/// Remove a petname for a peer. If contact has no petname and no notes, remove entry entirely.
pub fn remove_petname(app: &AppHandle, public_key: &str) -> Result<(), ContactError> {
    let store = app
        .store("contacts.json")
        .map_err(|e| ContactError::StorageError(format!("Failed to access store: {}", e)))?;

    if let Some(value) = store.get(public_key) {
        let mut contact: Contact = serde_json::from_value(value.clone())
            .map_err(|e| ContactError::StorageError(format!("Failed to deserialize contact: {}", e)))?;

        contact.petname = None;

        // If contact has no petname and no notes, remove entirely
        if contact.notes.is_none() {
            store.delete(public_key);
        } else {
            store.set(public_key.to_string(), serde_json::to_value(&contact).unwrap());
        }

        store
            .save()
            .map_err(|e| ContactError::StorageError(format!("Failed to save store: {}", e)))?;
    }

    Ok(())
}

/// Get all contacts from the store
pub fn get_contacts(app: &AppHandle) -> Result<Vec<Contact>, ContactError> {
    let store = app
        .store("contacts.json")
        .map_err(|e| ContactError::StorageError(format!("Failed to access store: {}", e)))?;

    let mut contacts = Vec::new();

    for (_, value) in store.entries() {
        let contact: Contact = serde_json::from_value(value.clone())
            .map_err(|e| ContactError::StorageError(format!("Failed to deserialize contact: {}", e)))?;
        contacts.push(contact);
    }

    Ok(contacts)
}

/// Get a single contact by public key
pub fn get_contact(app: &AppHandle, public_key: &str) -> Result<Option<Contact>, ContactError> {
    let store = app
        .store("contacts.json")
        .map_err(|e| ContactError::StorageError(format!("Failed to access store: {}", e)))?;

    if let Some(value) = store.get(public_key) {
        let contact: Contact = serde_json::from_value(value.clone())
            .map_err(|e| ContactError::StorageError(format!("Failed to deserialize contact: {}", e)))?;
        Ok(Some(contact))
    } else {
        Ok(None)
    }
}
