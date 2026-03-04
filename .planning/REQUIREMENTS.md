# Requirements: Aether

**Defined:** 2026-02-16
**Core Value:** "The Sovereign Node" — Users own their identity and data completely. No central servers, no "cloud" to delete you, and zero egress costs.

## v1.1 Requirements (Complete)

All 23 v1.1 Community requirements satisfied. See traceability below.

### Voice

- [x] **VOIC-05**: User can mute/unmute their microphone during a voice session
- [x] **VOIC-06**: Mute state is visually indicated in the voice panel

### Identity / Contacts

- [x] **IDEN-04**: User can assign local petnames to peers that override self-asserted names in all UI
- [x] **IDEN-05**: User can view and manage a contacts list of known peers

### Channels

- [x] **CHAN-01**: User can create a new named channel within a swarm
- [x] **CHAN-02**: User can rename an existing channel
- [x] **CHAN-03**: User can delete (archive) a channel with confirmation
- [x] **CHAN-04**: Channel list syncs between peers via CRDT metadata
- [x] **CHAN-05**: Default "general" channel cannot be deleted or renamed

### Swarm Management

- [x] **SWRM-01**: User can rename a swarm locally
- [x] **SWRM-02**: User can leave a swarm with full data cleanup
- [x] **SWRM-03**: User can regenerate an invite link for a swarm

### Unread

- [x] **UNRD-01**: User sees unread indicators on channels with new messages
- [x] **UNRD-02**: User sees unread indicator on swarms containing unread channels
- [x] **UNRD-03**: Unread state clears when user views the channel

### Mentions

- [x] **MENT-01**: User can @mention peers with autocomplete in message input
- [x] **MENT-02**: Messages mentioning the current user are visually highlighted
- [x] **MENT-03**: Mentions are stored as public key references (name-change resilient)

### Notifications

- [x] **NOTF-01**: User receives desktop notifications for new messages when app is not focused
- [x] **NOTF-02**: User receives notification when mentioned by name
- [x] **NOTF-03**: Notification displays sender name and message preview

### Moderation

- [x] **MOD-01**: User can hide a peer's messages locally (reversible filter)
- [x] **MOD-02**: User can block a peer (hide messages + refuse voice audio)

## v2.0 Requirements

Requirements for v2.0 Marketing Site. Each maps to roadmap phases.

### Site Foundation

- [x] **SITE-01**: Visitor sees a responsive site that works on mobile (375px+) and desktop
- [x] **SITE-02**: Landing page loads in under 2.5s LCP with total weight under 500KB
- [x] **SITE-03**: Site uses zero cookies, zero analytics, zero third-party CDN assets
- [x] **SITE-04**: Site is deployed and accessible at a public URL

### Landing Page

- [x] **LAND-01**: Visitor sees a vision-first hero explaining Aether's sovereign P2P philosophy within the first viewport
- [x] **LAND-02**: Visitor sees a "How It Works" section with visual steps (generate identity, share secret code, connect P2P)
- [x] **LAND-03**: Visitor sees feature highlight cards for voice chat, text chat, channels, moderation, and contacts
- [x] **LAND-04**: Visitor sees a "Trade-offs We Chose" section honestly documenting limitations
- [x] **LAND-05**: Visitor sees clear navigation (Home, Docs, Download, Demo, GitHub) with footer links
- [x] **LAND-06**: Visitor sees open-source trust signals (GitHub link, license badge, "View Source" CTA)

### ~~Downloads~~ (Moved to Future Requirements)

~~Phase 15 (Downloads) was intentionally removed from roadmap. Requirements moved to Future Requirements.~~

### User Documentation

- [x] **UDOC-01**: Visitor can read a Getting Started guide covering install, first launch, and identity
- [x] **UDOC-02**: Visitor can read guides for swarms, inviting peers, channels, voice chat, and moderation
- [x] **UDOC-03**: Visitor can navigate docs via sidebar with section hierarchy and prev/next links
- [x] **UDOC-04**: Visitor can search documentation via full-text search

### Technical Documentation

- [x] **TDOC-01**: Visitor can read an architecture overview with system diagram and component boundaries
- [x] **TDOC-02**: Visitor can read protocol documentation for networking, identity, CRDTs, voice, and encryption
- [x] **TDOC-03**: Visitor sees visual architecture diagrams (SVG) comparing centralized vs P2P models
- [x] **TDOC-04**: Technical docs include honest documentation of encryption scope and limitations

### Interactive Demo

- [x] **DEMO-01**: Visitor can open a demo page and create an ephemeral P2P text chat session in the browser
- [x] **DEMO-02**: Visitor can share a link to connect with another peer via WebRTC DataChannel
- [x] **DEMO-03**: Connected peers can exchange text messages directly P2P (no server relay for messages)
- [x] **DEMO-04**: Demo shows connection status (connecting/connected/disconnected) and prompts to download the full app

## Future Requirements

Deferred to future milestones. Tracked but not in current roadmap.

### Content Expansion

