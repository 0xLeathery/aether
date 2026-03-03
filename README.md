# Aether

**Sovereign peer-to-peer communication. No servers. No accounts. Just cryptographic identity and direct connections.**

Aether is a decentralized desktop app for messaging and voice chat. Your identity is an Ed25519 keypair stored in your OS keychain — not on someone else's server. Peers connect directly over libp2p with NAT traversal, and all data syncs via CRDTs with no central authority.

No email. No phone number. No password. Your keys are your identity.

## Features

- **Cryptographic Identity** — Ed25519 keypairs generated locally and stored in the platform keychain (iCloud Keychain, Windows Credential Manager, libsecret)
- **P2P Networking** — Direct connections via libp2p with mDNS (LAN) and Kademlia DHT (WAN) discovery, plus relay/dcutr NAT traversal
- **Swarms** — Isolated groups secured by Pre-Shared Keys, shareable via `aether://` invite URIs
- **CRDT Chat** — Persistent text messaging synced with Automerge, conflict-free across peers
- **P2P Voice** — Real-time mesh voice chat with Opus codec, adaptive jitter buffer, up to 8 participants
- **Channels** — Create, rename, and delete channels within swarms, with metadata synced via CRDT
- **Mentions** — @mention autocomplete with highlight notifications, resilient to name changes
- **Unread Tracking** — Per-channel and per-swarm unread indicators
- **Contacts & Petnames** — Assign local nicknames to peers that override their self-asserted display names
- **Moderation** — Tiered peer moderation (mute/hide/block) with global and per-swarm scopes
- **Desktop Notifications** — Focus-gated, throttled, mention-aware, moderation-filtered
- **Voice Controls** — Mute/unmute with zero-latency toggle (capture stream stays active)

## Architecture

```
┌─────────────────────────────────┐
│         Svelte 5 Frontend       │
│   Runes reactivity · TypeScript │
├─────────────────────────────────┤
│        Tauri v2 IPC Bridge      │
├─────────────────────────────────┤
│          Rust Backend           │
│  ┌───────────┐  ┌────────────┐  │
│  │  libp2p   │  │ Automerge  │  │
│  │  network  │  │   CRDTs    │  │
│  └───────────┘  └────────────┘  │
│  ┌───────────┐  ┌────────────┐  │
│  │   Opus    │  │  Keychain  │  │
│  │   voice   │  │  identity  │  │
│  └───────────┘  └────────────┘  │
└─────────────────────────────────┘
```

| Layer | Technology |
|-------|-----------|
| Frontend | Svelte 5, TypeScript, Vite 6 |
| Desktop Framework | Tauri v2 |
| Networking | libp2p (Kademlia, mDNS, relay, dcutr, QUIC/TCP, noise, yamux) |
| Data Sync | Automerge CRDTs |
| Voice | cpal (capture/playback), Opus codec, P2P mesh |
| Identity | ed25519-dalek, OS keychain via keyring |
| Async Runtime | Tokio |

## Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Platform dependencies for [Tauri v2](https://v2.tauri.app/start/prerequisites/)
- Linux only: `libasound2-dev` (for audio capture/playback)

## Getting Started

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Project Structure

```
src/                          # Svelte 5 frontend
├── App.svelte                # Root component — app state machine
├── lib/
│   ├── tauri.ts              # Typed IPC bridge (invoke + event listeners)
│   ├── stores/               # Reactive state (Svelte 5 Runes)
│   │   ├── identity          # Keypair, display name
│   │   ├── network           # Connection status, peer list
│   │   ├── swarm             # Swarm membership, metadata
│   │   ├── chat              # Messages, channels
│   │   ├── voice             # Voice session state
│   │   ├── contacts          # Petnames, contact list
│   │   ├── unread            # Per-channel unread counts
│   │   ├── moderation        # Block/mute/hide state
│   │   └── notification      # Desktop notification logic
│   └── components/
│       ├── layout/           # App shell, sidebar, channel list
│       ├── setup/            # First-launch identity creation
│       ├── chat/             # Message list, input, mention picker
│       ├── voice/            # Voice panel, controls
│       ├── peers/            # Peer list
│       ├── swarm/            # Swarm selector, invite/join dialogs
│       ├── contacts/         # Contact list, editor
│       ├── channel/          # Channel context menu, create/delete
│       ├── moderation/       # Peer actions, moderation list
│       └── profile/          # Avatar, profile popover

src-tauri/                    # Rust backend
├── src/
│   ├── lib.rs                # Tauri app builder, command registration
│   ├── error.rs              # Structured error types
│   ├── identity/             # Ed25519 keygen, keychain storage, ACL
│   ├── commands/             # Tauri IPC handlers
│   ├── network/              # libp2p swarm lifecycle, behaviours
│   ├── swarm/                # PSK management, metadata sync, storage
│   ├── chat/                 # Automerge documents, sync protocol
│   ├── voice/                # Opus codec, jitter buffer, mixer, capture/playback
│   ├── contacts/             # Contact storage
│   └── moderation/           # Moderation tiers, storage
├── Cargo.toml
└── tauri.conf.json
```

## How It Works

**Identity** — On first launch, Aether generates an Ed25519 keypair. The private key goes into your OS keychain; the public key becomes your permanent address. You choose a display name, but peers can override it locally with petnames.

**Swarms** — Creating a swarm generates a 32-byte Pre-Shared Key that isolates the group at the network layer. Share the `aether://` invite URI for others to join. All traffic within a swarm is encrypted with the PSK.

**Chat** — Each channel is backed by an Automerge CRDT document synced peer-to-peer over libp2p streams. Messages merge conflict-free without coordination. Documents persist to disk for offline access.

**Voice** — Audio flows over a direct P2P mesh. The pipeline runs: microphone capture → Opus encode → libp2p stream → network → Opus decode → adaptive jitter buffer → mixer → speaker playback. All at 48kHz with 20ms frames.

## Platforms

| Platform | Status |
|----------|--------|
| macOS (10.13+) | Supported |
| Windows | Supported |
| Linux | Supported |

## License

MIT
