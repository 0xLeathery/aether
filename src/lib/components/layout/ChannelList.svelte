<script lang="ts">
  import { swarmStore } from '../../stores/swarm.svelte';
  import { chatStore } from '../../stores/chat.svelte';
  import CreateChannelDialog from '../channel/CreateChannelDialog.svelte';
  import ChannelContextMenu from '../channel/ChannelContextMenu.svelte';
  import DeleteChannelDialog from '../channel/DeleteChannelDialog.svelte';

  let showCreateDialog = $state(false);
  let contextMenu = $state<{ x: number; y: number; channelId: string; channelName: string } | null>(null);
  let deleteTarget = $state<{ id: string; name: string } | null>(null);
  let editingChannelId = $state<string | null>(null);
  let editingName = $state('');

  let sortedChannels = $derived.by(() => {
    if (!swarmStore.activeSwarm) return [];
    const channels = [...swarmStore.activeSwarm.channels];
    return channels.sort((a, b) => {
      if (a.id === 'general') return -1;
      if (b.id === 'general') return 1;
      if (a.id === 'voice') return -1;
      if (b.id === 'voice') return 1;
      return a.name.localeCompare(b.name);
    });
  });

  function handleChannelClick(channel: { id: string; name: string }) {
    // Voice channel does not switch chat view
    if (channel.id === 'voice') return;
    swarmStore.selectChannel(channel.id);
    // Also tell chatStore to switch
    if (swarmStore.activeSwarm) {
      chatStore.switchChannel(swarmStore.activeSwarm.id, channel.id);
    }
  }

  function handleContextMenu(event: MouseEvent, channel: { id: string; name: string }) {
    if (channel.id === 'general' || channel.id === 'voice') return;
    if (!swarmStore.isCreator) return;
    event.preventDefault();
    contextMenu = {
      x: event.clientX,
      y: event.clientY,
      channelId: channel.id,
      channelName: channel.name,
    };
  }

  function startRename(channelId: string, currentName: string) {
    editingChannelId = channelId;
    editingName = currentName;
    contextMenu = null;
  }

  async function confirmRename() {
    if (!editingChannelId || !editingName.trim()) return;
    try {
      await swarmStore.renameChannel(editingChannelId, editingName.trim());
    } catch (err) {
      console.error('Rename failed:', err);
    }
    editingChannelId = null;
    editingName = '';
  }

  function cancelRename() {
    editingChannelId = null;
    editingName = '';
  }

  function startDelete(channelId: string, channelName: string) {
    deleteTarget = { id: channelId, name: channelName };
    contextMenu = null;
  }

  async function confirmDelete() {
    if (!deleteTarget) return;
    await swarmStore.deleteChannel(deleteTarget.id);
    deleteTarget = null;
  }

  async function handleCreate(name: string) {
    await swarmStore.createChannel(name);
  }
</script>

<div class="channel-list">
  <div class="header">
    <div class="header-row">
      <h2>CHANNELS</h2>
      {#if swarmStore.isCreator}
        <button class="add-btn" onclick={() => showCreateDialog = true} title="Create channel">[+]</button>
      {/if}
    </div>
    {#if swarmStore.activeSwarm}
      <p class="swarm-name">{swarmStore.activeSwarm.name}</p>
    {/if}
  </div>

  {#if swarmStore.activeSwarm}
    <div class="channels">
      {#each sortedChannels as channel (channel.id)}
        {#if editingChannelId === channel.id}
          <!-- Inline rename input -->
          <div class="channel-edit">
            <span class="channel-hash">#</span>
            <input
              type="text"
              bind:value={editingName}
              onkeydown={(e) => { if (e.key === 'Enter') confirmRename(); if (e.key === 'Escape') cancelRename(); }}
              autofocus
              class="rename-input"
            />
          </div>
        {:else}
          <button
            class="channel-button"
            class:active={swarmStore.activeChannelId === channel.id}
            class:voice-channel={channel.id === 'voice'}
            onclick={() => handleChannelClick(channel)}
            oncontextmenu={(e) => handleContextMenu(e, channel)}
          >
            <span class="channel-hash">#</span>
            <span class="channel-name">{channel.name}</span>
          </button>
        {/if}
      {/each}
    </div>
  {:else}
    <div class="empty-state">
      <p>Join a swarm to see channels</p>
    </div>
  {/if}
</div>

{#if contextMenu}
  <ChannelContextMenu
    x={contextMenu.x}
    y={contextMenu.y}
    channelId={contextMenu.channelId}
    channelName={contextMenu.channelName}
    onRename={() => startRename(contextMenu!.channelId, contextMenu!.channelName)}
    onDelete={() => startDelete(contextMenu!.channelId, contextMenu!.channelName)}
    onClose={() => contextMenu = null}
  />
{/if}

{#if showCreateDialog}
  <CreateChannelDialog
    onClose={() => showCreateDialog = false}
    onCreate={handleCreate}
  />
{/if}

{#if deleteTarget}
  <DeleteChannelDialog
    channelName={deleteTarget.name}
    onConfirm={confirmDelete}
    onCancel={() => deleteTarget = null}
  />
{/if}

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

  .header-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .header h2 {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-muted);
    letter-spacing: 0.15em;
    font-weight: 500;
    margin: 0;
  }

  .add-btn {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-muted);
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0;
  }

  .add-btn:hover {
    color: var(--accent-primary);
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

  .channel-button.voice-channel {
    opacity: 0.6;
  }

  .channel-button.voice-channel .channel-hash {
    color: var(--accent-amber, #ffb000);
  }

  .channel-hash {
    font-weight: bold;
    opacity: 0.7;
  }

  .channel-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .channel-edit {
    width: 100%;
    padding: 0.65rem 0.75rem;
    margin-bottom: 0.25rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .rename-input {
    flex: 1;
    font-family: var(--font-mono);
    font-size: 0.9rem;
    background: var(--bg-secondary);
    border: 1px solid var(--accent-primary);
    color: var(--text-primary);
    padding: 0.25rem 0.5rem;
    border-radius: 2px;
    outline: none;
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
