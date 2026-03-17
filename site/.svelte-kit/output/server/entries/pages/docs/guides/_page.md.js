import { a3 as head, e as escape_html, a as attr } from "../../../../chunks/index.js";
const metadata = {
  "title": "User Guides",
  "description": "Feature guides for Aether -- swarms, inviting peers, channels, voice chat, and moderation."
};
const { title, description } = metadata;
function _page_md($$renderer) {
  head("1oynsn4", $$renderer, ($$renderer2) => {
    $$renderer2.title(($$renderer3) => {
      $$renderer3.push(`<title>${escape_html(title)} - Aether Docs</title>`);
    });
    $$renderer2.push(`<meta name="description"${attr("content", description)}/>`);
  });
  $$renderer.push(`<h1 id="user-guides"><a href="#user-guides">User Guides</a></h1> <p>These guides cover every core feature of Aether. Each one is a short, focused walkthrough you can read in a few minutes.</p> <ul><li><p><strong><a href="/docs/guides/swarms">Swarms</a></strong> — Create private peer groups and manage your communities. Learn how swarm keys work, how to join and leave swarms, and how to keep them secure.</p></li> <li><p><strong><a href="/docs/guides/inviting-peers">Inviting Peers</a></strong> — Share secret codes to connect with others. No friend requests, no usernames — just a code and a direct connection.</p></li> <li><p><strong><a href="/docs/guides/channels">Channels</a></strong> — Organize conversations within a swarm. Create, rename, and delete channels. Understand how messages sync between peers via CRDTs.</p></li> <li><p><strong><a href="/docs/guides/voice-chat">Voice Chat</a></strong> — Start real-time P2P voice sessions with peers in your swarm. Learn about the mesh architecture, peer limits, and audio quality.</p></li> <li><p><strong><a href="/docs/guides/moderation">Moderation</a></strong> — Control your experience with hide and block. In a P2P system, moderation is local and sovereign — you decide what you see.</p></li></ul>`);
}
export {
  _page_md as default,
  metadata
};
