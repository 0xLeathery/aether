pub mod storage;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Moderation tier for a peer - cumulative (mute < hide < block)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModerationTier {
    /// Peer is muted - cannot send audio
    Mute,
    /// Peer is hidden - not shown in peer list
    Hide,
    /// Peer is blocked - cannot interact at all
    Block,
}

/// Moderation entry for a specific peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationEntry {
    /// The global moderation tier for this peer
    pub tier: ModerationTier,
    /// Per-swarm overrides - can override global tier for specific swarms
    /// A null value means "no moderation in this swarm"
    #[serde(default)]
    pub swarm_overrides: HashMap<String, Option<ModerationTier>>,
}
