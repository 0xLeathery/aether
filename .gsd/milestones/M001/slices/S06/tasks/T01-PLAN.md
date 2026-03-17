# T01: 05-persistent-chat 01

**Slice:** S06 — **Milestone:** M001

## Description

Build the complete Rust chat engine: CRDT document model with Automerge, file-based persistence, and libp2p-stream sync protocol.

Purpose: This is the foundational data layer for persistent chat. All message storage, sync, and conflict resolution happens here. The Automerge CRDT ensures eventual consistency without central coordination.

Output: A `chat` module in `src-tauri/src/chat/` with ChatService that manages per-channel Automerge documents, persists them to disk, and syncs between peers via libp2p-stream.

## Must-Haves

- [ ] "ChatMessage struct exists with id, sender_key, sender_name, content, timestamp fields and Reconcile/Hydrate derives"
- [ ] "ChatDocument wraps AutoCommit and can add messages, hydrate message list, and save/load from bytes"
- [ ] "Storage module can save and load Automerge documents as binary files in Tauri app data directory"
- [ ] "Sync protocol uses /aether/chat/1.0.0 StreamProtocol with length-prefixed Automerge sync messages"
- [ ] "ChatService manages per-channel documents, triggers sync on local send and peer connect, persists after mutations"

## Files

- `src-tauri/Cargo.toml`
- `src-tauri/src/error.rs`
- `src-tauri/src/chat/mod.rs`
- `src-tauri/src/chat/message.rs`
- `src-tauri/src/chat/document.rs`
- `src-tauri/src/chat/storage.rs`
- `src-tauri/src/chat/protocol.rs`
- `src-tauri/src/chat/sync.rs`
- `src-tauri/src/lib.rs`
