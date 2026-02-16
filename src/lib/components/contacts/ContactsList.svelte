<script lang="ts">
  import { contactsStore } from '../../stores/contacts.svelte';
  import ContactEditor from './ContactEditor.svelte';

  let { onClose }: { onClose: () => void } = $props();

  let editingKey = $state<string | null>(null);

  function formatDate(timestamp: number): string {
    const date = new Date(timestamp * 1000);
    return date.toLocaleDateString([], { year: 'numeric', month: 'short', day: 'numeric' });
  }

  function truncateKey(key: string): string {
    return key.substring(0, 12) + '...';
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      if (editingKey) {
        editingKey = null;
      } else {
        onClose();
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="contacts-list" role="dialog" aria-label="Contacts list">
  <div class="contacts-header">
    <span class="contacts-title">CONTACTS</span>
    <button class="close-btn" onclick={onClose}>[ CLOSE ]</button>
  </div>

  {#if contactsStore.loading}
    <div class="empty-state">
      <span class="loading-text">Loading contacts...</span>
    </div>
  {:else if contactsStore.contacts.length === 0}
    <div class="empty-state">
      <span class="empty-text">No contacts yet. Peers you've interacted with will appear here.</span>
    </div>
  {:else}
    <div class="contacts-rows">
      {#each contactsStore.contacts as contact}
        <div class="contact-row" role="button" tabindex="0"
          onclick={() => editingKey = contact.public_key_hex}
          onkeydown={(e) => { if (e.key === 'Enter') editingKey = contact.public_key_hex; }}
        >
          <code class="contact-key">{truncateKey(contact.public_key_hex)}</code>
          <span class="contact-petname" class:has-petname={!!contact.petname}>
            {contact.petname ?? 'No petname'}
          </span>
          <span class="contact-date">{formatDate(contact.added_at)}</span>
        </div>

        {#if editingKey === contact.public_key_hex}
          <div class="editor-wrapper">
            <ContactEditor
              publicKey={contact.public_key_hex}
              currentPetname={contact.petname}
              onClose={() => editingKey = null}
            />
          </div>
        {/if}
      {/each}
    </div>
  {/if}

  {#if contactsStore.error}
    <div class="contacts-error">{contactsStore.error}</div>
  {/if}
</div>

<style>
  .contacts-list {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-family: var(--font-mono);
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 420px;
    max-height: 70vh;
    display: flex;
    flex-direction: column;
    z-index: 200;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.6);
  }

  .contacts-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    border-bottom: 1px solid var(--border-color);
    flex-shrink: 0;
  }

  .contacts-title {
    font-size: 0.85rem;
    color: var(--accent-primary);
    letter-spacing: 0.15em;
    font-weight: bold;
  }

  .close-btn {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-muted);
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: 3px;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
    letter-spacing: 0.05em;
    transition: all 0.2s ease;
  }

  .close-btn:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .empty-state {
    padding: 2rem 1rem;
    text-align: center;
  }

  .empty-text, .loading-text {
    font-size: 0.85rem;
    color: var(--text-muted);
    line-height: 1.5;
  }

  .contacts-rows {
    overflow-y: auto;
    flex: 1;
  }

  .contact-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem 1rem;
    border-bottom: 1px solid var(--border-color);
    cursor: pointer;
    transition: background 0.2s ease;
  }

  .contact-row:hover {
    background: var(--bg-tertiary);
  }

  .contact-key {
    font-size: 0.75rem;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .contact-petname {
    font-size: 0.85rem;
    color: var(--text-muted);
    font-style: italic;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .contact-petname.has-petname {
    color: var(--accent-primary);
    font-style: normal;
    font-weight: 600;
  }

  .contact-date {
    font-size: 0.7rem;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .editor-wrapper {
    position: relative;
    padding: 0.5rem 1rem;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .editor-wrapper :global(.petname-editor) {
    position: relative;
    width: 100%;
  }

  .contacts-error {
    padding: 0.75rem 1rem;
    font-size: 0.8rem;
    color: #ff4444;
    background: rgba(255, 68, 68, 0.1);
    border-top: 1px solid #ff4444;
  }
</style>
