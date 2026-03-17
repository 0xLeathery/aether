# Phase 6: Foundation - Research

**Researched:** 2026-02-16
**Domain:** Voice mute controls, petname/contacts system, swarm management (rename/leave/regenerate invite)
**Confidence:** HIGH

## Summary

Phase 6 adds essential user-facing controls to the v1.0 Walking Skeleton: microphone mute/unmute during voice sessions, local petname assignment for peer identities, a contacts list view, and swarm management operations (rename, leave, regenerate invite link). All seven requirements (VOIC-05, VOIC-06, IDEN-04, IDEN-05, SWRM-01, SWRM-02, SWRM-03) build on existing infrastructure with no new crate dependencies.

The technical risk is LOW. Voice mute adds a single `AtomicBool` to the existing `VoiceSession`. Petnames and contacts use the established `tauri-plugin-store` pattern (same as `swarms.json`). Swarm rename is a single-field update. Swarm leave requires an ordered multi-resource cleanup (store entry, Automerge files, in-memory caches, network teardown). Invite regeneration is trivially re-encoding the existing PSK. The only meaningful complexity is in SWRM-02 (leave swarm) where cleanup must span four data domains without leaving orphaned state.

**Primary recommendation:** Build mic mute first (zero dependencies, immediate UX win), then contacts/petnames (new storage pattern), then swarm management (extends existing swarm storage). All features are independent and could be parallelized.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `std::sync::atomic::AtomicBool` | std | Voice mute flag | Already used for `is_active` in `VoiceSession`. Zero-cost, lock-free |
| `tauri-plugin-store` | 2 | Contacts persistence (`contacts.json`) | Already in `Cargo.toml`, established pattern from `swarms.json` |
| `std::fs` | std | Automerge file cleanup on swarm leave | Already used in `chat/storage.rs` for doc I/O |
| `tauri::Emitter` | 2 | New events (`voice-mute-changed`, `swarm-deleted`) | Already used for all existing events |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde` / `serde_json` | 1 | Contact struct serialization | Already in `Cargo.toml` |
| `uuid` | 1 (v4) | Not needed for Phase 6 | N/A -- no new IDs generated |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `tauri-plugin-store` for contacts | SQLite / `sled` / `redb` | Over-engineering for a simple key-value mapping. Store plugin is already integrated and sufficient for <1000 contacts |
| `AtomicBool` for mute | `Mutex<bool>` | Mutex adds unnecessary lock contention in the audio hot loop. AtomicBool is the correct choice |
| `std::fs::remove_dir_all` for cleanup | Custom recursive delete | `remove_dir_all` is idempotent-safe when wrapped in a path existence check. No custom code needed |

**Installation:**
```bash
# No new dependencies required. All libraries already in Cargo.toml or std.
```

## Architecture Patterns

### Recommended Project Structure

New files for Phase 6:
```
src-tauri/src/
├── contacts/              # NEW MODULE
│   ├── mod.rs             # ContactsService, Contact struct
│   └── storage.rs         # tauri-plugin-store operations on contacts.json
├── commands/
│   └── contacts.rs        # NEW: set_petname, remove_petname, get_contacts
├── swarm/
│   └── storage.rs         # MODIFIED: add rename_swarm, delete_swarm, get_invite_uri
├── voice/
│   └── session.rs         # MODIFIED: add is_muted AtomicBool, set_muted, is_muted
├── commands/
│   ├── voice.rs           # MODIFIED: add toggle_mute command
│   └── swarm.rs           # MODIFIED: add rename_swarm, leave_swarm, regenerate_invite commands
├── error.rs               # MODIFIED: add ContactError enum
└── lib.rs                 # MODIFIED: register new module + commands

