# Phase 10: Desktop Notifications - Research

**Researched:** 2026-02-24
**Domain:** Tauri v2 desktop notifications, window focus detection, notification deduplication
**Confidence:** HIGH

## Summary

Desktop notifications for Aether require the official `tauri-plugin-notification` (v2.x) for native OS toast notifications and Tauri's built-in window focus API (`getCurrentWindow().isFocused()`) for suppression when the app is focused. The notification trigger point already exists: the `chat-messages-updated` Tauri event fires on both incoming sync (remote messages) and outgoing send, and the unread store already listens to it. A new `notificationStore` should intercept the same event, check window focus, resolve sender names via `contactsStore.resolveName()`, and dispatch notifications via the plugin's `sendNotification()` API.

The architecture is straightforward because all three requirements (NOTF-01, NOTF-02, NOTF-03) can be satisfied on the frontend alone -- the backend already emits the necessary events, and the frontend already has message data, unread tracking, mention detection, and name resolution. No Rust-side changes are needed beyond registering the plugin in `lib.rs`.

**Primary recommendation:** Use `tauri-plugin-notification` (Rust crate v2 + `@tauri-apps/plugin-notification` npm) with frontend-only notification logic in a new `notificationStore.svelte.ts`. Intercept `chat-messages-updated` events, diff messages against last-known count, check focus state, check moderation, resolve names, and fire native notifications.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| NOTF-01 | User receives desktop notifications for new messages when app is not focused | `tauri-plugin-notification` sendNotification + `getCurrentWindow().isFocused()` for focus gating; trigger from `chat-messages-updated` event |
| NOTF-02 | User receives notification when mentioned by name | Check `mentions` array on new messages against current user's public key; use distinct title/body format to distinguish from general notifications |
| NOTF-03 | Notification displays sender name and message preview | Use `contactsStore.resolveName(sender_key, sender_name)` for petname > self-asserted > key resolution; truncate content for body preview |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `tauri-plugin-notification` | 2 (Rust crate) | Backend notification capability | Official Tauri plugin; cross-platform (macOS, Windows, Linux) |
| `@tauri-apps/plugin-notification` | ^2.0.0 (npm) | Frontend notification API | Official JS bindings; `sendNotification()`, `isPermissionGranted()`, `requestPermission()` |
| `@tauri-apps/api/window` | (already installed) | Window focus detection | `getCurrentWindow().isFocused()` and `onFocusChanged()` for focus tracking |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `@tauri-apps/api/event` | (already installed) | Listen to `chat-messages-updated` | Already used by chatStore and unreadStore; notificationStore uses same pattern |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `tauri-plugin-notification` | Web Notification API (`new Notification()`) | Web API works in Tauri webview but lacks OS integration, sound customization, and permission is auto-granted in Tauri -- plugin is more robust and future-proof |
| Frontend notification logic | Rust-side notification emission | Would require passing window focus state to Rust, duplicating name resolution logic; frontend already has all context needed |

**Installation:**
```bash
# Rust (in src-tauri/)
cargo add tauri-plugin-notification

# JavaScript
npm install @tauri-apps/plugin-notification
```

## Architecture Patterns

### Recommended Project Structure
```
src/
├── lib/stores/notification.svelte.ts    # New notification store
└── lib/tauri.ts                         # Add notification imports

src-tauri/
├── src/lib.rs                           # Register plugin
└── capabilities/default.json            # Add notification:default permission
```

### Pattern 1: Frontend Notification Store (Reactive)
**What:** A Svelte 5 runes-based store that listens to `chat-messages-updated` events, checks window focus, resolves names, and dispatches OS notifications.
**When to use:** Always -- this is the primary pattern for Phase 10.
**Example:**
```typescript
// Source: Tauri notification plugin docs + existing project patterns
import { sendNotification, isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import type { ChatMessagesUpdated } from '../tauri';

let windowFocused = $state(true);
let permissionGranted = $state(false);
let lastKnownCounts: Record<string, number> = {};

async function initialize(currentUserKey: string) {
  // Check/request notification permission
  permissionGranted = await isPermissionGranted();
  if (!permissionGranted) {
    const result = await requestPermission();
    permissionGranted = result === 'granted';
  }

  // Track window focus state
  const appWindow = getCurrentWindow();
  windowFocused = await appWindow.isFocused();
  await appWindow.onFocusChanged(({ payload: focused }) => {
    windowFocused = focused;
  });

  // Listen for chat updates
  await listen<ChatMessagesUpdated>('chat-messages-updated', async (event) => {
    if (windowFocused || !permissionGranted) return;
    // ... fetch new messages, diff, check moderation, send notification
  });
}
```

