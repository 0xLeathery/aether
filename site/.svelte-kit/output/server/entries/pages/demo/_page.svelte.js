import { a3 as head } from "../../../chunks/index.js";
import "qr-creator";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    head("1du1zi4", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>Demo - Aether</title>`);
      });
    });
    $$renderer2.push(`<div class="mx-auto max-w-4xl px-4 py-16 sm:px-6 lg:px-8">`);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div class="text-center"><h1 class="font-mono text-3xl font-bold text-accent-green sm:text-4xl">Try Aether in Your Browser</h1> <p class="mx-auto mt-4 max-w-2xl text-lg leading-relaxed text-text-secondary">Experience peer-to-peer communication without downloading anything. This
        interactive demo connects you directly to another person using WebRTC.</p> <button class="mt-10 rounded-lg bg-accent-green px-8 py-4 font-mono text-lg font-semibold text-bg-primary transition-opacity hover:opacity-90">Start Chat</button> <p class="mt-4 text-sm text-text-muted">No account needed. Fully ephemeral. Messages disappear on page close.</p></div>`);
    }
    $$renderer2.push(`<!--]--></div>`);
  });
}
export {
  _page as default
};
