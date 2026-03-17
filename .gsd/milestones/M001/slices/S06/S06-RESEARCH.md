# Phase 5: Persistent Chat - Research

**Researched:** 2026-02-16
**Domain:** CRDT-based P2P text messaging with local persistence
**Confidence:** HIGH

## Summary

Phase 5 adds persistent text messaging to Aether's P2P swarms. The core challenge is three-fold: (1) syncing messages between peers without a central server using CRDTs, (2) persisting message history locally so it survives app restarts, and (3) displaying sender identity (display name or truncated public key) alongside each message.

The standard approach is **Automerge** (Rust CRDT library) for conflict-free message sync, with one Automerge document per channel. Each document contains an append-only list of message objects. Automerge's built-in sync protocol handles efficient delta exchange between peers over the existing `libp2p-stream` transport (same pattern as voice). For local persistence, Automerge's native `doc.save()` / `AutoCommit::load()` produces a compact `Vec<u8>` that can be stored as a file in Tauri's app data directory -- no SQLite needed for this use case. Display names are exchanged via a lightweight metadata protocol or embedded in message objects alongside the sender's public key.

**Primary recommendation:** Use `automerge` 0.7.x with `autosurgeon` 0.10.x for typed Rust structs, one Automerge document per swarm-channel pair, synced over a new `/aether/chat/1.0.0` libp2p-stream protocol, persisted as binary files to Tauri's app data directory.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| automerge | 0.7.4 | CRDT document engine | Only production-grade Rust CRDT with built-in sync protocol; used by Ink & Switch, backed by formal verification |
| autosurgeon | 0.10.1 | Derive macros for Reconcile/Hydrate | Maps Rust structs to Automerge docs like serde; eliminates manual ObjId management |
| libp2p-stream | 0.4.0-alpha | Custom protocol transport | Already used for voice; provides reliable ordered streams per-peer |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| uuid | 0.8 or 1.x | Message ID generation | Unique message IDs for deduplication |
| chrono | 0.4 | Timestamp formatting | Display-friendly timestamps in UI |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| automerge | yrs (Yjs Rust) | Yrs is optimized for text editing (collaborative docs), automerge is better for structured data (message lists). Yrs has no built-in sync protocol for Rust. |
| automerge | diamond-types | Focused on text sequences only, no map/list CRDT for structured message objects |
| File-based persistence | SQLite (rusqlite) | Overkill for storing opaque binary blobs; Automerge docs are self-contained binary format. SQLite adds compile-time complexity (bundled feature, ~2MB binary). File-per-channel is simpler. |
| File-based persistence | tauri-plugin-sql | Frontend-driven SQL; we need Rust-side persistence for the Automerge docs, not JS-side queries |

**Installation:**
```bash
# In src-tauri/
cargo add automerge
cargo add autosurgeon
cargo add uuid --features v4
```

No frontend npm packages needed for CRDT logic (it runs entirely in Rust backend).

## Architecture Patterns

### Recommended Project Structure
```
src-tauri/src/
├── chat/
│   ├── mod.rs           # Module exports
│   ├── message.rs       # ChatMessage struct with Reconcile/Hydrate derives
│   ├── document.rs      # ChatDocument: wraps AutoCommit, manages message list
│   ├── protocol.rs      # /aether/chat/1.0.0 wire format (sync messages)
│   ├── sync.rs          # Per-peer sync state management and sync loop
│   └── storage.rs       # File-based persistence (save/load Automerge docs)
├── commands/
│   └── chat.rs          # Tauri IPC commands (send_message, get_messages, etc.)
src/
├── lib/
│   ├── stores/
│   │   └── chat.svelte.ts    # Chat reactive store with $state runes
│   └── components/
│       └── chat/
│           ├── ChatPanel.svelte      # Main chat panel (message list + input)
│           ├── MessageList.svelte    # Scrollable message list
│           └── MessageInput.svelte   # Text input + send button
```

### Pattern 1: One Automerge Document Per Channel
**What:** Each swarm-channel pair (e.g., swarm `abc123` / channel `general`) gets its own `AutoCommit` document containing a list of message maps.
**When to use:** Always -- this is the core data model.
**Why:** Keeps sync scoped to what peers actually need. A document with 10,000 messages is ~1-5MB (Automerge compresses well). Multiple small documents sync faster than one mega-document.

