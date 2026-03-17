---
id: S09
parent: M001
milestone: M001
provides:
  - "Rust mark_channel_read/get_unread_state commands with unread.json persistence"
  - "Reactive unreadStore with hasUnread/hasMention/hasSwarmUnread/hasSwarmMention"
  - "Channel-level unread dot indicators in ChannelList"
  - "Swarm-level unread dot indicators in SwarmSelector"
  - "Mark as read context menu action for all channels"
  - "Auto-clear unread on channel view in ChatPanel"
  - "ChatMessage with mentions Vec<String> field (#[serde(default)] backward compat)"
  - "MentionPicker autocomplete component with filtered peer display"
  - "MessageInput with @ detection, keyboard navigation, mention insertion"
  - "Mention public key extraction and @[pubkey] content transformation on send"
  - "mentionPeers derivation from message history + connected peers"
  - "Mention rendering in chat: @[pubkey] tokens parsed into amber @DisplayName clickable buttons"
  - "Message highlighting: amber left border + tinted background for messages mentioning current user"
  - "ContactEditor popup on mention click for setting petnames on mentioned peers"
  - "Mention-aware unread tracking: hasMention populated from actual message mentions data"
  - "Two-tier unread dot colors: amber for mention unreads, gray for regular unreads"
requires: []
affects: []
key_files: []
key_decisions:
  - "Count-based unread tracking (totalSeen vs message count) for efficient comparison without per-message read receipts"
  - "Context menu opened for all users on all channels; Rename/Delete restricted to creator on non-default channels"
  - "hasMention hardcoded false until Plan 03 adds mention-aware ChatMessage field"
  - "Mentions stored as Vec<String> of hex public keys for name-change resilience"
  - "#[serde(default)] on mentions field ensures pre-mention documents deserialize without errors"
  - "MentionPicker uses onmousedown with preventDefault to avoid input blur on click"
  - "mentionMap tracks displayName->publicKey mapping per message for @[pubkey] content transformation"
  - "ContactEditor opened via fixed-position backdrop overlay (adapted from plan's svelte:window approach which is invalid inside blocks in Svelte 5)"
  - "Display names resolved at render time via contactsStore.resolveName for live petname updates"
  - "Mention detection in unread store guards against null _currentUserKey for safety"
patterns_established:
  - "Unread store pattern: event-driven recalculation via onChatMessagesUpdated, local state + async persistence"
  - "Composite key pattern: swarmId/channelId for flat state lookup"
  - "Autocomplete picker pattern: parent manages selectedIndex state, child renders filtered list"
  - "Mention content transform: @DisplayName in input, @[pubkey] in stored content, mentions array for queries"
  - "ContentPart rendering pattern: parse content into typed parts (text/mention), render each with appropriate component"
  - "Click-to-edit contact pattern: mention click opens ContactEditor popup at cursor position"
observability_surfaces: []
drill_down_paths: []
duration: 5min
verification_result: passed
completed_at: 2026-02-16
blocker_discovered: false
---
# S09: Unread Mentions

**# Phase 08 Plan 01: Unread Tracking Summary**

## What Happened

# Phase 08 Plan 01: Unread Tracking Summary

**End-to-end unread tracking with Rust persistence, reactive Svelte store, channel/swarm dot indicators, context menu mark-as-read, and auto-clearing on channel view**

## Performance

- **Duration:** 3 min
- **Started:** 2026-02-16T07:07:33Z
- **Completed:** 2026-02-16T07:10:54Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments
- Rust backend with mark_channel_read/get_unread_state commands persisting to unread.json via tauri-plugin-store
- Reactive unreadStore with event-driven recalculation triggered by chat-messages-updated events
- Channel unread dots (gray, amber placeholder for mention dots wired in Plan 03)
- Swarm-level unread indicators when any child channel has unreads
- Context menu "Mark as read" available for all users on all channels
- Automatic unread clearing when user views a channel in ChatPanel

## Task Commits

