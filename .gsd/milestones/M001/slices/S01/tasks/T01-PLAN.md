# T01: 01-foundation-identity 01

**Slice:** S01 — **Milestone:** M001

## Description

Scaffold the Tauri v2 + Svelte project and implement the complete Rust identity module with Ed25519 key generation, platform keychain storage, and display name management.

Purpose: Establish the runtime foundation (Tauri desktop app) and the core identity backend that all subsequent UI and networking phases depend on. This is the "sovereign node" engine.

Output: A compilable, launchable Tauri desktop app with a working Rust identity module exposing Tauri IPC commands. Frontend is minimal placeholder — UI comes in Plan 02.

## Must-Haves

- [ ] "Tauri desktop application compiles and launches on macOS"
- [ ] "Ed25519 keypair can be generated and stored in system keychain"
- [ ] "Display name can be stored and retrieved alongside identity"
- [ ] "Identity persists across application restarts via keychain retrieval"
- [ ] "Keychain failure produces a specific, actionable error (not a panic)"

## Files

- `package.json`
- `vite.config.ts`
- `svelte.config.js`
- `tsconfig.json`
- `src/main.ts`
- `src/App.svelte`
- `src/app.css`
- `src-tauri/Cargo.toml`
- `src-tauri/tauri.conf.json`
- `src-tauri/capabilities/default.json`
- `src-tauri/src/main.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/error.rs`
- `src-tauri/src/identity/mod.rs`
- `src-tauri/src/identity/keypair.rs`
- `src-tauri/src/identity/storage.rs`
- `src-tauri/src/identity/display.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/commands/identity.rs`