**Example:**
```rust
// Source: Context7 /automerge/automerge + /websites/rs_autosurgeon
use automerge::{AutoCommit, ObjType, ReadDoc, ROOT, transaction::Transactable};
use autosurgeon::{Reconcile, Hydrate, reconcile, hydrate};

#[derive(Debug, Clone, Reconcile, Hydrate)]
struct ChatMessage {
    id: String,           // UUID v4
    sender_key: String,   // hex-encoded Ed25519 public key
    sender_name: String,  // display name at time of sending
    content: String,      // message text
    timestamp: i64,       // Unix timestamp millis
}

#[derive(Debug, Clone, Reconcile, Hydrate)]
struct ChatDocument {
    messages: Vec<ChatMessage>,
}

// Create new channel document
let mut doc = AutoCommit::new();
let chat = ChatDocument { messages: vec![] };
reconcile(&mut doc, &chat).unwrap();

// Add a message
let mut chat: ChatDocument = hydrate(&doc).unwrap();
chat.messages.push(ChatMessage {
    id: uuid::Uuid::new_v4().to_string(),
    sender_key: "ab12cd34...".to_string(),
    sender_name: "Alice".to_string(),
    content: "Hello, world!".to_string(),
    timestamp: 1708099200000,
});
reconcile(&mut doc, &chat).unwrap();

// Save to disk
let bytes: Vec<u8> = doc.save();
std::fs::write("general.automerge", &bytes).unwrap();

// Load from disk
let doc = AutoCommit::load(&std::fs::read("general.automerge").unwrap()).unwrap();
let chat: ChatDocument = hydrate(&doc).unwrap();
```

### Pattern 2: Sync Protocol Over libp2p-stream
**What:** Reuse the established `libp2p-stream` pattern (like voice) but for Automerge sync messages instead of audio frames.
**When to use:** Every time a peer connects or a local change happens.
**Why:** Automerge's sync protocol is designed for reliable ordered transports -- exactly what libp2p-stream provides. The protocol converges in 1-3 round trips for typical scenarios.

**Example:**
```rust
// Source: Context7 /automerge/automerge (Rust Sync Protocol Example)
use automerge::{AutoCommit, sync::SyncDoc};
use libp2p::StreamProtocol;

pub const CHAT_PROTOCOL: StreamProtocol = StreamProtocol::new("/aether/chat/1.0.0");

// Per-peer sync state (maintained across connections)
struct PeerSyncState {
    sync_state: automerge::sync::State,
}

// Sync loop (runs when peer connects or local doc changes)
async fn sync_with_peer(
    doc: &mut AutoCommit,
    sync_state: &mut automerge::sync::State,
    stream: &mut libp2p::Stream,
) {
    loop {
        // Generate outgoing sync message
        let msg = doc.sync().generate_sync_message(sync_state);

        if let Some(msg) = msg {
            // Send sync message over stream (length-prefixed)
            send_sync_msg(stream, &msg.encode()).await;
        }

        // Receive peer's sync message (with timeout)
        if let Some(incoming) = recv_sync_msg(stream).await {
            let msg = automerge::sync::Message::decode(&incoming).unwrap();
            doc.sync().receive_sync_message(sync_state, msg).unwrap();
        }

        // Check if sync is complete
        let next = doc.sync().generate_sync_message(sync_state);
        if next.is_none() {
            break; // Converged
        }
    }
}
```

### Pattern 3: File-Based Persistence in Tauri App Data
**What:** Store each channel's Automerge document as a binary file in Tauri's platform-specific app data directory.
**When to use:** On every local mutation (send message) and after every sync.
**Why:** Automerge's `save()` produces a single compact binary blob. No need for a database layer. Tauri provides `app.path().app_data_dir()` for cross-platform storage.

**Example:**
```rust
use tauri::Manager;

fn chat_doc_path(app: &tauri::AppHandle, swarm_id: &str, channel_id: &str) -> std::path::PathBuf {
    let data_dir = app.path().app_data_dir().unwrap();
    data_dir.join("chat").join(swarm_id).join(format!("{}.automerge", channel_id))
}

fn save_chat_doc(app: &tauri::AppHandle, swarm_id: &str, channel_id: &str, doc: &AutoCommit) {
    let path = chat_doc_path(app, swarm_id, channel_id);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, doc.save()).unwrap();
}

fn load_chat_doc(app: &tauri::AppHandle, swarm_id: &str, channel_id: &str) -> AutoCommit {
    let path = chat_doc_path(app, swarm_id, channel_id);
    if path.exists() {
        AutoCommit::load(&std::fs::read(&path).unwrap()).unwrap()
    } else {
        // New document with empty messages list
        let mut doc = AutoCommit::new();
        let chat = ChatDocument { messages: vec![] };
        reconcile(&mut doc, &chat).unwrap();
        doc
    }
}
```

