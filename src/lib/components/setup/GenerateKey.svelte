<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { identityStore } from '../../stores/identity';

  const dispatch = createEventDispatcher();

  let { displayName } = $props<{ displayName: string }>();

  let state = $state<'ready' | 'generating' | 'success' | 'error'>('ready');
  let generatedIdentity = $state<any>(null);

  async function handleGenerate() {
    state = 'generating';
    try {
      await identityStore.create(displayName);
      generatedIdentity = identityStore.identity;
      state = 'success';
    } catch (err) {
      state = 'error';
      console.error('Key generation failed:', err);
    }
  }

  function handleRetry() {
    state = 'ready';
  }

  function handleContinue() {
    dispatch('complete', { identity: generatedIdentity });
  }
</script>

<div class="generate-key">
  <div class="content">
    <h1>Generate Your Identity</h1>

    {#if state === 'ready'}
      <p class="subtext">Click below to generate your Ed25519 cryptographic keypair.</p>
      <button class="generate-btn" onclick={handleGenerate}>
        Generate Keypair
      </button>
    {:else if state === 'generating'}
      <div class="loading">
        <div class="spinner">Generating keypair<span class="cursor">_</span></div>
        <p class="loading-text">Creating your sovereign identity...</p>
      </div>
    {:else if state === 'success' && generatedIdentity}
      <div class="success">
        <p class="label">Your identity:</p>
        <div class="identity-box">
          <code class="short-id">{generatedIdentity.short_id}</code>
        </div>
        <p class="success-text">Identity created and stored securely.</p>
        <button class="continue-btn" onclick={handleContinue}>
          Continue
        </button>
      </div>
    {:else if state === 'error'}
      <div class="error">
        <p class="error-title">Key Generation Failed</p>
        <div class="error-box">
          <p class="error-message">{identityStore.error || 'An unknown error occurred'}</p>
          <p class="error-help">
            This usually means the system keychain is unavailable. On macOS, ensure Keychain Access is enabled.
            On Linux, ensure a keyring service is running (gnome-keyring or KDE Wallet).
            On Windows, Credential Manager should be available by default.
          </p>
        </div>
        <button class="retry-btn" onclick={handleRetry}>
          Retry
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .generate-key {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    padding: 2rem;
  }

  .content {
    max-width: 600px;
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

  .generate-btn,
  .continue-btn,
  .retry-btn {
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

  .generate-btn:hover,
  .continue-btn:hover,
  .retry-btn:hover {
    background: var(--accent-primary);
    color: var(--bg-primary);
    box-shadow: 0 0 20px rgba(0, 255, 65, 0.3);
  }

  .loading {
    text-align: center;
    padding: 2rem 0;
  }

  .spinner {
    font-family: var(--font-mono);
    font-size: 1.2rem;
    color: var(--accent-primary);
    margin-bottom: 1rem;
  }

  .cursor {
    animation: blink 1s infinite;
  }

  @keyframes blink {
    0%, 50% { opacity: 1; }
    51%, 100% { opacity: 0; }
  }

  .loading-text {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .success {
    text-align: center;
  }

  .label {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin-bottom: 0.5rem;
  }

  .identity-box {
    background: var(--bg-secondary);
    border: 2px solid var(--accent-primary);
    padding: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .short-id {
    font-family: var(--font-mono);
    font-size: 1.3rem;
    color: var(--accent-primary);
    letter-spacing: 0.05em;
  }

  .success-text {
    font-family: var(--font-mono);
    font-size: 0.9rem;
    color: var(--text-secondary);
    margin-bottom: 2rem;
  }

  .error {
    text-align: center;
  }

  .error-title {
    font-family: var(--font-mono);
    font-size: 1.2rem;
    color: #ff4444;
    margin-bottom: 1rem;
  }

  .error-box {
    background: var(--bg-secondary);
    border: 2px solid #ff4444;
    padding: 1.5rem;
    margin-bottom: 2rem;
    text-align: left;
  }

  .error-message {
    font-family: var(--font-mono);
    font-size: 1rem;
    color: #ff4444;
    margin-bottom: 1rem;
  }

  .error-help {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-secondary);
    line-height: 1.6;
  }

  .retry-btn {
    border-color: #ff4444;
    color: #ff4444;
  }

  .retry-btn:hover {
    background: #ff4444;
    color: var(--bg-primary);
    box-shadow: 0 0 20px rgba(255, 68, 68, 0.3);
  }
</style>
