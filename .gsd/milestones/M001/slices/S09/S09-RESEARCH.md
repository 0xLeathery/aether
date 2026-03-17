# Phase 8: Unread & Mentions - Research

**Researched:** 2026-02-16
**Domain:** Unread message tracking, @mention autocomplete and rendering, per-channel read state persistence
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Dot indicator style (not count badge, not bold name) for channel unread
- Channels with @mentions of the current user get a distinct color dot (vs regular unread dot)
- @mention autocomplete popup appears above the message input (like Discord/Slack)
- Each autocomplete entry shows resolved name + truncated public key for disambiguation
- Live filtering as user types characters after @ (e.g., @jo narrows list)
- Mentions display the current resolved name (petname > self-asserted), not frozen at send time
- Mentions are clickable -- clicking @name opens the peer's contact info
- Mentions stored as public key references but resolved to current names at render time
- Distinct dot color for mention-containing unreads vs regular unreads (two tiers of importance)
- Context menu "Mark as read" extends the existing channel context menu from Phase 7
- Unread state persists across app restarts (saved to disk)
- Discord-style popup above input for @mention autocomplete

### Claude's Discretion
- Unread dot positioning on channel entries
- Swarm-level unread indicator style
- Mention token rendering in the message input
- Mention highlight visual treatment in chat view
- @name text rendering in sent messages
- Unread clearing trigger (channel open vs scroll-to-bottom)
- Live message read behavior

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope
</user_constraints>

## Summary

Phase 8 adds two interconnected features to Aether: unread indicators on channels/swarms and @mention support in chat messages. Both features are primarily frontend-driven with thin backend persistence layers. No new crate dependencies are required -- everything uses the existing `tauri-plugin-store` for persistence, `automerge`/`autosurgeon` for the ChatMessage schema change (adding a `mentions` field), and Svelte 5 reactive stores for frontend state.

The unread system tracks per-channel read state using a "last seen message IDs" set approach (rather than timestamps, which break under CRDT out-of-order sync). The mentions system stores public key references in message content, resolved to current display names at render time. The message input needs upgrading from a plain `<input>` to a `contenteditable` div or `<textarea>` to support inline mention token rendering and the autocomplete popup.

**Primary recommendation:** Build unread tracking first (pure frontend + storage concern), then mentions (requires ChatMessage schema change in both Rust and TypeScript, plus significant UI work for autocomplete and rendering).

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| tauri-plugin-store | 2 | Persist unread state to `unread.json` | Already in use for swarms.json, contacts.json. Same pattern. |
| automerge | 0.7 | ChatMessage CRDT with new `mentions` field | Already in use. Schema change is additive (new Vec field). |
| autosurgeon | 0.10 | Derive Hydrate/Reconcile for updated ChatMessage | Already in use. |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| @tauri-apps/plugin-store | ^2.4.2 | Frontend-side store access for unread state | Already in package.json but unused from JS. May use for fast read-state persistence without IPC round-trip. |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Message ID set for unread | Timestamp-based "last read" | Timestamps break with CRDT out-of-order sync (Pitfall 6 from research). Message ID set is immune to reordering but uses more storage. |
| `contenteditable` div for input | Keep `<input>` + plain text mentions | `<input>` cannot render styled inline tokens (colored @mention pills). `contenteditable` enables rich rendering but adds complexity. Recommendation: use `<textarea>` with plain text @mentions and render the styled output only in sent messages. |
| Frontend-only unread state | Rust-backed UnreadTracker | Frontend-only is simpler and the chat store already receives `chat-messages-updated` events. Rust-backed adds IPC overhead for a purely local concern. |

**Installation:**
```bash
# No new packages needed. All dependencies already present.
```

## Architecture Patterns

