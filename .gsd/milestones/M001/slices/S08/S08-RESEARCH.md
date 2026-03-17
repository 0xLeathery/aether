# Phase 7: Channel Management - Research

**Researched:** 2026-02-16
**Domain:** Channel CRUD within swarms, CRDT channel metadata sync, creator-only permissions, type-to-confirm deletion
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Channel list placement: Claude's discretion based on existing sidebar layout
- Ordering: Alphabetical sort with "general" pinned at top
- Channel prefix symbol: Claude's discretion (options considered: #, ~, icon)
- Channel switching: Load messages on demand (not all channels in memory at once)
- Create: Plus button in channel list header opens a modal with name input and create button
- Rename: Right-click context menu on channel name -> "Rename" -> inline edit with Enter to confirm
- Naming rules: Claude's discretion for sensible validation
- Permissions: Only the swarm creator can create, rename, or delete channels -- other peers can only use them
- No archive concept -- channels are either active or deleted
- Delete requires type-to-confirm: user must type the channel name to confirm (like GitHub repo delete)
- Deletion removes all associated messages from the CRDT docs (full cleanup, not soft-delete)
- Delete propagates to all peers via CRDT sync
- Each channel gets its own separate Automerge document (not tags on a shared doc)
- Old pre-channel messages are dropped -- clean break, no migration
- New swarms start with two default channels: "general" (text) and "voice" (placeholder)

### Claude's Discretion
- Channel list sidebar placement and visual treatment
- Channel prefix symbol choice
- Channel name validation rules (casing, allowed characters, length limits)
- Loading/transition UX when switching channels
- How "voice" channel placeholder integrates with existing voice system

### Deferred Ideas (OUT OF SCOPE)
- **Channel reordering** -- Swarm creator should be able to custom-reorder channels (drag-to-reorder). Requires ordering metadata in CRDT.
- **RBAC / permissions system** -- Full role-based access control for swarm operations. User wants this as a broader feature beyond just channel management.
</user_constraints>

## Summary

Phase 7 adds channel management to Aether's swarm system: create, rename, and delete topic-based channels with state syncing between peers. The codebase already has substantial infrastructure for this -- `SwarmMetadata` already contains a `channels: Vec<Channel>` field, the `ChannelList.svelte` component already renders channels from swarm metadata, and the `ChatService` already operates on per-channel Automerge documents keyed by `swarm_id/channel_id`. The primary new work is: (1) a CRDT-backed channel metadata document for peer sync, (2) creator-only permission enforcement, (3) channel CRUD Tauri commands, (4) a context menu system (first in the codebase), and (5) type-to-confirm deletion UX.

The most significant architectural decision is how to sync channel metadata between peers. The user decided that channels sync via CRDT (CHAN-04) and that deletion propagates to all peers. This directly conflicts with CRDT Pitfall 1 from prior research (CRDTs resist deletion). The solution is a **dedicated Automerge "swarm metadata" document** that contains the channel list as a CRDT map. Channel deletion uses Automerge's native `delete` operation on the map, which produces tombstones that propagate via sync. Since only the swarm creator can modify channels, concurrent conflicting edits are prevented -- the single-writer model avoids the "add wins over delete" CRDT problem. The swarm metadata CRDT document syncs separately from per-channel message documents.

