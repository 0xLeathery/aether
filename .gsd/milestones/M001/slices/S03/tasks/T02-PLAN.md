# T02: 03-invitation-system 02

**Slice:** S03 — **Milestone:** M001

## Description

Build the frontend for swarm management: TypeScript invoke wrappers, reactive Svelte 5 swarm store, Create/Join dialogs with clipboard support, swarm selector in sidebar, and channel list driven by active swarm selection.

Purpose: Completes the invitation system user experience - users can create swarms, share Secret Codes, join swarms, and navigate between them.
Output: Fully functional swarm UI integrated into the existing three-column layout.

## Must-Haves

- [ ] "User can click Create Swarm, enter a name, and receive an aether:// Secret Code"
- [ ] "User can copy the Secret Code to clipboard with one click"
- [ ] "User can paste an aether:// code and join the corresponding swarm"
- [ ] "User can see a list of channels (General) within a joined swarm"
- [ ] "User can distinguish between multiple joined swarms in the sidebar"
- [ ] "Switching swarms updates the channel list to show that swarm's channels"

## Files

- `package.json`
- `src/lib/tauri.ts`
- `src/lib/stores/swarm.svelte.ts`
- `src/lib/components/swarm/SwarmSelector.svelte`
- `src/lib/components/swarm/InviteDialog.svelte`
- `src/lib/components/swarm/JoinDialog.svelte`
- `src/lib/components/layout/Sidebar.svelte`
- `src/lib/components/layout/ChannelList.svelte`
- `src/lib/components/layout/AppShell.svelte`
- `src/App.svelte`
