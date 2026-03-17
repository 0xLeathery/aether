# T03: 08-unread-mentions 03

**Slice:** S09 — **Milestone:** M001

## Description

Wire mention rendering in the chat view (colored clickable @names, message highlighting for current-user mentions) and upgrade the unread store to distinguish mention-containing unreads with a distinct amber dot color.

Purpose: This plan completes Phase 8 by connecting the mention data (from Plan 02) to both the visual rendering in chat and the unread indicator system (from Plan 01). Users can see highlighted mentions, click them to view contact info, and know at a glance when they've been mentioned via the amber unread dot.

Output: Full mention rendering in chat, clickable mentions opening ContactEditor, mention-aware unread dots (amber for mentions, gray for regular unreads).

## Must-Haves

- [ ] "Messages mentioning the current user are visually highlighted with a left border and tinted background"
- [ ] "@mentions in messages render as colored clickable text showing the resolved name"
- [ ] "Clicking a @mention opens the peer's contact info (ContactEditor)"
- [ ] "Mentions display the current resolved name (petname > self-asserted > key), not frozen at send time"
- [ ] "Channels with mention-containing unreads show an amber dot instead of gray"
- [ ] "Swarms with mention-containing unreads show an amber dot"

## Files

- `src/lib/components/chat/MessageList.svelte`
- `src/lib/stores/unread.svelte.ts`
- `src/lib/components/contacts/ContactEditor.svelte`