src/lib/
├── stores/
│   └── contacts.svelte.ts # NEW: contact/petname cache + resolveName helper
├── tauri.ts               # MODIFIED: new command wrappers + event listeners + Contact type
├── components/
│   ├── peers/
│   │   └── PeerList.svelte       # MODIFIED: show petnames, add "Set Petname" action
│   ├── contacts/
│   │   ├── ContactEditor.svelte  # NEW: inline petname editor popup
│   │   └── ContactsList.svelte   # NEW: full contacts list view
│   ├── voice/
│   │   └── VoicePanel.svelte     # MODIFIED: mute toggle button replacing "MIC LIVE"
│   ├── swarm/
│   │   └── SwarmSettings.svelte  # NEW: rename, leave, regenerate invite dialog
│   ├── chat/
│   │   └── MessageList.svelte    # MODIFIED: use petnames in getSenderDisplay()
│   └── layout/
│       └── Sidebar.svelte        # MODIFIED: add swarm settings gear icon
```

### Pattern 1: AtomicBool Mute Flag (follows existing `is_active` pattern)
**What:** Add `is_muted: Arc<AtomicBool>` to `VoiceSession`, check in encode-and-send task.
**When to use:** Any time a boolean flag needs to be shared between the main thread and a spawned tokio task without locking.
**Example:**
```rust
// Source: Existing pattern in src-tauri/src/voice/session.rs line 26
// VoiceSession already uses Arc<AtomicBool> for is_active

pub struct VoiceSession {
    is_active: Arc<AtomicBool>,
    is_muted: Arc<AtomicBool>,  // NEW -- follows same pattern
    // ... rest unchanged
}

impl VoiceSession {
    pub fn new() -> Self {
        Self {
            is_active: Arc::new(AtomicBool::new(false)),
            is_muted: Arc::new(AtomicBool::new(false)),  // Default unmuted
            // ... rest unchanged
        }
    }

    pub fn set_muted(&self, muted: bool) {
        self.is_muted.store(muted, Ordering::Relaxed);
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted.load(Ordering::Relaxed)
    }
}
```

### Pattern 2: Contacts Store (follows existing swarm storage pattern)
**What:** New `contacts.json` store file via `tauri-plugin-store`, following the exact same `StoreExt::store()` pattern used in `swarm/storage.rs`.
**When to use:** Any local-only key-value data that is NOT synced via Automerge.
**Example:**
```rust
// Source: Pattern from src-tauri/src/swarm/storage.rs lines 25-37
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Contact {
    pub public_key_hex: String,
    pub petname: Option<String>,
    pub notes: Option<String>,
    pub added_at: i64,
}

pub fn set_petname(app: &AppHandle, public_key: &str, petname: &str) -> Result<(), ContactError> {
    let store = app.store("contacts.json")
        .map_err(|e| ContactError::StorageError(format!("Failed to access store: {}", e)))?;

    // Load existing contact or create new
    let contact = match store.get(public_key) {
        Some(val) => {
            let mut c: Contact = serde_json::from_value(val.clone())
                .map_err(|e| ContactError::StorageError(e.to_string()))?;
            c.petname = Some(petname.to_string());
            c
        }
        None => Contact {
            public_key_hex: public_key.to_string(),
            petname: Some(petname.to_string()),
            notes: None,
            added_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap().as_secs() as i64,
        },
    };

    store.set(public_key.to_string(), serde_json::to_value(&contact).unwrap());
    store.save()
        .map_err(|e| ContactError::StorageError(format!("Failed to save store: {}", e)))?;

    Ok(())
}
```

### Pattern 3: Tauri Command (follows existing command pattern exactly)
**What:** New Tauri IPC command that takes `AppHandle` and optional `State` parameters.
**When to use:** Every new frontend-callable operation.
**Example:**
```rust
// Source: Pattern from src-tauri/src/commands/swarm.rs lines 9-11
#[tauri::command]
pub fn set_petname(app: AppHandle, public_key: String, petname: String) -> Result<(), String> {
    crate::contacts::storage::set_petname(&app, &public_key, &petname)
        .map_err(|e| e.to_string())
}
```

### Pattern 4: Svelte 5 Store (follows existing store pattern)
**What:** `.svelte.ts` file with `$state` runes, exported object with getters and methods.
**When to use:** Every new domain of frontend state.
**Example:**
```typescript
// Source: Pattern from src/lib/stores/voice.svelte.ts
// contacts.svelte.ts follows the exact same structure

import { setPetname, removePetname, getContacts, type Contact, type UnlistenFn } from '../tauri';

let contacts = $state<Contact[]>([]);
let loading = $state(false);
let error = $state<string | null>(null);
let initialized = $state(false);

