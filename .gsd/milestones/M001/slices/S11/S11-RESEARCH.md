# Phase 9: Peer Moderation - Research

**Researched:** 2026-02-17
**Domain:** Local peer moderation (mute/hide/block) with persistent state, frontend view filtering, voice audio suppression
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- Three cumulative tiers: Mute -> Hide -> Block
- Mute: drops peer's voice audio (separate from "mute my mic" which is self-mute)
- Hide: suppresses peer's messages (includes Mute behavior)
- Block: fully removes peer's messages and drops voice (includes Hide + Mute)
- Cumulative model: each tier includes everything below it
- Moderation state is global by default (block once, blocked everywhere)
- Per-swarm override available (can unblock in a specific swarm)
- Right-click context menu on peer name -- both in sidebar peer list AND on message author names in chat
- All three tiers always visible in context menu (Mute Peer / Hide Peer / Block Peer)
- Active states shown with checkmarks or "Unmute"/"Unhide"/"Unblock" labels
- Mute and Hide are instant (no confirmation)
- Block requires a confirmation dialog ("Block this peer?")
- Hidden peer (Hide tier): collapsed placeholder -- "Message from hidden user" -- click to reveal temporarily
- Blocked peer (Block tier): fully removed -- no trace in chat view
- Muted peer (Mute tier): messages visible normally (only voice is affected)
- Moderated peers always visible in sidebar peer list
- Small status icon indicating moderation state (speaker-off for muted, eye-off for hidden, block icon for blocked)
- All moderation tiers suppress @mentions from the moderated peer
- No unread indicators or notifications triggered by muted/hidden/blocked peers
- Quick undo via same right-click context menu (right-click -> Unblock/Unhide/Unmute)
- Dedicated management section in settings showing all moderated peers
- Management list shows: peer name + current tier (no dates)
- From management list: can change tier (escalate/de-escalate) or remove moderation entirely
- Unblocking restores all previously hidden messages (CRDT data is untouched, filter is removed)

### Claude's Discretion

- Storage format for moderation state (contacts store extension vs separate store)
- How per-swarm overrides are stored and surfaced in UI
- Icon choices for moderation states
- Settings/management page layout and navigation
- Exact styling of collapsed placeholder messages

### Deferred Ideas (OUT OF SCOPE)

None -- discussion stayed within phase scope
</user_constraints>

## Summary

Phase 9 adds local peer moderation with three graduated tiers (Mute, Hide, Block) that control what the user sees and hears from specific peers. This is a **purely local, view-layer feature** -- CRDT data is never modified, moderation is enforced by filtering at render time (messages) and at the audio pipeline level (voice). The existing `tauri-plugin-store` pattern used by contacts and unread state is the natural storage mechanism.

The architecture decomposes into four concerns: (1) a new `moderation.svelte.ts` store holding reactive moderation state with Tauri persistence, (2) frontend view filtering in MessageList and ChatPanel to hide/collapse messages and suppress unread/mention counts, (3) voice audio suppression in the Rust `AudioMixer::mix_next_frame` to skip muted peers, and (4) UI additions -- a PeerContextMenu component (right-click on peer names in sidebar and chat), moderation status icons in PeerList, a block confirmation dialog, and a moderation management section accessible from settings.

**Primary recommendation:** Create a dedicated `moderation.json` store (separate from contacts) with a flat `Map<publicKey, ModerationEntry>` where each entry has a `tier` field and optional `swarmOverrides`. Filter at the view layer in Svelte, not in Rust backend queries, to preserve the principle that CRDT data is untouched.

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| tauri-plugin-store | 2 | Persistent JSON key-value storage for moderation state | Already used for contacts.json and unread.json; same pattern, zero new deps |
| Svelte 5 runes | 5.x | Reactive moderation store ($state, $derived) | Project standard; .svelte.ts store pattern established in contacts/chat/unread |

### Supporting

No new libraries required. All functionality is built on existing project dependencies.

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Separate moderation.json store | Extend contacts.json with moderation_tier field | Contacts store is petname-focused; adding moderation mixes concerns and complicates the Contact struct. Separate store is cleaner, cheaper to query, and avoids backward-compat issues with existing contacts. |
| Frontend-only filtering | Backend Rust filtering in get_messages | Frontend filtering preserves CRDT integrity and allows "click to reveal" for hidden messages. Backend filtering would discard data before it reaches the UI, making reveal impossible. |

