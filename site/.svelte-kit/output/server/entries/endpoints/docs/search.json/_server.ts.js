import { json } from "@sveltejs/kit";
const __vite_glob_0_0 = `---
title: Architecture Overview
description: How Aether's peer-to-peer architecture works -- component boundaries, protocol layers, and design principles.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# Architecture Overview

Aether is built on the principle that communication software should not require a central authority. Every instance of Aether is a **sovereign node** -- a fully independent peer that owns its own identity, stores its own data, and connects directly to other peers. There is no server to sign up for, no account to lose, and no infrastructure bill to pay.

This page provides a high-level view of how the pieces fit together. Each subsystem has its own deep-dive page linked below.

## Centralized vs Peer-to-Peer

Most communication apps route every message through a company's servers. Aether takes a different approach: peers connect directly.

<svg viewBox="0 0 800 300" class="w-full my-8" aria-label="Diagram comparing centralized star topology with Aether's peer-to-peer mesh topology" role="img">
  <!-- Left side: Centralized model -->
  <text x="200" y="24" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="14" font-weight="bold">Centralized</text>

  <!-- Server (center) -->
  <rect x="160" y="110" width="80" height="40" rx="6" fill="none" stroke="#a0a0a0" stroke-width="2"/>
  <text x="200" y="135" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="11">Server</text>

  <!-- Clients -->
  <circle cx="80" cy="60" r="20" fill="none" stroke="#a0a0a0" stroke-width="1.5"/>
  <text x="80" y="64" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="9">Client A</text>

  <circle cx="320" cy="60" r="20" fill="none" stroke="#a0a0a0" stroke-width="1.5"/>
  <text x="320" y="64" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="9">Client B</text>

  <circle cx="80" cy="210" r="20" fill="none" stroke="#a0a0a0" stroke-width="1.5"/>
  <text x="80" y="214" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="9">Client C</text>

  <circle cx="320" cy="210" r="20" fill="none" stroke="#a0a0a0" stroke-width="1.5"/>
  <text x="320" y="214" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="9">Client D</text>

  <!-- Lines to server -->
  <line x1="98" y1="72" x2="162" y2="114" stroke="#a0a0a0" stroke-width="1.5"/>
  <line x1="302" y1="72" x2="238" y2="114" stroke="#a0a0a0" stroke-width="1.5"/>
  <line x1="98" y1="198" x2="162" y2="148" stroke="#a0a0a0" stroke-width="1.5"/>
  <line x1="302" y1="198" x2="238" y2="148" stroke="#a0a0a0" stroke-width="1.5"/>

  <!-- Caption -->
  <text x="200" y="275" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="10">All messages route through the server.</text>
  <text x="200" y="290" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="10">Server owns the data.</text>

  <!-- Divider -->
  <line x1="400" y1="20" x2="400" y2="290" stroke="#2a2a2a" stroke-width="1" stroke-dasharray="4 4"/>

  <!-- Right side: P2P model -->
  <text x="600" y="24" text-anchor="middle" fill="#00ff41" font-family="monospace" font-size="14" font-weight="bold">Peer-to-Peer (Aether)</text>

  <!-- Peers -->
  <circle cx="520" cy="80" r="20" fill="none" stroke="#00ff41" stroke-width="1.5"/>
  <text x="520" y="84" text-anchor="middle" fill="#e0e0e0" font-family="monospace" font-size="9">Peer A</text>

  <circle cx="680" cy="80" r="20" fill="none" stroke="#00ff41" stroke-width="1.5"/>
  <text x="680" y="84" text-anchor="middle" fill="#e0e0e0" font-family="monospace" font-size="9">Peer B</text>

  <circle cx="520" cy="210" r="20" fill="none" stroke="#00ff41" stroke-width="1.5"/>
  <text x="520" y="214" text-anchor="middle" fill="#e0e0e0" font-family="monospace" font-size="9">Peer C</text>

  <circle cx="680" cy="210" r="20" fill="none" stroke="#00ff41" stroke-width="1.5"/>
  <text x="680" y="214" text-anchor="middle" fill="#e0e0e0" font-family="monospace" font-size="9">Peer D</text>

  <!-- Mesh connections -->
  <line x1="540" y1="80" x2="660" y2="80" stroke="#00ff41" stroke-width="1.5"/>
  <line x1="520" y1="100" x2="520" y2="190" stroke="#00ff41" stroke-width="1.5"/>
  <line x1="680" y1="100" x2="680" y2="190" stroke="#00ff41" stroke-width="1.5"/>
  <line x1="540" y1="210" x2="660" y2="210" stroke="#00ff41" stroke-width="1.5"/>
  <line x1="536" y1="93" x2="664" y2="197" stroke="#00ff41" stroke-width="1.5"/>

  <!-- Caption -->
  <text x="600" y="275" text-anchor="middle" fill="#e0e0e0" font-family="monospace" font-size="10">Peers connect directly.</text>
  <text x="600" y="290" text-anchor="middle" fill="#e0e0e0" font-family="monospace" font-size="10">Each node owns its own data.</text>
</svg>

| | Centralized | Aether (P2P) |
|---|---|---|
| **Data ownership** | Server operator | You |
| **Single point of failure** | Yes | No |
| **Infrastructure cost** | Server bills | $0 |
| **Account deletion risk** | Yes | Impossible |
| **Offline access** | Requires server | Requires peers |

## System Architecture

Aether is a desktop application built with [Tauri v2](https://tauri.app/). The Rust backend handles all networking, cryptography, and audio processing. The Svelte 5 frontend provides the user interface. They communicate through Tauri's IPC bridge.

<svg viewBox="0 0 800 500" class="w-full my-8" aria-label="Aether system architecture diagram showing OS layer, Rust backend, IPC bridge, and Svelte frontend" role="img">
  <!-- Background layers -->

  <!-- OS Layer -->
  <rect x="40" y="400" width="720" height="70" rx="8" fill="none" stroke="#2a2a2a" stroke-width="2"/>
  <text x="60" y="425" fill="#a0a0a0" font-family="monospace" font-size="12" font-weight="bold">OS Layer</text>

  <rect x="60" y="435" width="140" height="28" rx="4" fill="none" stroke="#ffb000" stroke-width="1.5"/>
  <text x="130" y="454" text-anchor="middle" fill="#ffb000" font-family="monospace" font-size="10">System Keychain</text>

  <rect x="220" y="435" width="140" height="28" rx="4" fill="none" stroke="#a0a0a0" stroke-width="1.5"/>
  <text x="290" y="454" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="10">Network Interface</text>

  <rect x="380" y="435" width="140" height="28" rx="4" fill="none" stroke="#a0a0a0" stroke-width="1.5"/>
  <text x="450" y="454" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="10">Audio Hardware</text>

  <!-- Rust Backend -->
  <rect x="40" y="220" width="720" height="150" rx="8" fill="none" stroke="#2a2a2a" stroke-width="2"/>
  <text x="60" y="248" fill="#e0e0e0" font-family="monospace" font-size="12" font-weight="bold">Rust Backend (Tauri v2)</text>

  <rect x="60" y="260" width="150" height="40" rx="4" fill="none" stroke="#00ff41" stroke-width="1.5"/>
  <text x="135" y="284" text-anchor="middle" fill="#00ff41" font-family="monospace" font-size="11">libp2p 0.56</text>
  <text x="135" y="296" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="9">networking</text>

  <rect x="230" y="260" width="150" height="40" rx="4" fill="none" stroke="#00ff41" stroke-width="1.5"/>
  <text x="305" y="284" text-anchor="middle" fill="#00ff41" font-family="monospace" font-size="11">Automerge</text>
  <text x="305" y="296" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="9">CRDTs</text>

  <rect x="400" y="260" width="150" height="40" rx="4" fill="none" stroke="#00ff41" stroke-width="1.5"/>
  <text x="475" y="284" text-anchor="middle" fill="#00ff41" font-family="monospace" font-size="11">Opus codec</text>
  <text x="475" y="296" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="9">voice</text>

  <rect x="570" y="260" width="170" height="40" rx="4" fill="none" stroke="#ffb000" stroke-width="1.5"/>
  <text x="655" y="284" text-anchor="middle" fill="#ffb000" font-family="monospace" font-size="11">Identity (Ed25519)</text>
  <text x="655" y="296" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="9">keychain</text>

  <rect x="60" y="315" width="320" height="35" rx="4" fill="none" stroke="#a0a0a0" stroke-width="1.5" stroke-dasharray="4 4"/>
  <text x="220" y="337" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="10">PSK Swarm Isolation (XSalsa20)</text>

  <!-- IPC Bridge -->
  <rect x="40" y="170" width="720" height="35" rx="8" fill="none" stroke="#2a2a2a" stroke-width="2"/>
  <text x="400" y="192" text-anchor="middle" fill="#a0a0a0" font-family="monospace" font-size="12" font-weight="bold">IPC Bridge -- Tauri Commands</text>

  <!-- Svelte Frontend -->
  <rect x="40" y="30" width="720" height="120" rx="8" fill="none" stroke="#2a2a2a" stroke-width="2"/>
  <text x="60" y="58" fill="#e0e0e0" font-family="monospace" font-size="12" font-weight="bold">Svelte 5 Frontend</text>

  <rect x="60" y="70" width="130" height="35" rx="4" fill="none" stroke="#a0a0a0" stroke-width="1.5"/>
  <text x="125" y="92" text-anchor="middle" fill="#e0e0e0" font-family="monospace" font-size="10">UI Components</text>

  <rect x="210" y="70" width="130" height="35" rx="4" fill="none" stroke="#a0a0a0" stroke-width="1.5"/>
  <text x="275" y="92" text-anchor="middle" fill="#e0e0e0" font-family="monospace" font-size="10">Svelte Stores</text>

  <rect x="360" y="70" width="130" height="35" rx="4" fill="none" stroke="#a0a0a0" stroke-width="1.5"/>
  <text x="425" y="92" text-anchor="middle" fill="#e0e0e0" font-family="monospace" font-size="10">SvelteKit Router</text>

  <rect x="510" y="70" width="130" height="35" rx="4" fill="none" stroke="#a0a0a0" stroke-width="1.5"/>
  <text x="575" y="92" text-anchor="middle" fill="#e0e0e0" font-family="monospace" font-size="10">Rune Reactivity</text>

  <!-- Data flow arrows -->
  <!-- Keychain -> Identity -->
  <line x1="130" y1="435" x2="655" y2="305" stroke="#ffb000" stroke-width="1" stroke-dasharray="3 3"/>

  <!-- Network -> libp2p -->
  <line x1="290" y1="435" x2="135" y2="305" stroke="#a0a0a0" stroke-width="1" stroke-dasharray="3 3"/>

  <!-- Audio -> Opus -->
  <line x1="450" y1="435" x2="475" y2="305" stroke="#a0a0a0" stroke-width="1" stroke-dasharray="3 3"/>

  <!-- Rust -> IPC -->
  <line x1="400" y1="220" x2="400" y2="205" stroke="#a0a0a0" stroke-width="1.5" marker-end="url(#arrowhead)"/>

  <!-- IPC -> Frontend -->
  <line x1="400" y1="170" x2="400" y2="150" stroke="#a0a0a0" stroke-width="1.5" marker-end="url(#arrowhead)"/>

  <!-- Arrow marker definition -->
  <defs>
    <marker id="arrowhead" markerWidth="8" markerHeight="6" refX="8" refY="3" orient="auto">
      <polygon points="0 0, 8 3, 0 6" fill="#a0a0a0"/>
    </marker>
  </defs>
</svg>

## Component Deep Dives

Each subsystem is documented in detail on its own page.

- **[Networking](/docs/architecture/networking)** -- \`libp2p\` transport, Kademlia DHT peer discovery, mDNS for LAN, and NAT traversal strategies.

- **[Identity](/docs/architecture/identity)** -- Ed25519 keypairs stored in the OS system keychain, self-asserted display names, and the petname system.

- **[CRDTs](/docs/architecture/crdts)** -- Automerge documents for chat messages and channel metadata. Eventual consistency without coordination.

- **[Voice](/docs/architecture/voice)** -- Opus codec at 48kbps, full-mesh P2P audio topology, and the 8-peer limit.

- **[Encryption](/docs/architecture/encryption)** -- PSK swarm isolation, transport-layer encryption, and an honest accounting of what is and is not protected.

## Design Principles

These principles guide every architectural decision in Aether.

**Sovereign by default.** No feature depends on a central server. If the internet goes down, peers on the same LAN can still communicate.

**Transport-only encryption.** Data is encrypted in transit via PSK and the noise protocol. Local data stays accessible to the user -- no opaque encrypted blobs on your own machine. See the [encryption page](/docs/architecture/encryption) for the full scope and limitations.

**Hardware-backed identity.** Your Ed25519 private key is stored in the OS system keychain and never leaves the device. There are no seed phrases and no cloud backup. Your identity is bound to your hardware.

**Pure sync.** Data moves only when peers are connected. There is no background sync service, no push notifications, and no always-on relay. When you close Aether, nothing happens until you open it again.

**Desktop-first.** Aether is optimized for a single high-performance node running on macOS, Windows, or Linux. Mobile support is a future consideration, not a current constraint.
`;
const __vite_glob_0_1 = `---
title: CRDTs
description: How Aether uses Automerge CRDTs for conflict-free data synchronization between peers.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# CRDTs (Conflict-Free Replicated Data Types)

> This is a simplified overview of Aether's data synchronization model, not a formal CRDT specification.

In a peer-to-peer system, there is no central server to decide the "correct" state of the data. Two peers might make changes at the same time while disconnected. When they reconnect, those changes need to merge without conflicts. This is the problem CRDTs solve.

## What Are CRDTs

A **Conflict-Free Replicated Data Type** is a data structure that can be modified independently by multiple peers and merged automatically -- without coordination and without conflicts. Changes always converge to the same state regardless of the order they are applied.

Traditional databases use locking or consensus protocols to prevent conflicts. CRDTs take the opposite approach: they are mathematically designed so that conflicts are impossible. Every valid merge produces the same result.

## Automerge

Aether uses the **Automerge** library (Rust crate with \`derive\` macros) for all CRDT operations. Automerge provides a document model similar to JSON, but with built-in merge semantics.

Each piece of shared state in Aether -- chat history, channel metadata -- is an Automerge document. These documents are the single source of truth.

## How Sync Works

1. **Local-first** -- Changes are applied to your local copy of the document immediately. There is no "saving to server" step.
2. **Peer connection** -- When two peers connect (via [libp2p](/docs/architecture/networking)), they exchange Automerge sync messages.
3. **Merge** -- Each peer integrates the other's changes into their local document. Automerge's merge algorithm ensures both arrive at the same state.
4. **Eventual consistency** -- All peers converge to the same state, but there may be a delay while they are disconnected. This is the "eventual" part: consistency is guaranteed, but not instantaneous.

Data moves only when peers are online and connected. There is no background sync service and no push notifications. If you close Aether, your data stays frozen until you open it again and reconnect with peers.

## Chat Messages

Each chat message is an entry in an Automerge list. A message contains:

- **Sender** -- The public key of the peer who wrote it (see [Identity](/docs/architecture/identity))
- **Timestamp** -- When the message was created (local clock)
- **Content** -- The message text

New messages are appended to the list. When two peers send messages at the same time, Automerge's list merge semantics place them in a deterministic order. There is no server-side ordering -- the CRDT handles it.

## Channel Metadata

Channel names, creation timestamps, and archive status are stored in an Automerge map. Changes to channels (rename, archive) sync through the same mechanism as chat messages.

## Trade-offs

CRDTs are not free. There are costs to be aware of:

**History growth** -- Automerge tracks the full edit history of a document to enable merging. This means CRDT documents grow over time. Aether does not compact or garbage-collect old state in v1. For small-to-medium groups (the intended use case), this is manageable. Very large histories with thousands of messages may eventually impact performance.

**No deletion guarantees** -- In a peer-to-peer system, you cannot force all peers to delete data. A "delete" operation is really a "mark as deleted" -- the tombstone must be retained so other peers know to apply the deletion when they sync.

**Clock skew** -- Message timestamps use local device clocks. If a peer's clock is wrong, their messages may appear out of order. Automerge's causal ordering mitigates this for concurrent edits, but display order still relies on timestamps.

## Related Pages

- [Architecture Overview](/docs/architecture) -- Where CRDTs fit in the system stack
- [Networking](/docs/architecture/networking) -- How sync messages are transported between peers
- [Encryption](/docs/architecture/encryption) -- How CRDT data is protected in transit (and not at rest)
`;
const __vite_glob_0_2 = `---
title: Encryption
description: What Aether encrypts, what it does not, and why -- an honest accounting of the security model.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# Encryption

> This page documents Aether's encryption scope honestly, including its limitations. Aether provides transport-only encryption -- not end-to-end encryption in the Signal or WhatsApp sense.

## What IS Encrypted

### Network Transport

All peer-to-peer traffic is encrypted at the \`libp2p\` transport layer using **PSK (Pre-Shared Key) encryption with XSalsa20**. An eavesdropper monitoring your network traffic cannot read your messages, hear your voice calls, or see your channel metadata.

This encryption is always on. There is no unencrypted mode.

### Swarm Isolation

Each swarm's PSK acts as both an access control mechanism and an encryption key. Only peers who possess the correct 256-bit key can participate in the swarm. Non-members cannot even establish a connection -- the PSK handshake fails at the transport layer before any application data is exchanged.

### Authentication

Ed25519 key signing ensures that messages come from who they claim to be from. Each peer's [identity](/docs/architecture/identity) is verified cryptographically. Spoofing another peer's identity would require possessing their private key, which never leaves their device.

## What is NOT Encrypted

### Local Storage is Not Encrypted

Chat messages, channel metadata, and [CRDT](/docs/architecture/crdts) state are stored in **cleartext on your local filesystem**. Anyone with access to your device can read your messages.

This is a deliberate v1 design choice. Cleartext local storage enables:
- Full-text search across message history
- Easier debugging during development
- Simpler implementation with fewer failure modes

The trade-off is explicit: if someone gains access to your machine (physically or via malware), they can read your Aether data. Your OS login credentials are your primary defense for local data.

### No End-to-End Encryption for Message Content

The PSK encrypts the transport, but **all swarm members share the same key**. There is no per-message encryption between specific peers. Every member of a swarm can read every message in that swarm.

Think of it as a room with a locked door: you need the key to enter, but once inside, everyone can hear everything. This is the intended model for group communication -- a shared space, not a set of private channels.

### No Forward Secrecy

If a swarm's PSK is compromised, past network traffic that was captured by an eavesdropper could theoretically be decrypted. The key does not rotate automatically.

**Mitigation:** Create a new swarm with a new key. The old swarm's key remains compromised, but the new swarm starts with a fresh secret.

### No At-Rest Encryption

The OS system keychain protects your Ed25519 private key, but your **message database is not encrypted on disk**. This is distinct from the "local storage" point above -- it applies to all persisted application data, including CRDT documents and any cached state.

## Security Model Summary

Aether uses a **"locked room" model**:

| Layer | Protection | Status |
|-------|-----------|--------|
| Network transport | XSalsa20 via PSK | Encrypted |
| Swarm membership | PSK handshake | Protected |
| Peer authentication | Ed25519 signing | Verified |
| Local message storage | None | Cleartext |
| Database on disk | None | Cleartext |
| Per-message encryption | None | Not implemented |
| Forward secrecy | None | Not implemented |

This is **transport-only encryption**. Data is protected while it moves between peers. Data is not protected where it is stored.

For Aether's intended use case -- small trusted groups who want to communicate without depending on a server -- this model is appropriate. The PSK ensures outsiders cannot access the swarm. Within the swarm, trust is assumed.

For communications requiring forward secrecy, at-rest encryption, or per-message confidentiality, additional cryptographic layers would be needed.

## Future Improvements

These are planned but **not yet implemented**:

- **At-rest database encryption** -- Encrypt the local message store, requiring a user-provided passphrase to unlock
- **Per-peer encrypted channels** -- Double ratchet or similar protocol for private messages between specific peers within a swarm
- **Forward secrecy via key rotation** -- Automatic PSK rotation to limit the window of compromise

## Related Pages

- [Architecture Overview](/docs/architecture) -- Where encryption fits in the system stack
- [Networking](/docs/architecture/networking) -- Transport layer and PSK swarm isolation
- [Identity](/docs/architecture/identity) -- Ed25519 keys and keychain storage
`;
const __vite_glob_0_3 = `---
title: Identity
description: How Aether creates and manages cryptographic identities using Ed25519 keys and the OS system keychain.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# Identity System

> This is a simplified overview of Aether's identity model, not a cryptographic specification.

Every Aether peer has a unique cryptographic identity. There is no sign-up flow, no email address, and no username registry. Your identity is a keypair generated on your device and stored in your operating system's secure keychain.

## Ed25519 Keypair

Aether uses **Ed25519** (elliptic curve cryptography) for identity. When you first launch Aether, the app generates a public/private keypair:

- **Public key** -- Your globally unique identifier. Shared with every peer you connect to. The 256-bit key space makes collisions effectively impossible.
- **Private key** -- Proves you are who you claim to be. Used to sign messages. Never transmitted, never exported.

Ed25519 was chosen for its speed (thousands of signatures per second), small key size (32 bytes), and resistance to timing attacks.

## Keychain Storage

The private key is stored in the **OS system keychain**:

- **macOS** -- Keychain Services (with ACLs configured to avoid password prompts in production builds)
- **Windows** -- Credential Manager
- **Linux** -- Secret Service API (GNOME Keyring, KDE Wallet, etc.)

The keychain provides hardware-backed or OS-level protection depending on the platform. On Macs with a Secure Enclave, the key benefits from hardware isolation. On all platforms, the key is protected by the user's OS login credentials.

## Hardware Binding

Aether's identity is **bound to the device**. This is a deliberate design choice:

- No seed phrases
- No cloud backup
- No export functionality
- No multi-device identity sync

If you lose the device, you lose the identity. This is the trade-off: **security over convenience**. A key that cannot be exported cannot be stolen remotely. There is no "forgot password" flow because there is no password -- only the key on your hardware.

For users who need presence on multiple devices, each device has its own independent identity.

## Display Names

Names in Aether are **self-asserted**. You can claim any display name you want. There is no uniqueness enforcement and no central registry to check against.

This means two peers could both call themselves "Alice." Rather than solving this with a central authority, Aether uses **petnames**: any peer can assign a local alias to any other peer. If your friend calls themselves "xX_DarkLord_Xx" but you know them as "Dave," you can override their name locally. Only you see the petname -- it is not transmitted.

This is the sovereign naming model: no central authority decides what you are called. Your self-asserted name is a suggestion; each peer decides what to display.

## Verification

Peers are ultimately identified by their **public key**, not their display name. When no display name is set, Aether falls back to a truncated version of the public key as an identifier. This ensures every peer always has a unique, unforgeable label -- even if display names collide.

## Related Pages

- [Architecture Overview](/docs/architecture) -- Where identity fits in the system stack
- [Encryption](/docs/architecture/encryption) -- How keys are used for authentication and transport security
- [Networking](/docs/architecture/networking) -- How peers discover and connect to each other
`;
const __vite_glob_0_4 = "---\ntitle: Networking\ndescription: How Aether peers discover each other, establish connections, and communicate using libp2p.\n---\n\n<svelte:head>\n  <title>{title} - Aether Docs</title>\n  <meta name=\"description\" content={description} />\n</svelte:head>\n\n# Networking Protocol\n\n> This is a simplified overview of Aether's networking layer, not an exhaustive protocol specification.\n\nAether's networking is built on **`libp2p` 0.56** (Rust implementation), a modular peer-to-peer networking framework originally created for IPFS. `libp2p` handles peer discovery, connection management, and protocol multiplexing -- letting Aether focus on application logic rather than socket programming.\n\n## Transport\n\nAether uses **TCP** as its transport protocol. This is a deliberate choice: Aether's swarm isolation relies on Pre-Shared Key (PSK) encryption using XSalsa20, which operates at the transport layer. QUIC -- the more modern alternative -- has its own TLS layer built in, and that TLS layer conflicts with PSK encryption. TCP avoids this conflict.\n\nAll connections are direct peer-to-peer. There are no relay servers in the current architecture.\n\n## Peer Discovery\n\nFinding other peers happens through two mechanisms:\n\n**Kademlia DHT (Distributed Hash Table)** -- For internet-wide discovery. Peers announce themselves to the DHT and query it to find others who share the same swarm key. The DHT is distributed across all participating peers, so there is no central directory.\n\n**mDNS (Multicast DNS)** -- For local network discovery. If two Aether peers are on the same LAN, they can find each other without any internet connectivity. This is especially useful for offline or air-gapped environments.\n\n## NAT Traversal\n\nMost home networks sit behind a NAT (Network Address Translation) router, which makes direct incoming connections difficult. Aether uses STUN-like techniques for UDP holepunching to establish direct connections through NAT.\n\nThis works for most NAT types (full cone, restricted cone, port-restricted cone). However, **symmetric NAT** -- found in approximately 10-20% of consumer networks -- prevents holepunching. A TURN relay would solve this but is not currently implemented, as it requires server infrastructure (which has a cost). This is a known limitation.\n\n## PSK Swarm Isolation\n\nEach Aether swarm is protected by a **Pre-Shared Key** -- a 256-bit secret that acts as a network-level access control. The PSK enables XSalsa20 stream encryption via `libp2p`'s `PnetConfig` (Private Network Configuration).\n\nThis means:\n\n- Only peers who possess the correct PSK can participate in the swarm\n- Encryption happens at the transport layer, before any application data is exchanged\n- A peer without the key cannot even establish a connection -- the handshake fails immediately\n\nThink of it as a locked room: you need the key to enter. Everyone inside can communicate freely. See the [Encryption](/docs/architecture/encryption) page for the full security model and its limitations.\n\n## Connection Lifecycle\n\n1. **Bootstrap** -- Peer starts and connects to known DHT bootstrap nodes (or discovers local peers via mDNS)\n2. **Announce** -- Peer registers itself on the DHT with its swarm identifier\n3. **Discover** -- Peer queries the DHT for other peers in the same swarm\n4. **Connect** -- Direct TCP connection established with PSK handshake\n5. **Multiplex** -- Multiple protocols (chat sync, voice, control messages) run over a single connection\n\n## Related Pages\n\n- [Architecture Overview](/docs/architecture) -- High-level system diagram\n- [Encryption](/docs/architecture/encryption) -- PSK security model and limitations\n- [Voice](/docs/architecture/voice) -- Audio streaming over `libp2p`\n- [CRDTs](/docs/architecture/crdts) -- Data sync over established connections\n";
const __vite_glob_0_5 = "---\ntitle: Voice\ndescription: How Aether handles real-time voice communication with Opus codec and P2P mesh topology.\n---\n\n<svelte:head>\n  <title>{title} - Aether Docs</title>\n  <meta name=\"description\" content={description} />\n</svelte:head>\n\n# Voice Protocol\n\n> This is a simplified overview of Aether's voice system, not a real-time audio engineering specification.\n\nAether supports real-time voice chat between peers, with no mixing server or relay infrastructure. Audio is captured, encoded, transmitted, decoded, and played back entirely through direct peer-to-peer connections.\n\n## Opus Codec\n\nAether uses the **Opus** audio codec at approximately **48kbps**. Opus is an open standard designed specifically for real-time speech, offering excellent voice quality at low bitrate. It is the same codec used by Discord, WebRTC, and most modern VoIP applications.\n\nThe target end-to-end latency is under **50ms on LAN** -- fast enough that conversation feels natural with no perceptible delay.\n\n## Mesh Topology\n\nVoice sessions use a **full mesh** topology: each peer in a voice channel sends their audio stream directly to every other participant.\n\nWith N peers in a voice session:\n- Each peer sends **N-1** outgoing audio streams\n- Each peer receives **N-1** incoming audio streams\n- Total streams across all peers: **N x (N-1)**\n\nThis scales quadratically, which is why Aether enforces an **8-peer hard limit** for voice sessions. At 8 peers, each participant handles 7 streams -- manageable for modern hardware and home network connections. Beyond 8, bandwidth and CPU requirements become impractical without a mixing server (which Aether intentionally avoids).\n\n| Peers | Streams per peer | Total streams |\n|-------|-----------------|---------------|\n| 2 | 1 | 2 |\n| 4 | 3 | 12 |\n| 6 | 5 | 30 |\n| 8 | 7 | 56 |\n\n## Audio Pipeline\n\nThe full audio path, all processed in the Rust backend:\n\n1. **Capture** -- Microphone audio captured via `cpal` (cross-platform audio library) or system audio APIs\n2. **Encode** -- Raw PCM audio compressed with the Opus encoder at 48kbps\n3. **Transmit** -- Encoded packets sent over `libp2p` streams to each connected peer\n4. **Receive** -- Incoming packets from each peer arrive over separate `libp2p` streams\n5. **Decode** -- Each peer's audio decoded from Opus back to PCM\n6. **Mix** -- Decoded audio from all peers mixed into a single output stream\n7. **Playback** -- Mixed audio sent to the system speaker/headphone output\n\nAll encoding, decoding, and mixing happens in the Rust backend. The Svelte frontend only handles the UI (mute button, voice channel membership display).\n\n## Mute\n\nMuting **stops sending audio packets** -- the microphone is not captured and no data is transmitted. Other peers are notified of your mute state via a control message, so the UI can display who is muted. Unmuting resumes audio capture and transmission.\n\n## Connection Model\n\nVoice sessions are per-channel. When you join a voice channel:\n\n1. Aether initiates connections to all other participants currently in the channel\n2. Audio streams begin flowing in both directions\n3. When a new peer joins, existing participants establish new streams with them\n4. When you leave, all your streams stop\n\n## Limitations\n\nThe current voice implementation has known limitations:\n\n- **No echo cancellation** -- Use headphones to avoid feedback loops. This is the most important practical consideration.\n- **No noise suppression** -- Background noise is transmitted as-is.\n- **No automatic gain control** -- Peers with different microphone volumes may sound louder or quieter.\n- **8-peer limit** -- The full mesh topology does not scale beyond 8 participants.\n\nThese are targeted for future improvement. Echo cancellation and noise suppression are the highest-priority audio enhancements.\n\n## Related Pages\n\n- [Architecture Overview](/docs/architecture) -- Where voice fits in the system stack\n- [Networking](/docs/architecture/networking) -- Transport layer that carries audio streams\n- [Encryption](/docs/architecture/encryption) -- How voice data is protected in transit\n";
const __vite_glob_0_6 = '---\ntitle: Getting Started\ndescription: Install Aether, create your cryptographic identity, and connect to your first peer in minutes.\n---\n\n<svelte:head>\n  <title>{title} - Aether Docs</title>\n  <meta name="description" content={description} />\n</svelte:head>\n\n# Getting Started\n\nAether is a peer-to-peer chat application with no servers, no accounts, and no data collection. This guide walks you through installation, identity creation, and connecting with your first peer.\n\n## Install Aether\n\nDownload the latest release for your platform:\n\n- **macOS** -- Apple Silicon (.dmg) and Intel (.dmg)\n- **Windows** -- 64-bit installer (.msi)\n- **Linux** -- AppImage (.AppImage)\n\nVisit the [Download page](/download) for direct links. Aether runs on any modern operating system and requires no special hardware.\n\n## First Launch\n\nWhen you open Aether for the first time, the app automatically generates your **Ed25519 cryptographic identity** and stores it securely in your system keychain:\n\n- **macOS** -- Keychain Access\n- **Windows** -- Credential Manager\n- **Linux** -- Secret Service (GNOME Keyring / KWallet)\n\nThere are no accounts, no passwords, and no email verification. Your identity is a keypair -- a **public key** (your address that peers see) and a **private key** (which never leaves your device).\n\n## Set Your Display Name\n\nChoose a display name that other peers will see when you chat. You can pick any name you like -- there is no central registry enforcing uniqueness.\n\nPeers can set local "petnames" to override your display name on their end. This is the sovereign model: you choose how you present yourself, and others choose how they label you.\n\n## Create Your First Swarm\n\nA **swarm** is a private peer group. Think of it like a server, except there is no server -- just peers who share the same secret key.\n\nWhen you create a swarm, Aether generates a cryptographically random key in the format `aether://<64-hex-chars>`. This key **is** the swarm -- anyone who has it can join.\n\nFor more details on managing swarms, see the [Swarms guide](/docs/guides/swarms).\n\n## Invite a Peer\n\nShare your swarm\'s secret code with someone you trust. They paste the `aether://` code into the "Join Swarm" dialog and connect automatically via peer discovery.\n\nSend the code through a secure channel -- encrypted messenger, in person, or any method you trust. The code is sensitive: anyone who has it can join your swarm.\n\nFor the full invitation workflow, see the [Inviting Peers guide](/docs/guides/inviting-peers).\n\n## Next Steps\n\nYou are up and running. Explore the User Guides to learn about every feature:\n\n- **[Swarms](/docs/guides/swarms)** -- Create and manage private peer groups\n- **[Inviting Peers](/docs/guides/inviting-peers)** -- Share secret codes to connect\n- **[Channels](/docs/guides/channels)** -- Organize conversations within swarms\n- **[Voice Chat](/docs/guides/voice-chat)** -- Real-time P2P voice sessions\n- **[Moderation](/docs/guides/moderation)** -- Control your experience with hide and block\n';
const __vite_glob_0_7 = '---\ntitle: User Guides\ndescription: Feature guides for Aether -- swarms, inviting peers, channels, voice chat, and moderation.\n---\n\n<svelte:head>\n  <title>{title} - Aether Docs</title>\n  <meta name="description" content={description} />\n</svelte:head>\n\n# User Guides\n\nThese guides cover every core feature of Aether. Each one is a short, focused walkthrough you can read in a few minutes.\n\n- **[Swarms](/docs/guides/swarms)** -- Create private peer groups and manage your communities. Learn how swarm keys work, how to join and leave swarms, and how to keep them secure.\n\n- **[Inviting Peers](/docs/guides/inviting-peers)** -- Share secret codes to connect with others. No friend requests, no usernames -- just a code and a direct connection.\n\n- **[Channels](/docs/guides/channels)** -- Organize conversations within a swarm. Create, rename, and delete channels. Understand how messages sync between peers via CRDTs.\n\n- **[Voice Chat](/docs/guides/voice-chat)** -- Start real-time P2P voice sessions with peers in your swarm. Learn about the mesh architecture, peer limits, and audio quality.\n\n- **[Moderation](/docs/guides/moderation)** -- Control your experience with hide and block. In a P2P system, moderation is local and sovereign -- you decide what you see.\n';
const __vite_glob_0_8 = '---\ntitle: Channels\ndescription: Organize conversations within Aether swarms with channels synced via CRDTs.\n---\n\n<svelte:head>\n  <title>{title} - Aether Docs</title>\n  <meta name="description" content={description} />\n</svelte:head>\n\n# Channels\n\nChannels are organized conversation spaces within a [swarm](/docs/guides/swarms). They work like Discord channels, but sync between peers via `CRDTs` -- no server needed.\n\n## Default Channel\n\nEvery swarm starts with a **general** channel. This channel cannot be renamed or deleted. It is always there as a fallback conversation space.\n\n## Creating Channels\n\nName a new channel and it syncs to all swarm peers automatically via `CRDT` metadata. Every peer who is connected (or comes online later) sees the new channel appear.\n\n## Renaming Channels\n\nChange a channel name and the rename propagates to all peers. Channel identity is tracked internally, so renaming does not break message history.\n\n## Deleting Channels\n\nDeleting a channel requires confirmation. The channel is archived and can potentially be restored if any peer still has the `CRDT` state. Deletion syncs to all connected peers.\n\n## Message History\n\nMessages are stored locally on your device and sync between peers when connected. If you were offline, you catch up automatically when you reconnect -- this is **eventual consistency** via Automerge `CRDTs`.\n\nThere is no central message store. If all peers delete their local data, the messages are gone. This is by design: your data lives on your device, not on someone else\'s server.\n\n## Limitations\n\nMessages are **cleartext on disk** -- Aether uses transport-only encryption (encrypted in transit, not at rest). Anyone with access to your device can read your local message history.\n';
const __vite_glob_0_9 = "---\ntitle: Inviting Peers\ndescription: Share secret codes to connect with peers in your Aether swarms.\n---\n\n<svelte:head>\n  <title>{title} - Aether Docs</title>\n  <meta name=\"description\" content={description} />\n</svelte:head>\n\n# Inviting Peers\n\nAether has no friend requests, no usernames, and no email invitations. To invite someone, you share your swarm's secret code and they join directly.\n\n## Getting the Invite Code\n\nOpen a swarm and click **Invite** to copy the `aether://` code to your clipboard. This code contains the swarm's `PSK` (Pre-Shared Key) -- the only credential needed to join.\n\n## Sharing the Code\n\nSend the code through any secure channel you trust: an encrypted messenger, a face-to-face conversation, or any private method. The code is sensitive -- anyone who has it can join your swarm.\n\nAvoid posting invite codes in public channels unless you want an open swarm.\n\n## Joining with a Code\n\nThe recipient clicks **Join Swarm** and pastes the `aether://` code. Aether connects automatically via peer discovery over `DHT`. Peers on the same local network connect without internet.\n\n## Regenerating Invite Codes\n\nYou can generate a new shareable link for the same swarm. Note that old codes still work -- the underlying `PSK` does not change. Regeneration creates a fresh link format, not a new key.\n\n## How Connections Work\n\nPeers discover each other via a distributed hash table (`DHT`) and connect directly, peer-to-peer. There is no relay server in the middle. Once connected, messages sync via [CRDT](/docs/architecture/crdts)-based replication.\n\nFor more on what happens after peers connect, see the [Channels guide](/docs/guides/channels).\n";
const __vite_glob_0_10 = "---\ntitle: Moderation\ndescription: Control your Aether experience with local hide and block tools. Sovereign moderation for a P2P world.\n---\n\n<svelte:head>\n  <title>{title} - Aether Docs</title>\n  <meta name=\"description\" content={description} />\n</svelte:head>\n\n# Moderation\n\nIn a peer-to-peer system, there is no admin to ban people. Instead, **you** control **your** experience. Moderation in Aether is local and sovereign.\n\n## Hiding a Peer\n\nHiding a peer filters their messages from your view. It is reversible -- unhide anytime to see their messages again. The hidden peer does not know they have been hidden.\n\nHidden messages are still stored locally (they sync via `CRDTs` like any other message). Hiding only affects your display -- it does not delete anything.\n\n## Blocking a Peer\n\nBlocking goes further than hiding. A blocked peer's messages are hidden **and** their voice audio is refused. You will not hear them in [voice sessions](/docs/guides/voice-chat), and they will not hear you.\n\nLike hiding, blocking is reversible and the blocked peer does not know they have been blocked.\n\n## Managing Your List\n\nView your hidden and blocked peers in the moderation settings. Unhide or unblock anyone at any time. Changes take effect immediately.\n\n## Limitations\n\nHide and block are **local only**. Other peers in the swarm still see the person's messages and hear their voice. There is no global ban mechanism -- this is by design.\n\nIf someone is disruptive to your entire group, the recommended approach is to create a new [swarm](/docs/guides/swarms) and re-invite only the peers you trust. This mirrors how trust works in the real world: you choose who you spend time with.\n";
const __vite_glob_0_11 = '---\ntitle: Swarms\ndescription: Create private peer groups, share secret keys, and manage your Aether communities.\n---\n\n<svelte:head>\n  <title>{title} - Aether Docs</title>\n  <meta name="description" content={description} />\n</svelte:head>\n\n# Swarms\n\nA **swarm** is a private peer group identified by a shared secret key (a `PSK` -- Pre-Shared Key). Think of it like a private server, except there is no server. Just peers who share the same key.\n\n## Creating a Swarm\n\nWhen you create a swarm, Aether generates a cryptographically random 256-bit key encoded as an `aether://<64-hex-chars>` URI. This key **is** the swarm -- whoever has it can join. There is no registration, no approval process.\n\n## Joining a Swarm\n\nPaste the `aether://` code that another member shared with you. Aether connects you to the swarm automatically via peer discovery over `DHT` (distributed hash table). This works on the same LAN without internet.\n\n## Your Swarm List\n\nAll your swarms appear in the left sidebar. Click to switch between them. Each swarm has its own set of [channels](/docs/guides/channels) and peers.\n\n## Renaming a Swarm\n\nSwarm names are **local only**. Your name for a swarm is yours alone -- other peers name it whatever they want. There is no "official" name because there is no central authority.\n\n## Leaving a Swarm\n\nLeaving removes all local data for that swarm. You can rejoin later if you still have the secret code.\n\n## Security\n\nAnyone with the secret code can join your swarm. Treat the code like a password. If it gets shared with someone you do not trust, create a new swarm and re-invite only the peers you want.\n\nThere is no way to "revoke" a code or kick someone out. This is a trade-off of the P2P model -- the swarm key is the only credential, and there is no server to enforce access control. See [Moderation](/docs/guides/moderation) for how to manage your experience locally.\n';
const __vite_glob_0_12 = '---\ntitle: Voice Chat\ndescription: Real-time peer-to-peer voice sessions using Opus codec and WebRTC mesh networking.\n---\n\n<svelte:head>\n  <title>{title} - Aether Docs</title>\n  <meta name="description" content={description} />\n</svelte:head>\n\n# Voice Chat\n\nAether voice chat is direct peer-to-peer audio using the `Opus` codec. There is no voice server -- audio streams directly between connected peers via `WebRTC`.\n\n## Joining a Voice Session\n\nClick the voice button in a channel to join. Your microphone activates and you connect to other peers already in the session. Audio flows immediately.\n\n## Mute and Unmute\n\nToggle your microphone with the mute button. Your mute state is visible to other peers so they know when you are not transmitting.\n\n## Peer Limit\n\nVoice sessions have a hard limit of **8 simultaneous participants**. This is a mesh network constraint -- each peer sends audio to every other peer directly. At 8 peers, that is 56 audio streams, which approaches the practical bandwidth ceiling.\n\n## Audio Quality\n\nAether uses the `Opus` codec at approximately 48kbps per stream. On a local network, expect latency under 50ms. Over the internet, latency varies depending on the distance between peers.\n\n## Leaving Voice\n\nClick the voice button again or switch channels. Audio stops immediately and other peers see you leave the session.\n\n## Troubleshooting\n\nIf voice does not connect, check these common issues:\n\n- **Microphone permissions** -- Your OS may need to grant Aether access to the microphone.\n- **Firewall or NAT** -- Aether uses `STUN` to traverse NAT, but symmetric NAT (common on corporate networks) may prevent direct connections. Approximately 10-20% of users behind symmetric NAT may experience connection issues.\n- **No relay fallback** -- Aether does not use `TURN` relay servers. If a direct peer-to-peer path cannot be established, voice will not connect. This is a trade-off of the zero-server architecture.\n';
const prerender = true;
function stripMarkdown(raw) {
  let text = raw;
  text = text.replace(/^---[\s\S]*?---\n*/m, "");
  text = text.replace(/<svelte:head>[\s\S]*?<\/svelte:head>\n*/g, "");
  const titleMatch = text.match(/^#\s+(.+)$/m);
  const title = titleMatch ? titleMatch[1].trim() : "";
  text = text.replace(/^#{1,6}\s+/gm, "");
  text = text.replace(/\[([^\]]+)\]\([^)]+\)/g, "$1");
  text = text.replace(/!\[([^\]]*)\]\([^)]+\)/g, "$1");
  text = text.replace(/<[^>]+>/g, "");
  text = text.replace(/[*_`~]/g, "");
  text = text.replace(/^>\s*/gm, "");
  text = text.replace(/^---+$/gm, "");
  text = text.replace(/^[\s]*[-*+]\s+/gm, "");
  text = text.replace(/^[\s]*\d+\.\s+/gm, "");
  text = text.replace(/\n{2,}/g, " ");
  text = text.replace(/\s+/g, " ");
  text = text.trim();
  return { title, content: text.slice(0, 500) };
}
const GET = async () => {
  const modules = /* @__PURE__ */ Object.assign({
    "/src/routes/docs/architecture/+page.md": __vite_glob_0_0,
    "/src/routes/docs/architecture/crdts/+page.md": __vite_glob_0_1,
    "/src/routes/docs/architecture/encryption/+page.md": __vite_glob_0_2,
    "/src/routes/docs/architecture/identity/+page.md": __vite_glob_0_3,
    "/src/routes/docs/architecture/networking/+page.md": __vite_glob_0_4,
    "/src/routes/docs/architecture/voice/+page.md": __vite_glob_0_5,
    "/src/routes/docs/getting-started/+page.md": __vite_glob_0_6,
    "/src/routes/docs/guides/+page.md": __vite_glob_0_7,
    "/src/routes/docs/guides/channels/+page.md": __vite_glob_0_8,
    "/src/routes/docs/guides/inviting-peers/+page.md": __vite_glob_0_9,
    "/src/routes/docs/guides/moderation/+page.md": __vite_glob_0_10,
    "/src/routes/docs/guides/swarms/+page.md": __vite_glob_0_11,
    "/src/routes/docs/guides/voice-chat/+page.md": __vite_glob_0_12
  });
  const entries = [];
  for (const [path, raw] of Object.entries(modules)) {
    const href = path.replace("/src/routes", "").replace("/+page.md", "") || "/docs";
    if (href === "/docs") continue;
    const { title, content } = stripMarkdown(raw);
    entries.push({
      title: title || href.split("/").pop() || "Untitled",
      href,
      content
    });
  }
  return json(entries);
};
export {
  GET,
  prerender
};
