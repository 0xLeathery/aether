<script lang="ts">
  import { contactsStore } from '../../stores/contacts.svelte';

  let { publicKey, currentPetname, onClose }: {
    publicKey: string;
    currentPetname: string | null;
    onClose: () => void;
  } = $props();

  let petname = $state(currentPetname ?? '');
  let saving = $state(false);

  async function handleSave() {
    if (!petname.trim()) return;
    saving = true;
    try {
      await contactsStore.setPetname(publicKey, petname.trim());
      onClose();
    } catch {
      // Error is shown via contactsStore.error
    } finally {
      saving = false;
    }
  }

  async function handleRemove() {
    saving = true;
    try {
      await contactsStore.removePetname(publicKey);
      onClose();
    } catch {
      // Error is shown via contactsStore.error
    } finally {
      saving = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      handleSave();
    } else if (e.key === 'Escape') {
      onClose();
    }
  }
</script>

<div class="petname-editor" role="dialog" aria-label="Edit petname">
  <div class="editor-header">
    <span class="editor-title">SET PETNAME</span>
    <code class="editor-key">{publicKey.substring(0, 16)}...</code>
  </div>

  <div class="editor-body">
    <input
      class="petname-input"
      type="text"
      placeholder="Enter petname..."
      bind:value={petname}
      onkeydown={handleKeydown}
      disabled={saving}
    />

    {#if contactsStore.error}
      <div class="editor-error">{contactsStore.error}</div>
    {/if}
  </div>

  <div class="editor-actions">
    <button class="btn btn-save" onclick={handleSave} disabled={saving || !petname.trim()}>
      {saving ? 'SAVING...' : '[ SAVE ]'}
    </button>
    {#if currentPetname}
      <button class="btn btn-remove" onclick={handleRemove} disabled={saving}>
        [ REMOVE ]
      </button>
    {/if}
    <button class="btn btn-cancel" onclick={onClose} disabled={saving}>
      [ CANCEL ]
    </button>
  </div>
</div>

<style>
  .petname-editor {
    background: var(--bg-primary);
    border: 1px solid var(--accent-primary);
    border-radius: 4px;
    padding: 0.75rem;
    font-family: var(--font-mono);
    position: absolute;
    z-index: 100;
    width: 260px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
  }

  .editor-header {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    margin-bottom: 0.75rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  .editor-title {
    font-size: 0.75rem;
    color: var(--accent-primary);
    letter-spacing: 0.1em;
    font-weight: bold;
  }

  .editor-key {
    font-size: 0.7rem;
    color: var(--text-muted);
  }

  .editor-body {
    margin-bottom: 0.75rem;
  }

  .petname-input {
    width: 100%;
    padding: 0.5rem;
    font-family: var(--font-mono);
    font-size: 0.85rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 3px;
    color: var(--text-primary);
    outline: none;
    box-sizing: border-box;
  }

  .petname-input:focus {
    border-color: var(--accent-primary);
  }

  .petname-input:disabled {
    opacity: 0.5;
  }

  .editor-error {
    margin-top: 0.5rem;
    font-size: 0.75rem;
    color: #ff4444;
  }

  .editor-actions {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .btn {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    padding: 0.35rem 0.5rem;
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: 3px;
    cursor: pointer;
    transition: all 0.2s ease;
    letter-spacing: 0.05em;
  }

  .btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .btn-save {
    color: var(--accent-primary);
    border-color: var(--accent-primary);
  }

  .btn-save:hover:not(:disabled) {
    background: var(--accent-primary);
    color: var(--bg-primary);
  }

  .btn-remove {
    color: #ff4444;
    border-color: #ff4444;
  }

  .btn-remove:hover:not(:disabled) {
    background: #ff4444;
    color: var(--bg-primary);
  }

  .btn-cancel {
    color: var(--text-muted);
  }

  .btn-cancel:hover:not(:disabled) {
    background: var(--bg-tertiary);
  }
</style>