The creator-only permission model requires adding a `creator_key: String` field to `SwarmMetadata`. The creator is set when `create_swarm` is called (using the local user's public key). Joiners receive the creator key as part of the CRDT metadata sync. All channel mutation commands check `identity.public_key_hex == swarm.creator_key` before proceeding.

**Primary recommendation:** Build a new `SwarmMetadataDocument` (Automerge CRDT) that stores channel list + creator key, synced between peers over a new `/aether/swarm-meta/1.0.0` protocol. Channel CRUD operates on this document. Keep existing `tauri-plugin-store` swarm metadata for local-only fields (name, PSK). The CRDT document is the source of truth for channel list; local store is the source of truth for user-facing swarm name and connection details.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `automerge` | 0.7 | CRDT document for swarm channel metadata | Already in Cargo.toml, used for chat documents |
| `autosurgeon` | 0.10 | Derive Hydrate/Reconcile for channel metadata struct | Already in Cargo.toml, used for ChatMessage |
| `tauri-plugin-store` | 2 | Local-only swarm metadata (name, PSK, creator key cache) | Already in Cargo.toml, established pattern |
| `libp2p-stream` | 0.4.0-alpha | Sync protocol for swarm metadata document | Already in Cargo.toml, used for chat sync |
| `uuid` | 1 (v4) | Generate unique channel IDs | Already in Cargo.toml, used for message IDs |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde` / `serde_json` | 1 | Serialization of Channel, metadata types | Already in Cargo.toml |
| `hex` | 0.4 | Public key hex encoding for creator key comparison | Already in Cargo.toml |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Automerge CRDT for channel metadata | `tauri-plugin-store` only (no sync) | User explicitly wants CHAN-04 (channel list syncs between peers). Local-only storage fails this requirement. |
| Separate swarm-meta protocol | Reuse existing `/aether/chat/1.0.0` protocol | Chat protocol expects channel_id in header. Swarm metadata is swarm-scoped, not channel-scoped. Separate protocol avoids header ambiguity. |
| Automerge Map for channels | Automerge List for channels | Map keyed by channel_id enables O(1) lookup and clean deletion semantics. List requires scanning and has ambiguous deletion behavior in CRDTs. |

**Installation:**
```bash
# No new dependencies required. All libraries already in Cargo.toml.
```

## Architecture Patterns

### Recommended Project Structure

New and modified files for Phase 7:
```
src-tauri/src/
  swarm/
    mod.rs              # MODIFIED: re-export new types
    storage.rs          # MODIFIED: add creator_key to SwarmMetadata, channel CRUD helpers
    metadata_doc.rs     # NEW: SwarmMetadataDocument (Automerge CRDT for channel list)
    metadata_sync.rs    # NEW: Sync protocol for swarm metadata doc
  commands/
    swarm.rs            # MODIFIED: add create_channel, rename_channel, delete_channel
    channel.rs          # NEW: channel-specific Tauri commands (alternative: extend swarm.rs)
  chat/
    mod.rs              # MODIFIED: add remove_channel_document method
    storage.rs          # MODIFIED: add delete_channel_doc function
  error.rs              # MODIFIED: add ChannelError enum variants to SwarmError
  lib.rs                # MODIFIED: register new commands

src/lib/
  tauri.ts              # MODIFIED: new channel command wrappers, events, types
  stores/
    swarm.svelte.ts     # MODIFIED: channel CRUD methods, activeChannelId state
  components/
    layout/
      ChannelList.svelte  # MODIFIED: full rewrite -- interactive channel list with create, context menu
      MainContent.svelte  # MODIFIED: use swarm store activeChannelId instead of hardcoded [0]
    channel/              # NEW DIRECTORY
      CreateChannelDialog.svelte  # NEW: modal for channel creation
      DeleteChannelDialog.svelte  # NEW: type-to-confirm deletion modal
      ChannelContextMenu.svelte   # NEW: right-click context menu (rename, delete)
```

### Pattern 1: SwarmMetadataDocument (CRDT for Channel List)
**What:** A dedicated Automerge document per swarm that stores the channel list and creator identity. Synced between peers to propagate channel changes.
**When to use:** Any data that must be consistent across all peers in a swarm but is NOT per-message (structural metadata).
**Example:**
```rust
// Source: Follows existing ChatDocument pattern in chat/document.rs
use automerge::AutoCommit;
use autosurgeon::{Hydrate, Reconcile, hydrate, reconcile};
use std::collections::HashMap;

/// Channel entry in the CRDT metadata document
#[derive(Debug, Clone, Reconcile, Hydrate, serde::Serialize, serde::Deserialize)]
pub struct ChannelMeta {
    pub name: String,
    pub created_at: i64,
    pub created_by: String, // creator's public key hex
}

/// Swarm metadata stored in an Automerge CRDT document
/// Synced between all peers in the swarm
#[derive(Debug, Clone, Reconcile, Hydrate)]
pub struct SwarmMetaData {
    /// The swarm creator's Ed25519 public key (hex)
    pub creator_key: String,
    /// Channel map: channel_id -> ChannelMeta
    /// Using HashMap for O(1) lookup and clean CRDT deletion
    #[autosurgeon(with = "autosurgeon::map_with_parseable_keys")]
    pub channels: HashMap<String, ChannelMeta>,
}

pub struct SwarmMetadataDocument {
    doc: AutoCommit,
}

impl SwarmMetadataDocument {
    pub fn new(creator_key: &str) -> Result<Self, SwarmError> {
        let mut doc = AutoCommit::new();
        let mut channels = HashMap::new();

        // Default channels
        channels.insert("general".to_string(), ChannelMeta {
            name: "General".to_string(),
            created_at: now_millis(),
            created_by: creator_key.to_string(),
        });
        channels.insert("voice".to_string(), ChannelMeta {
            name: "Voice".to_string(),
            created_at: now_millis(),
            created_by: creator_key.to_string(),
        });

        let data = SwarmMetaData {
            creator_key: creator_key.to_string(),
            channels,
        };
        reconcile(&mut doc, &data)?;
        Ok(Self { doc })
    }

    pub fn add_channel(&mut self, id: &str, meta: ChannelMeta) -> Result<(), SwarmError> {
        let mut data: SwarmMetaData = hydrate(&self.doc)?;
        data.channels.insert(id.to_string(), meta);
        reconcile(&mut self.doc, &data)?;
        Ok(())
    }

    pub fn rename_channel(&mut self, id: &str, new_name: &str) -> Result<(), SwarmError> {
        let mut data: SwarmMetaData = hydrate(&self.doc)?;
        if let Some(ch) = data.channels.get_mut(id) {
            ch.name = new_name.to_string();
        }
        reconcile(&mut self.doc, &data)?;
        Ok(())
    }

    pub fn remove_channel(&mut self, id: &str) -> Result<(), SwarmError> {
        let mut data: SwarmMetaData = hydrate(&self.doc)?;
        data.channels.remove(id);
        reconcile(&mut self.doc, &data)?;
        Ok(())
    }

    pub fn get_channels(&self) -> Result<HashMap<String, ChannelMeta>, SwarmError> {
        let data: SwarmMetaData = hydrate(&self.doc)?;
        Ok(data.channels)
    }

    pub fn get_creator_key(&self) -> Result<String, SwarmError> {
        let data: SwarmMetaData = hydrate(&self.doc)?;
        Ok(data.creator_key)
    }
}
```

### Pattern 2: Creator Permission Check
**What:** Guard channel mutation operations behind a creator key check.
**When to use:** Every channel create/rename/delete command.
**Example:**
```rust
// In commands/swarm.rs (or commands/channel.rs)
fn verify_creator(app: &AppHandle, swarm_id: &str) -> Result<(), String> {
    // Load local identity
    let secret_bytes = storage::load_secret_key()
        .map_err(|e| format!("Identity error: {}", e))?;
    let signing_key = keypair::signing_key_from_bytes(&secret_bytes)
        .map_err(|e| format!("Key error: {}", e))?;
    let local_key = keypair::public_key_to_hex(&signing_key.verifying_key());

    // Load swarm metadata doc to get creator key
    // (from CRDT document or cached in local store)
    let swarm = swarm::storage::get_swarm(app, swarm_id)
        .map_err(|e| e.to_string())?;

    if local_key != swarm.creator_key {
        return Err("Only the swarm creator can manage channels".to_string());
    }
    Ok(())
}
```

### Pattern 3: Context Menu (First in Codebase)
**What:** A right-click context menu for channel actions (rename, delete).
**When to use:** The user specified right-click -> "Rename" -> inline edit.
**Example:**
```svelte
<!-- ChannelContextMenu.svelte -->
<script lang="ts">
  let { x, y, channelId, channelName, onRename, onDelete, onClose }: {
    x: number;
    y: number;
    channelId: string;
    channelName: string;
    onRename: () => void;
    onDelete: () => void;
    onClose: () => void;
  } = $props();
</script>

<svelte:window onclick={onClose} />

<div class="context-menu" style="left: {x}px; top: {y}px;">
  <button class="menu-item" onclick={onRename}>Rename</button>
  <button class="menu-item danger" onclick={onDelete}>Delete</button>
</div>

<style>
  .context-menu {
    position: fixed;
    z-index: 1000;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 0.25rem 0;
    min-width: 140px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
  }
  .menu-item {
    width: 100%;
    font-family: var(--font-mono);
    font-size: 0.85rem;
    padding: 0.5rem 0.75rem;
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    text-align: left;
  }
  .menu-item:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }
  .menu-item.danger:hover {
    color: #ff4444;
  }
