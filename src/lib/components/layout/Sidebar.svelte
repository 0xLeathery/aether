<script lang="ts">
  import Avatar from '../profile/Avatar.svelte';
  import ProfilePopover from '../profile/ProfilePopover.svelte';
  import PeerList from '../peers/PeerList.svelte';
  import SwarmSelector from '../swarm/SwarmSelector.svelte';
  import InviteDialog from '../swarm/InviteDialog.svelte';
  import JoinDialog from '../swarm/JoinDialog.svelte';
  import SwarmSettings from '../swarm/SwarmSettings.svelte';
  import { networkStore } from '../../stores/network.svelte';
  import { swarmStore } from '../../stores/swarm.svelte';

  let { identity }: { identity: any } = $props();

  let showProfile = $state(false);
  let showInvite = $state(false);
  let showJoin = $state(false);
  let showSwarmSettings = $state(false);

  function toggleProfile() {
    showProfile = !showProfile;
  }
</script>

<div class="sidebar">
  <div class="header">
    <h2>PEERS</h2>
  </div>

  <PeerList peers={networkStore.peers} />

  <div class="network-status">
    <div class="status-indicator" class:online={networkStore.status?.running}>
      <div class="status-dot"></div>
      <span class="status-text">{networkStore.status?.running ? 'online' : 'offline'}</span>
    </div>
  </div>

  <div class="header swarms-header">
    <h2>SWARMS</h2>
    {#if swarmStore.activeSwarm}
      <button class="settings-button" onclick={() => showSwarmSettings = true} title="Swarm Settings">
        [*]
      </button>
    {/if}
  </div>

  <SwarmSelector
    swarms={swarmStore.swarms}
    activeSwarmId={swarmStore.activeSwarm?.id ?? null}
    onSelect={(id) => swarmStore.selectSwarm(id)}
    onCreateClick={() => showInvite = true}
    onJoinClick={() => showJoin = true}
  />

  <div class="profile-section">
    <button class="profile-trigger" onclick={toggleProfile}>
      <Avatar publicKeyHex={identity.public_key_hex} size={32} />
      <span class="display-name">{identity.display_name}</span>
    </button>
  </div>

  <ProfilePopover bind:isOpen={showProfile} />
  <InviteDialog bind:open={showInvite} onClose={() => showInvite = false} />
  <JoinDialog bind:open={showJoin} onClose={() => showJoin = false} />

  {#if showSwarmSettings && swarmStore.activeSwarm}
    <SwarmSettings
      swarm={swarmStore.activeSwarm}
      onClose={() => showSwarmSettings = false}
    />
  {/if}
</div>

<style>
  .sidebar {
    width: 240px;
    background: var(--bg-secondary);
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
  }

  .swarms-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .settings-button {
    font-family: var(--font-mono);
    font-size: 0.8rem;
    color: var(--text-muted);
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 0.15rem 0.35rem;
    border-radius: 3px;
    transition: all 0.2s ease;
  }

  .settings-button:hover {
    color: var(--text-primary);
    background: var(--bg-tertiary);
  }

  .profile-section {
    border-top: 1px solid var(--border-color);
    padding: 1rem;
  }

  .profile-trigger {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    background: transparent;
    border: none;
    padding: 0.5rem;
    cursor: pointer;
    transition: background 0.2s ease;
    border-radius: 4px;
  }

  .profile-trigger:hover {
    background: var(--bg-tertiary);
  }

  .display-name {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .network-status {
    padding: 0.75rem 1rem;
    border-top: 1px solid var(--border-color);
    border-bottom: 1px solid var(--border-color);
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #ff4444;
    transition: background 0.3s ease;
  }

  .status-indicator.online .status-dot {
    background: var(--accent-primary);
  }

  .status-text {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.1em;
  }
</style>