function resolveName(publicKey: string, fallbackName: string): string {
    const contact = contacts.find(c => c.public_key_hex === publicKey);
    if (contact?.petname) return contact.petname;
    if (fallbackName && fallbackName.length > 0) return fallbackName;
    return publicKey.substring(0, 8) + '...';
}

async function initialize() { /* load contacts from backend */ }

export const contactsStore = {
    get contacts() { return contacts; },
    get loading() { return loading; },
    get error() { return error; },
    resolveName,
    initialize,
    // ... setPetname, removePetname methods
};
```

### Pattern 5: Swarm Deletion Cleanup Sequence
**What:** Ordered multi-resource cleanup when leaving a swarm.
**When to use:** SWRM-02 (leave swarm) implementation.
**Example:**
```rust
// Cleanup must happen in this order to avoid orphaned state:
// 1. Leave voice session if in the swarm being deleted
// 2. Stop network service if connected to this swarm
// 3. Remove all in-memory ChatDocument entries for "swarm_id/*"
// 4. Remove all PeerSyncStates entries for the swarm
// 5. Delete Automerge files: remove_dir_all("{app_data_dir}/chat/{swarm_id}/")
// 6. Remove SwarmMetadata entry from swarms.json store (store.delete(swarm_id) + store.save())
// 7. Emit "swarm-deleted" event to frontend
```

### Anti-Patterns to Avoid
- **Stopping cpal streams for mute:** DO NOT drop/recreate `CpalStream` on mute toggle. Keep capture running, just skip encoding/sending. Restarting cpal causes 50-200ms latency and audible pops on macOS.
- **Syncing petnames via Automerge:** Petnames are LOCAL-ONLY data. Putting them in a CRDT would leak your private contact book to all peers. Use `tauri-plugin-store`.
- **Sending silence frames when muted:** DO NOT encode and send zero-filled audio when muted. Skip the send entirely to save bandwidth. Drain `capture_rx` to prevent buffer buildup, but do not encode or transmit.
- **Hard-deleting without existence checks:** All cleanup steps must check existence before deleting. `remove_dir_all` will error if the directory does not exist. Wrap in `if path.exists()` checks for idempotency.
- **Holding std::sync::Mutex across await:** The `network.lock()` in swarm commands MUST be dropped before any `.await` point. The existing pattern in `commands/chat.rs` (line 67: `drop(service)`) demonstrates this correctly.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Mute state sharing across threads | Custom channel/message passing for mute | `Arc<AtomicBool>` | Lock-free, zero overhead, already proven in `is_active` pattern |
| Contact persistence | Custom file I/O, JSON parsing | `tauri-plugin-store` with `contacts.json` | Already integrated, handles atomicity, supports JS-side access |
| Invite link encoding | Custom encoding/serialization | Existing `swarm::uri::encode_secret_code()` | Already implemented and tested in v1.0 |
| Directory deletion | Custom recursive file walk | `std::fs::remove_dir_all` | Standard library, handles nested directories |
| Event emission | Custom IPC mechanism | `tauri::Emitter::emit()` | Already used for all existing events |

**Key insight:** Phase 6 requires ZERO new crate dependencies. Every feature builds on patterns and libraries already in the v1.0 codebase.

## Common Pitfalls

### Pitfall 1: Mute Toggle Sends Stale Audio Frame
**What goes wrong:** User toggles mute, but the next frame was already captured and in the `capture_rx` channel. The first frame after mute might still contain audio.
**Why it happens:** The `capture_rx` channel has capacity 10 (200ms of buffered frames). When mute is toggled, up to 10 frames are already queued.
**How to avoid:** Check `is_muted` AFTER receiving from `capture_rx` but BEFORE encoding. Drain frames silently when muted. The check point must be in the encode-and-send loop, not in the capture callback.
**Warning signs:** Brief audio "tail" after pressing mute.
```rust
// CORRECT: Check mute after receive, before encode
while is_active.load(Ordering::Relaxed) {
    let pcm = match capture_rx.recv() {
        Ok(p) => p,
        Err(_) => break,
    };

    // Mute check -- skip encode and send
    if is_muted.load(Ordering::Relaxed) {
        continue;  // Frame is discarded
    }

    // Proceed with encoding and sending...
}
```

### Pitfall 2: Swarm Leave Without Stopping Voice Session
**What goes wrong:** User leaves a swarm while in a voice session connected to that swarm. Voice tasks continue trying to send/receive on stale peer connections, causing errors or zombie tasks.
**Why it happens:** The leave-swarm command only cleans up swarm metadata and chat documents. It does not check if a voice session is active for that swarm.
**How to avoid:** The `leave_swarm` command must check `voice_session.is_in_session()` and call `voice_session.leave()` if the active swarm is being left. Do this FIRST, before any other cleanup.
**Warning signs:** Error logs about "Failed to send to peer" or "Stream closed" after leaving a swarm.

### Pitfall 3: Petname Resolution Creates Stale Cache
**What goes wrong:** User sets a petname for a peer, but the `MessageList.svelte` still shows the old display name because the contacts store was not refreshed.
**Why it happens:** The contacts store loads contacts once on initialization. If a petname is set while messages are displayed, the `resolveName()` function still uses the old data.
**How to avoid:** When `setPetname()` is called, update the local `contacts` array reactively (Svelte 5 `$state` reactivity will automatically propagate to all components using `resolveName()`). Do NOT rely on re-fetching from the backend -- mutate the local state directly after the backend confirms.
**Warning signs:** Petname appears in the contacts list but not in message sender names until page refresh.

### Pitfall 4: Swarm Leave Races with Active Chat Sync
**What goes wrong:** A chat sync is in progress (holding `chat_service` lock) when the leave-swarm command tries to clear documents from the `ChatService.documents` HashMap. Deadlock or inconsistent state results.
**Why it happens:** `leave_swarm` needs to modify `ChatService.documents` (remove entries), but an ongoing sync may be holding the `Arc<tokio::sync::Mutex<ChatService>>` lock.
**How to avoid:** Stop the network service FIRST (which terminates sync tasks since streams close), then acquire the chat service lock to clean up in-memory state. The sequence must be: (1) stop network, (2) lock chat service, (3) remove documents, (4) drop lock, (5) delete files on disk.
**Warning signs:** Occasional hangs when leaving a swarm while peers are connected.

### Pitfall 5: Invite Link Regeneration Confusion
**What goes wrong:** User expects "regenerate invite link" to create a new secret that invalidates the old link. But in Aether's architecture, the swarm IS the PSK. Regenerating the link just re-encodes the same PSK bytes.
**Why it happens:** The `aether://` URI is simply `hex::encode(psk_bytes)`. The PSK is the immutable identity of the swarm. You cannot change the PSK without creating an entirely new swarm.
**How to avoid:** Label the UI action "Copy Invite Link" rather than "Regenerate Invite Link." The URI is deterministic from the PSK -- calling `swarm::uri::encode_secret_code()` on the stored PSK will always produce the same link. The requirement SWRM-03 should be interpreted as "get/copy the invite link" not "create a new one."
**Warning signs:** User reports "regenerated link but old link still works."