### Recommended Project Structure
```
src/lib/
├── stores/
│   └── unread.svelte.ts         # NEW: Per-channel unread tracking
├── components/
│   ├── layout/
│   │   ├── ChannelList.svelte   # MODIFIED: Add unread dots
│   │   └── Sidebar.svelte       # MODIFIED: Add swarm-level unread indicator
│   ├── chat/
│   │   ├── MessageInput.svelte  # MODIFIED: Add @mention autocomplete
│   │   ├── MessageList.svelte   # MODIFIED: Mention highlighting, mention-message highlighting
│   │   ├── MentionPicker.svelte # NEW: Autocomplete popup component
│   │   └── ChatPanel.svelte     # MODIFIED: Wire unread clearing on channel view
│   └── channel/
│       └── ChannelContextMenu.svelte  # MODIFIED: Add "Mark as read" item

src-tauri/src/
├── chat/
│   ├── message.rs               # MODIFIED: Add mentions field to ChatMessage
│   └── document.rs              # No change (autosurgeon handles new field)
├── commands/
│   ├── chat.rs                  # MODIFIED: Update ChatMessageResponse with mentions
│   └── unread.rs                # NEW: mark_channel_read, get_unread_state commands
└── error.rs                     # MODIFIED: Add UnreadError if needed (may not be needed)
```

### Pattern 1: Unread State Store (Frontend-Driven)
**What:** A Svelte 5 reactive store that tracks which channels have unread messages and whether those unreads contain mentions of the current user. State persists to disk via tauri-plugin-store.
**When to use:** For all unread indicator logic.

```typescript
// unread.svelte.ts
import { LazyStore } from '@tauri-apps/plugin-store';

interface ChannelReadState {
  lastReadMessageIds: Set<string>;  // Messages the user has seen
  hasUnread: boolean;               // Derived: are there unseen messages?
  hasMention: boolean;              // Derived: do unseen messages mention current user?
}

let channelStates = $state<Map<string, ChannelReadState>>(new Map());
let store: LazyStore | null = null;

// On channel view: mark all current messages as read
function markRead(swarmId: string, channelId: string, messageIds: string[]) {
  const key = `${swarmId}/${channelId}`;
  // Add all messageIds to the seen set
  // Persist to store
}

// On chat-messages-updated event: check for new unseen messages
function onMessagesUpdated(swarmId: string, channelId: string, messages: ChatMessage[], currentUserKey: string) {
  const key = `${swarmId}/${channelId}`;
  const state = channelStates.get(key);
  const seenIds = state?.lastReadMessageIds ?? new Set();
  const unseenMessages = messages.filter(m => !seenIds.has(m.id));

  if (unseenMessages.length > 0) {
    const hasMention = unseenMessages.some(m =>
      m.mentions?.includes(currentUserKey)
    );
    // Update state with hasUnread: true, hasMention
  }
}
```

### Pattern 2: Mention Storage in ChatMessage (CRDT-Safe)
**What:** Mentions are stored as a `Vec<String>` of public key hexes in the ChatMessage struct. The content field contains the raw text with `@<pubkey_prefix>` markers. Display names are resolved at render time.
**When to use:** For all mention handling.

```rust
// chat/message.rs - Updated struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reconcile, Hydrate)]
pub struct ChatMessage {
    pub id: String,
    pub sender_key: String,
    pub sender_name: String,
    pub content: String,
    pub timestamp: i64,
    pub mentions: Vec<String>,  // NEW: Vec of mentioned peer public key hexes
}
```

**Frontend mention format in content:** `@[pubkey_hex_prefix_8chars]` embedded in text.
At render time: regex match `@\[([a-f0-9]{8,})\]` and replace with resolved display name.

**Why this format:**
- Public keys are stable (immune to name changes)
- 8-char prefix is sufficient for disambiguation within a swarm
- Square bracket delimiters prevent false matches with email addresses or other @ usage
- Content is human-readable even without rendering (shows truncated key)

### Pattern 3: Mention Autocomplete Popup
**What:** When user types `@` in the message input, a popup appears above the input showing filterable list of peers in the current swarm.
**When to use:** For the @mention picker UI.