</style>
```

### Pattern 4: Type-to-Confirm Deletion (GitHub Pattern)
**What:** User must type the channel name to confirm deletion.
**When to use:** Channel deletion only (user decided this pattern).
**Example:**
```svelte
<!-- DeleteChannelDialog.svelte -->
<script lang="ts">
  let { channelName, onConfirm, onCancel }: {
    channelName: string;
    onConfirm: () => void;
    onCancel: () => void;
  } = $props();

  let confirmText = $state('');
  let isMatch = $derived(confirmText === channelName);
</script>

<div class="modal-overlay" onclick={onCancel}>
  <div class="modal-container" onclick={(e) => e.stopPropagation()}>
    <h2>Delete Channel</h2>
    <p>Type <strong>{channelName}</strong> to confirm deletion.</p>
    <p class="warning">This will permanently delete all messages in this channel.</p>
    <input
      type="text"
      bind:value={confirmText}
      placeholder="Type channel name..."
    />
    <div class="actions">
      <button onclick={onCancel}>Cancel</button>
      <button class="danger" onclick={onConfirm} disabled={!isMatch}>
        Delete Channel
      </button>
    </div>
  </div>
</div>
```

### Pattern 5: Swarm Metadata Sync Protocol
**What:** A dedicated libp2p stream protocol for syncing the swarm metadata CRDT document.
**When to use:** When peers connect, sync channel metadata before syncing message documents.
**Example:**
```rust
// Source: Follows existing chat/protocol.rs pattern
use libp2p::StreamProtocol;

