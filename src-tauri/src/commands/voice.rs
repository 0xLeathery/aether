use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use crate::error::VoiceError;
use crate::network::NetworkService;
use crate::voice::VoiceSession;

/// Voice session status
#[derive(Serialize)]
pub struct VoiceStatus {
    active: bool,
    muted: bool,
    participants: Vec<String>,
    participant_count: usize,
    max_participants: usize,
}

/// Join a voice session
///
/// Connects to all online peers in the current swarm for voice communication
#[tauri::command]
pub async fn join_voice(
    app: AppHandle,
    voice_session: State<'_, tokio::sync::Mutex<VoiceSession>>,
    network: State<'_, std::sync::Mutex<NetworkService>>,
) -> Result<VoiceStatus, String> {
    // Get stream control from network service
    let stream_control = {
        let network_service = network.lock().map_err(|e| format!("Lock error: {}", e))?;

        if !network_service.is_running() {
            return Err("Network service not running".to_string());
        }

        network_service
            .stream_control()
            .ok_or_else(|| "Stream control not available".to_string())?
    };

    // Get online peer IDs from network service
    let peer_ids = {
        let network_service = network.lock().map_err(|e| format!("Lock error: {}", e))?;
        let peers = network_service.get_peers();

        // Filter to only online peers and parse PeerIds
        peers
            .into_iter()
            .filter(|(_, status)| *status == crate::network::peer_state::PeerStatus::Online)
            .filter_map(|(peer_id_str, _)| peer_id_str.parse().ok())
            .collect::<Vec<_>>()
    };

    // Join voice session (tokio::sync::Mutex can be held across await)
    let mut session = voice_session.lock().await;

    session
        .join(app, stream_control, peer_ids)
        .await
        .map_err(|e| match e {
            VoiceError::AlreadyInSession => "Already in a voice session".to_string(),
            VoiceError::SessionFull => "Session is full (maximum 8 participants)".to_string(),
            VoiceError::AudioDeviceNotFound(msg) => format!("Audio device not found: {}", msg),
            VoiceError::AudioStreamFailed(msg) => format!("Audio stream failed: {}", msg),
            VoiceError::CodecError(msg) => format!("Codec error: {}", msg),
            _ => format!("Voice session error: {}", e),
        })?;

    // Return current status
    Ok(VoiceStatus {
        active: session.is_in_session(),
        muted: session.is_muted(),
        participant_count: session.participant_count(),
        participants: Vec::new(), // Will be populated from events
        max_participants: 8,
    })
}

/// Leave the voice session
#[tauri::command]
pub async fn leave_voice(
    app: AppHandle,
    voice_session: State<'_, tokio::sync::Mutex<VoiceSession>>,
) -> Result<(), String> {
    let mut session = voice_session.lock().await;
    session.leave(app).await;

    Ok(())
}

/// Get current voice session status
#[tauri::command]
pub async fn get_voice_status(
    voice_session: State<'_, tokio::sync::Mutex<VoiceSession>>,
) -> Result<VoiceStatus, String> {
    let session = voice_session.lock().await;

    Ok(VoiceStatus {
        active: session.is_in_session(),
        muted: session.is_muted(),
        participant_count: session.participant_count(),
        participants: Vec::new(), // Will be populated from events
        max_participants: 8,
    })
}

/// Toggle microphone mute state
#[tauri::command]
pub async fn toggle_mute(
    app: AppHandle,
    voice_session: State<'_, tokio::sync::Mutex<VoiceSession>>,
) -> Result<bool, String> {
    let session = voice_session.lock().await;
    if !session.is_in_session() {
        return Err("Not in a voice session".to_string());
    }
    let new_muted = !session.is_muted();
    session.set_muted(new_muted);
    let _ = app.emit("voice-mute-changed", new_muted);
    Ok(new_muted)
}
