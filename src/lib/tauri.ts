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
  creator_key: string | null;
}

export interface ChannelInfo {
  id: string;
  name: string;
}

export interface ChannelsUpdated {
  swarm_id: string;
}

export interface ChannelDeleted {
  swarm_id: string;
  channel_id: string;
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

export async function renameSwarm(swarmId: string, newName: string): Promise<void> {
  return invoke<void>('rename_swarm', { swarmId, newName });
}

export async function leaveSwarm(swarmId: string): Promise<void> {
  return invoke<void>('leave_swarm', { swarmId });
}

export async function getInviteUri(swarmId: string): Promise<string> {
  return invoke<string>('get_invite_uri', { swarmId });
}

// Swarm event listeners
export function onSwarmDeleted(callback: (swarmId: string) => void): Promise<UnlistenFn> {
  return listen<string>('swarm-deleted', (event) => callback(event.payload));
}

// Channel commands
export async function createChannel(swarmId: string, name: string): Promise<ChannelInfo> {
  return invoke<ChannelInfo>('create_channel', { swarmId, name });
}

export async function renameChannel(swarmId: string, channelId: string, newName: string): Promise<void> {
  return invoke<void>('rename_channel', { swarmId, channelId, newName });
}

export async function deleteChannel(swarmId: string, channelId: string): Promise<void> {
  return invoke<void>('delete_channel', { swarmId, channelId });
}

export async function listChannels(swarmId: string): Promise<ChannelInfo[]> {
  return invoke<ChannelInfo[]>('list_channels', { swarmId });
}

export async function migrateChannelMetadata(): Promise<void> {
  return invoke<void>('migrate_channel_metadata');
}

// Channel event listeners
export function onChannelsUpdated(callback: (update: ChannelsUpdated) => void): Promise<UnlistenFn> {
  return listen<ChannelsUpdated>('channels-updated', (event) => callback(event.payload));
}

export function onChannelDeleted(callback: (update: ChannelDeleted) => void): Promise<UnlistenFn> {
  return listen<ChannelDeleted>('channel-deleted', (event) => callback(event.payload));
}

// Voice types
export interface VoiceStatus {
  active: boolean;
  muted: boolean;
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

export async function toggleMute(): Promise<boolean> {
  return invoke<boolean>('toggle_mute');
}

// Voice event listeners
export function onVoiceSessionJoined(callback: (participants: string[]) => void): Promise<UnlistenFn> {
  return listen<string[]>('voice-session-joined', (event) => callback(event.payload));
}

export function onVoiceSessionLeft(callback: () => void): Promise<UnlistenFn> {
  return listen('voice-session-left', () => callback());
}

export function onVoiceMuteChanged(callback: (muted: boolean) => void): Promise<UnlistenFn> {
  return listen<boolean>('voice-mute-changed', (event) => callback(event.payload));
}

// Contact types
export interface Contact {
  public_key_hex: string;
  petname: string | null;
  notes: string | null;
  added_at: number;
}

// Contact commands
export async function setPetname(publicKey: string, petname: string): Promise<void> {
  return invoke<void>('set_petname', { publicKey, petname });
}

export async function removePetname(publicKey: string): Promise<void> {
  return invoke<void>('remove_petname', { publicKey });
}

export async function getContacts(): Promise<Contact[]> {
  return invoke<Contact[]>('get_contacts');
}

// Chat types
export interface ChatMessage {
  id: string;
  sender_key: string;
  sender_name: string;
  content: string;
  timestamp: number;
  mentions: string[];
}

export interface ChatMessagesUpdated {
  swarm_id: string;
  channel_id: string;
}

// Chat commands
export async function sendMessage(swarmId: string, channelId: string, content: string, mentions: string[] = []): Promise<ChatMessage> {
  return invoke<ChatMessage>('send_message', { swarmId, channelId, content, mentions });
}

export async function getMessages(swarmId: string, channelId: string): Promise<ChatMessage[]> {
  return invoke<ChatMessage[]>('get_messages', { swarmId, channelId });
}

// Chat event listeners
export function onChatMessagesUpdated(callback: (update: ChatMessagesUpdated) => void): Promise<UnlistenFn> {
  return listen<ChatMessagesUpdated>('chat-messages-updated', (event) => callback(event.payload));
}

// Unread types
export interface ChannelReadState {
  total_seen: number;
}

// Unread commands
export async function markChannelRead(swarmId: string, channelId: string, totalSeen: number): Promise<void> {
  return invoke<void>('mark_channel_read', { swarmId, channelId, totalSeen });
}

export async function getUnreadState(): Promise<Record<string, ChannelReadState>> {
  return invoke<Record<string, ChannelReadState>>('get_unread_state');
}

// Moderation types
export type ModerationTier = 'mute' | 'hide' | 'block';

export interface ModerationEntry {
  tier: ModerationTier;
  swarm_overrides: Record<string, ModerationTier | null>;
}

// Moderation commands
export async function getModerationState(): Promise<Record<string, ModerationEntry>> {
  return invoke<Record<string, ModerationEntry>>('get_moderation_state');
}

export async function setModeration(
  publicKey: string,
  tier: ModerationTier,
  swarmOverrides?: Record<string, ModerationTier | null>
): Promise<void> {
  return invoke<void>('set_moderation', { 
    publicKey, 
    tier, 
    swarmOverrides: swarmOverrides ?? {} 
  });
}

export async function removeModeration(publicKey: string): Promise<void> {
  return invoke<void>('remove_moderation', { publicKey });
}

export async function mutePeerVoice(peerKeyHex: string): Promise<void> {
  return invoke<void>('mute_peer_voice', { peerKeyHex });
}

export async function unmutePeerVoice(peerKeyHex: string): Promise<void> {
  return invoke<void>('unmute_peer_voice', { peerKeyHex });
}
