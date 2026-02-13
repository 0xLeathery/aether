<script lang="ts">
  let { onNext }: { onNext: () => void } = $props();
</script>

<div class="welcome">
  <div class="content">
    <h1>Your Sovereign Node</h1>
    <div class="explainer">
      <p>
        We generate an Ed25519 cryptographic keypair stored in your device's secure enclave.
        This key IS your identity — no email, no password, no server.
      </p>
      <p>
        You own it completely. No central authority can revoke it, disable it, or delete your data.
        Your node, your rules.
      </p>
      <p>
        Everything that follows happens on your machine. We're about to generate your sovereign identity.
      </p>
    </div>
    <button class="continue-btn" onclick={onNext}>
      Continue
    </button>
  </div>
</div>

<style>
  .welcome {
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
    font-size: 2rem;
    color: var(--accent-primary);
    margin-bottom: 2rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .explainer {
    margin-bottom: 2.5rem;
  }

  .explainer p {
    font-family: var(--font-mono);
    font-size: 0.95rem;
    line-height: 1.6;
    color: var(--text-primary);
    margin-bottom: 1rem;
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

  .continue-btn:hover {
    background: var(--accent-primary);
    color: var(--bg-primary);
    box-shadow: 0 0 20px rgba(0, 255, 65, 0.3);
  }
</style>