```svelte
<!-- MentionPicker.svelte -->
<script lang="ts">
  let {
    peers,
    filter,
    onSelect,
    onClose,
    inputRect
  }: {
    peers: Array<{ publicKey: string; displayName: string }>;
    filter: string;
    onSelect: (publicKey: string, displayName: string) => void;
    onClose: () => void;
    inputRect: DOMRect;
  } = $props();

  let filteredPeers = $derived(
    peers.filter(p =>
      p.displayName.toLowerCase().includes(filter.toLowerCase())
    )
  );

  let selectedIndex = $state(0);
</script>

<div class="mention-picker" style="bottom: {window.innerHeight - inputRect.top + 4}px; left: {inputRect.left}px;">
  {#each filteredPeers as peer, i}
    <button
      class="mention-option"
      class:selected={i === selectedIndex}
      onclick={() => onSelect(peer.publicKey, peer.displayName)}
    >
      <span class="peer-name">{peer.displayName}</span>
      <span class="peer-key">{peer.publicKey.substring(0, 8)}...</span>
    </button>
  {/each}
</div>
```

### Pattern 4: Unread Dot in Channel List
**What:** A small dot indicator to the right of each channel name indicating unread status. Regular unread = muted color dot. Mention unread = accent/highlight color dot.
**When to use:** For all channel list entries.

```svelte
<!-- In ChannelList.svelte channel button -->
<button class="channel-button" ...>
  <span class="channel-hash">#</span>
  <span class="channel-name">{channel.name}</span>
  {#if unreadStore.hasUnread(swarmId, channel.id)}
    <span
      class="unread-dot"
      class:mention={unreadStore.hasMention(swarmId, channel.id)}
    ></span>
  {/if}
</button>
```

```css
.unread-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--text-muted);  /* regular unread: subtle gray dot */
  flex-shrink: 0;
  margin-left: auto;
}

.unread-dot.mention {
  background: var(--accent-amber);  /* mention unread: amber dot */
}
```

### Anti-Patterns to Avoid
- **Timestamp-based unread tracking:** CRDT sync can deliver messages out of order. A "last read timestamp" high-water mark will produce phantom unreads when older messages arrive via late sync. Use message ID sets instead (Pitfall 6).
- **Storing display names in mention references:** Display names change. If you store `@Bob` in the message content and Bob changes his name to `Robert`, the mention breaks. Store public key references; resolve names at render time.
- **Using `@html` without sanitization:** Message content is peer-controlled. Using `{@html}` for mention rendering opens XSS vectors. Sanitize first (escape `<`, `>`), then apply mention rendering.
- **Syncing unread state via Automerge:** Unread state is per-device, not per-identity. Putting it in a CRDT would create conflicts and leak reading habits. Use local-only tauri-plugin-store.
- **Blocking the UI during unread recalculation:** After a large CRDT sync, recalculating unread for many channels could be slow. Do it incrementally (only for the channel that received the update).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Unread persistence | Custom file-based storage | tauri-plugin-store (`unread.json`) | Already used for swarms.json and contacts.json. Same API. Handles atomic writes and JSON serialization. |
| Mention text parsing | Custom string parser | Simple regex on structured `@[pubkey]` format | Structured format avoids Unicode edge cases and ambiguity. The regex is trivial: `/@\[([a-f0-9]+)\]/g` |
| Name resolution | Inline name lookups | `contactsStore.resolveName()` | Already exists and handles the petname > self-asserted > truncated-key fallback chain. |
| Autocomplete filtering | Custom filter logic | Svelte 5 `$derived` with `.filter()` | Reactive by default, updates as user types. No debounce needed for small peer lists (<100). |

**Key insight:** Phase 8 adds no new backend dependencies. Everything builds on existing patterns (tauri-plugin-store for persistence, Automerge for message schema, Svelte 5 stores for reactive state). The complexity is in the frontend UI (autocomplete popup, dot indicators, mention rendering), not in new infrastructure.

## Common Pitfalls

### Pitfall 1: CRDT Sync Delivers Out-of-Order Messages, Breaking Timestamp-Based Unread
**What goes wrong:** User reads all messages. Later, an offline peer syncs and Automerge merges in messages with timestamps BEFORE the last-read timestamp. Unread count stays at 0 even though new (old-timestamped) messages appeared.
**Why it happens:** CRDT merge preserves causal ordering but can insert messages at any point in the timeline.
**How to avoid:** Track unread by message ID set, not by timestamp. `unread = total_message_ids - seen_message_ids`. This is immune to reordering.
**Warning signs:** Unread count goes to 0 after viewing, then stays at 0 even after sync brings new messages.

