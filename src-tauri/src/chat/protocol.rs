use futures::io::{AsyncReadExt, AsyncWriteExt};
use libp2p::StreamProtocol;

use crate::error::ChatError;

/// Chat sync protocol identifier
///
/// Uses Automerge sync messages over length-prefixed framing.
pub const CHAT_PROTOCOL: StreamProtocol = StreamProtocol::new("/aether/chat/1.0.0");

/// Maximum sync message size: 5MB
const MAX_SYNC_MSG_SIZE: u32 = 5 * 1024 * 1024;

/// Send a sync message over a libp2p stream
///
/// Wire format: [4 bytes: length (u32 big-endian)][N bytes: data]
pub async fn send_sync_msg(stream: &mut libp2p::Stream, data: &[u8]) -> Result<(), ChatError> {
    let len = data.len() as u32;
    stream
        .write_all(&len.to_be_bytes())
        .await
        .map_err(|e| ChatError::SyncFailed(format!("Failed to write length prefix: {}", e)))?;

    stream
        .write_all(data)
        .await
        .map_err(|e| ChatError::SyncFailed(format!("Failed to write sync message: {}", e)))?;

    stream
        .flush()
        .await
        .map_err(|e| ChatError::SyncFailed(format!("Failed to flush stream: {}", e)))?;

    Ok(())
}

/// Receive a sync message from a libp2p stream
///
/// Returns None on clean EOF. Returns Err on timeout or malformed data.
/// Rejects messages larger than 5MB.
pub async fn recv_sync_msg(
    stream: &mut libp2p::Stream,
    timeout_ms: u64,
) -> Result<Option<Vec<u8>>, ChatError> {
    let timeout_duration = std::time::Duration::from_millis(timeout_ms);

    // Read 4-byte length prefix with timeout
    let mut len_buf = [0u8; 4];
    match tokio::time::timeout(timeout_duration, stream.read_exact(&mut len_buf)).await {
        Ok(Ok(_)) => {}
        Ok(Err(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Ok(Err(e)) => {
            return Err(ChatError::SyncFailed(format!(
                "Failed to read length prefix: {}",
                e
            )))
        }
        Err(_) => return Ok(None), // Timeout = no more messages
    }

    let len = u32::from_be_bytes(len_buf);

    // Reject oversized messages
    if len > MAX_SYNC_MSG_SIZE {
        return Err(ChatError::SyncFailed(format!(
            "Sync message too large: {} bytes (max {})",
            len, MAX_SYNC_MSG_SIZE
        )));
    }

    // Read message data with timeout
    let mut data = vec![0u8; len as usize];
    match tokio::time::timeout(timeout_duration, stream.read_exact(&mut data)).await {
        Ok(Ok(_)) => Ok(Some(data)),
        Ok(Err(e)) => Err(ChatError::SyncFailed(format!(
            "Failed to read sync message data: {}",
            e
        ))),
        Err(_) => Err(ChatError::SyncFailed(
            "Timeout reading sync message data".to_string(),
        )),
    }
}