/// Swarm metadata sync protocol identifier
pub const SWARM_META_PROTOCOL: StreamProtocol =
    StreamProtocol::new("/aether/swarm-meta/1.0.0");

// Sync flow:
// 1. On peer connect, open stream with SWARM_META_PROTOCOL
// 2. Write swarm_id header (same framing as chat protocol)
// 3. Run Automerge sync loop (reuse sync::sync_document)
// 4. If changes received, emit "swarm-metadata-updated" event
// 5. Frontend re-reads channel list from backend
```

### Anti-Patterns to Avoid
- **Storing channel list ONLY in local `tauri-plugin-store`:** This fails CHAN-04 (channel list syncs between peers). The channel list must live in a CRDT document for peer sync. Local store caches but does not own the channel list.
- **Using Automerge Vec/List for channels:** Lists in Automerge have ambiguous deletion semantics. Use a Map keyed by channel_id instead. Maps support clean `delete(key)` with tombstones that propagate via sync.
- **Allowing all peers to mutate channel metadata:** The user decided only the swarm creator can create/rename/delete channels. Without this single-writer constraint, concurrent channel mutations (add vs delete) in a CRDT would cause the "add wins" ghost problem described in Pitfall 1.
- **Syncing channel metadata inside the existing chat protocol:** The chat protocol expects a `swarm_id\nchannel_id\n` header. Swarm metadata is not channel-scoped. Use a separate protocol to keep clean separation.
- **Deleting channels without cleaning up message documents:** Channel deletion must also delete the Automerge message document file (`{channel_id}.automerge`), evict the document from `ChatService.documents`, and remove sync states from `PeerSyncStates`.
- **Blocking UI on metadata sync:** Swarm metadata sync should happen in the background. The frontend should render channels from cached state immediately, then update reactively when sync completes.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Channel metadata sync | Custom JSON-over-TCP protocol | Automerge sync protocol over libp2p-stream | Already proven in chat sync. CRDT handles conflict resolution. |
| Channel ID generation | Custom timestamp-based IDs | `uuid::Uuid::new_v4()` | Already used for message IDs. Globally unique, no collision risk. |
| Context menu positioning | Custom viewport boundary detection | CSS `position: fixed` with `x/y` from `oncontextmenu` event | Simple and reliable. Adjust with `Math.min(x, window.innerWidth - menuWidth)` for edge cases. |
| Channel name validation | Custom regex from scratch | Standard Discord-like rules: lowercase, alphanumeric + hyphens, 1-32 chars | Well-understood, predictable. Normalize input rather than reject. |
| Creator key comparison | Custom crypto verification | Simple string equality of hex-encoded public keys | Keys are already Ed25519 hex strings. No cryptographic verification needed -- the key IS the identity. |

**Key insight:** Phase 7 requires ZERO new crate dependencies. All new functionality builds on existing Automerge, autosurgeon, libp2p-stream, and tauri-plugin-store patterns already proven in the chat system.

## Common Pitfalls

### Pitfall 1: CRDT Channel Deletion Ghost Problem
**What goes wrong:** Creator deletes a channel. An offline peer syncs later and the channel reappears because CRDTs use "add wins" semantics.
**Why it happens:** In a multi-writer CRDT, if peer B writes to a channel that peer A deleted, the merge produces a state where the channel exists (B's write wins over A's delete). This is fundamental CRDT behavior.
**How to avoid:** The single-writer constraint (only swarm creator can mutate channels) prevents this. Since only one identity can add/remove channels, there can be no concurrent conflicting operations. The creator's delete is authoritative. If any peer somehow writes a channel entry (which the enforced permissions prevent), the creator's next mutation will reconcile it away.
**Warning signs:** If the system ever allows multiple writers to the channel metadata CRDT, ghost channels will appear.

### Pitfall 2: Inconsistent State Between CRDT Metadata and Local Store
**What goes wrong:** The CRDT channel metadata says channels ["general", "dev", "design"] exist, but the local `swarms.json` still has `channels: [{id: "general", name: "General"}]` from before sync. The UI shows stale data.
**Why it happens:** Two sources of truth for channel list -- the CRDT document (synced) and the local store (cached). If they fall out of sync, the UI depends on which one the frontend reads.
**How to avoid:** Make the CRDT document the single source of truth for channel list. The local `SwarmMetadata.channels` field becomes a cache that is overwritten after every metadata sync. The frontend reads channels from the CRDT-backed source. On metadata sync event, re-fetch from backend.
**Warning signs:** Channel list differs between peers after sync has completed.

### Pitfall 3: Channel Deletion Without Message Document Cleanup
**What goes wrong:** Channel is deleted from metadata, but the `.automerge` file remains on disk. If a new channel with the same ID is later created (unlikely with UUIDs but possible), it inherits old messages.
**Why it happens:** Deletion only removes the channel entry from the metadata CRDT but does not clean up the separate per-channel Automerge message document.
**How to avoid:** Channel deletion must execute a multi-step cleanup sequence:
1. Remove channel from SwarmMetadataDocument (CRDT)
2. Delete `{app_data_dir}/chat/{swarm_id}/{channel_id}.automerge` from disk
3. Remove `"{swarm_id}/{channel_id}"` from `ChatService.documents` HashMap
4. Remove related entries from `PeerSyncStates`
5. Emit `channel-deleted` event to frontend
6. If deleted channel was active, auto-switch to "general"
**Warning signs:** Orphaned `.automerge` files in the chat data directory.

### Pitfall 4: Context Menu Opens Off-Screen
**What goes wrong:** Right-clicking a channel near the bottom or right edge of the window causes the context menu to render partially outside the viewport.
**Why it happens:** The context menu uses `position: fixed` with the raw mouse coordinates from the `contextmenu` event. No boundary checking.
**How to avoid:** Clamp the position:
```typescript
const menuWidth = 160; // approximate
const menuHeight = 80; // approximate
const x = Math.min(event.clientX, window.innerWidth - menuWidth);
const y = Math.min(event.clientY, window.innerHeight - menuHeight);
```
**Warning signs:** Context menu items cut off or hidden behind window edges.

### Pitfall 5: Race Between Metadata Sync and Channel Deletion
**What goes wrong:** Creator deletes a channel. Meanwhile, a sync with another peer is in progress that includes that channel's message document. The sync completes and re-creates the message document on disk after deletion cleaned it up.
**Why it happens:** Chat sync and metadata sync are independent. A chat sync for `swarm_id/channel_id` can trigger `get_or_load_doc()` which creates a new empty document if the file does not exist.
**How to avoid:** After processing a metadata sync that removes a channel, check if any message documents exist for removed channels and clean them up. The `channel-deleted` event should trigger cleanup on the receiving peer's side too. Additionally, `get_or_load_doc` could check if the channel exists in the metadata before creating a new document.
**Warning signs:** `.automerge` files reappearing after channel deletion on peer sync.

### Pitfall 6: Joiner Does Not Know Creator Key
**What goes wrong:** A new peer joins a swarm via `join_swarm`. They do not have the creator key, so they cannot determine who the creator is. Their local `SwarmMetadata.creator_key` is empty. They might assume they are the creator and try to manage channels.
**Why it happens:** `join_swarm` only receives the PSK via `aether://` URI. The creator identity is not encoded in the URI.
**How to avoid:** The creator key is stored in the CRDT metadata document. When a new peer joins and syncs the metadata document for the first time, they receive the creator key. Until metadata sync completes, the joiner should see channels as read-only (no create/rename/delete buttons). The local `SwarmMetadata.creator_key` is populated from the first metadata sync. For the swarm creator, `creator_key` is set at `create_swarm` time (known immediately).
**Warning signs:** New joiner sees channel management buttons before metadata sync.

