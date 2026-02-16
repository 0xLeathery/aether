use std::sync::Arc;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::chat::ChatService;
use crate::identity::{keypair, storage};
use crate::swarm::{self, ChannelMeta, SwarmMetadataDocument};

/// Serializable channel info returned to the frontend
#[derive(Serialize, Clone)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
}

/// Verify that the local user is the swarm creator
///
/// Loads the local identity from the keychain and compares against
/// the creator_key stored in the swarm's metadata CRDT document.
fn verify_creator(app: &AppHandle, swarm_id: &str) -> Result<(), String> {
    let secret_bytes =
        storage::load_secret_key().map_err(|e| format!("Identity error: {}", e))?;
    let signing_key =
        keypair::signing_key_from_bytes(&secret_bytes).map_err(|e| format!("Key error: {}", e))?;
    let local_key = keypair::public_key_to_hex(&signing_key.verifying_key());

    let doc = swarm::metadata_storage::load_metadata_doc(app, swarm_id)
        .map_err(|e| format!("Metadata error: {}", e))?
        .ok_or_else(|| "Swarm metadata not found".to_string())?;

    let creator_key = doc
        .get_creator_key()
        .map_err(|e| format!("Metadata error: {}", e))?;

    if creator_key != local_key {
        return Err("Only the swarm creator can manage channels".to_string());
    }

    Ok(())
}

/// Validate and normalize a channel name
///
/// Rules: lowercase, trim, replace spaces with hyphens, remove non-alphanumeric
/// (except hyphens), collapse consecutive hyphens, strip leading/trailing hyphens,
/// truncate to 32 chars. Rejects empty names and reserved names (general, voice).
fn validate_channel_name(name: &str) -> Result<String, String> {
    let normalized: String = name
        .to_lowercase()
        .trim()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect();

    // Collapse consecutive hyphens
    let mut result = String::new();
    let mut prev_hyphen = false;
    for ch in normalized.chars() {
        if ch == '-' {
            if !prev_hyphen {
                result.push(ch);
            }
            prev_hyphen = true;
        } else {
            result.push(ch);
            prev_hyphen = false;
        }
    }

    // Strip leading/trailing hyphens
    let result = result.trim_matches('-').to_string();

    // Truncate to 32 chars
    let result: String = result.chars().take(32).collect();

    if result.is_empty() {
        return Err("Channel name cannot be empty".to_string());
    }

    if result == "general" || result == "voice" {
        return Err(format!("'{}' is a reserved channel name", result));
    }

    Ok(result)
}

/// Get or create the metadata document for a swarm
///
/// Tries to load from disk; if not found, creates a new document
/// using the local identity as creator (backward compat for pre-Phase-7 swarms).
fn get_or_create_metadata_doc(
    app: &AppHandle,
    swarm_id: &str,
) -> Result<SwarmMetadataDocument, String> {
    if let Some(doc) = swarm::metadata_storage::load_metadata_doc(app, swarm_id)
        .map_err(|e| format!("Metadata error: {}", e))?
    {
        return Ok(doc);
    }

    // No metadata doc exists — create one
    // Try to get creator_key from local swarm store
    let creator_key = if let Ok(metadata) = swarm::storage::get_swarm(app, swarm_id) {
        metadata.creator_key
    } else {
        None
    };

    // If no creator_key cached, use local identity (backward compat)
    let creator_key = match creator_key {
        Some(key) => key,
        None => {
            let secret_bytes =
                storage::load_secret_key().map_err(|e| format!("Identity error: {}", e))?;
            let signing_key = keypair::signing_key_from_bytes(&secret_bytes)
                .map_err(|e| format!("Key error: {}", e))?;
            keypair::public_key_to_hex(&signing_key.verifying_key())
        }
    };

    let mut doc = SwarmMetadataDocument::new(&creator_key)
        .map_err(|e| format!("Failed to create metadata document: {}", e))?;

    swarm::metadata_storage::save_metadata_doc(app, swarm_id, &mut doc)
        .map_err(|e| format!("Failed to save metadata document: {}", e))?;

    Ok(doc)
}

/// Create a new channel in a swarm
///
/// Only the swarm creator can create channels. The channel name is
/// validated and normalized. Persists to CRDT metadata doc and local store.
#[tauri::command]
pub async fn create_channel(
    app: AppHandle,
    chat_service: State<'_, Arc<tokio::sync::Mutex<ChatService>>>,
    swarm_id: String,
    name: String,
) -> Result<ChannelInfo, String> {
    // Only allow the creator
    verify_creator(&app, &swarm_id)?;

    // Validate and normalize the name
    let normalized = validate_channel_name(&name)?;

    // Generate a unique channel ID
    let channel_id = uuid::Uuid::new_v4().to_string();

    // Load identity for created_by field
    let secret_bytes =
        storage::load_secret_key().map_err(|e| format!("Identity error: {}", e))?;
    let signing_key =
        keypair::signing_key_from_bytes(&secret_bytes).map_err(|e| format!("Key error: {}", e))?;
    let public_key = keypair::public_key_to_hex(&signing_key.verifying_key());

    // Add channel to CRDT metadata document
    let mut doc = get_or_create_metadata_doc(&app, &swarm_id)?;
    let meta = ChannelMeta {
        name: normalized.clone(),
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64,
        created_by: public_key,
    };
    doc.add_channel(&channel_id, meta)
        .map_err(|e| format!("Failed to add channel: {}", e))?;
    swarm::metadata_storage::save_metadata_doc(&app, &swarm_id, &mut doc)
        .map_err(|e| format!("Failed to save metadata: {}", e))?;

    // Update local SwarmMetadata channels cache
    if let Ok(mut swarm_meta) = swarm::storage::get_swarm(&app, &swarm_id) {
        swarm_meta.channels.push(swarm::Channel {
            id: channel_id.clone(),
            name: normalized.clone(),
        });
        let _ = swarm::storage::save_swarm(&app, &swarm_meta);
    }

    // Emit event to frontend
    let _ = app.emit(
        "channels-updated",
        serde_json::json!({ "swarm_id": swarm_id }),
    );

    // Suppress unused variable warning — chat_service reserved for future use
    let _ = chat_service;

    Ok(ChannelInfo {
        id: channel_id,
        name: normalized,
    })
}

