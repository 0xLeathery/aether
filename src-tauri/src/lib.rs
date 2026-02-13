mod commands;
mod error;
mod identity;
mod network;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(std::sync::Mutex::new(network::NetworkService::new()))
        .setup(|app| {
            // Try to auto-start network service
            // This will fail gracefully if identity doesn't exist yet
            let app_handle = app.handle().clone();
            let network_state = app.state::<std::sync::Mutex<network::NetworkService>>();

            if let Ok(mut network) = network_state.lock() {
                match network.start(app_handle) {
                    Ok(_) => {
                        println!("Network service started successfully");
                    }
                    Err(e) => {
                        // Expected error if identity not created yet
                        println!("Network service not started: {}", e);
                    }
                }
            }

            Ok(())
        })
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
