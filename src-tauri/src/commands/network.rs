use crate::error::NetworkError;
use crate::network::NetworkService;
use serde::Serialize;
use tauri::{AppHandle, State};

#[derive(Serialize)]
pub struct NetworkStatus {
    pub running: bool,
    pub peer_id: Option<String>,
    pub listening_addrs: Vec<String>,
}

#[derive(Serialize)]
pub struct PeerInfo {
    pub peer_id: String,
    pub status: crate::network::peer_state::PeerStatus,
}

/// Get current network status (running state, peer ID, listening addresses)
#[tauri::command]
pub async fn get_network_status(
    service: State<'_, std::sync::Mutex<NetworkService>>,
) -> Result<NetworkStatus, NetworkError> {
    let svc = service
        .lock()
        .map_err(|_| NetworkError::SwarmStart("Lock poisoned".into()))?;

    Ok(NetworkStatus {
        running: svc.is_running(),
        peer_id: svc.local_peer_id().map(|p| p.to_string()),
        listening_addrs: svc.listening_addrs().iter().map(|a| a.to_string()).collect(),
    })
}

/// Get list of discovered peers and their connection status
#[tauri::command]
pub async fn get_peers(
    service: State<'_, std::sync::Mutex<NetworkService>>,
) -> Result<Vec<PeerInfo>, NetworkError> {
    let svc = service
        .lock()
        .map_err(|_| NetworkError::SwarmStart("Lock poisoned".into()))?;

    Ok(svc
        .get_peers()
        .into_iter()
        .map(|(peer_id, status)| PeerInfo { peer_id, status })
        .collect())
}

/// Start the network service manually (called after identity creation)
#[tauri::command]
pub async fn start_network(
    app: AppHandle,
    service: State<'_, std::sync::Mutex<NetworkService>>,
) -> Result<(), NetworkError> {
    let mut svc = service
        .lock()
        .map_err(|_| NetworkError::SwarmStart("Lock poisoned".into()))?;

    if svc.is_running() {
        return Ok(());
    }

    svc.start(app)
}
