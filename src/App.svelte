<script lang="ts">
  import { onMount } from 'svelte';
  import { identityStore } from './lib/stores/identity';
  import SetupFlow from './lib/components/setup/SetupFlow.svelte';
  import AppShell from './lib/components/layout/AppShell.svelte';

  let appState = $state<'loading' | 'setup' | 'app'>('loading');

  onMount(async () => {
    await identityStore.initialize();

    if (identityStore.identity) {
      appState = 'app';
    } else {
      appState = 'setup';
    }
  });

  function handleSetupComplete() {
    appState = 'app';
  }
</script>

{#if appState === 'loading'}
  <div class="loading-screen">
    <div class="loading-text">Initializing<span class="cursor">_</span></div>
  </div>
{:else if appState === 'setup'}
  <SetupFlow on:setup-complete={handleSetupComplete} />
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
