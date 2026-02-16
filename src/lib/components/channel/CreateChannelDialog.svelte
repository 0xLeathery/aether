<script lang="ts">
  let { onClose, onCreate }: {
    onClose: () => void;
    onCreate: (name: string) => Promise<void>;
  } = $props();

  let channelName = $state('');
  let creating = $state(false);
  let errorMsg = $state<string | null>(null);

  let normalized = $derived.by(() => {
    return channelName
      .toLowerCase()
      .replace(/\s+/g, '-')
      .replace(/[^a-z0-9-]/g, '')
      .replace(/-{2,}/g, '-')
      .replace(/^-|-$/g, '');
  });

  let validationMsg = $derived.by(() => {
    if (!channelName) return null;
    if (channelName !== channelName.toLowerCase() || /[^a-z0-9\s-]/.test(channelName)) {
      return 'Only lowercase letters, numbers, and hyphens';
    }
    if (normalized === 'general' || normalized === 'voice') {
      return 'Reserved name';
    }
    if (normalized.length > 32) {
      return '1-32 characters';
    }
    return null;
  });

  let canCreate = $derived(
    normalized.length > 0 &&
    normalized.length <= 32 &&
    normalized !== 'general' &&
    normalized !== 'voice' &&
    !creating
  );

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement;
    channelName = target.value;
  }

  async function handleCreate() {
    if (!canCreate) return;
    creating = true;
    errorMsg = null;

    try {
      await onCreate(normalized);
      onClose();
    } catch (err) {
      errorMsg = err instanceof Error ? err.message : 'Failed to create channel';
    } finally {
      creating = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && canCreate) {
      handleCreate();
    }
    if (e.key === 'Escape') {
      onClose();
    }
  }
</script>

<div class="modal-overlay" onclick={onClose}>
  <div class="modal-container" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <h2>CREATE CHANNEL</h2>
      <button class="close-button" onclick={onClose}>[x]</button>
    </div>

    <div class="modal-body">
      <div class="form-group">
        <label for="channel-name">CHANNEL NAME</label>
        <input
          id="channel-name"
          type="text"
          value={channelName}
          oninput={handleInput}
          onkeydown={handleKeydown}
          placeholder="channel-name"
          disabled={creating}
          autofocus
        />
        {#if normalized && normalized !== channelName}
          <div class="preview"># {normalized}</div>
        {/if}
      </div>

      {#if validationMsg}
        <div class="validation-msg">{validationMsg}</div>
      {/if}

      {#if errorMsg}
        <div class="error-message">{errorMsg}</div>
      {/if}
    </div>

    <div class="modal-footer">
      <button class="button button-secondary" onclick={onClose} disabled={creating}>
        Cancel
      </button>
      <button class="button button-create" onclick={handleCreate} disabled={!canCreate}>
        {creating ? 'Creating...' : 'Create'}
      </button>
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
    max-width: 420px;
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
    font-size: 0.85rem;
    color: var(--text-muted);
    letter-spacing: 0.15em;
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
    gap: 0.75rem;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.15em;
    font-weight: 500;
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

  .preview {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    color: var(--text-secondary);
    opacity: 0.7;
  }

  .validation-msg {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    color: var(--accent-amber, #ffb000);
    padding: 0.4rem 0;
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

  .button-create {
    background: var(--accent-primary);
    color: var(--bg-primary);
    border-color: var(--accent-primary);
  }

  .button-create:hover:not(:disabled) {
    background: #00dd38;
    border-color: #00dd38;
  }
</style>