**Recommendation: Separate `moderation.json` store.** Rationale:
1. Contacts store is keyed by public_key and stores petname/notes. Adding moderation would require schema migration and complicates the Contact type.
2. Moderation state has different access patterns (checked on every message render, every voice frame mix, every unread calculation).
3. Separate store can be loaded/cached independently in a dedicated Svelte store.
4. Moderation management UI in settings can operate on the store without loading contacts.

## Architecture Patterns

### Recommended Project Structure

```
src/lib/stores/
  moderation.svelte.ts         # Reactive moderation store (Svelte 5 runes)

src/lib/components/
  moderation/
    PeerContextMenu.svelte     # Right-click context menu (Mute/Hide/Block)
    BlockConfirmDialog.svelte   # Confirmation dialog for Block action
    ModerationList.svelte       # Settings management panel

src-tauri/src/
  moderation/
    mod.rs                     # ModerationTier enum, ModerationEntry struct
    storage.rs                 # tauri-plugin-store CRUD for moderation.json
  commands/
    moderation.rs              # Tauri IPC commands
```

### Pattern 1: Moderation Store (Frontend)

**What:** A Svelte 5 runes-based store that mirrors the existing contacts/unread store pattern. Loads moderation state from Tauri backend on init, provides reactive lookup functions, and persists changes through Tauri commands.

**When to use:** Every component that needs to check if a peer is moderated reads from this store.

**Example:**
```typescript
// moderation.svelte.ts
import { getModerationState, setModeration, removeModeration, type ModerationEntry } from '../tauri';

export type ModerationTier = 'mute' | 'hide' | 'block';

let entries = $state<Map<string, ModerationEntry>>(new Map());
let initialized = $state(false);

async function initialize() {
  if (initialized) return;
  const state = await getModerationState();
  entries = new Map(Object.entries(state));
  initialized = true;
}

// Effective tier considering per-swarm overrides
function getEffectiveTier(publicKey: string, swarmId?: string): ModerationTier | null {
  const entry = entries.get(publicKey);
  if (!entry) return null;
  if (swarmId && entry.swarm_overrides?.[swarmId] !== undefined) {
    return entry.swarm_overrides[swarmId]; // null means "unmoderated in this swarm"
  }
  return entry.tier;
}

function isMuted(publicKey: string, swarmId?: string): boolean {
  const tier = getEffectiveTier(publicKey, swarmId);
  return tier === 'mute' || tier === 'hide' || tier === 'block';
}

function isHidden(publicKey: string, swarmId?: string): boolean {
  const tier = getEffectiveTier(publicKey, swarmId);
  return tier === 'hide' || tier === 'block';
}

function isBlocked(publicKey: string, swarmId?: string): boolean {
  return getEffectiveTier(publicKey, swarmId) === 'block';
}

export const moderationStore = {
  get entries() { return entries; },
  get initialized() { return initialized; },
  initialize,
  getEffectiveTier,
  isMuted,
  isHidden,
  isBlocked,
  // ... setTier, removeTier, setSwarmOverride actions
};
```

**Key design notes:**
- Cumulative tier model: `isHidden` returns true for both 'hide' and 'block'. `isMuted` returns true for all three.
- `getEffectiveTier` handles per-swarm overrides transparently.
- Store follows the exact same export pattern as contacts/unread stores.

### Pattern 2: Per-Swarm Override Storage

**What:** Each moderation entry has an optional `swarm_overrides` map. A swarm override of `null` means "no moderation in this swarm" (i.e., un-moderated locally). An override of a different tier means "use this tier in this swarm instead."

**Example data structure:**
```json
{
  "abc123...pubkey": {
    "tier": "block",
    "swarm_overrides": {
      "swarm-id-xyz": null
    }
  }
}
```
This means: peer is blocked globally, but in swarm `swarm-id-xyz` they are unblocked.

**UI surfacing:** In the PeerContextMenu, when inside a swarm, show an additional "Override for this swarm" sub-section if a global moderation exists. The management list in settings shows global tier + any swarm-specific overrides.

### Pattern 3: Message View Filtering

