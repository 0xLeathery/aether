# T01: 07-channel-management 01

**Slice:** S08 — **Milestone:** M001

## Description

Build the backend CRDT infrastructure for channel metadata: a SwarmMetadataDocument that stores channel list + creator key in an Automerge document, file-based persistence, and a dedicated metadata sync protocol for peer-to-peer propagation.

Purpose: This is the data foundation for all channel operations. Without the CRDT metadata document and sync protocol, channels cannot be created, synced, or managed across peers.

Output: Three new Rust modules (metadata_doc, metadata_sync, metadata_storage) in the swarm crate, updated error types, and SwarmMetadata with creator_key field.

## Must-Haves

- [ ] "SwarmMetadataDocument can create, rename, and remove channels in an Automerge CRDT document"
- [ ] "Metadata document stores creator_key and channel map with O(1) lookup"
- [ ] "Metadata sync protocol exchanges swarm metadata between peers over libp2p-stream"
- [ ] "SwarmMetadata local store includes creator_key field for permission caching"
- [ ] "Default channels (general + voice) are created when a new metadata document is initialized"

## Files

- `src-tauri/src/swarm/metadata_doc.rs`
- `src-tauri/src/swarm/metadata_sync.rs`
- `src-tauri/src/swarm/metadata_storage.rs`
- `src-tauri/src/swarm/mod.rs`
- `src-tauri/src/swarm/storage.rs`
- `src-tauri/src/error.rs`
