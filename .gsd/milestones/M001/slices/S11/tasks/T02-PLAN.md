# T02: 09-peer-moderation 02

**Slice:** S11 — **Milestone:** M001

## Description

Build all moderation UI components: peer context menu (right-click), block confirmation dialog, message view filtering (hide placeholders + block removal), moderation status icons in peer list, and context menu triggers on both peer list and message author names.

Purpose: Users can see and interact with moderation controls, and moderated content is filtered appropriately in the chat view.
Output: Complete moderation UI layer with context menus, dialogs, message filtering, and status indicators.

## Must-Haves

- [ ] "Right-click on peer name in sidebar opens context menu with Mute/Hide/Block options"
- [ ] "Right-click on message author name in chat opens same moderation context menu"
- [ ] "Block action shows confirmation dialog before applying"
- [ ] "Mute and Hide actions apply instantly without confirmation"
- [ ] "Hidden peer messages show collapsed placeholder 'Message from hidden user' with click-to-reveal"
- [ ] "Blocked peer messages are fully removed from chat view (no DOM element)"
- [ ] "Muted peer messages display normally (only voice affected)"
- [ ] "Moderation status icons appear next to moderated peers in sidebar peer list"
- [ ] "Active moderation state shown in context menu (Unmute/Unhide/Unblock labels)"
- [ ] "Context menu does not appear on own user's name"

## Files

- `src/lib/components/moderation/PeerContextMenu.svelte`
- `src/lib/components/moderation/BlockConfirmDialog.svelte`
- `src/lib/components/peers/PeerList.svelte`
- `src/lib/components/chat/MessageList.svelte`
- `src/lib/components/layout/Sidebar.svelte`
