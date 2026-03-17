# Phase 3: Invitation System - Research

**Researched:** 2026-02-14
**Domain:** Swarm isolation with libp2p PSK, custom URI schemes, clipboard integration, and multi-swarm state management
**Confidence:** HIGH

## Summary

Phase 3 implements invitation-based swarm formation using libp2p's Pre-Shared Key (PSK) protocol for network isolation, Tauri's deep linking plugin for `aether://` URI scheme handling, and the Store plugin for persistent swarm/channel state. The core mechanism: generate a random 32-byte swarm key, encode it in base16 within an `aether://<hex-key>` URI, distribute via clipboard, and use the PSK to isolate libp2p network traffic.

The critical integration point is applying PSK to transports BEFORE building the swarm. libp2p's pnet layer wraps transports with XSalsa20 encryption, ensuring nodes can only discover and communicate with peers sharing the same 32-byte key. Phase 2's existing network service must be refactored to support multiple swarms: either multiple swarm instances (complex) or dynamic swarm restart with different PSKs (simpler for MVP).

The invitation flow: User 1 generates swarm key → encodes as `aether://<hex>` → copies to clipboard → shares externally (Signal/email) → User 2 opens URI (deep link) → Aether receives URI via plugin → extracts key → joins swarm with PSK. All swarms persist in JSON store indexed by swarm ID (hash of PSK), with channel lists stored per-swarm. Frontend uses Svelte 5 runes-based store pattern (`.svelte.ts`) for reactive swarm/channel state.

**Primary recommendation:** Use `libp2p-pnet` 0.24+ with `PreSharedKey::new([u8; 32])` for PSK generation, `rand::rngs::OsRng` for cryptographic randomness, `hex` crate for URI encoding/decoding, `tauri-plugin-deep-link` for `aether://` scheme registration, `tauri-plugin-clipboard-manager` for copy operations, and `tauri-plugin-store` for swarm/channel persistence. Refactor `NetworkService` to support swarm switching (restart with new PSK) rather than multiple concurrent swarms for MVP simplicity.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| libp2p-pnet | 0.24+ (via libp2p 0.56) | Pre-Shared Key network isolation | Only standard mechanism for private libp2p networks, proven in IPFS private swarms |
| tauri-plugin-deep-link | 2.0.0 | Custom URI scheme (`aether://`) | Official Tauri v2 plugin for deep linking, handles OS registration and URL events |
| tauri-plugin-clipboard-manager | 2.1.11+ | Copy Secret Code to clipboard | Official Tauri v2 plugin, cross-platform (macOS/Windows/Linux) |
| tauri-plugin-store | 2.x | Persistent swarm/channel JSON storage | Official Tauri v2 key-value store, async, auto-save, JSON format |
| hex | 0.4 (already in Cargo.toml) | Base16 encoding for swarm keys | Standard Rust hex encoding, RFC 4648 compliant |
| rand | 0.8 (already in deps) | Cryptographic random key generation | De facto CSPRNG library in Rust, `OsRng` is platform secure |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| serde | 1.x (already in deps) | Serialize swarm/channel state | Always - for JSON store integration |
| serde_json | 1.x (already in deps) | JSON persistence format | Always - store plugin uses JSON |
| base64 | 0.22 (already in deps) | Alternative encoding (optional) | Only if hex proves problematic, base64 is URL-safe alternative |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| libp2p PSK | Custom DHT filtering | PSK provides cryptographic isolation + encryption, DHT filtering only provides discovery filtering (peers could still connect) |
| Deep link plugin | Manual URI scheme registration | Plugin handles all OS differences (macOS `.plist`, Windows registry, Linux `.desktop`), manual is error-prone |
| Store plugin | SQLite database | Store plugin simpler for key-value data, SQLite overkill for swarm metadata, adds migration complexity |
| Hex encoding | Base64 or Base32 | Hex is standard for PSK (libp2p compatibility), base64 more compact but harder to visually verify |
| Single swarm restart | Multiple concurrent swarms | Multiple swarms require separate ports, connection managers, complex state - restart simpler for MVP |

