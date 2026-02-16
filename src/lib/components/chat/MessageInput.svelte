<script lang="ts">
  let { onSend, disabled = false }: {
    onSend: (content: string) => void;
    disabled?: boolean;
  } = $props();

  let inputValue = $state('');
  let inputEl: HTMLInputElement | undefined = $state();

  function handleSend() {
    const trimmed = inputValue.trim();
    if (trimmed.length === 0 || disabled) return;

    onSend(trimmed);
    inputValue = '';

    // Re-focus input after send
    requestAnimationFrame(() => {
      inputEl?.focus();
    });
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSend();
    }
  }
</script>

<div class="message-input-container">
  <input
    type="text"
    class="message-input"
    placeholder="> Type a message..."
    bind:value={inputValue}
    bind:this={inputEl}
    onkeydown={handleKeyDown}
    {disabled}
  />
  <button
    class="send-button"
    onclick={handleSend}
    disabled={disabled || inputValue.trim().length === 0}
  >
    SEND
  </button>
</div>

<style>
  .message-input-container {
    display: flex;
    gap: 0.5rem;
    padding: 0.75rem;
    border-top: 1px solid var(--border-color);
    background: var(--bg-primary);
  }

  .message-input {
    flex: 1;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 3px;
    padding: 0.6rem 0.75rem;
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--accent-primary);
    outline: none;
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
  }

  .message-input::placeholder {
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .message-input:focus {
    border-color: var(--accent-primary);
    box-shadow: 0 0 6px rgba(0, 255, 65, 0.15);
  }

  .message-input:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .send-button {
    padding: 0.6rem 1.25rem;
    font-family: var(--font-mono);
    font-size: 0.8rem;
    font-weight: bold;
    letter-spacing: 0.1em;
    background: transparent;
    border: 2px solid var(--accent-primary);
    color: var(--accent-primary);
    border-radius: 3px;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .send-button:hover:not(:disabled) {
    background: var(--accent-primary);
    color: var(--bg-primary);
    box-shadow: 0 0 10px var(--accent-primary);
  }

  .send-button:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }
</style>