**What:** Filter messages at the Svelte render layer, not at the data fetch layer. `chatStore.messages` remains the full CRDT-sourced list. The component applies moderation as a derived/computed filter.

**When to use:** In MessageList.svelte and ChatPanel.svelte.

**Example:**
```svelte
<!-- MessageList.svelte -->
{#each messages as msg, i}
  {#if moderationStore.isBlocked(msg.sender_key, swarmId)}
    <!-- Blocked: completely hidden, no DOM element -->
  {:else if moderationStore.isHidden(msg.sender_key, swarmId)}
    <div class="message-row message-hidden">
      <span class="hidden-placeholder" onclick={() => revealMessage(msg.id)}>
        Message from hidden user
      </span>
    </div>
  {:else}
    <!-- Normal message rendering -->
  {/if}
{/each}
```

**Key insight:** The "click to reveal" for hidden messages requires a local `revealedIds` set ($state). This is session-only state (not persisted). Blocked messages have no reveal mechanism -- they are fully suppressed.

### Pattern 4: Voice Audio Suppression

**What:** When mixing audio in the Rust `AudioMixer::mix_next_frame`, skip peers whose audio should be muted. This requires passing a set of muted peer IDs to the mixer.

**Two approaches:**
1. **Mixer-level skip set:** Add a `muted_peers: HashSet<PeerId>` to AudioMixer. In `mix_next_frame`, skip jitter buffers for muted peers. Simple, minimal changes.
2. **Feed-level skip:** In the receive-and-decode task in `VoiceSession`, check moderation before calling `mixer.feed_frame()`. Avoids buffering audio that will never be played.

**Recommendation:** Approach 1 (mixer-level skip set) is simpler and safer. The jitter buffer memory cost is negligible (a few frames per peer), and the skip in mix is a single `HashSet::contains` check. Approach 2 requires plumbing moderation state into the async receive task, which is more invasive.

**Implementation sketch:**
```rust
// AudioMixer
pub struct AudioMixer {
    peer_buffers: HashMap<PeerId, JitterBuffer>,
    muted_peers: HashSet<PeerId>,
    max_participants: usize,
}

impl AudioMixer {
    pub fn mute_peer(&mut self, peer_id: PeerId) {
        self.muted_peers.insert(peer_id);
    }

    pub fn unmute_peer(&mut self, peer_id: &PeerId) {
        self.muted_peers.remove(peer_id);
    }

    pub fn mix_next_frame(&mut self, frame_size: usize) -> Vec<f32> {
        // ... existing code, but skip muted peers:
        for (peer_id, buffer) in self.peer_buffers.iter_mut() {
            if self.muted_peers.contains(peer_id) {
                buffer.get_frame(); // drain but discard
                continue;
            }
            // ... mix as normal
        }
    }
}
```

**IPC flow:** Frontend calls a new `mute_peer_voice` / `unmute_peer_voice` Tauri command. The command acquires the VoiceSession mutex, then calls `mixer.mute_peer()`. This keeps voice mute state in Rust (where audio processing happens) while moderation state lives in the frontend store.

**Alternative:** The frontend moderation store could call this command reactively whenever moderation state changes for a peer who is a voice participant. An `$effect` in the moderation store or in VoicePanel could synchronize.

### Pattern 5: Right-Click Context Menu

**What:** A reusable PeerContextMenu component triggered by `oncontextmenu` on peer names in both PeerList and MessageList.

**Svelte 5 event binding:**
```svelte
<span class="peer-name" oncontextmenu={(e) => { e.preventDefault(); openMenu(peer.peer_id, e.clientX, e.clientY); }}>
  {peerName}
</span>
```

**Key design:** The existing `ChannelContextMenu.svelte` establishes the pattern: fixed-position div with clamped coordinates, `<svelte:window onclick={onClose} />` for click-away dismissal, and callback props for actions. PeerContextMenu follows this exact pattern but with moderation-specific menu items.

### Pattern 6: Unread/Mention Suppression

**What:** The unread store's `recalculate` function must ignore messages from moderated peers when counting unreads and checking mentions.

**Where:** In `unread.svelte.ts`, the `recalculate` function fetches messages and checks for new unseen messages + mentions. It needs to filter out messages from peers who are hidden or blocked (any tier that suppresses messages).