## Code Examples

Verified patterns from the existing codebase:

### Voice Mute Toggle Command
```rust
// New command in src-tauri/src/commands/voice.rs
// Follows exact pattern of join_voice/leave_voice

#[tauri::command]
pub async fn toggle_mute(
    app: AppHandle,
    voice_session: State<'_, tokio::sync::Mutex<VoiceSession>>,
) -> Result<bool, String> {
    let session = voice_session.lock().await;

    if !session.is_in_session() {
        return Err("Not in a voice session".to_string());
    }

    let new_muted = !session.is_muted();
    session.set_muted(new_muted);

    // Emit mute state change event
    let _ = app.emit("voice-mute-changed", new_muted);

    Ok(new_muted)
}
```

### Rename Swarm Command
```rust
// New command in src-tauri/src/commands/swarm.rs
// Follows existing create_swarm/join_swarm pattern

#[tauri::command]
pub fn rename_swarm(app: AppHandle, swarm_id: String, new_name: String) -> Result<(), SwarmError> {
    let mut metadata = swarm::storage::get_swarm(&app, &swarm_id)?;
    metadata.name = new_name;
    swarm::storage::save_swarm(&app, &metadata)?;
    Ok(())
}
```

### Leave Swarm Command (Ordered Cleanup)
```rust
// New command in src-tauri/src/commands/swarm.rs
// This is the most complex command in Phase 6

#[tauri::command]
pub async fn leave_swarm(
    app: AppHandle,
    network: State<'_, std::sync::Mutex<NetworkService>>,
    voice_session: State<'_, tokio::sync::Mutex<VoiceSession>>,
    chat_service: State<'_, Arc<tokio::sync::Mutex<ChatService>>>,
    swarm_id: String,
) -> Result<(), String> {
    // 1. Leave voice if active
    {
        let mut session = voice_session.lock().await;
        if session.is_in_session() {
            session.leave(app.clone()).await;
        }
    }

    // 2. Stop network (closes all streams, terminates sync tasks)
    {
        let mut network_service = network.lock().unwrap();
        network_service.stop();
    }

    // 3. Clear in-memory chat documents for this swarm
    {
        let mut service = chat_service.lock().await;
        service.remove_swarm_documents(&swarm_id);
        // NOTE: remove_swarm_documents() is a new method to add to ChatService
    }

    // 4. Delete Automerge files from disk
    let data_dir = app.path().app_data_dir()
        .map_err(|e| format!("Path error: {}", e))?;
    let swarm_chat_dir = data_dir.join("chat").join(&swarm_id);
    if swarm_chat_dir.exists() {
        std::fs::remove_dir_all(&swarm_chat_dir)
            .map_err(|e| format!("Failed to delete chat data: {}", e))?;
    }

    // 5. Remove swarm metadata from store
    {
        let store = app.store("swarms.json")
            .map_err(|e| format!("Store error: {}", e))?;
        store.delete(&swarm_id);
        store.save().map_err(|e| format!("Save error: {}", e))?;
    }

    // 6. Emit deletion event
    let _ = app.emit("swarm-deleted", &swarm_id);

    Ok(())
}
```

