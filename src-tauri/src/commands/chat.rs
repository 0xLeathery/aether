use serde::Serialize;
use std::sync::Arc;
use tauri::{AppHandle, State};

use crate::chat::{ChatMessage, ChatService};
use crate::identity::{display, keypair, storage};
use crate::network::NetworkService;

/// Serializable chat message response for frontend
#[derive(Serialize, Clone)]
pub struct ChatMessageResponse {
    pub id: String,
    pub sender_key: String,
    pub sender_name: String,
    pub content: String,
    pub timestamp: i64,
}

impl From<ChatMessage> for ChatMessageResponse {
    fn from(msg: ChatMessage) -> Self {
        Self {
            id: msg.id,
            sender_key: msg.sender_key,
            sender_name: msg.sender_name,
            content: msg.content,
            timestamp: msg.timestamp,
        }
    }
}

/// Send a message to a channel
///
/// Persists the message via ChatService, triggers sync to online peers,
/// and returns the sent message.
#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    chat_service: State<'_, Arc<tokio::sync::Mutex<ChatService>>>,
    network: State<'_, std::sync::Mutex<NetworkService>>,
    swarm_id: String,
    channel_id: String,
    content: String,
) -> Result<ChatMessageResponse, String> {
    // Load identity from keychain for sender info
    let secret_bytes = storage::load_secret_key().map_err(|e| format!("Identity error: {}", e))?;
    let signing_key =
        keypair::signing_key_from_bytes(&secret_bytes).map_err(|e| format!("Key error: {}", e))?;
    let verifying_key = signing_key.verifying_key();
    let sender_key = keypair::public_key_to_hex(&verifying_key);
    let sender_name =
        display::load_display_name().map_err(|e| format!("Display name error: {}", e))?;

    // Lock ChatService and send message
    let mut service = chat_service.lock().await;

    // Ensure actor identity is set
    service.set_actor(&sender_key);

    let msg = service
        .send_message(&app, &swarm_id, &channel_id, &sender_key, &sender_name, &content)
        .map_err(|e| format!("Send error: {}", e))?;

    let response = ChatMessageResponse::from(msg);

    // Drop the lock before spawning sync tasks
    drop(service);

    // Fire-and-forget sync to online peers
    let sync_info = {
        let network_service = network.lock().map_err(|e| format!("Lock error: {}", e))?;
        if network_service.is_running() {
            let peers = network_service.get_peers();
            let control = network_service.stream_control();
            Some((peers, control))
        } else {
            None
        }
    };

    if let Some((peers, Some(control))) = sync_info {
        let chat_arc = Arc::clone(&chat_service);

        for (peer_id_str, status) in peers {
            if status == crate::network::peer_state::PeerStatus::Online {
                if let Ok(peer_id) = peer_id_str.parse() {
                    ChatService::trigger_sync_to_peer(
                        app.clone(),
                        Arc::clone(&chat_arc),
                        control.clone(),
                        peer_id,
                        swarm_id.clone(),
                        channel_id.clone(),
                    );
                }
            }
        }
    }

    Ok(response)
}

/// Get all messages for a channel
///
/// Returns message history sorted by timestamp.
#[tauri::command]
pub async fn get_messages(
    app: AppHandle,
    chat_service: State<'_, Arc<tokio::sync::Mutex<ChatService>>>,
    swarm_id: String,
    channel_id: String,
) -> Result<Vec<ChatMessageResponse>, String> {
    let mut service = chat_service.lock().await;

    // Ensure actor identity is set for document loading
    if let Ok(secret_bytes) = storage::load_secret_key() {
        if let Ok(signing_key) = keypair::signing_key_from_bytes(&secret_bytes) {
            let verifying_key = signing_key.verifying_key();
            let sender_key = keypair::public_key_to_hex(&verifying_key);
            service.set_actor(&sender_key);
        }
    }

    let messages = service
        .get_messages(&app, &swarm_id, &channel_id)
        .map_err(|e| format!("Get messages error: {}", e))?;

    Ok(messages.into_iter().map(ChatMessageResponse::from).collect())
}