**Example:**
```typescript
async function recalculate(swarmId: string, channelId: string) {
  const messages = await getMessages(swarmId, channelId);
  // Filter out messages from hidden/blocked peers
  const visibleMessages = messages.filter(m =>
    !moderationStore.isHidden(m.sender_key, swarmId)
  );
  // ... count unreads against visibleMessages instead of messages
}
```

**Important subtlety:** For the **Mute** tier (voice-only), messages are still visible, so muted peers' messages should still count toward unreads. Only Hide and Block tiers suppress unread counts. However, all three tiers suppress @mentions from that peer (per user decision).

### Anti-Patterns to Avoid

- **Modifying CRDT state:** Never delete/filter messages in Automerge documents. Moderation is a local view filter only.
- **Backend-level message filtering:** Don't add moderation filtering to `get_messages` Tauri command. The frontend needs the full list for "click to reveal" and to properly handle unblock (messages reappear instantly).
- **Storing moderation in contacts.json:** Mixes concerns. Contact is about identity (petname, notes). Moderation is about visibility control. Different access patterns, different lifecycle.
- **PeerId string mismatch:** The frontend uses hex-encoded Ed25519 public keys (from `sender_key`), while voice uses libp2p `PeerId`. The voice mute commands need to convert between these formats. Use the same hex-to-PeerId conversion already present in the network module.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Persistent key-value storage | Custom file I/O | `tauri-plugin-store` (already in use) | Handles serialization, file locking, auto-save, cross-platform paths |
| Context menu positioning | Manual viewport math | Same clamping pattern from ChannelContextMenu | Already solved; Math.min with window dimensions |
| Reactive state management | Custom event emitter | Svelte 5 `$state`/`$derived` runes in `.svelte.ts` | Project standard, fine-grained reactivity, no boilerplate |

**Key insight:** This phase requires zero new dependencies. Every capability needed (persistent storage, reactive state, audio mixing, CRDT message access) already exists in the codebase. The work is integration and wiring, not new infrastructure.

## Common Pitfalls

### Pitfall 1: Tier Cumulation Logic Errors
**What goes wrong:** Implementing tiers as independent flags (isMuted, isHidden, isBlocked as separate booleans) leads to inconsistent states where a peer is blocked but not hidden.
**Why it happens:** Natural instinct is to model each tier separately.
**How to avoid:** Store a single `tier` enum value. Derive boolean helpers cumulatively: `isHidden = tier === 'hide' || tier === 'block'`. Never store multiple independent booleans.
**Warning signs:** Tests where you can set `hidden=false, blocked=true`.

### Pitfall 2: Unread Count Drift After Moderation Change
**What goes wrong:** User blocks a peer, unread count doesn't update. Or user unblocks, old messages don't retroactively create unreads.
**Why it happens:** Unread recalculation only triggers on new messages, not on moderation state changes.
**How to avoid:** When moderation state changes, trigger unread recalculation for all channels where the moderated peer has messages. The unread store's `recalculate` can be called explicitly.
**Warning signs:** Stale unread badges after blocking/unblocking.

### Pitfall 3: Voice Mute Desync
**What goes wrong:** User blocks a peer in the moderation store but voice audio continues playing.
**Why it happens:** Voice mute state lives in Rust (AudioMixer), moderation state lives in Svelte (moderation store). No automatic sync.
**How to avoid:** When moderation tier changes to any level >= mute AND the peer is a current voice participant, call the `mute_peer_voice` Tauri command. When tier is removed, call `unmute_peer_voice`. Tie this to the moderation store's setter.
**Warning signs:** Can still hear a "blocked" peer talking.

### Pitfall 4: Context Menu on Own User
**What goes wrong:** User right-clicks their own name in the peer list or message list and sees moderation options.
**Why it happens:** Context menu doesn't filter out the current user.
**How to avoid:** Check `msg.sender_key !== currentUserKey` before showing the moderation menu.
**Warning signs:** User can mute/block themselves.

### Pitfall 5: Per-Swarm Override Confusion
**What goes wrong:** User overrides moderation for a swarm, switches swarms, and the override bleeds into other swarms.
**Why it happens:** Override lookup doesn't correctly scope to the active swarm.
**How to avoid:** Always pass `swarmId` to `getEffectiveTier()`. The current active swarm ID comes from `swarmStore.activeSwarm.id`.
**Warning signs:** Moderation state appears different than expected when switching swarms.

