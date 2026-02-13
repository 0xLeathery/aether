import { hasIdentity, createIdentity, getIdentity, updateDisplayName, type IdentityInfo } from '../tauri';

let identity = $state<IdentityInfo | null>(null);
let loading = $state(false);
let initialized = $state(false);
let error = $state<string | null>(null);

async function initialize() {
  if (initialized) return;

  loading = true;
  error = null;

  try {
    const exists = await hasIdentity();
    if (exists) {
      identity = await getIdentity();
    }
    initialized = true;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to initialize identity';
    console.error('Identity initialization error:', err);
  } finally {
    loading = false;
  }
}

async function create(displayName: string) {
  loading = true;
  error = null;

  try {
    identity = await createIdentity(displayName);
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to create identity';
    console.error('Identity creation error:', err);
    throw err;
  } finally {
    loading = false;
  }
}

async function updateName(newName: string) {
  if (!identity) return;

  loading = true;
  error = null;

  try {
    await updateDisplayName(newName);
    identity = await getIdentity();
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to update display name';
    console.error('Display name update error:', err);
    throw err;
  } finally {
    loading = false;
  }
}

export const identityStore = {
  get identity() { return identity; },
  get loading() { return loading; },
  get initialized() { return initialized; },
  get error() { return error; },
  initialize,
  create,
  updateName,
};
