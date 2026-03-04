---
title: Moderation
description: Control your Aether experience with local hide and block tools. Sovereign moderation for a P2P world.
---

<svelte:head>
  <title>{title} - Aether Docs</title>
  <meta name="description" content={description} />
</svelte:head>

# Moderation

In a peer-to-peer system, there is no admin to ban people. Instead, **you** control **your** experience. Moderation in Aether is local and sovereign.

## Hiding a Peer

Hiding a peer filters their messages from your view. It is reversible -- unhide anytime to see their messages again. The hidden peer does not know they have been hidden.

Hidden messages are still stored locally (they sync via `CRDTs` like any other message). Hiding only affects your display -- it does not delete anything.

## Blocking a Peer

Blocking goes further than hiding. A blocked peer's messages are hidden **and** their voice audio is refused. You will not hear them in [voice sessions](/docs/guides/voice-chat), and they will not hear you.

Like hiding, blocking is reversible and the blocked peer does not know they have been blocked.

## Managing Your List

View your hidden and blocked peers in the moderation settings. Unhide or unblock anyone at any time. Changes take effect immediately.

## Limitations

Hide and block are **local only**. Other peers in the swarm still see the person's messages and hear their voice. There is no global ban mechanism -- this is by design.

If someone is disruptive to your entire group, the recommended approach is to create a new [swarm](/docs/guides/swarms) and re-invite only the peers you trust. This mirrors how trust works in the real world: you choose who you spend time with.