### Pitfall 6: Message Count Mismatch with Unread Tracking
**What goes wrong:** `markRead` stores `totalSeen` based on `chatStore.messages.length`, but unread recalculation filters by moderation. The counts diverge.
**Why it happens:** `totalSeen` counts ALL messages (including from moderated peers), but visible message count differs.
**How to avoid:** Keep `totalSeen` based on the FULL message list (unfiltered). Only apply moderation filtering when CHECKING for unreads (in the `recalculate` comparison). This way, `totalSeen` remains a stable watermark into the CRDT message array, and moderation changes don't corrupt the unread tracking state.
**Warning signs:** Phantom unreads or missing unreads after moderation changes.

## Code Examples

### Backend: Moderation Storage (Rust)

```rust
// src-tauri/src/moderation/mod.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModerationTier {
    Mute,
    Hide,
    Block,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationEntry {
    pub tier: ModerationTier,
    /// Per-swarm overrides. Value of None means "no moderation in this swarm".
    /// Value of Some(tier) means "use this tier in this swarm".
    #[serde(default)]
    pub swarm_overrides: HashMap<String, Option<ModerationTier>>,
}
```

```rust
// src-tauri/src/moderation/storage.rs
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use super::ModerationEntry;
use std::collections::HashMap;

pub fn get_moderation_state(app: &AppHandle) -> Result<HashMap<String, ModerationEntry>, String> {
    let store = app.store("moderation.json")
        .map_err(|e| format!("Failed to access moderation store: {}", e))?;

    let mut result = HashMap::new();
    for (key, value) in store.entries() {
        let entry: ModerationEntry = serde_json::from_value(value.clone())
            .map_err(|e| format!("Failed to deserialize moderation entry: {}", e))?;
        result.insert(key.clone(), entry);
    }
    Ok(result)
}

pub fn set_moderation(app: &AppHandle, public_key: &str, entry: &ModerationEntry) -> Result<(), String> {
    let store = app.store("moderation.json")
        .map_err(|e| format!("Failed to access moderation store: {}", e))?;

    store.set(public_key.to_string(), serde_json::to_value(entry).unwrap());
    store.save().map_err(|e| format!("Failed to save moderation store: {}", e))?;
    Ok(())
}

pub fn remove_moderation(app: &AppHandle, public_key: &str) -> Result<(), String> {
    let store = app.store("moderation.json")
        .map_err(|e| format!("Failed to access moderation store: {}", e))?;

    store.delete(public_key);
    store.save().map_err(|e| format!("Failed to save moderation store: {}", e))?;
    Ok(())
}
```

### Frontend: Context Menu Trigger on Peer Name

```svelte
<!-- In PeerList.svelte -->
<span
  class="peer-id"
  oncontextmenu={(e) => {
    e.preventDefault();
    if (peer.peer_id !== currentUserKey) {
      contextMenu = { publicKey: peer.peer_id, x: e.clientX, y: e.clientY };
    }
  }}
>
  {getPeerDisplayName(peer.peer_id)}
</span>

{#if contextMenu}
  <PeerContextMenu
    x={contextMenu.x}
    y={contextMenu.y}
    publicKey={contextMenu.publicKey}
    currentTier={moderationStore.getEffectiveTier(contextMenu.publicKey, activeSwarmId)}
    onMute={() => handleSetTier(contextMenu.publicKey, 'mute')}
    onHide={() => handleSetTier(contextMenu.publicKey, 'hide')}
    onBlock={() => showBlockConfirm = contextMenu.publicKey}
    onRemove={() => handleRemoveTier(contextMenu.publicKey)}
    onClose={() => contextMenu = null}
  />
{/if}
```

### Frontend: Hidden Message Placeholder

```svelte
<!-- In MessageList.svelte, inside the message loop -->
{#if moderationStore.isBlocked(msg.sender_key, swarmId)}
  <!-- Blocked: fully hidden, zero DOM footprint -->
{:else if moderationStore.isHidden(msg.sender_key, swarmId) && !revealedIds.has(msg.id)}
  <div class="message-row message-hidden-placeholder">
    <span class="msg-time">{formatTime(msg.timestamp)}</span>
    <button class="hidden-msg-btn" onclick={() => revealMessage(msg.id)}>
      Message from hidden user
    </button>
  </div>
{:else}
  <!-- Normal rendering (including temporarily revealed hidden messages) -->
{/if}
```

