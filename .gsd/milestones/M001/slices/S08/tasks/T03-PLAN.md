# T03: 07-channel-management 03

**Slice:** S08 — **Milestone:** M001

## Description

Build the channel management UI: interactive channel list with active state, create dialog, right-click context menu with rename/delete, type-to-confirm deletion, and wire MainContent to use activeChannelId from the swarm store.

Purpose: This is the user-facing layer that makes channel management tangible. Without the UI, the backend infrastructure from Plans 01-02 is invisible. The user specifically requested Discord-familiar interaction patterns.

Output: Rewritten ChannelList.svelte with full interactivity, three new channel/ components (CreateChannelDialog, DeleteChannelDialog, ChannelContextMenu), and updated MainContent.svelte.

## Must-Haves

- [ ] "User can see channels listed with # prefix, general pinned at top, rest sorted alphabetically"
- [ ] "User can click a channel to switch the active chat view to that channel"
- [ ] "Swarm creator sees a [+] button in the channel header and can create channels via a modal dialog"
- [ ] "Swarm creator can right-click a non-default channel to see a context menu with Rename and Delete options"
- [ ] "Rename triggers inline editing with Enter to confirm, updating the channel name"
- [ ] "Delete opens a type-to-confirm dialog requiring the channel name to enable the delete button"
- [ ] "The voice channel appears in the list but does not switch the chat view"
- [ ] "Non-creators do not see the [+] button or context menu options"
- [ ] "MainContent uses activeChannelId from swarm store instead of hardcoded first channel"

## Files

- `src/lib/components/layout/ChannelList.svelte`
- `src/lib/components/layout/MainContent.svelte`
- `src/lib/components/channel/CreateChannelDialog.svelte`
- `src/lib/components/channel/DeleteChannelDialog.svelte`
- `src/lib/components/channel/ChannelContextMenu.svelte`
