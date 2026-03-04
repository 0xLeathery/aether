---
title: Inviting Peers
description: Share secret codes to connect with peers in your Aether swarms.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# Inviting Peers

Aether has no friend requests, no usernames, and no email invitations. To invite someone, you share your swarm's secret code and they join directly.

## Getting the Invite Code

Open a swarm and click **Invite** to copy the `aether://` code to your clipboard. This code contains the swarm's `PSK` (Pre-Shared Key) -- the only credential needed to join.

## Sharing the Code

Send the code through any secure channel you trust: an encrypted messenger, a face-to-face conversation, or any private method. The code is sensitive -- anyone who has it can join your swarm.

Avoid posting invite codes in public channels unless you want an open swarm.

## Joining with a Code

The recipient clicks **Join Swarm** and pastes the `aether://` code. Aether connects automatically via peer discovery over `DHT`. Peers on the same local network connect without internet.

## Regenerating Invite Codes

You can generate a new shareable link for the same swarm. Note that old codes still work -- the underlying `PSK` does not change. Regeneration creates a fresh link format, not a new key.

## How Connections Work

Peers discover each other via a distributed hash table (`DHT`) and connect directly, peer-to-peer. There is no relay server in the middle. Once connected, messages sync via [CRDT](/docs/architecture/crdts)-based replication.

For more on what happens after peers connect, see the [Channels guide](/docs/guides/channels).
