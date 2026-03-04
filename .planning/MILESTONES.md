# Milestones

## v2.0 Marketing Site (Shipped: 2026-03-04)

**Phases completed:** 5 phases (11-15), 11 plans
**Timeline:** 1 day (2026-03-04)
**Codebase:** ~3,357 LOC (Svelte/TypeScript/CSS/Markdown), 19 commits, 44 files
**Requirements:** 22/22 satisfied (3 deferred to Future)

**Key accomplishments:**
- SvelteKit marketing site with Tailwind v4 design system, responsive layout, zero third-party dependencies
- Vision-first landing page with hero, how-it-works, feature cards, trade-offs, and trust signals
- Browser-based WebRTC P2P text chat demo with QR sharing and connection state management
- Full documentation suite: 6 user guides, 6 protocol deep-dives, SVG architecture diagrams
- FlexSearch-powered client-side full-text search across all documentation
- Retroactive verification and milestone audit with all gaps resolved

**Tech debt accepted:**
- System message field stripped at deserialization in demo (cosmetic)
- Google STUN servers contacted at runtime during demo sessions
- Dead NavLink import in MobileMenu (cosmetic)

**Archive:** `.planning/milestones/v2.0-ROADMAP.md`, `.planning/milestones/v2.0-REQUIREMENTS.md`

---

## v1.0 Walking Skeleton (Shipped: 2026-02-16)

**Phases completed:** 6 phases (1-5 + 3.1), 13 plans
**Timeline:** 4 days (2026-02-13 → 2026-02-16)
**Execution time:** 2.3 hours (137 min, 10.5 min avg/plan)
**Codebase:** ~6,948 LOC (3,474 Rust + 3,474 Svelte/TypeScript), 72 commits, 162 files

**Key accomplishments:**
- Sovereign Ed25519 identity with hardware-backed keychain storage (TouchID/Windows Hello)
- P2P networking via libp2p with mDNS discovery, Kademlia DHT, and NAT traversal (relay+dcutr+autonat)
- PSK-encrypted swarm isolation with Secret Code invitation system (aether:// URIs)
- Real-time P2P voice mesh with Opus codec, adaptive jitter buffer (15-120ms), 8-participant hard limit
- CRDT-persistent text chat with Automerge sync protocol and file-based persistence
- Terminal-aesthetic desktop UI with Tauri v2 + Svelte 5 three-column layout

**Tech debt accepted:**
- Multi-peer voice/chat sync testing (requires multiple machines)
- Sample rate mismatch (24kHz capture vs 48kHz Opus, device/OS handles upsampling)
- Compiler warnings (unused imports/functions, non-blocking)
- Accessibility: keyboard navigation on modal overlays

**Archive:** `.planning/milestones/v1.0-ROADMAP.md`, `.planning/milestones/v1.0-REQUIREMENTS.md`

---

## v1.1 Community (Shipped: 2026-02-24)

**Phases completed:** 7 phases (6-10 + 8.1 + 9.1), 16 plans
**Timeline:** 9 days (2026-02-16 → 2026-02-24)
**Requirements:** 23/23 satisfied

**Key accomplishments:**
- Voice mute/unmute toggle with capture stream optimization
- Petnames, contacts list with local identity management
- Channel management: create, rename, delete with CRDT-synced metadata
- Unread tracking with count-based indicators and @mention system
- Peer moderation: mute, hide, block tiers with per-swarm overrides
- Desktop notifications with focus tracking and mention detection
- CI pipeline fixed (opus-codec, Linux release matrix)
- isCreator integration bug fix

**Archive:** `.planning/milestones/`

---