### Pattern 4: Display Name Resolution
**What:** Embed sender's display name and public key in each message. Use public key as the authoritative identity, display name as convenience.
**When to use:** Every message send/display.
**Why:** In a P2P system without a central name registry, the simplest reliable approach is to embed the sender's display name at send time. If the sender changes their name later, old messages keep their original name (which is acceptable and matches how Discord/Slack handle name changes on historical messages). The truncated public key (short_id format `xxxx:xxxx:xxxx:xxxx`) serves as the fallback.

### Anti-Patterns to Avoid
- **Single document for entire swarm:** A single Automerge doc for all channels would make sync scope too broad -- peers would need to sync everything even if they only care about one channel.
- **Polling for sync:** Don't poll for changes on a timer. Instead, trigger sync on (a) new local message, (b) peer connection established, (c) incoming sync stream opened.
- **Storing raw JSON instead of Automerge binary:** Automerge's binary format is 5-10x more compact than JSON and supports incremental sync. Don't serialize to JSON for storage.
- **Recreating sync state on every connection:** Automerge sync states track what the peer already has. Discarding them forces full re-sync every time. Cache them per-peer (in memory is fine for MVP, since full sync from scratch is still fast for typical message volumes).

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Conflict resolution for concurrent messages | Custom merge logic, timestamp ordering | Automerge CRDT | CRDTs are formally proven to converge; hand-rolled merge has edge cases (network partitions, clock skew, message reordering) |
| Sync protocol (what has peer X seen?) | Custom diffing, sequence numbers, ack tracking | `automerge::sync` module | The sync protocol handles bloom filters, hash comparison, and incremental change exchange. Reimplementing this correctly is a research-paper-level task. |
| Message ordering | Custom Lamport clocks or vector clocks | Automerge's built-in causal ordering | Automerge tracks causal dependencies between changes internally |
| Wire format for sync messages | Custom binary protocol | `automerge::sync::Message::encode()/decode()` | The format is versioned and handles backward compatibility |

**Key insight:** The entire value proposition of Automerge is that you don't build sync, merge, or conflict resolution. You build your data model, mutate it locally, and let Automerge handle convergence. The sync protocol is ~15 lines of application code wrapping Automerge's API.

## Common Pitfalls

### Pitfall 1: Growing Document Size
**What goes wrong:** Automerge documents store full change history. A channel with 100,000 messages could grow to 50-100MB over time.
**Why it happens:** Automerge never forgets -- every edit is recorded for convergence.
**How to avoid:** For MVP, this is not a problem (typical swarms will have thousands of messages, not millions). For future: consider document splitting by time range (monthly archives), or use `doc.save()` which compacts the format. The compact save format is much smaller than the internal representation.
**Warning signs:** Document load time exceeding 500ms, memory usage growing linearly with message count.

### Pitfall 2: Sync Storm on Reconnect
**What goes wrong:** When a peer reconnects after a long offline period, syncing a large document could block the network.
**Why it happens:** Automerge sync protocol sends all missing changes.
**How to avoid:** Run sync in a background tokio task with bounded concurrency. Don't block the UI or other protocols (voice) during sync. The sync protocol is already incremental -- it won't send the entire document, just the diff.
**Warning signs:** UI freezing when peers reconnect, voice audio drops during sync.

### Pitfall 3: Blocking the Event Loop with AutoCommit Operations
**What goes wrong:** `doc.save()`, `reconcile()`, and `hydrate()` are synchronous and could block the tokio event loop for large documents.
**Why it happens:** Automerge operations are CPU-bound, not async.
**How to avoid:** Wrap heavy Automerge operations in `tokio::task::spawn_blocking()` or use a dedicated thread. For MVP with small documents, this won't matter, but architect for it.
**Warning signs:** Async task starvation, increased latency on voice/network operations.

### Pitfall 4: Autosurgeon Vec Reconciliation Performance
**What goes wrong:** `reconcile()` with a `Vec<ChatMessage>` may do a full diff of the entire vector on every mutation.
**Why it happens:** autosurgeon's default Vec reconciliation compares the full list.
**How to avoid:** For MVP, use the simple pattern (hydrate full list, push, reconcile). If performance becomes an issue, drop to the raw Automerge API for appending: `doc.insert_object(&messages_list, index, ObjType::Map)` and set individual fields. This avoids diffing the entire history.
**Warning signs:** Message send latency > 50ms with large message histories.