### Get Invite URI Command
```rust
// Re-encode the stored PSK as an aether:// URI
// The PSK is already stored in SwarmMetadata.psk_hex as "aether://..."

#[tauri::command]
pub fn get_invite_uri(app: AppHandle, swarm_id: String) -> Result<String, SwarmError> {
    let metadata = swarm::storage::get_swarm(&app, &swarm_id)?;
    // psk_hex already contains the full aether:// URI
    Ok(metadata.psk_hex.clone())
}
```

### Frontend Mute Toggle Button (VoicePanel.svelte modification)
```svelte
<!-- Replaces the static "MIC LIVE" indicator -->
{#if voiceStore.active}
  <div class="voice-status">
    <button
      class="mute-toggle"
      class:muted={voiceStore.muted}
      onclick={handleToggleMute}
    >
      <div class="mic-indicator" class:muted={voiceStore.muted}>
        <div class="mic-pulse" class:muted={voiceStore.muted}></div>
        <span class="mic-label">
          {voiceStore.muted ? 'MIC MUTED' : 'MIC LIVE'}
        </span>
      </div>
    </button>
    <!-- ... participant list unchanged ... -->
  </div>
{/if}
```

### Frontend Petname Resolution in MessageList.svelte
```svelte
<script lang="ts">
  import type { ChatMessage } from '../../tauri';
  import { contactsStore } from '../../stores/contacts.svelte';

  // ... existing props ...

  function getSenderDisplay(msg: ChatMessage): string {
    // Use contacts store to resolve petname -> sender_name -> truncated key
    return contactsStore.resolveName(msg.sender_key, msg.sender_name);
  }
</script>
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `new App()` (Svelte 4) | `mount()` (Svelte 5) | Svelte 5 GA | Already using mount() in main.ts |
| `createEventDispatcher()` | Callback props `onFoo` | Svelte 5 GA | Already using callback props throughout |
| `on:click` directive | `onclick` attribute | Svelte 5 GA | Already using attribute syntax |
| `$:` reactive statements | `$state` / `$effect` runes | Svelte 5 GA | Already using runes in all stores |

**Deprecated/outdated:**
- Nothing relevant to Phase 6. All current patterns are Svelte 5 and Tauri v2 compliant.

## Requirement-Specific Analysis

### VOIC-05 + VOIC-06: Mic Mute with Visual Indicator
**Complexity:** Low
**Approach:** Single `AtomicBool` field on `VoiceSession`, one new Tauri command (`toggle_mute`), one new event (`voice-mute-changed`), VoicePanel.svelte modification.
**Key insight:** The encode-and-send task in `session.rs` (line 110-171) already loops with `while is_active.load(Ordering::Relaxed)`. Adding `if is_muted.load(Ordering::Relaxed) { continue; }` after `capture_rx.recv()` is the only backend change needed in the hot path.
**Files touched:** `voice/session.rs`, `commands/voice.rs`, `stores/voice.svelte.ts`, `tauri.ts`, `VoicePanel.svelte`

### IDEN-04: Petnames Override Self-Asserted Names
**Complexity:** Low-Medium
**Approach:** New `contacts/` module with `Contact` struct and storage functions. New `contacts.svelte.ts` store with `resolveName(publicKey, fallbackName)` helper. All UI components that display peer names call `resolveName()` instead of using `sender_name` directly.
**Key insight:** The resolution chain is: petname > sender_name > truncated public key. This matches the Spritely Institute petname model. Petnames are NEVER synced -- they exist only in `contacts.json` on the local device.
**UI touchpoints that display peer names:**
1. `MessageList.svelte` -- `getSenderDisplay()` function (line 40-45)
2. `PeerList.svelte` -- `truncatePeerId()` function (line 19-21)
3. `VoicePanel.svelte` -- participant display (line 61-62)

### IDEN-05: Contacts List
**Complexity:** Low
**Approach:** New `ContactsList.svelte` component that calls `getContacts()` and displays all peers with petnames. Shows public key (truncated), petname, and last-seen metadata. Accessible from the sidebar.
**Data model:** `Contact { public_key_hex, petname, notes, added_at }` stored in `contacts.json`. The contacts list is a simple read from the store -- no pagination needed for Phase 6 (P2P communities are small, <100 peers).

### SWRM-01: Rename Swarm Locally
**Complexity:** Trivial
**Approach:** Update `SwarmMetadata.name` in `swarms.json` store. Single `rename_swarm(swarm_id, new_name)` Tauri command. Re-fetch swarm list in the frontend after rename.
**Key insight:** Swarm names are already local-only (`SwarmMetadata` is stored in `tauri-plugin-store`, not synced via Automerge). Renaming has zero network implications.

### SWRM-02: Leave Swarm with Full Data Cleanup
**Complexity:** Medium (highest in Phase 6)
**Approach:** Ordered cleanup sequence across four data domains:
1. Voice session teardown (if active)
2. Network service stop (closes all streams)
3. ChatService in-memory document eviction (new `remove_swarm_documents` method)
4. Disk cleanup (`remove_dir_all` on `chat/{swarm_id}/`)
5. Store entry deletion (`store.delete(swarm_id)` + `store.save()`)
6. Frontend state update (remove from `swarmStore.swarms`, clear `activeSwarm` if needed)

**New ChatService method needed:**
```rust
/// Remove all in-memory documents for a swarm
pub fn remove_swarm_documents(&mut self, swarm_id: &str) {
    let prefix = format!("{}/", swarm_id);
    self.documents.retain(|key, _| !key.starts_with(&prefix));
    self.sync_states.remove_swarm(swarm_id);
}
```

**New PeerSyncStates method needed:**
```rust
/// Remove all sync states for a swarm
pub fn remove_swarm(&mut self, swarm_id: &str) {
    let prefix = format!("{}/", swarm_id);
    for (_, channels) in self.states.iter_mut() {
        channels.retain(|key, _| !key.starts_with(&prefix));
    }
}
```

### SWRM-03: Regenerate Invite Link
**Complexity:** Trivial
**Approach:** The `aether://` URI is deterministically derived from the PSK. `SwarmMetadata.psk_hex` already stores the full `aether://...` URI. "Regenerating" the invite link is simply returning this stored value. A `get_invite_uri(swarm_id)` command returns it, and the frontend copies it to the clipboard using the existing `@tauri-apps/plugin-clipboard-manager`.
**Key insight:** The PSK cannot be rotated without creating a new swarm (different PSK = different swarm identity in libp2p `pnet`). "Regenerate" in this context means "copy the existing invite link," not "create a new secret." The UI should clarify this.

