# Roadmap: Aether

## Milestones

- ✅ **v1.0 Walking Skeleton** — Phases 1-5 (shipped 2026-02-16)
- ✅ **v1.1 Community** — Phases 6-10 (shipped 2026-02-24)
- 🚧 **v2.0 Marketing Site** — Phases 11-15 (in progress)

## Phases

<details>
<summary>✅ v1.0 Walking Skeleton (Phases 1-5) — SHIPPED 2026-02-16</summary>

- [x] Phase 1: Foundation & Identity (2/2 plans) — completed 2026-02-13
- [x] Phase 2: Sovereign Network (2/2 plans) — completed 2026-02-13
- [x] Phase 3: Invitation System (2/2 plans) — completed 2026-02-16
- [x] Phase 3.1: Fix Keychain Prompts (1/1 plan, INSERTED) — completed 2026-02-16
- [x] Phase 4: Real-Time Voice (3/3 plans) — completed 2026-02-16
- [x] Phase 5: Persistent Chat (3/3 plans) — completed 2026-02-16

Full details: `.planning/milestones/v1.0-ROADMAP.md`

</details>

<details>
<summary>✅ v1.1 Community (Phases 6-10) — SHIPPED 2026-02-24</summary>

- [x] **Phase 6: Foundation** - Voice mute, petnames/contacts, swarm management — completed 2026-02-16
- [x] **Phase 7: Channel Management** - Create, rename, archive channels with CRDT sync — completed 2026-02-16
- [x] **Phase 8: Unread & Mentions** - Unread indicators and @mention system — completed 2026-02-16
- [x] **Phase 8.1: Fix CI CMake** - opus-codec, CI workflow, Linux release — completed 2026-02-19
- [x] **Phase 9: Peer Moderation** - Hide and block peers locally — completed 2026-02-23
- [x] **Phase 9.1: Fix isCreator Integration Bug** - Call setLocalIdentity so channel create/rename/delete UI renders — completed 2026-02-23
- [x] **Phase 10: Desktop Notifications** - Native OS notifications for messages and mentions — completed 2026-02-23

Full details: `.planning/milestones/`

</details>

### 🚧 v2.0 Marketing Site (In Progress)

**Milestone Goal:** Build a public-facing marketing website that explains Aether's sovereign P2P philosophy, provides user and technical documentation, offers an interactive browser-based P2P demo, and drives desktop app downloads.

- [x] **Phase 11: Site Scaffold** - SvelteKit project in /site with Vercel deployment, global nav/footer, responsive foundation
- [x] **Phase 12: Landing Page** - Vision-first hero, how-it-works, feature cards, trade-offs, open-source trust signals
- [x] **Phase 13: Interactive Demo** - Browser-based WebRTC P2P text chat sandbox with share-link signaling
- [x] **Phase 14: Documentation** - User guides and technical architecture docs with sidebar navigation and search (completed 2026-03-04)
- [ ] **Phase 15: Downloads** - Platform-aware download page with OS detection, checksums, and system requirements

## Phase Details

### Phase 11: Site Scaffold
**Goal**: A deployed, empty-but-functional SvelteKit site with responsive layout, global navigation, and zero third-party dependencies -- the foundation every other phase builds on
**Depends on**: Phase 10 (v1.1 complete)
**Requirements**: SITE-01, SITE-02, SITE-03, SITE-04, LAND-05
**Success Criteria** (what must be TRUE):
  1. Visitor can load the site at a public Vercel URL and see a responsive layout that works from 375px mobile to desktop widths
  2. Visitor sees a navigation bar with Home, Docs, Download, Demo, and GitHub links, plus a footer with the same links
  3. Site serves zero cookies, loads zero third-party scripts, and includes zero analytics
  4. Running `cd site && npm run build` succeeds without affecting the Tauri app build at the repo root
  5. Page weight is under 500KB and LCP is under 2.5s on the deployed URL
**Plans**: 2 plans

Plans:
- [x] 11-01-PLAN.md — SvelteKit scaffold, Tailwind v4 design system, Nav/Footer/MobileMenu components
- [x] 11-02-PLAN.md — Route stubs with contextual teasers, docs sidebar layout, Vercel deployment

### Phase 12: Landing Page
**Goal**: Visitors understand Aether's sovereign P2P philosophy, see what it does, and trust the project within a single page scroll
**Depends on**: Phase 11 (scaffold deployed with nav/footer)
**Requirements**: LAND-01, LAND-02, LAND-03, LAND-04, LAND-06
**Success Criteria** (what must be TRUE):
  1. Visitor sees a hero section in the first viewport that communicates the "Sovereign Node" P2P philosophy and includes a download CTA
  2. Visitor sees a "How It Works" section with visual steps for generating identity, sharing a secret code, and connecting peer-to-peer
  3. Visitor sees feature highlight cards for voice chat, text chat, channels, moderation, and contacts
  4. Visitor sees a "Trade-offs We Chose" section that honestly documents limitations (simultaneous presence, transport-only encryption, hardware-bound identity, 8-peer voice limit)
  5. Visitor sees open-source trust signals: GitHub link, license badge, and "View Source" call-to-action
**Plans**: 1 plan

