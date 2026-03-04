# Aether

**Private group chat and voice calls, directly between devices. No servers. No accounts. No sign-up.**

Aether is an open-source desktop app for people who want to communicate without trusting a third party. Your identity is a passkey stored on your device — nothing lives on anyone else's server. You connect directly to the people you're talking to.

CA: 2A5sL4m3ZGDpJ3pCbCz6P9wR3cW3VskpAL3jBPN9pump
---

## How It Works

**1. Get your passkey**
On first launch, Aether creates a passkey on your device and stores it in your OS keychain (the same place your saved passwords live). This passkey *is* your identity — no sign-up, no email, no phone number.

**2. Create or join a group**
Start a group and share the invite link with the people you want in it. Only people with the link can join — like a private room with a one-time door code.

**3. Chat and call**
Messages go directly between devices — no server in the middle. If someone's offline, their messages catch up automatically when they reconnect. Voice works the same way: direct device-to-device, no middleman.

---

## What You Get

- **Text chat** — Messages sync across everyone in the group automatically, even if someone was offline when you sent it.
- **Voice calls** — Up to 8 people, no media server. Audio goes directly between devices.
- **Channels** — Organise conversations into channels within a group, like you would in Slack or Discord.
- **Mentions** — @mention people to get their attention. Works with notifications.
- **Contacts & nicknames** — Assign your own nicknames to people that only you see.
- **Moderation** — Mute, hide, or block people per-group or globally.
- **Desktop notifications** — Smart notifications that respect focus, mutes, and mentions.

### How does it compare?

| | Aether | Signal | Discord | Matrix |
|---|---|---|---|---|
| No account required | ✓ | ✗ (phone) | ✗ (email) | ✗ (email) |
| No central server | ✓ | ✗ | ✗ | ✗ (federated) |
| No metadata leakage | ✓ | partial | ✗ | partial |
| Local-only identity | ✓ | ✗ | ✗ | ✗ |
| Voice chat | ✓ | ✓ | ✓ | ✓ |
| Open source | ✓ | ✓ | ✗ | ✓ |

---

## Trade-offs

Aether is honest about what it gives up:

- **Everyone needs to be online** — Messages catch up when you reconnect, but there's no server holding them for you while you're away.
- **Voice caps at 8 people** — Direct connections between every participant don't scale the way a central server does.
- **Your identity lives on your device** — If you lose your device without a backup, your identity is gone. There's no "forgot password" flow.
- **You control who gets the invite** — There's no central moderation. If you share the invite link with the wrong person, they're in.

---

## Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Tauri v2 [platform dependencies](https://v2.tauri.app/start/prerequisites/)
- **Linux only:** `libasound2-dev`

### Run

```bash
git clone https://github.com/0xLeathery/aether.git
cd aether
npm install
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

Installers are emitted to `src-tauri/target/release/bundle/`.

### Platform Support

| Platform | Status |
|----------|--------|
| macOS (10.13+) | Supported |
| Windows | Supported |
| Linux | Supported |

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

<details>
<summary>Project Structure</summary>

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

</details>

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

[Elastic License 2.0](LICENSE) — free to use, modify, and self-host. You may not offer Aether as a hosted or managed service to third parties.
