---
title: Channels
description: Organize conversations within Aether swarms with channels synced via CRDTs.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# Channels

Channels are organized conversation spaces within a [swarm](/docs/guides/swarms). They work like Discord channels, but sync between peers via `CRDTs` -- no server needed.

## Default Channel

Every swarm starts with a **general** channel. This channel cannot be renamed or deleted. It is always there as a fallback conversation space.

## Creating Channels

Name a new channel and it syncs to all swarm peers automatically via `CRDT` metadata. Every peer who is connected (or comes online later) sees the new channel appear.

## Renaming Channels

Change a channel name and the rename propagates to all peers. Channel identity is tracked internally, so renaming does not break message history.

## Deleting Channels

Deleting a channel requires confirmation. The channel is archived and can potentially be restored if any peer still has the `CRDT` state. Deletion syncs to all connected peers.

## Message History

Messages are stored locally on your device and sync between peers when connected. If you were offline, you catch up automatically when you reconnect -- this is **eventual consistency** via Automerge `CRDTs`.

There is no central message store. If all peers delete their local data, the messages are gone. This is by design: your data lives on your device, not on someone else's server.

## Limitations

Messages are **cleartext on disk** -- Aether uses transport-only encryption (encrypted in transit, not at rest). Anyone with access to your device can read your local message history.
