---
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