## Code Examples

Verified patterns from the existing codebase, adapted for Phase 7:

### Channel Name Validation (Claude's Discretion)
```typescript
// Recommended validation rules (Discord-inspired):
// - Lowercase only (auto-convert on input)
// - Allowed characters: a-z, 0-9, hyphens (-)
// - Length: 1-32 characters
// - No leading/trailing hyphens
// - No consecutive hyphens
// - Reserved names: "general", "voice" (cannot create, only system-generated)

function validateChannelName(name: string): { valid: boolean; error?: string } {
  const normalized = name.toLowerCase().trim();

  if (normalized.length === 0) return { valid: false, error: 'Name cannot be empty' };
  if (normalized.length > 32) return { valid: false, error: 'Name must be 32 characters or fewer' };
  if (!/^[a-z0-9][a-z0-9-]*[a-z0-9]$|^[a-z0-9]$/.test(normalized)) {
    return { valid: false, error: 'Only lowercase letters, numbers, and hyphens allowed' };
  }
  if (normalized.includes('--')) return { valid: false, error: 'No consecutive hyphens' };
  if (['general', 'voice'].includes(normalized)) {
    return { valid: false, error: 'Reserved channel name' };
  }

  return { valid: true };
}

function normalizeChannelName(input: string): string {
  return input
    .toLowerCase()
    .replace(/\s+/g, '-')
    .replace(/[^a-z0-9-]/g, '')
    .replace(/-{2,}/g, '-')
    .replace(/^-|-$/g, '')
    .substring(0, 32);
}
```

