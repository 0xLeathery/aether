use std::collections::HashMap;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::error::ModerationError;
use crate::moderation::{ModerationEntry, ModerationTier};

/// Get all moderation entries from the store
pub fn get_moderation_state(
    app: &AppHandle,
) -> Result<HashMap<String, ModerationEntry>, ModerationError> {
    let store = app
        .store("moderation.json")
        .map_err(|e| ModerationError::StorageError(format!("Failed to access store: {}", e)))?;

    let mut entries = HashMap::new();

    for (key, value) in store.entries() {
        let entry: ModerationEntry = serde_json::from_value(value.clone()).map_err(|e| {
            ModerationError::StorageError(format!("Failed to deserialize moderation entry: {}", e))
        })?;
        entries.insert(key.clone(), entry);
    }

    Ok(entries)
}

/// Set or update a moderation entry for a peer
pub fn set_moderation(
    app: &AppHandle,
    public_key: &str,
    entry: &ModerationEntry,
) -> Result<(), ModerationError> {
    let store = app
        .store("moderation.json")
        .map_err(|e| ModerationError::StorageError(format!("Failed to access store: {}", e)))?;

    store.set(public_key.to_string(), serde_json::to_value(entry).unwrap());

    store
        .save()
        .map_err(|e| ModerationError::StorageError(format!("Failed to save store: {}", e)))?;

    Ok(())
}

/// Remove a moderation entry for a peer
pub fn remove_moderation(app: &AppHandle, public_key: &str) -> Result<(), ModerationError> {
    let store = app
        .store("moderation.json")
        .map_err(|e| ModerationError::StorageError(format!("Failed to access store: {}", e)))?;

    store.delete(public_key);

    store
        .save()
        .map_err(|e| ModerationError::StorageError(format!("Failed to save store: {}", e)))?;

    Ok(())
}

/// Get the effective moderation tier for a peer, considering swarm overrides
pub fn get_effective_tier(
    app: &AppHandle,
    public_key: &str,
    swarm_id: Option<&str>,
) -> Result<Option<ModerationTier>, ModerationError> {
    let entries = get_moderation_state(app)?;

    if let Some(entry) = entries.get(public_key) {
        // Check for swarm-specific override
        if let Some(swarm) = swarm_id {
            if let Some(override_tier) = entry.swarm_overrides.get(swarm) {
                // Override exists - return it (may be null meaning no moderation in this swarm)
                return Ok(override_tier.clone());
            }
        }
        // No override, return global tier
        Ok(Some(entry.tier.clone()))
    } else {
        Ok(None)
    }
}
