import { getNetworkStatus, getPeers, startNetwork, onPeerStatusUpdate, type PeerInfo, type NetworkStatus, type PeerStatusUpdate, type UnlistenFn } from '../tauri';

let status = $state<NetworkStatus | null>(null);
let peers = $state<PeerInfo[]>([]);
let initialized = $state(false);
let error = $state<string | null>(null);
let unlisten: UnlistenFn | null = null;

async function initialize() {
  if (initialized) return;

  try {
    // Get initial network status
    status = await getNetworkStatus();

    // If network is running, get initial peer list
    if (status.running) {
      peers = await getPeers();
    }

    // Set up peer status update listener
    unlisten = await onPeerStatusUpdate((update: PeerStatusUpdate) => {
      // Update or add peer in the list
      const peerIndex = peers.findIndex(p => p.peer_id === update.peer_id);

      if (peerIndex >= 0) {
        // Update existing peer (immutable pattern for Svelte 5 reactivity)
        peers = peers.map((p, i) =>
          i === peerIndex ? { ...p, status: update.status } : p
        );
      } else {
        // Add new peer
        peers = [...peers, { peer_id: update.peer_id, status: update.status }];
      }
    });

    initialized = true;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to initialize network';
    console.error('Network initialization error:', err);
  }
}

async function start() {
  error = null;

  try {
    await startNetwork();
    // Refresh status after starting
    status = await getNetworkStatus();
    peers = await getPeers();
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to start network';
    console.error('Network start error:', err);
    throw err;
  }
}

function cleanup() {
  if (unlisten) {
    unlisten();
    unlisten = null;
  }
  status = null;
  peers = [];
  initialized = false;
  error = null;
}

export const networkStore = {
  get status() { return status; },
  get peers() { return peers; },
  get initialized() { return initialized; },
  get error() { return error; },
  get onlinePeerCount() { return peers.filter(p => p.status === 'online').length; },
  initialize,
  start,
  cleanup,
};
