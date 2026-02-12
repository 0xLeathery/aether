mod commands;
mod error;
mod identity;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::identity::has_identity,
            commands::identity::create_identity,
            commands::identity::get_identity,
            commands::identity::update_display_name,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
