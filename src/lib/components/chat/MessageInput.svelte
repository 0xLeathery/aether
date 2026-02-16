<script lang="ts">
  import MentionPicker from './MentionPicker.svelte';

  interface MentionPeer {
    publicKey: string;
    displayName: string;
  }

  let { onSend, disabled = false, peers = [] }: {
    onSend: (content: string, mentions: string[]) => void;
    disabled?: boolean;
    peers?: MentionPeer[];
  } = $props();

  let inputValue = $state('');
  let inputEl: HTMLInputElement | undefined = $state();

  // Mention picker state
  let showMentionPicker = $state(false);
  let mentionFilter = $state('');
  let mentionSelectedIndex = $state(0);
  let mentionMap = $state<Map<string, string>>(new Map());
  let inputRect = $state<{ bottom: number; left: number }>({ bottom: 0, left: 0 });

  // Compute filtered peers for keyboard navigation index bounds
  let filteredPeersForCount = $derived(
    peers.filter(p => p.displayName.toLowerCase().includes(mentionFilter.toLowerCase()))
  );

  function handleInput() {
    const input = inputEl;
    if (!input) return;
    const value = input.value;
    const cursorPos = input.selectionStart ?? value.length;

    // Find the last @ before cursor
    const textBeforeCursor = value.substring(0, cursorPos);
    const atIndex = textBeforeCursor.lastIndexOf('@');

    if (atIndex >= 0 && (atIndex === 0 || /\s/.test(textBeforeCursor[atIndex - 1]))) {
      // Text after @ up to cursor is the filter
      const filterText = textBeforeCursor.substring(atIndex + 1);
      // Only show picker if filter doesn't contain spaces
      if (!filterText.includes(' ') || filterText.length === 0) {
        mentionFilter = filterText;
        showMentionPicker = true;
        mentionSelectedIndex = 0;
        // Position the picker above the input
        const rect = input.getBoundingClientRect();
        inputRect = { bottom: window.innerHeight - rect.top + 4, left: rect.left };
        return;
      }
    }
    showMentionPicker = false;
  }

  function selectMention(publicKey: string, displayName: string) {
    const input = inputEl;
    if (!input) return;

    const value = input.value;
    const cursorPos = input.selectionStart ?? value.length;
    const textBeforeCursor = value.substring(0, cursorPos);
    const atIndex = textBeforeCursor.lastIndexOf('@');

    if (atIndex >= 0) {
      // Replace @filter with @DisplayName + trailing space
      const before = value.substring(0, atIndex);
      const after = value.substring(cursorPos);
      const insertText = `@${displayName} `;
      inputValue = before + insertText + after;

      // Track mention mapping
      const newMap = new Map(mentionMap);
      newMap.set(displayName, publicKey);
      mentionMap = newMap;

      // Hide picker and refocus
      showMentionPicker = false;

      // Set cursor position after inserted mention
      const newCursorPos = atIndex + insertText.length;
      requestAnimationFrame(() => {
        input.focus();
        input.setSelectionRange(newCursorPos, newCursorPos);
      });
    }
  }

  function selectMentionByIndex() {
    if (filteredPeersForCount.length > 0 && mentionSelectedIndex < filteredPeersForCount.length) {
      const peer = filteredPeersForCount[mentionSelectedIndex];
      selectMention(peer.publicKey, peer.displayName);
    }
  }

  function handleSend() {
    const trimmed = inputValue.trim();
    if (trimmed.length === 0 || disabled) return;

    // Build mentions array and transform content
    let finalContent = trimmed;
    const mentions: string[] = [];

    for (const [displayName, publicKey] of mentionMap) {
      // Replace @DisplayName with @[pubkey] in content
      const mentionPattern = `@${displayName}`;
      if (finalContent.includes(mentionPattern)) {
        finalContent = finalContent.replaceAll(mentionPattern, `@[${publicKey}]`);
        if (!mentions.includes(publicKey)) {
          mentions.push(publicKey);
        }
      }
    }

    onSend(finalContent, mentions);
    inputValue = '';
    mentionMap = new Map();
    showMentionPicker = false;

    // Re-focus input after send
    requestAnimationFrame(() => {
      inputEl?.focus();
    });
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (showMentionPicker && filteredPeersForCount.length > 0) {
      if (event.key === 'ArrowDown') {
        event.preventDefault();
        mentionSelectedIndex = (mentionSelectedIndex + 1) % filteredPeersForCount.length;
        return;
      }
      if (event.key === 'ArrowUp') {
        event.preventDefault();
        mentionSelectedIndex = (mentionSelectedIndex - 1 + filteredPeersForCount.length) % filteredPeersForCount.length;
        return;
      }
      if (event.key === 'Enter') {
        event.preventDefault();
        selectMentionByIndex();
        return;
      }
      if (event.key === 'Escape') {
        event.preventDefault();
        showMentionPicker = false;
        return;
      }
    }

    // Normal Enter-to-send logic
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSend();
    }
  }
</script>

<div class="message-input-container">
  {#if showMentionPicker && filteredPeersForCount.length > 0}
    <MentionPicker
      {peers}
      filter={mentionFilter}
      selectedIndex={mentionSelectedIndex}
      position={inputRect}
      onSelect={selectMention}
      onClose={() => showMentionPicker = false}
    />
  {/if}

  <input
    type="text"
    class="message-input"
    placeholder="> Type a message..."
    bind:value={inputValue}
    bind:this={inputEl}
    onkeydown={handleKeyDown}
    oninput={handleInput}
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
    position: relative;
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
