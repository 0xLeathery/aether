---
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

Aether uses the **Automerge** library (Rust crate with `derive` macros) for all CRDT operations. Automerge provides a document model similar to JSON, but with built-in merge semantics.

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