Plans:
- [x] 12-01-PLAN.md — Complete landing page: hero, How It Works, feature cards, trade-offs, trust signals

### Phase 13: Interactive Demo
**Goal**: Visitors can experience P2P communication directly in their browser without downloading anything -- the only "try before you download" demo in the P2P messaging space
**Depends on**: Phase 11 (scaffold deployed with HTTPS, needed for WebRTC)
**Requirements**: DEMO-01, DEMO-02, DEMO-03, DEMO-04
**Success Criteria** (what must be TRUE):
  1. Visitor can open the /demo page and create an ephemeral P2P text chat session entirely in the browser
  2. Visitor can share a link that another person opens to connect via WebRTC DataChannel
  3. Connected peers can exchange text messages directly P2P with no server relaying message content
  4. Demo displays connection status (connecting/connected/disconnected) and prominently prompts the visitor to download the full app
**Plans**: 2 plans

Plans:
- [x] 13-01-PLAN.md — Signaling relay API route, WebRTC connection manager module, demo page options
- [x] 13-02-PLAN.md — Interactive demo page UI with chat, QR sharing, connection states, download banner

### Phase 14: Documentation
**Goal**: Visitors can learn everything about using and understanding Aether -- from first install to protocol internals -- through searchable, navigable documentation
**Depends on**: Phase 11 (scaffold with mdsvex configured)
**Requirements**: UDOC-01, UDOC-02, UDOC-03, UDOC-04, TDOC-01, TDOC-02, TDOC-03, TDOC-04
**Success Criteria** (what must be TRUE):
  1. Visitor can read a Getting Started guide that walks through install, first launch, and identity creation
  2. Visitor can read guides covering swarms, inviting peers, channels, voice chat, and moderation
  3. Visitor can read an architecture overview with system diagram and protocol documentation for networking, identity, CRDTs, voice, and encryption
  4. Visitor can navigate docs via a sidebar with section hierarchy and prev/next links between pages
  5. Visitor can search all documentation via full-text search and find relevant pages
**Plans**: 4 plans

Plans:
- [ ] 14-01-PLAN.md — Docs infrastructure: rehype plugins, nav tree, sidebar/prev-next components, prose styles
- [ ] 14-02-PLAN.md — User documentation: Getting Started guide + 5 user guides (swarms, peers, channels, voice, moderation)
- [ ] 14-03-PLAN.md — Technical documentation: architecture overview with SVG diagrams + 5 protocol deep-dives
- [ ] 14-04-PLAN.md — Full-text search: FlexSearch integration with search endpoint and sidebar search UI

### Phase 15: Downloads
**Goal**: Visitors can download the correct Aether binary for their platform with confidence in its authenticity
**Depends on**: Phase 11 (scaffold deployed), Phase 14 (docs exist to inform download decision)
**Requirements**: DOWN-01, DOWN-02, DOWN-03
**Success Criteria** (what must be TRUE):
  1. Visitor sees their detected operating system with a primary download button for their platform (macOS, Windows, or Linux)
  2. Visitor can access all platform downloads with version number, file size, and system requirements listed
  3. Visitor can verify any download via SHA-256 checksums displayed on the page
**Plans**: 1 plan

Plans:
- [ ] 15-01: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 11 > 12 > 13 > 14 > 15

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Foundation & Identity | v1.0 | 2/2 | ✓ Complete | 2026-02-13 |
| 2. Sovereign Network | v1.0 | 2/2 | ✓ Complete | 2026-02-13 |
| 3. Invitation System | v1.0 | 2/2 | ✓ Complete | 2026-02-16 |
| 3.1 Fix Keychain Prompts | v1.0 | 1/1 | ✓ Complete | 2026-02-16 |
| 4. Real-Time Voice | v1.0 | 3/3 | ✓ Complete | 2026-02-16 |
| 5. Persistent Chat | v1.0 | 3/3 | ✓ Complete | 2026-02-16 |
| 6. Foundation | v1.1 | 3/3 | ✓ Complete | 2026-02-16 |
| 7. Channel Management | v1.1 | 3/3 | ✓ Complete | 2026-02-16 |
| 8. Unread & Mentions | v1.1 | 3/3 | ✓ Complete | 2026-02-16 |
| 8.1 Fix CI CMake | v1.1 | 1/1 | ✓ Complete | 2026-02-19 |
| 9. Peer Moderation | v1.1 | 4/4 | ✓ Complete | 2026-02-23 |
| 9.1 Fix isCreator Bug | v1.1 | 1/1 | ✓ Complete | 2026-02-23 |
| 10. Desktop Notifications | v1.1 | 1/1 | ✓ Complete | 2026-02-23 |
| 11. Site Scaffold | v2.0 | 2/2 | ✓ Complete | 2026-03-04 |
| 12. Landing Page | v2.0 | 1/1 | Complete | 2026-03-04 |
| 13. Interactive Demo | v2.0 | Complete    | 2026-03-04 | 2026-03-04 |
| 14. Documentation | 4/4 | Complete   | 2026-03-04 | - |
| 15. Downloads | v2.0 | 0/? | Not started | - |

---
*Created: 2026-02-13*
*Last updated: 2026-03-04 (Phase 14 planned)*