### Pitfall 2: ChatMessage Schema Change Breaks Existing Documents
**What goes wrong:** Adding a `mentions: Vec<String>` field to `ChatMessage` and loading an existing `.automerge` file that was created without the `mentions` field causes a hydration error or panic.
**Why it happens:** `autosurgeon::hydrate` expects the document schema to match the struct. Missing fields need a default.
**How to avoid:** Add `#[serde(default)]` to the `mentions` field. Autosurgeon's Hydrate derive handles missing fields by using the Default impl (empty Vec for `Vec<String>`). Test by loading an existing pre-mentions document.
**Warning signs:** "Failed to hydrate document" error when opening channels that had messages before the schema change.

### Pitfall 3: Message Input Loses Focus During Autocomplete Interaction
**What goes wrong:** User types `@` and the popup appears. Clicking on a popup option causes the input to lose focus. The selected mention is inserted but the cursor position is wrong, or the input needs to be re-focused manually.
**Why it happens:** Clicking a button in the popup triggers a blur event on the input. By the time the click handler runs, the input's selection/cursor state is lost.
**How to avoid:** Use `onmousedown` with `event.preventDefault()` on popup buttons instead of `onclick`. This prevents the blur event from firing. After insertion, manually set the input's selection position.
**Warning signs:** Input loses focus every time a mention is selected from the popup.

### Pitfall 4: Unread State File Grows Unboundedly
**What goes wrong:** The `unread.json` store file grows larger over time because the "seen message IDs" set accumulates every message ID the user has ever read. After months of use with active channels, the file becomes megabytes.
**Why it happens:** Message ID sets are append-only. No cleanup mechanism.
**How to avoid:** Compact the seen set. When a user views a channel, instead of adding IDs to the set, store just the total message count at time of viewing. Unread = current_total - stored_total. This is a single integer per channel. For the mention distinction, separately track whether any unseen messages contain mentions.
**Warning signs:** App startup slows down as `unread.json` grows.

### Pitfall 5: Mention Autocomplete Shows Stale Peer List
**What goes wrong:** User types `@` and sees peers who have disconnected or are not in the swarm. Or new peers who just connected are missing from the list.
**Why it happens:** The peer list for autocomplete is sourced from the wrong store. The `networkStore.peers` only shows currently connected peers, not all known peers in the swarm (some may be offline).
**How to avoid:** Source the autocomplete list from unique `sender_key` values in the current channel's message history PLUS currently connected peers. This covers both active and historically-present peers. Alternatively, maintain a "known peers in swarm" set built from all message history.
**Warning signs:** The autocomplete popup is either too restrictive (only online peers) or too broad (peers from other swarms).

## Code Examples

