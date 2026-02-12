<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  let displayName = $state('');
  let error = $state('');

  function handleContinue() {
    const trimmed = displayName.trim();
    if (!trimmed) {
      error = 'Display name is required';
      return;
    }
    error = '';
    dispatch('next', { displayName: trimmed });
  }

  function handleInput() {
    if (error && displayName.trim()) {
      error = '';
    }
  }
</script>

<div class="setup-name">
  <div class="content">
    <h1>Choose a Display Name</h1>
    <p class="subtext">This is how others will see you. You can change it anytime.</p>

    <div class="input-group">
      <input
        type="text"
        bind:value={displayName}
        on:input={handleInput}
        on:keydown={(e) => e.key === 'Enter' && handleContinue()}
        placeholder="Enter your display name"
        class:has-error={error}
        autofocus
      />
      {#if error}
        <span class="error-message">{error}</span>
      {/if}
    </div>

    <button
      class="continue-btn"
      on:click={handleContinue}
      disabled={!displayName.trim()}
    >
      Continue
    </button>
  </div>
</div>

<style>
  .setup-name {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    padding: 2rem;
  }

  .content {
    max-width: 500px;
    width: 100%;
  }

  h1 {
    font-family: var(--font-mono);
    font-size: 1.75rem;
    color: var(--accent-primary);
    margin-bottom: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .subtext {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin-bottom: 2rem;
  }

  .input-group {
    margin-bottom: 2rem;
  }

  input {
    width: 100%;
    font-family: var(--font-mono);
    font-size: 1.1rem;
    padding: 1rem;
    background: var(--bg-secondary);
    border: 2px solid var(--border-color);
    color: var(--text-primary);
    outline: none;
    transition: border-color 0.2s ease;
  }

  input:focus {
    border-color: var(--accent-primary);
  }

  input.has-error {
    border-color: #ff4444;
  }

  .error-message {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: #ff4444;
    display: block;
    margin-top: 0.5rem;
  }

  .continue-btn {
    font-family: var(--font-mono);
    font-size: 1rem;
    padding: 0.75rem 2rem;
    background: transparent;
    border: 2px solid var(--accent-primary);
    color: var(--accent-primary);
    cursor: pointer;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    transition: all 0.2s ease;
  }

  .continue-btn:hover:not(:disabled) {
    background: var(--accent-primary);
    color: var(--bg-primary);
    box-shadow: 0 0 20px rgba(0, 255, 65, 0.3);
  }

  .continue-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }
</style>
