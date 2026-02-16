use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize, Clone)]
pub struct ChannelReadState {
    pub total_seen: usize,
}

/// Save the total_seen count for a channel to persistent storage
#[tauri::command]
pub fn mark_channel_read(
    app: AppHandle,
    swarm_id: String,
    channel_id: String,
    total_seen: usize,
) -> Result<(), String> {
    let store = app
        .store("unread.json")
        .map_err(|e| format!("Failed to access unread store: {}", e))?;

    let key = format!("{}/{}", swarm_id, channel_id);
    let state = ChannelReadState { total_seen };

    store.set(
        key,
        serde_json::to_value(&state).map_err(|e| format!("Failed to serialize state: {}", e))?,
    );

    store
        .save()
        .map_err(|e| format!("Failed to save unread store: {}", e))?;

    Ok(())
}

/// Get all persisted unread state entries
#[tauri::command]
pub fn get_unread_state(app: AppHandle) -> Result<HashMap<String, ChannelReadState>, String> {
    let store = app
        .store("unread.json")
        .map_err(|e| format!("Failed to access unread store: {}", e))?;

    let mut result = HashMap::new();

    for (key, value) in store.entries() {
        let state: ChannelReadState = serde_json::from_value(value.clone())
            .map_err(|e| format!("Failed to deserialize state for {}: {}", key, e))?;
        result.insert(key.clone(), state);
    }

    Ok(result)
}
