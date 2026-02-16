<script lang="ts">
  import { onDestroy } from 'svelte';
  import MessageList from './MessageList.svelte';
  import MessageInput from './MessageInput.svelte';
  import { chatStore } from '../../stores/chat.svelte';
  import { unreadStore } from '../../stores/unread.svelte';

  let { swarmId, channelId, currentUserKey }: {
    swarmId: string;
    channelId: string;
    currentUserKey: string;
  } = $props();

  let sendError = $state<string | null>(null);

  // Initialize chat store and load messages when swarm/channel changes
  $effect(() => {
    const _swarm = swarmId;
    const _channel = channelId;

    (async () => {
      await chatStore.initialize();
      await unreadStore.initialize(currentUserKey);
      await chatStore.loadMessages(_swarm, _channel);
      // Clear unread when viewing channel
      if (chatStore.messages.length > 0) {
        unreadStore.markRead(_swarm, _channel, chatStore.messages.length);
      }
    })();
  });

  async function handleSend(content: string) {
    sendError = null;
    try {
      await chatStore.send(content);
    } catch (err) {
      sendError = err instanceof Error ? err.message : 'Failed to send message';
    }
  }

  onDestroy(() => {
    chatStore.cleanup();
  });
</script>

<div class="chat-panel">
  {#if chatStore.loading}
    <div class="loading-state">
      <span class="loading-text">Loading messages...</span>
    </div>
  {:else}
    <MessageList messages={chatStore.messages} {currentUserKey} />
  {/if}

  <MessageInput onSend={handleSend} disabled={chatStore.sending} />

  {#if sendError}
    <div class="send-error">{sendError}</div>
  {/if}

  {#if chatStore.error}
    <div class="store-error">{chatStore.error}</div>
  {/if}
</div>

<style>
  .chat-panel {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    background: var(--bg-primary);
    width: 100%;
  }

  .loading-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .loading-text {
    font-family: var(--font-mono);
    font-size: 0.85rem;
    color: var(--text-muted);
    letter-spacing: 0.02em;
  }

  .send-error {
    padding: 0.35rem 0.75rem;
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: #ff4444;
    background: rgba(255, 68, 68, 0.1);
    border-top: 1px solid #ff4444;
  }

  .store-error {
    padding: 0.35rem 0.75rem;
    font-family: var(--font-mono);
    font-size: 0.75rem;
    color: #ff4444;
    background: rgba(255, 68, 68, 0.1);
    border-top: 1px solid #ff4444;
  }
</style>
