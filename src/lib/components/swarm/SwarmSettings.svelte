<script lang="ts">
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';
  import { swarmStore } from '../../stores/swarm.svelte';
  import type { SwarmMetadata } from '../../tauri';

  let { swarm, onClose }: { swarm: SwarmMetadata; onClose: () => void } = $props();

  // Rename state
  let newName = $state(swarm.name);
  let renaming = $state(false);

  // Invite link state
  let copied = $state(false);
  let copyError = $state<string | null>(null);

  // Leave state
  let confirmLeave = $state(false);
  let leaving = $state(false);

  // Error state
  let error = $state<string | null>(null);

  async function handleRename() {
    if (!newName.trim() || newName.trim() === swarm.name) return;

    renaming = true;
    error = null;

    try {
      await swarmStore.renameSwarm(swarm.id, newName.trim());
      onClose();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to rename swarm';
    } finally {
      renaming = false;
    }
  }

  async function handleCopyInvite() {
    copyError = null;

    try {
      const uri = await swarmStore.getInviteUri(swarm.id);
      await writeText(uri);
      copied = true;
      setTimeout(() => {
        copied = false;
      }, 2000);
    } catch (err) {
      copyError = err instanceof Error ? err.message : 'Failed to copy invite link';
    }
  }

  async function handleLeave() {
    leaving = true;
    error = null;

    try {
      await swarmStore.leaveSwarm(swarm.id);
      onClose();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to leave swarm';
      leaving = false;
    }
  }

  function handleClose() {
    newName = swarm.name;
    confirmLeave = false;
    error = null;
    copyError = null;
    onClose();
  }
</script>

<div class="modal-overlay" onclick={handleClose}>
  <div class="modal-container" data-testid="swarm-settings" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <h2>Swarm Settings</h2>
      <button class="close-button" onclick={handleClose}>[x]</button>
    </div>

    <div class="modal-body">
      <!-- Rename Section -->
      <div class="section">
        <label for="swarm-rename" class="section-label">SWARM NAME</label>
        <div class="input-row">
          <input
            id="swarm-rename"
            type="text"
            bind:value={newName}
            disabled={renaming}
          />
          <button
            class="button button-primary"
            onclick={handleRename}
            disabled={renaming || !newName.trim() || newName.trim() === swarm.name}
          >
            {renaming ? 'Saving...' : 'Rename'}
          </button>
        </div>
      </div>

      <!-- Invite Link Section -->
      <div class="section">
        <label class="section-label">INVITE LINK</label>
        <button class="button button-secondary full-width" onclick={handleCopyInvite}>
          {copied ? 'Copied!' : 'Copy Invite Link'}
        </button>
        {#if copyError}
          <div class="error-message">{copyError}</div>
        {/if}
      </div>

      <!-- Danger Zone -->
      <div class="section danger-zone">
        <label class="section-label danger-label">DANGER ZONE</label>
        {#if !confirmLeave}
          <button class="button button-danger full-width" onclick={() => confirmLeave = true}>
            Leave Swarm
          </button>
        {:else}
          <p class="confirm-text">Are you sure? This will delete all local data for this swarm.</p>
          <div class="confirm-buttons">
            <button
              class="button button-secondary"
              onclick={() => confirmLeave = false}
              disabled={leaving}
            >
              Cancel
            </button>
            <button
              class="button button-danger"
              onclick={handleLeave}
              disabled={leaving}
            >
              {leaving ? 'Leaving...' : 'Confirm Leave'}
            </button>
          </div>
        {/if}
      </div>

      {#if error}
        <div class="error-message">{error}</div>
      {/if}
    </div>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-container {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    width: 90%;
    max-width: 450px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }

  .modal-header {
    padding: 1.5rem;
    border-bottom: 1px solid var(--border-color);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .modal-header h2 {
    font-family: var(--font-mono);
    font-size: 1.1rem;
    color: var(--text-primary);
    font-weight: 500;
    margin: 0;
  }

  .close-button {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    color: var(--text-muted);
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
  }

  .close-button:hover {
    color: var(--text-primary);
  }

  .modal-body {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .section-label {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.15em;
    font-weight: 500;
  }

  .input-row {
    display: flex;
    gap: 0.5rem;
  }

  .input-row input {
    flex: 1;
    font-family: var(--font-mono);
    font-size: 0.9rem;
    padding: 0.6rem 0.75rem;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-primary);
    transition: border-color 0.2s ease;
  }

  .input-row input:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .input-row input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .danger-zone {
    border: 1px solid rgba(255, 68, 68, 0.3);
    border-radius: 4px;
    padding: 1rem;
  }

  .danger-label {
    color: #ff4444;
  }

  .confirm-text {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-secondary);
    line-height: 1.5;
    margin: 0;
  }

  .confirm-buttons {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .button {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    padding: 0.55rem 1rem;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s ease;
    text-transform: uppercase;
    letter-spacing: 0.1em;
  }

  .button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .button-primary {
    background: var(--accent-primary);
    color: var(--bg-primary);
    border-color: var(--accent-primary);
  }

  .button-primary:hover:not(:disabled) {
    background: #00dd38;
    border-color: #00dd38;
  }

  .button-secondary {
    background: transparent;
    color: var(--text-secondary);
  }

  .button-secondary:hover:not(:disabled) {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .button-danger {
    background: rgba(255, 68, 68, 0.15);
    color: #ff4444;
    border-color: rgba(255, 68, 68, 0.4);
  }

  .button-danger:hover:not(:disabled) {
    background: rgba(255, 68, 68, 0.25);
    border-color: #ff4444;
  }

  .full-width {
    width: 100%;
  }

  .error-message {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    color: #ff4444;
    padding: 0.5rem 0.75rem;
    background: rgba(255, 68, 68, 0.1);
    border: 1px solid rgba(255, 68, 68, 0.3);
    border-radius: 4px;
  }
</style>
