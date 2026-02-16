use tauri::AppHandle;

use crate::contacts::{self, Contact};

/// Set a petname for a peer
#[tauri::command]
pub fn set_petname(app: AppHandle, public_key: String, petname: String) -> Result<(), String> {
    contacts::storage::set_petname(&app, &public_key, &petname)
        .map_err(|e| e.to_string())
}

/// Remove a petname for a peer
#[tauri::command]
pub fn remove_petname(app: AppHandle, public_key: String) -> Result<(), String> {
    contacts::storage::remove_petname(&app, &public_key)
        .map_err(|e| e.to_string())
}

/// Get all contacts
#[tauri::command]
pub fn get_contacts(app: AppHandle) -> Result<Vec<Contact>, String> {
    contacts::storage::get_contacts(&app)
        .map_err(|e| e.to_string())
}