## Open Questions

1. **Should "leave swarm" also delete contacts associated with that swarm?**
   - What we know: Contacts are global (keyed by public key, not swarm-scoped). A peer may be in multiple swarms.
   - What's unclear: Should we add a `swarm_ids` field to Contact to track which swarms a peer was encountered in?
   - Recommendation: Keep contacts global and swarm-independent for Phase 6. A peer is a peer regardless of which swarm they were met in.

2. **Should mute state persist across voice session rejoin?**
   - What we know: Currently `is_muted` resets to `false` on `VoiceSession::new()`. If a user leaves and rejoins, they start unmuted.
   - What's unclear: Is this the desired behavior?
   - Recommendation: Default unmuted on join (standard convention). The `is_muted` flag resets when the session is left. This is the safest default -- users should explicitly mute themselves.

3. **Where should the "Contacts" UI live?**
   - What we know: The sidebar currently has: SwarmSelector, ChannelList, PeerList, VoicePanel. Adding a full contacts view needs a location.
   - What's unclear: Should it be a separate panel, a tab in the sidebar, or a modal dialog?
   - Recommendation: Add a "Contacts" button to the sidebar that opens a slide-over panel or modal. Keep it out of the main navigation flow -- it is a management feature, not a primary interaction surface.

## Sources