### Unread Store (Frontend)
```typescript
// stores/unread.svelte.ts
// Source: Built on existing store patterns from chat.svelte.ts and swarm.svelte.ts

import { invoke } from '@tauri-apps/api/core';
import { onChatMessagesUpdated, getMessages, type ChatMessage, type UnlistenFn } from '../tauri';

interface ChannelUnreadState {
  totalSeen: number;        // Message count when user last viewed this channel
  hasUnread: boolean;
  hasMention: boolean;
}

let states = $state<Record<string, ChannelUnreadState>>({});
let initialized = $state(false);
let unlisten: UnlistenFn | null = null;

async function initialize(currentUserKey: string) {
  if (initialized) return;

  // Load persisted state from Rust-side store
  try {
    const saved = await invoke<Record<string, { totalSeen: number }>>('get_unread_state');
    for (const [key, value] of Object.entries(saved)) {
      states[key] = { totalSeen: value.totalSeen, hasUnread: false, hasMention: false };
    }
  } catch {
    // First run or corrupted store -- start fresh
  }

  // Listen for sync updates on non-active channels
  unlisten = await onChatMessagesUpdated(async (update) => {
    // Recalculate unread for the updated channel
    await recalculate(update.swarm_id, update.channel_id, currentUserKey);
  });

  initialized = true;
}

async function recalculate(swarmId: string, channelId: string, currentUserKey: string) {
  const key = `${swarmId}/${channelId}`;
  const messages = await getMessages(swarmId, channelId);
  const state = states[key] ?? { totalSeen: 0, hasUnread: false, hasMention: false };

  const unseenCount = messages.length - state.totalSeen;
  if (unseenCount > 0) {
    const unseenMessages = messages.slice(state.totalSeen);
    const hasMention = unseenMessages.some(m =>
      (m as any).mentions?.includes(currentUserKey)
    );
    states = { ...states, [key]: { ...state, hasUnread: true, hasMention } };
  }
}

async function markRead(swarmId: string, channelId: string, messageCount: number) {
  const key = `${swarmId}/${channelId}`;
  states = { ...states, [key]: { totalSeen: messageCount, hasUnread: false, hasMention: false } };

  // Persist to disk
  await invoke('mark_channel_read', { swarmId, channelId, totalSeen: messageCount });
}

function hasUnread(swarmId: string, channelId: string): boolean {
  return states[`${swarmId}/${channelId}`]?.hasUnread ?? false;
}

function hasMention(swarmId: string, channelId: string): boolean {
  return states[`${swarmId}/${channelId}`]?.hasMention ?? false;
}

function hasSwarmUnread(swarmId: string): boolean {
  return Object.entries(states).some(([key, s]) => key.startsWith(`${swarmId}/`) && s.hasUnread);
}

function hasSwarmMention(swarmId: string): boolean {
  return Object.entries(states).some(([key, s]) => key.startsWith(`${swarmId}/`) && s.hasMention);
}

function cleanup() {
  if (unlisten) { unlisten(); unlisten = null; }
  states = {};
  initialized = false;
}

export const unreadStore = {
  get states() { return states; },
  get initialized() { return initialized; },
  initialize,
  markRead,
  hasUnread,
  hasMention,
  hasSwarmUnread,
  hasSwarmMention,
  cleanup,
};
```

### ChatMessage Schema Change (Rust)
```rust
// chat/message.rs - Updated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Reconcile, Hydrate)]
pub struct ChatMessage {
    pub id: String,
    pub sender_key: String,
    pub sender_name: String,
    pub content: String,
    pub timestamp: i64,
    #[serde(default)]  // CRITICAL: backward compat with existing docs
    pub mentions: Vec<String>,
}
```

### Mention Rendering in MessageList
```svelte
<!-- In MessageList.svelte -->
<script lang="ts">
  import { contactsStore } from '../../stores/contacts.svelte';

  // Parse content for mention tokens: @[pubkey_prefix]
  function renderMentionContent(content: string): Array<{ type: 'text' | 'mention'; value: string; publicKey?: string }> {
    const parts: Array<{ type: 'text' | 'mention'; value: string; publicKey?: string }> = [];
    const regex = /@\[([a-f0-9]{8,})\]/g;
    let lastIndex = 0;
    let match;

    while ((match = regex.exec(content)) !== null) {
      // Text before the mention
      if (match.index > lastIndex) {
        parts.push({ type: 'text', value: content.slice(lastIndex, match.index) });
      }
      // The mention itself
      const pubkeyPrefix = match[1];
      const displayName = contactsStore.resolveName(pubkeyPrefix, pubkeyPrefix.substring(0, 8) + '...');
      parts.push({ type: 'mention', value: `@${displayName}`, publicKey: pubkeyPrefix });
      lastIndex = match.index + match[0].length;
    }

    // Remaining text
    if (lastIndex < content.length) {
      parts.push({ type: 'text', value: content.slice(lastIndex) });
    }

    return parts;
  }
</script>

<!-- In message rendering -->
{#each renderMentionContent(msg.content) as part}
  {#if part.type === 'mention'}
    <button class="mention-link" onclick={() => openContactInfo(part.publicKey)}>
      {part.value}
    </button>
  {:else}
    <span>{part.value}</span>
  {/if}
{/each}
```

