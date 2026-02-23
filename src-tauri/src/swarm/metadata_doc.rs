use std::collections::HashMap;

use automerge::AutoCommit;
use autosurgeon::{Hydrate, Reconcile, hydrate, reconcile};
use serde::{Deserialize, Serialize};

use crate::error::ChannelError;

/// Channel entry in the CRDT metadata document
///
/// Stores the channel's display name, creation timestamp,
/// and the public key of the user who created it.
#[derive(Debug, Clone, Reconcile, Hydrate, Serialize, Deserialize)]
pub struct ChannelMeta {
    pub name: String,
    pub created_at: i64,
    pub created_by: String,
}

/// Swarm metadata stored in an Automerge CRDT document
///
/// Contains the swarm creator's identity and the channel map.
/// This is the source of truth for channel list, synced between peers.
/// Uses HashMap for O(1) channel lookup and clean CRDT deletion semantics.
#[derive(Debug, Clone, Reconcile, Hydrate)]
struct SwarmMetaData {
    creator_key: String,
    channels: HashMap<String, ChannelMeta>,
}

/// A swarm metadata document wrapping an Automerge AutoCommit instance
///
/// Each SwarmMetadataDocument represents a single swarm's metadata,
/// including the channel list and creator identity. It provides typed
/// operations for channel CRUD, with Automerge handling CRDT conflict
/// resolution internally.
pub struct SwarmMetadataDocument {
    doc: AutoCommit,
}

/// Get the current time as Unix epoch milliseconds
fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

