# S13: Desktop Notifications

**Goal:** Add native desktop notifications for incoming messages and @mentions using tauri-plugin-notification.
**Demo:** Add native desktop notifications for incoming messages and @mentions using tauri-plugin-notification.

## Must-Haves


## Tasks

- [x] **T01: 10-desktop-notifications 01** `est:3min`
  - Add native desktop notifications for incoming messages and @mentions using tauri-plugin-notification.

Purpose: Satisfies the final three v1.1 requirements (NOTF-01/02/03), completing the v1.1 Community milestone. Users will receive timely native OS alerts for conversation activity when the app is not focused.

Output: Working notification pipeline — plugin installed, notification store created, integrated into app initialization.

## Files Likely Touched

- `src-tauri/Cargo.toml`
- `src-tauri/src/lib.rs`
- `src-tauri/capabilities/default.json`
- `package.json`
- `src/lib/stores/notification.svelte.ts`
- `src/App.svelte`
