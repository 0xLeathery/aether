use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::error::SwarmError;

/// Channel metadata
#[derive(Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
}

/// Swarm metadata stored in Tauri Store
#[derive(Serialize, Deserialize, Clone)]
pub struct SwarmMetadata {
    pub id: String,
    pub name: String,
    pub psk_hex: String, // Full aether:// URI
    pub created_at: i64, // Unix timestamp
    pub channels: Vec<Channel>,
    /// The swarm creator's Ed25519 public key (hex).
    /// Option for backward compatibility with existing swarms.
    /// The CRDT metadata document is the source of truth;
    /// this is a local cache for permission checks.
    #[serde(default)]
    pub creator_key: Option<String>,
}

/// Save swarm metadata to Tauri Store
pub fn save_swarm(app: &AppHandle, metadata: &SwarmMetadata) -> Result<(), SwarmError> {
    let store = app
        .store("swarms.json")
        .map_err(|e| SwarmError::StorageError(format!("Failed to access store: {}", e)))?;

    store
        .set(metadata.id.clone(), serde_json::to_value(metadata).unwrap());

    store
        .save()
        .map_err(|e| SwarmError::StorageError(format!("Failed to save store: {}", e)))?;

    Ok(())
}

/// List all swarms from Tauri Store
pub fn list_swarms(app: &AppHandle) -> Result<Vec<SwarmMetadata>, SwarmError> {
    let store = app
        .store("swarms.json")
        .map_err(|e| SwarmError::StorageError(format!("Failed to access store: {}", e)))?;

    let mut swarms = Vec::new();

    for (_, value) in store.entries() {
        let metadata: SwarmMetadata = serde_json::from_value(value.clone())
            .map_err(|e| SwarmError::StorageError(format!("Failed to deserialize swarm: {}", e)))?;
        swarms.push(metadata);
    }

    Ok(swarms)
}

/// Get a single swarm by ID
pub fn get_swarm(app: &AppHandle, swarm_id: &str) -> Result<SwarmMetadata, SwarmError> {
    let store = app
        .store("swarms.json")
        .map_err(|e| SwarmError::StorageError(format!("Failed to access store: {}", e)))?;

    let value = store
        .get(swarm_id)
        .ok_or_else(|| SwarmError::NotFound(swarm_id.to_string()))?;

    let metadata: SwarmMetadata = serde_json::from_value(value.clone())
        .map_err(|e| SwarmError::StorageError(format!("Failed to deserialize swarm: {}", e)))?;

    Ok(metadata)
}

/// Delete a swarm from the store
pub fn delete_swarm(app: &AppHandle, swarm_id: &str) -> Result<(), SwarmError> {
    let store = app
        .store("swarms.json")
        .map_err(|e| SwarmError::StorageError(format!("Failed to access store: {}", e)))?;

    store.delete(swarm_id);

    store
        .save()
        .map_err(|e| SwarmError::StorageError(format!("Failed to save store: {}", e)))?;

    Ok(())
}

/// Check if swarm exists
pub fn has_swarm(app: &AppHandle, swarm_id: &str) -> Result<bool, SwarmError> {
    let store = app
        .store("swarms.json")
        .map_err(|e| SwarmError::StorageError(format!("Failed to access store: {}", e)))?;

    Ok(store.has(swarm_id))
}
