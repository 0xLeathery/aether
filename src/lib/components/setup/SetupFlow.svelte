<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Welcome from './Welcome.svelte';
  import SetupName from './SetupName.svelte';
  import GenerateKey from './GenerateKey.svelte';
  import SetupComplete from './SetupComplete.svelte';

  const dispatch = createEventDispatcher();

  type Step = 'welcome' | 'name' | 'generate' | 'complete';

  let currentStep = $state<Step>('welcome');
  let displayName = $state('');
  let identity = $state<any>(null);

  function handleWelcomeNext() {
    currentStep = 'name';
  }

  function handleNameNext(event: CustomEvent<{ displayName: string }>) {
    displayName = event.detail.displayName;
    currentStep = 'generate';
  }

  function handleGenerateComplete(event: CustomEvent<{ identity: any }>) {
    identity = event.detail.identity;
    currentStep = 'complete';
  }

  function handleEnter() {
    dispatch('setup-complete');
  }
</script>

<div class="setup-flow">
  <div class="step-container" class:visible={currentStep === 'welcome'}>
    {#if currentStep === 'welcome'}
      <Welcome on:next={handleWelcomeNext} />
    {/if}
  </div>

  <div class="step-container" class:visible={currentStep === 'name'}>
    {#if currentStep === 'name'}
      <SetupName on:next={handleNameNext} />
    {/if}
  </div>

  <div class="step-container" class:visible={currentStep === 'generate'}>
    {#if currentStep === 'generate'}
      <GenerateKey {displayName} on:complete={handleGenerateComplete} />
    {/if}
  </div>

  <div class="step-container" class:visible={currentStep === 'complete'}>
    {#if currentStep === 'complete' && identity}
      <SetupComplete {identity} on:enter={handleEnter} />
    {/if}
  </div>
</div>

<style>
  .setup-flow {
    position: relative;
    width: 100%;
    min-height: 100vh;
  }

  .step-container {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    min-height: 100vh;
    opacity: 0;
    transition: opacity 0.3s ease;
    pointer-events: none;
  }

  .step-container.visible {
    opacity: 1;
    pointer-events: all;
  }
</style>
