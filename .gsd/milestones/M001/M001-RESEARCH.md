# Project Research Summary

**Project:** Aether v2.0 Marketing Site
**Domain:** SvelteKit marketing website + documentation + interactive WebRTC P2P demo
**Researched:** 2026-03-04
**Confidence:** HIGH

## Executive Summary

Aether v2.0 is a marketing site for an existing Tauri v2 P2P desktop application, and the research makes the recommended approach clear: build a standalone SvelteKit application in `/site` alongside the existing monorepo without touching the Tauri app's build pipeline. The stack is a natural fit — the project already uses Svelte 5, and SvelteKit adds static site generation via `adapter-cloudflare`, documentation via mdsvex, and Tailwind CSS v4 for styling. Cloudflare Pages is the deploy target because its unlimited free bandwidth and 310+ edge locations eliminate cost risk even at launch traffic spikes. The entire marketing site, except the WebRTC demo sandbox, is prerendered static HTML.

The site's standout differentiator is an interactive browser-based P2P demo — no other P2P messaging tool offers a "try before you download" experience. The demo uses native WebRTC DataChannel APIs with manual SDP copy-paste signaling (philosophically aligned with Aether's "Secret Code" invite flow) or an optional Cloudflare Worker relay for better UX. This is the highest-complexity feature and the only one requiring server infrastructure. Everything else — landing page, download detection, documentation — is static content with well-established SvelteKit patterns.

The key risks are build pipeline collision (SvelteKit and the Tauri Vite SPA must remain fully isolated), the inherent signaling requirement for WebRTC (pure "serverless" browser P2P is impossible), code signing gaps before the download page goes live, and documentation staleness. All are avoidable with the right monorepo structure from day one and deliberate phase sequencing: scaffold and hard workspace boundaries first, landing page second, demo infrastructure third, documentation fourth, and distribution last — with code signing procurement beginning in Phase 1 regardless.

---

## Key Findings

### Recommended Stack

The marketing site is a SvelteKit 2.53 project in `/site` with its own `package.json`, `svelte.config.js`, and `vite.config.ts`. It shares the Git repo with the Tauri desktop app but has zero code dependencies on it. Deployment is Cloudflare Pages via `@sveltejs/adapter-cloudflare`. Documentation is authored in Markdown with mdsvex 0.12 (Svelte 5 compatible) and Shiki 4 for syntax highlighting. Tailwind CSS v4's Vite plugin removes the PostCSS config entirely. The WebRTC demo uses native browser `RTCPeerConnection` and `RTCDataChannel` — no PeerJS, no simple-peer, no js-libp2p. See [STACK.md](STACK.md) for full details.

**Core technologies:**
- **SvelteKit ^2.53:** Full-stack web framework for the `/site` workspace — zero learning curve given existing Svelte 5 usage in the desktop app
- **@sveltejs/adapter-cloudflare ^6.3:** Static + edge deployment adapter; enables prerendered marketing pages plus a single SSR route for OS-aware download detection
- **Tailwind CSS ^4.2:** Utility-first CSS via Vite plugin; CSS-first config eliminates `tailwind.config.js`; fast iteration on marketing page layouts
- **mdsvex ^0.12:** Svelte 5-compatible Markdown preprocessor; `.md` files become Svelte components, enabling interactive architecture diagrams embedded in documentation prose
- **Shiki ^4.0:** Build-time syntax highlighting via mdsvex; zero runtime cost; TextMate grammar accuracy for Rust and TypeScript examples in technical docs
- **Native WebRTC API (browser built-in):** `RTCPeerConnection` + `RTCDataChannel` for the P2P demo; no library adds bundle size while demonstrating raw P2P concepts
- **GitHub Releases:** Binary hosting for platform installers; metadata baked in at build time to avoid GitHub API rate limits at runtime

**Critical compatibility note:** mdsvex 0.12 and Shiki 4 are both ESM-only. The `/site/package.json` must set `"type": "module"`.

**What NOT to add:** PeerJS (requires signaling server), simple-peer (4+ years unmaintained), @libp2p/webrtc (200KB+ for a concept demo), Docusaurus/VitePress (introduces React/Vue), Astro (second meta-framework), Three.js or Framer Motion ports (heavy animation for negligible gain), any analytics or third-party CDN dependencies.

---

### Expected Features

See [FEATURES.md](FEATURES.md) for full competitor analysis and dependency graph.

**Must have (table stakes):**
- **Vision-first landing page** — hero leads with "Sovereign Node" philosophy; includes How It Works, Feature Highlights, Philosophy/Why P2P, GitHub trust signals, and a repeated Download CTA
- **Platform-aware download section** — SSR route at `/download` auto-detects OS; primary CTA for detected platform; links to GitHub Releases; SHA-256 checksums displayed
- **User documentation** — 8-10 pages: Getting Started, Creating a Swarm, Inviting Peers, Voice Chat, Channels, Petnames, Peer Moderation, Notifications, Troubleshooting, FAQ; searchable with sidebar navigation
- **Technical documentation** — Architecture Overview, P2P Networking, Identity System, CRDT Chat Sync, Voice Protocol, Encryption Model, Moderation Model, Data Model; with Mermaid/SVG architecture diagrams
- **Minimal navigation** — `Home | Docs | Download | Demo | GitHub`; no enterprise nav patterns
- **Mobile-first responsive design** — all pages functional at 375px; hamburger nav; doc sidebar collapses on mobile
- **Core Web Vitals compliance** — LCP under 2.5s, CLS under 0.1, landing page under 500KB
- **Open source transparency signals** — GitHub link in header, license badge, no analytics, no third-party scripts, self-hosted assets, no cookies banner

**Should have (differentiators):**
- **Interactive browser P2P demo** at `/sandbox` — WebRTC DataChannel text chat; starts with same-browser demo (solo-friendly), optionally upgrades to room-code multi-device; prominent "Download for full experience" CTAs; the only feature of its kind in the P2P messaging space
- **Visual architecture diagrams** — SVG or animated Svelte-powered: centralized vs P2P comparison, connection flow, swarm topology, encryption layers; every competitor either has no diagrams or only static images
- **Honest limitations section** — dedicated "Trade-offs We Chose" on landing page; documents simultaneous-presence requirement, transport-only encryption, hardware-bound identity, 8-peer voice limit, no push notifications; builds trust with privacy-conscious audience

**Defer to later:**
- Blog/news section (content commitment risk; link to GitHub releases instead)
- Community forum on-site (contradicts "community lives in the app")
- Video content (production effort beyond v2.0 scope)
- Internationalization (add when user base demands it)
- Analytics of any kind (zero analytics is the gold standard for this audience)

---

### Architecture Approach

The architecture is a two-application monorepo: the existing Tauri desktop app owns the repo root, and the marketing site is a fully independent SvelteKit project at `/site`. There is zero code sharing between them — no shared Svelte components, no shared stores, no shared types. The boundary is enforced at the `package.json` level. The only shared artifacts are the Git repository and GitHub Actions CI. The site builds independently (`cd site && npm run build`) and deploys to Cloudflare Pages on every push touching `site/**`. A new `site.yml` workflow handles site CI without touching `ci.yml` or `release.yml`. See [ARCHITECTURE.md](ARCHITECTURE.md) for the full directory structure, data flows, and signaling patterns.

**Major components:**

1. **Landing pages (`routes/`)** — Prerendered static HTML; marketing content, philosophy, feature showcase; no runtime JavaScript except hydration of interactive components
2. **Documentation (`routes/docs/[...slug]/`)** — Catch-all route loading `.md` files from `content/`; processed by mdsvex at build time; sidebar navigation via `DocsSidebar.svelte`
3. **Download page (`routes/download/`)** — SSR route; client-side platform detection via `detectPlatform()` + GitHub Releases API; release metadata baked at build time to avoid rate-limit dependency
4. **WebRTC sandbox (`routes/sandbox/`)** — `export const ssr = false`; fully client-side; `RTCPeerConnection` + `RTCDataChannel`; state in `lib/sandbox/store.svelte.ts` using `$state` runes; signaling via manual SDP base64 strings or optional Cloudflare Worker relay
5. **Sandbox engine (`lib/sandbox/`)** — `peer.ts` (connection lifecycle), `signaling.ts` (SDP encode/decode), `protocol.ts` (message types mirroring Aether concepts), `store.svelte.ts` (reactive state)
6. **Cloudflare Pages deployment** — Static output for all routes except `/download`; `site.yml` CI builds on every push to `site/**`; Git-connected deploy for automatic production deploys from `main`

**Key pattern:** Static-first with client-side islands. The sandbox page is the sole island. All other pages are prerendered HTML with zero runtime JavaScript for content. Set `export const prerender = true` in `+layout.ts` and `export const ssr = false` in `sandbox/+page.ts`.

---

### Critical Pitfalls

See [PITFALLS.md](PITFALLS.md) for full recovery strategies, technical debt patterns, and the complete "Looks Done But Isn't" checklist.

1. **Monorepo build pipeline collision** — If SvelteKit's dependency or its `svelte.config.js` bleeds into the Tauri app's root build, Tauri builds break with cryptic errors. Prevent by keeping `/site` as a completely independent workspace with its own `package.json`. Never modify the root `svelte.config.js` or root build script. Verify: `npm run build` at repo root builds only the Tauri app; `cd site && npm run build` builds only the marketing site.

2. **WebRTC requires signaling — "serverless browser P2P" is impossible** — Browsers have no peer discovery mechanism. Every WebRTC connection requires a signaling exchange before DataChannel can open. The demo must either use manual SDP copy-paste (on-brand, high friction) or a lightweight relay (better UX, minimal infrastructure). Start with a same-browser demo to allow solo visitors to experience P2P without needing a second person.

3. **STUN-only configuration breaks demo for 10-20% of visitors** — Corporate firewalls and symmetric NAT require TURN relay. Google STUN alone is insufficient. Use Cloudflare's free TURN service (1,000 GB/month). Configure TURN before the demo goes public. Fetch short-lived credentials from a server endpoint — never hardcode in client JavaScript.

4. **Code signing not in place before download page goes live** — Unsigned macOS binaries trigger Gatekeeper; unsigned Windows binaries trigger SmartScreen. Recovery cost is HIGH (reputation damage persists). Apple Developer Program ($99/year) and Windows EV certificate setup (Azure Key Vault) take days to weeks. Begin procurement in Phase 1 even though the download page ships in Phase 5.

5. **Documentation staleness within weeks of launch** — App evolves; docs don't. All doc pages must include a `last_verified` date in frontmatter. A CI job should warn when pages exceed 90 days without review. Separate evergreen content (philosophy, vision) from versioned content (setup guides, code examples).

---

## Implications for Roadmap

Based on the dependency graph in FEATURES.md, the architecture hard constraints, and the pitfall-to-phase mapping in PITFALLS.md, five phases are recommended. The ordering is driven by one principle: Pitfalls 1 and 7 (monorepo collision, shared component coupling) can only be prevented at setup time. Every phase depends on the workspace isolation established in Phase 1.

### Phase 1: Monorepo Scaffold + Hard Boundaries

**Rationale:** All other phases depend on this. Build pipeline collision (Pitfall 1) and shared component coupling (Pitfall 7) can only be prevented from day one. Once any feature work is done in the wrong structure, untangling it costs 3-5 days. This phase has zero user-visible output but determines project velocity for everything that follows.
**Delivers:** `/site` as a fully independent SvelteKit workspace; `site.yml` CI workflow; Tailwind CSS v4 configured; mdsvex configured with Shiki; global layout (`+layout.svelte`) with Nav + Footer placeholders; placeholder routes for all top-level pages; deployment pipeline connected to Cloudflare Pages; documentation freshness policy established in frontmatter schema; code signing procurement begun
**Addresses:** Navigation structure, responsive design foundation, open source transparency (GitHub link in layout)
**Avoids:** Pitfall 1 (build collision), Pitfall 7 (shared component coupling)

### Phase 2: Landing Page + Core Marketing Content

**Rationale:** The landing page IS the marketing site in minimal viable form. A scaffold with placeholder content is not launchable. This phase produces the first publicly shippable artifact. It is dependency-free from the demo and documentation — it needs only copy, design, and existing screenshot assets from the desktop app.
**Delivers:** Hero section with "Sovereign Node" value proposition; How It Works (3-step visual); Feature Highlights cards; Philosophy/Why P2P section; Honest Limitations ("Trade-offs We Chose"); Open Source trust signals; repeated Download CTA at page bottom; features page
**Uses:** Tailwind CSS v4 for layout iteration; `@sveltejs/enhanced-img` for screenshot optimization; Svelte built-in transitions (no Three.js, no Framer Motion)
**Implements:** Landing page component architecture from ARCHITECTURE.md
**Avoids:** Technical jargon in hero copy; auto-playing video; heavy JavaScript animations; third-party CDN dependencies

### Phase 3: Interactive WebRTC Demo Sandbox

**Rationale:** The demo is the primary differentiator and the highest-complexity feature. It has its own infrastructure dependency (signaling) and cross-browser compatibility concerns. Building it after the landing page means the site has a shippable state while the demo is in development. HTTPS must be confirmed on the production domain before the demo ships (Pitfall 3).
**Delivers:** `/sandbox` route with `ssr = false`; same-browser demo (solo-friendly, two virtual peers); optional room-code multi-device mode via signaling relay; connection state UI (idle/offering/answering/connected/failed); "Download for the full experience" CTA throughout; cross-browser verified (Chrome, Firefox, Safari, Edge); Cloudflare TURN configured; TURN credentials fetched server-side (not hardcoded); demo component lazy-loaded on the landing page
**Uses:** Native `RTCPeerConnection` + `RTCDataChannel`; `lib/sandbox/` engine (peer.ts, signaling.ts, protocol.ts, store.svelte.ts); optional Cloudflare Worker signaling relay
**Implements:** Pattern 2 (manual SDP signaling) from ARCHITECTURE.md with Pattern 3 (Cloudflare Worker relay) as optional upgrade
**Avoids:** Pitfall 2 (signaling denial), Pitfall 3 (HTTPS), Pitfall 4 (STUN-only); UX pitfall of solo visitor seeing "waiting for peer" forever

### Phase 4: Documentation (User + Technical)

**Rationale:** Documentation is the largest writing effort (18-22 pages estimated) and requires deep codebase knowledge for accuracy. It can be drafted in parallel with the demo but depends on the mdsvex configuration from Phase 1. Technical accuracy matters more than speed — the audience is technically literate and will notice errors. Freshness policy must be enforced from the first commit.
**Delivers:** User docs (Getting Started, Creating a Swarm, Inviting Peers, Voice Chat, Channels, Petnames, Peer Moderation, Notifications, Troubleshooting, FAQ); Technical docs (Architecture Overview, P2P Networking, Identity System, CRDT Chat Sync, Voice Protocol, Encryption Model, Moderation Model, Data Model); Mermaid/SVG architecture diagrams; `last_verified` frontmatter on all pages; docs sidebar with section hierarchy; previous/next navigation; docs search (Pagefind recommended)
**Uses:** mdsvex for Markdown rendering; Shiki + @shikijs/transformers for code blocks; rehype-slug + rehype-autolink-headings for deep-linking; `routes/docs/[...slug]/` catch-all route pattern
**Avoids:** Pitfall 6 (documentation staleness); anti-pattern of auto-generated API docs substituted for user documentation; walls of text without visual diagrams

### Phase 5: Download Page + Distribution

**Rationale:** The download page depends on signed binaries (Pitfall 5). Code signing procurement starts in Phase 1 but the page itself ships last because it has the highest trust requirements — an unsigned binary download on a privacy tool's site is worse than no download link at all. The GitHub Releases integration is straightforward once signing is in place.
**Delivers:** `/download` SSR route with User-Agent detection; auto-detected primary CTA for visitor's platform; all three platforms (macOS, Windows, Linux) with file sizes and system requirements; SHA-256 checksums displayed; link to build-from-source instructions; GitHub Releases metadata baked into static build (fallback to runtime API fetch); signed and notarized binaries on all platforms
**Uses:** `lib/utils/platform.ts` (detectPlatform with Apple Silicon detection via WebGL renderer); `lib/utils/github-releases.ts` (Release API fetch); SSR route for server-side User-Agent detection
**Avoids:** Pitfall 5 (unsigned binaries); GitHub API rate limiting (60 req/hr); linking to HTTP download URLs; missing checksums

