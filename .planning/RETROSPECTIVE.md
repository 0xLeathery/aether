# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into future planning.*

## Milestone: v2.0 — Marketing Site

**Shipped:** 2026-03-04
**Phases:** 5 | **Plans:** 11

### What Was Built
- SvelteKit marketing site with Tailwind v4, responsive design, zero third-party dependencies
- Vision-first landing page communicating sovereign P2P philosophy
- Browser-based WebRTC P2P text chat demo (unique in the P2P messaging space)
- Full documentation: 6 user guides, architecture overview, 5 protocol deep-dives, SVG diagrams
- FlexSearch client-side full-text search across all documentation
- Retroactive verification system and milestone audit process

### What Worked
- Single-file page pattern (landing ~300 LOC, demo ~559 LOC) kept content editing simple
- Hand-written inline SVGs eliminated all icon library dependencies
- Prerendered search.json (7.5KB) gave zero-runtime-cost full-text search
- Milestone audit + gap closure phase (Phase 15) caught real issues before shipping
- Native WebRTC API without wrapper libraries reduced demo complexity
- Docs-before-downloads ordering built trust before asking for commitment

### What Was Inefficient
- Phase 11 lacked VERIFICATION.md — required retroactive verification in Phase 15
- SITE-04 referenced Cloudflare Pages even though Vercel was the actual deployment — stale requirement text
- prerender.handleHttpError was added as a workaround for incremental doc creation, then had to be removed
- Nyquist validation coverage was partial (only Phase 12 had draft) — validation wasn't prioritized during execution

### Patterns Established
- Contextual stub pages instead of generic "coming soon" placeholders
- Prose typography via `.prose` CSS class with Tailwind @apply
- Architecture SVGs inline in markdown (no separate Svelte components)
- Protocol docs include Related Pages section for cross-linking
- Encryption docs lead with what IS encrypted before documenting limitations (honest positive framing)

### Key Lessons
1. Run verification immediately after each phase — retroactive verification is more expensive than inline verification
2. Keep requirement text platform-agnostic when deployment target may change during development
3. Single-file pages work well for marketing content but need clear commenting structure at scale
4. Milestone audit is valuable — it caught 8 real gaps that Phase 15 resolved before shipping

### Cost Observations
- Model mix: 100% opus (quality profile)
- v2.0 execution time: ~51 min across 11 plans (5.7 min avg)
- Notable: Fastest milestone yet per-plan, despite being the first non-Tauri work

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Phases | Plans | Avg Plan Duration | Key Change |
|-----------|--------|-------|-------------------|------------|
| v1.0 | 6 | 13 | 10.5 min | Initial process, learning curve |
| v1.1 | 7 | 16 | 3.8 min | Process matured, 3x faster |
| v2.0 | 5 | 11 | 5.7 min | New domain (web), audit process added |

### Cumulative Quality

| Milestone | Requirements | Satisfied | Tech Debt Items |
|-----------|-------------|-----------|-----------------|
| v1.0 | 19 | 19/19 | 4 |
| v1.1 | 23 | 23/23 | 2 |
| v2.0 | 22 | 22/22 (3 deferred) | 3 |

### Top Lessons (Verified Across Milestones)

1. Connectivity-first validation prevents wasted work (v1.0 NAT traversal, v2.0 WebRTC demo)
2. Zero-dependency approach pays off in maintenance and trust (v1.0 no sidecar, v2.0 no icon libraries/analytics)
3. Milestone audits catch real gaps — formalize verification into every phase, not just milestone end
