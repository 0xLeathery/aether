<script lang="ts">
  import Avatar from '../profile/Avatar.svelte';

  let { identity }: { identity: any } = $props();
</script>

<div class="main-content">
  <div class="welcome-display">
    <Avatar publicKeyHex={identity.public_key_hex} size={100} />
    <h1>Welcome to Aether</h1>
    <code class="short-id">{identity.short_id}</code>
    <p class="tagline">The Sovereign Node</p>
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
</style>
