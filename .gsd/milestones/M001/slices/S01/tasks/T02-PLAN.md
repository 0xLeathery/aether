# T02: 01-foundation-identity 02

**Slice:** S01 — **Milestone:** M001

## Description

Build the complete frontend experience: setup flow for first-launch identity creation and the main app shell with profile display, deterministic avatars, and the terminal/hacker aesthetic.

Purpose: This is what users see and interact with. The setup flow implements the explain-first sovereignty approach (user decision), and the app shell establishes the "command center" visual language that persists through all subsequent phases.

Output: A fully functional identity creation flow and main application chrome. First launch shows setup wizard; subsequent launches show the main app with loaded identity.

## Must-Haves

- [ ] "First-launch user sees a sovereignty explainer before any action"
- [ ] "User can set a display name during setup"
- [ ] "User can generate an Ed25519 keypair through the setup flow"
- [ ] "After setup, user sees the main app with three-column layout"
- [ ] "User can view their public key identifier in the UI"
- [ ] "User sees a deterministic avatar generated from their public key"
- [ ] "Clicking avatar/name opens a profile popover panel"
- [ ] "Subsequent launches skip setup and load identity automatically"
- [ ] "If key generation fails, user sees actionable error with retry option"

## Files

- `src/lib/tauri.ts`
- `src/lib/stores/identity.ts`
- `src/lib/components/setup/Welcome.svelte`
- `src/lib/components/setup/SetupName.svelte`
- `src/lib/components/setup/GenerateKey.svelte`
- `src/lib/components/setup/SetupComplete.svelte`
- `src/lib/components/setup/SetupFlow.svelte`
- `src/lib/components/layout/AppShell.svelte`
- `src/lib/components/layout/Sidebar.svelte`
- `src/lib/components/layout/ChannelList.svelte`
- `src/lib/components/layout/MainContent.svelte`
- `src/lib/components/profile/Avatar.svelte`
- `src/lib/components/profile/ProfilePopover.svelte`
- `src/App.svelte`
- `src/app.css`
