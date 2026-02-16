<script lang="ts">
  let { channelName, onConfirm, onCancel }: {
    channelName: string;
    onConfirm: () => Promise<void>;
    onCancel: () => void;
  } = $props();

  let confirmText = $state('');
  let deleting = $state(false);
  let errorMsg = $state<string | null>(null);

  let isMatch = $derived(confirmText === channelName);

  async function handleDelete() {
    if (!isMatch || deleting) return;
    deleting = true;
    errorMsg = null;

    try {
      await onConfirm();
      onCancel();
    } catch (err) {
      errorMsg = err instanceof Error ? err.message : 'Failed to delete channel';
    } finally {
      deleting = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && isMatch && !deleting) {
      handleDelete();
    }
    if (e.key === 'Escape') {
      onCancel();
    }
  }
</script>

<div class="modal-overlay" onclick={onCancel}>
  <div class="modal-container" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <h2>DELETE CHANNEL</h2>
      <button class="close-button" onclick={onCancel}>[x]</button>
    </div>

    <div class="modal-body">
      <p class="warning-text">
        This will permanently delete all messages in <strong>#{channelName}</strong>. This action cannot be undone.
      </p>

      <div class="form-group">
        <label for="confirm-input">
          Type <strong class="channel-name-highlight">{channelName}</strong> to confirm:
        </label>
        <input
          id="confirm-input"
          type="text"
          bind:value={confirmText}
          onkeydown={handleKeydown}
          placeholder={channelName}
          disabled={deleting}
          autofocus
        />
      </div>

      {#if errorMsg}
        <div class="error-message">{errorMsg}</div>
      {/if}
    </div>

    <div class="modal-footer">
      <button class="button button-secondary" onclick={onCancel} disabled={deleting}>
        Cancel
      </button>
      <button
        class="button button-danger"
        onclick={handleDelete}
        disabled={!isMatch || deleting}
      >
        {deleting ? 'Deleting...' : 'Delete Channel'}
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
    color: #ff4444;
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
    gap: 1rem;
  }

  .warning-text {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-secondary);
    line-height: 1.6;
    margin: 0;
  }

  .warning-text strong {
    color: #ff4444;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-group label {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    color: var(--text-secondary);
    line-height: 1.5;
  }

  .channel-name-highlight {
    color: var(--text-primary);
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
    border-color: #ff4444;
  }

  .form-group input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
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

  .button-danger {
    background: transparent;
    border: 1px solid #ff4444;
    color: #ff4444;
  }

  .button-danger:hover:not(:disabled) {
    background: #ff4444;
    color: #0a0a0a;
  }
</style>
