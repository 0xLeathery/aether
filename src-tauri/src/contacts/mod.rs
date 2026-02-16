pub mod storage;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Contact {
    pub public_key_hex: String,
    pub petname: Option<String>,
    pub notes: Option<String>,
    pub added_at: i64,
}