### Pattern 2: Message Diffing for New-Message Detection
**What:** Track the last-known message count per channel key (`swarmId/channelId`). When `chat-messages-updated` fires, fetch messages and compare count to detect genuinely new messages.
**When to use:** Always -- prevents duplicate notifications on document sync convergence.
**Example:**
```typescript
const key = `${swarmId}/${channelId}`;
const messages = await getMessages(swarmId, channelId);
const previousCount = lastKnownCounts[key] ?? 0;

if (messages.length > previousCount) {
  const newMessages = messages.slice(previousCount);
  lastKnownCounts[key] = messages.length;
  // Process newMessages for notifications...
}
```

### Pattern 3: Name Resolution Chain (Petname > Self-Asserted > Key)
**What:** Use `contactsStore.resolveName(sender_key, sender_name)` to resolve display names. This already implements the priority chain: petname > self-asserted name > truncated public key.
**When to use:** Always -- satisfies NOTF-03 requirement for resolved sender names.
**Example:**
```typescript
import { contactsStore } from './contacts.svelte';

// sender_name is the self-asserted name at time of sending (stored in ChatMessage)
const displayName = contactsStore.resolveName(msg.sender_key, msg.sender_name);
```

### Pattern 4: Mention-Distinct Notifications
**What:** Check if new messages contain the current user's public key in `mentions` array. Use a different notification title to distinguish mentions from general messages.
**When to use:** Always -- satisfies NOTF-02 requirement.
**Example:**
```typescript
const isMention = msg.mentions.includes(currentUserKey);
const title = isMention
  ? `@${displayName} mentioned you in #${channelName}`
  : `${displayName} in #${channelName}`;
const body = msg.content.substring(0, 100);

sendNotification({ title, body });
```

### Pattern 5: Moderation Suppression
**What:** Notifications must respect moderation state -- muted/hidden/blocked peers should not trigger notifications.
**When to use:** Always -- consistent with how unread store already filters moderated peers.
**Example:**
```typescript
import { moderationStore } from './moderation.svelte';

// Skip notification for moderated senders (all tiers suppress)
if (moderationStore.isMuted(msg.sender_key, swarmId)) return;
```

### Anti-Patterns to Avoid
- **Emitting notifications from Rust backend:** Would duplicate name resolution logic and require passing window focus state across IPC boundary. The frontend already has all needed context.
- **Using Web Notification API directly:** Works but bypasses Tauri's permission system and lacks OS-level integration for sound/icon customization.
- **Notifying on every sync event:** The same document may sync multiple times during convergence. Always diff message counts, not sync events.
- **Notifying for own messages:** Must filter out messages where `sender_key === currentUserKey`.
- **No throttle on rapid messages:** In active channels, many messages can arrive in quick succession. Batch or throttle to avoid notification spam.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| OS notification dispatch | Custom notification shell commands or platform-specific code | `tauri-plugin-notification` | Cross-platform (macOS, Windows, Linux), handles permissions, sounds, icons |
| Window focus detection | Custom visibility API or document.hasFocus() polling | `getCurrentWindow().isFocused()` + `onFocusChanged()` | Tauri provides native window focus state; `document.hasFocus()` can be unreliable in webview |
| Permission management | Manual OS permission checks | `isPermissionGranted()` + `requestPermission()` from plugin | Plugin abstracts platform differences |
| Name resolution | Custom name lookup | `contactsStore.resolveName()` | Already implements petname > self-asserted > key chain |

**Key insight:** The entire notification pipeline can be built in the frontend. The backend already emits `chat-messages-updated` events with swarm/channel IDs, and the frontend already has message fetching, name resolution, mention detection, and moderation filtering.

## Common Pitfalls

### Pitfall 1: Duplicate Notifications on Sync Convergence
**What goes wrong:** Automerge sync may trigger multiple `chat-messages-updated` events for the same set of messages as documents converge with multiple peers.
**Why it happens:** Each peer sync that receives changes emits an event. The same messages may arrive from different peers.
**How to avoid:** Track last-known message count per channel. Only notify on genuinely new messages (count increased). The message count is monotonically increasing because Automerge CRDT lists only grow.
**Warning signs:** User receives 2-3 identical notifications for the same message.

### Pitfall 2: Notification Spam in Active Channels
**What goes wrong:** In a busy channel with rapid messages, each message triggers a separate notification, overwhelming the user's notification center.
**Why it happens:** No rate limiting on notification dispatch.
**How to avoid:** Implement a simple throttle: batch notifications within a 2-3 second window. If multiple messages arrive in the window, show a summary notification ("3 new messages in #general") instead of individual ones.
**Warning signs:** Notification center fills up with dozens of individual messages.

### Pitfall 3: Notifying for Own Messages
**What goes wrong:** User sends a message and immediately receives a notification for it.
**Why it happens:** `chat-messages-updated` fires for both local sends and remote syncs. If the notification store listens to the same event, it sees its own messages.
**How to avoid:** Filter out messages where `msg.sender_key === currentUserKey`.
**Warning signs:** Every sent message produces a notification.

### Pitfall 4: Notifications When Viewing the Active Channel
**What goes wrong:** User is looking at #general and receives a notification for a new message in #general.
**Why it happens:** Window is focused but the notification logic only checks window focus, not which channel is active.
**How to avoid:** Also check if the message is for the currently-viewed channel (`swarmStore.activeSwarm?.id === swarmId && swarmStore.activeChannelId === channelId`). If the user is viewing that channel and the window is focused, skip the notification.
**Warning signs:** User gets notified about messages they are actively reading.

### Pitfall 5: Linux isFocused() Bug
**What goes wrong:** `appWindow.isFocused()` always returns `false` on some Linux window managers.
**Why it happens:** Known Tauri issue (#11323) where certain Linux compositors don't report focus state correctly.
**How to avoid:** Use `onFocusChanged()` listener as primary source (more reliable than polling `isFocused()`). Initialize `windowFocused` to `true` (assume focused until proven otherwise). This is a known Tauri bug -- not something we can fix, but we can make behavior acceptable.
**Warning signs:** Linux users receive notifications even when the app is focused.

### Pitfall 6: Permission Request Timing
**What goes wrong:** App requests notification permission immediately on startup, before user understands why.
**Why it happens:** Permission requested in store initialization.
**How to avoid:** Request permission during notification store init (happens after identity setup and app shell render). On macOS, the OS dialog explains the app wants to send notifications. This is acceptable UX since the app is a communication tool where notifications are expected.
**Warning signs:** Users decline permission because they don't understand the context.

## Code Examples

Verified patterns from official sources:

### Plugin Registration (Rust - lib.rs)
```rust
// Source: https://v2.tauri.app/plugin/notification/
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())  // Add this line
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        // ... existing handlers
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Capability Permission (default.json)
```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "default capability",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:default",
    "notification:default"
  ]
}
```

