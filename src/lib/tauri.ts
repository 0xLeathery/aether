import { invoke } from '@tauri-apps/api/core';

export interface IdentityInfo {
  public_key_hex: string;
  short_id: string;
  display_name: string;
}

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
