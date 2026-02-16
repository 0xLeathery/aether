<script lang="ts">
  import type { ChatMessage } from '../../tauri';

  let { messages, currentUserKey }: {
    messages: ChatMessage[];
    currentUserKey: string;
  } = $props();

  let scrollContainer: HTMLDivElement | undefined = $state();

  // Auto-scroll to bottom when messages change
  $effect(() => {
    // Access messages.length to track changes
    const _len = messages.length;
    if (scrollContainer) {
      // Use tick-like delay to let DOM update
      requestAnimationFrame(() => {
        if (scrollContainer) {
          scrollContainer.scrollTop = scrollContainer.scrollHeight;
        }
      });
    }
  });

  function formatTime(timestamp: number): string {
    const date = new Date(timestamp);
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false });
  }

  function formatDate(timestamp: number): string {
    const date = new Date(timestamp);
    return date.toLocaleDateString([], { year: 'numeric', month: 'short', day: 'numeric' });
  }

  function getDateKey(timestamp: number): string {
    const date = new Date(timestamp);
    return `${date.getFullYear()}-${date.getMonth()}-${date.getDate()}`;
  }

  function getSenderDisplay(msg: ChatMessage): string {
    if (msg.sender_name && msg.sender_name.length > 0) {
      return msg.sender_name;
    }
    return msg.sender_key.substring(0, 8) + '...';
  }

  function isOwnMessage(msg: ChatMessage): boolean {
    return msg.sender_key === currentUserKey;
  }

  function shouldShowDateSeparator(index: number): boolean {
    if (index === 0) return true;
    return getDateKey(messages[index].timestamp) !== getDateKey(messages[index - 1].timestamp);
  }

  function isToday(timestamp: number): boolean {
    const today = new Date();
    const date = new Date(timestamp);
    return date.getFullYear() === today.getFullYear()
      && date.getMonth() === today.getMonth()
      && date.getDate() === today.getDate();
  }
</script>

<div class="message-list" bind:this={scrollContainer}>
  {#if messages.length === 0}
    <div class="empty-state">
      <span class="empty-text">No messages yet. Start the conversation.</span>
    </div>
  {:else}
    {#each messages as msg, i}
      {#if shouldShowDateSeparator(i)}
        <div class="date-separator">
          <span class="date-label">
            {isToday(msg.timestamp) ? 'Today' : formatDate(msg.timestamp)}
          </span>
        </div>
      {/if}
      <div class="message-row">
        <span class="msg-time">{formatTime(msg.timestamp)}</span>
        <span class="msg-sender">
          {getSenderDisplay(msg)}
          {#if isOwnMessage(msg)}
            <span class="you-badge">YOU</span>
          {/if}
        </span>
        <span class="msg-content">{msg.content}</span>
      </div>
    {/each}
  {/if}
</div>

<style>
  .message-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.75rem;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    min-height: 0;
  }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .empty-text {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-muted);
    letter-spacing: 0.02em;
  }

  .date-separator {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem 0;
    margin: 0.25rem 0;
  }

  .date-label {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: var(--text-muted);
    letter-spacing: 0.05em;
    text-transform: uppercase;
    padding: 0.125rem 0.75rem;
    border: 1px solid var(--border-color);
    border-radius: 2px;
  }

  .message-row {
    display: flex;
    gap: 0.5rem;
    padding: 0.25rem 0;
    font-family: var(--font-mono);
    font-size: 0.85rem;
    line-height: 1.4;
    align-items: baseline;
  }

  .msg-time {
    color: var(--text-muted);
    font-size: 0.8rem;
    flex-shrink: 0;
    white-space: nowrap;
  }

  .msg-sender {
    color: var(--accent-primary);
    font-weight: 600;
    flex-shrink: 0;
    white-space: nowrap;
  }

  .msg-sender::after {
    content: ':';
  }

  .you-badge {
    font-size: 0.65rem;
    color: var(--accent-amber);
    padding: 0.05rem 0.25rem;
    border: 1px solid var(--accent-amber);
    border-radius: 2px;
    letter-spacing: 0.05em;
    margin-left: 0.25rem;
    font-weight: bold;
    vertical-align: middle;
  }

  .msg-content {
    color: var(--text-primary);
    word-break: break-word;
    min-width: 0;
  }
</style>