### Frontend: Moderation Status Icons in PeerList

```svelte
<!-- In PeerList.svelte, after peer name -->
{#if moderationStore.isMuted(peer.peer_id) && !moderationStore.isHidden(peer.peer_id)}
  <span class="mod-icon" title="Muted">[M]</span>
{:else if moderationStore.isHidden(peer.peer_id) && !moderationStore.isBlocked(peer.peer_id)}
  <span class="mod-icon" title="Hidden">[H]</span>
{:else if moderationStore.isBlocked(peer.peer_id)}
  <span class="mod-icon mod-icon--blocked" title="Blocked">[B]</span>
{/if}
```

Note: The exact icon representation (text brackets vs. Unicode symbols vs. SVG) is at Claude's discretion. The monospace terminal aesthetic of the app suggests text-based indicators like `[M]` / `[H]` / `[B]` or Unicode characters.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Svelte 4 stores (writable/derived) | Svelte 5 runes ($state/$derived) in .svelte.ts | Svelte 5 (2024) | All stores in this project use runes pattern |
| on:contextmenu directive | oncontextmenu attribute | Svelte 5 (2024) | Event binding syntax change |

**Deprecated/outdated:**
- Svelte 4 `createEventDispatcher` and `on:event` syntax: replaced by callback props (`onFoo`) in Svelte 5

## Open Questions

1. **PeerId <-> PublicKey Mapping for Voice Mute**
   - What we know: The frontend identifies peers by hex-encoded Ed25519 public keys (`sender_key`). The Rust voice system uses libp2p `PeerId`. The network module already converts between these.
   - What's unclear: Whether the existing conversion is easily accessible from the voice commands, or if a new lookup is needed.
   - Recommendation: During implementation, check how `join_voice` resolves peer IDs. The conversion likely already exists in `NetworkService::get_peers()`. If not, add a `public_key_to_peer_id` helper.

2. **Mention Suppression Scope**
   - What we know: User decided "all moderation tiers suppress @mentions from the moderated peer."
   - What's unclear: Does this mean (a) @mentions from a muted peer don't trigger mention highlights/indicators, or (b) the mention text itself is hidden?
   - Recommendation: Interpret as (a) -- the mention highlight and unread-mention indicator are suppressed, but if the message is visible (mute tier), the @mention text renders normally without special styling. This is the least surprising behavior.

3. **Settings Navigation Entry Point**
   - What we know: There needs to be a moderation management section in settings.
   - What's unclear: There is no formal "Settings" page yet. The existing SwarmSettings dialog is swarm-specific.
   - Recommendation: Add a "MODERATION" section in the Sidebar (below CONTACTS), similar to how CONTACTS has a "VIEW CONTACTS" button. Clicking opens a ModerationList modal dialog following the same fixed-position pattern as ContactsList. This avoids building a full settings page.

## Sources

### Primary (HIGH confidence)
- `/tauri-apps/plugins-workspace` (Context7) - tauri-plugin-store v2 Rust API: store.get, store.set, store.save, store.delete, store.entries
- `/llmstxt/svelte_dev_llms-small_txt` (Context7) - Svelte 5 runes ($state, $derived, $effect), event handling (oncontextmenu), .svelte.ts patterns
- Codebase analysis (HIGH) - contacts.svelte.ts, contacts/storage.rs, unread.rs, chat.svelte.ts, unread.svelte.ts, voice/mixer.rs, voice/session.rs, ChannelContextMenu.svelte, MessageList.svelte, PeerList.svelte, ChatPanel.svelte, ContactsList.svelte

### Secondary (MEDIUM confidence)
- None needed -- all patterns derived from existing codebase

### Tertiary (LOW confidence)
- None

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Zero new dependencies; all patterns already established in codebase
- Architecture: HIGH - Direct extension of existing store/component/command patterns; moderation is a well-understood problem domain
- Pitfalls: HIGH - Identified from careful analysis of data flow between stores (unread, chat, voice, moderation)

**Research date:** 2026-02-17
**Valid until:** 2026-03-17 (stable; no external dependencies to change)