### Create Channel Command (Rust)
```rust
// Follows existing create_swarm pattern in commands/swarm.rs
#[tauri::command]
pub async fn create_channel(
    app: AppHandle,
    chat_service: State<'_, Arc<tokio::sync::Mutex<ChatService>>>,
    swarm_id: String,
    name: String,
) -> Result<Channel, String> {
    // 1. Verify creator permission
    verify_creator(&app, &swarm_id)?;

    // 2. Validate name
    let normalized = normalize_channel_name(&name);
    if normalized.is_empty() {
        return Err("Invalid channel name".to_string());
    }

    // 3. Generate channel ID
    let channel_id = uuid::Uuid::new_v4().to_string();

    // 4. Add to CRDT metadata document
    // (SwarmMetadataDocument manages the Automerge doc)
    let meta_doc = get_or_load_metadata_doc(&app, &swarm_id)?;
    meta_doc.add_channel(&channel_id, ChannelMeta {
        name: normalized.clone(),
        created_at: now_millis(),
        created_by: get_local_public_key()?,
    })?;
    save_metadata_doc(&app, &swarm_id, meta_doc)?;

    // 5. Update local SwarmMetadata cache
    let mut metadata = swarm::storage::get_swarm(&app, &swarm_id)
        .map_err(|e| e.to_string())?;
    metadata.channels.push(Channel {
        id: channel_id.clone(),
        name: normalized.clone(),
    });
    swarm::storage::save_swarm(&app, &metadata)
        .map_err(|e| e.to_string())?;

    // 6. Trigger metadata sync to peers
    trigger_metadata_sync_to_peers(&app, &swarm_id);

    Ok(Channel { id: channel_id, name: normalized })
}
```

### Delete Channel Command with Full Cleanup (Rust)
```rust
#[tauri::command]
pub async fn delete_channel(
    app: AppHandle,
    chat_service: State<'_, Arc<tokio::sync::Mutex<ChatService>>>,
    swarm_id: String,
    channel_id: String,
) -> Result<(), String> {
    // Guard: cannot delete "general" or "voice"
    if channel_id == "general" || channel_id == "voice" {
        return Err("Cannot delete default channels".to_string());
    }

    // 1. Verify creator permission
    verify_creator(&app, &swarm_id)?;

    // 2. Remove from CRDT metadata document
    let meta_doc = get_or_load_metadata_doc(&app, &swarm_id)?;
    meta_doc.remove_channel(&channel_id)?;
    save_metadata_doc(&app, &swarm_id, meta_doc)?;

    // 3. Delete Automerge message document from disk
    if let Ok(data_dir) = app.path().app_data_dir() {
        let doc_path = data_dir
            .join("chat")
            .join(&swarm_id)
            .join(format!("{}.automerge", &channel_id));
        if doc_path.exists() {
            let _ = std::fs::remove_file(&doc_path);
        }
    }

    // 4. Evict from in-memory ChatService
    {
        let mut service = chat_service.lock().await;
        service.remove_channel_document(&swarm_id, &channel_id);
    }

    // 5. Update local SwarmMetadata cache
    let mut metadata = swarm::storage::get_swarm(&app, &swarm_id)
        .map_err(|e| e.to_string())?;
    metadata.channels.retain(|ch| ch.id != channel_id);
    swarm::storage::save_swarm(&app, &metadata)
        .map_err(|e| e.to_string())?;

    // 6. Emit event to frontend
    let _ = app.emit("channel-deleted", serde_json::json!({
        "swarm_id": swarm_id,
        "channel_id": channel_id,
    }));

    // 7. Trigger metadata sync to peers
    trigger_metadata_sync_to_peers(&app, &swarm_id);

    Ok(())
}
```

### ChannelList.svelte with Active Channel, Create, and Context Menu
```svelte
<script lang="ts">
  import { swarmStore } from '../../stores/swarm.svelte';

  let showCreateDialog = $state(false);
  let contextMenu = $state<{ x: number; y: number; channelId: string; channelName: string } | null>(null);

  // Sort channels: "general" first, then "voice", then alphabetical
  let sortedChannels = $derived(() => {
    if (!swarmStore.activeSwarm) return [];
    const channels = [...swarmStore.activeSwarm.channels];
    return channels.sort((a, b) => {
      if (a.id === 'general') return -1;
      if (b.id === 'general') return 1;
      if (a.id === 'voice') return -1;
      if (b.id === 'voice') return 1;
      return a.name.localeCompare(b.name);
    });
  });

  function handleContextMenu(event: MouseEvent, channel: Channel) {
    // Only show for non-default channels, and only for creator
    if (channel.id === 'general' || channel.id === 'voice') return;
    if (!swarmStore.isCreator) return;

    event.preventDefault();
    contextMenu = {
      x: event.clientX,
      y: event.clientY,
      channelId: channel.id,
      channelName: channel.name,
    };
  }
</script>
```