### Unread Persistence (Rust Backend)
```rust
// commands/unread.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize, Clone)]
pub struct ChannelReadState {
    pub total_seen: usize,
}

#[tauri::command]
pub fn mark_channel_read(
    app: AppHandle,
    swarm_id: String,
    channel_id: String,
    total_seen: usize,
) -> Result<(), String> {
    let store = app.store("unread.json")
        .map_err(|e| format!("Store error: {}", e))?;

    let key = format!("{}/{}", swarm_id, channel_id);
    let state = ChannelReadState { total_seen };
    store.set(key, serde_json::to_value(&state).unwrap());
    store.save().map_err(|e| format!("Save error: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn get_unread_state(
    app: AppHandle,
) -> Result<HashMap<String, ChannelReadState>, String> {
    let store = app.store("unread.json")
        .map_err(|e| format!("Store error: {}", e))?;

    let mut result = HashMap::new();
    for (key, value) in store.entries() {
        if let Ok(state) = serde_json::from_value::<ChannelReadState>(value.clone()) {
            result.insert(key.clone(), state);
        }
    }

    Ok(result)
}
```

## Discretion Recommendations

These are the areas marked as "Claude's Discretion" in CONTEXT.md. Based on codebase analysis:

### 1. Unread Dot Positioning on Channel Entries
**Recommendation:** Place the dot at the far right of the channel button, using `margin-left: auto` in the flex layout. This follows the existing layout where `channel-hash` is left, `channel-name` fills the middle, and the dot sits at the end. The existing `.channel-button` already uses `display: flex; align-items: center;` which makes this trivial.

### 2. Swarm-Level Unread Indicator Style
**Recommendation:** A small dot to the right of the swarm name in `SwarmSelector.svelte`, following the same dot style as channels. Two colors: muted dot for regular unread, amber dot for mention-containing unread. This is consistent with the channel-level indicator and the existing status dot pattern used in `PeerList.svelte` and the network status indicator.

### 3. Mention Token Rendering in the Message Input
**Recommendation:** Keep the current `<input type="text">` element. When the user selects a mention from the autocomplete, insert the display name as plain text (e.g., `@Bob`) into the input value. Store the mapping of display-name-to-pubkey in component state. On send, replace each `@DisplayName` with `@[pubkey_hex_prefix]` before calling `chatStore.send()`. This avoids the complexity of `contenteditable` while still providing a functional mention input. The raw input shows `@Bob` and the sent message stores `@[8a3f02bc]`.

### 4. Mention Highlight Visual Treatment in Chat View
**Recommendation:** Messages that mention the current user get a subtle left border accent and a slightly different background, similar to how GitHub highlights comment mentions. Use `border-left: 3px solid var(--accent-amber)` and `background: rgba(255, 176, 0, 0.05)` on the message row. This is noticeable but not overwhelming.

### 5. @name Text Rendering in Sent Messages
**Recommendation:** Render @mentions as inline colored text (not a pill/badge). Use `color: var(--accent-amber); font-weight: 600; cursor: pointer;` to make mentions visually distinct from regular text. Clicking opens the peer's contact info (as per locked decision). This matches the monospace terminal aesthetic of the app -- pills/badges would feel out of place.

### 6. Unread Clearing Trigger
**Recommendation:** Clear unread on channel open (when `handleChannelClick` fires in ChannelList.svelte). This is simpler than scroll-based detection and matches user expectation: "I clicked on the channel, so I've acknowledged the new messages." Scroll-based detection adds complexity (IntersectionObserver, scroll position tracking) for marginal UX benefit in this app's context.

### 7. Live Message Read Behavior
**Recommendation:** Messages arriving in the currently viewed channel are automatically marked as read (auto-read). Since the current MessageList auto-scrolls to bottom on new messages (via the `$effect` in MessageList.svelte), the user is always seeing new messages. There is no scenario where the user is "scrolled up and misses" messages because the auto-scroll behavior forces visibility. If scroll-back is added later, revisit this to only auto-read when scrolled to bottom.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Timestamp-based "last read" for unread | Message count or ID-set based tracking | Always better for CRDTs | Prevents phantom unreads from out-of-order sync |
| Name-based @mention matching | Public key-based mentions with render-time name resolution | Standard in P2P/decentralized apps | Immune to name changes and impersonation |
| `createEventDispatcher` + `on:event` (Svelte 4) | Callback props `onFoo` (Svelte 5) | Svelte 5 | Must use callback props for all new components |
| `new App()` (Svelte 4) | `mount()` (Svelte 5) | Svelte 5 | Already done in codebase |

