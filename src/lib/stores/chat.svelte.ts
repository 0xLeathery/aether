import { sendMessage, getMessages, onChatMessagesUpdated, type ChatMessage, type UnlistenFn } from '../tauri';

let messages = $state<ChatMessage[]>([]);
let sending = $state(false);
let loading = $state(false);
let error = $state<string | null>(null);
let initialized = $state(false);
let currentSwarmId = $state<string | null>(null);
let currentChannelId = $state<string | null>(null);

let unlistenMessages: UnlistenFn | null = null;

async function initialize() {
  if (initialized) return;

  try {
    // Set up event listener for sync updates
    unlistenMessages = await onChatMessagesUpdated((update) => {
      // Re-fetch messages if the update matches the current channel
      if (update.swarm_id === currentSwarmId && update.channel_id === currentChannelId) {
        refreshMessages();
      }
    });

    initialized = true;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to initialize chat';
    console.error('Chat initialization error:', err);
  }
}

async function refreshMessages() {
  if (!currentSwarmId || !currentChannelId) return;

  try {
    messages = await getMessages(currentSwarmId, currentChannelId);
  } catch (err) {
    console.error('Chat refresh error:', err);
  }
}

async function loadMessages(swarmId: string, channelId: string) {
  currentSwarmId = swarmId;
  currentChannelId = channelId;
  loading = true;
  error = null;

  try {
    messages = await getMessages(swarmId, channelId);
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to load messages';
    console.error('Chat load error:', err);
  } finally {
    loading = false;
  }
}

async function send(content: string, mentions: string[] = []) {
  if (!currentSwarmId || !currentChannelId) {
    error = 'No channel selected';
    return;
  }

  sending = true;
  error = null;

  try {
    const msg = await sendMessage(currentSwarmId, currentChannelId, content, mentions);
    // Optimistic update: append the returned message (already persisted on backend)
    messages = [...messages, msg];
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to send message';
    console.error('Chat send error:', err);
    throw err;
  } finally {
    sending = false;
  }
}

async function switchChannel(swarmId: string, channelId: string) {
  // Clear existing messages for clean transition
  messages = [];
  await loadMessages(swarmId, channelId);
}

function cleanup() {
  if (unlistenMessages) {
    unlistenMessages();
    unlistenMessages = null;
  }
  messages = [];
  sending = false;
  loading = false;
  error = null;
  initialized = false;
  currentSwarmId = null;
  currentChannelId = null;
}

export const chatStore = {
  get messages() { return messages; },
  get sending() { return sending; },
  get loading() { return loading; },
  get error() { return error; },
  get initialized() { return initialized; },
  get currentChannelId() { return currentChannelId; },
  initialize,
  loadMessages,
  send,
  switchChannel,
  cleanup,
};
