---
title: Swarms
description: Create private peer groups, share secret keys, and manage your Aether communities.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# Swarms

A **swarm** is a private peer group identified by a shared secret key (a `PSK` -- Pre-Shared Key). Think of it like a private server, except there is no server. Just peers who share the same key.

## Creating a Swarm

When you create a swarm, Aether generates a cryptographically random 256-bit key encoded as an `aether://<64-hex-chars>` URI. This key **is** the swarm -- whoever has it can join. There is no registration, no approval process.

## Joining a Swarm

Paste the `aether://` code that another member shared with you. Aether connects you to the swarm automatically via peer discovery over `DHT` (distributed hash table). This works on the same LAN without internet.

## Your Swarm List

All your swarms appear in the left sidebar. Click to switch between them. Each swarm has its own set of [channels](/docs/guides/channels) and peers.

## Renaming a Swarm

Swarm names are **local only**. Your name for a swarm is yours alone -- other peers name it whatever they want. There is no "official" name because there is no central authority.

## Leaving a Swarm

Leaving removes all local data for that swarm. You can rejoin later if you still have the secret code.

## Security

Anyone with the secret code can join your swarm. Treat the code like a password. If it gets shared with someone you do not trust, create a new swarm and re-invite only the peers you want.

There is no way to "revoke" a code or kick someone out. This is a trade-off of the P2P model -- the swarm key is the only credential, and there is no server to enforce access control. See [Moderation](/docs/guides/moderation) for how to manage your experience locally.
