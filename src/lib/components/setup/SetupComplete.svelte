<script lang="ts">
  let { identity, onEnter }: { identity: any; onEnter: () => void } = $props();

  const platform = navigator.platform.toLowerCase();
  const keychainName = platform.includes('mac') ? 'macOS Keychain' :
    platform.includes('win') ? 'Windows Credential Manager' :
    platform.includes('linux') ? 'system keyring' :
    'system keychain';
</script>

<div class="setup-complete">
  <div class="content">
    <h1>Identity Created</h1>

    <div class="identity-display">
      <p class="display-name">{identity.display_name}</p>
      <code class="short-id">{identity.short_id}</code>
    </div>

    <p class="note">
      Your key is stored in {keychainName}. It will load automatically next time.
    </p>

    <button class="enter-btn" onclick={onEnter}>
      Enter Aether
    </button>
  </div>
</div>

<style>
  .setup-complete {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    padding: 2rem;
  }

  .content {
    max-width: 600px;
    width: 100%;
    text-align: center;
  }

  h1 {
    font-family: var(--font-mono);
    font-size: 2rem;
    color: var(--accent-primary);
    margin-bottom: 2.5rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .identity-display {
    background: var(--bg-secondary);
    border: 2px solid var(--accent-primary);
    padding: 2rem;
    margin-bottom: 2rem;
  }

  .display-name {
    font-family: var(--font-mono);
    font-size: 1.5rem;
    color: var(--text-primary);
    margin-bottom: 1rem;
  }

  .short-id {
    font-family: var(--font-mono);
    font-size: 1rem;
    color: var(--text-secondary);
    letter-spacing: 0.05em;
  }

  .note {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-secondary);
    margin-bottom: 2rem;
    line-height: 1.6;
  }

  .enter-btn {
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

  .enter-btn:hover {
    background: var(--accent-primary);
    color: var(--bg-primary);
    box-shadow: 0 0 20px rgba(0, 255, 65, 0.3);
  }
</style>
