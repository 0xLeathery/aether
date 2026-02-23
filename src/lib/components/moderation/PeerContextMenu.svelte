<script lang="ts">
  import type { ModerationTier } from '../../tauri';

  let {
    x,
    y,
    publicKey,
    currentTier,
    onMute,
    onHide,
    onBlock,
    onRemove,
    onClose,
  }: {
    x: number;
    y: number;
    publicKey: string;
    currentTier: ModerationTier | null;
    onMute: () => void;
    onHide: () => void;
    onBlock: () => void;
    onRemove: () => void;
    onClose: () => void;
  } = $props();

  let clampedX = $derived(
    Math.min(x, (typeof window !== 'undefined' ? window.innerWidth : 800) - 180)
  );
  let clampedY = $derived(
    Math.min(y, (typeof window !== 'undefined' ? window.innerHeight : 600) - 160)
  );

  function handleMute() {
    onMute();
    onClose();
  }

  function handleHide() {
    onHide();
    onClose();
  }

  function handleBlock() {
    onBlock();
    onClose();
  }

  function handleRemove() {
    onRemove();
    onClose();
  }

  // Determine labels and handlers based on current tier
  // Active tier button calls onRemove (to undo), others call their set handler
  let muteLabel = $derived(currentTier === 'mute' ? '* Unmute Peer' : 'Mute Peer');
  let hideLabel = $derived(currentTier === 'hide' ? '* Unhide Peer' : 'Hide Peer');
  let blockLabel = $derived(currentTier === 'block' ? '* Unblock Peer' : 'Block Peer');

  function handleMuteClick() {
    if (currentTier === 'mute') {
      handleRemove();
    } else {
      handleMute();
    }
  }

  function handleHideClick() {
    if (currentTier === 'hide') {
      handleRemove();
    } else {
      handleHide();
    }
  }

  function handleBlockClick() {
    if (currentTier === 'block') {
      handleRemove();
    } else {
      handleBlock();
    }
  }
</script>

<svelte:window onclick={onClose} />

<div
  class="context-menu"
  style="left: {clampedX}px; top: {clampedY}px;"
  onclick={(e) => e.stopPropagation()}
>
  <button class="menu-item" onclick={handleMuteClick}>
    {muteLabel}
  </button>
  <button class="menu-item" onclick={handleHideClick}>
    {hideLabel}
  </button>
  <button class="menu-item menu-item--danger" onclick={handleBlockClick}>
    {blockLabel}
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
    min-width: 160px;
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
