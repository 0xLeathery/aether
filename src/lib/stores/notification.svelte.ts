import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { getMessages, onChatMessagesUpdated, type UnlistenFn } from '../tauri';
import { contactsStore } from './contacts.svelte';
import { moderationStore } from './moderation.svelte';
import { swarmStore } from './swarm.svelte';

let permissionGranted = $state(false);
let initialized = $state(false);
let windowFocused = $state(true); // Safe default — avoids spurious notifications during startup
let currentUserKey: string | null = null;

let lastKnownCounts: Record<string, number> = {};
let lastNotifyTime: Record<string, number> = {};

let unlistenMessages: UnlistenFn | null = null;
let unlistenFocus: UnlistenFn | null = null;

const THROTTLE_MS = 3000;

function getChannelName(swarmId: string, channelId: string): string {
  const swarm = swarmStore.activeSwarm;
  if (swarm && swarm.id === swarmId) {
    const channel = swarm.channels.find((ch) => ch.id === channelId);
    if (channel) return channel.name;
  }
  return channelId;
}

async function handleUpdate(swarmId: string, channelId: string) {
  if (!permissionGranted) return;
  if (windowFocused) return;

  const key = `${swarmId}/${channelId}`;

  try {
    const messages = await getMessages(swarmId, channelId);
    const previousCount = lastKnownCounts[key];

    // First encounter — set baseline, no notification
    if (previousCount === undefined) {
      lastKnownCounts[key] = messages.length;
      return;
    }

    // No new messages
    if (messages.length <= previousCount) {
      lastKnownCounts[key] = messages.length;
      return;
    }

    const newMessages = messages.slice(previousCount);
    lastKnownCounts[key] = messages.length;

    // Filter out own messages
    const relevant = newMessages.filter(
      (msg) =>
        msg.sender_key !== currentUserKey &&
        !moderationStore.isMuted(msg.sender_key, swarmId),
    );

    if (relevant.length === 0) return;

    // Throttle check
    const now = Date.now();
    const lastTime = lastNotifyTime[key];
    if (lastTime !== undefined && now - lastTime < THROTTLE_MS) return;
    lastNotifyTime[key] = now;

    // Check for mentions
    const mentions = relevant.filter((m) =>
      (m.mentions ?? []).includes(currentUserKey!),
    );

    // Pick the most recent relevant message for notification content
    const latest = relevant[relevant.length - 1];
    const displayName = contactsStore.resolveName(
      latest.sender_key,
      latest.sender_name,
    );
    const channelName = getChannelName(swarmId, channelId);

    // Build notification
    let title: string;
    if (mentions.length > 0) {
      title = `${displayName} mentioned you in #${channelName}`;
    } else {
      title = `${displayName} in #${channelName}`;
    }

    let body = latest.content.substring(0, 100);
    if (relevant.length > 1) {
      body += ` (+${relevant.length - 1} more)`;
    }

    sendNotification({ title, body });
  } catch (err) {
    console.error('Notification handleUpdate error:', err);
  }
}

async function initialize(userKey: string) {
  if (initialized) return;
  currentUserKey = userKey;

  try {
    // Permission management
    let granted = await isPermissionGranted();
    if (!granted) {
      const permission = await requestPermission();
      granted = permission === 'granted';
    }
    permissionGranted = granted;

    // Window focus tracking
    const appWindow = getCurrentWindow();
    try {
      windowFocused = await appWindow.isFocused();
    } catch (_) {
      // Linux isFocused() bug mitigation — keep safe default (true)
      windowFocused = true;
    }

    unlistenFocus = await appWindow.onFocusChanged(({ payload }) => {
      windowFocused = payload;
    });

    // Message event handling
    unlistenMessages = await onChatMessagesUpdated((update) => {
      handleUpdate(update.swarm_id, update.channel_id);
    });

    initialized = true;
  } catch (err) {
    console.error('Notification store initialization error:', err);
  }
}

function cleanup() {
  if (unlistenMessages) {
    unlistenMessages();
    unlistenMessages = null;
  }
  if (unlistenFocus) {
    unlistenFocus();
    unlistenFocus = null;
  }
  permissionGranted = false;
  initialized = false;
  windowFocused = true;
  currentUserKey = null;
  lastKnownCounts = {};
  lastNotifyTime = {};
}

export const notificationStore = {
  get permissionGranted() {
    return permissionGranted;
  },
  get initialized() {
    return initialized;
  },
  initialize,
  cleanup,
};
