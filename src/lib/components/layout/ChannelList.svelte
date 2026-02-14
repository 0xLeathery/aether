<script lang="ts">
  import { swarmStore } from '../../stores/swarm.svelte';
</script>

<div class="channel-list">
  <div class="header">
    <h2>CHANNELS</h2>
    {#if swarmStore.activeSwarm}
      <p class="swarm-name">{swarmStore.activeSwarm.name}</p>
    {/if}
  </div>

  {#if swarmStore.activeSwarm}
    <div class="channels">
      {#each swarmStore.activeSwarm.channels as channel}
        <button class="channel-button" class:active={channel.id === 'general'}>
          <span class="channel-hash">#</span> {channel.name}
        </button>
      {/each}
    </div>
  {:else}
    <div class="empty-state">
      <p>Join a swarm to see channels</p>
    </div>
  {/if}
</div>

<style>
  .channel-list {
    width: 200px;
    background: var(--bg-primary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  .header {
    padding: 1rem;
    border-bottom: 1px solid var(--border-color);
  }

  .header h2 {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-muted);
    letter-spacing: 0.15em;
    font-weight: 500;
    margin: 0;
  }

  .swarm-name {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-secondary);
    margin-top: 0.25rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .channels {
    padding: 0.5rem;
    overflow-y: auto;
  }

  .channel-button {
    width: 100%;
    font-family: var(--font-mono);
    font-size: 0.9rem;
    padding: 0.65rem 0.75rem;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 4px;
    color: var(--text-secondary);
    text-align: left;
    cursor: pointer;
    transition: all 0.2s ease;
    margin-bottom: 0.25rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .channel-button:hover {
    background: var(--bg-secondary);
    color: var(--text-primary);
  }

  .channel-button.active {
    background: var(--bg-secondary);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .channel-hash {
    font-weight: bold;
    opacity: 0.7;
  }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem 1rem;
    text-align: center;
  }

  .empty-state p {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-secondary);
    line-height: 1.5;
  }
</style>
