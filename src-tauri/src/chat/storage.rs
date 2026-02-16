use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

use crate::error::ChatError;
use super::document::ChatDocument;

/// Get the file path for a channel's Automerge document
///
/// Path format: {app_data_dir}/chat/{swarm_id}/{channel_id}.automerge
pub fn doc_path(app: &AppHandle, swarm_id: &str, channel_id: &str) -> Result<PathBuf, ChatError> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| ChatError::StorageError(format!("Failed to get app data dir: {}", e)))?;
    Ok(data_dir
        .join("chat")
        .join(swarm_id)
        .join(format!("{}.automerge", channel_id)))
}

/// Save a chat document to disk
///
/// Creates parent directories if they don't exist.
pub fn save_doc(
    app: &AppHandle,
    swarm_id: &str,
    channel_id: &str,
    doc: &mut ChatDocument,
) -> Result<(), ChatError> {
    let path = doc_path(app, swarm_id, channel_id)?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ChatError::StorageError(format!("Failed to create directories: {}", e)))?;
    }

    let bytes = doc.to_bytes();
    std::fs::write(&path, &bytes)
        .map_err(|e| ChatError::StorageError(format!("Failed to write document: {}", e)))?;

    Ok(())
}

/// Load a chat document from disk
///
/// Returns None if the file doesn't exist, Some(doc) if it does,
/// or Err on corruption/IO error.
pub fn load_doc(
    app: &AppHandle,
    swarm_id: &str,
    channel_id: &str,
) -> Result<Option<ChatDocument>, ChatError> {
    let path = doc_path(app, swarm_id, channel_id)?;

    if !path.exists() {
        return Ok(None);
    }

    let bytes = std::fs::read(&path)
        .map_err(|e| ChatError::StorageError(format!("Failed to read document: {}", e)))?;

    let doc = ChatDocument::from_bytes(&bytes)?;
    Ok(Some(doc))
}