**Installation:**
```bash
# Add to src-tauri/Cargo.toml
[dependencies]
tauri-plugin-deep-link = "2.0.0"
tauri-plugin-clipboard-manager = "2.1"
tauri-plugin-store = "2"

# Frontend (npm/pnpm)
npm install @tauri-apps/plugin-deep-link @tauri-apps/plugin-clipboard-manager @tauri-apps/plugin-store
```

## Architecture Patterns

### Recommended Project Structure
```
src-tauri/src/
├── swarm/                    # NEW: Swarm management
│   ├── mod.rs                # Public API, SwarmManager struct
│   ├── key.rs                # PSK generation, encoding, validation
│   ├── storage.rs            # Swarm/channel persistence via Store plugin
│   └── uri.rs                # aether:// URI parsing/validation
├── network/                  # EXISTING: Modified for PSK support
│   ├── mod.rs                # Add PSK parameter to start()
│   ├── swarm.rs              # Modify build_swarm() to accept PSK
│   └── ...                   # Other files unchanged
├── commands/
│   └── swarm.rs              # NEW: Tauri commands for swarm operations
└── ...

src/lib/
├── stores/
│   ├── identity.svelte.ts    # EXISTING
│   └── swarm.svelte.ts       # NEW: Multi-swarm state management
└── components/
    ├── swarm/                # NEW: Swarm UI components
    │   ├── SwarmSelector.svelte
    │   ├── InviteDialog.svelte
    │   └── JoinDialog.svelte
    └── layout/
        └── ChannelList.svelte # MODIFIED: Show channels for active swarm
```

### Pattern 1: Generate and Encode Swarm Key
**What:** Create cryptographically secure 32-byte PSK and encode as `aether://<hex>`
**When to use:** User clicks "Create Swarm" or "Generate Invite Code"
**Example:**
```rust
// Source: https://docs.rs/libp2p-pnet/latest/libp2p_pnet/ + https://docs.rs/rand/latest/rand/
use libp2p::pnet::PreSharedKey;
use rand::rngs::OsRng;
use rand::RngCore;

pub fn generate_swarm_key() -> PreSharedKey {
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    PreSharedKey::new(key_bytes)
}

pub fn encode_secret_code(key: &PreSharedKey) -> String {
    // PSK stores 32 bytes internally
    let key_bytes = key.as_ref(); // &[u8; 32]
    format!("aether://{}", hex::encode(key_bytes))
}

pub fn decode_secret_code(uri: &str) -> Result<PreSharedKey, SwarmError> {
    // Validate aether:// scheme
    if !uri.starts_with("aether://") {
        return Err(SwarmError::InvalidUri("Missing aether:// scheme"));
    }

    let hex_part = uri.trim_start_matches("aether://");
    let key_bytes = hex::decode(hex_part)
        .map_err(|_| SwarmError::InvalidUri("Invalid hex encoding"))?;

    if key_bytes.len() != 32 {
        return Err(SwarmError::InvalidUri("Key must be 32 bytes"));
    }

    let mut arr = [0u8; 32];
    arr.copy_from_slice(&key_bytes);
    Ok(PreSharedKey::new(arr))
}
```

