<script lang="ts">
  import type { SwarmMetadata } from '../../tauri';

  let {
    swarms,
    activeSwarmId,
    onSelect,
    onCreateClick,
    onJoinClick
  }: {
    swarms: SwarmMetadata[];
    activeSwarmId: string | null;
    onSelect: (swarmId: string) => void;
    onCreateClick: () => void;
    onJoinClick: () => void;
  } = $props();
</script>

<div class="swarm-selector">
  {#if swarms.length === 0}
    <div class="empty-state">
      <p class="hint">Create or join a swarm</p>
    </div>
  {:else}
    <div class="swarm-list">
      {#each swarms as swarm}
        <button
          class="swarm-button"
          class:active={swarm.id === activeSwarmId}
          onclick={() => onSelect(swarm.id)}
        >
          {swarm.name}
        </button>
      {/each}
    </div>
  {/if}

  <div class="actions">
    <button class="action-button" onclick={onCreateClick}>
      <span class="icon">+</span> Create
    </button>
    <button class="action-button" onclick={onJoinClick}>
      Join
    </button>
  </div>
</div>

<style>
  .swarm-selector {
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem 1rem;
  }

  .hint {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    color: var(--text-muted);
    text-align: center;
    line-height: 1.4;
  }

  .swarm-list {
    flex: 1;
    padding: 0.5rem;
    overflow-y: auto;
  }

  .swarm-button {
    width: 100%;
    font-family: var(--font-mono);
    font-size: 0.9rem;
    padding: 0.75rem 1rem;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 4px;
    color: var(--text-secondary);
    text-align: left;
    cursor: pointer;
    transition: all 0.2s ease;
    margin-bottom: 0.25rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .swarm-button:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .swarm-button.active {
    background: rgba(0, 255, 65, 0.1);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .actions {
    border-top: 1px solid var(--border-color);
    padding: 0.75rem;
    display: flex;
    gap: 0.5rem;
  }

  .action-button {
    flex: 1;
    font-family: var(--font-mono);
    font-size: 0.8rem;
    padding: 0.65rem 0.75rem;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.25rem;
  }

  .action-button:hover {
    background: var(--bg-primary);
    border-color: var(--accent-primary);
    color: var(--accent-primary);
  }

  .icon {
    font-size: 1rem;
    font-weight: bold;
  }
</style>