### Metadata Document Storage Path
```rust
// Follows existing chat/storage.rs pattern
// Path: {app_data_dir}/swarm-meta/{swarm_id}.automerge

pub fn metadata_doc_path(app: &AppHandle, swarm_id: &str) -> Result<PathBuf, SwarmError> {
    let data_dir = app.path().app_data_dir()
        .map_err(|e| SwarmError::StorageError(format!("Path error: {}", e)))?;
    Ok(data_dir.join("swarm-meta").join(format!("{}.automerge", swarm_id)))
}
```

## Discretion Recommendations

### Channel List Placement and Visual Treatment
**Recommendation:** Keep the existing `ChannelList.svelte` in its current position between Sidebar and MainContent in the AppShell layout. The existing 200px width and `var(--bg-primary)` background work well. Add a "CHANNELS" header with a `[+]` button for creators, matching the existing "SWARMS" header pattern in Sidebar.svelte (monospace, uppercase, muted color, letter-spacing).

### Channel Prefix Symbol
**Recommendation:** Use `#` (hash). The existing `ChannelList.svelte` already uses `#` (line 17: `<span class="channel-hash">#</span>`). This is familiar from Discord and Slack. The `#` is rendered in `var(--text-muted)` with 0.7 opacity, matching the existing style. For the "voice" channel, use a different symbol -- the existing monospace style means a simple `[v]` or Unicode speaker icon would work, but `#` is simpler and consistent.

### Channel Name Validation Rules
**Recommendation:** Discord-like rules as shown in code examples above. Lowercase, alphanumeric + hyphens, 1-32 characters, no consecutive hyphens, auto-normalize on input. This provides a clean, predictable naming system without surprising users.

### Loading/Transition UX When Switching Channels
**Recommendation:** Follow the existing pattern in `ChatPanel.svelte` -- the `chatStore.switchChannel()` method clears messages immediately (`messages = []`) and shows a loading state while new messages load. This gives instant visual feedback. Add a subtle fade or opacity transition on the channel list active state change (already has `transition: all 0.2s ease` on `.channel-button`).

### Voice Channel Placeholder
**Recommendation:** Render the "voice" channel in the channel list with the same `#` prefix but with a different visual treatment -- perhaps `var(--accent-amber)` color instead of the standard channel color. Clicking it does NOT switch the chat view (voice has no messages). Instead, clicking the voice channel could focus/scroll to the VoicePanel in MainContent, or simply do nothing with a tooltip "Voice channels are managed from the voice panel." This avoids over-engineering a voice-channel integration that is better handled in a future phase. The voice channel exists as a visual placeholder per the user's decision.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Channels hardcoded at swarm creation | Dynamic channel CRUD via CRDT | Phase 7 | Enables multi-topic conversations within a swarm |
| No creator concept | Creator key in SwarmMetadata + CRDT | Phase 7 | Enables permission-gated channel management |
| Single "general" channel per swarm | Default "general" + "voice" channels | Phase 7 | Voice placeholder for future integration |
| Automerge Vec for data lists | Automerge Map via HashMap for channels | Phase 7 | Clean deletion semantics, O(1) lookup |

**Deprecated/outdated:**
- The current `SwarmMetadata.channels: Vec<Channel>` in `swarm/storage.rs` becomes a local cache, not the source of truth. The CRDT document becomes authoritative for channel list.
- `MainContent.svelte` line 17 (`swarmStore.activeSwarm?.channels?.[0]?.id`) hardcodes the first channel as active. This must change to use an explicit `activeChannelId` from the swarm store.

## Open Questions

1. **HashMap key type compatibility with autosurgeon**
   - What we know: autosurgeon provides `map_with_parseable_keys` adaptor for HashMap types where keys implement `ToString + FromStr`. String keys should work natively without the adaptor.
   - What's unclear: Whether `HashMap<String, ChannelMeta>` requires the `map_with_parseable_keys` adaptor or works directly with `#[derive(Reconcile, Hydrate)]`.
   - Recommendation: Test both approaches. String keys may work without the adaptor since `String` implements `AsRef<str>`. If not, use `#[autosurgeon(with = "autosurgeon::map_with_parseable_keys")]`.

