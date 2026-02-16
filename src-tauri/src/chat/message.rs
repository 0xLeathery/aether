use autosurgeon::{Hydrate, Reconcile};
use serde::{Deserialize, Serialize};

/// A chat message in an Automerge CRDT document
///
/// Each message is uniquely identified by a UUID v4 and contains
/// the sender's Ed25519 public key, display name at send time,
/// message content, Unix epoch millisecond timestamp, and optional
/// mentions (hex-encoded public keys of mentioned peers).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reconcile, Hydrate)]
pub struct ChatMessage {
    /// Unique message identifier (UUID v4)
    pub id: String,
    /// Hex-encoded Ed25519 public key of the sender
    pub sender_key: String,
    /// Display name of the sender at time of sending
    pub sender_name: String,
    /// Message text content
    pub content: String,
    /// Unix epoch milliseconds when the message was sent
    pub timestamp: i64,
    /// Hex-encoded public keys of mentioned peers (backward-compatible default)
    #[serde(default)]
    pub mentions: Vec<String>,
}

impl ChatMessage {
    /// Create a new chat message with auto-generated UUID and current timestamp
    pub fn new(sender_key: String, sender_name: String, content: String, mentions: Vec<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            sender_key,
            sender_name,
            content,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as i64,
            mentions,
        }
    }
}