Each task was committed atomically:

1. **Task 1: Create unread persistence backend and reactive frontend store** - `5d00298` (feat)
2. **Task 2: Add unread dots to channels/swarms, "Mark as read" to context menu, and clearing on channel view** - `73e7c66` (feat)

## Files Created/Modified
- `src-tauri/src/commands/unread.rs` - Rust commands for unread state persistence via tauri-plugin-store
- `src-tauri/src/commands/mod.rs` - Added unread module registration
- `src-tauri/src/lib.rs` - Registered mark_channel_read and get_unread_state in generate_handler
- `src/lib/tauri.ts` - TypeScript bindings for unread commands (ChannelReadState type, markChannelRead, getUnreadState)
- `src/lib/stores/unread.svelte.ts` - Reactive unread store with event-driven recalculation and swarm-level aggregation
- `src/lib/components/layout/ChannelList.svelte` - Unread dot indicators on channels, handleMarkAsRead, removed creator-only context menu guard
- `src/lib/components/channel/ChannelContextMenu.svelte` - "Mark as read" menu item, isCreator-conditional Rename/Delete
- `src/lib/components/swarm/SwarmSelector.svelte` - Swarm-level unread dot indicators
- `src/lib/components/chat/ChatPanel.svelte` - Unread store initialization and auto-clearing on channel view

## Decisions Made
- Count-based unread tracking (totalSeen vs message count) avoids per-message read receipts while supporting accurate detection
- Context menu opened for all users on all channels; Rename/Delete items restricted to creator on non-default channels
- hasMention is hardcoded false until Plan 03 adds mention-aware ChatMessage field

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Unread tracking foundation complete for Plan 02 (mention parsing) and Plan 03 (mention-aware UI)
- hasMention getter ready for Plan 03 to activate with real mention data
- unreadStore.recalculate ready to add mention detection logic

## Self-Check: PASSED

All files verified present, all commits verified in git log.

---
*Phase: 08-unread-mentions*
*Completed: 2026-02-16*

# Phase 08 Plan 02: Mention Parsing Summary

**@mention autocomplete picker with live filtering, keyboard navigation, and pubkey-based mention storage in ChatMessage CRDT**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-16T07:13:08Z
- **Completed:** 2026-02-16T07:18:31Z
- **Tasks:** 2
- **Files modified:** 8

## Accomplishments
- ChatMessage CRDT extended with mentions: Vec<String> field, backward-compatible via #[serde(default)]
- MentionPicker autocomplete popup displaying peer names + truncated public keys with live filtering
- MessageInput upgraded with @ detection, keyboard navigation (Up/Down/Enter/Escape), and mention insertion
- Full pipeline from @DisplayName input through @[pubkey] content transform to mentions array in backend
- mentionPeers derived from message history senders + connected network peers via contactsStore name resolution

## Task Commits

Each task was committed atomically:

1. **Task 1: Add mentions field to ChatMessage CRDT and update IPC layer** - `3b0af4b` (feat)
2. **Task 2: Create MentionPicker component and upgrade MessageInput with @ autocomplete** - `b8f1287` (feat)

## Files Created/Modified
- `src-tauri/src/chat/message.rs` - ChatMessage with mentions: Vec<String> and #[serde(default)]
- `src-tauri/src/chat/mod.rs` - ChatService::send_message accepts mentions parameter
- `src-tauri/src/commands/chat.rs` - ChatMessageResponse with mentions, send_message command accepts mentions
- `src/lib/tauri.ts` - ChatMessage interface with mentions: string[], sendMessage accepts mentions
- `src/lib/components/chat/MentionPicker.svelte` - Autocomplete popup with filtered peers, keyboard selection
- `src/lib/components/chat/MessageInput.svelte` - @ detection, mention insertion, content transformation
- `src/lib/components/chat/ChatPanel.svelte` - mentionPeers derivation, handleSend with mentions
- `src/lib/stores/chat.svelte.ts` - chatStore.send() accepts and passes mentions