**Deprecated/outdated:**
- Svelte 4 event dispatchers: Must not use `dispatch()` or `on:event` syntax
- Svelte 4 `$:` reactive declarations: Use `$derived` and `$effect` instead

## Open Questions

1. **Peer list source for autocomplete**
   - What we know: `networkStore.peers` shows currently connected peers. Message history contains `sender_key` from past peers.
   - What's unclear: Should the autocomplete show only online peers, all historically-seen peers, or both? The locked decision says "peers in the current swarm" which implies a broader set than just online.
   - Recommendation: Combine unique `sender_key` values from the current channel's message history with currently connected peers. This gives the broadest useful set. Deduplicate by public key.

2. **Mention format for full vs prefix public keys**
   - What we know: Full public keys are 64 hex chars. Using the full key in message content makes messages long and ugly in the raw CRDT data.
   - What's unclear: Is an 8-char prefix sufficient for disambiguation within a swarm?
   - Recommendation: Store the full public key in the `mentions` Vec (for reliable matching) but use an 8-char prefix in the content text `@[8a3f02bc]` for readability. On render, match against the `mentions` Vec entries, not the inline prefix.

3. **Unread count persistence strategy: count vs ID set**
   - What we know: Message ID sets are immune to CRDT reordering but grow unboundedly. Message counts are compact but can drift if messages are somehow removed.
   - What's unclear: Can Automerge messages ever decrease in count? (Answer: effectively no -- Automerge is append-only for Vec items, tombstones don't reduce count.)
   - Recommendation: Use message count (`totalSeen: number`). It is compact (single integer per channel), does not grow over time, and is reliable because Automerge message lists are monotonically growing. The count approach is immune to CRDT reordering because we compare total count, not specific messages.

## Sources

### Primary (HIGH confidence)
- Codebase analysis: `src/lib/stores/chat.svelte.ts`, `src/lib/components/chat/MessageList.svelte`, `src/lib/components/layout/ChannelList.svelte`, `src-tauri/src/chat/message.rs`, `src-tauri/src/chat/mod.rs` -- All integration points verified against actual source code
- `.planning/research/ARCHITECTURE.md` -- Unread tracking and mention detection architectural patterns, verified against codebase
- `.planning/research/PITFALLS.md` -- Pitfall 6 (CRDT reordering breaks timestamp-based unread), Pitfall 10 (Unicode mention parsing)
- `.planning/research/FEATURES.md` -- Feature design for unread indicators and mention detection, including P2P edge cases
- `.planning/research/STACK.md` -- No new dependencies needed confirmation
- Svelte 5 patterns: `~/.claude/projects/-Users-annon-projects-aether/memory/svelte5-patterns.md` -- Verified rune patterns, callback props

### Secondary (MEDIUM confidence)
- `.planning/research/STACK.md` Section 8 (Unread) -- Frontend-only tracking recommendation, tauri-plugin-store from JS side via LazyStore
- autosurgeon `#[serde(default)]` backward compatibility for new fields -- Based on autosurgeon 0.10 using serde derives; needs validation with actual document load test

### Tertiary (LOW confidence)
- Contenteditable vs textarea vs input for mention input -- Recommended plain `<input>` approach based on complexity analysis, but may need revision if user expectations differ
- 8-char public key prefix uniqueness -- Probabilistically safe for small swarms (<100 peers) but collision risk increases with scale. Not verified with mathematical analysis.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- No new dependencies. All patterns reuse existing codebase patterns.
- Architecture: HIGH -- Unread is a well-understood pattern. Mentions use public key references (validated by prior research).
- Pitfalls: HIGH -- CRDT reordering pitfall is documented in existing research. Schema backward compat is a known autosurgeon concern.
- UI/UX discretion items: MEDIUM -- Recommendations based on codebase aesthetic analysis and Discord/Slack conventions, but taste-dependent.

**Research date:** 2026-02-16
**Valid until:** 2026-03-16 (30 days -- stable domain, no fast-moving dependencies)