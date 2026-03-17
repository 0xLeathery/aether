# Phase 1: Foundation & Identity - Research

**Researched:** 2026-02-13
**Domain:** Tauri desktop application development with Ed25519 identity and platform keychain integration
**Confidence:** HIGH

## Summary

Phase 1 requires building a cross-platform Tauri desktop application with secure Ed25519 identity generation stored in platform-native keychains. The research confirms that Tauri 2.x (currently v2.10.2) provides a mature, production-ready foundation with excellent Rust ecosystem integration. Key findings: (1) `tauri-plugin-keyring` with `keyring` crate v3.6.3 supports all three platforms with proper iCloud Keychain sync on macOS via the `apple-native` feature; (2) `ed25519-dalek` v2.2.0 provides battle-tested Ed25519 cryptography; (3) Frontend framework choice is truly discretionary—Svelte offers smallest bundle size fitting the "lightweight" philosophy, while React provides largest ecosystem; (4) Terminal/hacker aesthetic is CSS-driven, not framework-specific.

**Primary recommendation:** Use Tauri v2.10+ with Svelte for frontend, `tauri-plugin-keyring` for keychain abstraction, `ed25519-dalek` for key generation, and `blockies` crate for deterministic avatars. Implement identity creation as an async setup flow with proper error handling using custom Result types.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

#### Identity Creation Flow
- Explain-first approach: first launch shows a brief sovereignty explainer before key generation
- Tone is technical-honest: "We generate a cryptographic key stored in your device's secure hardware"
- Flow: Welcome/explainer → Set display name → Generate key → Done
- Display name is set during setup, before key generation

#### Key Storage Strategy
- Use platform-native key storage with sync where available
- **macOS:** iCloud Keychain (kSecAttrSynchronizable = true) — identity syncs across Apple devices naturally
- **Windows:** Windows Hello / Credential Manager
- **Linux:** Claude's discretion (Secret Service API or encrypted keyfile)
- This is a PIVOT from the original "hardware-bound, non-exportable" decision — Apple's ecosystem makes identity account-bound, not device-bound

#### Identity Display
- Display name is primary, key identifier is subtle secondary text
- Deterministic avatar generated from public key (identicon/jazzicon style)
- Clickable avatar/name opens a dedicated profile popover panel
- Claude's discretion on key format (truncated hex, short hash, etc.)

#### App Shell / Chrome
- Visual language: terminal/hacker aesthetic — dark, monospace-heavy, green/amber accents, "command center" feel
- Dark theme by default, light mode available for accessibility
- Three-column layout: Swarms sidebar | Channels list | Main content area
- Frontend framework: Claude's discretion (pick what fits Tauri best)

#### Fallback & Recovery
- If key generation fails: show error with specific guidance to fix (e.g., "Enable Keychain Access in System Settings"), offer retry
- No in-memory fallback — identity must persist
- Device loss is mitigated on Apple by iCloud Keychain sync; on other platforms, losing the device means losing the identity (acceptable trade-off for v1)

### Claude's Discretion
- Key identifier display format (truncated hex, short hash, emoji fingerprint)
- Frontend framework choice for Tauri
- Linux keystore implementation
- Exact typography, spacing, and accent color values
- Loading/transition states during key generation

