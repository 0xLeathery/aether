import { c as ensure_array_like, a as attr, b as attr_class, s as stringify, e as escape_html, d as derived } from "../../../chunks/index.js";
import { p as page } from "../../../chunks/index2.js";
import "flexsearch";
const docsNav = [
  {
    title: "Getting Started",
    href: "/docs/getting-started"
  },
  {
    title: "User Guides",
    href: "/docs/guides",
    children: [
      { title: "Swarms", href: "/docs/guides/swarms" },
      { title: "Inviting Peers", href: "/docs/guides/inviting-peers" },
      { title: "Channels", href: "/docs/guides/channels" },
      { title: "Voice Chat", href: "/docs/guides/voice-chat" },
      { title: "Moderation", href: "/docs/guides/moderation" }
    ]
  },
  {
    title: "Architecture",
    href: "/docs/architecture",
    children: [
      { title: "Networking", href: "/docs/architecture/networking" },
      { title: "Identity", href: "/docs/architecture/identity" },
      { title: "CRDTs", href: "/docs/architecture/crdts" },
      { title: "Voice", href: "/docs/architecture/voice" },
      { title: "Encryption", href: "/docs/architecture/encryption" }
    ]
  }
];
function flattenNav(items = docsNav) {
  const result = [];
  for (const item of items) {
    result.push({ title: item.title, href: item.href });
    if (item.children) {
      result.push(...flattenNav(item.children));
    }
  }
  return result;
}
function DocNav($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    function isActive(href) {
      return page.url.pathname === href;
    }
    $$renderer2.push(`<nav class="space-y-2"><!--[-->`);
    const each_array = ensure_array_like(docsNav);
    for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
      let item = each_array[$$index_1];
      $$renderer2.push(`<div><a${attr("href", item.href)}${attr_class(`block font-mono text-sm font-bold transition-colors ${stringify(isActive(item.href) ? "text-accent-green" : "text-text-secondary hover:text-text-primary")}`)}>${escape_html(item.title)}</a> `);
      if (item.children) {
        $$renderer2.push("<!--[0-->");
        $$renderer2.push(`<div class="mt-1 ml-2 space-y-1 border-l border-border pl-3"><!--[-->`);
        const each_array_1 = ensure_array_like(item.children);
        for (let $$index = 0, $$length2 = each_array_1.length; $$index < $$length2; $$index++) {
          let child = each_array_1[$$index];
          $$renderer2.push(`<a${attr("href", child.href)}${attr_class(`block font-mono text-sm transition-colors ${stringify(isActive(child.href) ? "text-accent-green" : "text-text-secondary hover:text-text-primary")}`)}>${escape_html(child.title)}</a>`);
        }
        $$renderer2.push(`<!--]--></div>`);
      } else {
        $$renderer2.push("<!--[-1-->");
      }
      $$renderer2.push(`<!--]--></div>`);
    }
    $$renderer2.push(`<!--]--></nav>`);
  });
}
function DocSearch($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let query = "";
    $$renderer2.push(`<div class="relative mb-4"><input type="text" placeholder="Search docs..."${attr("value", query)} class="w-full rounded-lg border border-border bg-bg-tertiary px-3 py-2 font-mono text-sm text-text-primary placeholder:text-text-muted focus:border-accent-green focus:outline-none"/> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></div>`);
  });
}
function PrevNext($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const flat = flattenNav();
    let currentIndex = derived(() => flat.findIndex((item) => item.href === page.url.pathname));
    let prev = derived(() => currentIndex() > 0 ? flat[currentIndex() - 1] : null);
    let next = derived(() => currentIndex() >= 0 && currentIndex() < flat.length - 1 ? flat[currentIndex() + 1] : null);
    $$renderer2.push(`<div class="mt-12 flex items-center justify-between border-t border-border pt-6">`);
    if (prev()) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<a${attr("href", prev().href)} class="font-mono text-sm text-text-secondary transition-colors hover:text-accent-green">← ${escape_html(prev().title)}</a>`);
    } else {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<div></div>`);
    }
    $$renderer2.push(`<!--]--> `);
    if (next()) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<a${attr("href", next().href)} class="font-mono text-sm text-text-secondary transition-colors hover:text-accent-green">${escape_html(next().title)} →</a>`);
    } else {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<div></div>`);
    }
    $$renderer2.push(`<!--]--></div>`);
  });
}
function _layout($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { children } = $$props;
    let sidebarOpen = false;
    $$renderer2.push(`<div class="flex min-h-[calc(100vh-4rem)]"><button class="fixed bottom-4 right-4 z-40 rounded-lg border border-border bg-bg-secondary p-3 text-text-primary shadow-lg md:hidden" aria-label="Toggle documentation sidebar"><svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16"></path></svg></button> <aside${attr_class("fixed inset-y-0 left-0 z-30 w-64 shrink-0 transform border-r border-border bg-bg-primary pt-16 transition-transform duration-200 md:static md:translate-x-0 md:pt-0", void 0, {
      "-translate-x-full": (
        // Close sidebar on navigation (mobile)
        !sidebarOpen
      ),
      "translate-x-0": sidebarOpen
    })}><div class="sticky top-16 p-6"><h2 class="mb-4 font-mono text-sm font-semibold uppercase tracking-wider text-text-muted">Documentation</h2> `);
    DocSearch($$renderer2);
    $$renderer2.push(`<!----> `);
    DocNav($$renderer2);
    $$renderer2.push(`<!----></div></aside> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> <div class="mx-auto w-full max-w-3xl flex-1 px-4 py-8 sm:px-6 lg:px-8"><div class="prose">`);
    children($$renderer2);
    $$renderer2.push(`<!----></div> `);
    PrevNext($$renderer2);
    $$renderer2.push(`<!----></div></div>`);
  });
}
export {
  _layout as default
};
