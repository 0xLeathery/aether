import { a3 as head, e as escape_html, a as attr } from "../../../../../chunks/index.js";
const metadata = {
  "title": "Voice Chat",
  "description": "Real-time peer-to-peer voice sessions using Opus codec and WebRTC mesh networking."
};
const { title, description } = metadata;
function _page_md($$renderer) {
  head("1s2c3o0", $$renderer, ($$renderer2) => {
    $$renderer2.title(($$renderer3) => {
      $$renderer3.push(`<title>${escape_html(title)} - Aether Docs</title>`);
    });
    $$renderer2.push(`<meta name="description"${attr("content", description)}/>`);
  });
  $$renderer.push(`<h1 id="voice-chat"><a href="#voice-chat">Voice Chat</a></h1> <p>Aether voice chat is direct peer-to-peer audio using the <code>Opus</code> codec. There is no voice server — audio streams directly between connected peers via <code>WebRTC</code>.</p> <h2 id="joining-a-voice-session"><a href="#joining-a-voice-session">Joining a Voice Session</a></h2> <p>Click the voice button in a channel to join. Your microphone activates and you connect to other peers already in the session. Audio flows immediately.</p> <h2 id="mute-and-unmute"><a href="#mute-and-unmute">Mute and Unmute</a></h2> <p>Toggle your microphone with the mute button. Your mute state is visible to other peers so they know when you are not transmitting.</p> <h2 id="peer-limit"><a href="#peer-limit">Peer Limit</a></h2> <p>Voice sessions have a hard limit of <strong>8 simultaneous participants</strong>. This is a mesh network constraint — each peer sends audio to every other peer directly. At 8 peers, that is 56 audio streams, which approaches the practical bandwidth ceiling.</p> <h2 id="audio-quality"><a href="#audio-quality">Audio Quality</a></h2> <p>Aether uses the <code>Opus</code> codec at approximately 48kbps per stream. On a local network, expect latency under 50ms. Over the internet, latency varies depending on the distance between peers.</p> <h2 id="leaving-voice"><a href="#leaving-voice">Leaving Voice</a></h2> <p>Click the voice button again or switch channels. Audio stops immediately and other peers see you leave the session.</p> <h2 id="troubleshooting"><a href="#troubleshooting">Troubleshooting</a></h2> <p>If voice does not connect, check these common issues:</p> <ul><li><strong>Microphone permissions</strong> — Your OS may need to grant Aether access to the microphone.</li> <li><strong>Firewall or NAT</strong> — Aether uses <code>STUN</code> to traverse NAT, but symmetric NAT (common on corporate networks) may prevent direct connections. Approximately 10-20% of users behind symmetric NAT may experience connection issues.</li> <li><strong>No relay fallback</strong> — Aether does not use <code>TURN</code> relay servers. If a direct peer-to-peer path cannot be established, voice will not connect. This is a trade-off of the zero-server architecture.</li></ul>`);
}
export {
  _page_md as default,
  metadata
};
