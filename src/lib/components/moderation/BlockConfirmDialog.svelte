<script lang="ts">
  let {
    peerName,
    onConfirm,
    onCancel,
  }: {
    peerName: string;
    onConfirm: () => void;
    onCancel: () => void;
  } = $props();

  function handleConfirm() {
    onConfirm();
  }

  function handleCancel() {
    onCancel();
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={handleCancel}></div>
<div class="dialog-container">
  <div class="dialog">
    <h3 class="dialog-title">Block {peerName}?</h3>
    <p class="dialog-description">
      This will hide all messages and mute voice audio from this peer.
    </p>
    <div class="dialog-actions">
      <button class="btn btn-secondary" onclick={handleCancel}>
        Cancel
      </button>
      <button class="btn btn-danger" onclick={handleConfirm}>
        Block
      </button>
    </div>
  </div>
</div>

<style>
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    z-index: 999;
  }

  .dialog-container {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 1000;
  }

  .dialog {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 1.5rem;
    min-width: 300px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
  }

  .dialog-title {
    font-family: var(--font-mono);
    font-size: 1rem;
    color: var(--text-primary);
    margin: 0 0 0.75rem 0;
  }

  .dialog-description {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-muted);
    margin: 0 0 1.5rem 0;
    line-height: 1.5;
  }

  .dialog-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
  }

  .btn {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    padding: 0.5rem 1rem;
    border-radius: 3px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-secondary {
    background: transparent;
    border: 1px solid var(--border-color);
    color: var(--text-secondary);
  }

  .btn-secondary:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .btn-danger {
    background: #ff4444;
    border: 1px solid #ff4444;
    color: white;
  }

  .btn-danger:hover {
    background: #ff6666;
    border-color: #ff6666;
  }
</style>