### Pattern 2: Apply PSK to Swarm Transports
**What:** Configure PnetConfig and apply to all transports before building swarm
**When to use:** When starting network with a specific swarm key
**Example:**
```rust
// Source: https://docs.rs/libp2p/latest/libp2p/pnet/ + Phase 2 research
use libp2p::{SwarmBuilder, pnet::{PreSharedKey, PnetConfig}};

pub fn build_swarm_with_psk(
    keypair: libp2p::identity::Keypair,
    psk: PreSharedKey,
) -> Result<Swarm<AetherBehaviour>, NetworkError> {
    let peer_id = PeerId::from_public_key(&keypair.public());
    let pnet_config = PnetConfig::new(psk);

    // CRITICAL: PSK must be applied BEFORE other transport layers
    let swarm = SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        // Apply PSK to TCP transport
        .with_tcp(
            tcp::Config::default().port_reuse(true),
            |tcp| pnet_config.apply(tcp), // Wrap transport with PSK layer
            noise::Config::new,
            yamux::Config::default,
        )?
        // Apply PSK to QUIC transport
        .with_quic_config(|quic| pnet_config.apply(quic))
        .with_relay_client(noise::Config::new, yamux::Config::default)?
        .with_behaviour(|key, relay_client| {
            // Behaviour setup unchanged from Phase 2
            Ok(AetherBehaviour { /* ... */ })
        })?
        .build();

    Ok(swarm)
}
```

### Pattern 3: Register and Handle Deep Links
**What:** Register `aether://` scheme and handle incoming URIs
**When to use:** App startup (register) and when user opens Secret Code link
**Example:**
```rust
// Source: https://v2.tauri.app/plugin/deep-linking/
// In src-tauri/src/lib.rs

use tauri_plugin_deep_link;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            #[cfg(desktop)]
            app.deep_link().register("aether")?;

            app.deep_link().on_open_url(|event| {
                for url in event.urls() {
                    println!("Received deep link: {}", url);
                    // Emit to frontend to trigger join flow
                    app.emit("deep-link-received", url).ok();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

```typescript
// Frontend handler (src/lib/stores/swarm.svelte.ts)
import { listen } from '@tauri-apps/api/event';

async function initDeepLinkHandler() {
  await listen('deep-link-received', (event) => {
    const uri = event.payload as string;
    // Trigger join swarm dialog with pre-filled URI
    joinSwarmUri.set(uri);
    showJoinDialog.set(true);
  });
}
```

### Pattern 4: Clipboard Copy Operation
**What:** Copy Secret Code to system clipboard with one click
**When to use:** User clicks "Copy Invite Code" button
**Example:**
```typescript
// Source: https://v2.tauri.app/plugin/clipboard/
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

async function copySecretCode(code: string) {
  try {
    await writeText(code);
    // Show toast notification: "Invite code copied!"
    showToast('Invite code copied to clipboard');
  } catch (err) {
    console.error('Failed to copy to clipboard:', err);
    showToast('Failed to copy code', 'error');
  }
}
```

### Pattern 5: Persist Swarm and Channel State
**What:** Store swarm metadata (ID, name, PSK) and channel list per swarm
**When to use:** After creating/joining swarm, creating channels
**Example:**
```rust
// Source: https://v2.tauri.app/plugin/store/
use tauri_plugin_store::StoreExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SwarmMetadata {
    id: String,           // Hash of PSK for indexing
    name: String,         // User-provided name
    psk_hex: String,      // Hex-encoded PSK (for restart)
    created_at: i64,
    channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize)]
struct Channel {
    id: String,
    name: String,
}