### Primary (HIGH confidence)
- Existing codebase analysis -- all code patterns verified against actual source files:
  - `src-tauri/src/voice/session.rs` -- `AtomicBool` pattern for `is_active`, encode-and-send loop structure
  - `src-tauri/src/swarm/storage.rs` -- `tauri-plugin-store` usage pattern (`StoreExt::store()`, `set()`, `save()`)
  - `src-tauri/src/commands/voice.rs` -- Tauri command pattern with `State<'_, tokio::sync::Mutex<...>>`
  - `src-tauri/src/commands/swarm.rs` -- Swarm command pattern, lock acquisition
  - `src-tauri/src/chat/mod.rs` -- ChatService document cache, sync listener
  - `src-tauri/src/swarm/uri.rs` -- `encode_secret_code()` / `decode_secret_code()` for invite URIs
  - `src/lib/stores/voice.svelte.ts` -- Svelte 5 store pattern with `$state`, event listeners, cleanup
  - `src/lib/tauri.ts` -- TypeScript IPC wrapper pattern
  - `src/lib/components/voice/VoicePanel.svelte` -- Current "MIC LIVE" indicator (to be replaced)
  - `src/lib/components/chat/MessageList.svelte` -- `getSenderDisplay()` function (to use petnames)
  - `src/lib/components/peers/PeerList.svelte` -- Peer display (to add petname resolution)
- [tauri-plugin-store Rust API](https://docs.rs/tauri-plugin-store/latest/tauri_plugin_store/struct.Store.html) -- `delete()`, `set()`, `get()`, `has()`, `save()`, `entries()`, `keys()` methods confirmed (HIGH confidence)
- [Spritely Institute Petnames Paper](https://files.spritely.institute/papers/petnames.html) -- Petname system design principles (HIGH confidence)

### Secondary (MEDIUM confidence)
- `.planning/research/ARCHITECTURE.md` -- v1.1 architecture integration plan, Section 2 (Petnames) and Section 5 (Mic Mute)
- `.planning/research/FEATURES.md` -- Feature landscape, priority tiers, anti-features to avoid
- `.planning/research/PITFALLS.md` -- Pitfalls 2 (impersonation), 3 (resource cleanup), 7 (silence frames), 13 (mute state not communicated to peers)
- `.planning/research/STACK.md` -- Confirmed no new dependencies needed for mute, contacts, or swarm management

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- Zero new dependencies, all patterns verified in existing codebase
- Architecture: HIGH -- All new code follows established patterns with minimal structural changes
- Pitfalls: HIGH -- Known pitfalls documented from codebase analysis and milestone research

**Research date:** 2026-02-16
**Valid until:** 2026-03-16 (stable -- no external API dependencies, all internal patterns)