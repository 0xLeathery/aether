<script lang="ts">
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';
  import { swarmStore } from '../../stores/swarm.svelte';

  let { open = $bindable(false), onClose }: { open?: boolean; onClose: () => void } = $props();

  let swarmName = $state('');
  let generatedCode = $state<string | null>(null);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let copied = $state(false);

  async function handleGenerate() {
    if (!swarmName.trim()) {
      error = 'Swarm name is required';
      return;
    }

    loading = true;
    error = null;

    try {
      const uri = await swarmStore.createNewSwarm(swarmName.trim());
      generatedCode = uri;
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to generate swarm';
    } finally {
      loading = false;
    }
  }

  async function handleCopy() {
    if (!generatedCode) return;

    try {
      await writeText(generatedCode);
      copied = true;
      setTimeout(() => {
        copied = false;
      }, 2000);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to copy to clipboard';
    }
  }

  function handleClose() {
    swarmName = '';
    generatedCode = null;
    error = null;
    copied = false;
    onClose();
  }
</script>

{#if open}
  <div class="modal-overlay" onclick={handleClose}>
    <div class="modal-container" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h2>Create Swarm</h2>
      </div>

      {#if !generatedCode}
        <!-- Step 1: Input swarm name -->
        <div class="modal-body">
          <div class="form-group">
            <label for="swarm-name">Swarm Name</label>
            <input
              id="swarm-name"
              type="text"
              bind:value={swarmName}
              placeholder="My Private Swarm"
              disabled={loading}
            />
          </div>

          {#if error}
            <div class="error-message">{error}</div>
          {/if}
        </div>

        <div class="modal-footer">
          <button class="button button-secondary" onclick={handleClose} disabled={loading}>
            Cancel
          </button>
          <button class="button button-primary" onclick={handleGenerate} disabled={loading}>
            {loading ? 'Generating...' : 'Generate Secret Code'}
          </button>
        </div>
      {:else}
        <!-- Step 2: Show generated code -->
        <div class="modal-body">
          <p class="info-text">Share this Secret Code to invite others to your swarm:</p>
          <code class="secret-code">{generatedCode}</code>

          {#if error}
            <div class="error-message">{error}</div>
          {/if}
        </div>

        <div class="modal-footer">
          <button class="button button-secondary" onclick={handleClose}>
            Close
          </button>
          <button class="button button-primary" onclick={handleCopy}>
            {copied ? 'Copied!' : 'Copy to Clipboard'}
          </button>
        </div>
      {/if}
    </div>
  </div>
{/if}

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
    max-width: 500px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
  }

  .modal-header {
    padding: 1.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  .modal-header h2 {
    font-family: var(--font-mono);
    font-size: 1.1rem;
    color: var(--text-primary);
    font-weight: 500;
    margin: 0;
  }

  .modal-body {
    padding: 1.5rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.1em;
  }

  .form-group input {
    font-family: var(--font-mono);
    font-size: 0.95rem;
    padding: 0.75rem;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-primary);
    transition: border-color 0.2s ease;
  }

  .form-group input:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .form-group input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .info-text {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin-bottom: 1rem;
    line-height: 1.5;
  }

  .secret-code {
    display: block;
    font-family: var(--font-mono);
    font-size: 0.85rem;
    padding: 1rem;
    background: var(--bg-primary);
    border: 1px solid var(--accent-secondary);
    border-radius: 4px;
    color: var(--accent-secondary);
    word-break: break-all;
    line-height: 1.6;
  }

  .error-message {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: #ff4444;
    margin-top: 0.75rem;
    padding: 0.75rem;
    background: rgba(255, 68, 68, 0.1);
    border: 1px solid rgba(255, 68, 68, 0.3);
    border-radius: 4px;
  }

  .modal-footer {
    padding: 1rem 1.5rem;
    border-top: 1px solid var(--border-color);
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
  }

  .button {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    padding: 0.65rem 1.25rem;
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

  .button-secondary {
    background: transparent;
    color: var(--text-secondary);
  }

  .button-secondary:hover:not(:disabled) {
    background: var(--bg-tertiary);
    color: var(--text-primary);
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
</style>
