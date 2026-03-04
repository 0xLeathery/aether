---
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

- **[Networking](/docs/architecture/networking)** -- `libp2p` transport, Kademlia DHT peer discovery, mDNS for LAN, and NAT traversal strategies.

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