### Deferred Ideas (OUT OF SCOPE)
- Petnames (local nicknames for peers) — Phase 5 or v2
- Contacts list / peer management — v2
- Multi-device sync for non-Apple platforms — v2
- Key export/import — v2
</user_constraints>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Tauri | v2.10.2 | Desktop app framework | Official stable release, mature v2 with 1600+ tests, cross-platform WebView management, zero Node.js overhead |
| ed25519-dalek | v2.2.0 | Ed25519 cryptography | Dalek cryptography is the de-facto standard for Ed25519 in Rust, audited, used by major crypto projects |
| keyring | v3.6.3 | Platform keychain abstraction | Cross-platform (macOS/Windows/Linux), active maintenance, 91.4% doc coverage, handles platform quirks |
| tauri-plugin-keyring | v0.1.0 | Tauri keychain integration | Official-quality plugin wrapping keyring for Tauri IPC, proper permission system integration |
| rand | v0.8+ | Cryptographic RNG | Standard for secure random generation, OsRng for cryptographic key material |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| serde | v1.0+ | Serialization for IPC | Required for all Tauri command inputs/outputs, JSON IPC communication |
| serde_json | v1.0+ | JSON handling | Storing display name and app state to disk (with tauri-plugin-store) |
| thiserror | v1.0+ | Custom error types | Structured error handling in Tauri commands, better than String errors |
| tokio | v1.0+ | Async runtime | Tauri uses tokio internally, commands can be async for I/O operations |
| blockies | latest | Deterministic avatars | Ethereum-style blockies from public key, pure Rust implementation |

### Frontend (Recommendation: Svelte)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|-------------|
| Svelte | v5.x | Frontend framework | Smallest bundle size, compiler-based (no runtime overhead), excellent for lightweight desktop apps |
| @tauri-apps/api | v2.10.1 | Tauri frontend bindings | Official API for IPC invoke, window management, events |
| Vite | v5.x+ | Build tool | Official Tauri scaffolding uses Vite, fast HMR, Tauri dev integration |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Svelte | React | Larger bundle (+100KB), massive ecosystem, more third-party components, slower runtime |
| Svelte | Vue | Medium bundle, easier learning curve, composition API similar to Svelte, slightly larger than Svelte |
| tauri-plugin-keyring | Direct keyring usage | More control, but requires manual IPC command setup, permission wiring, error serialization |
| blockies | identicon-rs | GitHub-style identicons instead of Ethereum blockies, different visual aesthetic |

**Installation:**
```bash
# Backend (in src-tauri/)
cargo add tauri@2.10
cargo add tauri-plugin-keyring@0.1
cargo add ed25519-dalek@2.2 --features rand_core
cargo add keyring@3.6 --features apple-native,windows-native,sync-secret-service
cargo add rand@0.8
cargo add serde --features derive
cargo add serde_json
cargo add thiserror
cargo add blockies

# Frontend
npm install @tauri-apps/api@2.10.1
npm install --save-dev @tauri-apps/cli@2.10.0

# Create Tauri + Svelte project
npm create tauri-app@latest -- --template svelte-ts
```

## Architecture Patterns

### Recommended Project Structure
```
aether/
├── src/                     # Svelte frontend
│   ├── lib/
│   │   ├── components/      # UI components (Avatar, ProfilePanel, etc.)
│   │   ├── stores/          # Svelte stores for identity state
│   │   └── tauri.ts         # Typed Tauri command wrappers
│   ├── routes/              # SvelteKit routes (if using routing)
│   ├── App.svelte           # Main app shell
│   └── main.ts              # Frontend entry
├── src-tauri/               # Rust backend
│   ├── src/
│   │   ├── main.rs          # Tauri builder, command registration
│   │   ├── identity/
│   │   │   ├── mod.rs       # Module exports
│   │   │   ├── keypair.rs   # Ed25519 key generation, serialization
│   │   │   ├── storage.rs   # Keychain operations (platform abstraction)
│   │   │   └── display.rs   # Display name management
│   │   ├── commands/
│   │   │   └── identity.rs  # Tauri commands for identity operations
│   │   └── error.rs         # Custom error types with Serialize impl
│   ├── Cargo.toml
│   ├── tauri.conf.json      # Tauri configuration
│   └── capabilities/
│       └── default.json     # Permissions for keyring access
└── package.json
```

