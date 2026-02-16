use std::path::PathBuf;

use tauri::AppHandle;
use tauri::Manager;

use crate::error::ChannelError;
use super::metadata_doc::SwarmMetadataDocument;

/// Get the file path for a swarm's metadata Automerge document
///
/// Path format: {app_data_dir}/swarm-meta/{swarm_id}.automerge
pub fn metadata_doc_path(app: &AppHandle, swarm_id: &str) -> Result<PathBuf, ChannelError> {
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| ChannelError::StorageError(format!("Failed to get app data dir: {}", e)))?;
    Ok(data_dir
        .join("swarm-meta")
        .join(format!("{}.automerge", swarm_id)))
}

/// Save a swarm metadata document to disk
///
/// Creates parent directories if they don't exist.
pub fn save_metadata_doc(
    app: &AppHandle,
    swarm_id: &str,
    doc: &mut SwarmMetadataDocument,
) -> Result<(), ChannelError> {
    let path = metadata_doc_path(app, swarm_id)?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            ChannelError::StorageError(format!("Failed to create directories: {}", e))
        })?;
    }

    let bytes = doc.to_bytes();
    std::fs::write(&path, &bytes).map_err(|e| {
        ChannelError::StorageError(format!("Failed to write metadata document: {}", e))
    })?;

    Ok(())
}

/// Load a swarm metadata document from disk
///
/// Returns None if the file doesn't exist, Some(doc) if it does,
/// or Err on corruption/IO error.
pub fn load_metadata_doc(
    app: &AppHandle,
    swarm_id: &str,
) -> Result<Option<SwarmMetadataDocument>, ChannelError> {
    let path = metadata_doc_path(app, swarm_id)?;

    if !path.exists() {
        return Ok(None);
    }

    let bytes = std::fs::read(&path).map_err(|e| {
        ChannelError::StorageError(format!("Failed to read metadata document: {}", e))
    })?;

    let doc = SwarmMetadataDocument::from_bytes(&bytes)?;
    Ok(Some(doc))
}

/// Delete a swarm metadata document from disk
///
/// Used during leave_swarm cleanup to remove the metadata file.
pub fn delete_metadata_doc(app: &AppHandle, swarm_id: &str) -> Result<(), ChannelError> {
    let path = metadata_doc_path(app, swarm_id)?;

    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| {
            ChannelError::StorageError(format!("Failed to delete metadata document: {}", e))
        })?;
    }

    Ok(())
}
