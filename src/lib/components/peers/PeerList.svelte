<script lang="ts">
  import type { PeerInfo } from '../../tauri';
  import { contactsStore } from '../../stores/contacts.svelte';
  import ContactEditor from '../contacts/ContactEditor.svelte';

  let { peers }: { peers: PeerInfo[] } = $props();

  let editingPeerId = $state<string | null>(null);

  function getStatusColor(status: string): string {
    switch (status) {
      case 'online':
        return '#00ff41'; // green
      case 'connecting':
        return '#ffb000'; // amber
      case 'offline':
        return '#555555'; // gray
      default:
        return '#555555';
    }
  }

  function getPeerDisplayName(peerId: string): string {
    return contactsStore.resolveName(peerId, '');
  }

  function getPeerPetname(peerId: string): string | null {
    const contact = contactsStore.contacts.find(c => c.public_key_hex === peerId);
    return contact?.petname ?? null;
  }

  function formatStatus(status: string): string {
    return status.charAt(0).toUpperCase() + status.slice(1);
  }
</script>

{#if peers.length === 0}
  <div class="empty-state">
    <p class="scanning">Scanning network<span class="pulse">...</span></p>
  </div>
{:else}
  <div class="peer-list">
    {#each peers as peer}
      <div class="peer-row">
        <div class="status-dot" style="background-color: {getStatusColor(peer.status)}"></div>
        <span class="peer-id">{getPeerDisplayName(peer.peer_id)}</span>
        <button class="edit-btn" onclick={() => editingPeerId = peer.peer_id} title="Set petname">
          [~]
        </button>
        <span class="peer-status">{formatStatus(peer.status)}</span>
      </div>

      {#if editingPeerId === peer.peer_id}
        <div class="editor-container">
          <ContactEditor
            publicKey={peer.peer_id}
            currentPetname={getPeerPetname(peer.peer_id)}
            onClose={() => editingPeerId = null}
          />
        </div>
      {/if}
    {/each}
  </div>
{/if}

<style>
  .empty-state {
    padding: 1.5rem 1rem;
    text-align: center;
  }

  .scanning {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-muted);
  }

  .pulse {
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.3; }
  }

  .peer-list {
    display: flex;
    flex-direction: column;
  }

  .peer-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 1rem;
    border-bottom: 1px solid var(--border-color);
    transition: background 0.2s ease;
  }

  .peer-row:hover {
    background: var(--bg-tertiary);
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .peer-id {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-primary);
    flex: 1;
  }

  .peer-status {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-muted);
    text-transform: capitalize;
  }

  .edit-btn {
    font-family: var(--font-mono);
    font-size: 0.7rem;
    color: var(--text-muted);
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0.1rem 0.25rem;
    transition: color 0.2s ease;
    flex-shrink: 0;
  }

  .edit-btn:hover {
    color: var(--accent-primary);
  }

  .editor-container {
    padding: 0.5rem 1rem;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    position: relative;
  }

  .editor-container :global(.petname-editor) {
    position: relative;
    width: 100%;
  }
</style>
