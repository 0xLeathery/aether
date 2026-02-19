use std::collections::HashMap;
use tauri::State;
use tokio::sync::Mutex;

use crate::error::ModerationError;
use crate::moderation::{ModerationEntry, ModerationTier};
use crate::moderation::storage as mod_storage;
use crate::voice::VoiceSession;

/// Get all moderation entries
#[tauri::command]
pub async fn get_moderation_state(
    app: tauri::AppHandle,
) -> Result<HashMap<String, ModerationEntry>, ModerationError> {
    mod_storage::get_moderation_state(&app)
}

/// Set or update moderation for a peer
#[tauri::command]
pub async fn set_moderation(
    app: tauri::AppHandle,
    public_key: String,
    tier: ModerationTier,
    swarm_overrides: Option<HashMap<String, Option<ModerationTier>>>,
) -> Result<(), ModerationError> {
    let entry = ModerationEntry {
        tier,
        swarm_overrides: swarm_overrides.unwrap_or_default(),
    };
    
    mod_storage::set_moderation(&app, &public_key, &entry)
}

/// Remove moderation for a peer
#[tauri::command]
pub async fn remove_moderation(
    app: tauri::AppHandle,
    public_key: String,
) -> Result<(), ModerationError> {
    mod_storage::remove_moderation(&app, &public_key)
}

/// Mute a peer's voice - prevents their audio from being mixed
#[tauri::command]
pub async fn mute_peer_voice(
    voice_session: State<'_, Mutex<VoiceSession>>,
    peer_key_hex: String,
) -> Result<(), String> {
    let session = voice_session.lock().await;
    session.mute_peer(peer_key_hex).await
}

/// Unmute a peer's voice - allows their audio to be mixed
#[tauri::command]
pub async fn unmute_peer_voice(
    voice_session: State<'_, Mutex<VoiceSession>>,
    peer_key_hex: String,
) -> Result<(), String> {
    let session = voice_session.lock().await;
    session.unmute_peer(peer_key_hex).await
}
