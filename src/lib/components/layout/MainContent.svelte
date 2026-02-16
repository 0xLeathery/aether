<script lang="ts">
  import { onMount } from 'svelte';
  import Avatar from '../profile/Avatar.svelte';
  import VoicePanel from '../voice/VoicePanel.svelte';
  import { voiceStore } from '../../stores/voice.svelte';
  import { swarmStore } from '../../stores/swarm.svelte';

  let { identity }: { identity: any } = $props();

  onMount(() => {
    voiceStore.initialize();
  });
</script>

<div class="main-content">
  <div class="welcome-display">
    <Avatar publicKeyHex={identity.public_key_hex} size={100} />
    <h1>Welcome to Aether</h1>
    <code class="short-id">{identity.short_id}</code>
    <p class="tagline">The Sovereign Node</p>

    {#if swarmStore.activeSwarm}
      <div class="voice-section">
        <VoicePanel identity={identity} />
      </div>
    {/if}
  </div>
</div>

<style>
  .main-content {
    flex: 1;
    background: var(--bg-primary);
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
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

  .voice-section {
    margin-top: 2rem;
    width: 100%;
    max-width: 400px;
  }
</style>