## Decisions Made
- Mentions stored as hex public key strings for resilience against display name changes
- #[serde(default)] on mentions field ensures backward compatibility with pre-mention CRDT documents
- MentionPicker uses onmousedown with preventDefault instead of onclick to prevent input blur
- mentionMap tracks displayName->publicKey mapping within each message for accurate content transformation
- Autocomplete parent-manages-index pattern: MessageInput owns selectedIndex, MentionPicker renders based on it

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- ChatMessage.mentions field ready for Plan 03 to query for mention-aware unread indicators
- MentionPicker UX complete; Plan 03 will wire hasMention in unreadStore using the mentions array
- @[pubkey] content format ready for display-side rendering of mention highlights

## Self-Check: PASSED

All files verified present, all commits verified in git log.

---
*Phase: 08-unread-mentions*
*Completed: 2026-02-16*

# Phase 08 Plan 03: Mention UI Summary

**Clickable amber @mentions in chat with message highlighting, ContactEditor popup on click, and mention-aware amber/gray unread dot indicators**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-16T07:20:56Z
- **Completed:** 2026-02-16T07:26:04Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Messages containing @[pubkey] tokens render as amber-colored @DisplayName clickable buttons with live name resolution
- Messages mentioning current user highlighted with amber left border and tinted background
- Clicking a mention opens ContactEditor popup at click position for setting petnames
- Unread store now detects mentions of current user in unseen messages, enabling amber dot indicators
- Full Phase 8 feature set complete: unread tracking + mention parsing + mention UI

## Task Commits

Each task was committed atomically:

1. **Task 1: Render mentions as clickable colored text and highlight messages mentioning current user** - `ba452fd` (feat)
2. **Task 2: Upgrade unread store with mention-aware tracking for distinct dot colors** - `6a26842` (feat)

## Files Created/Modified
- `src/lib/components/chat/MessageList.svelte` - Mention rendering with ContentPart parsing, ContactEditor popup, message highlighting CSS
- `src/lib/stores/unread.svelte.ts` - Mention-aware recalculate checking unseen messages for current user's public key

## Decisions Made
- ContactEditor opened via fixed-position backdrop overlay instead of svelte:window onclick (svelte:window cannot be inside blocks in Svelte 5)
- Display names resolved at render time via contactsStore.resolveName for live petname updates
- Mention detection guards against null _currentUserKey with ternary fallback to false

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed svelte:window placement inside conditional block**
- **Found during:** Task 1 (mention rendering)
- **Issue:** Plan specified `<svelte:window onclick>` inside `{#if editingContact}` block, but Svelte 5 requires svelte:window at top level
- **Fix:** Replaced with a transparent backdrop overlay div with onclick handler
- **Files modified:** src/lib/components/chat/MessageList.svelte
- **Verification:** svelte-check passes without placement error
- **Committed in:** ba452fd (Task 1 commit)

**2. [Rule 3 - Blocking] Adapted ContactEditor props to match actual interface**
- **Found during:** Task 1 (mention rendering)
- **Issue:** Plan assumed ContactEditor accepts displayName/x/y props, but actual interface is publicKey/currentPetname/onClose
- **Fix:** Used actual props with getContactPetname helper, positioned via wrapper div with fixed positioning
- **Files modified:** src/lib/components/chat/MessageList.svelte
- **Verification:** svelte-check passes, ContactEditor renders correctly
- **Committed in:** ba452fd (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** Both fixes necessary for Svelte 5 compatibility and correct component usage. No scope creep.

## Issues Encountered
None beyond the auto-fixed deviations above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 8 (Unread/Mentions) fully complete
- All three plans delivered: unread tracking (01), mention parsing (02), mention UI (03)
- Ready for Phase 9 execution

## Self-Check: PASSED

All files verified present, all commits verified in git log.

---
*Phase: 08-unread-mentions*
*Completed: 2026-02-16*
