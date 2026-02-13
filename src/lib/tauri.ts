import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { UnlistenFn } from '@tauri-apps/api/event';

// Re-export UnlistenFn for convenience
export type { UnlistenFn };

// Identity types
export interface IdentityInfo {
  public_key_hex: string;
  short_id: string;
  display_name: string;
}

// Network types
export type PeerStatus = 'connecting' | 'online' | 'offline';

export interface PeerInfo {
  peer_id: string;
  status: PeerStatus;
}

export interface NetworkStatus {
  running: boolean;
  peer_id: string | null;
  listening_addrs: string[];
}

export interface PeerStatusUpdate {
  peer_id: string;
  status: PeerStatus;
  multiaddr: string | null;
}

// Identity commands
export async function hasIdentity(): Promise<boolean> {
  return invoke<boolean>('has_identity');
}

export async function createIdentity(displayName: string): Promise<IdentityInfo> {
  return invoke<IdentityInfo>('create_identity', { displayName });
}

export async function getIdentity(): Promise<IdentityInfo> {
  return invoke<IdentityInfo>('get_identity');
}

export async function updateDisplayName(newName: string): Promise<void> {
  return invoke<void>('update_display_name', { newName });
}

// Network commands
export async function getNetworkStatus(): Promise<NetworkStatus> {
  return invoke<NetworkStatus>('get_network_status');
}

export async function getPeers(): Promise<PeerInfo[]> {
  return invoke<PeerInfo[]>('get_peers');
}

export async function startNetwork(): Promise<void> {
  return invoke<void>('start_network');
}

// Network event listeners
export function onPeerStatusUpdate(callback: (update: PeerStatusUpdate) => void): Promise<UnlistenFn> {
  return listen<PeerStatusUpdate>('peer-status', (event) => callback(event.payload));
}
