import { getMessages, getUnreadState, markChannelRead, onChatMessagesUpdated, type UnlistenFn } from '../tauri';
import { moderationStore } from './moderation.svelte';

interface ChannelUnreadState {
  totalSeen: number;
  hasUnread: boolean;
  hasMention: boolean;
}

let states = $state<Record<string, ChannelUnreadState>>({});
let initialized = $state(false);
let unlistenMessages: UnlistenFn | null = null;
let _currentUserKey: string | null = null;

async function initialize(currentUserKey: string) {
  if (initialized) return;
  _currentUserKey = currentUserKey;

  try {
    // Load persisted state
    const persisted = await getUnreadState();
    const loaded: Record<string, ChannelUnreadState> = {};
    for (const [key, value] of Object.entries(persisted)) {
      loaded[key] = {
        totalSeen: value.total_seen,
        hasUnread: false, // Will be recalculated when messages are checked
        hasMention: false,
      };
    }
    states = loaded;

    // Set up event listener for incoming messages
    unlistenMessages = await onChatMessagesUpdated((update) => {
      recalculate(update.swarm_id, update.channel_id);
    });

    initialized = true;
  } catch (err) {
    console.error('Unread store initialization error:', err);
  }
}

async function recalculate(swarmId: string, channelId: string) {
  const key = `${swarmId}/${channelId}`;

  try {
    const messages = await getMessages(swarmId, channelId);
    const state = states[key];
    const totalSeen = state?.totalSeen ?? 0;
    const unseenCount = messages.length - totalSeen;

    if (unseenCount > 0) {
      // Check unseen messages, filtering out all moderated peers
      const unseenMessages = messages.slice(totalSeen);

      // All moderation tiers suppress unreads (per user decision)
      const visibleUnseen = unseenMessages.filter(m =>
        !moderationStore.isMuted(m.sender_key, swarmId)
      );
      const hasUnread = visibleUnseen.length > 0;

      // All moderation tiers suppress mentions (visibleUnseen already filtered)
      const hasMention = _currentUserKey
        ? visibleUnseen.some(m => (m.mentions ?? []).includes(_currentUserKey!))
        : false;

      states = {
        ...states,
        [key]: {
          totalSeen,
          hasUnread,
          hasMention,
        },
      };
    } else if (state?.hasUnread) {
      // No unreads -- ensure clean state
      states = { ...states, [key]: { ...state, hasUnread: false, hasMention: false } };
    }
  } catch (err) {
    console.error('Unread recalculate error:', err);
  }
}

async function markRead(swarmId: string, channelId: string, messageCount: number) {
  const key = `${swarmId}/${channelId}`;

  states = {
    ...states,
    [key]: {
      totalSeen: messageCount,
      hasUnread: false,
      hasMention: false,
    },
  };

  try {
    await markChannelRead(swarmId, channelId, messageCount);
  } catch (err) {
    console.error('Failed to persist unread state:', err);
  }
}

function recalculateAll() {
  for (const key of Object.keys(states)) {
    const [swarmId, channelId] = key.split('/');
    if (swarmId && channelId) {
      recalculate(swarmId, channelId);
    }
  }
}

function hasUnread(swarmId: string, channelId: string): boolean {
  const key = `${swarmId}/${channelId}`;
  return states[key]?.hasUnread ?? false;
}

function hasMention(swarmId: string, channelId: string): boolean {
  const key = `${swarmId}/${channelId}`;
  return states[key]?.hasMention ?? false;
}

function hasSwarmUnread(swarmId: string): boolean {
  const prefix = `${swarmId}/`;
  return Object.entries(states).some(([key, state]) => key.startsWith(prefix) && state.hasUnread);
}

function hasSwarmMention(swarmId: string): boolean {
  const prefix = `${swarmId}/`;
  return Object.entries(states).some(([key, state]) => key.startsWith(prefix) && state.hasMention);
}

function cleanup() {
  if (unlistenMessages) {
    unlistenMessages();
    unlistenMessages = null;
  }
  states = {};
  initialized = false;
  _currentUserKey = null;
}

export const unreadStore = {
  get states() { return states; },
  get initialized() { return initialized; },
  initialize,
  recalculate,
  recalculateAll,
  markRead,
  hasUnread,
  hasMention,
  hasSwarmUnread,
  hasSwarmMention,
  cleanup,
};
