# T01: 10-desktop-notifications 01

**Slice:** S13 — **Milestone:** M001

## Description

Add native desktop notifications for incoming messages and @mentions using tauri-plugin-notification.

Purpose: Satisfies the final three v1.1 requirements (NOTF-01/02/03), completing the v1.1 Community milestone. Users will receive timely native OS alerts for conversation activity when the app is not focused.

Output: Working notification pipeline — plugin installed, notification store created, integrated into app initialization.

## Must-Haves

- [ ] "User receives a native desktop notification when a new message arrives while the app window is not focused"
- [ ] "User receives a distinct notification when mentioned by @name, with title format differentiating it from general messages"
- [ ] "Notification displays the sender's resolved name (petname > self-asserted > key) and a truncated message preview"
- [ ] "No notification fires for the user's own messages"
- [ ] "No notification fires for messages from moderated (muted/hidden/blocked) peers"
- [ ] "No notification fires when the window is focused and user is viewing the same channel"
- [ ] "Rapid messages in the same channel are throttled to avoid notification spam"

## Files

- `src-tauri/Cargo.toml`
- `src-tauri/src/lib.rs`
- `src-tauri/capabilities/default.json`
- `package.json`
- `src/lib/stores/notification.svelte.ts`
- `src/App.svelte`
