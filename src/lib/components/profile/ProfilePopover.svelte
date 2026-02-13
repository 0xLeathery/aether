<script lang="ts">
  import { identityStore } from '../../stores/identity.svelte';
  import Avatar from './Avatar.svelte';

  let { isOpen = $bindable(false) }: { isOpen?: boolean } = $props();

  let isEditing = $state(false);
  let editedName = $state('');
  let copyFeedback = $state(false);

  const identity = $derived(identityStore.identity);

  function startEdit() {
    if (!identity) return;
    editedName = identity.display_name;
    isEditing = true;
  }

  async function saveEdit() {
    if (!editedName.trim() || !identity) return;
    try {
      await identityStore.updateName(editedName.trim());
      isEditing = false;
    } catch (err) {
      console.error('Failed to update name:', err);
    }
  }

  function cancelEdit() {
    isEditing = false;
    editedName = '';
  }

  async function copyPublicKey() {
    if (!identity) return;
    try {
      await navigator.clipboard.writeText(identity.public_key_hex);
      copyFeedback = true;
      setTimeout(() => {
        copyFeedback = false;
      }, 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      saveEdit();
    } else if (event.key === 'Escape') {
      cancelEdit();
    }
  }

  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (isOpen && !target.closest('.profile-popover')) {
      isOpen = false;
      if (isEditing) {
        cancelEdit();
      }
    }
  }
</script>

<svelte:window onclick={handleClickOutside} />

{#if isOpen && identity}
  <div class="profile-popover">
    <div class="popover-content">
      <div class="avatar-section">
        <Avatar publicKeyHex={identity.public_key_hex} size={80} />
      </div>

      <div class="name-section">
        {#if isEditing}
          <input
            type="text"
            bind:value={editedName}
            onkeydown={handleKeydown}
            onblur={saveEdit}
            class="name-input"
          />
        {:else}
          <button class="name-display" onclick={startEdit}>
            {identity.display_name}
          </button>
        {/if}
      </div>

      <div class="id-section">
        <div class="short-id">{identity.short_id}</div>
      </div>

      <div class="key-section">
        <div class="key-label">Public Key</div>
        <div class="key-display">
          <code class="key-text">{identity.public_key_hex.substring(0, 24)}...</code>
          <button class="copy-btn" onclick={copyPublicKey} disabled={copyFeedback}>
            {copyFeedback ? 'Copied!' : 'Copy'}
          </button>
        </div>
      </div>

      <div class="storage-info">
        Identity stored in {navigator.platform.toLowerCase().includes('mac') ? 'macOS Keychain' :
          navigator.platform.toLowerCase().includes('win') ? 'Windows Credential Manager' :
          'system keyring'}
      </div>
    </div>
  </div>
{/if}

<style>
  .profile-popover {
    position: fixed;
    bottom: 80px;
    left: 20px;
    width: 320px;
    background: var(--bg-secondary);
    border: 2px solid var(--accent-primary);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
    z-index: 1000;
  }

  .popover-content {
    padding: 1.5rem;
  }

  .avatar-section {
    display: flex;
    justify-content: center;
    margin-bottom: 1rem;
  }

  .name-section {
    margin-bottom: 0.75rem;
    text-align: center;
  }

  .name-display {
    font-family: var(--font-mono);
    font-size: 1.2rem;
    color: var(--text-primary);
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    transition: color 0.2s ease;
  }

  .name-display:hover {
    color: var(--accent-primary);
  }

  .name-input {
    font-family: var(--font-mono);
    font-size: 1.2rem;
    color: var(--text-primary);
    background: var(--bg-primary);
    border: 1px solid var(--accent-primary);
    padding: 0.25rem 0.5rem;
    width: 100%;
    text-align: center;
    outline: none;
  }

  .id-section {
    margin-bottom: 1rem;
    text-align: center;
  }

  .short-id {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    color: var(--text-secondary);
    letter-spacing: 0.05em;
  }

  .key-section {
    margin-bottom: 1rem;
  }

  .key-label {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    display: block;
    margin-bottom: 0.5rem;
  }

  .key-display {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    background: var(--bg-primary);
    padding: 0.5rem;
    border: 1px solid var(--border-color);
  }

  .key-text {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    color: var(--text-secondary);
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .copy-btn {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    padding: 0.25rem 0.5rem;
    background: transparent;
    border: 1px solid var(--accent-primary);
    color: var(--accent-primary);
    cursor: pointer;
    text-transform: uppercase;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .copy-btn:hover:not(:disabled) {
    background: var(--accent-primary);
    color: var(--bg-primary);
  }

  .copy-btn:disabled {
    opacity: 0.5;
    cursor: default;
  }

  .storage-info {
    font-family: var(--font-mono);
    font-size: 0.7rem;
    color: var(--text-muted);
    text-align: center;
    line-height: 1.4;
  }
</style>
