import { a3 as head, e as escape_html, a as attr } from "../../../../../chunks/index.js";
const metadata = {
  "title": "Channels",
  "description": "Organize conversations within Aether swarms with channels synced via CRDTs."
};
const { title, description } = metadata;
function _page_md($$renderer) {
  head("1gfp1e1", $$renderer, ($$renderer2) => {
    $$renderer2.title(($$renderer3) => {
      $$renderer3.push(`<title>${escape_html(title)} - Aether Docs</title>`);
    });
    $$renderer2.push(`<meta name="description"${attr("content", description)}/>`);
  });
  $$renderer.push(`<h1 id="channels"><a href="#channels">Channels</a></h1> <p>Channels are organized conversation spaces within a <a href="/docs/guides/swarms">swarm</a>. They work like Discord channels, but sync between peers via <code>CRDTs</code> — no server needed.</p> <h2 id="default-channel"><a href="#default-channel">Default Channel</a></h2> <p>Every swarm starts with a <strong>general</strong> channel. This channel cannot be renamed or deleted. It is always there as a fallback conversation space.</p> <h2 id="creating-channels"><a href="#creating-channels">Creating Channels</a></h2> <p>Name a new channel and it syncs to all swarm peers automatically via <code>CRDT</code> metadata. Every peer who is connected (or comes online later) sees the new channel appear.</p> <h2 id="renaming-channels"><a href="#renaming-channels">Renaming Channels</a></h2> <p>Change a channel name and the rename propagates to all peers. Channel identity is tracked internally, so renaming does not break message history.</p> <h2 id="deleting-channels"><a href="#deleting-channels">Deleting Channels</a></h2> <p>Deleting a channel requires confirmation. The channel is archived and can potentially be restored if any peer still has the <code>CRDT</code> state. Deletion syncs to all connected peers.</p> <h2 id="message-history"><a href="#message-history">Message History</a></h2> <p>Messages are stored locally on your device and sync between peers when connected. If you were offline, you catch up automatically when you reconnect — this is <strong>eventual consistency</strong> via Automerge <code>CRDTs</code>.</p> <p>There is no central message store. If all peers delete their local data, the messages are gone. This is by design: your data lives on your device, not on someone else’s server.</p> <h2 id="limitations"><a href="#limitations">Limitations</a></h2> <p>Messages are <strong>cleartext on disk</strong> — Aether uses transport-only encryption (encrypted in transit, not at rest). Anyone with access to your device can read your local message history.</p>`);
}
export {
  _page_md as default,
  metadata
};
