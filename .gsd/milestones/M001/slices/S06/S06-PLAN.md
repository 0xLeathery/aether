# S06: Persistent Chat

**Goal:** Build the complete Rust chat engine: CRDT document model with Automerge, file-based persistence, and libp2p-stream sync protocol.
**Demo:** Build the complete Rust chat engine: CRDT document model with Automerge, file-based persistence, and libp2p-stream sync protocol.

## Must-Haves


## Tasks

- [x] **T01: 05-persistent-chat 01** `est:4min`
  - Build the complete Rust chat engine: CRDT document model with Automerge, file-based persistence, and libp2p-stream sync protocol.

Purpose: This is the foundational data layer for persistent chat. All message storage, sync, and conflict resolution happens here. The Automerge CRDT ensures eventual consistency without central coordination.

Output: A `chat` module in `src-tauri/src/chat/` with ChatService that manages per-channel Automerge documents, persists them to disk, and syncs between peers via libp2p-stream.
- [x] **T02: 05-persistent-chat 02** `est:3min`
  - Wire the Rust chat engine to the Svelte 5 frontend: Tauri IPC commands, TypeScript types, and a reactive chat store.

Purpose: Bridges the backend CRDT engine (Plan 01) to the frontend. The store provides the reactive data layer that UI components (Plan 03) will consume. Event-driven updates ensure the UI refreshes when peer sync brings new messages.

Output: Working IPC bridge where frontend can send messages, fetch message history, and reactively update when sync delivers new messages.
- [x] **T03: 05-persistent-chat 03** `est:3min`
  - Build the terminal-aesthetic chat UI: message list, text input, and integration into the main content area.

Purpose: This is the user-facing layer that makes persistent chat tangible. Users see messages, type replies, and watch conversations sync in real-time. The terminal aesthetic maintains visual consistency with the rest of Aether.

Output: Three chat components (ChatPanel, MessageList, MessageInput) integrated into MainContent, with a human verification checkpoint to confirm the full send/receive/persist flow works.

## Files Likely Touched

- `src-tauri/Cargo.toml`
- `src-tauri/src/error.rs`
- `src-tauri/src/chat/mod.rs`
- `src-tauri/src/chat/message.rs`
- `src-tauri/src/chat/document.rs`
- `src-tauri/src/chat/storage.rs`
- `src-tauri/src/chat/protocol.rs`
- `src-tauri/src/chat/sync.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/commands/chat.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/lib.rs`
- `src/lib/tauri.ts`
- `src/lib/stores/chat.svelte.ts`
- `src/lib/components/chat/ChatPanel.svelte`
- `src/lib/components/chat/MessageList.svelte`
- `src/lib/components/chat/MessageInput.svelte`
- `src/lib/components/layout/MainContent.svelte`
