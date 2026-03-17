import "clsx";
import { a as attr, b as attr_class, s as stringify, d as derived, e as escape_html, c as ensure_array_like } from "../../chunks/index.js";
import { n as navLinks, s as siteConfig } from "../../chunks/constants.js";
import { p as page } from "../../chunks/index2.js";
function NavLink($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { href, children } = $$props;
    let isExternal = derived(() => href.startsWith("http"));
    let isActive = derived(() => href === "/" ? page.url.pathname === "/" : page.url.pathname.startsWith(href));
    if (isExternal()) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<a${attr("href", href)} target="_blank" rel="noopener noreferrer" class="relative py-1 text-text-secondary transition-colors duration-200 hover:text-text-primary">`);
      children($$renderer2);
      $$renderer2.push(`<!----></a>`);
    } else {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<a${attr("href", href)}${attr_class(`relative py-1 transition-colors duration-200 ${stringify(isActive() ? "text-accent-green" : "text-text-secondary hover:text-text-primary")}`)}>`);
      children($$renderer2);
      $$renderer2.push(`<!----> `);
      if (isActive()) {
        $$renderer2.push("<!--[0-->");
        $$renderer2.push(`<span class="absolute bottom-0 left-0 h-0.5 w-full bg-accent-green"></span>`);
      } else {
        $$renderer2.push("<!--[-1-->");
      }
      $$renderer2.push(`<!--]--></a>`);
    }
    $$renderer2.push(`<!--]-->`);
  });
}
function MobileMenu($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    navLinks.filter((l) => !l.external);
    navLinks.find((l) => l.label === "GitHub");
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]-->`);
  });
}
function Nav($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const internalLinks = navLinks.filter((l) => !l.external);
    const githubLink = navLinks.find((l) => l.label === "GitHub");
    let mobileMenuOpen = false;
    $$renderer2.push(`<nav${attr_class(`fixed top-0 z-50 w-full border-b border-border bg-bg-primary/95 backdrop-blur-sm transition-transform duration-300 ${stringify("translate-y-0")}`)}><div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8"><a href="/" class="flex items-center gap-2 font-mono text-lg font-bold text-accent-green"><span class="text-xl" aria-hidden="true">>_</span> <span>${escape_html(siteConfig.name)}</span></a> <div class="hidden items-center gap-6 md:flex"><!--[-->`);
    const each_array = ensure_array_like(internalLinks);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let link = each_array[$$index];
      NavLink($$renderer2, {
        href: link.href,
        children: ($$renderer3) => {
          $$renderer3.push(`<!---->${escape_html(link.label)}`);
        }
      });
    }
    $$renderer2.push(`<!--]--></div> <div class="hidden items-center gap-3 md:flex"><a href="/demo" class="rounded-md bg-accent-green px-4 py-2 text-sm font-semibold text-bg-primary transition-colors duration-200 hover:bg-accent-green-dim">Try Demo</a> <a${attr("href", githubLink?.href)} target="_blank" rel="noopener noreferrer" class="rounded-md border border-border-bright px-4 py-2 text-sm font-semibold text-text-secondary transition-colors duration-200 hover:border-text-secondary hover:text-text-primary">GitHub</a></div> <button class="flex items-center justify-center md:hidden"${attr("aria-label", "Open menu")}${attr("aria-expanded", mobileMenuOpen)}><svg class="h-6 w-6 text-text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">`);
    {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>`);
    }
    $$renderer2.push(`<!--]--></svg></button></div></nav> `);
    MobileMenu($$renderer2);
    $$renderer2.push(`<!---->`);
  });
}
function Footer($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const internalLinks = navLinks.filter((l) => !l.external);
    const githubLink = navLinks.find((l) => l.label === "GitHub");
    $$renderer2.push(`<footer class="mt-auto border-t border-border bg-bg-secondary"><div class="mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8"><div class="grid grid-cols-1 gap-8 md:grid-cols-3"><div><a href="/" class="font-mono text-lg font-bold text-accent-green"><span aria-hidden="true">>_</span> ${escape_html(siteConfig.name)}</a> <p class="mt-2 text-sm text-text-secondary">${escape_html(siteConfig.tagline)}</p> <p class="mt-1 text-sm text-text-muted">Own your identity. Own your data.</p></div> <div><h3 class="font-mono text-sm font-semibold uppercase tracking-wider text-text-muted">Navigate</h3> <ul class="mt-3 flex flex-col gap-2"><!--[-->`);
    const each_array = ensure_array_like(internalLinks);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let link = each_array[$$index];
      $$renderer2.push(`<li><a${attr("href", link.href)} class="text-sm text-text-secondary transition-colors duration-200 hover:text-text-primary">${escape_html(link.label)}</a></li>`);
    }
    $$renderer2.push(`<!--]--></ul></div> <div><h3 class="font-mono text-sm font-semibold uppercase tracking-wider text-text-muted">Open Source</h3> <ul class="mt-3 flex flex-col gap-2"><li><span class="inline-flex items-center gap-2 rounded-full border border-border-bright px-2.5 py-0.5 text-xs font-medium text-text-secondary">${escape_html(siteConfig.license)} License</span></li> <li><a${attr("href", githubLink?.href)} target="_blank" rel="noopener noreferrer" class="text-sm text-text-secondary transition-colors duration-200 hover:text-text-primary">View Source on GitHub</a></li> <li><a${attr("href", `${stringify(siteConfig.github)}/issues`)} target="_blank" rel="noopener noreferrer" class="text-sm text-text-secondary transition-colors duration-200 hover:text-text-primary">Report an Issue</a></li></ul></div></div> <div class="mt-8 flex flex-col items-center justify-between gap-2 border-t border-border pt-8 text-xs text-text-muted sm:flex-row"><span>v${escape_html(siteConfig.version)}</span> <span class="font-mono">Built for sovereignty</span> <span>Source-available software</span></div></div></footer>`);
  });
}
function _layout($$renderer, $$props) {
  let { children } = $$props;
  $$renderer.push(`<div class="flex min-h-screen flex-col">`);
  Nav($$renderer);
  $$renderer.push(`<!----> <main class="flex-1 pt-16">`);
  children($$renderer);
  $$renderer.push(`<!----></main> `);
  Footer($$renderer);
  $$renderer.push(`<!----></div>`);
}
export {
  _layout as default
};