### Permission Check + Send (JavaScript)
```typescript
// Source: https://v2.tauri.app/plugin/notification/
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';

let permissionGranted = await isPermissionGranted();
if (!permissionGranted) {
  const permission = await requestPermission();
  permissionGranted = permission === 'granted';
}

if (permissionGranted) {
  sendNotification({ title: 'Aether', body: 'New message from Alice' });
}
```

### Window Focus Tracking (JavaScript)
```typescript
// Source: https://v2.tauri.app/reference/javascript/api/namespacewindow/
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();
let focused = await appWindow.isFocused();

const unlisten = await appWindow.onFocusChanged(({ payload }) => {
  focused = payload;
});
```

### Full Notification Store Skeleton
```typescript
// notification.svelte.ts
import { sendNotification, isPermissionGranted, requestPermission } from '@tauri-apps/plugin-notification';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { getMessages, onChatMessagesUpdated, type ChatMessage, type UnlistenFn } from '../tauri';
import { contactsStore } from './contacts.svelte';
import { moderationStore } from './moderation.svelte';
import { swarmStore } from './swarm.svelte';

let windowFocused = $state(true);
let permissionGranted = $state(false);
let initialized = $state(false);
let currentUserKey = '';
let lastKnownCounts: Record<string, number> = {};
let unlistenMessages: UnlistenFn | null = null;

// Throttle: max one notification per channel per N ms
const THROTTLE_MS = 3000;
let lastNotifyTime: Record<string, number> = {};

async function initialize(userKey: string) {
  if (initialized) return;
  currentUserKey = userKey;

  // Request permission
  permissionGranted = await isPermissionGranted();
  if (!permissionGranted) {
    const result = await requestPermission();
    permissionGranted = result === 'granted';
  }

  // Track focus
  const appWindow = getCurrentWindow();
  windowFocused = await appWindow.isFocused();
  await appWindow.onFocusChanged(({ payload: focused }) => {
    windowFocused = focused;
  });

  // Listen for messages
  unlistenMessages = await onChatMessagesUpdated(async (update) => {
    await handleUpdate(update.swarm_id, update.channel_id);
  });

  initialized = true;
}

async function handleUpdate(swarmId: string, channelId: string) {
  if (!permissionGranted) return;

  // Skip if window focused AND viewing this channel
  if (windowFocused &&
      swarmStore.activeSwarm?.id === swarmId &&
      swarmStore.activeChannelId === channelId) {
    return;
  }

  // Skip if window is focused (general suppression for NOTF-01)
  if (windowFocused) return;

  const key = `${swarmId}/${channelId}`;
  const messages = await getMessages(swarmId, channelId);
  const prevCount = lastKnownCounts[key] ?? messages.length; // First load: no notification
  lastKnownCounts[key] = messages.length;

  if (messages.length <= prevCount) return;

  const newMessages = messages.slice(prevCount);

  // Filter: own messages and moderated senders
  const relevant = newMessages.filter(m =>
    m.sender_key !== currentUserKey &&
    !moderationStore.isMuted(m.sender_key, swarmId)
  );

  if (relevant.length === 0) return;

  // Throttle per channel
  const now = Date.now();
  if (lastNotifyTime[key] && now - lastNotifyTime[key] < THROTTLE_MS) return;
  lastNotifyTime[key] = now;

  // Check for mentions
  const mentions = relevant.filter(m => m.mentions.includes(currentUserKey));
  // ... dispatch notification (mention or general)
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Tauri v1 `notification` JS API | `tauri-plugin-notification` v2 plugin | Tauri v2 (2024) | Plugin-based architecture; requires Cargo + npm install + capability config |
| `appWindow` global | `getCurrentWindow()` import | Tauri v2 (2024) | No more global `appWindow`; must import from `@tauri-apps/api/window` |

**Deprecated/outdated:**
- `@tauri-apps/api/notification`: Tauri v1 API -- replaced by plugin system in v2
- `window.__TAURI__`: Global window object pattern from Tauri v1 -- use module imports in v2

## Open Questions

1. **Notification click-to-navigate**
   - What we know: `onAction()` API exists but is documented primarily for mobile. Desktop click handling for notifications is limited in Tauri v2.
   - What's unclear: Whether clicking a notification on macOS/Windows/Linux can bring the Tauri window to focus and navigate to the relevant channel.
   - Recommendation: Defer click-to-navigate as a nice-to-have. The core requirements (NOTF-01/02/03) don't require it. If pursued, `appWindow.setFocus()` + channel selection on notification click would be the approach, but may require Rust-side event handling.

2. **macOS notification badge count**
   - What we know: STATE.md notes "Tauri notification badge count bugs on macOS (test in Phase 10)"
   - What's unclear: Whether `tauri-plugin-notification` supports dock badge counts, and if the reported bugs still exist.
   - Recommendation: Skip badge count for this phase. Toast notifications satisfy all three requirements. Badge count is a polish item for a future phase.

3. **Notification sound**
   - What we know: `tauri-plugin-notification` v2.3.3 added desktop sound support.
   - What's unclear: Whether custom sounds are needed or the default OS notification sound suffices.
   - Recommendation: Use default OS notification sound (no custom sound asset needed). This is the simplest approach and consistent with other chat apps.

## Sources

### Primary (HIGH confidence)
- [Tauri Notification Plugin Docs](https://v2.tauri.app/plugin/notification/) - Setup, permissions, JS API, Rust API, code examples
- [Tauri Notification JS API Reference](https://v2.tauri.app/reference/javascript/notification/) - `sendNotification()`, `isPermissionGranted()`, `requestPermission()`, all types
- [Tauri Window API Reference](https://v2.tauri.app/reference/javascript/api/namespacewindow/) - `getCurrentWindow()`, `isFocused()`, `onFocusChanged()`, `setFocus()`
- [Tauri Plugin Permissions Docs](https://v2.tauri.app/learn/security/using-plugin-permissions/) - Capability configuration format
- Existing codebase analysis: `unread.svelte.ts`, `chat.svelte.ts`, `contacts.svelte.ts`, `moderation.svelte.ts`, `tauri.ts`, `lib.rs`, `chat/mod.rs`

### Secondary (MEDIUM confidence)
- [crates.io: tauri-plugin-notification](https://crates.io/crates/tauri-plugin-notification) - Latest version 2.3.3
- [npmjs: @tauri-apps/plugin-notification](https://www.npmjs.com/package/@tauri-apps/plugin-notification) - npm package
- [Tauri GitHub Issue #11323](https://github.com/tauri-apps/tauri/issues/11323) - Linux `isFocused()` bug

### Tertiary (LOW confidence)
- [Tauri GitHub Issue #4770](https://github.com/tauri-apps/tauri/issues/4770) - Desktop notification click event status (closed, unclear resolution)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Official Tauri plugin, well-documented, codebase already uses Tauri plugin pattern
- Architecture: HIGH - All integration points verified by reading existing codebase; event flow, name resolution, moderation filtering, and unread tracking are all already implemented
- Pitfalls: HIGH - Deduplication, throttling, and focus gating are well-understood patterns; Linux bug documented in official tracker

**Research date:** 2026-02-24
**Valid until:** 2026-03-24 (stable -- Tauri v2 plugin API is released and unlikely to change)