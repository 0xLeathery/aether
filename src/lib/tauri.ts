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

// Swarm types
export interface Channel {
  id: string;
  name: string;
}

export interface SwarmMetadata {
  id: string;
  name: string;
  psk_hex: string;
  created_at: number;
  channels: Channel[];
}

// Swarm commands
export async function createSwarm(name: string): Promise<string> {
  return invoke<string>('create_swarm', { name });
}

export async function joinSwarm(uri: string, name: string): Promise<string> {
  return invoke<string>('join_swarm', { uri, name });
}

export async function listSwarms(): Promise<SwarmMetadata[]> {
  return invoke<SwarmMetadata[]>('list_swarms');
}

export async function switchSwarm(swarmId: string): Promise<void> {
  return invoke<void>('switch_swarm', { swarmId });
}

// Voice types
export interface VoiceStatus {
  active: boolean;
  participants: string[];
  participant_count: number;
  max_participants: number;
}

// Voice commands
export async function joinVoice(): Promise<VoiceStatus> {
  return invoke<VoiceStatus>('join_voice');
}

export async function leaveVoice(): Promise<void> {
  return invoke<void>('leave_voice');
}

export async function getVoiceStatus(): Promise<VoiceStatus> {
  return invoke<VoiceStatus>('get_voice_status');
}

// Voice event listeners
export function onVoiceSessionJoined(callback: (participants: string[]) => void): Promise<UnlistenFn> {
  return listen<string[]>('voice-session-joined', (event) => callback(event.payload));
}

export function onVoiceSessionLeft(callback: () => void): Promise<UnlistenFn> {
  return listen('voice-session-left', () => callback());
}
