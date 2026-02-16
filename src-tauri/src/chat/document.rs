use automerge::AutoCommit;
use autosurgeon::{Hydrate, Reconcile, hydrate, reconcile};

use crate::error::ChatError;
use super::message::ChatMessage;

/// Internal data structure for Automerge reconciliation
///
/// This struct maps to the Automerge document schema.
/// Messages are stored as a Vec and managed via autosurgeon derives.
#[derive(Debug, Clone, Reconcile, Hydrate)]
struct ChatDocData {
    messages: Vec<ChatMessage>,
}

/// A chat document wrapping an Automerge AutoCommit instance
///
/// Each ChatDocument represents a single channel's message history.
/// It provides typed operations for adding and retrieving messages,
/// with Automerge handling CRDT conflict resolution internally.
pub struct ChatDocument {
    doc: AutoCommit,
}

impl ChatDocument {
    /// Create a new empty chat document
    pub fn new() -> Result<Self, ChatError> {
        let mut doc = AutoCommit::new();
        let data = ChatDocData { messages: vec![] };
        reconcile(&mut doc, &data)
            .map_err(|e| ChatError::DocumentCorrupted(format!("Failed to initialize document: {}", e)))?;
        Ok(Self { doc })
    }

    /// Load a chat document from saved bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ChatError> {
        let doc = AutoCommit::load(bytes)
            .map_err(|e| ChatError::DocumentCorrupted(format!("Failed to load document: {}", e)))?;
        Ok(Self { doc })
    }

    /// Save the document to bytes
    pub fn to_bytes(&mut self) -> Vec<u8> {
        self.doc.save()
    }

    /// Set the Automerge actor ID from a hex-encoded Ed25519 public key
    ///
    /// This ties document changes to the user's cryptographic identity.
    pub fn set_actor(&mut self, pub_key_hex: &str) -> Result<(), ChatError> {
        let bytes = hex::decode(pub_key_hex)
            .map_err(|e| ChatError::MessageError(format!("Invalid hex public key: {}", e)))?;
        self.doc.set_actor(automerge::ActorId::from(bytes.as_slice()));
        Ok(())
    }

    /// Add a message to the document
    ///
    /// Hydrates the current message list, appends the new message,
    /// and reconciles back into the Automerge document.
    pub fn add_message(&mut self, msg: ChatMessage) -> Result<ChatMessage, ChatError> {
        let mut data: ChatDocData = hydrate(&self.doc)
            .map_err(|e| ChatError::DocumentCorrupted(format!("Failed to hydrate document: {}", e)))?;
        data.messages.push(msg.clone());
        reconcile(&mut self.doc, &data)
            .map_err(|e| ChatError::DocumentCorrupted(format!("Failed to reconcile document: {}", e)))?;
        Ok(msg)
    }

    /// Get all messages sorted by timestamp
    pub fn get_messages(&self) -> Result<Vec<ChatMessage>, ChatError> {
        let data: ChatDocData = hydrate(&self.doc)
            .map_err(|e| ChatError::DocumentCorrupted(format!("Failed to hydrate document: {}", e)))?;
        let mut messages = data.messages;
        messages.sort_by_key(|m| m.timestamp);
        Ok(messages)
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
