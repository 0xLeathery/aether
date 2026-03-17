# S09: Unread Mentions

**Goal:** Build the unread tracking system end-to-end: Rust backend persistence, reactive frontend store, channel-level dot indicators, swarm-level unread indicators, "Mark as read" context menu action, and automatic clearing on channel view.
**Demo:** Build the unread tracking system end-to-end: Rust backend persistence, reactive frontend store, channel-level dot indicators, swarm-level unread indicators, "Mark as read" context menu action, and automatic clearing on channel view.

## Must-Haves


## Tasks

- [x] **T01: 08-unread-mentions 01** `est:3min`
  - Build the unread tracking system end-to-end: Rust backend persistence, reactive frontend store, channel-level dot indicators, swarm-level unread indicators, "Mark as read" context menu action, and automatic clearing on channel view.

Purpose: Users can see at a glance which channels have new messages, enabling efficient navigation across multi-channel swarms. This is the foundation that Plan 03 extends with mention-aware dot colors.

Output: Working unread indicators on channels and swarms, persisted across restarts, cleared on channel view or via context menu.
- [x] **T02: 08-unread-mentions 02** `est:5min`
  - Add the mentions field to ChatMessage (Rust CRDT + TypeScript type) and build the @mention autocomplete picker in the message input, enabling users to mention peers by typing @ and selecting from a filtered popup.

Purpose: Mentions let users direct attention to specific peers. Storing public key references ensures mentions survive name changes. The autocomplete picker provides a discoverable, Discord-like UX.

Output: ChatMessage schema with mentions field (backward-compatible), MentionPicker component, MessageInput with @ detection and mention insertion.
- [x] **T03: 08-unread-mentions 03** `est:5min`
  - Wire mention rendering in the chat view (colored clickable @names, message highlighting for current-user mentions) and upgrade the unread store to distinguish mention-containing unreads with a distinct amber dot color.

Purpose: This plan completes Phase 8 by connecting the mention data (from Plan 02) to both the visual rendering in chat and the unread indicator system (from Plan 01). Users can see highlighted mentions, click them to view contact info, and know at a glance when they've been mentioned via the amber unread dot.

Output: Full mention rendering in chat, clickable mentions opening ContactEditor, mention-aware unread dots (amber for mentions, gray for regular unreads).

## Files Likely Touched

- `src-tauri/src/commands/unread.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/unread.svelte.ts`
- `src/lib/components/layout/ChannelList.svelte`
- `src/lib/components/channel/ChannelContextMenu.svelte`
- `src/lib/components/swarm/SwarmSelector.svelte`
- `src/lib/components/chat/ChatPanel.svelte`
- `src-tauri/src/chat/message.rs`
- `src-tauri/src/commands/chat.rs`
- `src/lib/tauri.ts`
- `src/lib/components/chat/MentionPicker.svelte`
- `src/lib/components/chat/MessageInput.svelte`
- `src/lib/components/chat/MessageList.svelte`
- `src/lib/stores/unread.svelte.ts`
- `src/lib/components/contacts/ContactEditor.svelte`