---

### Phase Ordering Rationale

- **Phase 1 must be first** — The monorepo hard boundary is impossible to retrofit safely once features are built. Establishing the workspace isolation, CI, and mdsvex config here is a prerequisite for all other phases.
- **Phase 2 before Phase 3** — The landing page has zero external dependencies and produces the first launchable artifact. The demo requires signaling infrastructure and TURN provisioning; sequencing it after the landing page ensures the project is never blocked on demo complexity.
- **Phase 3 before Phase 4** — The demo is the riskier feature (WebRTC cross-browser issues, TURN configuration, signaling UX) and should be resolved before the project enters the high-volume content writing phase.
- **Phase 4 before Phase 5** — Documentation is the trust signal that makes a download decision credible. Publishing a download page before documentation exists tells visitors there is nothing to learn about the product.
- **Code signing starts in Phase 1, ships in Phase 5** — Apple notarization and Windows EV certificate procurement take days to weeks. The download page waits until signing is confirmed, but procurement cannot wait.

---

### Research Flags

**Phases likely needing deeper research during planning:**

- **Phase 3 (WebRTC Demo):** Signaling relay architecture has two valid options (manual SDP vs Cloudflare Worker Durable Objects). The choice significantly affects UX. If the Worker relay is chosen, Durable Objects for room state has its own architecture considerations. Recommend `/gsd:research-phase` when scoping Phase 3, specifically for: (1) Cloudflare Worker + Durable Objects signaling pattern, (2) Safari WebRTC unified-plan compatibility, (3) TURN credential rotation mechanism.
- **Phase 5 (Download / Distribution):** macOS notarization and Windows EV certificate setup via Azure Key Vault have changed with Tauri v2. The exact CI/CD configuration for signing in GitHub Actions depends on current Tauri action versions. Recommend `/gsd:research-phase` for the signing CI configuration specifically.

**Phases with standard patterns (skip research-phase):**

- **Phase 1 (Scaffold):** SvelteKit monorepo workspace isolation is well-documented and the architecture is fully prescribed in ARCHITECTURE.md. No ambiguity.
- **Phase 2 (Landing Page):** Static marketing page with Tailwind CSS + Svelte components is a fully established pattern. All decisions are content and copy decisions, not engineering research questions.
- **Phase 4 (Documentation):** mdsvex + SvelteKit catch-all route for docs is documented in ARCHITECTURE.md. Content writing is effort, not research. Pagefind has official SvelteKit integration guidance.

---

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All versions confirmed from npm; official docs cited; SvelteKit, Tailwind CSS v4, mdsvex, Shiki all verified with current releases. Critical ESM-only compatibility confirmed. |
| Features | HIGH | Competitor analysis covers 6 P2P messaging tools directly. Conversion lift data for interactive demos is MEDIUM confidence (single industry source). Feature prioritization is opinionated but grounded in pattern analysis. |
| Architecture | HIGH | SvelteKit adapter patterns, WebRTC DataChannel flow, and monorepo isolation are all from official or well-documented sources. Cloudflare Worker signaling pattern is MEDIUM confidence (community reference, not official Cloudflare guide). |
| Pitfalls | HIGH | All critical pitfalls cite official documentation (Tauri signing docs, MDN WebRTC secure context, Cloudflare TURN pricing). TURN failure rate (10-20%) is from a third-party guide — MEDIUM confidence on the specific figure, but the direction is well-established. |

**Overall confidence: HIGH**

### Gaps to Address

- **Exact Tauri binary naming conventions for GitHub Releases:** The `github-releases.ts` asset pattern matching (`/\.dmg$/i`, `/\.exe$/i`, etc.) assumes standard Tauri filenames. The actual release asset names from `release.yml` need verification to ensure download page patterns match real artifacts.
- **Signaling relay decision:** Research presents two valid approaches (manual SDP vs Cloudflare Worker relay). The choice is a UX trade-off, not a technical constraint. Resolve at the start of Phase 3 planning — ideally with a quick prototype of both approaches.
- **Apple Silicon vs Intel macOS binary distinction:** The `detectPlatform()` function uses a WebGL renderer heuristic for Apple Silicon detection. This may produce false results in some browsers. If Tauri builds produce separate `aarch64` and `x86_64` binaries, the detection logic needs validation against real browser environments.
- **Search for docs:** Research recommends Pagefind for documentation search but does not provide a full integration guide. This is standard SvelteKit practice but should be confirmed in Phase 4 task breakdown.

---

## Sources

