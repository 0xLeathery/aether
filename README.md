# Aether

**Sovereign peer-to-peer messaging and voice chat. No servers. No accounts. No tokens. Just your keys.**

Aether is an open-source desktop app for private group communication. Your identity is a cryptographic keypair stored in your OS keychain — nothing lives on anyone else's server. Peers find each other and connect directly. Messages sync without a middleman.

> **No token. No blockchain. No crypto.** Aether is pure open-source software. There is no associated cryptocurrency, NFT, or token of any kind.

---

## Why Aether?

Most "private" messaging apps still require a phone number, email address, or central server that can be subpoenaed, hacked, or shut down. Aether removes that dependency entirely:

| | Aether | Signal | Discord | Matrix |
|---|---|---|---|---|
| No account required | ✓ | ✗ (phone) | ✗ (email) | ✗ (email) |
| No central server | ✓ | ✗ | ✗ | ✗ (federated) |
| No metadata leakage | ✓ | partial | ✗ | partial |
| Local identity | ✓ | ✗ | ✗ | ✗ |
| Voice chat | ✓ | ✓ | ✓ | ✓ |
| Open source | ✓ | ✓ | ✗ | ✓ |

---

## Features

- **Cryptographic identity** — Ed25519 keypairs generated on your device, stored in your OS keychain (iCloud Keychain, Windows Credential Manager, libsecret). No registration, no email, no phone.
- **Direct P2P networking** — libp2p with mDNS (LAN) and Kademlia DHT (internet) discovery, plus relay/DCUTR for NAT traversal. Peers connect directly.
- **Swarms** — Isolated groups secured by a Pre-Shared Key. Share an `aether://` invite URI to let others join.
- **Persistent chat** — Messages sync via Automerge CRDTs, conflict-free across peers. Works offline; syncs when peers reconnect.
- **P2P voice** — Mesh voice chat using the Opus codec with an adaptive jitter buffer. Up to 8 participants, no media server.
- **Channels** — Create, rename, and delete channels within a swarm. Metadata syncs via CRDT.
- **Mentions** — @mention autocomplete with notifications, resilient to display name changes.
- **Contacts & petnames** — Assign local nicknames to peers that override their self-asserted names.
- **Moderation** — Tiered mute/hide/block with global and per-swarm scope.
- **Desktop notifications** — Focus-gated, throttled, mention-aware, moderation-filtered.

---

## Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Tauri v2 [platform dependencies](https://v2.tauri.app/start/prerequisites/)
- **Linux only:** `libasound2-dev`

### Run

```bash
git clone https://github.com/your-org/aether.git
cd aether
npm install
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

Installers are emitted to `src-tauri/target/release/bundle/`.

---

## How It Works

**1. Generate your identity**
On first launch, Aether generates an Ed25519 keypair. The private key goes into your OS keychain. Your public key is your permanent address — share it like a username.

**2. Create or join a swarm**
Creating a swarm generates a 32-byte Pre-Shared Key that isolates the group at the network layer. Share the `aether://` invite URI with anyone you want in the group.

**3. Chat and call**
Text messages are backed by Automerge CRDT documents that sync peer-to-peer over libp2p. Voice flows over a direct P2P mesh: microphone → Opus encode → libp2p stream → Opus decode → jitter buffer → speaker.

---

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
| Desktop | Tauri v2 |
| Networking | libp2p (Kademlia, mDNS, relay, DCUTR, QUIC/TCP, Noise, Yamux) |
| Data sync | Automerge CRDTs |
| Voice | cpal, Opus codec, P2P mesh |
| Identity | ed25519-dalek, OS keychain via keyring |
| Async runtime | Tokio |

---

## Project Structure

```
src/                          # Svelte 5 frontend
├── App.svelte                # Root component — app state machine
├── lib/
│   ├── tauri.ts              # Typed IPC bridge
│   ├── stores/               # Reactive state (Svelte 5 Runes)
│   │   ├── identity          # Keypair, display name
│   │   ├── network           # Connection status, peers
│   │   ├── swarm             # Swarm membership, metadata
│   │   ├── chat              # Messages, channels
│   │   ├── voice             # Voice session state
│   │   ├── contacts          # Petnames
│   │   ├── unread            # Per-channel unread counts
│   │   ├── moderation        # Block/mute/hide state
│   │   └── notification      # Desktop notification logic
│   └── components/           # UI components by feature

src-tauri/                    # Rust backend
├── src/
│   ├── identity/             # Ed25519 keygen, keychain, ACL
│   ├── commands/             # Tauri IPC handlers
│   ├── network/              # libp2p swarm lifecycle
│   ├── swarm/                # PSK management, metadata sync
│   ├── chat/                 # Automerge documents, sync protocol
│   ├── voice/                # Opus codec, jitter buffer, mixer
│   ├── contacts/             # Contact storage
│   └── moderation/           # Moderation tiers
├── Cargo.toml
└── tauri.conf.json
```

---

## Platform Support

| Platform | Status |
|----------|--------|
| macOS (10.13+) | Supported |
| Windows | Supported |
| Linux | Supported |

---

## Trade-offs

Aether is honest about what it gives up for sovereignty:

- **Voice is limited to 8 participants** — mesh topology doesn't scale like a media server
- **Peers must be online simultaneously** to exchange new messages (CRDT state syncs on reconnect for history)
- **Identity is hardware-bound** — if you lose your keychain without a backup, your identity is gone
- **No moderation at the network layer** — PSK-based swarms rely on you not sharing the invite with bad actors

---

## Contributing

Issues and PRs are welcome. The project is built with Tauri v2 (Rust) and Svelte 5 (TypeScript) — familiarity with either is enough to get started.

```bash
npm run tauri dev   # hot-reload dev build
cargo test          # Rust unit tests (from src-tauri/)
npm run check       # TypeScript type check
```

---

## License

MIT
