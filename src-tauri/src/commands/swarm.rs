use tauri::{AppHandle, State};
use std::sync::Mutex;

use crate::error::SwarmError;
use crate::network::NetworkService;
use crate::swarm::{self, Channel, SwarmKey, SwarmMetadata};

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

    // Create metadata with General channel
    let metadata = SwarmMetadata {
        id: swarm_id,
        name,
        psk_hex: uri.clone(),
        created_at,
        channels: vec![Channel {
            id: "general".to_string(),
            name: "General".to_string(),
        }],
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

    // Create metadata with General channel
    let metadata = SwarmMetadata {
        id: swarm_id.clone(),
        name,
        psk_hex: uri,
        created_at,
        channels: vec![Channel {
            id: "general".to_string(),
            name: "General".to_string(),
        }],
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
