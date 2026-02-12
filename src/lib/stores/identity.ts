import { hasIdentity, createIdentity, getIdentity, updateDisplayName, type IdentityInfo } from '../tauri';

class IdentityStore {
  identity = $state<IdentityInfo | null>(null);
  loading = $state(false);
  initialized = $state(false);
  error = $state<string | null>(null);

  async initialize() {
    if (this.initialized) return;

    this.loading = true;
    this.error = null;

    try {
      const exists = await hasIdentity();
      if (exists) {
        this.identity = await getIdentity();
      }
      this.initialized = true;
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Failed to initialize identity';
      console.error('Identity initialization error:', err);
    } finally {
      this.loading = false;
    }
  }

  async create(displayName: string) {
    this.loading = true;
    this.error = null;

    try {
      this.identity = await createIdentity(displayName);
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Failed to create identity';
      console.error('Identity creation error:', err);
      throw err;
    } finally {
      this.loading = false;
    }
  }

  async updateName(newName: string) {
    if (!this.identity) return;

    this.loading = true;
    this.error = null;

    try {
      await updateDisplayName(newName);
      // Refresh identity to get updated data
      this.identity = await getIdentity();
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Failed to update display name';
      console.error('Display name update error:', err);
      throw err;
    } finally {
      this.loading = false;
    }
  }
}

export const identityStore = new IdentityStore();
