<script lang="ts">
  interface Peer {
    publicKey: string;
    displayName: string;
  }

  let { peers, filter, selectedIndex, position, onSelect, onClose }: {
    peers: Peer[];
    filter: string;
    selectedIndex: number;
    position: { bottom: number; left: number };
    onSelect: (publicKey: string, displayName: string) => void;
    onClose: () => void;
  } = $props();

  let filteredPeers = $derived(
    peers.filter(p => p.displayName.toLowerCase().includes(filter.toLowerCase()))
  );
</script>

<div
  class="mention-picker"
  style="bottom: {position.bottom}px; left: {position.left}px;"
>
  {#if filteredPeers.length === 0}
    <div class="no-matches">No matches</div>
  {:else}
    {#each filteredPeers as peer, i}
      <button
        class="mention-item"
        class:selected={i === selectedIndex}
        onmousedown={(e) => { e.preventDefault(); onSelect(peer.publicKey, peer.displayName); }}
      >
        <span class="peer-name">{peer.displayName}</span>
        <span class="peer-key">{peer.publicKey.substring(0, 8)}...</span>
      </button>
    {/each}
  {/if}
</div>

<style>
  .mention-picker {
    position: fixed;
    z-index: 1000;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 0.25rem 0;
    min-width: 200px;
    max-width: 320px;
    max-height: 200px;
    overflow-y: auto;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
  }

  .mention-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    padding: 0.4rem 0.75rem;
    font-family: var(--font-mono);
    font-size: 0.8rem;
    background: transparent;
    border: none;
    text-align: left;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .mention-item:hover,
  .mention-item.selected {
    background: var(--bg-tertiary);
  }

  .peer-name {
    color: var(--accent-primary);
    font-weight: 600;
  }

  .peer-key {
    color: var(--text-muted);
    font-size: 0.7rem;
  }

  .no-matches {
    padding: 0.5rem 0.75rem;
    font-family: var(--font-mono);
    font-size: 0.8rem;
    color: var(--text-muted);
  }
</style>