2. **Metadata sync timing relative to chat sync**
   - What we know: Currently, chat sync triggers on peer connection via `start_sync_listener`. Metadata sync needs to happen too.
   - What's unclear: Should metadata sync happen before or after chat sync? If a peer learns about new channels via metadata sync, they may then need to sync those channels' message documents.
   - Recommendation: Trigger metadata sync FIRST on peer connection. After metadata sync completes and channels are known, trigger chat sync for all known channels. This ensures the joiner knows which channels exist before trying to load their messages.

3. **How receiving peers handle channel deletion from CRDT sync**
   - What we know: The creator deletes a channel from the CRDT metadata document. Other peers receive this change via metadata sync.
   - What's unclear: When a non-creator peer receives a metadata sync that removes a channel, should they automatically delete their local message `.automerge` file and clear their in-memory cache?
   - Recommendation: Yes. On metadata sync, compare the previous channel list with the new one. For any removed channels, run the same cleanup sequence (delete file, evict from ChatService, remove sync states). Emit `channel-deleted` event to update the frontend. If the deleted channel was active, auto-switch to "general".

4. **Creator key migration for existing swarms**
   - What we know: Existing swarms created before Phase 7 do not have a `creator_key` field. They have `channels: [{id: "general", name: "General"}]` only.
   - What's unclear: How to handle swarms that pre-date the creator concept.
   - Recommendation: When loading a swarm that lacks a `creator_key`, treat the local user as the creator (they are the only user who has this swarm locally). Populate `creator_key` with the local user's public key and save. This is safe because in v1.0, swarms are functionally single-user until peers join. The first person to create the metadata CRDT document establishes the creator key.

## Sources

### Primary (HIGH confidence)
- Existing codebase analysis -- all code patterns verified against actual source files:
  - `src-tauri/src/chat/document.rs` -- Automerge AutoCommit + autosurgeon Hydrate/Reconcile pattern
  - `src-tauri/src/chat/mod.rs` -- ChatService per-channel document cache, sync orchestration
  - `src-tauri/src/chat/sync.rs` -- Automerge sync loop with PeerSyncStates
  - `src-tauri/src/chat/protocol.rs` -- libp2p-stream protocol definition, framing
  - `src-tauri/src/chat/storage.rs` -- Automerge document persistence pattern
  - `src-tauri/src/swarm/storage.rs` -- SwarmMetadata with existing Channel struct and Vec<Channel>
  - `src-tauri/src/commands/swarm.rs` -- create_swarm, join_swarm, leave_swarm cleanup patterns
  - `src-tauri/src/commands/chat.rs` -- send_message, get_messages with peer sync trigger
  - `src-tauri/src/error.rs` -- Error enum patterns with thiserror + Serialize
  - `src-tauri/src/lib.rs` -- Command registration, managed state pattern
  - `src/lib/components/layout/ChannelList.svelte` -- Existing channel list with # prefix
  - `src/lib/components/layout/Sidebar.svelte` -- Sidebar layout, header patterns
  - `src/lib/components/layout/MainContent.svelte` -- Channel derivation from swarm, ChatPanel rendering
  - `src/lib/components/layout/AppShell.svelte` -- Three-panel layout (Sidebar, ChannelList, MainContent)
  - `src/lib/components/swarm/SwarmSettings.svelte` -- Modal pattern, danger zone, button styles
  - `src/lib/components/chat/ChatPanel.svelte` -- Channel switching, $effect for swarm/channel changes
  - `src/lib/stores/swarm.svelte.ts` -- Svelte 5 store pattern with reactive getters
  - `src/lib/stores/chat.svelte.ts` -- switchChannel, loadMessages, chat event listener
  - `src/lib/tauri.ts` -- TypeScript IPC bindings, Channel type definition
- Context7 `/automerge/automerge` -- Rust sync protocol, AutoCommit API (HIGH confidence)
- Context7 `/websites/rs_autosurgeon` -- Reconcile/Hydrate derive, HashMap with parseable keys adaptor (HIGH confidence)

### Secondary (MEDIUM confidence)
- `.planning/research/ARCHITECTURE.md` -- Section 3 (Channel CRUD) integration architecture
- `.planning/research/PITFALLS.md` -- Pitfall 1 (CRDT deletion ghosts), Pitfall 3 (resource cleanup), Pitfall 12 (inconsistent stores)
- `.planning/phases/06-foundation/06-RESEARCH.md` -- Phase 6 patterns (leave_swarm cleanup, store access, Tauri command template)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- Zero new dependencies, all patterns verified in existing codebase
- Architecture: HIGH -- CRDT metadata document follows proven ChatDocument pattern; single-writer constraint solves CRDT deletion problem
- Pitfalls: HIGH -- Six specific pitfalls identified with concrete prevention strategies; prior research (PITFALLS.md) cross-referenced

**Research date:** 2026-02-16
**Valid until:** 2026-03-16 (stable -- no external API dependencies, all internal patterns)