use std::sync::Arc;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};

use crate::chat::ChatService;
use crate::error::SwarmError;
use crate::identity::{keypair, storage};
use crate::network::NetworkService;
use crate::swarm::{self, Channel, SwarmKey, SwarmMetadata};
use crate::voice::VoiceSession;

/// Create a new swarm with generated PSK
#[tauri::command]
pub fn create_swarm(app: AppHandle, name: String) -> Result<String, SwarmError> {
    // Generate new swarm key
    let key = SwarmKey::generate();
    let swarm_id = key.swarm_id();
    let uri = swarm::uri::encode_secret_code(&key);

    // Get current timestamp
    let created_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Load identity to get creator public key
    let secret_bytes = storage::load_secret_key()
        .map_err(|e| SwarmError::StorageError(format!("Identity error: {}", e)))?;
    let signing_key = keypair::signing_key_from_bytes(&secret_bytes)
        .map_err(|e| SwarmError::StorageError(format!("Key error: {}", e)))?;
    let public_key_hex = keypair::public_key_to_hex(&signing_key.verifying_key());

    // Create metadata CRDT document with default channels (general + voice)
    let mut meta_doc = swarm::SwarmMetadataDocument::new(&public_key_hex)?;
    swarm::metadata_storage::save_metadata_doc(&app, &swarm_id, &mut meta_doc)?;

    // Create local metadata with both default channels and creator_key
    let metadata = SwarmMetadata {
        id: swarm_id,
        name,
        psk_hex: uri.clone(),
        created_at,
        channels: vec![
            Channel {
                id: "general".to_string(),
                name: "general".to_string(),
            },
            Channel {
                id: "voice".to_string(),
                name: "voice".to_string(),
            },
        ],
        creator_key: Some(public_key_hex),
    };

    // Save to store
    swarm::storage::save_swarm(&app, &metadata)?;

    // Return the aether:// URI for sharing
    Ok(uri)
}

/// Join an existing swarm via aether:// URI
#[tauri::command]
pub fn join_swarm(app: AppHandle, uri: String, name: String) -> Result<String, SwarmError> {
    // Decode URI
    let key = swarm::uri::decode_secret_code(&uri)?;
    let swarm_id = key.swarm_id();

    // Check if already joined
    if swarm::storage::has_swarm(&app, &swarm_id)? {
        return Err(SwarmError::AlreadyJoined);
    }

    // Get current timestamp
    let created_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Create metadata with default channels (general + voice)
    // creator_key is None — populated from CRDT metadata sync on first peer connection
    let metadata = SwarmMetadata {
        id: swarm_id.clone(),
        name,
        psk_hex: uri,
        created_at,
        channels: vec![
            Channel {
                id: "general".to_string(),
                name: "general".to_string(),
            },
            Channel {
                id: "voice".to_string(),
                name: "voice".to_string(),
            },
        ],
        creator_key: None,
    };

    // Save to store
    swarm::storage::save_swarm(&app, &metadata)?;

    // Return swarm ID
    Ok(swarm_id)
}

/// List all joined swarms
#[tauri::command]
pub fn list_swarms(app: AppHandle) -> Result<Vec<SwarmMetadata>, SwarmError> {
    swarm::storage::list_swarms(&app)
}

/// Switch to a different swarm (restarts network with new PSK)
#[tauri::command]
pub fn switch_swarm(
    app: AppHandle,
    network: State<Mutex<NetworkService>>,
    swarm_id: String,
) -> Result<(), SwarmError> {
    // Load swarm metadata
    let metadata = swarm::storage::get_swarm(&app, &swarm_id)?;

    // Decode PSK from stored URI
    let key = swarm::uri::decode_secret_code(&metadata.psk_hex)?;
    let psk = key.to_psk();

    // Lock network service
    let mut network_service = network.lock().unwrap();

    // Stop current swarm
    network_service.stop();

    // Start with new PSK
    network_service
        .start_with_psk(app.clone(), psk)
        .map_err(|e| SwarmError::StorageError(format!("Failed to start network: {}", e)))?;

    Ok(())
}

/// Rename a swarm locally
#[tauri::command]
pub fn rename_swarm(app: AppHandle, swarm_id: String, new_name: String) -> Result<(), String> {
    let mut metadata =
        swarm::storage::get_swarm(&app, &swarm_id).map_err(|e| e.to_string())?;
    metadata.name = new_name;
    swarm::storage::save_swarm(&app, &metadata).map_err(|e| e.to_string())?;
    Ok(())
}

/// Leave a swarm with full ordered cleanup
///
/// Cleanup order: voice -> network -> chat docs -> disk files -> store -> event
#[tauri::command]
pub async fn leave_swarm(
    app: AppHandle,
    network: State<'_, Mutex<NetworkService>>,
    voice_session: State<'_, tokio::sync::Mutex<VoiceSession>>,
    chat_service: State<'_, Arc<tokio::sync::Mutex<ChatService>>>,
    swarm_id: String,
) -> Result<(), String> {
    // Step 1: Leave voice session if active
    {
        let mut session = voice_session.lock().await;
        if session.is_in_session() {
            session.leave(app.clone()).await;
        }
    }

    // Step 2: Stop network service (closes all streams, terminates sync tasks)
    // std::sync::Mutex dropped before any .await
    {
        let mut network_service = network.lock().unwrap();
        network_service.stop();
    }

    // Step 3: Clear in-memory chat documents for this swarm
    {
        let mut service = chat_service.lock().await;
        service.remove_swarm_documents(&swarm_id);
    }

    // Step 4: Delete Automerge files from disk (chat documents)
    if let Ok(data_dir) = app.path().app_data_dir() {
        let swarm_chat_dir = data_dir.join("chat").join(&swarm_id);
        if swarm_chat_dir.exists() {
            let _ = std::fs::remove_dir_all(&swarm_chat_dir);
        }
    }

    // Step 4b: Delete metadata CRDT document from disk
    let _ = swarm::metadata_storage::delete_metadata_doc(&app, &swarm_id);

    // Step 5: Remove swarm metadata from store
    swarm::storage::delete_swarm(&app, &swarm_id).map_err(|e| e.to_string())?;

    // Step 6: Emit deletion event to frontend
    let _ = app.emit("swarm-deleted", &swarm_id);

    Ok(())
}

/// Get the invite URI for a swarm
#[tauri::command]
pub fn get_invite_uri(app: AppHandle, swarm_id: String) -> Result<String, String> {
    let metadata =
        swarm::storage::get_swarm(&app, &swarm_id).map_err(|e| e.to_string())?;
    // psk_hex already contains the full aether:// URI
    Ok(metadata.psk_hex.clone())
}
