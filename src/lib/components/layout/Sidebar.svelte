<script lang="ts">
  import Avatar from '../profile/Avatar.svelte';
  import ProfilePopover from '../profile/ProfilePopover.svelte';
  import PeerList from '../peers/PeerList.svelte';
  import { networkStore } from '../../stores/network.svelte';

  let { identity }: { identity: any } = $props();

  let showProfile = $state(false);

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

  <div class="header">
    <h2>SWARMS</h2>
  </div>

  <div class="empty-state">
    <p>No swarms yet</p>
    <span class="hint">Swarms enable mesh collaboration</span>
  </div>

  <div class="profile-section">
    <button class="profile-trigger" onclick={toggleProfile}>
      <Avatar publicKeyHex={identity.public_key_hex} size={32} />
      <span class="display-name">{identity.display_name}</span>
    </button>
  </div>

  <ProfilePopover bind:isOpen={showProfile} />
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

  .empty-state {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem 1rem;
    text-align: center;
  }

  .empty-state p {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin-bottom: 0.5rem;
  }

  .empty-state .hint {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-muted);
    line-height: 1.4;
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
