# S01: Foundation Identity

**Goal:** Scaffold the Tauri v2 + Svelte project and implement the complete Rust identity module with Ed25519 key generation, platform keychain storage, and display name management.
**Demo:** Scaffold the Tauri v2 + Svelte project and implement the complete Rust identity module with Ed25519 key generation, platform keychain storage, and display name management.

## Must-Haves


## Tasks

- [x] **T01: 01-foundation-identity 01** `est:9min`
  - Scaffold the Tauri v2 + Svelte project and implement the complete Rust identity module with Ed25519 key generation, platform keychain storage, and display name management.

Purpose: Establish the runtime foundation (Tauri desktop app) and the core identity backend that all subsequent UI and networking phases depend on. This is the "sovereign node" engine.

Output: A compilable, launchable Tauri desktop app with a working Rust identity module exposing Tauri IPC commands. Frontend is minimal placeholder — UI comes in Plan 02.
- [x] **T02: 01-foundation-identity 02** `est:12min`
  - Build the complete frontend experience: setup flow for first-launch identity creation and the main app shell with profile display, deterministic avatars, and the terminal/hacker aesthetic.

Purpose: This is what users see and interact with. The setup flow implements the explain-first sovereignty approach (user decision), and the app shell establishes the "command center" visual language that persists through all subsequent phases.

Output: A fully functional identity creation flow and main application chrome. First launch shows setup wizard; subsequent launches show the main app with loaded identity.

## Files Likely Touched

- `package.json`
- `vite.config.ts`
- `svelte.config.js`
- `tsconfig.json`
- `src/main.ts`
- `src/App.svelte`
- `src/app.css`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `src-tauri/capabilities/default.json`
- `src-tauri/src/main.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/error.rs`
- `src-tauri/src/identity/mod.rs`
- `src-tauri/src/identity/keypair.rs`
- `src-tauri/src/identity/storage.rs`
- `src-tauri/src/identity/display.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/identity.rs`
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
