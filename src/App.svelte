<script lang="ts">
  import { onMount } from 'svelte';
  import { identityStore } from './lib/stores/identity.svelte';
  import { networkStore } from './lib/stores/network.svelte';
  import { swarmStore } from './lib/stores/swarm.svelte';
  import { migrateChannelMetadata } from './lib/tauri';
  import SetupFlow from './lib/components/setup/SetupFlow.svelte';
  import AppShell from './lib/components/layout/AppShell.svelte';

  let appState = $state<'loading' | 'setup' | 'app'>('loading');

  onMount(async () => {
    await identityStore.initialize();

    if (identityStore.identity) {
      // Set local identity for isCreator evaluation before any swarm data loads
      swarmStore.setLocalIdentity(identityStore.identity.public_key_hex);

      // Identity exists, initialize network and swarms
      await networkStore.initialize();
      await networkStore.start();

      // Load swarms, then activate default swarm (defers network restart until after Tokio runtime ready)
      await swarmStore.initialize();
      await migrateChannelMetadata();
      await swarmStore.activateDefaultSwarm();

      appState = 'app';
    } else {
      appState = 'setup';
    }
  });

  async function handleSetupComplete() {
    // Set local identity for isCreator evaluation (identity was just created)
    swarmStore.setLocalIdentity(identityStore.identity!.public_key_hex);

    // After identity creation, start the network and initialize swarms
    await networkStore.start();

    // Load swarms, then activate default swarm (defers network restart until after Tokio runtime ready)
    await swarmStore.initialize();
    await migrateChannelMetadata();
    await swarmStore.activateDefaultSwarm();

    appState = 'app';
  }
</script>

{#if appState === 'loading'}
  <div class="loading-screen">
    <div class="loading-text">Initializing<span class="cursor">_</span></div>
  </div>
{:else if appState === 'setup'}
  <SetupFlow onSetupComplete={handleSetupComplete} />
{:else if appState === 'app' && identityStore.identity}
  <AppShell identity={identityStore.identity} />
{/if}

<style>
  .loading-screen {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    background: var(--bg-primary);
  }

  .loading-text {
    font-family: var(--font-mono);
    font-size: 1.2rem;
    color: var(--accent-primary);
  }

  .cursor {
    animation: blink 1s infinite;
  }

  @keyframes blink {
    0%, 50% { opacity: 1; }
    51%, 100% { opacity: 0; }
  }
</style>
