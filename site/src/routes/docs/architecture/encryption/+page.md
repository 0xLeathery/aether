---
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

All peer-to-peer traffic is encrypted at the `libp2p` transport layer using **PSK (Pre-Shared Key) encryption with XSalsa20**. An eavesdropper monitoring your network traffic cannot read your messages, hear your voice calls, or see your channel metadata.

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
