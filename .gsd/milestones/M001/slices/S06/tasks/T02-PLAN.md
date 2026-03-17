# T02: 05-persistent-chat 02

**Slice:** S06 — **Milestone:** M001

## Description

Wire the Rust chat engine to the Svelte 5 frontend: Tauri IPC commands, TypeScript types, and a reactive chat store.

Purpose: Bridges the backend CRDT engine (Plan 01) to the frontend. The store provides the reactive data layer that UI components (Plan 03) will consume. Event-driven updates ensure the UI refreshes when peer sync brings new messages.

Output: Working IPC bridge where frontend can send messages, fetch message history, and reactively update when sync delivers new messages.

## Must-Haves

- [ ] "Frontend can invoke send_message command and receive the sent ChatMessage back"
- [ ] "Frontend can invoke get_messages command and receive Vec<ChatMessage> for a channel"
- [ ] "Frontend receives chat-messages-updated events when sync brings in new messages from peers"
- [ ] "Chat store manages reactive message state, handles sending, and listens for sync updates"

## Files

- `src-tauri/src/commands/chat.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/chat.svelte.ts`