#[tauri::command]
async fn save_swarm(app: AppHandle, metadata: SwarmMetadata) -> Result<(), String> {
    let store = app.store("swarms.json")
        .map_err(|e| e.to_string())?;

    // Store swarm by ID
    store.set(metadata.id.clone(), serde_json::to_value(&metadata).unwrap());
    store.save().await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn list_swarms(app: AppHandle) -> Result<Vec<SwarmMetadata>, String> {
    let store = app.store("swarms.json")
        .map_err(|e| e.to_string())?;

    let entries = store.entries().await
        .map_err(|e| e.to_string())?;

    let swarms: Vec<SwarmMetadata> = entries.into_iter()
        .filter_map(|(_key, value)| serde_json::from_value(value).ok())
        .collect();

    Ok(swarms)
}
```

### Pattern 6: Svelte 5 Multi-Swarm State Management
**What:** Reactive store for active swarm, swarm list, and channel selection
**When to use:** Frontend needs to track which swarm is active and its channels
**Example:**
```typescript
// Source: https://svelte.dev/docs/svelte/stores + Project's identity.svelte.ts pattern
// src/lib/stores/swarm.svelte.ts

import { listSwarms, switchSwarm, type SwarmMetadata } from '../tauri';

let swarms = $state<SwarmMetadata[]>([]);
let activeSwarm = $state<SwarmMetadata | null>(null);
let loading = $state(false);
let error = $state<string | null>(null);

async function initialize() {
  loading = true;
  error = null;

  try {
    swarms = await listSwarms();
    // Auto-select first swarm if available
    if (swarms.length > 0 && !activeSwarm) {
      await selectSwarm(swarms[0].id);
    }
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to load swarms';
  } finally {
    loading = false;
  }
}

async function selectSwarm(swarmId: string) {
  loading = true;
  error = null;

  try {
    // Find swarm metadata
    const swarm = swarms.find(s => s.id === swarmId);
    if (!swarm) throw new Error('Swarm not found');

    // Trigger backend to restart network with this swarm's PSK
    await switchSwarm(swarmId);

    activeSwarm = swarm;
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to switch swarm';
    throw err;
  } finally {
    loading = false;
  }
}

export const swarmStore = {
  get swarms() { return swarms; },
  get activeSwarm() { return activeSwarm; },
  get loading() { return loading; },
  get error() { return error; },
  initialize,
  selectSwarm,
};
```

### Anti-Patterns to Avoid
- **Storing PSK in plain config files**: PSKs are secrets. Store hex-encoded in Tauri Store (app-local JSON), not version-controlled config files.
- **Not validating deep link URIs**: Always validate `aether://` scheme and hex length (64 chars = 32 bytes) before decoding. Malformed URIs could crash or expose errors.
- **Multiple concurrent swarms for MVP**: Running multiple libp2p swarms simultaneously requires separate ports, peer IDs, connection managers. For v1, restart network service when switching swarms (simpler).
- **Reusing identity keypair as swarm key**: Ed25519 identity keypair (Phase 1) is NOT the swarm PSK. PSK is a separate random 32-byte value for network isolation.
- **Not handling deep link race condition**: App might receive deep link before UI is ready. Queue incoming URIs and process after app initialization.
- **Clipboard without user feedback**: Always show toast/notification after copy operation (success or failure) per UX best practices.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Cryptographic random generation | `rand::thread_rng()` or timestamp-based | `rand::rngs::OsRng` | OsRng uses OS CSPRNG (getrandom syscall), thread_rng is not guaranteed cryptographically secure |
| PSK network isolation | Custom DHT filtering or peer allowlist | libp2p-pnet PSK | PSK provides cryptographic guarantee at transport layer, custom filtering is bypassable |
| URI scheme registration | Manual OS registry/plist editing | tauri-plugin-deep-link | Plugin handles all OS differences, updates, and security considerations |
| Clipboard integration | Shell commands (`pbcopy`, `xclip`, `clip.exe`) | tauri-plugin-clipboard-manager | Cross-platform abstraction, handles permissions, no shell injection risk |
| Swarm metadata persistence | Custom JSON file I/O | tauri-plugin-store | Auto-save, atomic writes, resource table management, concurrent access safety |
| Hex encoding/decoding | String manipulation or byte loops | `hex` crate | Handles padding, case-insensitivity, RFC 4648 compliance, edge cases |

**Key insight:** Phase 3 touches security-critical areas (random key generation, URI handling, persistent secrets). Using battle-tested libraries is essential. Custom implementations risk subtle vulnerabilities (weak RNG, injection attacks, file corruption).

## Common Pitfalls

### Pitfall 1: Not Using Cryptographically Secure RNG
**What goes wrong:** Swarm keys generated with weak RNG are predictable, attackers can enumerate and join swarms
**Why it happens:** Using `rand::thread_rng()` or timestamp-based generation instead of `OsRng`
**How to avoid:** Always use `rand::rngs::OsRng` for swarm key generation. Never use `thread_rng()` for secrets.
**Warning signs:** Security audit flags weak randomness, swarms can be brute-forced

### Pitfall 2: PSK Applied After Transport Setup
**What goes wrong:** Swarm builds successfully but PSK is ignored, nodes connect to public network
**Why it happens:** PSK must wrap transport layer (encryption before noise/yamux), not applied as behaviour
**How to avoid:** Use `pnet_config.apply(transport)` in SwarmBuilder, BEFORE noise/yamux configuration
**Warning signs:** Nodes with different PSKs can still connect, no isolation

### Pitfall 3: Deep Link Handler Not Registered Before URL Event
**What goes wrong:** User opens `aether://` link, OS launches app, but handler not yet registered - event lost
**Why it happens:** Race condition between app startup and deep link event
**How to avoid:** Register deep link handler in `setup()` hook (before window created), queue events until UI ready
**Warning signs:** Deep links work inconsistently, especially on first launch

### Pitfall 4: Swarm ID Collision (Hash-Based)
**What goes wrong:** Two swarms with different PSKs hash to same ID, overwrite each other in store
**Why it happens:** Using weak hash (first 8 bytes) or truncated hash for swarm ID
**How to avoid:** Use full SHA256 hash of PSK as swarm ID, or generate unique UUID and store PSK separately
**Warning signs:** Swarms disappear from list, wrong channels shown

### Pitfall 5: Not Stopping Network Before PSK Switch
**What goes wrong:** Attempt to change PSK on running swarm, connections fail, state corruption
**Why it happens:** PSK is applied at swarm creation time, cannot be changed on running swarm
**How to avoid:** Stop network service, rebuild swarm with new PSK, restart - treat as full network restart
**Warning signs:** Peers show as offline after swarm switch, connection errors in logs

### Pitfall 6: URI Validation Bypass
**What goes wrong:** Malicious URI `aether://../../../etc/passwd` or `aether://<script>alert(1)</script>` causes path traversal or injection
**Why it happens:** Not validating URI format, hex length, or special characters
**How to avoid:** Strict validation: check `aether://` prefix, exactly 64 hex chars, decode to 32 bytes, reject otherwise
**Warning signs:** URI parser crashes, unexpected file access, XSS in displayed codes

### Pitfall 7: PSK Stored in Git-Committed Config
**What goes wrong:** Swarm keys leaked in version control, anyone can join private swarms
**Why it happens:** Storing swarms.json or PSK in `src-tauri/tauri.conf.json` or similar tracked files
**How to avoid:** Always use Tauri Store plugin (writes to app data directory, not project), add `swarms.json` to `.gitignore`
**Warning signs:** Swarm keys visible in git history, unauthorized peers joining

## Code Examples

Verified patterns from official sources:

### Complete Swarm Key Generation and URI Encoding
```rust
// Source: https://docs.rs/rand/latest/rand/ + https://docs.rs/hex/latest/hex/
use rand::rngs::OsRng;
use rand::RngCore;
use libp2p::pnet::PreSharedKey;

pub struct SwarmKey {
    psk: PreSharedKey,
}

impl SwarmKey {
    pub fn generate() -> Self {
        let mut key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut key_bytes);
        Self {
            psk: PreSharedKey::new(key_bytes),
        }
    }

    pub fn from_uri(uri: &str) -> Result<Self, String> {
        if !uri.starts_with("aether://") {
            return Err("Invalid URI scheme".to_string());
        }

        let hex_part = uri.trim_start_matches("aether://");

        if hex_part.len() != 64 {
            return Err("Invalid key length (expected 64 hex chars)".to_string());
        }

        let key_bytes = hex::decode(hex_part)
            .map_err(|e| format!("Invalid hex encoding: {}", e))?;

        let mut arr = [0u8; 32];
        arr.copy_from_slice(&key_bytes);

        Ok(Self {
            psk: PreSharedKey::new(arr),
        })
    }

    pub fn to_uri(&self) -> String {
        let key_bytes = self.psk.as_ref();
        format!("aether://{}", hex::encode(key_bytes))
    }

    pub fn fingerprint(&self) -> String {
        self.psk.fingerprint().to_string()
    }

    pub fn inner(&self) -> PreSharedKey {
        self.psk.clone()
    }
}
```

### Tauri Command: Create Swarm
```rust
// Source: Integration of libp2p-pnet + tauri-plugin-store + tauri commands
use tauri::{command, AppHandle, State};
use tauri_plugin_store::StoreExt;
use sha2::{Sha256, Digest};

#[command]
async fn create_swarm(
    app: AppHandle,
    name: String,
) -> Result<String, String> {
    // Generate swarm key
    let swarm_key = SwarmKey::generate();
    let uri = swarm_key.to_uri();

    // Generate swarm ID (SHA256 of PSK)
    let mut hasher = Sha256::new();
    hasher.update(swarm_key.inner().as_ref());
    let swarm_id = hex::encode(hasher.finalize());

    // Create swarm metadata
    let metadata = SwarmMetadata {
        id: swarm_id.clone(),
        name,
        psk_hex: uri.clone(),
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        channels: vec![
            Channel {
                id: "general".to_string(),
                name: "General".to_string(),
            }
        ],
    };

    // Persist to store
    let store = app.store("swarms.json")
        .map_err(|e| e.to_string())?;

    store.set(swarm_id, serde_json::to_value(&metadata).unwrap());
    store.save().await.map_err(|e| e.to_string())?;

    Ok(uri)
}
```

### Tauri Command: Join Swarm
```rust
#[command]
async fn join_swarm(
    app: AppHandle,
    uri: String,
    name: String,
) -> Result<String, String> {
    // Validate and decode URI
    let swarm_key = SwarmKey::from_uri(&uri)
        .map_err(|e| format!("Invalid invite code: {}", e))?;

    // Generate swarm ID
    let mut hasher = Sha256::new();
    hasher.update(swarm_key.inner().as_ref());
    let swarm_id = hex::encode(hasher.finalize());

    // Check if already joined
    let store = app.store("swarms.json")
        .map_err(|e| e.to_string())?;

    if store.has(&swarm_id).await.map_err(|e| e.to_string())? {
        return Err("Already joined this swarm".to_string());
    }

    // Create metadata
    let metadata = SwarmMetadata {
        id: swarm_id.clone(),
        name,
        psk_hex: uri,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        channels: vec![
            Channel {
                id: "general".to_string(),
                name: "General".to_string(),
            }
        ],
    };

    // Persist
    store.set(swarm_id.clone(), serde_json::to_value(&metadata).unwrap());
    store.save().await.map_err(|e| e.to_string())?;

    Ok(swarm_id)
}
```

### Frontend: Invite Dialog Component
```svelte
<!-- Source: Svelte 5 patterns + tauri-plugin-clipboard-manager -->
<script lang="ts">
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { createSwarm } from '$lib/tauri';

let {
  open = $bindable(false),
}: {
  open?: boolean;
} = $props();

let swarmName = $state('');
let generatedCode = $state<string | null>(null);
let loading = $state(false);
let error = $state<string | null>(null);

async function handleGenerate() {
  if (!swarmName.trim()) {
    error = 'Swarm name is required';
    return;
  }

  loading = true;
  error = null;

  try {
    generatedCode = await createSwarm(swarmName.trim());
  } catch (err) {
    error = err instanceof Error ? err.message : 'Failed to create swarm';
  } finally {
    loading = false;
  }
}

async function handleCopy() {
  if (!generatedCode) return;

  try {
    await writeText(generatedCode);
    // Show toast notification
    console.log('Copied to clipboard');
  } catch (err) {
    error = 'Failed to copy to clipboard';
  }
}
</script>

{#if open}
<dialog>
  <h2>Create Swarm</h2>

  {#if !generatedCode}
    <input
      type="text"
      bind:value={swarmName}
      placeholder="Swarm name..."
      disabled={loading}
    />

    <button onclick={handleGenerate} disabled={loading}>
      {loading ? 'Generating...' : 'Generate Invite Code'}
    </button>
  {:else}
    <div class="code-display">
      <code>{generatedCode}</code>
      <button onclick={handleCopy}>Copy to Clipboard</button>
    </div>

    <p class="hint">Share this code with people you want to invite</p>
  {/if}

  {#if error}
    <p class="error">{error}</p>
  {/if}

  <button onclick={() => open = false}>Close</button>
</dialog>
{/if}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Base64 encoding for PSK | Hex encoding (base16) | libp2p spec v1.0.0 | Hex is standard in PSK file format, easier visual verification, libp2p compatibility |
| Custom URI handling per OS | tauri-plugin-deep-link | Tauri v2.0 (2024) | Unified API, automatic OS registration, permission model |
| Manual clipboard API | tauri-plugin-clipboard-manager | Tauri v2.0 (2024) | Cross-platform, permissions-based, no shell commands |
| File-based persistence | tauri-plugin-store | Tauri v2.0 (2024) | Auto-save, atomic writes, resource table integration |
| Svelte stores | Svelte 5 runes (`.svelte.ts`) | Svelte 5.0 (2024) | Universal reactivity, fine-grained updates, simpler DX |
| XSalsa20 PSK encryption | Still XSalsa20 | Unchanged | Proven secure, 192-bit nonce space, no rotation needed |

**Deprecated/outdated:**
- **`register_uri_scheme_protocol`**: Use `tauri-plugin-deep-link` instead (standardized in Tauri v2).
- **Svelte 4 stores for shared state**: Use `.svelte.ts` with `$state` runes for better performance and DX.
- **Base64 PSK encoding in libp2p**: Spec uses base16 (hex), stick to standard for interoperability.

## Open Questions

1. **Multi-Swarm Strategy: Concurrent vs Sequential**
   - What we know: libp2p Swarm is a single instance, PSK applied at creation time
   - What's unclear: Can one app host multiple concurrent swarms (different ports/peers)? Or must we stop/restart network when switching?
   - Recommendation: For MVP (Phase 3), implement sequential swarm switching (stop network, rebuild with new PSK, restart). This is simpler, avoids port conflicts, and sufficient for v1. Defer concurrent multi-swarm to v2 if needed.

2. **Swarm ID Generation**
   - What we know: Need unique ID to index swarms in store, distinguish in UI
   - What's unclear: Use SHA256(PSK), UUID, or fingerprint? Collision risk vs simplicity?
   - Recommendation: Use SHA256(PSK) as swarm ID. Deterministic (same PSK = same ID), collision-resistant, no extra state. Truncate to 16 bytes (32 hex chars) for display if needed.

3. **Channel Creation Authority**
   - What we know: Phase 3 requires "see a list of channels" (UX-03), but doesn't specify who creates channels
   - What's unclear: Can any peer create channels? Is there a swarm "owner"? How do channels sync?
   - Recommendation: For MVP, pre-create "General" channel on swarm creation/join. Defer dynamic channel creation to Phase 5 (when CRDT message sync is implemented). Channels are local metadata for now.

4. **Deep Link Cold Start on macOS**
   - What we know: macOS requires app in `/Applications` for deep links, dynamic registration doesn't work
   - What's unclear: How to test deep links in dev mode (app not installed)?
   - Recommendation: Test deep links using production builds signed and installed. For dev, use "Paste Invite Code" dialog as alternative flow. Document this limitation for contributors.

5. **PSK Storage Encryption**
   - What we know: Swarm PSKs stored hex-encoded in `swarms.json` (Tauri Store plugin)
   - What's unclear: Should PSKs be encrypted at rest? Phase 1 stores identity keypair in OS keychain, should swarms follow?
   - Recommendation: For MVP, store PSKs hex-encoded in Store plugin (app-local JSON, not accessible to other apps). Defer keychain storage to v2. Document that swarms.json is a secret file.

6. **Invite Code Revocation**
   - What we know: Swarm key is static, once shared it cannot be "revoked"
   - What's unclear: How to handle compromised invite codes? Regenerate swarm?
   - Recommendation: Document that invite codes are permanent - sharing a code gives permanent access. For v1, no revocation mechanism. Consider "leave swarm" (delete local metadata) but not "kick peer" (requires coordination protocol, defer to v2).

## Sources

### Primary (HIGH confidence)
- [libp2p Private Networks PSK Spec](https://github.com/libp2p/specs/blob/master/pnet/Private-Networks-PSK-V1.md) - PSK format, XSalsa20 encryption, security guarantees
- [libp2p::pnet Rust docs](https://docs.rs/libp2p/latest/libp2p/pnet/index.html) - PreSharedKey API, PnetConfig usage
- [libp2p-pnet crate](https://docs.rs/libp2p-pnet/latest/libp2p_pnet/) - Core types, KeyParseError, fingerprint
- [Tauri Deep Linking Plugin](https://v2.tauri.app/plugin/deep-linking/) - URI scheme registration, on_open_url handler
- [Tauri Clipboard Manager Plugin](https://v2.tauri.app/plugin/clipboard/) - writeText, readText, permissions
- [Tauri Store Plugin](https://v2.tauri.app/plugin/store/) - load, set, get, save, JSON format
- [rand crate OsRng docs](https://rust-random.github.io/book/guide-rngs.html) - CSPRNG, security requirements
- [hex crate](https://docs.rs/hex/latest/hex/) - RFC 4648 base16 encoding
- [Svelte 5 State Management](https://svelte.dev/docs/kit/state-management) - Runes, universal reactivity
- [Svelte 5 Stores](https://svelte.dev/docs/svelte/stores) - Migration guide, .svelte.ts patterns

### Secondary (MEDIUM confidence)
- [Loopwerk: Refactoring Svelte stores to $state runes](https://www.loopwerk.io/articles/2025/svelte-5-stores/) - Community patterns for runes migration
- [Mainmatter: Runes and Global State](https://mainmatter.com/blog/2025/03/11/global-state-in-svelte-5/) - Best practices for shared state
- [Custom URI Schemes - HackTricks](https://book.hacktricks.wiki/en/mobile-pentesting/ios-pentesting/ios-custom-uri-handlers-deeplinks-custom-schemes.html) - Security considerations, hijacking risks
- [Tauri Persistent State Guide](https://aptabase.com/blog/persistent-state-tauri-apps) - Community best practices for store plugin
- [libp2p::pnet::PreSharedKey struct](https://starry-network.github.io/starry_node/libp2p/pnet/struct.PreSharedKey.html) - API methods, new(), as_ref()

### Tertiary (LOW confidence)
- WebSearch: "libp2p multiple swarm instances" - No clear answer on concurrent swarms, needs validation
- Community discussions: Deep link race conditions on macOS - anecdotal reports, not official docs

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All plugins are official Tauri v2, libp2p-pnet is standard PSK implementation
- Architecture: HIGH - Patterns verified in docs, similar to Phase 1/2 established patterns
- Pitfalls: MEDIUM - PSK/security pitfalls from general knowledge + some community reports, need validation in implementation
- Multi-swarm strategy: MEDIUM - Sequential approach recommended based on libp2p architecture, but concurrent not fully ruled out

**Research date:** 2026-02-14
**Valid until:** ~30 days (Tauri v2 stable, libp2p stable, but Svelte 5 patterns evolving rapidly)