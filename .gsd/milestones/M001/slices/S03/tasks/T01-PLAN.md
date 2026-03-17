# T01: 03-invitation-system 01

**Slice:** S03 — **Milestone:** M001

## Description

Build the Rust backend for swarm management: PSK key generation, aether:// URI encoding, persistent swarm storage via Tauri Store plugin, and Tauri IPC commands for create/join/list/switch operations.

Purpose: Provides the cryptographic foundation and data persistence layer that the frontend (Plan 03-02) will invoke to create and join swarms.
Output: Complete Rust swarm module with Tauri commands, ready for frontend integration.

## Must-Haves

- [ ] "Swarm PSK is generated with cryptographically secure randomness (OsRng)"
- [ ] "Secret Code encodes as aether://<64-hex-chars> and decodes back to 32-byte PSK"
- [ ] "Swarm metadata persists across app restarts via Tauri Store plugin"
- [ ] "Network service restarts with new PSK when switching swarms"
- [ ] "Creating a swarm auto-creates a General channel"
- [ ] "Joining a swarm with same PSK detects duplicate and rejects"

## Files

- `src-tauri/Cargo.toml`
- `src-tauri/src/error.rs`
- `src-tauri/src/swarm/mod.rs`
- `src-tauri/src/swarm/key.rs`
- `src-tauri/src/swarm/storage.rs`
- `src-tauri/src/swarm/uri.rs`
- `src-tauri/src/commands/swarm.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/network/swarm.rs`
- `src-tauri/src/network/mod.rs`
- `src-tauri/src/lib.rs`
