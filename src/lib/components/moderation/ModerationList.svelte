<script lang="ts">
  import { moderationStore } from '../../stores/moderation.svelte';
  import { contactsStore } from '../../stores/contacts.svelte';
  import type { ModerationTier } from '../../tauri';

  let { onClose }: { onClose: () => void } = $props();

  function truncateKey(key: string): string {
    return key.substring(0, 8) + '...';
  }

  function resolveName(publicKey: string): string {
    return contactsStore.resolveName(publicKey, truncateKey(publicKey));
  }

  function tierLabel(tier: ModerationTier): string {
    return tier.toUpperCase();
  }

  function changeTier(publicKey: string, tier: ModerationTier) {
    moderationStore.setTier(publicKey, tier);
  }

  function removePeer(publicKey: string) {
    moderationStore.removeTier(publicKey);
  }

  function hasOverrides(overrides: Record<string, unknown>): boolean {
    return Object.keys(overrides).length > 0;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      onClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="moderation-list" role="dialog" aria-label="Moderated peers">
  <div class="moderation-header">
    <span class="moderation-title">MODERATED PEERS</span>
    <button class="close-btn" onclick={onClose}>[ CLOSE ]</button>
  </div>

  {#if moderationStore.loading}
    <div class="empty-state">
      <span class="loading-text">Loading...</span>
    </div>
  {:else if moderationStore.getAllEntries().length === 0}
    <div class="empty-state">
      <span class="empty-text">No moderated peers</span>
    </div>
  {:else}
    <div class="moderation-rows">
      {#each moderationStore.getAllEntries() as [publicKey, entry]}
        <div class="moderation-row">
          <div class="peer-info">
            <span class="peer-name">{resolveName(publicKey)}</span>
            <span class="tier-badge tier-badge--{entry.tier}">{tierLabel(entry.tier)}</span>
            {#if hasOverrides(entry.swarm_overrides)}
              <span class="overrides-indicator">(overrides)</span>
            {/if}
          </div>
          <div class="tier-controls">
            <button
              class="tier-btn"
              class:active={entry.tier === 'mute'}
              onclick={() => changeTier(publicKey, 'mute')}
              title="Mute"
            >M</button>
            <button
              class="tier-btn"
              class:active={entry.tier === 'hide'}
              onclick={() => changeTier(publicKey, 'hide')}
              title="Hide"
            >H</button>
            <button
              class="tier-btn tier-btn--block"
              class:active={entry.tier === 'block'}
              onclick={() => changeTier(publicKey, 'block')}
              title="Block"
            >B</button>
            <button
              class="remove-btn"
              onclick={() => removePeer(publicKey)}
              title="Remove moderation"
            >[x]</button>
          </div>
        </div>
      {/each}
    </div>
  {/if}

  {#if moderationStore.error}
    <div class="moderation-error">{moderationStore.error}</div>
  {/if}
</div>

<style>
  .moderation-list {
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

  .moderation-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    border-bottom: 1px solid var(--border-color);
    flex-shrink: 0;
  }

  .moderation-title {
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

  .moderation-rows {
    overflow-y: auto;
    flex: 1;
  }

  .moderation-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.625rem 1rem;
    border-bottom: 1px solid var(--border-color);
    transition: background 0.2s ease;
  }

  .moderation-row:hover {
    background: var(--bg-tertiary);
  }

  .peer-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
    flex: 1;
  }

  .peer-name {
    font-size: 0.85rem;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .tier-badge {
    font-size: 0.65rem;
    padding: 0.1rem 0.35rem;
    border-radius: 2px;
    letter-spacing: 0.1em;
    font-weight: bold;
    flex-shrink: 0;
  }

  .tier-badge--mute {
    color: var(--text-muted);
    border: 1px solid var(--text-muted);
  }

  .tier-badge--hide {
    color: #e6a817;
    border: 1px solid #e6a817;
  }

  .tier-badge--block {
    color: #ff4444;
    border: 1px solid #ff4444;
  }

  .overrides-indicator {
    font-size: 0.65rem;
    color: var(--text-muted);
    font-style: italic;
    flex-shrink: 0;
  }

  .tier-controls {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    flex-shrink: 0;
    margin-left: 0.5rem;
  }

  .tier-btn {
    font-family: var(--font-mono);
    font-size: 0.7rem;
    width: 1.5rem;
    height: 1.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: 2px;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .tier-btn:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .tier-btn.active {
    background: var(--bg-tertiary);
    color: var(--accent-primary);
    border-color: var(--accent-primary);
  }

  .tier-btn--block.active {
    color: #ff4444;
    border-color: #ff4444;
  }

  .remove-btn {
    font-family: var(--font-mono);
    font-size: 0.7rem;
    padding: 0.15rem 0.3rem;
    background: transparent;
    border: 1px solid var(--border-color);
    border-radius: 2px;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 0.2s ease;
    margin-left: 0.25rem;
  }

  .remove-btn:hover {
    color: #ff4444;
    border-color: #ff4444;
    background: rgba(255, 68, 68, 0.1);
  }

  .moderation-error {
    padding: 0.75rem 1rem;
    font-size: 0.8rem;
    color: #ff4444;
    background: rgba(255, 68, 68, 0.1);
    border-top: 1px solid #ff4444;
  }
</style>