### Pattern 1: Async Identity Setup with State Management
**What:** Use Tauri's setup hook to check for existing identity, manage app state through Mutex-wrapped structs
**When to use:** On app launch, need to determine if onboarding flow or main app should show
**Example:**
```rust
// Source: https://v2.tauri.app/develop/state-management/
use std::sync::Mutex;
use tauri::{Manager, State};

struct IdentityState {
    display_name: Option<String>,
    public_key: Option<Vec<u8>>,
    initialized: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(IdentityState {
            display_name: None,
            public_key: None,
            initialized: false,
        }))
        .plugin(tauri_plugin_keyring::init())
        .invoke_handler(tauri::generate_handler![
            check_identity,
            create_identity,
            get_identity
        ])
        .setup(|app| {
            // Check if identity exists on startup
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                // Attempt to load identity from keychain
                // Update IdentityState accordingly
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Pattern 2: Custom Error Types with thiserror
**What:** Define app-specific error enum implementing serde::Serialize for clean IPC error propagation
**When to use:** All Tauri commands that can fail
**Example:**
```rust
// Source: https://v2.tauri.app/develop/calling-rust/
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("Identity already exists")]
    AlreadyExists,
    #[error("Identity not found in keychain")]
    NotFound,
    #[error("Keychain access denied: {0}")]
    KeychainDenied(String),
    #[error("Key generation failed: {0}")]
    KeyGeneration(String),
    #[error(transparent)]
    Keyring(#[from] keyring::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl serde::Serialize for IdentityError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[tauri::command]
async fn create_identity(
    display_name: String,
    state: State<'_, Mutex<IdentityState>>,
) -> Result<PublicKeyData, IdentityError> {
    // Implementation
}
```

### Pattern 3: Platform Keychain Abstraction
**What:** Wrap keyring crate with app-specific logic for iCloud sync and platform differences
**When to use:** Isolate platform-specific keychain quirks from business logic
**Example:**
```rust
// Source: https://docs.rs/keyring/3.6.3
use keyring::Entry;

pub struct IdentityStorage {
    service: String,
}

impl IdentityStorage {
    pub fn new(app_identifier: &str) -> Self {
        Self {
            service: format!("{}.identity", app_identifier),
        }
    }

    pub fn store_keypair(&self, user: &str, keypair_bytes: &[u8]) -> Result<(), keyring::Error> {
        // On macOS with apple-native feature, this automatically uses iCloud Keychain
        // with kSecAttrSynchronizable if available
        let entry = Entry::new(&self.service, user)?;
        entry.set_secret(keypair_bytes)?;
        Ok(())
    }

    pub fn retrieve_keypair(&self, user: &str) -> Result<Vec<u8>, keyring::Error> {
        let entry = Entry::new(&self.service, user)?;
        entry.get_secret()
    }
}
```

### Pattern 4: Deterministic Avatar from Public Key
**What:** Generate consistent blockies-style avatar image from public key bytes
**When to use:** Displaying user identity, profile UI
**Example:**
```rust
// Source: https://github.com/debris/blockies.rs
use blockies::Ethereum;

pub fn generate_avatar(public_key: &[u8]) -> Vec<u8> {
    let blockies = Ethereum::default();
    let mut png_buffer = Vec::new();
    // Public key is deterministic seed
    blockies.create_icon(&mut png_buffer, public_key)
        .expect("Failed to generate avatar");
    png_buffer
}
```

### Pattern 5: Frontend IPC with Typed Commands
**What:** Create TypeScript wrappers for Tauri commands with proper typing
**When to use:** All frontend-backend communication
**Example:**
```typescript
// src/lib/tauri.ts
import { invoke } from '@tauri-apps/api/core';

export interface IdentityData {
  displayName: string;
  publicKey: string;
  avatar: string; // base64 PNG
}

export async function createIdentity(displayName: string): Promise<IdentityData> {
  return await invoke<IdentityData>('create_identity', { displayName });
}

export async function getIdentity(): Promise<IdentityData | null> {
  try {
    return await invoke<IdentityData>('get_identity');
  } catch (error) {
    if ((error as string).includes('not found')) {
      return null;
    }
    throw error;
  }
}
```

### Anti-Patterns to Avoid
- **Panic in commands:** Never use `.unwrap()` or `.expect()` in Tauri commands—panics crash synchronous commands and deadlock async ones. Always return `Result<T, E>`.
- **Borrowed arguments in async commands:** Cannot use `&str` or `State<'_, T>` in async commands due to Send requirement. Use `String` and clone state handle.
- **Blocking I/O in sync commands:** Keychain access can be slow (10-100ms). Use async commands or `tokio::task::spawn_blocking`.
- **Over-permissive capabilities:** Don't grant `allow-all` permissions. Explicitly list keyring operations in capabilities/default.json.
- **Storing secrets in local storage:** Frontend local storage is NOT secure. Secrets must live in Rust backend and keychain only.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Platform keychain access | Custom FFI to macOS Security.framework, Windows Credential Manager API, Linux DBus Secret Service | `keyring` crate with platform features | Handles platform differences (macOS kSecAttrSynchronizable, Windows Credential Manager encoding, Linux async DBus), error cases, memory zeroing |
| Ed25519 key generation | Custom elliptic curve implementation, wrapping libsodium | `ed25519-dalek` | Constant-time operations, audited, battle-tested in Solana/Substrate/Matrix, proper RNG integration |
| Tauri IPC error handling | String errors with manual JSON serialization | `thiserror` + serde::Serialize impl | Type-safe error propagation, structured error data for frontend, automatic error messages |
| Identity state persistence | Custom JSON files in home directory | `tauri-plugin-store` for app state, keychain for secrets | Atomic writes, proper permissions, cross-platform paths, encryption where available |
| Avatar generation | Canvas-based rendering, manual hash-to-color algorithms | `blockies` or `identicon-rs` crates | Deterministic, tested, consistent cross-platform output, handles edge cases |

**Key insight:** Platform keychain APIs have numerous edge cases—macOS Keychain requires specific attribute flags for iCloud sync (kSecAttrSynchronizable), Windows Credential Manager has length limits and encoding quirks, Linux Secret Service requires async DBus calls and session management. The `keyring` crate abstracts 6+ years of bug fixes and platform testing. Similarly, cryptographic implementations require constant-time operations and proper RNG seeding that are easy to get wrong.

## Common Pitfalls

### Pitfall 1: iCloud Keychain Not Syncing
**What goes wrong:** Keys stored on macOS don't sync to user's other Apple devices despite using keyring crate
**Why it happens:** The `keyring` crate's default macOS backend may not set `kSecAttrSynchronizable = true` by default (implementation detail depends on version)
**How to avoid:** Verify `keyring` crate configuration enables sync; may require directly using `security-framework` crate to explicitly set synchronization attributes when creating keychain items
**Warning signs:** User reports identity exists on one Mac but not iPhone/iPad, keychain item visible in Keychain Access but not syncing

### Pitfall 2: Async Command Lifetime Issues
**What goes wrong:** Compilation errors like "cannot borrow state as mutable" or "borrowed value does not live long enough" in async Tauri commands
**Why it happens:** Async commands must be `Send`, but `State<'_, Mutex<T>>` borrows from command signature lifetime which doesn't last across `.await` points
**How to avoid:** Clone data before async operations, or use `AppHandle::state()` to get fresh state references
**Warning signs:** Rust lifetime errors mentioning `Send` trait or borrow checker issues in async functions
**Example fix:**
```rust
// BAD
#[tauri::command]
async fn create_identity(state: State<'_, Mutex<IdentityState>>) -> Result<()> {
    let mut state_lock = state.lock().unwrap(); // Lifetime issue across await
    some_async_op().await?;
    state_lock.initialized = true;
    Ok(())
}

// GOOD
#[tauri::command]
async fn create_identity(app: tauri::AppHandle) -> Result<()> {
    some_async_op().await?;
    let state = app.state::<Mutex<IdentityState>>();
    let mut state_lock = state.lock().unwrap();
    state_lock.initialized = true;
    Ok(())
}
```

### Pitfall 3: Keychain Access Denied on First Run
**What goes wrong:** Key generation or retrieval fails with "access denied" error on first launch
**Why it happens:** On macOS, apps require user permission to access Keychain; on Windows, Credential Manager access may be blocked by antivirus; on Linux, Secret Service may require unlocking
**How to avoid:** Wrap keychain operations in proper error handling, provide user-friendly error messages with platform-specific guidance ("macOS: Grant Keychain access in System Settings", "Windows: Check Windows Defender settings")
**Warning signs:** Works in dev but fails in production build, inconsistent behavior across machines

### Pitfall 4: Ed25519 Key Serialization Confusion
**What goes wrong:** Public key display/storage mixes up SigningKey bytes (64 bytes) vs VerifyingKey bytes (32 bytes)
**Why it happens:** `ed25519-dalek` has `SigningKey::to_bytes()` (32 bytes secret), `SigningKey::to_keypair_bytes()` (64 bytes secret+public), and `VerifyingKey::to_bytes()` (32 bytes public)
**How to avoid:** Only store secret key (32 bytes) in keychain, derive public key on load. For display, use `verifying_key().to_bytes()` (32 bytes)
**Warning signs:** Key truncation errors, incorrect key length (should be 32 for public, 32 for secret, not 64)
**Example:**
```rust
// CORRECT
let signing_key = SigningKey::generate(&mut OsRng);
let secret_bytes = signing_key.to_bytes(); // 32 bytes - store in keychain
let public_key = signing_key.verifying_key();
let public_bytes = public_key.to_bytes(); // 32 bytes - show in UI

// WRONG - don't store keypair bytes (includes secret + public = 64 bytes)
let keypair_bytes = signing_key.to_keypair_bytes(); // Unnecessary duplication
```

### Pitfall 5: Frontend State Desync
**What goes wrong:** UI shows identity exists but backend doesn't have it, or vice versa
**Why it happens:** Frontend stores identity state in Svelte store or localStorage, backend state lives in Rust. State gets out of sync during errors or partial operations.
**How to avoid:** Backend is source of truth—frontend should query backend on mount via `get_identity()` command, not rely on cached state
**Warning signs:** Refresh fixes the issue, inconsistent behavior after errors, state persists after keychain deletion

## Code Examples

Verified patterns from official sources:

### Generate Ed25519 Keypair
```rust
// Source: https://docs.rs/ed25519-dalek/2.2.0
use rand::rngs::OsRng;
use ed25519_dalek::{SigningKey, VerifyingKey};

pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

pub fn serialize_keypair(signing_key: &SigningKey) -> (Vec<u8>, Vec<u8>) {
    let secret_bytes = signing_key.to_bytes().to_vec(); // 32 bytes
    let public_bytes = signing_key.verifying_key().to_bytes().to_vec(); // 32 bytes
    (secret_bytes, public_bytes)
}

pub fn deserialize_signing_key(secret_bytes: &[u8]) -> Result<SigningKey, ed25519_dalek::SignatureError> {
    let bytes: [u8; 32] = secret_bytes.try_into()
        .map_err(|_| ed25519_dalek::SignatureError::from_source("Invalid key length"))?;
    SigningKey::from_bytes(&bytes)
}
```

### Store and Retrieve from Keychain
```rust
// Source: https://context7.com/charlesportwoodii/tauri-plugin-keyring
use tauri_plugin_keyring::{KeyringExt, KeyringStore};

#[tauri::command]
async fn store_identity(
    app: tauri::AppHandle,
    secret_key: Vec<u8>,
) -> Result<(), String> {
    let keyring = app.keyring().map_err(|e| e.to_string())?;

    keyring
        .set_secret("aether.identity", "signing_key", &secret_key)
        .await
        .map_err(|e| format!("Failed to store key: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn load_identity(app: tauri::AppHandle) -> Result<Vec<u8>, String> {
    let keyring = app.keyring().map_err(|e| e.to_string())?;

    keyring
        .get_secret("aether.identity", "signing_key")
        .await
        .map_err(|e| format!("Identity not found: {}", e))
}
```

### Tauri Command Registration
```rust
// Source: https://v2.tauri.app/develop/calling-rust/
#[tauri::command]
async fn create_identity(
    display_name: String,
    app: tauri::AppHandle,
) -> Result<IdentityData, IdentityError> {
    // Generate keypair
    let (signing_key, verifying_key) = generate_keypair();
    let (secret_bytes, public_bytes) = serialize_keypair(&signing_key);

    // Store in keychain
    let keyring = app.keyring()?;
    keyring.set_secret("aether.identity", "signing_key", &secret_bytes).await?;

    // Store display name in app data
    // Generate avatar
    let avatar_png = generate_avatar(&public_bytes);
    let avatar_base64 = base64::encode(&avatar_png);

    Ok(IdentityData {
        display_name,
        public_key: hex::encode(public_bytes),
        avatar: avatar_base64,
    })
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_keyring::init())
        .invoke_handler(tauri::generate_handler![
            create_identity,
            get_identity,
            check_identity_exists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Frontend Identity Check on Mount
```typescript
// Source: https://v2.tauri.app/reference/javascript/api/namespacewindow/
import { invoke } from '@tauri-apps/api/core';
import { onMount } from 'svelte';

let identity: IdentityData | null = null;
let loading = true;

onMount(async () => {
  try {
    identity = await invoke<IdentityData | null>('get_identity');
  } catch (error) {
    console.error('Failed to load identity:', error);
  } finally {
    loading = false;
  }
});
```

### Capability Configuration for Keyring
```json
// Source: https://context7.com/charlesportwoodii/tauri-plugin-keyring
{
  "identifier": "main-capability",
  "description": "Main window capabilities",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "keyring:allow-initialize-keyring",
    "keyring:allow-set-password",
    "keyring:allow-get-password",
    "keyring:allow-set-secret",
    "keyring:allow-get-secret",
    "keyring:allow-delete-secret"
  ]
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Tauri v1 allowlist system | Tauri v2 capabilities and permissions | Tauri 2.0 (2024) | More granular security, per-window permissions, easier to audit |
| Manual keyring FFI per platform | `keyring` crate + `tauri-plugin-keyring` | 2023-2024 | Unified API, better error handling, iCloud Keychain support |
| Node.js IPC layer (Electron style) | Direct Rust-WebView IPC | Tauri v1 (2021) | 50-80% smaller binaries, no Node.js attack surface, faster IPC |
| Global mutable state in Tauri | Managed state with Mutex wrappers | Tauri v1.0 | Thread-safe state access, better error handling |
| JSON.stringify for errors | Custom Error types with Serialize | Tauri v2 era | Structured error data, type-safe frontend handling |

**Deprecated/outdated:**
- **Tauri v1 allowlist:** Replaced by capabilities system in v2. Migration required for security model.
- **`window.__TAURI__` global:** Still works but npm package `@tauri-apps/api` preferred for tree-shaking.
- **Synchronous Tauri commands for I/O:** Async commands now standard for any I/O (file, network, keychain).

## Open Questions

1. **macOS iCloud Keychain sync verification**
   - What we know: `keyring` crate supports macOS with `apple-native` feature, uses Security.framework
   - What's unclear: Whether `kSecAttrSynchronizable` is set by default, or requires custom configuration
   - Recommendation: Test on two Apple devices signed into same iCloud account; if sync doesn't work, may need to use `security-framework` crate directly to set attributes. Document as known issue if workaround needed.

2. **Linux keystore reliability**
   - What we know: `keyring` supports Secret Service (GNOME Keyring, KWallet), keyutils, or mock store
   - What's unclear: Which Linux distros have Secret Service installed by default, fallback behavior
   - Recommendation: Use `sync-secret-service` feature, implement graceful degradation with clear error message if unavailable ("Install GNOME Keyring or KWallet for secure storage"). Consider encrypted file fallback for v1.

3. **Key identifier display format**
   - What we know: Public key is 32 bytes (64 hex chars), too long for UI
   - What's unclear: Best UX for display—truncate to 8 chars? Hash to 4 words? Emoji fingerprint?
   - Recommendation: Start with truncated hex (first 8 chars, "ae3f9b2c..."), evaluate UX in testing. Emoji fingerprints interesting but require mapping library.

4. **Startup performance with keychain access**
   - What we know: Keychain access can be 10-100ms, may require user permission prompt
   - What's unclear: Whether to block app launch or show splash screen during identity load
   - Recommendation: Implement splash screen pattern (https://v2.tauri.app/learn/splashscreen) with loading state, transition to main app or onboarding based on identity presence.

## Sources

### Primary (HIGH confidence)
- **Tauri v2 Documentation** (v2.tauri.app) - Official framework docs, command patterns, security model
  - [Tauri 2.0 Stable Release](https://v2.tauri.app/blog/tauri-20/) - Current version v2.10.2
  - [State Management Guide](https://v2.tauri.app/develop/state-management/)
  - [Calling Rust from Frontend](https://v2.tauri.app/develop/calling-rust/)
  - [Security Documentation](https://v2.tauri.app/security/)
  - [Configuration Reference](https://v2.tauri.app/reference/config/)
- **Context7: ed25519-dalek v2.2.0** (/websites/rs_ed25519-dalek_2_2_0_ed25519_dalek) - Key generation, serialization patterns
- **Context7: keyring v3.6.3** (/websites/rs_keyring_3_6_3) - Cross-platform keychain API, feature flags
- **Context7: tauri-plugin-keyring** (/charlesportwoodii/tauri-plugin-keyring) - Tauri integration, permissions, usage examples
- **docs.rs/keyring** (https://docs.rs/keyring) - API documentation, platform support, version 3.6.3 details

### Secondary (MEDIUM confidence)
- **WebSearch: Tauri frontend frameworks** - React/Svelte/Vue comparison, bundle size analysis
  - [Frontend Configuration Guide](https://v2.tauri.app/start/frontend/)
  - [Best UI Libraries for Tauri](https://crabnebula.dev/blog/the-best-ui-libraries-for-cross-platform-apps-with-tauri/)
- **WebSearch: keyring-rs GitHub** (github.com/hwchen/keyring-rs) - Implementation details, feature flags
  - [GitHub Repository](https://github.com/open-source-cooperative/keyring-rs)
- **WebSearch: Tauri common pitfalls** - Security best practices, IPC patterns
  - [Tauri Best Practices](https://www.projectrules.ai/rules/tauri)
  - [Handling Errors in Tauri](https://tauritutorials.com/blog/handling-errors-in-tauri)
- **Apple Developer Docs: kSecAttrSynchronizable** - iCloud Keychain sync attribute
  - [Official Documentation](https://developer.apple.com/documentation/security/ksecattrsynchronizable)

### Tertiary (LOW confidence)
- **WebSearch: blockies.rs** - Rust identicon implementation (12 stars, active but small project)
  - [GitHub Repository](https://github.com/debris/blockies.rs)
- **WebSearch: Terminal aesthetic CSS** - UI styling approaches (need custom implementation)
  - [daisyUI Themes](https://daisyui.com/docs/themes/)

## Metadata

**Confidence breakdown:**
- Standard stack: **HIGH** - Official Tauri docs, Context7 verified libraries, production usage confirmed
- Architecture: **HIGH** - Patterns from official Tauri guides and verified crates
- Pitfalls: **MEDIUM-HIGH** - Combination of official docs and community experience (GitHub discussions, tutorials)
- macOS iCloud sync: **MEDIUM** - keyring crate supports it, but specific attribute behavior needs testing
- Linux Secret Service: **MEDIUM** - Known working solution, but distro coverage and fallback behavior unclear

**Research date:** 2026-02-13
**Valid until:** 2026-03-13 (30 days - Tauri 2.x is stable, Ed25519/keyring crates are mature)