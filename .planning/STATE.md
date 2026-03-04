---
gsd_state_version: 1.0
milestone: v2.0
milestone_name: Marketing Site
status: in-progress
stopped_at: Completed 15-01-PLAN.md
last_updated: "2026-03-04T03:37:38.767Z"
last_activity: 2026-03-04 -- Phase 15 Plan 01 complete
progress:
  total_phases: 5
  completed_phases: 4
  total_plans: 11
  completed_plans: 10
  percent: 98
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-04)

**Core value:** "The Sovereign Node" -- Users own their identity and data completely. No central servers, no "cloud" to delete you, zero egress costs.
**Current focus:** Phase 15 -- Milestone Verification & Closure (v2.0 Marketing Site)

## Current Position

Phase: 15 of 15 (Milestone Verification & Closure)
Plan: 1 of 2 in current phase
Status: Phase 15 Plan 01 complete, Plan 02 remaining
Last activity: 2026-03-04 -- Phase 15 Plan 01 complete

Progress: [██████████] 98% (v2.0 phases 11-15)

## Performance Metrics

**v1.0 Velocity:**
- Total plans completed: 13
- Average duration: 10.5 min
- Total execution time: 2.3 hours (137 min)

**v1.1 Velocity:**
- Total plans completed: 16
- Average duration: 3.8 min
- Total execution time: 60 min

**v2.0 Velocity:**
- Total plans completed: 8
- Average duration: 5.6 min
- Total execution time: ~45 min

## Accumulated Context

### Decisions

- v2.0 site lives in /site directory, fully independent from Tauri app (zero shared code)
- SvelteKit + adapter-vercel for deployment (changed from adapter-cloudflare during Phase 11 context)
- Tailwind CSS v4 via Vite plugin (no PostCSS config)
- mdsvex 0.12 for Markdown documentation (Svelte 5 compatible)
- Native WebRTC API for demo (no PeerJS/simple-peer)
- Zero analytics, zero cookies, zero third-party CDN assets
- LAND-05 (navigation) assigned to scaffold phase (global layout concern)
- Docs before downloads (trust signal ordering)
- Specified runtime: nodejs22.x in adapter-vercel config (local Node v25.7.0 unsupported by adapter)
- Stub pages have contextual teasers, not generic "coming soon" placeholders
- Docs sidebar layout shell built into scaffold for Phase 14 readiness
- [Phase 11]: Specified runtime nodejs22.x in adapter-vercel config to bypass Node v25.7.0 compatibility
- [Phase 12]: Single-file landing page (~300 lines) rather than component decomposition for easy content editing
- [Phase 12]: Hand-written inline SVG icons instead of icon library -- zero external dependencies
- [Phase 13]: Non-trickle ICE gathering with 10s timeout for simpler SDP signaling
- [Phase 13]: In-memory Map room store with TTL cleanup -- ephemeral by design, survives Vercel Fluid Compute
- [Phase 13]: Joiner creates DataChannel, initiator receives via ondatachannel -- deterministic negotiation
- [Phase 13]: Single-file 559-line demo page consistent with Phase 12 single-file pattern
- [Phase 13]: Auto-join via ?room= URL parameter for frictionless share-link experience
- [Phase 14]: Added prerender handleHttpError for /docs/* to allow incremental doc page creation across plans
- [Phase 14]: Prose typography via .prose CSS class with Tailwind @apply -- consistent theme variable usage
- [Phase 14]: Contributing card links to GitHub repo externally (contributing docs out of scope for v2.0)
- [Phase 14]: List format for guide index links to avoid nested <a> tags from rehype-autolink-headings
- [Phase 14]: Concise guide pages (150-300 words) with cross-links for natural feature discovery
- [Phase 14]: All architecture SVGs inline in markdown (no separate Svelte components) for simplicity
- [Phase 14]: Each protocol doc includes Related Pages section for cross-linking between architecture docs
- [Phase 14]: Encryption page leads with what IS encrypted before documenting limitations (honest positive framing)
- [Phase 14]: FlexSearch Index with forward tokenize for prefix matching (simple, fast for ~15 docs)
- [Phase 14]: Prerendered search.json with 500-char content per entry (7.5KB total, zero runtime cost)
- [Phase 15]: SITE-04 updated to platform-agnostic text (removed Cloudflare Pages reference) since Vercel was the actual deployment
- [Phase 15]: LAND-05 marked complete; GitHub link consolidation into navLinks deferred to Plan 15-02
- [Phase 15]: DOWN-01/02/03 confirmed already in Future Requirements section -- no redundant edits needed

### Pending Todos

- **UI Polish**: Discord-familiar interaction patterns (carried from v1.1)

### Blockers/Concerns

**Carried from v1.0/v1.1 (tech debt):**
- Multi-peer voice/chat sync untested (requires multiple machines)
- PSK swarms lack relay-based NAT traversal for WAN use case

**v2.0 specific:**
- Code signing (DIST-01, DIST-02) deferred to future milestone -- downloads page links to unsigned GitHub Releases
- WebRTC demo signaling approach (manual SDP vs Cloudflare Worker) to be resolved during Phase 13 planning
- STUN-only may fail for 10-20% of visitors behind symmetric NAT -- TURN configuration needed for demo

## Session Continuity

Last session: 2026-03-04T03:37:38.765Z
Stopped at: Completed 15-01-PLAN.md
Resume file: None

---
*Initialized: 2026-02-13*
*Last updated: 2026-03-04 (Phase 15 Plan 01 complete)*
