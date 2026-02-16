<script lang="ts">
  import { onMount } from 'svelte';
  import Avatar from '../profile/Avatar.svelte';
  import VoicePanel from '../voice/VoicePanel.svelte';
  import ChatPanel from '../chat/ChatPanel.svelte';
  import { voiceStore } from '../../stores/voice.svelte';
  import { swarmStore } from '../../stores/swarm.svelte';

  let { identity }: { identity: any } = $props();

  onMount(() => {
    voiceStore.initialize();
  });

  // Use the store's activeChannelId (set by ChannelList on click)
  let activeChannelId = $derived(swarmStore.activeChannelId);
</script>

{#if swarmStore.activeSwarm && activeChannelId}
  <div class="main-content main-content--chat">
    <div class="voice-bar">
      <VoicePanel identity={identity} />
    </div>
    <div class="chat-area">
      <ChatPanel
        swarmId={swarmStore.activeSwarm.id}
        channelId={activeChannelId}
        currentUserKey={identity.public_key_hex}
      />
    </div>
  </div>
{:else}
  <div class="main-content main-content--welcome">
    <div class="welcome-display">
      <Avatar publicKeyHex={identity.public_key_hex} size={100} />
      <h1>Welcome to Aether</h1>
      <code class="short-id">{identity.short_id}</code>
      <p class="tagline">The Sovereign Node</p>
    </div>
  </div>
{/if}

<style>
  .main-content {
    flex: 1;
    background: var(--bg-primary);
    display: flex;
    height: 100vh;
  }

  .main-content--welcome {
    align-items: center;
    justify-content: center;
  }

  .main-content--chat {
    flex-direction: column;
    overflow: hidden;
  }

  .voice-bar {
    flex-shrink: 0;
    border-bottom: 1px solid var(--border-color);
  }

  .voice-bar :global(.voice-panel) {
    max-width: none;
    margin: 0;
    border: none;
    border-radius: 0;
    padding: 0.5rem 1rem;
  }

  .chat-area {
    flex: 1;
    display: flex;
    min-height: 0;
    border-top: 1px solid var(--accent-primary);
  }

  .welcome-display {
    text-align: center;
  }

  .welcome-display :global(canvas) {
    margin: 0 auto 1.5rem;
  }

  h1 {
    font-family: var(--font-mono);
    font-size: 2rem;
    color: var(--accent-primary);
    margin-bottom: 0.75rem;
    letter-spacing: 0.05em;
  }

  .short-id {
    font-family: var(--font-mono);
    font-size: 1rem;
    color: var(--text-secondary);
    letter-spacing: 0.05em;
    display: block;
    margin-bottom: 1rem;
  }

  .tagline {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.2em;
  }
</style>