### Primary (HIGH confidence)
- [SvelteKit adapter-cloudflare docs](https://svelte.dev/docs/kit/adapter-cloudflare) — deployment config, prerendering strategy
- [SvelteKit images docs](https://svelte.dev/docs/kit/images) — @sveltejs/enhanced-img usage
- [Tailwind CSS v4 SvelteKit guide](https://tailwindcss.com/docs/guides/sveltekit) — Vite plugin setup, CSS-first config
- [mdsvex docs](https://mdsvex.pngwn.io/docs) + [Svelte CLI mdsvex](https://svelte.dev/docs/cli/mdsvex) — Svelte 5 compatibility, highlighter API
- [Tauri v2 macOS Code Signing](https://v2.tauri.app/distribute/sign/macos/) — signing and notarization requirements
- [Tauri v2 Windows Code Signing](https://v2.tauri.app/distribute/sign/windows/) — HSM/Azure Key Vault requirements
- [MDN getUserMedia Secure Context](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia) — HTTPS requirement for WebRTC APIs
- [Cloudflare Pages pricing](https://www.cloudflare.com/plans/developer-platform/) — unlimited free bandwidth confirmed
- [Cloudflare TURN pricing](https://developers.cloudflare.com/realtime/turn/faq/) — free tier (1,000 GB/month) confirmed
- [Signal](https://signal.org), [Briar](https://briarproject.org), [Element](https://element.io), [Session](https://getsession.org), [Jami](https://jami.net), [Tox](https://tox.chat) — competitor analysis
- [@sveltejs/kit npm](https://www.npmjs.com/package/@sveltejs/kit) — v2.53.4 current
- [mdsvex npm](https://www.npmjs.com/package/mdsvex) — v0.12.6 current
- [Shiki npm](https://www.npmjs.com/package/shiki) — v4.0.1 current
- [SvelteKit monorepo $lib alias issue](https://github.com/sveltejs/kit/issues/14518) — confirmed monorepo alias conflict

### Secondary (MEDIUM confidence)
- [WebRTC NAT Traversal Guide](https://webrtc.link/en/articles/stun-turn-servers-webrtc-nat-traversal/) — STUN/TURN failure rate estimates
- [P2PCF: Cloudflare Workers signaling](https://github.com/gfodor/p2pcf) — Worker signaling relay pattern
- [Josh Collinsworth: static SvelteKit Markdown blog](https://joshcollinsworth.com/blog/build-static-sveltekit-markdown-blog) — mdsvex + catch-all route pattern
- [Navattic Interactive Demo Best Practices](https://www.navattic.com/blog/interactive-demos) — 25-40% conversion lift data
- [Serverless WebRTC](https://github.com/cjb/serverless-webrtc) — manual SDP exchange reference
- [WebRTC DataChannel overview](https://webrtcforthecurious.com/docs/02-signaling/) — signaling flow documentation

### Tertiary (LOW confidence)
- [Landing page statistics 2026](https://www.involve.me/blog/landing-page-statistics) — 83% mobile traffic figure
- [TURN Server Costs Guide](https://dev.to/alakkadshaw/turn-server-costs-a-complete-guide-1c4b) — cost comparison for managed TURN services

---
*Research completed: 2026-03-04*
*Ready for roadmap: yes*

# Architecture Research: Marketing Site + WebRTC P2P Sandbox

**Domain:** SvelteKit marketing site with interactive browser-based P2P demo, integrated into existing Tauri app monorepo
**Researched:** 2026-03-04
**Confidence:** HIGH

## System Overview

The marketing site is a separate SvelteKit application (`/site`) that lives alongside the existing Tauri desktop app in the same monorepo. It produces static HTML (via `adapter-static`) for landing pages, docs, and downloads, while embedding an interactive WebRTC sandbox that demonstrates Aether's P2P concepts in the browser.

```
Monorepo Root (aether/)
==============================================================================================================

  Existing Desktop App                           New Marketing Site
  ──────────────────                             ──────────────────
  ┌─────────────────────────┐                    ┌──────────────────────────────────────────────────┐
  │  src/                   │                    │  site/                                            │
  │  ├── lib/stores/        │   NO shared code   │  ├── src/                                        │
  │  │   (Tauri IPC stores) │  ◄─── boundary ──► │  │   ├── routes/            (SvelteKit pages)     │
  │  ├── lib/components/    │                    │  │   │   ├── (landing)/     (hero, features)      │
  │  │   (Desktop UI)       │                    │  │   │   ├── docs/          (user + tech docs)    │
  │  └── App.svelte         │                    │  │   │   ├── download/      (platform detect)     │
  │                         │                    │  │   │   └── sandbox/       (WebRTC demo)         │
  ├─────────────────────────┤                    │  │   ├── lib/                                     │
  │  src-tauri/             │                    │  │   │   ├── components/    (site UI components)   │
  │  ├── src/network/       │                    │  │   │   ├── sandbox/       (WebRTC P2P engine)    │
  │  │   (libp2p)           │                    │  │   │   └── content/       (markdown docs)        │
  │  ├── src/chat/          │                    │  │   └── app.html                                 │
  │  │   (Automerge)        │                    │  ├── static/                (assets, icons)        │
  │  └── src/voice/         │                    │  ├── svelte.config.js       (adapter-static)       │
  │      (cpal + Opus)      │                    │  ├── vite.config.ts                                │
  │                         │                    │  └── package.json           (independent deps)     │
  └─────────────────────────┘                    └──────────────────────────────────────────────────┘

  Build: `npm run build`                          Build: `cd site && npm run build`
  Output: dist/ (Tauri webview)                   Output: site/build/ (static HTML)
  Deploy: Tauri bundler                           Deploy: Cloudflare Pages
```

**Critical design decision:** The `/site` directory is a **fully independent SvelteKit project** with its own `package.json`, `svelte.config.js`, and `vite.config.ts`. It does NOT share code with the desktop app's `src/` directory. This avoids the known monorepo `$lib` alias conflicts (where SvelteKit's Vite plugin injects `$lib` that resolves relative to the wrong package) and keeps the Tauri app's build pipeline untouched.

### Component Responsibilities

| Component | Responsibility | Implementation |
|-----------|----------------|----------------|
| **Landing pages** (`routes/`) | Marketing content, vision, feature showcase | Static SvelteKit pages, prerendered HTML |
| **Documentation** (`routes/docs/`) | User guides + technical architecture docs | Markdown files processed by mdsvex |
| **Download page** (`routes/download/`) | Platform detection, installer links to GitHub Releases | Client-side JS with `navigator.userAgentData` |
| **WebRTC sandbox** (`routes/sandbox/`) | Interactive P2P demo: connect two browsers, send messages | WebRTC DataChannel with lightweight signaling |
| **Sandbox engine** (`lib/sandbox/`) | WebRTC connection management, signaling, message protocol | PeerJS (or raw RTCPeerConnection) + signaling worker |
| **Site components** (`lib/components/`) | Reusable UI: nav, footer, code blocks, diagrams | Svelte 5 components, site-specific design system |

## Recommended Project Structure

```
aether/
├── src/                     # EXISTING -- Tauri desktop app frontend (unchanged)
├── src-tauri/               # EXISTING -- Rust backend (unchanged)
├── package.json             # EXISTING -- desktop app deps (unchanged)
├── vite.config.ts           # EXISTING -- desktop app Vite config (unchanged)
├── svelte.config.js         # EXISTING -- desktop app Svelte config (unchanged)
│
├── site/                    # NEW -- marketing site (independent SvelteKit app)
│   ├── package.json         # Site-specific dependencies
│   ├── svelte.config.js     # adapter-static + mdsvex config
│   ├── vite.config.ts       # Site Vite config (port 5173)
│   ├── tsconfig.json        # Site TypeScript config
│   │
│   ├── src/
│   │   ├── app.html         # HTML shell
│   │   ├── app.css          # Global site styles
│   │   │
│   │   ├── routes/
│   │   │   ├── +layout.svelte          # Site-wide nav + footer
│   │   │   ├── +layout.ts             # Prerender all pages
│   │   │   ├── +page.svelte           # Landing / hero page
│   │   │   │
│   │   │   ├── features/
│   │   │   │   └── +page.svelte       # Feature deep-dive
│   │   │   │
│   │   │   ├── docs/
│   │   │   │   ├── +layout.svelte     # Docs sidebar layout
│   │   │   │   ├── +page.svelte       # Docs index
│   │   │   │   ├── [...slug]/
│   │   │   │   │   ├── +page.svelte   # Dynamic doc renderer
│   │   │   │   │   └── +page.ts       # Load markdown content
│   │   │   │   └── content/           # Markdown source files
│   │   │   │       ├── getting-started.md
│   │   │   │       ├── swarms.md
│   │   │   │       ├── voice-chat.md
│   │   │   │       ├── architecture.md
│   │   │   │       └── protocol.md
│   │   │   │
│   │   │   ├── download/
│   │   │   │   └── +page.svelte       # Platform detection + download links
│   │   │   │
│   │   │   └── sandbox/
│   │   │       └── +page.svelte       # Interactive WebRTC demo
│   │   │
│   │   └── lib/
│   │       ├── components/
│   │       │   ├── Nav.svelte          # Site navigation
│   │       │   ├── Footer.svelte       # Site footer
│   │       │   ├── Hero.svelte         # Landing hero section
│   │       │   ├── FeatureCard.svelte  # Feature showcase card
│   │       │   ├── CodeBlock.svelte    # Syntax-highlighted code
│   │       │   ├── PlatformCard.svelte # Download platform card
│   │       │   └── DocsSidebar.svelte  # Docs navigation sidebar
│   │       │
│   │       ├── sandbox/
│   │       │   ├── peer.ts            # WebRTC peer connection manager
│   │       │   ├── signaling.ts       # Signaling channel abstraction
│   │       │   ├── protocol.ts        # Message types (mirrors Aether concepts)
│   │       │   ├── store.svelte.ts    # Sandbox reactive state ($state runes)
│   │       │   └── ui/
│   │       │       ├── ChatDemo.svelte      # Text chat demo panel
│   │       │       ├── ConnectionPanel.svelte # Connect/disconnect UI
│   │       │       ├── PeerStatus.svelte    # Peer connection status
│   │       │       └── MessageList.svelte   # Message display
│   │       │
│   │       └── utils/
│   │           ├── platform.ts        # OS/arch detection for downloads
│   │           └── github-releases.ts # Fetch latest release assets from GH API
│   │
│   ├── static/
│   │   ├── favicon.ico
│   │   ├── og-image.png          # Social sharing image
│   │   └── screenshots/          # App screenshots for marketing
│   │
│   └── build/                    # Output (gitignored)
│
├── .github/
│   └── workflows/
│       ├── ci.yml               # EXISTING -- Tauri CI
│       ├── release.yml          # EXISTING -- Tauri release build
│       └── site.yml             # NEW -- site build + deploy to Cloudflare Pages
│
└── .gitignore                   # Add: site/node_modules/, site/build/, site/.svelte-kit/
```

### Structure Rationale

- **`site/` at repo root (not `apps/site/`):** No need for a `packages/` or `apps/` directory restructure. The existing desktop app is the repo root project; adding a single `/site` directory is minimal disruption. Turborepo is overkill for two projects.
- **Independent `package.json`:** The desktop app uses `svelte` (SPA via Vite) while the site uses `@sveltejs/kit` (SSG). These have different dependency trees and should not share a root `node_modules`. No workspace configuration needed.
- **`lib/sandbox/` separate from `lib/components/`:** The WebRTC sandbox engine is complex enough to warrant its own module. Components are UI; sandbox is a P2P networking layer.
- **`routes/docs/[...slug]/`:** Catch-all route for documentation pages. Markdown files live in `content/` and are loaded by the `+page.ts` loader, rendered via mdsvex.

## Architectural Patterns

### Pattern 1: Static-First with Client-Side Islands

**What:** All pages are statically prerendered at build time via `adapter-static`. The sandbox page is the sole "island" that hydrates client-side JavaScript for WebRTC interactivity.

**When to use:** Marketing sites where SEO matters, content changes infrequently, and only specific pages need interactivity.

**Trade-offs:**
- Pro: Zero server costs, instant page loads, CDN-cacheable everywhere
- Pro: Simple deployment (just static files to Cloudflare Pages)
- Con: Sandbox demo requires client-side JavaScript (no SSR for WebRTC)
- Con: Documentation updates require rebuild + redeploy

**Configuration:**

```typescript
// site/src/routes/+layout.ts
export const prerender = true;  // Prerender all pages by default
```

```typescript
// site/src/routes/sandbox/+page.ts
export const ssr = false;  // Sandbox is client-only (WebRTC needs browser APIs)
```

```javascript
// site/svelte.config.js
import adapter from '@sveltejs/adapter-static';
import { mdsvex } from 'mdsvex';

export default {
  extensions: ['.svelte', '.md'],
  preprocess: [vitePreprocess(), mdsvex({ extensions: ['.md'] })],
  kit: {
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: undefined,  // No SPA fallback -- fully static
      precompress: true      // Gzip + Brotli for Cloudflare
    })
  }
};
```

### Pattern 2: Manual SDP Signaling for Zero-Infrastructure Demo

**What:** The WebRTC sandbox uses a **copy-paste signaling** approach where Peer A generates an SDP offer (encoded as a compact string), shares it with Peer B (who pastes it in), and Peer B returns an SDP answer. No signaling server required.

**When to use:** Demos where you want to emphasize the "zero infrastructure" philosophy of the product. Perfectly mirrors Aether's "Secret Code" invite flow.

**Trade-offs:**
- Pro: Zero server costs -- completely serverless P2P demo
- Pro: Philosophically aligned with Aether's "Sovereign Node" thesis
- Pro: Users experience the manual connection flow, building understanding of P2P
- Con: Multi-step UX (copy-paste-copy-paste) adds friction
- Con: Cannot do "instant" demo connections without a signaling intermediary

**Example:**

```typescript
// site/src/lib/sandbox/signaling.ts
export interface SignalingPayload {
  sdp: RTCSessionDescriptionInit;
  candidates: RTCIceCandidateInit[];
}

export function encodeOffer(payload: SignalingPayload): string {
  // Compress and base64-encode the SDP + candidates into a shareable string
  const json = JSON.stringify(payload);
  return btoa(json);
}

export function decodeOffer(encoded: string): SignalingPayload {
  const json = atob(encoded);
  return JSON.parse(json);
}
```

**Upgrade path:** If copy-paste friction is too high, add a lightweight Cloudflare Worker signaling relay (see Pattern 3). The sandbox code should abstract signaling behind an interface so the transport is swappable.

### Pattern 3: Cloudflare Worker Signaling Relay (Optional Upgrade)

**What:** A minimal Cloudflare Worker that acts as a WebSocket-based signaling relay. Two peers connect to the same "room" (identified by a short code), exchange SDP offers/answers through the Worker, then communicate directly via WebRTC DataChannel. The Worker handles zero data after connection -- it is purely a matchmaker.

**When to use:** When the copy-paste UX is deemed too frictional for a marketing demo. Adds a small infrastructure cost but massively improves first-visit experience.

**Trade-offs:**
- Pro: One-click demo experience ("Share this link to connect")
- Pro: Cloudflare Workers free tier allows 100K requests/day -- more than enough
- Pro: Worker handles no data after signaling -- aligns with "zero infrastructure" messaging
- Con: Introduces a server dependency (even if minimal)
- Con: Additional code to maintain (Cloudflare Worker + Durable Object for room state)

**Architecture:**

```
Browser A                   Cloudflare Worker              Browser B
─────────                   ─────────────────              ─────────
1. Create room ──────────►  Store in Durable Object
2. Get room code ◄────────  Return short code (e.g., "AETHER-7X3K")
                                                    3. Join room code ──────────►
4. SDP Offer ────────────►  Relay via WebSocket  ──────────► 5. Receive offer
6. Receive answer ◄───────  Relay via WebSocket  ◄────────── 7. SDP Answer
8. ICE candidates ───────►  Relay  ──────────────────────► 9. ICE candidates
                            (Worker goes idle)
10. DataChannel established ◄─────────────────────────────► 11. DataChannel established
     Direct P2P communication (Worker no longer involved)
```

### Pattern 4: GitHub Releases as Download CDN

**What:** The download page fetches the latest release metadata from GitHub's API (`GET /repos/{owner}/{repo}/releases/latest`), detects the visitor's platform, and presents the correct installer link. No custom download infrastructure.

**When to use:** When your app is already distributed via GitHub Releases (Aether already has this via `release.yml` workflow).

**Trade-offs:**
- Pro: Zero cost, leverages existing release pipeline
- Pro: GitHub handles bandwidth, checksums, and CDN
- Con: GitHub API rate limit (60 req/hr unauthenticated) -- mitigated by caching in static build or client-side caching
- Con: Requires GitHub repo to be public (or use a token)

**Example:**

```typescript
// site/src/lib/utils/platform.ts
export type Platform = 'macos-arm64' | 'macos-x64' | 'windows-x64' | 'linux-x64' | 'unknown';

export function detectPlatform(): Platform {
  // Use modern API first, fall back to userAgent
  const ua = navigator.userAgent.toLowerCase();

  if (ua.includes('mac')) {
    // Apple Silicon detection via WebGL renderer
    const canvas = document.createElement('canvas');
    const gl = canvas.getContext('webgl');
    const renderer = gl?.getParameter(gl.RENDERER) || '';
    if (renderer.includes('Apple') && !renderer.includes('Intel')) {
      return 'macos-arm64';
    }
    return 'macos-x64';
  }
  if (ua.includes('win')) return 'windows-x64';
  if (ua.includes('linux')) return 'linux-x64';
  return 'unknown';
}
```

```typescript
// site/src/lib/utils/github-releases.ts
interface ReleaseAsset {
  name: string;
  browser_download_url: string;
  size: number;
}

interface Release {
  tag_name: string;
  assets: ReleaseAsset[];
}

const REPO = 'username/aether';  // Update with actual repo

export async function getLatestRelease(): Promise<Release> {
  const res = await fetch(`https://api.github.com/repos/${REPO}/releases/latest`);
  return res.json();
}

export function getAssetForPlatform(release: Release, platform: Platform): ReleaseAsset | null {
  const patterns: Record<Platform, RegExp> = {
    'macos-arm64': /\.dmg$/i,       // or filter by aarch64 in filename
    'macos-x64':   /\.dmg$/i,
    'windows-x64': /\.exe$/i,       // NSIS installer
    'linux-x64':   /\.AppImage$/i,
    'unknown':     /never/
  };
  return release.assets.find(a => patterns[platform].test(a.name)) ?? null;
}
```

## Data Flow

### WebRTC Sandbox Data Flow

```
User visits /sandbox
    │
    ├── "Create Room" path (Peer A)
    │   │
    │   ├── new RTCPeerConnection(config)
    │   ├── pc.createDataChannel("aether-demo")
    │   ├── pc.createOffer() → pc.setLocalDescription()
    │   ├── Gather ICE candidates (trickle or wait for complete)
    │   ├── Encode offer + candidates → display as shareable code
    │   │
    │   └── User shares code with Peer B (copy-paste or room link)
    │
    ├── "Join Room" path (Peer B)
    │   │
    │   ├── Decode received offer
    │   ├── new RTCPeerConnection(config)
    │   ├── pc.setRemoteDescription(offer)
    │   ├── pc.createAnswer() → pc.setLocalDescription()
    │   ├── Gather ICE candidates
    │   ├── Encode answer + candidates → display for Peer A
    │   │
    │   └── DataChannel opens on "ondatachannel" event
    │
    └── Connected state (both peers)
        │
        ├── DataChannel.send(JSON.stringify({ type: 'message', content, sender, ts }))
        ├── DataChannel.onmessage → parse → update sandbox store
        └── Connection status monitored via pc.oniceconnectionstatechange
```

### Sandbox State Management

```
┌──────────────────────────────────────────────┐
│  sandbox/store.svelte.ts ($state runes)      │
│                                              │
│  connectionState: 'idle' | 'offering' |      │
│                   'answering' | 'connected'  │
│  localOffer: string | null                   │
│  remoteAnswer: string | null                 │
│  messages: Message[]                         │
│  peerId: string (random display name)        │
│  dataChannel: RTCDataChannel | null          │
│                                              │
│  Methods:                                    │
│    createOffer() → sets connectionState,     │
│                    generates localOffer       │
│    acceptOffer(code) → processes offer,      │
│                        generates answer       │
│    acceptAnswer(code) → completes handshake  │
│    sendMessage(text) → sends via DataChannel │
│    disconnect() → cleanup                    │
└──────────────────────────────────────────────┘
         │
         ▼
┌──────────────────────────────────────────────┐
│  sandbox/ui/ components                      │
│                                              │
│  ConnectionPanel.svelte                      │
│    └── reads connectionState, shows          │
│        appropriate step (create/join/paste)   │
│                                              │
│  ChatDemo.svelte                             │
│    └── reads messages[], calls sendMessage() │
│                                              │
│  PeerStatus.svelte                           │
│    └── reads connectionState + ICE state     │
└──────────────────────────────────────────────┘
```

### Key Data Flows

1. **Documentation rendering:** Markdown files in `content/` are loaded by `+page.ts` at build time, processed through mdsvex (Markdown + Svelte components), and prerendered to static HTML. No runtime processing.

2. **Download page:** On page mount, `detectPlatform()` runs client-side, then `getLatestRelease()` fetches from GitHub API. The page shows the detected platform's installer prominently, with other platforms listed below. GitHub Release asset URLs are direct download links.

3. **Sandbox messaging:** Messages flow exclusively through the WebRTC DataChannel after connection. No server touches the data. Messages are JSON-encoded (`{ type, content, sender, timestamp }`) and parsed on receipt. The sandbox store holds the message array and UI components reactively render it.

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| 0-1k visitors/day | Static site on Cloudflare Pages free tier. Manual SDP signaling. GitHub API calls for downloads. No concerns. |
| 1k-10k visitors/day | Add Cloudflare Worker signaling relay if copy-paste friction hurts conversion. Cache GitHub Release data at build time (bake into static HTML) to avoid API rate limits. |
| 10k+ visitors/day | Worker signaling with Durable Objects for room state. TURN server (Cloudflare or Metered.ca free tier) for NAT-traversed WebRTC connections. Pre-signed download URLs to avoid GitHub API dependency. |

### Scaling Priorities

1. **First bottleneck: GitHub API rate limit.** At 60 unauthenticated requests/hour, the download page breaks at moderate traffic. Fix: Bake release metadata into static HTML at build time (fetch during `npm run build` and inject as static data). Client-side fetch becomes a fallback for stale data.

2. **Second bottleneck: WebRTC NAT traversal.** STUN works for most users, but symmetric NATs (corporate networks, mobile carriers) require TURN. Fix: Add a free TURN server (Google's public STUN `stun:stun.l.google.com:19302` is already free; for TURN, Cloudflare Calls or Metered.ca free tier).

## Anti-Patterns

### Anti-Pattern 1: Sharing Code Between Desktop App and Marketing Site

**What people do:** Create a shared `packages/common/` directory with types, components, or utilities used by both the Tauri desktop app and the SvelteKit marketing site.

**Why it's wrong:** The desktop app uses Tauri IPC (`invoke`, `listen`) throughout its stores and components. The marketing site has zero Tauri dependency. Shared components inevitably import Tauri APIs or depend on store shapes that assume IPC. SvelteKit's `$lib` alias also conflicts in monorepo setups, resolving to the wrong package. The maintenance burden of keeping shared code compatible across two very different runtime environments exceeds the benefit.

**Do this instead:** Keep the codebases independent. If you need similar visual styling, share CSS variables or a Tailwind config -- not Svelte components. The site's design system should be purpose-built for marketing (large hero sections, documentation layouts) which has nothing in common with the desktop app's chat/voice UI.

### Anti-Pattern 2: Using PeerJS Cloud for Production Signaling

**What people do:** Use PeerJS's free cloud signaling server (`0.peerjs.com`) for the sandbox demo, avoiding the need to run any signaling infrastructure.

**Why it's wrong:** PeerJS's free cloud server is explicitly "not suitable for production." It has no SLA, goes down periodically, and adds a third-party dependency that contradicts Aether's sovereignty message. If the PeerJS cloud server goes down, your marketing demo is broken.

**Do this instead:** Use manual SDP copy-paste signaling (zero infrastructure, on-brand) or self-host a minimal Cloudflare Worker relay (free tier, you control it). Both approaches align with Aether's "no middleman" philosophy.

### Anti-Pattern 3: Server-Side Rendering the Sandbox Page

**What people do:** Attempt to SSR the sandbox page, resulting in build errors because WebRTC APIs (`RTCPeerConnection`, `RTCDataChannel`) do not exist in Node.js.

**Why it's wrong:** WebRTC is a browser-only API. Any import of sandbox code during SSR crashes the build. Dynamic imports and `browser` checks add complexity for no benefit -- the sandbox has no meaningful server-rendered content (it is all interactive).

**Do this instead:** Set `export const ssr = false` on the sandbox route. The page renders a loading state on the server and hydrates fully on the client. All other pages remain statically prerendered.

### Anti-Pattern 4: Restructuring the Entire Repo into apps/packages

**What people do:** When adding a second project, restructure the entire repo into `apps/desktop/`, `apps/site/`, `packages/shared/` with Turborepo or Nx, moving all existing files.

**Why it's wrong:** The Tauri desktop app has path-dependent configuration everywhere (`tauri.conf.json` references `../dist`, CI workflows reference `./src-tauri`, etc.). Moving files breaks every path reference and CI pipeline. The restructure cost is high and the benefit is near-zero for two independent projects.

**Do this instead:** Add `/site` as a peer directory at the repo root. It has its own build pipeline. The only shared artifact is the Git repository and GitHub Actions CI.

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| **Cloudflare Pages** | Git-connected deploy from `/site` directory | Set build command to `cd site && npm run build`, output dir to `site/build` |
| **GitHub Releases API** | Client-side fetch or build-time static data injection | Rate limit: 60/hr unauth. Cache at build time for reliability. |
| **Google STUN** | `stun:stun.l.google.com:19302` in RTCPeerConnection config | Free, public, reliable. Sufficient for demo. |
| **Cloudflare Workers** (optional) | WebSocket signaling relay for sandbox | Only if manual SDP UX is rejected. Free tier: 100K req/day. |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| Desktop app (`src/`) <-> Site (`site/`) | **None -- fully independent** | No shared code, no shared build, no shared dependencies. They happen to be in the same Git repo. |
| Site pages <-> Sandbox engine (`lib/sandbox/`) | Svelte store imports | Sandbox store uses `$state` runes, same pattern as desktop app stores. |
| Sandbox engine <-> Browser WebRTC | Native `RTCPeerConnection` API | No abstraction library needed for a demo. Raw API keeps bundle small. |
| Site <-> GitHub Releases | REST API (`fetch`) | Consider build-time caching to avoid runtime API calls. |
| Site build <-> Cloudflare Pages | `adapter-static` output | Cloudflare Pages auto-detects static output. No server functions needed. |
| Docs content <-> mdsvex | Markdown preprocessing at build time | `.md` files become Svelte components. Supports Svelte component embeds in docs. |

### CI/CD Integration

The existing CI workflows (`ci.yml`, `release.yml`) remain untouched. A new `site.yml` workflow handles the marketing site:

```yaml
# .github/workflows/site.yml
name: Site

on:
  push:
    branches: [main]
    paths: ['site/**']
  pull_request:
    branches: [main]
    paths: ['site/**']

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: 20 }
      - run: cd site && npm ci && npm run build
      # Cloudflare Pages handles deploy via Git integration
      # This job just verifies the build succeeds on PRs
```

**Deploy strategy:** Connect Cloudflare Pages directly to the GitHub repo. Set the build configuration:
- Build command: `cd site && npm install && npm run build`
- Build output directory: `site/build`
- Root directory: `/` (repo root, Cloudflare handles the `cd site`)

This means pushes to `main` that touch `site/` auto-deploy. The desktop app's CI runs independently -- they do not block each other.

## Sandbox Protocol Design

The WebRTC sandbox should mirror Aether's concepts to educate users about the product:

```typescript
// site/src/lib/sandbox/protocol.ts

// Mirror Aether's message types in a simplified browser form
export interface SandboxMessage {
  type: 'chat';
  id: string;           // UUID v4
  sender: string;       // Random display name (like Aether's self-asserted profiles)
  content: string;      // Plain text (like Aether's text-only constraint)
  timestamp: number;    // Unix ms
}

export interface SandboxControl {
  type: 'identity';     // Announce display name on connect
  sender: string;
  displayName: string;
}

export type SandboxPayload = SandboxMessage | SandboxControl;

// DataChannel message handler
export function handleMessage(raw: string): SandboxPayload {
  return JSON.parse(raw) as SandboxPayload;
}
```

This design intentionally mirrors Aether's real protocol concepts:
- **Self-asserted identity:** Users pick a display name (like Aether's `IDEN-02`)
- **Plain text messages:** No rich text (like Aether's current scope)
- **Peer-to-peer data flow:** WebRTC DataChannel carries all messages (like libp2p streams)
- **Connection via shared code:** SDP offer/answer mirrors Aether's Secret Code invite flow

The sandbox is a teaching tool. It should make visitors think "this is cool, I want the full app."

## Sources

- [SvelteKit adapter-static docs](https://svelte.dev/docs/kit/adapter-static) -- HIGH confidence
- [SvelteKit Cloudflare deployment](https://developers.cloudflare.com/pages/framework-guides/deploy-a-svelte-kit-site/) -- HIGH confidence
- [SvelteKit monorepo $lib alias issue](https://github.com/sveltejs/kit/issues/14518) -- HIGH confidence (known bug)
- [mdsvex for Markdown in SvelteKit](https://joshcollinsworth.com/blog/build-static-sveltekit-markdown-blog) -- MEDIUM confidence
- [WebRTC manual SDP signaling demo](https://github.com/david-tkalcec/webrtc-manual-sdp-signaling) -- HIGH confidence
- [P2PCF: Cloudflare Workers signaling](https://github.com/gfodor/p2pcf) -- MEDIUM confidence
- [PeerJS docs](https://peerjs.com/) -- HIGH confidence (evaluated, not recommended for production)
- [WebRTC DataChannel overview](https://webrtcforthecurious.com/docs/02-signaling/) -- HIGH confidence
- [Durable Objects for WebSocket state](https://dev.to/kiyoe/zero-egress-costs-how-i-built-p2p-file-sharing-with-cloudflare-4lhc) -- MEDIUM confidence
- [Browser platform detection (MDN)](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Browser_detection_using_the_user_agent) -- HIGH confidence
- [Tauri release workflow](https://github.com/tauri-apps/tauri-action) -- HIGH confidence (already in repo)
- [Turborepo SvelteKit starter](https://vercel.com/templates/svelte/turborepo-sveltekit-starter) -- MEDIUM confidence (evaluated, not recommended)

---
*Architecture research for: Aether v2.0 Marketing Site + WebRTC Sandbox*
*Researched: 2026-03-04*

# Stack Research: v2.0 Marketing Site

**Domain:** Marketing website + documentation + interactive WebRTC P2P demo
**Researched:** 2026-03-04
**Confidence:** HIGH
**Scope:** Stack ADDITIONS for the `/site` directory (SvelteKit marketing site). Existing desktop app stack (Tauri v2, libp2p 0.56, Automerge, cpal, Opus, ed25519-dalek) is validated and NOT re-evaluated.

## Existing Stack Reference (DO NOT CHANGE)

| Technology | Installed Version | Purpose |
|------------|-------------------|---------|
| Svelte | 5.50.3 | UI framework |
| Vite | 6.4.1 | Build tool |
| TypeScript | 5.9.3 | Type safety |
| @tauri-apps/api | ^2.0.0 | Desktop API bindings |
| @sveltejs/vite-plugin-svelte | ^5.0.0 | Svelte Vite integration |

The marketing site shares Svelte 5 with the desktop app but runs as a standalone SvelteKit project in `/site`. It does NOT import Tauri APIs. The two codebases coexist in the monorepo but build independently.

---

## Monorepo Strategy

**Use npm workspaces** (not pnpm, not Turborepo) because the existing project uses npm with `package-lock.json`. Switching package managers mid-project adds friction with no proportional benefit for a two-package workspace.

```jsonc
// Root package.json additions:
{
  "workspaces": ["site"],
  "scripts": {
    "site:dev": "npm -w site run dev",
    "site:build": "npm -w site run build",
    "site:preview": "npm -w site run preview"
  }
}
```

The desktop app keeps its existing root `package.json`. The marketing site gets its own `site/package.json`. No shared packages needed -- the two apps have completely different dependency trees (Tauri plugins vs. SvelteKit adapters).

---

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| SvelteKit | ^2.53 | Full-stack web framework | Already using Svelte 5 in the desktop app. SvelteKit adds routing, SSG/SSR, adapters, and `<enhanced:img>`. Zero learning curve for the existing team. |
| @sveltejs/adapter-cloudflare | ^6.3 | Deployment adapter | Enables mixed prerendered + dynamic pages on Cloudflare. Unlimited free bandwidth, 500 builds/month on free tier, global edge CDN. Best cost profile for a $0-infra project. |
| Tailwind CSS | ^4.2 | Utility-first CSS | v4 removes PostCSS dependency and config files -- just `@import "tailwindcss"` in CSS. Integrates via `@tailwindcss/vite` plugin. Fast iteration for marketing pages. |
| mdsvex | ^0.12 | Markdown in Svelte | Renders `.md` files as Svelte components. Documentation pages written in Markdown with embedded interactive Svelte components (architecture diagrams, code examples). Standard SvelteKit documentation pattern. |
| Shiki | ^4.0 | Syntax highlighting | Used by Astro, VitePress, and SvelteKit ecosystem. Accurate TextMate grammar-based highlighting with VS Code themes. Configured as mdsvex custom highlighter. |

### WebRTC Demo Sandbox

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Native WebRTC API | Browser built-in | P2P data channels | No library needed. The demo is a controlled sandbox where two browser tabs exchange messages. Using `RTCPeerConnection` + `RTCDataChannel` directly keeps the bundle tiny and teaches users what Aether does under the hood. |
| Manual SDP exchange | N/A | Signaling substitute | Users copy-paste SDP offers/answers between tabs (or use a lightweight in-page relay). This mirrors Aether's "Secret Code" invite flow and avoids needing a signaling server -- keeping the $0-infra philosophy. |
| Google STUN | stun:stun.l.google.com:19302 | NAT traversal | Free public STUN server for ICE candidate gathering. Sufficient for a demo (not production voice). No TURN server needed since the demo is text-only data channels. |

**Why NOT PeerJS or simple-peer:**
- PeerJS (v1.5.5) requires a PeerJS signaling server (`peer` package, v1.0.2, 2+ years stale). This contradicts the zero-server philosophy and adds infrastructure cost.
- simple-peer (v9.11.1) has not been published in 4+ years. Effectively unmaintained.
- The demo is intentionally minimal -- ~150 lines of vanilla WebRTC. A library adds bundle size and abstracts away the very thing we want to demonstrate (raw P2P connections).

**Why NOT @libp2p/webrtc (v6.0.11):**
- @libp2p/webrtc is designed for libp2p protocol negotiation, not standalone browser demos. It pulls in the entire js-libp2p stack (~200KB+ gzipped).
- The marketing demo shows the *concept* of P2P, not the actual libp2p protocol. The desktop app uses Rust libp2p; the browser demo is a teaching tool, not a port.

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| @sveltejs/enhanced-img | ^0.10 | Image optimization | Marketing page hero images, screenshots, team photos. Auto-generates avif/webp, sets width/height to prevent CLS. Build-time only, no runtime cost. |
| @shikijs/transformers | ^4.0 | Code block enhancements | Line highlighting, diff markers, focus lines in technical documentation code examples. |
| rehype-slug | ^6.0 | Heading anchors | Adds `id` attributes to headings in mdsvex-rendered documentation pages. Enables deep linking to sections. |
| rehype-autolink-headings | ^7.0 | Heading link icons | Adds clickable anchor links next to headings in docs. Standard documentation UX pattern. |
| remark-gfm | ^4.0 | GitHub-flavored Markdown | Tables, task lists, strikethrough in documentation. mdsvex uses remark under the hood. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| @sveltejs/kit | ^2.53 | SvelteKit CLI/framework | Installed in `site/package.json`, not root |
| svelte-check | ^4.0 | Type checking for Svelte | Run `svelte-check` in CI for the `/site` workspace |
| @types/node | ^22 | Node.js type definitions | For SvelteKit server-side code (adapter hooks) |

---

## Installation

```bash
# From project root, create the site workspace
mkdir -p site

# Initialize SvelteKit project in /site
cd site
npm create svelte@latest .
# Select: Skeleton project, TypeScript, ESLint, Prettier

# Core framework (installed by create-svelte)
npm install @sveltejs/kit svelte

# Deployment adapter
npm install @sveltejs/adapter-cloudflare

# Styling
npm install tailwindcss @tailwindcss/vite

# Documentation / Markdown
npm install mdsvex shiki @shikijs/transformers
npm install rehype-slug rehype-autolink-headings remark-gfm

# Image optimization
npm install -D @sveltejs/enhanced-img

# Dev tools
npm install -D svelte-check @types/node
```

Back at root, wire up the workspace:

```bash
# In root package.json, add:
# "workspaces": ["site"]

# Then install to link workspaces:
cd /path/to/aether
npm install
```

---

## Key Configuration Files

### site/svelte.config.js

```javascript
import adapter from '@sveltejs/adapter-cloudflare';
import { mdsvex } from 'mdsvex';
import { createHighlighter } from 'shiki';
import rehypeSlug from 'rehype-slug';
import rehypeAutolinkHeadings from 'rehype-autolink-headings';
import remarkGfm from 'remark-gfm';

/** @type {import('mdsvex').MdsvexOptions} */
const mdsvexOptions = {
  extensions: ['.md'],
  remarkPlugins: [remarkGfm],
  rehypePlugins: [rehypeSlug, rehypeAutolinkHeadings],
  highlight: {
    highlighter: async (code, lang) => {
      const highlighter = await createHighlighter({
        themes: ['github-dark'],
        langs: [lang || 'text']
      });
      const html = highlighter.codeToHtml(code, { lang: lang || 'text', theme: 'github-dark' });
      return `{@html \`${html}\`}`;
    }
  }
};

export default {
  extensions: ['.svelte', '.md'],
  preprocess: [mdsvex(mdsvexOptions)],
  kit: {
    adapter: adapter(),
    prerender: {
      // Prerender all marketing and docs pages
      entries: ['*']
    }
  }
};
```

### site/vite.config.ts

```typescript
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { enhancedImages } from '@sveltejs/enhanced-img';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    enhancedImages(), // Must come before sveltekit()
    tailwindcss(),
    sveltekit()
  ]
});
```

---

## Deployment: Cloudflare Pages

**Why Cloudflare Pages over Vercel or Netlify:**

| Criterion | Cloudflare Pages | Vercel | Netlify |
|-----------|-----------------|--------|---------|
| Free bandwidth | Unlimited | 100 GB/mo | 100 GB/mo |
| Free builds | 500/month | 100 hours/mo | 300 min/mo |
| Edge locations | 310+ cities | ~30 regions | ~10 regions |
| Custom domain SSL | Free, auto | Free, auto | Free, auto |
| Serverless functions | Workers (if needed) | Serverless Functions | Netlify Functions |
| Cost at scale | $0 for static | $20+/mo at traffic | $19+/mo at traffic |
| SvelteKit adapter | Official `adapter-cloudflare` | Official `adapter-vercel` | Official `adapter-netlify` |

**Recommendation: Cloudflare Pages** because Aether's marketing site will be almost entirely prerendered (static). Unlimited free bandwidth means zero cost regardless of traffic spikes from launch announcements or HN front page. The only dynamic page might be a download redirect based on user-agent (detecting OS for platform-specific binary links), which Cloudflare Workers handles trivially.

**Setup:**
1. Connect GitHub repo to Cloudflare Pages dashboard
2. Set build command: `cd site && npm run build`
3. Set output directory: `site/.svelte-kit/cloudflare`
4. Cloudflare auto-deploys on push to `main`, preview deploys on PRs

---

## Page Rendering Strategy

| Page | Rendering | Rationale |
|------|-----------|-----------|
| Landing page (`/`) | Prerendered (static) | Content changes rarely. Maximum performance. |
| Features (`/features`) | Prerendered | Static marketing content. |
| Docs (`/docs/*`) | Prerendered | Markdown content, no dynamic data. |
| Technical docs (`/technical/*`) | Prerendered | Markdown content, no dynamic data. |
| Download (`/download`) | SSR via Worker | Detects user-agent to highlight correct platform binary. Falls back gracefully if JS disabled. |
| Demo (`/demo`) | Prerendered shell + client-side JS | The page shell is static. WebRTC code runs entirely client-side. |

Set `export const prerender = true` on all routes except `/download`. The download page uses SSR to read the `User-Agent` header and pre-select the correct platform binary.

---

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| SvelteKit (monorepo `/site`) | Astro + Starlight | If documentation were the *only* goal and no interactive demo was needed. Starlight is purpose-built for docs but cannot easily host an interactive WebRTC sandbox. SvelteKit handles both marketing + docs + interactive pages in one framework. |
| mdsvex | Contentlayer / Content Collections | If using Astro. mdsvex is the natural choice for SvelteKit -- it's listed in the official Svelte CLI docs and integrates as a preprocessor. |
| Tailwind CSS v4 | Plain CSS / CSS Modules | If the team prefers semantic class names. Tailwind is faster for marketing pages where you iterate on layout/spacing frequently. v4's CSS-first config eliminates the JS config file complexity. |
| Cloudflare Pages | Vercel | If you need server-side rendering for most pages (Vercel's serverless functions are more flexible). Not needed here -- site is 95% static. |
| Cloudflare Pages | GitHub Pages | If you need zero config and don't care about edge functions. GitHub Pages can't run the `/download` SSR route. |
| Native WebRTC API | PeerJS | If you need a production-grade signaling infrastructure. The demo is intentionally minimal and educational. |
| Shiki | Prism.js | If bundle size is critical and you don't need accurate highlighting. Shiki runs at build time via mdsvex, so runtime bundle impact is zero. |

## What NOT to Add

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| **@libp2p/webrtc** | Massive dependency tree for a simple demo. The demo shows the P2P *concept*, not the actual protocol. | Native `RTCPeerConnection` API (~0 KB) |
| **PeerJS** | Requires a signaling server (contradicts zero-infra philosophy). | Manual SDP exchange in the browser demo |
| **simple-peer** | Unmaintained (last publish 4+ years ago). | Native WebRTC API |
| **Docusaurus / VitePress** | Would introduce React/Vue into a Svelte project. Inconsistent DX and double the framework knowledge required. | mdsvex + SvelteKit routing |
| **Astro** | Adding a second meta-framework when SvelteKit already handles SSG + SSR + interactive pages. Two build configs, two routing systems. | SvelteKit with `adapter-cloudflare` |
| **@tauri-apps/api** in `/site` | The marketing site is a web app, not a Tauri app. Importing Tauri APIs would crash at runtime in browsers. | Keep Tauri imports in root `/src` only |
| **Three.js / Canvas animations** | Heavy 3D libraries for marketing flair. Adds 500KB+ to bundle for negligible conversion impact. | CSS animations + Svelte transitions (`svelte/transition`, `svelte/animate`) |
| **svelte-motion / Framer Motion ports** | Unnecessary dependency for marketing page animations. Svelte's built-in `transition:`, `animate:`, and `tweened`/`spring` stores cover all needs. | Built-in Svelte motion primitives |
| **Database (Supabase, Firebase, etc.)** | The marketing site has no user accounts, no dynamic content, no CMS. All content is in Markdown files in the repo. | Static Markdown files via mdsvex |
| **CMS (Sanity, Contentful, etc.)** | Over-engineering for a project with one maintainer. Markdown in git provides version history, PR review, and zero cost. | Markdown files in `/site/src/docs/` |
| **Analytics (Google Analytics, Plausible, etc.)** | Can be added later if needed. Not a build-time dependency. Defer to avoid scope creep. | Add in a later phase if metrics are needed |

---

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| @sveltejs/kit@^2.53 | svelte@^5.0 | SvelteKit 2 requires Svelte 5. Both already in use. |
| @sveltejs/adapter-cloudflare@^6.3 | @sveltejs/kit@^2.0 | Versioned in lockstep with Kit releases. |
| @sveltejs/enhanced-img@^0.10 | @sveltejs/kit@^2.0 | Pre-1.0 -- pin minor version and test before upgrading. May have breaking changes in 0.11. |
| tailwindcss@^4.2 | @tailwindcss/vite@^4.2 | v4 uses Vite plugin directly (no PostCSS). Must use matching versions. |
| mdsvex@^0.12 | svelte@^5.0 | v0.12 added Svelte 5 support. Earlier versions break with Svelte 5 runes. |
| shiki@^4.0 | mdsvex@^0.12 | Shiki 4 is ESM-only. mdsvex 0.12 supports ESM highlighters. |
| rehype-slug@^6.0 | mdsvex@^0.12 | rehype plugins work via mdsvex's unified pipeline. |
| remark-gfm@^4.0 | mdsvex@^0.12 | remark plugins work via mdsvex's unified pipeline. |

**Critical compatibility note:** mdsvex 0.12 and Shiki 4 are both ESM-only. The SvelteKit project must use `"type": "module"` in `package.json` (which `create-svelte` sets by default). If the root workspace `package.json` does not have `"type": "module"`, the site's own `package.json` must set it independently.

---

## Download Section: Binary Hosting

Platform-specific binaries (`.dmg`, `.msi`, `.AppImage`) should NOT be hosted on Cloudflare Pages (file size limits, not designed for large binary hosting).

**Recommendation: GitHub Releases**
- Tauri's CI pipeline already builds platform binaries
- GitHub Releases provides direct download URLs with no bandwidth limits for public repos
- The `/download` page links to `https://github.com/{org}/aether/releases/latest/download/Aether_{version}_{platform}.{ext}`
- User-agent detection on the SSR page pre-selects the correct download button

No additional infrastructure or CDN needed. GitHub handles the binary distribution.

---

## Sources

- [SvelteKit docs - adapter-static](https://svelte.dev/docs/kit/adapter-static) -- prerendering strategy, HIGH confidence
- [SvelteKit docs - adapter-cloudflare](https://svelte.dev/docs/kit/adapter-cloudflare) -- deployment config, HIGH confidence
- [SvelteKit docs - images](https://svelte.dev/docs/kit/images) -- @sveltejs/enhanced-img usage, HIGH confidence
- [@sveltejs/kit on npm](https://www.npmjs.com/package/@sveltejs/kit) -- v2.53.4 latest, HIGH confidence
- [@sveltejs/adapter-cloudflare on npm](https://www.npmjs.com/package/@sveltejs/adapter-cloudflare) -- v6.3 confirmed, HIGH confidence
- [@sveltejs/enhanced-img on npm](https://www.npmjs.com/package/@sveltejs/enhanced-img) -- v0.10.3 latest, HIGH confidence
- [mdsvex docs](https://mdsvex.pngwn.io/docs) -- configuration, highlighter API, HIGH confidence
- [mdsvex on npm](https://www.npmjs.com/package/mdsvex) -- v0.12.6, HIGH confidence
- [Svelte CLI mdsvex docs](https://svelte.dev/docs/cli/mdsvex) -- official integration guide, HIGH confidence
- [Tailwind CSS v4 SvelteKit guide](https://tailwindcss.com/docs/guides/sveltekit) -- Vite plugin setup, HIGH confidence
- [tailwindcss on npm](https://www.npmjs.com/package/tailwindcss) -- v4.2.1 latest, HIGH confidence
- [Shiki on npm](https://www.npmjs.com/package/shiki) -- v4.0.1 latest, HIGH confidence
- [Cloudflare Pages pricing](https://www.cloudflare.com/plans/developer-platform/) -- free tier confirmed unlimited bandwidth, HIGH confidence
- [PeerJS on npm](https://www.npmjs.com/package/peerjs) -- v1.5.5, requires signaling server, MEDIUM confidence
- [simple-peer on npm](https://www.npmjs.com/package/simple-peer) -- v9.11.1, last published 4+ years ago, HIGH confidence (for staleness claim)
- [@libp2p/webrtc on npm](https://www.npmjs.com/package/@libp2p/webrtc) -- v6.0.11 actively maintained but oversized for demo, HIGH confidence
- [WebRTC without signaling server](https://github.com/lesmana/webrtc-without-signaling-server) -- manual SDP pattern, MEDIUM confidence
- [Google STUN server list](https://dev.to/alakkadshaw/google-stun-server-list-21n4) -- stun.l.google.com:19302 confirmed, HIGH confidence
- [What's new in Svelte: March 2026](https://svelte.dev/blog/whats-new-in-svelte-march-2026) -- ecosystem status, HIGH confidence

---
*Stack research for: Aether v2.0 Marketing Site*
*Researched: 2026-03-04*

# Feature Landscape: v2.0 Marketing Site

**Domain:** Marketing Website for P2P Communication Desktop App
**Researched:** 2026-03-04
**Milestone:** v2.0 -- Public-facing marketing site with interactive P2P demo
**Confidence:** HIGH (marketing site patterns are well-established; P2P demo is the novel element)

---

## Table Stakes

Features visitors expect from a privacy/P2P communication tool's marketing website. Missing any of these makes the site feel unfinished or untrustworthy.

---

### 1. Vision-First Landing Page (Hero Section)

| Aspect | Detail |
|--------|--------|
| **Why Expected** | Every competitor (Signal, Element, Session, Briar, Jami, Tox) leads with a clear value proposition in the hero. Visitors decide within 3-5 seconds whether to stay. |
| **Complexity** | Low |
| **Depends On** | Brand identity (copy, color palette, typography), screenshot/illustration assets |

**What Competitors Do:**
- **Signal:** "Speak Freely" -- 5 words, immediately understood. Sub-line explains end-to-end encryption.
- **Session:** "Send Messages, Not Metadata" -- attacks the problem directly.
- **Briar:** "Secure messaging, anywhere" -- positions for hostile environments.
- **Jami:** "Share, freely and privately" -- emphasizes the freedom angle.
- **Element:** "Communicate on your terms" -- sovereignty framing.

**What Aether Should Do:**
- Lead with the "Sovereign Node" philosophy -- this IS the differentiator. Aether is not "yet another encrypted messenger."
- Hero headline should convey: no servers, you ARE the infrastructure, zero cost.
- Sub-headline should explain: P2P encrypted voice + text, connect with a secret code, no accounts.
- Single primary CTA: "Download" (detected OS). Secondary CTA: "Try the Demo" (links to sandbox).
- Do NOT bury the value prop behind animation or scrolling. First viewport = understanding.

**Expected Sections Below the Fold:**
1. **How It Works** -- 3-step visual (Generate Identity -> Share Secret Code -> Connect P2P). Follow Briar's pattern of visual diagrams comparing centralized vs decentralized.
2. **Feature Highlights** -- Cards for: Voice Chat, Text Chat, Channels, Peer Moderation, Petnames. Each with icon + 1-2 sentence description.
3. **Philosophy / Why P2P** -- Dedicated section explaining sovereign communication. Attack the "Client-Server flaw" described in PROJECT.md. Use concrete language: "No server to shut down. No company to sell your data. No cloud bill."
4. **Social Proof / Trust Signals** -- Open source badge, license info, link to GitHub repo. For a pre-launch project without enterprise adopters, transparency IS the trust signal.
5. **Download CTA (repeated)** -- Repeat the download section at page bottom. Standard pattern across all competitors.

**Anti-Pattern:** Do NOT use enterprise-speak ("unified communications platform") or vague privacy language ("we take your privacy seriously"). Be specific and technical. The target audience for a P2P app is technically literate.

---

### 2. Platform-Aware Download Section

| Aspect | Detail |
|--------|--------|
| **Why Expected** | Every desktop app website (Signal, Element, Briar Desktop, Jami) auto-detects the visitor's OS and surfaces the right download. Friction at download = lost users. |
| **Complexity** | Low |
| **Depends On** | Tauri build artifacts (macOS .dmg, Windows .msi/.exe, Linux .AppImage/.deb), hosting for binaries |

**Expected Behavior:**
- Auto-detect visitor OS via `navigator.userAgent` or `navigator.platform`
- Show the primary download button for detected OS (e.g., "Download for macOS" with Apple icon)
- Below primary button: "Also available for Windows, Linux" with links to other platforms
- Each platform shows: file size, version number, system requirements
- Link to GitHub Releases as canonical source for all binaries

**Platform Detection Implementation:**
```typescript
function detectPlatform(): 'macos' | 'windows' | 'linux' | 'unknown' {
  const ua = navigator.userAgent.toLowerCase();
  if (ua.includes('mac')) return 'macos';
  if (ua.includes('win')) return 'windows';
  if (ua.includes('linux')) return 'linux';
  return 'unknown';
}
```

**What Competitors Do:**
- **Signal:** Footer with all platform badges. Simple.
- **Jami:** Dedicated download page with per-platform sections.
- **Element:** "Download" button in header + dedicated download page with platform tabs.
- **Session:** Multiple platform buttons in a grid layout.

**Recommendation for Aether:**
- Dedicated `/download` page with auto-detected primary CTA
- Show all three platforms with icons (macOS, Windows, Linux)
- Include SHA-256 checksums for each binary -- privacy-conscious users expect this
- Link to build-from-source instructions on GitHub for maximum trust
- Include minimum OS version requirements (Tauri v2 requirements)

**Data Needed from Build Pipeline:**
- Download URLs (GitHub Releases or self-hosted CDN)
- File sizes per platform
- Current version number
- SHA-256 checksums

---

### 3. User Documentation (Getting Started Guides)

| Aspect | Detail |
|--------|--------|
| **Why Expected** | P2P apps have unusual setup flows (no account creation, secret codes, swarm concepts). Users WILL be confused without guides. Signal gets away without docs because "install and text" is understood. Aether cannot. |
| **Complexity** | Medium |
| **Depends On** | Screenshot assets from the desktop app, content writing |

**Required Documentation Pages:**

| Page | Content | Priority |
|------|---------|----------|
| **Getting Started** | Install, first launch, identity generation, UI overview | Critical |
| **Creating a Swarm** | What a swarm is, how to create one, the Secret Code concept | Critical |
| **Inviting Peers** | Sharing the `aether://` URI, how peers join | Critical |
| **Voice Chat** | Joining voice, mute/unmute, latency expectations, 8-peer limit | High |
| **Channels** | Creating channels, renaming, deleting, how sync works | High |
| **Petnames & Contacts** | Setting petnames, why names are self-asserted, Zooko's Triangle simplified | Medium |
| **Peer Moderation** | Hide vs block, what "subjective moderation" means in practice | Medium |
| **Notifications** | Enabling, mention detection, when notifications fire | Medium |
| **Troubleshooting** | NAT traversal issues, connectivity problems, keychain permissions | High |
| **FAQ** | Common questions about P2P model, data persistence, privacy guarantees | High |

**Format Expectations:**
- Searchable (full-text search across all docs)
- Screenshots with annotations
- Step-by-step numbered instructions
- Sidebar navigation with section hierarchy
- Previous/Next page navigation
- Mobile-responsive (users may read docs on phone while setting up desktop app)

**What Competitors Do:**
- **Briar:** Dedicated manual at briarproject.org/manual/ with quick start guide
- **Signal:** support.signal.org with categorized articles
- **Element:** docs.element.io with comprehensive user guide
- **Jami:** Wiki-based documentation

**Anti-Pattern:** Do NOT use auto-generated API docs as user documentation. User docs need human-written narratives, not function signatures.

---

### 4. Technical Documentation (Architecture & Protocol)

| Aspect | Detail |
|--------|--------|
| **Why Expected** | Open-source P2P projects live or die by developer trust. Technical docs prove the architecture is sound. Contributors need them. Security auditors need them. |
| **Complexity** | Medium-High |
| **Depends On** | Deep knowledge of existing codebase architecture, protocol decisions |

**Required Technical Documentation:**

| Page | Content | Audience |
|------|---------|----------|
| **Architecture Overview** | System diagram, component boundaries, data flow | Developers, contributors |
| **P2P Networking** | libp2p usage, DHT discovery, PSK swarm isolation, NAT traversal (UDP holepunching), relay fallback | Protocol reviewers |
| **Identity System** | Ed25519 keypairs, hardware-backed keychain, self-asserted names, Zooko's Triangle approach | Security reviewers |
| **CRDT Chat Sync** | Automerge document model, sync protocol, eventual consistency, conflict resolution | Contributors |
| **Voice Protocol** | Opus codec, mesh topology, 8-peer limit rationale, latency targets, cpal audio pipeline | Contributors |
| **Encryption Model** | Transport-only encryption (honest about current limitations), PSK pre-shared keys, libp2p Noise | Security reviewers |
| **Moderation Model** | Subjective moderation philosophy, block/hide mechanics, why no admin/ban | Philosophy-minded users |
| **Data Model** | Local storage structure, Automerge docs, Tauri Store files, what data lives where | Contributors |

**Format Expectations:**
- Architecture diagrams (Mermaid or SVG)
- Code snippets showing key structures (Rust structs, message formats)
- Explicit version pinning (libp2p 0.56, Automerge 0.7, etc.)
- Honest about limitations and trade-offs (transport-only encryption, no offline delivery, no key export)
- Linked from GitHub README for discoverability

**What Competitors Do Well:**
- **Briar:** "How it Works" page with comparison diagrams (centralized vs P2P)
- **Matrix:** spec.matrix.org with full protocol specification
- **Session:** Whitepaper and technical documentation linked from footer
- **Element:** Comprehensive docs.element.io for self-hosting

**What Competitors Do Poorly:**
- Most P2P projects have sparse or outdated technical docs
- Architecture diagrams are rare (usually just text descriptions)
- Aether can differentiate by having clear, visual, honest technical documentation

---

### 5. Clear Navigation Structure

| Aspect | Detail |
|--------|--------|
| **Why Expected** | Visitors arrive with different intents: download the app, understand the project, read docs, try the demo. Navigation must route them immediately. |
| **Complexity** | Low |
| **Depends On** | Information architecture decisions, page inventory |

**Recommended Navigation:**

```
Home | Docs | Download | Demo | GitHub
```

5 items. Simple. Every competitor uses 4-7 top-level nav items.

**Comparison of Competitor Nav:**
| App | Nav Items |
|-----|-----------|
| Signal | Get Signal, Help, Blog, Developers, Careers, Donate |
| Element | Product, Solutions, Resources, Blog, Pricing, Download |
| Jami | Download, Extensions, Contribute, Services, Blog, Wiki |
| Briar | News, How it Works, About, Get Involved, Download |
| Session | Blog, FAQ, Support, GitHub, Download |
| Tox | Download, About, FAQ, Clients, Wiki, Blog |

**Aether's nav should be sparse.** No "Solutions" or "Pricing" -- this is an open-source tool, not a SaaS product. The nav signals what kind of project this is.

**Secondary Navigation (Footer):**
- GitHub repository link
- License information (state explicitly)
- Privacy policy (even if minimal -- "we collect nothing" IS a policy)
- Community links (if applicable)
- Version/changelog

---

### 6. Responsive Design (Mobile-First)

| Aspect | Detail |
|--------|--------|
| **Why Expected** | ~83% of landing page traffic is mobile. Even for a desktop-only app, the WEBSITE must work on mobile. Users discover on phone, download on desktop later. |
| **Complexity** | Low-Medium |
| **Depends On** | CSS framework choice, layout testing |

**Expected Behavior:**
- All pages readable and navigable on 375px width (iPhone SE)
- Navigation collapses to hamburger menu on mobile
- Download page works on mobile (even if the app is desktop-only -- show "Available for macOS, Windows, and Linux" with a prompt to visit on desktop)
- Documentation sidebar collapses to expandable menu on mobile
- Interactive demo gracefully degrades or shows "Visit on desktop for the full experience"
- Images/screenshots responsive with proper aspect ratios
- Touch-friendly tap targets (minimum 44px)

**Anti-Pattern:** Do NOT build a desktop-only website for a desktop-only app. The WEBSITE is not the APP.

---

### 7. Fast Page Load Performance

| Aspect | Detail |
|--------|--------|
| **Why Expected** | Users expect pages to load in under 2 seconds. Privacy-conscious users may be on Tor or VPNs with higher latency. Google Core Web Vitals affect discoverability. |
| **Complexity** | Low (with SvelteKit SSG) |
| **Depends On** | Static site generation, image optimization, hosting |

**Expected Behavior:**
- Largest Contentful Paint (LCP) under 2.5 seconds
- First Input Delay (FID) under 100ms
- Cumulative Layout Shift (CLS) under 0.1
- Total page weight under 500KB for landing page (excluding demo)
- No render-blocking JavaScript for content pages
- Images lazy-loaded below the fold
- Fonts subset or system fonts to avoid FOIT/FOUT

**SvelteKit Advantage:**
- Static adapter (`@sveltejs/adapter-static`) pre-renders all pages at build time
- Zero client-side JavaScript for pure content pages (SvelteKit only hydrates interactive components)
- Markdown pages via mdsvex compile to static HTML
- Vite's build pipeline handles tree-shaking, code-splitting, and minification

**Anti-Pattern:** Do NOT load a heavy JavaScript framework just for a marketing page. SvelteKit's static output is ideal here.

---

### 8. Open Source Transparency Signals

| Aspect | Detail |
|--------|--------|
| **Why Expected** | P2P/privacy app users are inherently skeptical. They will look for: source code access, license clarity, reproducible builds, no tracking. |
| **Complexity** | Low |
| **Depends On** | GitHub repository being public, license file |

**Expected Signals:**
- GitHub link prominently in header/footer (all competitors do this)
- License badge on landing page (GPLv3, MIT, Apache -- whatever Aether uses)
- "View Source" or "Star on GitHub" CTA
- No analytics tracking on the website (or use privacy-respecting analytics like Plausible -- but "no analytics" is the gold standard for this audience)
- No cookies banner (because there should be no cookies)
- No third-party scripts (no Google Fonts CDN, no external tracking pixels)
- Self-host all assets (fonts, icons, images)

**What Competitors Do:**
- **Signal:** Clean, no trackers, links to GitHub
- **Briar:** Links to source, build instructions, security audit results
- **Tox:** "Free software" messaging prominently displayed
- **Session:** GitHub and whitepaper linked from footer

**Anti-Pattern:** Do NOT use Google Analytics, Google Fonts CDN, Cloudflare analytics, or any third-party service that phones home. This would undermine the entire privacy message. Self-host everything.

---

## Differentiators

Features that set Aether's marketing site apart from every other P2P communication tool's website. Not expected, but create significant value.

---

### 9. Interactive Browser-Based P2P Demo (Live Sandbox)

| Aspect | Detail |
|--------|--------|
| **Value Proposition** | No other P2P communication tool offers a "try before you download" experience in the browser. This is the single biggest differentiator for the marketing site. |
| **Complexity** | High |
| **Depends On** | WebRTC browser APIs, signaling mechanism, UI subset implementation |

**Why This Matters:**
- P2P apps have a cold-start problem: you need at least 2 peers to demonstrate value. A browser demo solves this.
- Every competitor requires download-first. Signal, Briar, Session, Jami -- ALL require installation before the user sees anything.
- An interactive demo reduces the "leap of faith" for new users.
- Product-led growth research shows interactive demos increase conversion by 25-40% over static screenshots.

**What the Demo Should Be:**
A simplified, browser-based P2P chat between two tabs/windows using WebRTC DataChannels. The visitor opens the demo page, gets a shareable link (or QR code), and sends it to a friend. They connect directly in the browser and can exchange text messages.

**Scope for v2.0 (keep it focused):**
- Text chat only (no voice in browser -- voice over WebRTC adds significant complexity)
- Two peers only (no multi-peer mesh in browser)
- Ephemeral sessions (no persistence, no identity -- just raw P2P messaging)
- Connection via shareable link containing an encoded SDP offer (or use a lightweight signaling relay)
- Clear labeling: "This is a simplified demo. Download Aether for the full experience with voice chat, channels, identity, and more."

**Signaling Approach (the hard part):**
WebRTC requires an initial signaling exchange. Options:

| Approach | Complexity | UX | Privacy |
|----------|-----------|-----|---------|
| **Manual SDP copy-paste** | Low | Poor (users copy giant text blocks) | Perfect (zero server) |
| **Lightweight signaling relay** | Medium | Good (shareable link) | Acceptable (relay sees only encrypted SDP, not messages) |
| **PeerJS** | Low | Good (built-in signaling) | Poor (uses PeerJS cloud server) |

**Recommendation:** Use a lightweight signaling relay. This is acceptable because: (1) the relay only facilitates the initial handshake, not ongoing communication, (2) all message data flows P2P via DataChannels after connection, (3) honest labeling ("signaling relay used for connection setup only") maintains trust. Self-host the relay to avoid third-party dependencies.

**Technical Stack for Demo:**
- WebRTC `RTCPeerConnection` and `RTCDataChannel` (native browser APIs)
- Svelte component embedded in the SvelteKit site
- Minimal signaling relay (could be a simple WebSocket server, or even a serverless function)
- No additional libraries required for basic DataChannel chat

**UX Flow:**
1. Visitor clicks "Try the Demo" on landing page
2. Demo page loads with "Create a session" button
3. Clicking creates a WebRTC offer and generates a shareable link
4. Visitor shares link with a friend (or opens in another tab to self-demo)
5. Friend opens link, WebRTC connection establishes
6. Both can send text messages directly P2P
7. Connection status shown visually (connecting / connected / disconnected)
8. Prominent "Download Aether for the full experience" CTA throughout

**Anti-Pattern:** Do NOT try to replicate the full desktop app in the browser. The demo should be a teaser, not a product. Keep it minimal, fast, and impressive in its simplicity.

---

### 10. Visual Architecture Diagrams

| Aspect | Detail |
|--------|--------|
| **Value Proposition** | Most P2P projects explain their architecture with walls of text. Clear, visual diagrams instantly communicate the difference between centralized and decentralized models. Briar does this well; Aether can do it better. |
| **Complexity** | Medium |
| **Depends On** | Diagram design, possibly animated SVGs |

**Diagrams to Include:**

1. **Centralized vs P2P comparison** -- Side-by-side: "Discord/Slack model" (all arrows go to central server) vs "Aether model" (mesh of direct connections). Briar does this; it is extremely effective.
2. **Connection flow** -- Step-by-step: Identity generation -> Secret Code creation -> DHT discovery -> NAT traversal -> Direct P2P connection. Animated or interactive.
3. **Swarm topology** -- Visual showing multiple peers in a swarm, each with channels, messages flowing between them via CRDT sync.
4. **Encryption layers** -- Where encryption happens (transport layer via libp2p Noise), what is protected, what is not (honest about cleartext at rest).

**Implementation:**
- SVG diagrams (scalable, theme-aware, accessible)
- Consider Svelte-powered animation on scroll (nodes appearing, connections forming)
- Keep diagrams simple -- max 5-7 elements per diagram
- Use the same visual language across all diagrams (consistent node shapes, colors, arrow styles)

**Why This Differentiates:**
- Signal's site has zero architecture diagrams
- Session has a whitepaper but no visual architecture on the site
- Jami has no architecture visuals
- Tox has no architecture visuals
- Only Briar has meaningful "how it works" diagrams, and they are static images

---

### 11. Honest Limitations Section

| Aspect | Detail |
|--------|--------|
| **Value Proposition** | Privacy-conscious users respect honesty. Documenting what Aether does NOT do (and why) builds more trust than marketing spin. No competitor does this well. |
| **Complexity** | Low |
| **Depends On** | Content writing, willingness to be transparent |

**What to Document:**
- "Aether requires both peers to be online simultaneously" (Pure Sync trade-off)
- "Messages are encrypted in transit but stored as cleartext on your device" (Transport-only encryption)
- "Your identity is bound to your device's hardware keychain and cannot be exported to another device" (Hardware-backed identity trade-off)
- "Voice chat is limited to 8 participants" (Mesh topology constraint)
- "There are no push notifications -- you receive messages only when the app is running" (No relay infrastructure)
- "There is no global user directory -- you find peers via Secret Codes shared out-of-band" (Privacy by design)

**Why This Differentiates:**
- Most messaging apps hide limitations in fine print
- Being upfront about trade-offs attracts users who WANT these specific trade-offs (sovereign, no servers, no cloud)
- Prevents disappointed users who expect Signal-like always-online delivery
- Positions Aether as a thoughtful engineering project, not a marketing exercise

**Placement:** Dedicated section on landing page ("Trade-offs We Chose") or a standalone "/philosophy" page. NOT buried in FAQ.

---

### 12. Keyboard Shortcut / Command Reference Card

| Aspect | Detail |
|--------|--------|
| **Value Proposition** | Power users (Aether's target audience) expect keyboard shortcuts. A printable/bookmarkable reference card in docs adds polish. |
| **Complexity** | Low |
| **Depends On** | Existing keyboard shortcuts in the desktop app |

**Format:** Single-page reference card in docs with all keyboard shortcuts, grouped by context (chat, voice, navigation). Printable CSS. Could also be a downloadable PDF.

---

## Anti-Features

Features to explicitly NOT build on the marketing site.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| **User accounts / login** | The marketing site is informational. There is nothing to "log in" to. Adding accounts creates a server dependency that contradicts the P2P message. | No accounts. The demo is anonymous/ephemeral. |
| **Blog / news section** | Blogs require ongoing content commitment. An abandoned blog (last post 6 months ago) signals a dead project. For a small team, this is a maintenance trap. | Link to GitHub releases/changelog for updates. If a blog is desired later, add it as a future milestone. |
| **Community forum on-site** | Forums require moderation infrastructure, accounts, and ongoing maintenance. Contradicts "community lives in the app." | Point users to the app itself for community. Link to GitHub Discussions for developer questions. |
| **Analytics / tracking** | ANY tracking undermines the privacy message. Even "privacy-friendly" analytics like Plausible are still tracking. The target audience will check. | Zero analytics. If usage data is needed, check download counts on GitHub Releases. Server access logs (self-hosted) are sufficient for basic traffic understanding. |
| **Third-party CDN dependencies** | Loading fonts from Google, scripts from CDNs, or images from external hosts leaks visitor metadata to third parties. | Self-host all assets: fonts, icons, JavaScript, CSS, images. |
| **Newsletter signup** | Requires an email service, stores PII, adds GDPR complexity. The target audience is hostile to email collection. | GitHub "Watch" or RSS feed for releases if update notifications are desired. |
| **Pricing page** | Aether is free. A pricing page (even saying "$0") frames it as a product competing on price rather than philosophy. | State "Free and open source" on the landing page. No dedicated pricing page. |
| **Comparison tables with competitors** | Direct comparison ("Aether vs Signal vs Discord") invites feature-by-feature battles Aether will lose on volume. It also dates quickly. | Focus on Aether's unique philosophy. Let users draw their own comparisons. The "Trade-offs" section implicitly positions against competitors. |
| **Auto-playing video** | Intrusive, bandwidth-heavy, accessibility issues. Privacy-conscious users often block autoplay. | Static screenshots or animated SVG diagrams. Optional video with play button if needed. |
| **Cookie consent banner** | If you need a cookie banner, you are doing something wrong for a privacy tool's website. | Use zero cookies. No session tracking. Static site with no server-side state. |
| **Heavy JavaScript animations / parallax** | Adds page weight, causes CLS issues, frustrates users on slow connections or with motion sensitivity. | Subtle CSS transitions. `prefers-reduced-motion` media query respected. |

---

## Feature Dependencies

```
Landing Page (Hero + Value Prop)
    '-- self-contained (copy + design)

Download Section
    |-- depends on: Tauri build artifacts existing (CI/CD pipeline for releases)
    |-- depends on: Hosting for binaries (GitHub Releases or CDN)
    '-- depends on: Platform detection JS (trivial)

User Documentation
    |-- depends on: mdsvex setup in SvelteKit
    |-- depends on: Screenshot assets from desktop app
    |-- depends on: Content writing (significant effort)
    '-- depends on: Search implementation (full-text search library)

Technical Documentation
    |-- depends on: mdsvex setup in SvelteKit (shared with User Docs)
    |-- depends on: Architecture diagram assets (SVG/Mermaid)
    |-- depends on: Deep codebase knowledge for accurate content
    '-- depends on: Content writing (significant effort)

Interactive P2P Demo
    |-- depends on: WebRTC implementation (RTCPeerConnection + DataChannel)
    |-- depends on: Signaling relay (WebSocket server or serverless function)
    |-- depends on: Demo UI (Svelte component)
    |-- depends on: Hosting for signaling relay (separate from static site)
    '-- does NOT depend on: desktop app codebase (browser-native implementation)

Visual Architecture Diagrams
    |-- depends on: Diagram design/illustration work
    '-- self-contained (SVG assets embedded in pages)

Honest Limitations Section
    '-- self-contained (copy only)
```

**Critical Path:** The interactive P2P demo is the highest-complexity feature and the only one requiring a server component (signaling relay). Everything else is static content.

**Independent Features (can be built in parallel):**
- Landing page content + design
- Download section
- User documentation (content writing)
- Technical documentation (content writing)
- Architecture diagrams
- Limitations section

**Sequential Dependency:**
1. SvelteKit project scaffold must come first (shared infrastructure for all pages)
2. mdsvex configuration must precede documentation writing (defines the authoring format)
3. Demo signaling relay must be built/deployed before demo UI can be fully tested

---

## Competitor Analysis Summary

| Feature | Signal | Element | Session | Briar | Jami | Tox | Aether (Planned) |
|---------|--------|---------|---------|-------|------|-----|-------------------|
| Clear hero / value prop | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| Platform-aware download | Footer only | Yes | Yes | Yes | Yes | Yes | Yes (auto-detect) |
| User documentation | support.signal.org | docs.element.io | FAQ + support | Manual | Wiki | Wiki | Yes (in-site) |
| Technical / protocol docs | GitHub only | Matrix spec | Whitepaper | Wiki | Limited | Limited | Yes (in-site) |
| Architecture diagrams | No | Some | No | Yes (good) | No | No | Yes (animated) |
| Browser demo / sandbox | No | Yes (Element Web) | No | No | No | No | Yes (WebRTC) |
| Honest limitations | No | No | No | Partially | No | No | Yes (differentiator) |
| No tracking | Yes | No (uses analytics) | Partial | Yes | Partial | Yes | Yes (zero tracking) |
| Self-hosted assets | Mostly | No | Mostly | Yes | No | Mostly | Yes (all) |
| Open source visibility | GitHub link | GitHub link | GitHub link | GitHub link | GitHub link | GitHub link | Prominent CTA |

**Key Insight:** No P2P messaging tool offers a browser-based interactive demo. Element Web exists but it is a full client requiring a Matrix account and server, not a quick "try it" sandbox. Aether's demo would be unique in the space.

**Second Insight:** Technical documentation is universally weak across P2P communication tools. The Matrix spec is the gold standard but it serves a protocol, not a product. Individual tools rarely document their architecture in an accessible way. Aether's visual, honest technical docs can stand out.

---

## MVP Recommendation for v2.0 Marketing Site

**Priority Tier 1 -- Build First (foundation):**
1. **SvelteKit project scaffold** -- Monorepo `/site` directory, routing, layout, mdsvex config. Everything else depends on this.
2. **Landing page** -- Hero, feature highlights, philosophy section, download CTA. This IS the marketing site in minimal viable form.
3. **Download section** -- Platform detection, links to GitHub Releases. Converts visitors to users.

**Priority Tier 2 -- Build Second (content):**
4. **User documentation** -- Getting started guide, swarm creation, voice chat basics. Most important docs pages first.
5. **Technical documentation** -- Architecture overview, networking, identity. Builds developer trust and contributor pipeline.
6. **Visual architecture diagrams** -- Centralized vs P2P comparison, connection flow. High visual impact.

**Priority Tier 3 -- Build Third (differentiator):**
7. **Interactive P2P demo** -- WebRTC text chat sandbox. Highest complexity, highest impact. Build after the static site is solid.
8. **Honest limitations section** -- Low effort, high trust impact. Can be added at any point.
9. **Keyboard shortcut reference** -- Polish item for docs.

**Defer to Later:**
- Blog (only if content cadence can be maintained)
- Video content (requires production effort beyond current scope)
- Internationalization / translations (add when user base demands it)
- Native app store badges (desktop-only, not applicable)

---

## Content Effort Estimate

| Content Area | Pages | Estimated Writing Effort | Notes |
|--------------|-------|--------------------------|-------|
| Landing page | 1 | Low (copy + layout) | Requires design decisions |
| Download page | 1 | Low (structured data) | Requires CI/CD for artifacts |
| User docs | 8-10 | High (narrative + screenshots) | Largest content investment |
| Technical docs | 6-8 | High (accuracy + diagrams) | Requires deep codebase knowledge |
| FAQ | 1 | Medium | Can grow incrementally |
| Limitations / philosophy | 1 | Low (already articulated in PROJECT.md) | Copy from existing decisions |

**Total estimated pages:** 18-22 content pages plus the interactive demo component.

---

## Sources

- [Signal website](https://signal.org) -- Hero structure, minimal navigation, privacy-first design. HIGH confidence.
- [Element website](https://element.io) -- Enterprise-grade marketing for open-source comms, Element Web as browser client. HIGH confidence.
- [Session website](https://getsession.org) -- "Send Messages, Not Metadata" hero, metadata-focused messaging. HIGH confidence.
- [Briar Project](https://briarproject.org) -- Best-in-class "How It Works" diagrams for P2P architecture. HIGH confidence.
- [Briar How It Works](https://briarproject.org/how-it-works/) -- Centralized vs P2P visual comparison approach. HIGH confidence.
- [Jami website](https://jami.net) -- Feature grid, advantage cards, download page structure. HIGH confidence.
- [Tox website](https://tox.chat) -- Minimal P2P messaging site, "What makes Tox different" section. HIGH confidence.
- [PeerJS](https://peerjs.com/) -- Simplified WebRTC P2P library for browser demos. MEDIUM confidence.
- [Serverless WebRTC](https://github.com/cjb/serverless-webrtc) -- Manual SDP exchange demo, no signaling server. HIGH confidence.
- [p2p-chat](https://github.com/michal-wrzosek/p2p-chat) -- Serverless P2P chat on WebRTC reference. MEDIUM confidence.
- [Navattic Interactive Demo Best Practices](https://www.navattic.com/blog/interactive-demos) -- 40,000+ demos built, 25-40% conversion lift data. MEDIUM confidence.
- [MDsveX docs](https://svelte.dev/docs/cli/mdsvex) -- Official Svelte CLI documentation for markdown preprocessor. HIGH confidence.
- [SvelteKit static adapter](https://svelte.dev/docs/kit/adapter-static) -- Static site generation for marketing/docs sites. HIGH confidence.
- [Landing page statistics 2026](https://www.involve.me/blog/landing-page-statistics) -- 83% mobile traffic, conversion benchmarks. MEDIUM confidence.
- [Landing page best practices 2026](https://www.moburst.com/blog/landing-page-design-trends-2026/) -- Design trends, bold typography, reduced friction. MEDIUM confidence.

# Pitfalls Research

**Domain:** Marketing Site with Interactive P2P Demo for Desktop App
**Researched:** 2026-03-04
**Confidence:** HIGH

---

## Critical Pitfalls

### Pitfall 1: Monorepo Build Pipeline Collision Between Tauri App and SvelteKit Site

**What goes wrong:**
The existing Aether app uses plain Svelte 5 + Vite (NOT SvelteKit). Adding a SvelteKit marketing site in `/site` creates two competing build pipelines in the same repo. The Tauri `beforeBuildCommand` in `tauri.conf.json` currently runs `npm run build` which invokes Vite directly. If the new SvelteKit site shares `node_modules` at the root, dependency resolution collides: SvelteKit pulls in `@sveltejs/kit`, its adapter, and its own Vite plugin configuration, while the Tauri app uses `@sveltejs/vite-plugin-svelte` directly. Version conflicts between Svelte preprocessors, Vite plugins, or even Svelte itself cause cryptic build failures.

**Why it happens:**
Developers naturally want to share code and keep everything at the root `package.json`. The existing `svelte.config.js` at root is for the Tauri app's Svelte 5 SPA -- but SvelteKit expects its own `svelte.config.js` with adapter configuration. If both coexist at root or get confused by `node_modules` hoisting, builds break. Additionally, Tauri's `frontendDist` points to `../dist` while SvelteKit outputs to `build/` by default.

**How to avoid:**
- Use a proper workspace monorepo structure with pnpm workspaces (or npm workspaces). The Tauri app stays at root, the marketing site lives in `/site` as a separate workspace with its own `package.json`, `svelte.config.js`, and `vite.config.ts`.
- Never hoist SvelteKit's dependencies to root. The `/site` workspace must be self-contained.
- The Tauri `tauri.conf.json` `beforeBuildCommand` must NOT be modified -- it stays `npm run build` for the Vite SPA. The marketing site has its own build script (`npm run build` inside `/site`).
- Use separate CI jobs: one for Tauri app, one for marketing site deployment. They should never share a build step.

**Warning signs:**
- `Cannot find module '@sveltejs/kit'` errors when building the Tauri app.
- `svelte.config.js` at root suddenly requiring an adapter.
- Build output directory confusion (Tauri looking at `/build` instead of `/dist`).
- `npm run dev` starting the wrong app.

**Phase to address:**
Phase 1 (Project Setup / Monorepo Structure) -- this must be the very first thing established before any feature work.

---

### Pitfall 2: WebRTC Demo Requires a Signaling Server (There Is No "Serverless" Browser P2P)

**What goes wrong:**
The team tries to build a "serverless" browser P2P demo to match Aether's "zero infrastructure" philosophy. They discover that WebRTC absolutely requires a signaling mechanism to exchange SDP offers/answers between peers before a connection can be established. Unlike the desktop app (which uses libp2p's DHT for peer discovery), browsers have no equivalent. The demo either (a) never works because there is no way for two browser tabs to find each other, or (b) uses a janky copy-paste SDP flow that no visitor will actually complete.

**Why it happens:**
Aether's core philosophy is "no servers." The desktop app uses libp2p DHT + mDNS for discovery, which avoids central infrastructure. But browsers cannot access raw UDP/TCP sockets. WebRTC is one of the only P2P transports available in browsers, and it mandates a signaling exchange before connection. There is no browser-native peer discovery protocol. Developers conflate "P2P data transfer" (which WebRTC does) with "P2P discovery" (which still needs a coordinator).

**How to avoid:**
- Accept that the browser demo needs a lightweight signaling service. This does NOT violate Aether's philosophy -- the signaling server only exchanges connection metadata, never user data. Frame it as "introduction" not "infrastructure."
- Use a minimal WebSocket signaling server (a few dozen lines of code). Deploy on Cloudflare Workers or Deno Deploy for near-zero cost and zero ops.
- Alternatively, use a shared room-code approach: visitor A creates a room, gets a code, shares it with visitor B who joins. The signaling server matches them by room code, exchanges SDP, then the server is no longer involved. This mirrors Aether's "Secret Code" UX.
- For the absolute simplest approach, the demo can run two peers in the SAME browser tab using `RTCPeerConnection` with local SDP exchange (no server needed). This demonstrates the protocol without requiring a second human. This is the recommended starting point.
- js-libp2p supports WebRTC transport with Circuit Relay for browser-to-browser connections, but this requires a relay node (still a server). Only use this if you want the demo to show actual libp2p interop.

**Warning signs:**
- Spending more than a day trying to make "serverless browser P2P" work.
- Copy-paste SDP UX where users must manually exchange JSON blobs.
- The demo only works when both users are on the same LAN.

**Phase to address:**
Phase 2 (Interactive Demo / Sandbox) -- must be designed with signaling architecture from day one.

---

### Pitfall 3: HTTPS Required for All WebRTC APIs in Production

**What goes wrong:**
The WebRTC demo works perfectly on `localhost` during development. When deployed to production at `http://aether.example.com`, `getUserMedia()` (for voice demo), `RTCPeerConnection`, and other WebRTC APIs silently fail or throw security errors. The entire demo is broken on launch day.

**Why it happens:**
Browsers enforce Secure Context requirements for WebRTC APIs. `getUserMedia()` only works on HTTPS origins or `localhost`. `RTCPeerConnection` also requires a secure context in modern browsers. Developers test on `localhost` (which is always treated as secure) and never encounter this until production deployment. Even if the demo only uses DataChannels (no media), some browsers still restrict `RTCPeerConnection` on insecure origins.

**How to avoid:**
- Deploy the marketing site to HTTPS from the very first deployment. Use Vercel, Netlify, or Cloudflare Pages -- all provide HTTPS by default with zero configuration.
- During local development, test with `localhost` (secure by default) but also test with a LAN IP to surface HTTPS issues early.
- If using a custom domain, set up TLS certificates before building the demo, not after.
- If the demo includes voice/audio features, the `allow` attribute must be set on any iframes: `allow="microphone; camera"`.

**Warning signs:**
- Demo works on localhost but not on staging/production.
- Console errors about "insecure context" or "NotAllowedError" in production.
- `getUserMedia` returning undefined (not even throwing an error in some browsers).

**Phase to address:**
Phase 3 (Deployment / Infrastructure) -- HTTPS must be in place before the demo is deployed.

---

### Pitfall 4: STUN/TURN Misconfiguration Breaks Demo for 20%+ of Users

**What goes wrong:**
The WebRTC demo uses only free Google STUN servers (`stun:stun.l.google.com:19302`). It works for developers on residential connections but fails silently for visitors behind corporate firewalls, university networks, or symmetric NAT. Approximately 10-20% of real-world connections require TURN relay. The demo appears "broken" for a significant percentage of visitors, and there is no error message -- just silence.

**Why it happens:**
STUN is free and handles ~80% of NAT scenarios. TURN relay costs money because it relays actual media traffic. Developers skip TURN setup to avoid cost/complexity. The 80% success rate feels like "it works" during testing because developers rarely test from restrictive networks.

**How to avoid:**
- Use Cloudflare's free TURN service (1,000 GB/month free tier, STUN is unlimited). This is the clear winner for a demo with limited traffic.
- Configure ICE servers with both STUN and TURN endpoints. TURN is a fallback, not a replacement for STUN.
- Add connection state UI to the demo: show ICE connection state (checking, connected, failed) so visitors know what is happening. If TURN fails, show a clear "connection could not be established" message rather than silent failure.
- For a demo-only context (low traffic), Xirsys free tier (500 GB) or Metered free tier (500 MB) also work.
- TURN credentials should be short-lived and fetched from an API endpoint, not hardcoded in client JavaScript.

**Warning signs:**
- Demo works on your home WiFi but fails at a coffee shop or office.
- No TURN server configured at all, or TURN configured but with expired/hardcoded credentials.
- ICE connection state stuck on "checking" indefinitely with no user feedback.

**Phase to address:**
Phase 2 (Interactive Demo) for configuration, Phase 3 (Deployment) for TURN server provisioning.

---

### Pitfall 5: Code Signing and Notarization Not Set Up Before Download Page Goes Live

**What goes wrong:**
The marketing site goes live with a download section linking to Tauri-built `.dmg` and `.exe` files. macOS users see "Aether cannot be opened because it is from an unidentified developer" (Gatekeeper block). Windows users see a SmartScreen warning: "Windows protected your PC -- this app is from an unknown publisher." Visitors immediately distrust the app. Some visitors report it as malware.

**Why it happens:**
Code signing is expensive (Apple Developer Program: $99/year, Windows EV certificates: $200-400/year), requires setup time (Apple notarization can take hours on first submission), and involves complex CI/CD configuration. Developers defer it as "we'll do it later" and focus on the website. But the moment you offer public downloads, unsigned binaries destroy credibility.

**How to avoid:**
- Set up code signing BEFORE building the download page. This is not optional for a public release.
- macOS: Requires Apple Developer ID Application certificate (paid $99/year account). Notarization must also be configured -- it is a separate step from code signing. Only the Account Holder role can create Developer ID certificates.
- Windows: EV code signing certificates now require hardware security modules (HSMs). Azure Key Vault is the recommended approach for Tauri v2. Self-signed certificates will still trigger SmartScreen.
- Tauri v2 has built-in support for both macOS and Windows signing in CI/CD. Use the official Tauri GitHub Actions.
- If budget is a constraint, launch as macOS-only first (cheaper, simpler signing) and add Windows later.
- Include entitlements file for macOS WebView (`allow-jit`, `allow-unsigned-executable-memory`) or the signed app will crash on launch.

**Warning signs:**
- Download links pointing to unsigned binaries.
- "We'll add code signing later" in the project plan.
- CI/CD pipeline that builds but does not sign.
- Gatekeeper or SmartScreen warnings during QA testing.

**Phase to address:**
Phase 4 (Download/Distribution) -- but certificate procurement should start in Phase 1 because Apple notarization setup takes days, and Windows EV certs take weeks to arrive.

---

### Pitfall 6: Documentation Becomes Stale Within Weeks of Launch

**What goes wrong:**
The team writes beautiful documentation for the marketing site: setup guides, architecture deep-dives, API references. Within two months, the desktop app ships new features (from v1.1 community features), changes CRDT schemas, or updates libp2p versions. The documentation still describes v1.0 behavior. New users follow outdated setup guides and hit errors. Stale documentation is worse than no documentation because users trust it.

**Why it happens:**
Documentation and code live in separate workflows. The app is developed with CI/CD, tests, and PRs. Documentation is treated as a "write once" artifact. There is no process to flag when code changes invalidate documentation. The marketing site deployment pipeline is separate from the app build pipeline, so documentation can go months without updates.

**How to avoid:**
- Co-locate documentation source files near the code they describe. Architecture docs live in the repo, not in a separate CMS.
- Use version-pinned code examples. Every code snippet in documentation should reference a specific git tag or version. When the tagged version changes, a CI check should flag stale examples.
- Implement a documentation freshness policy: every doc page has a `last_verified` date in frontmatter. A CI job warns when pages exceed 90 days without review.
- For technical documentation (architecture, protocol specs), generate from source where possible. Rust doc comments, TypeScript type exports, and libp2p protocol definitions should be the single source of truth.
- Separate "evergreen" content (vision, philosophy, why P2P matters) from "versioned" content (setup guides, API references, configuration). Evergreen content rarely needs updates. Versioned content must track app releases.

**Warning signs:**
- Documentation references features or configurations that no longer exist.
- Users opening issues that are clearly caused by following outdated docs.
- No `last_verified` or `last_updated` metadata on documentation pages.
- Documentation PRs are never part of feature development PRs.

**Phase to address:**
Phase 5 (Documentation) for initial creation, but the freshness policy and CI checks must be established in Phase 1 as part of the monorepo structure.

---

### Pitfall 7: Shared Svelte Components Between Tauri App and Marketing Site Create Coupling Nightmare

**What goes wrong:**
The team sees shared UI opportunity: "The chat component in the desktop app could be reused in the marketing site demo." They extract components into a shared package. Now every change to a shared component must work in both the Tauri app (plain Svelte 5 + Vite) and the SvelteKit marketing site (SvelteKit + adapter-static). Different build pipelines, different module resolution, different SSR contexts. A component that uses `$app/environment` or Tauri APIs breaks the marketing site. A component that uses SvelteKit's `$app/navigation` breaks the Tauri app. Deploys of either app become blocked by the other.

**Why it happens:**
Code reuse seems obviously good. Svelte components look the same in both contexts. But SvelteKit and plain Svelte have different module resolution, different global stores, different lifecycle behaviors, and different build processes. `svelte-package` has known issues in monorepos where workspace dependency resolution points to source vs. dist at different stages.

**How to avoid:**
- Do NOT share Svelte components between the Tauri app and marketing site. They are different products with different runtimes.
- If visual consistency is needed, share design tokens (CSS custom properties, a shared Tailwind config) and utility functions (pure TypeScript, no framework imports). Never share `.svelte` files.
- The WebRTC demo on the marketing site should be a completely independent implementation. It can be inspired by the desktop app's UX but must not import from the Tauri app's source tree.
- If you absolutely must share components later, use `svelte-package` to build a proper library package in the workspace. But defer this to a future milestone -- it is premature optimization for v2.0.

**Warning signs:**
- Import paths crossing workspace boundaries (e.g., `import Chat from '../../src/lib/Chat.svelte'`).
- Components using `window.__TAURI__` without guards, breaking in the browser.
- `svelte-package` build errors about unresolved workspace dependencies.
- "Works in Tauri but breaks on the marketing site" or vice versa.

**Phase to address:**
Phase 1 (Monorepo Setup) -- establish hard boundaries between workspaces from the start. The `/site` workspace must NEVER import from `/src`.

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Hardcoding TURN credentials in client JS | No backend needed for ICE config | Credentials leak via view-source, cannot rotate | Never in production. Only for local dev. |
| Using Google STUN without TURN fallback | Zero cost, zero setup | 10-20% of demo visitors cannot connect | Only during initial development. Must add TURN before launch. |
| Copy-pasting SDP manually for demo | No signaling server needed | UX is terrible, no visitor will complete the flow | Only as a development tool, never as the public demo. |
| Single `package.json` at root for both apps | Simpler setup initially | Dependency conflicts, build pipeline collisions, version hell | Never. Workspace monorepo from day one. |
| Skipping code signing for "beta" launch | Saves $99-400 and days of CI setup | First impressions destroyed; "malware" reputation | Only if distributing via TestFlight/sideload to known testers. Never for public downloads. |
| Inlining documentation in marketing page components | Fast to write, no separate build | Impossible to search, version, or maintain at scale | Only for the initial landing page copy. Never for user/technical docs. |
| Same-tab WebRTC demo (no real networking) | No signaling server, works everywhere | Does not prove P2P works, feels fake | Acceptable as the starting demo, but must add real multi-device demo before launch. |

## Integration Gotchas

Common mistakes when connecting the marketing site to the existing Aether ecosystem.

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| SvelteKit + existing Svelte app | Trying to convert the Tauri app from plain Svelte to SvelteKit to "unify" everything | Keep the Tauri app as plain Svelte 5 + Vite. SvelteKit is ONLY for the marketing site. The Tauri app's adapter-static/SSR constraints are different from a public website. |
| WebRTC demo + libp2p desktop app | Trying to use js-libp2p in the browser demo to match the Rust libp2p desktop app's protocol | Use plain WebRTC APIs for the browser demo. js-libp2p adds significant bundle size and complexity. The demo is a showcase, not protocol interop. |
| Download links + CI/CD builds | Manually uploading binaries and updating download links | Use GitHub Releases as the source of truth. The marketing site fetches latest release metadata via GitHub API. Links never go stale. |
| Documentation content + app version | Writing docs against `main` branch (which may have unreleased changes) | Documentation targets specific tagged releases. Use git tags as version gates. |
| Marketing site deployment + Tauri app deployment | Deploying both from the same CI pipeline/workflow | Separate deployment pipelines. Marketing site deploys on every push to `/site`. Tauri app builds on tagged releases only. |
| WebRTC demo + Aether's PSK swarms | Trying to make the browser demo join actual Aether desktop swarms | The browser demo is self-contained. It demonstrates P2P concepts, not Aether protocol interop. Desktop and browser are different networks. |
| Analytics/tracking on marketing site | Adding Google Analytics or similar tracking | This contradicts Aether's privacy-first philosophy. Use privacy-respecting analytics (Plausible, Umami) or none at all. |

## Performance Traps

Patterns that work at small scale but fail as usage grows.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Bundling the entire WebRTC demo on the landing page | Landing page loads 500KB+ of WebRTC/demo code even for visitors who never click "try demo" | Lazy-load the demo component. Use dynamic `import()` triggered by user action. The landing page should be under 100KB. | Immediately on mobile devices with slow connections. |
| TURN server in a single region | Demo works fast in US/EU but has 200ms+ latency for visitors in Asia/Oceania | Use Cloudflare TURN (global anycast) or deploy regional TURN servers. For a demo, single region is acceptable initially but document the limitation. | When marketing targets non-US/EU audiences. |
| Fetching GitHub releases on every page load | Download page makes API call on every visit, hits GitHub rate limit (60 req/hr unauthenticated) | Cache release metadata at build time (SSG). Rebuild/redeploy marketing site on new GitHub release (webhook). | At ~60 visitors/hour to the download page. |
| Unoptimized hero images/videos on landing page | Landing page takes 5+ seconds to load on mobile | Use responsive images (`srcset`), WebP/AVIF format, lazy loading for below-fold content. If using video, use a poster image and load video on interaction. | Immediately on 3G/4G connections. |
| Documentation search with client-side full-text index | Initial load downloads entire search index (can be MBs for large docs) | Use a service like Pagefind (generates a compressed index at build time with partial loading) or defer search to a server endpoint | When documentation exceeds ~50 pages. |

## Security Mistakes

Domain-specific security issues beyond general web security.

| Mistake | Risk | Prevention |
|---------|------|------------|
| Exposing TURN server credentials in client-side JavaScript without rotation | Attackers use your TURN server as a free relay for their own traffic, running up bandwidth costs | Fetch short-lived TURN credentials from a server endpoint. Use time-limited HMAC-based credentials (RFC 5766 long-term credentials). |
| WebRTC demo leaking local IP addresses via ICE candidates | Visitors' private/local IP addresses visible in the browser's ICE candidate gathering, privacy violation | Use `iceCandidatePoolSize: 0` and filter candidates to only include relay (TURN) candidates in the demo if privacy is critical. Document the behavior. |
| Download links pointing to HTTP instead of HTTPS | Binary downloads can be intercepted and replaced (MITM), especially on public WiFi | All download links must be HTTPS. GitHub Releases are HTTPS by default. If hosting binaries elsewhere, enforce HTTPS. |
| Not verifying binary checksums on download page | Users cannot verify the binary they downloaded matches what was built | Publish SHA-256 checksums alongside every binary. Automate checksum generation in CI. Display checksums on the download page. |
| Marketing site accepting user input (contact forms, newsletter signup) without rate limiting | Spam bots abuse forms, email bombing, potential for XSS if input is reflected | Use Cloudflare Turnstile (privacy-respecting captcha alternative) or honeypot fields. Server-side rate limiting on any form endpoint. For a privacy-first product, avoid collecting email addresses at all if possible. |
| Mixing Aether's sovereign identity claims with third-party tracking scripts | Visitors see Google Analytics, Facebook Pixel, etc. on a site that promises "zero tracking" and "sovereign data." Instant credibility loss. | Use self-hosted analytics (Umami, Plausible self-hosted) or no analytics. If analytics are needed, be transparent about it on the privacy page. |

## UX Pitfalls

Common user experience mistakes in this domain.

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| WebRTC demo requires a second person to try | Solo visitor lands on site, clicks "try P2P demo," sees "waiting for peer to connect..." forever. Leaves. | Default to a same-browser demo (two virtual peers in one tab). Optionally offer a "try with a friend" mode with room codes. |
| Download page does not auto-detect OS | Visitor on macOS sees Windows/Linux/macOS buttons with no indication which is for them. Decision fatigue. | Auto-detect OS via `navigator.platform` / `navigator.userAgent`. Pre-select the correct download button. Show "Not on macOS? Download for [other OS]" link below. |
| "Coming soon" placeholders on the marketing site | Visitors see empty sections labeled "documentation coming soon" or "demo under construction." Looks abandoned. | Launch with fewer pages that are complete. A landing page + download page with no documentation is better than a documentation section with placeholder content. |
| Technical jargon on the landing page | Non-technical visitors see "CRDT," "DHT," "libp2p," "Ed25519" and bounce immediately. | Lead with benefits ("Your conversations, your rules. No servers, no snooping."). Technical details live in a separate "How it works" or "Architecture" page for developers. |
| Demo requires microphone/camera permission immediately | Browser permission dialog pops up before the visitor understands what the demo does. Most deny and never come back. | Start the demo with text/data channel only. Voice is opt-in: "Want to try voice? Click here" which then requests mic permission with context. |
| No fallback when WebRTC is blocked | Corporate networks, some mobile carriers, and privacy browsers block WebRTC. Demo shows nothing. | Detect WebRTC support before loading the demo. If unavailable, show a pre-recorded video of the demo with an explanation: "WebRTC is blocked on your network. Here's what the demo looks like." |

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **WebRTC Demo:** Works in Chrome and Firefox but has not been tested in Safari (Safari has WebRTC quirks with unified plan, codec support, and ICE handling) -- verify cross-browser testing
- [ ] **Download Page:** Links work but binaries are unsigned -- verify Gatekeeper/SmartScreen behavior on a fresh machine (not developer machine with exceptions)
- [ ] **Documentation:** Content is written but code examples have not been tested against the current app version -- verify every code snippet runs
- [ ] **Landing Page:** Looks good on desktop but has not been tested on mobile -- verify responsive layout, touch interactions, and performance on throttled connection
- [ ] **TURN Configuration:** TURN server is configured but credentials are hardcoded -- verify credential rotation mechanism exists
- [ ] **SEO:** Pages render but have no meta tags, Open Graph, or structured data -- verify social sharing preview with Facebook/Twitter/LinkedIn debugger tools
- [ ] **Analytics:** Site is live but nobody knows if anyone visits -- verify privacy-respecting analytics are collecting data (or verify the conscious decision to not track)
- [ ] **Demo Cleanup:** WebRTC connections are established but never cleaned up -- verify `RTCPeerConnection.close()` is called on page navigation/unmount to prevent memory leaks
- [ ] **Accessibility:** Site looks good but has not been tested with screen reader or keyboard navigation -- verify ARIA labels, focus management, color contrast (WCAG AA minimum)
- [ ] **Error States:** Demo has "connected" state but no "failed" or "timed out" states -- verify all ICE connection states are handled with user-facing messages

## Recovery Strategies

When pitfalls occur despite prevention, how to recover.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Build pipeline collision (monorepo) | MEDIUM | Extract the marketing site into a proper workspace. May require moving files, updating imports, and reconfiguring CI. Takes 1-2 days. |
| No signaling server for demo | LOW | Add a minimal WebSocket signaling server (Cloudflare Workers, ~50 lines). Or fall back to same-tab demo. Takes a few hours. |
| Unsigned binaries published | HIGH | Remove download links immediately. Set up code signing. Re-build and re-sign all binaries. Re-upload. Re-notify any users who downloaded unsigned versions. Reputation damage may persist. |
| Stale documentation | MEDIUM | Audit all doc pages against current app version. Update code examples. Add freshness dates. Takes 2-5 days depending on doc volume. |
| TURN not configured, demo fails for corporate users | LOW | Add TURN configuration (Cloudflare free tier). Redeploy. Takes a few hours. |
| Shared component coupling | HIGH | Untangling shared components requires duplicating code, removing cross-workspace imports, and testing both apps independently. Takes 3-5 days. |
| WebRTC demo broken in Safari | MEDIUM | Safari WebRTC quirks are well-documented. Usually requires SDP munging or codec negotiation changes. Takes 1-2 days. |
| Analytics tracking contradicts privacy philosophy | LOW | Remove tracking scripts. Issue a transparency note. Replace with self-hosted alternative. Takes hours. |

## Pitfall-to-Phase Mapping

How roadmap phases should address these pitfalls.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Monorepo build collision | Phase 1: Project Setup | Both `npm run dev` and `npm run build` work independently for each workspace without conflict |
| Signaling server requirement | Phase 2: Interactive Demo | Demo connects two peers in under 10 seconds with no manual SDP exchange |
| HTTPS requirement | Phase 3: Deployment | All WebRTC APIs function on production domain (not just localhost) |
| STUN/TURN misconfiguration | Phase 2-3: Demo + Deployment | Demo tested from corporate network / VPN / mobile tethering |
| Code signing not configured | Phase 4: Download/Distribution (start procurement in Phase 1) | Downloaded binary opens without Gatekeeper/SmartScreen warning on a fresh machine |
| Documentation staleness | Phase 5: Documentation (freshness policy in Phase 1) | CI job validates doc freshness dates; all code examples run against tagged release |
| Shared component coupling | Phase 1: Monorepo Setup | No import paths cross workspace boundaries; `grep -r "from '../../src"` returns zero results in `/site` |
| WebRTC demo UX (needs second person) | Phase 2: Interactive Demo | Solo visitor can complete the demo end-to-end without another person |
| Download page OS detection | Phase 4: Download/Distribution | Visitor's OS is pre-selected on page load |
| Privacy contradiction (tracking) | Phase 3: Deployment | No third-party tracking scripts loaded; verified via browser network tab |
| Demo cross-browser compatibility | Phase 2: Interactive Demo | Demo tested and passing in Chrome, Firefox, Safari, and Edge |
| Demo performance (lazy loading) | Phase 2-3: Demo + Landing Page | Landing page Lighthouse performance score > 90 without demo loaded |

## Sources

- [Tauri v2 SvelteKit Configuration](https://v2.tauri.app/start/frontend/sveltekit/) -- Official Tauri docs on SvelteKit integration, adapter-static requirements
- [MDN: getUserMedia() Secure Context](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia) -- HTTPS requirement documentation
- [WebRTC NAT Traversal Guide](https://webrtc.link/en/articles/stun-turn-servers-webrtc-nat-traversal/) -- STUN/TURN server requirements and failure rates
- [Cloudflare TURN Pricing](https://developers.cloudflare.com/realtime/turn/faq/) -- Free tier details for TURN relay
- [Tauri v2 macOS Code Signing](https://v2.tauri.app/distribute/sign/macos/) -- Official signing and notarization guide
- [Tauri v2 Windows Code Signing](https://v2.tauri.app/distribute/sign/windows/) -- HSM/Azure Key Vault requirements
- [Ship Tauri v2 Like a Pro: Code Signing](https://dev.to/tomtomdu73/ship-your-tauri-v2-app-like-a-pro-code-signing-for-macos-and-windows-part-12-3o9n) -- Practical pitfalls with certificate setup
- [SvelteKit Monorepo Component Sharing](https://github.com/sveltejs/kit/discussions/12208) -- Known issues with svelte-package in monorepos
- [libp2p WebRTC Transport](https://libp2p.io/docs/webrtc/) -- Browser-to-browser relay requirements
- [js-libp2p WebRTC](https://docs.libp2p.io/guides/getting-started/webrtc/) -- Browser node capabilities and limitations
- [TURN Server Costs Guide](https://dev.to/alakkadshaw/turn-server-costs-a-complete-guide-1c4b) -- Cost comparison for managed TURN services
- [Documentation Maintenance Best Practices](https://beta.buildwithfern.com/post/documentation-maintenance-best-practices) -- Freshness policies and audit schedules
- [WebRTC Security 2025](https://webrtc.ventures/2025/07/webrtc-security-in-2025-protocols-vulnerabilities-and-best-practices/) -- Current security landscape

---
*Pitfalls research for: Marketing Site with Interactive P2P Demo (Aether v2.0)*
*Researched: 2026-03-04*

<claude-mem-context>
# Recent Activity

<!-- This section is auto-generated by claude-mem. Edit content outside the tags. -->

### Feb 16, 2026

| ID | Time | T | Title | Read |
|----|------|---|-------|------|
| #5897 | 12:15 PM | 🔵 | Aether v1.1 Feature Landscape Research Completed | ~551 |
| #5896 | 12:14 PM | 🔵 | Aether v1.1 Community Features Architecture Research Complete | ~548 |
</claude-mem-context>