/// Rename a channel in a swarm
///
/// Only the swarm creator can rename channels. Default channels
/// (general, voice) cannot be renamed.
#[tauri::command]
pub async fn rename_channel(
    app: AppHandle,
    swarm_id: String,
    channel_id: String,
    new_name: String,
) -> Result<(), String> {
    // Guard: default channels cannot be renamed
    if channel_id == "general" || channel_id == "voice" {
        return Err("Cannot rename default channels".to_string());
    }

    // Only allow the creator
    verify_creator(&app, &swarm_id)?;

    // Validate and normalize the new name
    let normalized = validate_channel_name(&new_name)?;

    // Rename in CRDT metadata document
    let mut doc = get_or_create_metadata_doc(&app, &swarm_id)?;
    doc.rename_channel(&channel_id, &normalized)
        .map_err(|e| format!("Failed to rename channel: {}", e))?;
    swarm::metadata_storage::save_metadata_doc(&app, &swarm_id, &mut doc)
        .map_err(|e| format!("Failed to save metadata: {}", e))?;

    // Update local SwarmMetadata channels cache
    if let Ok(mut swarm_meta) = swarm::storage::get_swarm(&app, &swarm_id) {
        for ch in &mut swarm_meta.channels {
            if ch.id == channel_id {
                ch.name = normalized.clone();
            }
        }
        let _ = swarm::storage::save_swarm(&app, &swarm_meta);
    }

    // Emit event to frontend
    let _ = app.emit(
        "channels-updated",
        serde_json::json!({ "swarm_id": swarm_id }),
    );

    Ok(())
}

/// Delete a channel from a swarm with full cleanup
///
/// Only the swarm creator can delete channels. Default channels
/// (general, voice) cannot be deleted. Cleanup includes:
/// 1. Remove from CRDT metadata document
/// 2. Delete message Automerge file from disk
/// 3. Evict from ChatService in-memory cache
/// 4. Update local SwarmMetadata channels
/// 5. Emit channel-deleted event
#[tauri::command]
pub async fn delete_channel(
    app: AppHandle,
    chat_service: State<'_, Arc<tokio::sync::Mutex<ChatService>>>,
    swarm_id: String,
    channel_id: String,
) -> Result<(), String> {
    // Guard: default channels cannot be deleted
    if channel_id == "general" || channel_id == "voice" {
        return Err("Cannot delete default channels".to_string());
    }

    // Only allow the creator
    verify_creator(&app, &swarm_id)?;

    // Step 1: Remove from CRDT metadata document
    let mut doc = get_or_create_metadata_doc(&app, &swarm_id)?;
    doc.remove_channel(&channel_id)
        .map_err(|e| format!("Failed to remove channel: {}", e))?;
    swarm::metadata_storage::save_metadata_doc(&app, &swarm_id, &mut doc)
        .map_err(|e| format!("Failed to save metadata: {}", e))?;

    // Step 2: Delete message Automerge file from disk
    if let Ok(data_dir) = app.path().app_data_dir() {
        let chat_file = data_dir
            .join("chat")
            .join(&swarm_id)
            .join(format!("{}.automerge", channel_id));
        if chat_file.exists() {
            let _ = std::fs::remove_file(&chat_file);
        }
    }

    // Step 3: Evict from ChatService in-memory cache
    {
        let mut service = chat_service.lock().await;
        service.remove_channel_document(&swarm_id, &channel_id);
    }

    // Step 4: Update local SwarmMetadata channels cache
    if let Ok(mut swarm_meta) = swarm::storage::get_swarm(&app, &swarm_id) {
        swarm_meta.channels.retain(|ch| ch.id != channel_id);
        let _ = swarm::storage::save_swarm(&app, &swarm_meta);
    }

    // Step 5: Emit channel-deleted event
    let _ = app.emit(
        "channel-deleted",
        serde_json::json!({ "swarm_id": swarm_id, "channel_id": channel_id }),
    );

    Ok(())
}

/// List all channels in a swarm
///
/// Reads from the CRDT metadata document if available,
/// falling back to the local SwarmMetadata channels cache.
#[tauri::command]
pub fn list_channels(app: AppHandle, swarm_id: String) -> Result<Vec<ChannelInfo>, String> {
    // Try metadata document first (source of truth)
    if let Ok(Some(doc)) = swarm::metadata_storage::load_metadata_doc(&app, &swarm_id) {
        if let Ok(channels) = doc.get_channels() {
            return Ok(channels
                .into_iter()
                .map(|(id, meta)| ChannelInfo {
                    id,
                    name: meta.name,
                })
                .collect());
        }
    }

    // Fall back to local SwarmMetadata
    let swarm_meta =
        swarm::storage::get_swarm(&app, &swarm_id).map_err(|e| format!("Swarm error: {}", e))?;

    Ok(swarm_meta
        .channels
        .into_iter()
        .map(|ch| ChannelInfo {
            id: ch.id,
            name: ch.name,
        })
        .collect())
}