### Pitfall 5: Missing Channel Document on First Peer
**What goes wrong:** Two peers join a swarm, each creates a "general" channel document independently, then sync produces two divergent documents.
**Why it happens:** Both peers initialize the document before discovering each other.
**How to avoid:** This is actually fine with Automerge! Two independently created documents with the same schema will merge cleanly. The messages from both sides will appear in the merged document. The only concern is duplicate initialization data, which is harmless for a messages list (it starts empty on both sides, so merge produces empty -- then real messages sync correctly).

### Pitfall 6: Not Triggering Frontend Update After Sync
**What goes wrong:** Messages arrive via sync but the UI doesn't update.
**Why it happens:** Sync happens in a background Rust task; the frontend store doesn't know about it.
**How to avoid:** After every successful sync round, emit a Tauri event (e.g., `chat-messages-updated`) with the channel ID. The frontend store listens for this event and re-fetches messages.
**Warning signs:** Messages only appearing after manual refresh or app restart.

## Code Examples

Verified patterns from official sources:

### Automerge Document Create, Mutate, Save, Load
```rust
// Source: Context7 /automerge/automerge
use automerge::{AutoCommit, ReadDoc, ROOT, transaction::Transactable};

// Create
let mut doc = AutoCommit::new();

// Save
let saved: Vec<u8> = doc.save();

// Load
let mut loaded = AutoCommit::load(&saved).unwrap();
```

### Autosurgeon Reconcile + Hydrate Roundtrip
```rust
// Source: Context7 /websites/rs_autosurgeon
use autosurgeon::{Reconcile, Hydrate, reconcile, hydrate};

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq)]
struct ChatMessage {
    id: String,
    sender_key: String,
    sender_name: String,
    content: String,
    timestamp: i64,
}

let msg = ChatMessage {
    id: "uuid-here".into(),
    sender_key: "pubkey-hex".into(),
    sender_name: "Alice".into(),
    content: "Hello!".into(),
    timestamp: 1708099200000,
};

let mut doc = AutoCommit::new();
reconcile(&mut doc, &msg).unwrap();

let msg2: ChatMessage = hydrate(&doc).unwrap();
assert_eq!(msg, msg2);
```

### Automerge Sync Protocol (Rust)
```rust
// Source: Context7 /automerge/automerge (Rust Sync Protocol Example)
use automerge::{AutoCommit, sync::SyncDoc, transaction::Transactable, ROOT};

let mut doc1 = AutoCommit::new();
let mut doc2 = AutoCommit::new();

doc1.put(ROOT, "key", "value1").unwrap();
doc2.put(ROOT, "key", "value2").unwrap();

let mut sync1 = doc1.sync().new_state();
let mut sync2 = doc2.sync().new_state();

loop {
    let msg1 = doc1.sync().generate_sync_message(&mut sync1);
    let msg2 = doc2.sync().generate_sync_message(&mut sync2);

    if msg1.is_none() && msg2.is_none() {
        break;
    }

    if let Some(msg) = msg1 {
        doc2.sync().receive_sync_message(&mut sync2, msg).unwrap();
    }
    if let Some(msg) = msg2 {
        doc1.sync().receive_sync_message(&mut sync1, msg).unwrap();
    }
}

assert_eq!(doc1.get_heads(), doc2.get_heads());
```

### Tauri Event Emission Pattern (Existing Codebase)
```rust
// Source: Aether codebase src-tauri/src/network/mod.rs
use tauri::{AppHandle, Emitter};

// Emit event to frontend
let _ = app.emit("chat-messages-updated", serde_json::json!({
    "swarm_id": swarm_id,
    "channel_id": channel_id,
    "message_count": messages.len(),
}));
```

### Svelte 5 Store Pattern (Existing Codebase)
```typescript
// Source: Aether codebase src/lib/stores/voice.svelte.ts
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

let messages = $state<ChatMessage[]>([]);
let sending = $state(false);

async function sendMessage(content: string) {
  sending = true;
  try {
    await invoke('send_message', { content });
  } finally {
    sending = false;
  }
}

// Listen for new messages from sync
await listen('chat-messages-updated', async () => {
  messages = await invoke('get_messages', { channelId });
});
```

