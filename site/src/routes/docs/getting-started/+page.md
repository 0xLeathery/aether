---
title: Getting Started
description: Install Aether, create your cryptographic identity, and connect to your first peer in minutes.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# Getting Started

Aether is a peer-to-peer chat application with no servers, no accounts, and no data collection. This guide walks you through installation, identity creation, and connecting with your first peer.

## Install Aether

Download the latest release for your platform:

- **macOS** -- Apple Silicon (.dmg) and Intel (.dmg)
- **Windows** -- 64-bit installer (.msi)
- **Linux** -- AppImage (.AppImage)

Visit the [Download page](/download) for direct links. Aether runs on any modern operating system and requires no special hardware.

## First Launch

When you open Aether for the first time, the app automatically generates your **Ed25519 cryptographic identity** and stores it securely in your system keychain:

- **macOS** -- Keychain Access
- **Windows** -- Credential Manager
- **Linux** -- Secret Service (GNOME Keyring / KWallet)

There are no accounts, no passwords, and no email verification. Your identity is a keypair -- a **public key** (your address that peers see) and a **private key** (which never leaves your device).

## Set Your Display Name

Choose a display name that other peers will see when you chat. You can pick any name you like -- there is no central registry enforcing uniqueness.

Peers can set local "petnames" to override your display name on their end. This is the sovereign model: you choose how you present yourself, and others choose how they label you.

## Create Your First Swarm

A **swarm** is a private peer group. Think of it like a server, except there is no server -- just peers who share the same secret key.

When you create a swarm, Aether generates a cryptographically random key in the format `aether://<64-hex-chars>`. This key **is** the swarm -- anyone who has it can join.

For more details on managing swarms, see the [Swarms guide](/docs/guides/swarms).

## Invite a Peer

Share your swarm's secret code with someone you trust. They paste the `aether://` code into the "Join Swarm" dialog and connect automatically via peer discovery.

Send the code through a secure channel -- encrypted messenger, in person, or any method you trust. The code is sensitive: anyone who has it can join your swarm.

For the full invitation workflow, see the [Inviting Peers guide](/docs/guides/inviting-peers).

## Next Steps

You are up and running. Explore the User Guides to learn about every feature:

- **[Swarms](/docs/guides/swarms)** -- Create and manage private peer groups
- **[Inviting Peers](/docs/guides/inviting-peers)** -- Share secret codes to connect
- **[Channels](/docs/guides/channels)** -- Organize conversations within swarms
- **[Voice Chat](/docs/guides/voice-chat)** -- Real-time P2P voice sessions
- **[Moderation](/docs/guides/moderation)** -- Control your experience with hide and block