- **CONT-01**: Blog / news section with release announcements
- **CONT-02**: Video walkthrough / recorded demo
- **CONT-03**: Internationalization / translated content

### Downloads (Deferred from v2.0)

- **DOWN-01**: Visitor sees their detected OS with a primary download button (macOS/Windows/Linux)
- **DOWN-02**: Visitor can access all platform downloads with version, file size, and system requirements
- **DOWN-03**: Visitor can verify downloads via SHA-256 checksums displayed on the page

### Distribution

- **DIST-01**: Code-signed binaries for macOS (Apple Developer ID + notarization)
- **DIST-02**: Code-signed binaries for Windows (EV certificate)

### Multi-Device

- **MDEV-01**: User can link multiple devices to the same identity
- **MDEV-02**: Message history syncs across linked devices

### Security

- **SEC-01**: User data is encrypted at rest (local database encryption)
- **SEC-02**: Web of trust verification for peer identity

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| User accounts / login on site | Site is informational; no state to persist. Contradicts serverless message. |
| Community forum | Community lives in the app. Link to GitHub Discussions for developer questions. |
| Analytics / tracking | Any tracking undermines privacy message. Check GitHub download counts instead. |
| Newsletter signup | Requires email service, stores PII, GDPR complexity. Use GitHub Watch/RSS. |
| Pricing page | Free and open source. No pricing context needed. |
| Competitor comparison tables | Feature-by-feature battles Aether loses on volume. Focus on philosophy instead. |
| Voice demo in browser | Voice over WebRTC adds significant complexity. Text demo sufficient for v2.0. |
| Auto-playing video | Intrusive, bandwidth-heavy, accessibility issues. Use static visuals. |

## Traceability

### v1.1 (Complete)

| Requirement | Phase | Status |
|-------------|-------|--------|
| VOIC-05 | Phase 6 | ✓ Done |
| VOIC-06 | Phase 6 | ✓ Done |
| IDEN-04 | Phase 6 | ✓ Done |
| IDEN-05 | Phase 6 | ✓ Done |
| SWRM-01 | Phase 6 | ✓ Done |
| SWRM-02 | Phase 6 | ✓ Done |
| SWRM-03 | Phase 6 | ✓ Done |
| CHAN-01 | Phase 9.1 | ✓ Done |
| CHAN-02 | Phase 9.1 | ✓ Done |
| CHAN-03 | Phase 9.1 | ✓ Done |
| CHAN-04 | Phase 7 | ✓ Done |
| CHAN-05 | Phase 7 | ✓ Done |
| UNRD-01 | Phase 8 | ✓ Done |
| UNRD-02 | Phase 8 | ✓ Done |
| UNRD-03 | Phase 8 | ✓ Done |
| MENT-01 | Phase 8 | ✓ Done |
| MENT-02 | Phase 8 | ✓ Done |
| MENT-03 | Phase 8 | ✓ Done |
| MOD-01 | Phase 9 | ✓ Done |
| MOD-02 | Phase 9 | ✓ Done |
| NOTF-01 | Phase 10 | ✓ Done |
| NOTF-02 | Phase 10 | ✓ Done |
| NOTF-03 | Phase 10 | ✓ Done |

### v2.0 (Complete)

| Requirement | Phase | Status |
|-------------|-------|--------|
| SITE-01 | Phase 15 | Complete |
| SITE-02 | Phase 15 | Complete |
| SITE-03 | Phase 15 | Complete |
| SITE-04 | Phase 15 | Complete |
| LAND-01 | Phase 12 | Complete |
| LAND-02 | Phase 12 | Complete |
| LAND-03 | Phase 12 | Complete |
| LAND-04 | Phase 12 | Complete |
| LAND-05 | Phase 15 | Complete |
| LAND-06 | Phase 12 | Complete |
| DOWN-01 | Future | Deferred |
| DOWN-02 | Future | Deferred |
| DOWN-03 | Future | Deferred |
| UDOC-01 | Phase 14 | Complete |
| UDOC-02 | Phase 14 | Complete |
| UDOC-03 | Phase 14 | Complete |
| UDOC-04 | Phase 14 | Complete |
| TDOC-01 | Phase 14 | Complete |
| TDOC-02 | Phase 14 | Complete |
| TDOC-03 | Phase 14 | Complete |
| TDOC-04 | Phase 14 | Complete |
| DEMO-01 | Phase 13 | Complete |
| DEMO-02 | Phase 13 | Complete |
| DEMO-03 | Phase 13 | Complete |
| DEMO-04 | Phase 13 | Complete |

**Coverage:**
- v1.1 requirements: 23 total (23 complete)
- v2.0 requirements: 22 total (3 moved to Future)
- Complete: 22 (all non-deferred v2.0 requirements resolved)
- Deferred to Future: 3 (DOWN-01, DOWN-02, DOWN-03) -- confirmed in Future Requirements section
- Unmapped: 0

---
*Requirements defined: 2026-02-16*
*Last updated: 2026-03-04 after Phase 15 milestone verification closure*