impl SwarmMetadataDocument {
    /// Create a new metadata document with default channels (general + voice)
    ///
    /// The creator_key is the Ed25519 public key hex of the swarm creator.
    /// Two default channels are created: "general" and "voice".
    pub fn new(creator_key: &str) -> Result<Self, ChannelError> {
        let mut doc = AutoCommit::new();

        let mut channels = HashMap::new();
        channels.insert(
            "general".to_string(),
            ChannelMeta {
                name: "general".to_string(),
                created_at: now_millis(),
                created_by: creator_key.to_string(),
            },
        );
        channels.insert(
            "voice".to_string(),
            ChannelMeta {
                name: "voice".to_string(),
                created_at: now_millis(),
                created_by: creator_key.to_string(),
            },
        );

        let data = SwarmMetaData {
            creator_key: creator_key.to_string(),
            channels,
        };

        reconcile(&mut doc, &data).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to initialize metadata document: {}", e))
        })?;

        Ok(Self { doc })
    }

    /// Load a metadata document from saved bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChannelError> {
        let doc = AutoCommit::load(bytes).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to load metadata document: {}", e))
        })?;
        Ok(Self { doc })
    }

    /// Save the document to bytes
    pub fn to_bytes(&mut self) -> Vec<u8> {
        self.doc.save()
    }

    /// Add a channel to the metadata document
    ///
    /// Hydrates the current channel map, inserts the new channel,
    /// and reconciles back into the Automerge document.
    pub fn add_channel(&mut self, id: &str, meta: ChannelMeta) -> Result<(), ChannelError> {
        let mut data: SwarmMetaData = hydrate(&self.doc).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to hydrate metadata: {}", e))
        })?;
        data.channels.insert(id.to_string(), meta);
        reconcile(&mut self.doc, &data).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to reconcile metadata: {}", e))
        })?;
        Ok(())
    }

    /// Rename a channel in the metadata document
    ///
    /// Returns ChannelError::NotFound if the channel does not exist.
    pub fn rename_channel(&mut self, id: &str, new_name: &str) -> Result<(), ChannelError> {
        let mut data: SwarmMetaData = hydrate(&self.doc).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to hydrate metadata: {}", e))
        })?;
        let channel = data
            .channels
            .get_mut(id)
            .ok_or_else(|| ChannelError::NotFound(id.to_string()))?;
        channel.name = new_name.to_string();
        reconcile(&mut self.doc, &data).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to reconcile metadata: {}", e))
        })?;
        Ok(())
    }

    /// Remove a channel from the metadata document
    ///
    /// Returns ChannelError::DefaultChannel if the channel is "general" or "voice".
    pub fn remove_channel(&mut self, id: &str) -> Result<(), ChannelError> {
        if id == "general" || id == "voice" {
            return Err(ChannelError::DefaultChannel(id.to_string()));
        }
        let mut data: SwarmMetaData = hydrate(&self.doc).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to hydrate metadata: {}", e))
        })?;
        data.channels.remove(id);
        reconcile(&mut self.doc, &data).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to reconcile metadata: {}", e))
        })?;
        Ok(())
    }

    /// Get all channels from the metadata document
    pub fn get_channels(&self) -> Result<HashMap<String, ChannelMeta>, ChannelError> {
        let data: SwarmMetaData = hydrate(&self.doc).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to hydrate metadata: {}", e))
        })?;
        Ok(data.channels)
    }

    /// Get the creator's public key from the metadata document
    pub fn get_creator_key(&self) -> Result<String, ChannelError> {
        let data: SwarmMetaData = hydrate(&self.doc).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to hydrate metadata: {}", e))
        })?;
        Ok(data.creator_key)
    }

    /// Validate that all non-default channels were created by the swarm creator
    ///
    /// Hydrates the document, checks each channel's created_by field against
    /// the creator_key. Removes any channels where created_by does not match
    /// (except default channels "general" and "voice" which are always valid).
    /// Returns Ok(true) if unauthorized channels were removed, Ok(false) if
    /// all channels are valid. Returns Ok(false) if creator_key is empty.
    pub fn validate_channels_creator(&mut self) -> Result<bool, ChannelError> {
        let mut data: SwarmMetaData = hydrate(&self.doc).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to hydrate metadata: {}", e))
        })?;

        if data.creator_key.is_empty() {
            return Ok(false);
        }

        let mut needs_removal = false;
        let mut valid_channels = HashMap::new();

        for (id, meta) in &data.channels {
            // Default channels are always valid
            if id == "general" || id == "voice" {
                valid_channels.insert(id.clone(), meta.clone());
                continue;
            }
            // Only keep channels created by the swarm creator
            if meta.created_by == data.creator_key {
                valid_channels.insert(id.clone(), meta.clone());
            } else {
                needs_removal = true;
            }
        }

        if needs_removal {
            data.channels = valid_channels;
            reconcile(&mut self.doc, &data).map_err(|e| {
                ChannelError::DocumentError(format!("Failed to reconcile metadata: {}", e))
            })?;
        }

        Ok(needs_removal)
    }

    /// Fill missing created_by fields with the creator_key
    ///
    /// Iterates all channels and sets created_by to creator_key for any
    /// channel where created_by is empty. Returns Ok(true) if any fields
    /// were filled, Ok(false) if none needed filling or creator_key is empty.
    pub fn fill_missing_created_by(&mut self) -> Result<bool, ChannelError> {
        let mut data: SwarmMetaData = hydrate(&self.doc).map_err(|e| {
            ChannelError::DocumentError(format!("Failed to hydrate metadata: {}", e))
        })?;

        if data.creator_key.is_empty() {
            return Ok(false);
        }

        let mut filled = false;

        for (_id, meta) in data.channels.iter_mut() {
            if meta.created_by.is_empty() {
                meta.created_by = data.creator_key.clone();
                filled = true;
            }
        }

        if filled {
            reconcile(&mut self.doc, &data).map_err(|e| {
                ChannelError::DocumentError(format!("Failed to reconcile metadata: {}", e))
            })?;
        }

        Ok(filled)
    }

    /// Get mutable access to the inner AutoCommit (for sync protocol)
    pub fn doc_mut(&mut self) -> &mut AutoCommit {
        &mut self.doc
    }

    /// Get immutable access to the inner AutoCommit (for sync protocol)
    pub fn doc(&self) -> &AutoCommit {
        &self.doc
    }
}