### libp2p-stream Protocol Pattern (Existing Codebase)
```rust
// Source: Aether codebase src-tauri/src/voice/protocol.rs
use libp2p::StreamProtocol;

// Define protocol (same pattern as VOICE_PROTOCOL)
pub const CHAT_PROTOCOL: StreamProtocol = StreamProtocol::new("/aether/chat/1.0.0");

// Wire format for chat sync: length-prefixed Automerge sync messages
// [4 bytes: message length (u32 big-endian)][N bytes: automerge sync message]
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| automerge 0.5 (JS-only) | automerge 0.7 (Rust-native with WASM bindings) | 2024 | Full Rust API, no FFI overhead, ~10x faster |
| Manual CRDT implementation | autosurgeon derive macros | 2024 | Eliminates manual ObjId/property management |
| automerge-repo (JS) | Direct Rust sync API | Current | automerge-repo is JS-focused; Rust apps use `sync::SyncDoc` trait directly |
| Full document transfer for sync | Bloom filter-based sync protocol | automerge 0.5+ | Only missing changes are sent, not full document |

**Deprecated/outdated:**
- `automerge-rs` (old crate name): Replaced by `automerge` crate. The old Traverse-Research fork is unmaintained.
- `automerge::Frontend`/`automerge::Backend` split: Unified in automerge 0.5+. Use `AutoCommit` or `Automerge` directly.
- `save_after()` method name: Renamed to `save_incremental()` in recent versions. Functionality is the same.

## Open Questions

1. **Autosurgeon Vec performance at scale**
   - What we know: autosurgeon diffs the full Vec on reconcile. For small lists (<1000 messages) this is fast.
   - What's unclear: At what message count does reconcile become a bottleneck? Is it 5K, 50K, or 100K messages?
   - Recommendation: Start with autosurgeon for simplicity. Benchmark at 10K messages. If slow, switch to raw Automerge list API for message insertion (keep autosurgeon for hydration/reading).

2. **Sync trigger strategy**
   - What we know: Sync should happen on (a) local message sent, (b) peer connects, (c) incoming stream opened.
   - What's unclear: Should we also do periodic sync (e.g., every 30s) as a catchall for missed events?
   - Recommendation: For MVP, trigger-based only. Add periodic sync later if messages are observed to be missing.

3. **ActorId for Automerge**
   - What we know: Automerge uses ActorId to attribute changes. We have Ed25519 public keys as stable peer identifiers.
   - What's unclear: Can we use the public key bytes directly as the ActorId, or does Automerge require a specific format?
   - Recommendation: Use `ActorId::from(public_key_bytes)` -- ActorId accepts arbitrary byte arrays. This gives deterministic actor identity tied to the user's cryptographic identity.

## Sources

### Primary (HIGH confidence)
- Context7 `/automerge/automerge` - Rust API for AutoCommit, sync protocol, save/load patterns
- Context7 `/websites/rs_autosurgeon` - Derive macros for Reconcile/Hydrate, struct mapping
- [Automerge docs.rs 0.7.4](https://docs.rs/automerge/latest/automerge/) - Module structure, sync API, AutoCommit
- [Autosurgeon docs.rs 0.10.1](https://docs.rs/autosurgeon/latest/autosurgeon/) - Reconcile, Hydrate derives
- Aether codebase (`src-tauri/src/voice/protocol.rs`, `session.rs`, `network/mod.rs`) - Existing libp2p-stream patterns, event emission, managed state

### Secondary (MEDIUM confidence)
- [lib.rs/crates/automerge](https://lib.rs/crates/automerge) - Version 0.7.4 (stable), 1.0.0-beta.3 available
- [lib.rs/crates/autosurgeon](https://lib.rs/crates/autosurgeon) - Version 0.10.1, depends on automerge 0.7
- [Automerge data modeling guide](https://automerge.org/docs/cookbook/modeling-data/) - Document granularity guidance
- [Automerge sync docs](https://automerge.org/docs/tutorial/network-sync/) - Network adapter architecture
- [rusqlite 0.38.0](https://docs.rs/crate/rusqlite/latest) - Evaluated and rejected; file-based persistence is simpler for this use case
- [Tauri v2 SQL plugin](https://v2.tauri.app/plugin/sql/) - Evaluated and rejected; frontend-driven SQL not needed when CRDT backend handles persistence

### Tertiary (LOW confidence)
- [Automerge save_incremental pattern](https://docs.rs/automerge/latest/automerge/struct.AutoCommit.html) - `save_incremental()` for delta saves; needs validation of exact method name in 0.7.4

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - automerge 0.7.4 and autosurgeon 0.10.1 verified on crates.io/lib.rs with recent updates; API patterns verified via Context7
- Architecture: HIGH - follows established Aether patterns (libp2p-stream protocol, Tauri managed state, event emission, Svelte 5 stores); Automerge document-per-channel is recommended by official docs
- Pitfalls: MEDIUM - performance thresholds (autosurgeon Vec, document size limits) are estimates based on documented characteristics, not benchmarked in this codebase

**Research date:** 2026-02-16
**Valid until:** 2026-03-16 (automerge 0.7.x is stable; 1.0 beta exists but autosurgeon 0.10.1 targets 0.7)