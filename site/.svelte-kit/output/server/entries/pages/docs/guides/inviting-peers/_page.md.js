import { a3 as head, e as escape_html, a as attr } from "../../../../../chunks/index.js";
const metadata = {
  "title": "Inviting Peers",
  "description": "Share secret codes to connect with peers in your Aether swarms."
};
const { title, description } = metadata;
function _page_md($$renderer) {
  head("cjjo7z", $$renderer, ($$renderer2) => {
    $$renderer2.title(($$renderer3) => {
      $$renderer3.push(`<title>${escape_html(title)} - Aether Docs</title>`);
    });
    $$renderer2.push(`<meta name="description"${attr("content", description)}/>`);
  });
  $$renderer.push(`<h1 id="inviting-peers"><a href="#inviting-peers">Inviting Peers</a></h1> <p>Aether has no friend requests, no usernames, and no email invitations. To invite someone, you share your swarm’s secret code and they join directly.</p> <h2 id="getting-the-invite-code"><a href="#getting-the-invite-code">Getting the Invite Code</a></h2> <p>Open a swarm and click <strong>Invite</strong> to copy the <code>aether://</code> code to your clipboard. This code contains the swarm’s <code>PSK</code> (Pre-Shared Key) — the only credential needed to join.</p> <h2 id="sharing-the-code"><a href="#sharing-the-code">Sharing the Code</a></h2> <p>Send the code through any secure channel you trust: an encrypted messenger, a face-to-face conversation, or any private method. The code is sensitive — anyone who has it can join your swarm.</p> <p>Avoid posting invite codes in public channels unless you want an open swarm.</p> <h2 id="joining-with-a-code"><a href="#joining-with-a-code">Joining with a Code</a></h2> <p>The recipient clicks <strong>Join Swarm</strong> and pastes the <code>aether://</code> code. Aether connects automatically via peer discovery over <code>DHT</code>. Peers on the same local network connect without internet.</p> <h2 id="regenerating-invite-codes"><a href="#regenerating-invite-codes">Regenerating Invite Codes</a></h2> <p>You can generate a new shareable link for the same swarm. Note that old codes still work — the underlying <code>PSK</code> does not change. Regeneration creates a fresh link format, not a new key.</p> <h2 id="how-connections-work"><a href="#how-connections-work">How Connections Work</a></h2> <p>Peers discover each other via a distributed hash table (<code>DHT</code>) and connect directly, peer-to-peer. There is no relay server in the middle. Once connected, messages sync via <a href="/docs/architecture/crdts">CRDT</a>-based replication.</p> <p>For more on what happens after peers connect, see the <a href="/docs/guides/channels">Channels guide</a>.</p>`);
}
export {
  _page_md as default,
  metadata
};
