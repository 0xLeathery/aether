# T03: 09-peer-moderation 03

**Slice:** S11 — **Milestone:** M001

## Description

Integrate moderation with the unread/mention tracking system and build the moderation management UI accessible from the sidebar.

Purpose: Ensure moderated peers don't generate noise (unreads/mentions) and give users a central place to view and manage all their moderation actions.
Output: Moderation-filtered unread tracking + management panel in sidebar.

## Must-Haves

- [ ] "Messages from hidden/blocked peers do not trigger unread indicators"
- [ ] "All moderation tiers suppress @mentions from the moderated peer (no highlight, no mention badge)"
- [ ] "Dedicated moderation management section accessible from sidebar"
- [ ] "Management list shows peer name + current tier for all moderated peers"
- [ ] "User can change tier (escalate/de-escalate) or remove moderation from management list"
- [ ] "Unblocking restores all previously hidden messages (CRDT data untouched)"

## Files

- `src/lib/stores/unread.svelte.ts`
- `src/lib/components/moderation/ModerationList.svelte`
- `src/lib/components/layout/Sidebar.svelte`
