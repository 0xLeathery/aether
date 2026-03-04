---
title: Networking
description: How Aether peers discover each other, establish connections, and communicate using libp2p.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# Networking Protocol

> This is a simplified overview of Aether's networking layer, not an exhaustive protocol specification.

Aether's networking is built on **`libp2p` 0.56** (Rust implementation), a modular peer-to-peer networking framework originally created for IPFS. `libp2p` handles peer discovery, connection management, and protocol multiplexing -- letting Aether focus on application logic rather than socket programming.

## Transport

Aether uses **TCP** as its transport protocol. This is a deliberate choice: Aether's swarm isolation relies on Pre-Shared Key (PSK) encryption using XSalsa20, which operates at the transport layer. QUIC -- the more modern alternative -- has its own TLS layer built in, and that TLS layer conflicts with PSK encryption. TCP avoids this conflict.

All connections are direct peer-to-peer. There are no relay servers in the current architecture.

## Peer Discovery

Finding other peers happens through two mechanisms:

**Kademlia DHT (Distributed Hash Table)** -- For internet-wide discovery. Peers announce themselves to the DHT and query it to find others who share the same swarm key. The DHT is distributed across all participating peers, so there is no central directory.

**mDNS (Multicast DNS)** -- For local network discovery. If two Aether peers are on the same LAN, they can find each other without any internet connectivity. This is especially useful for offline or air-gapped environments.

## NAT Traversal

Most home networks sit behind a NAT (Network Address Translation) router, which makes direct incoming connections difficult. Aether uses STUN-like techniques for UDP holepunching to establish direct connections through NAT.

This works for most NAT types (full cone, restricted cone, port-restricted cone). However, **symmetric NAT** -- found in approximately 10-20% of consumer networks -- prevents holepunching. A TURN relay would solve this but is not currently implemented, as it requires server infrastructure (which has a cost). This is a known limitation.

## PSK Swarm Isolation

Each Aether swarm is protected by a **Pre-Shared Key** -- a 256-bit secret that acts as a network-level access control. The PSK enables XSalsa20 stream encryption via `libp2p`'s `PnetConfig` (Private Network Configuration).

This means:

- Only peers who possess the correct PSK can participate in the swarm
- Encryption happens at the transport layer, before any application data is exchanged
- A peer without the key cannot even establish a connection -- the handshake fails immediately

Think of it as a locked room: you need the key to enter. Everyone inside can communicate freely. See the [Encryption](/docs/architecture/encryption) page for the full security model and its limitations.

## Connection Lifecycle

1. **Bootstrap** -- Peer starts and connects to known DHT bootstrap nodes (or discovers local peers via mDNS)
2. **Announce** -- Peer registers itself on the DHT with its swarm identifier
3. **Discover** -- Peer queries the DHT for other peers in the same swarm
4. **Connect** -- Direct TCP connection established with PSK handshake
5. **Multiplex** -- Multiple protocols (chat sync, voice, control messages) run over a single connection

## Related Pages

- [Architecture Overview](/docs/architecture) -- High-level system diagram
- [Encryption](/docs/architecture/encryption) -- PSK security model and limitations
- [Voice](/docs/architecture/voice) -- Audio streaming over `libp2p`
- [CRDTs](/docs/architecture/crdts) -- Data sync over established connections
