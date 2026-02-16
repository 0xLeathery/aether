mod chat;
mod commands;
mod contacts;
mod error;
mod identity;
mod network;
mod swarm;
mod voice;

use std::sync::Arc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(std::sync::Mutex::new(network::NetworkService::new()))
        .manage(tokio::sync::Mutex::new(voice::VoiceSession::new()))
        .manage(Arc::new(tokio::sync::Mutex::new(chat::ChatService::new())))
        .invoke_handler(tauri::generate_handler![
            commands::identity::has_identity,
            commands::identity::create_identity,
            commands::identity::get_identity,
            commands::identity::update_display_name,
            commands::network::get_network_status,
            commands::network::get_peers,
            commands::network::start_network,
            commands::swarm::create_swarm,
            commands::swarm::join_swarm,
            commands::swarm::list_swarms,
            commands::swarm::switch_swarm,
            commands::swarm::rename_swarm,
            commands::swarm::leave_swarm,
            commands::swarm::get_invite_uri,
            commands::voice::join_voice,
            commands::voice::leave_voice,
            commands::voice::get_voice_status,
            commands::voice::toggle_mute,
            commands::channel::create_channel,
            commands::channel::rename_channel,
            commands::channel::delete_channel,
            commands::channel::list_channels,
            commands::chat::send_message,
            commands::chat::get_messages,
            commands::contacts::set_petname,
            commands::contacts::remove_petname,
            commands::contacts::get_contacts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
