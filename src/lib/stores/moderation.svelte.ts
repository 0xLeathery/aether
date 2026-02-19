import {
  getModerationState,
  setModeration,
  removeModeration,
  mutePeerVoice,
  unmutePeerVoice,
  type ModerationEntry,
  type ModerationTier,
} from '../tauri';

let entries = $state<Map<string, ModerationEntry>>(new Map());
let initialized = $state(false);
let loading = $state(false);
let error = $state<string | null>(null);

async function initialize() {
  if (initialized) return;

  loading = true;
  error = null;

  try {
    const state = await getModerationState();
    entries = new Map(Object.entries(state));
    initialized = true;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to load moderation state';
    console.error('Moderation initialization error:', err);
  } finally {
    loading = false;
  }
}

function getEffectiveTier(publicKey: string, swarmId?: string): ModerationTier | null {
  const entry = entries.get(publicKey);
  if (!entry) return null;

  // Check for swarm-specific override
  if (swarmId !== undefined && entry.swarm_overrides[swarmId] !== undefined) {
    return entry.swarm_overrides[swarmId];
  }

  return entry.tier;
}

// Cumulative tier helpers (per user decision: mute < hide < block)
function isMuted(publicKey: string, swarmId?: string): boolean {
  const tier = getEffectiveTier(publicKey, swarmId);
  return tier !== null; // mute, hide, or block all count as "muted"
}

function isHidden(publicKey: string, swarmId?: string): boolean {
  const tier = getEffectiveTier(publicKey, swarmId);
  return tier === 'hide' || tier === 'block';
}

function isBlocked(publicKey: string, swarmId?: string): boolean {
  const tier = getEffectiveTier(publicKey, swarmId);
  return tier === 'block';
}

async function setTier(publicKey: string, tier: ModerationTier) {
  error = null;

  // Optimistic update - preserve existing swarm_overrides
  const existing = entries.get(publicKey);
  const entry: ModerationEntry = {
    tier,
    swarm_overrides: existing?.swarm_overrides ?? {},
  };
  
  entries.set(publicKey, entry);

  try {
    await setModeration(publicKey, tier, entry.swarm_overrides);
    // Sync voice mute state
    await syncVoiceMute(publicKey);
  } catch (err) {
    // Rollback on error
    if (!existing) {
      entries.delete(publicKey);
    } else {
      entries.set(publicKey, existing);
    }
    error = err instanceof Error ? err.message : 'Failed to set moderation tier';
    console.error('Set tier error:', err);
    throw err;
  }
}

async function removeTier(publicKey: string) {
  error = null;

  // Optimistic update
  const existing = entries.get(publicKey);
  entries.delete(publicKey);

  try {
    await removeModeration(publicKey);
    // Sync voice unmute
    await syncVoiceMute(publicKey);
  } catch (err) {
    // Rollback on error
    if (existing) {
      entries.set(publicKey, existing);
    }
    error = err instanceof Error ? err.message : 'Failed to remove moderation tier';
    console.error('Remove tier error:', err);
    throw err;
  }
}

async function setSwarmOverride(
  publicKey: string,
  swarmId: string,
  tier: ModerationTier | null
) {
  error = null;

  // Get or create entry
  const existing = entries.get(publicKey);
  const entry: ModerationEntry = existing ?? {
    tier: 'mute', // Default tier when creating new entry via override
    swarm_overrides: {},
  };

  // Update override
  entry.swarm_overrides[swarmId] = tier;
  entries.set(publicKey, entry);

  try {
    await setModeration(publicKey, entry.tier, entry.swarm_overrides);
    // Sync voice mute state if this peer is now muted in any swarm
    await syncVoiceMute(publicKey);
  } catch (err) {
    // Rollback on error
    if (existing) {
      entries.set(publicKey, existing);
    } else {
      entries.delete(publicKey);
    }
    error = err instanceof Error ? err.message : 'Failed to set swarm override';
    console.error('Set swarm override error:', err);
    throw err;
  }
}

async function syncVoiceMute(publicKey: string) {
  // Private helper - syncs Rust mixer state based on current tier
  // Wrapped in try/catch because voice commands may fail if not in a session
  try {
    if (isMuted(publicKey)) {
      await mutePeerVoice(publicKey);
    } else {
      await unmutePeerVoice(publicKey);
    }
  } catch (err) {
    // Voice commands fail gracefully if not in a session - that's OK
    console.debug('Voice sync skipped (not in session):', err);
  }
}

function getAllEntries(): [string, ModerationEntry][] {
  return Array.from(entries.entries());
}

export const moderationStore = {
  get entries() { return entries; },
  get initialized() { return initialized; },
  get loading() { return loading; },
  get error() { return error; },
  initialize,
  getEffectiveTier,
  isMuted,
  isHidden,
  isBlocked,
  setTier,
  removeTier,
  setSwarmOverride,
  getAllEntries,
};
