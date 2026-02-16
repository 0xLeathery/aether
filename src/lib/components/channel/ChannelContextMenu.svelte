<script lang="ts">
  let { x, y, channelId, channelName, onRename, onDelete, onClose }: {
    x: number;
    y: number;
    channelId: string;
    channelName: string;
    onRename: () => void;
    onDelete: () => void;
    onClose: () => void;
  } = $props();

  let clampedX = $derived(Math.min(x, (typeof window !== 'undefined' ? window.innerWidth : 800) - 160));
  let clampedY = $derived(Math.min(y, (typeof window !== 'undefined' ? window.innerHeight : 600) - 80));

  function handleRename() {
    onRename();
    onClose();
  }

  function handleDelete() {
    onDelete();
    onClose();
  }
</script>

<svelte:window onclick={onClose} />

<div
  class="context-menu"
  style="left: {clampedX}px; top: {clampedY}px;"
  onclick={(e) => e.stopPropagation()}
>
  <button class="menu-item" onclick={handleRename}>
    Rename
  </button>
  <button class="menu-item menu-item--danger" onclick={handleDelete}>
    Delete
  </button>
</div>

<style>
  .context-menu {
    position: fixed;
    z-index: 1000;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 0.25rem 0;
    min-width: 140px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
  }

  .menu-item {
    display: block;
    width: 100%;
    padding: 0.5rem 0.75rem;
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-secondary);
    background: transparent;
    border: none;
    text-align: left;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .menu-item:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .menu-item--danger:hover {
    color: #ff4444;
    background: rgba(255, 68, 68, 0.1);
  }
</style>
