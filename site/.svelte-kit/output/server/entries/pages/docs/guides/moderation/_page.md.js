import { a3 as head, e as escape_html, a as attr } from "../../../../../chunks/index.js";
const metadata = {
  "title": "Moderation",
  "description": "Control your Aether experience with local hide and block tools. Sovereign moderation for a P2P world."
};
const { title, description } = metadata;
function _page_md($$renderer) {
  head("dg017l", $$renderer, ($$renderer2) => {
    $$renderer2.title(($$renderer3) => {
      $$renderer3.push(`<title>${escape_html(title)} - Aether Docs</title>`);
    });
    $$renderer2.push(`<meta name="description"${attr("content", description)}/>`);
  });
  $$renderer.push(`<h1 id="moderation"><a href="#moderation">Moderation</a></h1> <p>In a peer-to-peer system, there is no admin to ban people. Instead, <strong>you</strong> control <strong>your</strong> experience. Moderation in Aether is local and sovereign.</p> <h2 id="hiding-a-peer"><a href="#hiding-a-peer">Hiding a Peer</a></h2> <p>Hiding a peer filters their messages from your view. It is reversible — unhide anytime to see their messages again. The hidden peer does not know they have been hidden.</p> <p>Hidden messages are still stored locally (they sync via <code>CRDTs</code> like any other message). Hiding only affects your display — it does not delete anything.</p> <h2 id="blocking-a-peer"><a href="#blocking-a-peer">Blocking a Peer</a></h2> <p>Blocking goes further than hiding. A blocked peer’s messages are hidden <strong>and</strong> their voice audio is refused. You will not hear them in <a href="/docs/guides/voice-chat">voice sessions</a>, and they will not hear you.</p> <p>Like hiding, blocking is reversible and the blocked peer does not know they have been blocked.</p> <h2 id="managing-your-list"><a href="#managing-your-list">Managing Your List</a></h2> <p>View your hidden and blocked peers in the moderation settings. Unhide or unblock anyone at any time. Changes take effect immediately.</p> <h2 id="limitations"><a href="#limitations">Limitations</a></h2> <p>Hide and block are <strong>local only</strong>. Other peers in the swarm still see the person’s messages and hear their voice. There is no global ban mechanism — this is by design.</p> <p>If someone is disruptive to your entire group, the recommended approach is to create a new <a href="/docs/guides/swarms">swarm</a> and re-invite only the peers you trust. This mirrors how trust works in the real world: you choose who you spend time with.</p>`);
}
export {
  _page_md as default,
  metadata
};
