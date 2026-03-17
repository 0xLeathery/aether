# M001: Aether v1.0–v2.0 (Shipped)

**Vision:** A Local-First, Sovereign Node communication platform that replaces centralized chat (Discord, Slack) with a decentralized, serverless architecture.

## Success Criteria

- Desktop app complete with ~6,948 LOC (Rust + Svelte/TypeScript)
- Marketing site shipped with ~3,357 LOC (Svelte/TypeScript/CSS/Markdown)
- All 64 requirements across 3 original milestones (v1.0, v1.1, v2.0) validated


## Slices

- [x] **S01: Foundation Identity** `risk:medium` `depends:[]`
  > After this: Scaffold the Tauri v2 + Svelte project and implement the complete Rust identity module with Ed25519 key generation, platform keychain storage, and display name management.
- [x] **S02: Sovereign Network** `risk:medium` `depends:[S01]`
  > After this: Build the libp2p network core: behaviour composition, swarm initialization, transport configuration, and background event loop.
- [x] **S03: Invitation System** `risk:medium` `depends:[S02]`
  > After this: Build the Rust backend for swarm management: PSK key generation, aether:// URI encoding, persistent swarm storage via Tauri Store plugin, and Tauri IPC commands for create/join/list/switch operations.
- [x] **S04: Fix Keychain Password Prompts** `risk:medium` `depends:[S03]`
  > After this: Eliminate macOS keychain password prompts in production builds by adding post-creation ACL modification via the macOS `security` command-line tool.
- [x] **S05: Real Time Voice** `risk:medium` `depends:[S04]`
  > After this: Build the audio capture, codec, mixing, and playback pipeline for real-time voice.
- [x] **S06: Persistent Chat** `risk:medium` `depends:[S05]`
  > After this: Build the complete Rust chat engine: CRDT document model with Automerge, file-based persistence, and libp2p-stream sync protocol.
- [x] **S07: Foundation** `risk:medium` `depends:[S06]`
  > After this: Add microphone mute/unmute functionality to voice sessions with visual feedback.
- [x] **S08: Channel Management** `risk:medium` `depends:[S07]`
  > After this: Build the backend CRDT infrastructure for channel metadata: a SwarmMetadataDocument that stores channel list + creator key in an Automerge document, file-based persistence, and a dedicated metadata sync protocol for peer-to-peer propagation.
- [x] **S09: Unread Mentions** `risk:medium` `depends:[S08]`
  > After this: Build the unread tracking system end-to-end: Rust backend persistence, reactive frontend store, channel-level dot indicators, swarm-level unread indicators, "Mark as read" context menu action, and automatic clearing on channel view.
- [x] **S10: Fix Ci Cmake Build Errors** `risk:medium` `depends:[S09]`
  > After this: Switch from the broken `opus` crate (audiopus_sys 0.2.2, CMake < 3.5) to `opus-codec` to fix CI build failures. Add PR check workflow and Linux to the release matrix.
- [x] **S11: Peer Moderation** `risk:medium` `depends:[S10]`
  > After this: Build the moderation data layer end-to-end: Rust backend types, persistent storage, Tauri IPC commands, TypeScript bindings, reactive Svelte 5 store, and voice mixer peer-mute integration.
- [x] **S12: Fix Iscreator Integration Bug** `risk:medium` `depends:[S11]`
  > After this: Fix the orphaned `setLocalIdentity()` call so `isCreator` evaluates correctly, add CRDT sync-layer creator validation to prevent unauthorized channel mutations from peers, and auto-migrate channels with missing creator metadata on app startup.
- [x] **S13: Desktop Notifications** `risk:medium` `depends:[S12]`
  > After this: Add native desktop notifications for incoming messages and @mentions using tauri-plugin-notification.
- [x] **S14: Site Scaffold** `risk:medium` `depends:[S13]`
  > After this: Deployed SvelteKit site with responsive layout, global nav/footer, stub pages, and zero third-party dependencies.
- [x] **S15: Landing Page** `risk:medium` `depends:[S14]`
  > After this: Vision-first hero, how-it-works, feature cards, trade-offs, and open-source trust signals on the landing page.
- [x] **S16: Interactive Demo** `risk:medium` `depends:[S15]`
  > After this: Browser-based WebRTC P2P text chat sandbox with share-link signaling on `/demo`.
- [x] **S17: Documentation** `risk:medium` `depends:[S16]`
  > After this: User guides and technical architecture docs with sidebar navigation, prev/next links, and full-text search.
- [x] **S18: Milestone Verification & Closure** `risk:medium` `depends:[S17]`
  > After this: All v2.0 requirements validated; site deployed and operational.
