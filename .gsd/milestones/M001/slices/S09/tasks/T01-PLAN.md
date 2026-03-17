# T01: 08-unread-mentions 01

**Slice:** S09 — **Milestone:** M001

## Description

Build the unread tracking system end-to-end: Rust backend persistence, reactive frontend store, channel-level dot indicators, swarm-level unread indicators, "Mark as read" context menu action, and automatic clearing on channel view.

Purpose: Users can see at a glance which channels have new messages, enabling efficient navigation across multi-channel swarms. This is the foundation that Plan 03 extends with mention-aware dot colors.

Output: Working unread indicators on channels and swarms, persisted across restarts, cleared on channel view or via context menu.

## Must-Haves

- [ ] "User sees an unread dot on channels with new messages after sync or remote send"
- [ ] "Unread dot clears when user clicks on the channel"
- [ ] "User sees an unread indicator on swarms containing channels with unread messages"
- [ ] "Right-clicking a channel shows 'Mark as read' in the context menu"
- [ ] "Unread state persists across app restarts"

## Files

- `src-tauri/src/commands/unread.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/unread.svelte.ts`
- `src/lib/components/layout/ChannelList.svelte`
- `src/lib/components/channel/ChannelContextMenu.svelte`
- `src/lib/components/swarm/SwarmSelector.svelte`
- `src/lib/components/chat/ChatPanel.svelte`
