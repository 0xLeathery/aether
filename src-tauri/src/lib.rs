mod commands;
mod error;
mod identity;
mod network;
mod swarm;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(std::sync::Mutex::new(network::NetworkService::new()))
        .invoke_handler(tauri::generate_handler![
            commands::identity::has_identity,
            commands::identity::create_identity,
            commands::identity::get_identity,
            commands::identity::update_display_name,
            commands::network::get_network_status,
            commands::network::get_peers,
            commands::network::start_network,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
