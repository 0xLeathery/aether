# Phase 9.1: Fix isCreator Integration Bug - Research

**Researched:** 2026-02-23
**Domain:** Svelte 5 store wiring, Tauri command authorization, Automerge CRDT sync filtering
**Confidence:** HIGH

## Summary

This phase fixes a single integration bug: `swarmStore.setLocalIdentity()` is exported but never called, so `localPublicKey` remains null, `isCreator` always evaluates to false, and the channel create/rename/delete UI is permanently hidden. The primary fix is a one-line call in `App.svelte`. However, the user's CONTEXT.md decisions expand scope to include backend enforcement (Rust-side creator checks in Tauri commands), CRDT sync-layer filtering (reject unauthorized channel operations from peers), and auto-migration for channels with missing creator metadata.

The backend already has `verify_creator()` in `commands/channel.rs` that checks local identity against the CRDT metadata document's `creator_key`. This means backend enforcement is partially done for create/rename/delete commands. The remaining work is: (1) wiring the `setLocalIdentity` call, (2) adding sync-layer filtering to reject unauthorized CRDT changes, and (3) auto-migrating channels missing creator metadata.

**Primary recommendation:** Wire `swarmStore.setLocalIdentity(identity.public_key_hex)` in `App.svelte` after identity loads, add post-sync validation in the metadata sync path to reject unauthorized channel mutations, and run a silent startup migration for channels with missing creator metadata.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Call `swarmStore.setLocalIdentity(identity.public_key_hex)` once in App.svelte during app startup, after identity loads from keychain
- If identity is not available at startup (first launch, keychain locked), skip the call -- channel management UI stays hidden until identity loads
- Call once per session -- no retry, no reactive subscription to identity changes
- Do NOT use swarmStore self-initialization pattern -- keep it as an explicit call in App.svelte
- Only the swarm creator can create, rename, and delete channels (matches existing `isCreator` gate logic)
- **UI gate:** Hide [+] create button, rename menu item, and delete menu item for non-creators (existing behavior, just needs isCreator to actually work)
- **Backend enforcement:** Add creator identity check in Tauri command handlers (Rust side) for create_channel, rename_channel, delete_channel -- reject with error if caller is not swarm creator
- **CRDT sync enforcement:** Filter out channel-create/rename/delete CRDT operations from non-creator peers at the sync layer -- do not merge unauthorized operations
- Auto-migrate channels with missing or incorrect creator metadata on app startup
- Set creator field to the swarm creator's public key for any channel lacking valid creator metadata
- Migration runs silently -- no user action required, no UI prompt
- The default "general" channel is unaffected (it's system-created, not user-managed)

### Claude's Discretion
None specified -- all decisions are locked.

### Deferred Ideas (OUT OF SCOPE)
None -- discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| CHAN-01 | User can create a new named channel within a swarm | The `setLocalIdentity` fix makes `isCreator` evaluate correctly, revealing the [+] create channel button. Backend `create_channel` command already has `verify_creator()`. Sync-layer filtering prevents unauthorized peers from injecting channels. |
| CHAN-02 | User can rename an existing channel | Same `isCreator` fix reveals the "Rename" context menu item. Backend `rename_channel` command already has `verify_creator()`. Sync-layer filtering prevents unauthorized renames. |
| CHAN-03 | User can delete (archive) a channel with confirmation | Same `isCreator` fix reveals the "Delete" context menu item. Backend `delete_channel` command already has `verify_creator()`. Sync-layer filtering prevents unauthorized deletions. |
</phase_requirements>

## Standard Stack

### Core (already in project)
| Library | Purpose | Why Standard |
|---------|---------|--------------|
| Svelte 5 (runes) | Frontend reactivity with `$state`, `$derived`, `$props` | Already used throughout; `swarmStore` uses `$state` for `localPublicKey` |
| Tauri 2 | Desktop app framework, Rust backend, IPC invoke/listen | All commands already registered in `lib.rs` |
| Automerge + Autosurgeon | CRDT document for channel metadata | `SwarmMetadataDocument` already wraps `AutoCommit` with typed hydrate/reconcile |
| ed25519-dalek | Identity keypair, public key hex used as creator identity | Already used for all identity operations |

### Supporting
| Library | Purpose | When to Use |
|---------|---------|-------------|
| tauri-plugin-store | Local swarm metadata persistence (`swarms.json`) | Already used by `swarm::storage` |
| libp2p-stream | Stream protocol for metadata sync | Already used for `SWARM_META_PROTOCOL` |

### Alternatives Considered
None -- this is a bug fix phase using existing stack. No new libraries needed.

## Architecture Patterns

### Pattern 1: Identity Wiring in App.svelte
**What:** After `identityStore.initialize()` resolves with a valid identity, call `swarmStore.setLocalIdentity(identity.public_key_hex)` to populate the `localPublicKey` state variable before any swarm operations occur.
**When to use:** Both startup paths -- existing identity load and post-setup identity creation.
**How it works:**

In `App.svelte` `onMount`:
```typescript
// After identityStore.initialize()
if (identityStore.identity) {
  swarmStore.setLocalIdentity(identityStore.identity.public_key_hex);
  // ... then networkStore, swarmStore init
}
```

In `handleSetupComplete`:
```typescript
async function handleSetupComplete() {
  // Identity was just created, identityStore.identity is now set
  swarmStore.setLocalIdentity(identityStore.identity!.public_key_hex);
  // ... then networkStore, swarmStore init
}
```

**Key detail:** The `setLocalIdentity` function (swarm.svelte.ts L268-270) simply sets `localPublicKey = publicKeyHex`. The `isCreator` getter (L276) compares `activeSwarm?.creator_key` with `localPublicKey`. Once `localPublicKey` is non-null and matches, `isCreator` returns true.

### Pattern 2: Backend verify_creator (Already Exists)
**What:** The `verify_creator()` function in `commands/channel.rs` (L21-41) loads the local identity from keychain, loads the CRDT metadata doc, extracts `creator_key`, and compares. If they don't match, it returns `Err("Only the swarm creator can manage channels")`.
**Status:** Already implemented and called in `create_channel`, `rename_channel`, `delete_channel`. No changes needed.

### Pattern 3: CRDT Sync-Layer Filtering (New)
**What:** After receiving and applying sync messages in `sync_metadata_document()`, validate that any new channel entries were created by the swarm creator. If unauthorized changes are detected, roll them back.
**When to use:** In the metadata sync path (`metadata_sync.rs`) after receiving changes.
**Approach options:**

**Option A: Post-sync validation** -- After sync completes, hydrate the document, compare channels before/after, reject unauthorized additions/modifications by re-reconciling with the pre-sync channel set. This is simpler but has a race window.

**Option B: Pre-merge snapshot + diff** -- Take a snapshot of channels before sync, apply sync, diff channels after, revert unauthorized changes. More robust.

**Key challenge:** Automerge's sync protocol applies changes atomically. You can't selectively reject individual operations within a sync message. The approach is: (1) snapshot channels before sync, (2) apply sync normally, (3) hydrate and check, (4) if unauthorized changes found, reconcile with corrected data (effectively overwriting the unauthorized changes locally). The corrected state will propagate back to peers on next sync.

**Important:** The CRDT metadata document stores `creator_key` as a top-level field. After sync, check `doc.get_creator_key()` to identify the legitimate creator, then verify all channel entries have `created_by` matching the creator key. Remove any channels where `created_by` does not match.

### Pattern 4: Auto-Migration for Missing Creator Metadata
**What:** On app startup, iterate all swarms, load their metadata documents, and ensure every channel has a valid `created_by` field matching the swarm's `creator_key`.
**When to use:** During swarm initialization, before UI renders.
**Where:** Best done as a new function called from `App.svelte` after `swarmStore.initialize()`, or as a Tauri command called once at startup.
**Key detail:** The `SwarmMetadata.creator_key` in `swarm::storage` is `Option<String>` with `#[serde(default)]`. For swarms where `creator_key` is `None` (pre-Phase-7 backward compat), the migration should load the CRDT doc's `creator_key` and backfill it into the local store.

### Anti-Patterns to Avoid
- **Reactive identity subscription:** CONTEXT.md explicitly says "no reactive subscription to identity changes." Call `setLocalIdentity` once per session, not in a `$effect`.
- **Self-initialization in swarmStore:** CONTEXT.md says "Do NOT use swarmStore self-initialization pattern." Keep the call explicit in App.svelte.
- **Modifying Automerge sync protocol:** Don't try to intercept individual Automerge operations during sync. Use post-sync validation instead.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Creator identity comparison | Custom crypto comparison | Simple string equality on hex public keys | Both sides use `keypair::public_key_to_hex()` producing identical hex strings |
| CRDT conflict resolution | Custom merge logic | Automerge's built-in CRDT resolution + post-sync validation | Automerge handles concurrent edits; we only need to enforce authorization after merge |
| Channel metadata persistence | Custom file format | Existing `SwarmMetadataDocument` + `metadata_storage` module | Already handles save/load/bytes conversion |

**Key insight:** The backend creator verification is already fully implemented. The only missing piece on the frontend is a single function call. The CRDT sync filtering and migration are the substantive new work.

## Common Pitfalls

### Pitfall 1: setLocalIdentity Called Too Late
**What goes wrong:** If `setLocalIdentity` is called after `swarmStore.initialize()` and `activateDefaultSwarm()`, the `isCreator` getter may be evaluated (e.g., by ChannelList rendering) before `localPublicKey` is set, resulting in a flash of non-creator UI.
**Why it happens:** Svelte 5 reactivity evaluates derived state immediately when dependencies change.
**How to avoid:** Call `setLocalIdentity` BEFORE `swarmStore.initialize()` and `activateDefaultSwarm()`. Order: identityStore.initialize() -> setLocalIdentity -> networkStore -> swarmStore.
**Warning signs:** [+] button not visible on first render but appearing after a re-render.

### Pitfall 2: Missing setLocalIdentity in Setup Flow Path
**What goes wrong:** New users who go through the setup flow (identity creation) never get `setLocalIdentity` called because `handleSetupComplete` doesn't include it.
**Why it happens:** Two code paths to "app ready" state: existing identity (onMount) and new identity (handleSetupComplete). Easy to fix one and miss the other.
**How to avoid:** Add `setLocalIdentity` to both paths.
**Warning signs:** New users can't manage channels in their first-created swarm.

### Pitfall 3: Joined Swarm Has No creator_key
**What goes wrong:** When a user joins a swarm via invite URI, the local `SwarmMetadata.creator_key` is `None` (see `join_swarm` in commands/swarm.rs L82-99). The `isCreator` getter checks `activeSwarm?.creator_key`, which will be null until the metadata CRDT syncs from the creator peer.
**Why it happens:** The joiner doesn't know who the creator is until they sync the metadata document.
**How to avoid:** This is expected behavior -- joiners are not creators. The `isCreator` check correctly returns false. After CRDT metadata sync, the local `creator_key` cache should be updated from the synced CRDT doc.
**Warning signs:** None -- this is correct behavior. Just document it clearly.

### Pitfall 4: CRDT Sync Filter Deleting Legitimate Channels
**What goes wrong:** Over-aggressive sync filtering removes channels that were legitimately created by the swarm creator on another device or before the metadata document was available locally.
**Why it happens:** If the local node doesn't yet have the correct `creator_key` (e.g., first sync), it might incorrectly judge all channels as unauthorized.
**How to avoid:** Only filter channels if the local node has a confirmed `creator_key` in its CRDT document. If the document is brand new (no prior creator_key), accept all incoming data as the initial state.
**Warning signs:** Channels disappearing after peer sync.

### Pitfall 5: Migration Overwriting CRDT State
**What goes wrong:** Auto-migration modifies the CRDT document locally, which then propagates to peers during sync, potentially overwriting the creator's authoritative state.
**Why it happens:** Migration runs before sync, setting `created_by` fields locally. When sync happens, both sides have different views.
**How to avoid:** Migration should only fill in MISSING `created_by` fields, never overwrite existing ones. Use the CRDT doc's `creator_key` as the source of truth for what to fill in.
**Warning signs:** Channel metadata flip-flopping between syncs.

## Code Examples

### Current isCreator Evaluation Chain (Broken)

```typescript
// swarm.svelte.ts L24 -- always null because setLocalIdentity never called
let localPublicKey = $state<string | null>(null);

// swarm.svelte.ts L268-270 -- exported but never called
function setLocalIdentity(publicKeyHex: string) {
  localPublicKey = publicKeyHex;
}

// swarm.svelte.ts L276 -- always false because localPublicKey is null
get isCreator() { return activeSwarm?.creator_key != null && activeSwarm.creator_key === localPublicKey; },
```

### Fix: Wire setLocalIdentity in App.svelte

```typescript
// App.svelte onMount -- add setLocalIdentity call before swarm init
onMount(async () => {
  await identityStore.initialize();
  if (identityStore.identity) {
    swarmStore.setLocalIdentity(identityStore.identity.public_key_hex);
    await networkStore.initialize();
    await networkStore.start();
    await swarmStore.initialize();
    await swarmStore.activateDefaultSwarm();
    appState = 'app';
  } else {
    appState = 'setup';
  }
});

// handleSetupComplete -- add setLocalIdentity after identity creation
async function handleSetupComplete() {
  if (identityStore.identity) {
    swarmStore.setLocalIdentity(identityStore.identity.public_key_hex);
  }
  await networkStore.start();
  await swarmStore.initialize();
  await swarmStore.activateDefaultSwarm();
  appState = 'app';
}
```

### Backend verify_creator (Already Implemented)

```rust
// commands/channel.rs L21-41 -- already checks creator identity
fn verify_creator(app: &AppHandle, swarm_id: &str) -> Result<(), String> {
    let secret_bytes = storage::load_secret_key().map_err(|e| format!("Identity error: {}", e))?;
    let signing_key = keypair::signing_key_from_bytes(&secret_bytes).map_err(|e| format!("Key error: {}", e))?;
    let local_key = keypair::public_key_to_hex(&signing_key.verifying_key());

    let doc = swarm::metadata_storage::load_metadata_doc(app, swarm_id)
        .map_err(|e| format!("Metadata error: {}", e))?
        .ok_or_else(|| "Swarm metadata not found".to_string())?;

    let creator_key = doc.get_creator_key().map_err(|e| format!("Metadata error: {}", e))?;

    if creator_key != local_key {
        return Err("Only the swarm creator can manage channels".to_string());
    }
    Ok(())
}
```

### Post-Sync Validation Pattern (New Code Needed)

```rust
// In metadata_sync.rs -- after sync_metadata_document completes
// Pseudo-code for post-sync creator validation
pub fn validate_post_sync(doc: &mut SwarmMetadataDocument) -> Result<bool, ChannelError> {
    let creator_key = doc.get_creator_key()?;
    let channels = doc.get_channels()?;

    let mut needs_correction = false;
    let mut valid_channels = HashMap::new();

    for (id, meta) in &channels {
        // Default channels are always valid
        if id == "general" || id == "voice" {
            valid_channels.insert(id.clone(), meta.clone());
            continue;
        }
        // Only keep channels created by the swarm creator
        if meta.created_by == creator_key {
            valid_channels.insert(id.clone(), meta.clone());
        } else {
            needs_correction = true;
        }
    }

    if needs_correction {
        // Reconcile with corrected channel set
        // This overwrites unauthorized changes locally
        // On next sync, our corrected state propagates
    }

    Ok(needs_correction)
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| No creator tracking | `creator_key` in CRDT metadata doc | Phase 7 | Enables creator-only permissions |
| Global context menu guard | Per-item `isCreator` prop | Phase 8 (08-01) | Non-creators get "Mark as read" |
| UI-only gating | Backend `verify_creator()` | Phase 7 | Prevents API-level bypass |

**Deprecated/outdated:**
- Phase 7's original `handleContextMenu` had `if (!swarmStore.isCreator) return;` blocking all context menus for non-creators. Phase 8 Plan 01 removed this and moved the check to `ChannelContextMenu.svelte` as a prop-based conditional. This is the current correct pattern.

## Open Questions

1. **CRDT sync filtering granularity**
   - What we know: Automerge sync applies changes atomically. Post-sync validation can check and correct unauthorized changes.
   - What's unclear: Whether overwriting unauthorized changes locally will cause sync oscillation (peer A adds unauthorized channel, peer B removes it, peer A re-adds it on next sync).
   - Recommendation: After post-sync correction, the corrected state becomes the new local truth. Since the creator's node also runs the same validation, the creator's state is authoritative. Non-creator peers will converge to the creator's state because the creator never removes their own legitimate channels. If a rogue peer keeps re-adding unauthorized channels, each sync cycle will remove them locally. This is acceptable behavior -- eventual consistency with creator authority.

2. **Migration timing relative to sync**
   - What we know: Migration runs at app startup. Sync happens when peers connect (after network starts).
   - What's unclear: If migration sets `created_by` before first sync, and the creator's CRDT doc has different `created_by` values, will they conflict?
   - Recommendation: Migration should only fill in `created_by` when it's missing (empty string or absent). It should use the CRDT doc's `creator_key` as the value. Since the creator's doc already has correct `created_by` values, sync will converge correctly.

## Sources

### Primary (HIGH confidence)
- Direct codebase analysis of:
  - `src/App.svelte` (L1-75) -- startup flow, both code paths identified
  - `src/lib/stores/swarm.svelte.ts` (L1-293) -- full store including setLocalIdentity, isCreator
  - `src/lib/stores/identity.svelte.ts` (L1-69) -- identity initialization and IdentityInfo type
  - `src/lib/tauri.ts` (L1-298) -- IdentityInfo interface with `public_key_hex` field
  - `src-tauri/src/commands/channel.rs` (L1-341) -- verify_creator, channel CRUD commands
  - `src-tauri/src/swarm/metadata_doc.rs` (L1-175) -- SwarmMetadataDocument with creator_key, channels
  - `src-tauri/src/swarm/metadata_sync.rs` (L1-138) -- sync_metadata_document function
  - `src-tauri/src/swarm/metadata_storage.rs` (L1-82) -- save/load metadata doc
  - `src-tauri/src/swarm/storage.rs` (L1-102) -- SwarmMetadata with creator_key: Option<String>
  - `src-tauri/src/commands/swarm.rs` (L1-212) -- create_swarm sets creator_key, join_swarm sets None
  - `src/lib/components/layout/ChannelList.svelte` (L1-334) -- isCreator UI gates
  - `src/lib/components/channel/ChannelContextMenu.svelte` (L1-89) -- isCreator conditional render
  - `.planning/v1.1-MILESTONE-AUDIT.md` -- root cause analysis and gap identification

### Secondary (MEDIUM confidence)
- Automerge sync protocol behavior (sync applies atomically, post-sync validation is standard pattern)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- all libraries already in use, no new dependencies
- Architecture: HIGH -- codebase fully analyzed, both frontend and backend paths traced end-to-end
- Pitfalls: HIGH -- root cause identified precisely (single missing call), edge cases mapped from actual code paths
- CRDT sync filtering: MEDIUM -- post-sync validation pattern is sound but implementation details for Automerge rollback need validation during implementation

**Research date:** 2026-02-23
**Valid until:** 2026-03-23 (stable -- bug fix in mature codebase)