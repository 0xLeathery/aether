# Project: Aether

## What This Is
A Local-First, Sovereign Node communication platform that replaces centralized chat (Discord, Slack) with a decentralized, serverless architecture. Users connect peer-to-peer via encrypted mesh networking for real-time voice and CRDT-synced text chat, with zero infrastructure costs. Includes a public marketing site with interactive P2P demo and comprehensive documentation.

## Core Value
**"The Sovereign Node"**: Users own their identity and data completely. No central servers, no "cloud" to delete you, and zero egress costs.

## Context
**Why build this?**
To solve the "Client-Server" flaw of modern chat apps (Discord, Slack) which forces a middleman into every interaction. We are attacking "real-time group coordination" from scratch using a Local-First, Federated architecture.

**Success looks like:**
A "bare bones" Tauri app that can send a "Hello World" and voice audio between two computers on different networks (behind NAT) without a central server, using only a "Secret Code" to connect.

**Current state (v2.0 shipped 2026-03-04):**
Desktop app complete with ~6,948 LOC (Rust + Svelte/TypeScript). Marketing site shipped with ~3,357 LOC (Svelte/TypeScript/CSS/Markdown) — landing page, interactive WebRTC P2P demo, full documentation suite with search. Tech stack: Tauri v2, Svelte 5, libp2p 0.56, Automerge CRDTs, Opus codec (desktop); SvelteKit, Tailwind v4, FlexSearch, WebRTC (site). All 64 requirements across 3 milestones satisfied.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| **Pure Sync** | Maximizes decentralization; data only moves when peers are connected. | No offline delivery without a peer online. |
| **Desktop Only (v1)** | Focus engineering on high-performance "Command Center" node. | Tauri + Rust implementation. No mobile app for v1. |
| **Text & Voice Only** | Reduces complexity and bandwidth requirements for v1. | Images/Media out of scope. |
| **Hardware-Backed Identity** | Secure, non-exportable keys using system keychain (TouchID/Windows Hello). | Identity bound to device/hardware; no seed phrases. |
| **Secret Code Invite** | Simple access control; possession of the key = access. | `aether://<swarm-key>` format. |
| **Transport-Only Encryption** | Prioritizes v1 velocity and search performance. | Data encrypted in transit, cleartext on disk. |
| **Self-Asserted Profiles** | Low friction identity; users claim names, peers can override locally (Petnames). | No global registry collision handling. |
| **libp2p over Hyperswarm** | libp2p has mature Rust support; Hyperswarm requires Node.js sidecar (+50MB, IPC latency). | Pure Rust networking, no sidecar. |
| **Automerge over Yjs** | Automerge has native Rust crate with derive macros; Yjs is JS-first. | Automerge CRDTs for chat sync. |
| **PSK Swarm Isolation** | Pre-shared keys provide simple encrypted group formation. | TCP-only for PSK swarms (QUIC TLS conflicts with XSalsa20). |
| **Connectivity-First Phases** | Validate P2P mesh and voice transport before UI features. | If NAT traversal fails, product fails — test early. |
| **security CLI for keychain ACL** | security-framework crate caused crashes; CLI is stable. | macOS production builds skip password prompts. |
| **SvelteKit monorepo site** | Separate /site directory; zero shared code with Tauri app. | Independent build/deploy pipeline via Vercel. |
| **Tailwind v4 via Vite plugin** | No PostCSS config needed; native CSS theme variables. | Clean design system with @theme tokens. |
| **Native WebRTC for demo** | No PeerJS/simple-peer dependency; minimal code, full control. | 559-line single-file demo page. |
| **Zero analytics/cookies** | Site practices what app preaches about privacy. | SITE-03 requirement baked into architecture. |
| **FlexSearch client-side** | Prerendered search.json; zero runtime server cost. | 7.5KB search index, prefix matching. |

## Constraints
- **Tech Stack:** Tauri v2 (Rust), libp2p 0.56 (Networking), Automerge (CRDTs), cpal + Opus (Audio).
- **Site Stack:** SvelteKit, Tailwind v4, adapter-vercel, mdsvex, FlexSearch.
- **Environment:** Desktop (macOS, Windows, Linux). Site deployed on Vercel.
- **Security:** End-to-end encryption in transit; Sovereign identity via system keychain.
- **Budget/Ops:** $0 infrastructure cost (User devices are the servers; Vercel free tier for site).

## Requirements

### Validated

- ✓ **IDEN-01**: User can generate an Ed25519 keypair stored in system keychain — v1.0
- ✓ **IDEN-02**: User can set a self-asserted display name associated with their key — v1.0
- ✓ **IDEN-03**: User's identity persists across app restarts (key retrieved from keychain) — v1.0
- ✓ **NET-01**: User can discover peers via DHT using a shared Swarm Key — v1.0
- ✓ **NET-02**: User can connect to peers behind NAT via UDP holepunching — v1.0
- ✓ **NET-03**: User can see connection status of peers (online/offline/connecting) — v1.0
- ✓ **NET-04**: User can connect to peers on the same LAN without internet — v1.0
- ✓ **CHAT-01**: User can send and receive text messages in a channel — v1.0
- ✓ **CHAT-02**: Messages sync between peers via CRDTs (eventual consistency) — v1.0
- ✓ **CHAT-03**: User can view message history persisted locally — v1.0
- ✓ **CHAT-04**: User sees display names (or truncated public key) next to messages — v1.0
- ✓ **VOIC-01**: User can join a voice session with peers in a channel — v1.0
- ✓ **VOIC-02**: Voice streams directly between peers (P2P mesh, no relay) — v1.0
- ✓ **VOIC-03**: Voice uses Opus codec with sub-50ms end-to-end latency target — v1.0
- ✓ **VOIC-04**: Voice group size is hard-limited to 8 participants — v1.0
- ✓ **UX-01**: User can generate a Secret Code to invite others — v1.0
- ✓ **UX-02**: User can join a swarm by pasting a Secret Code — v1.0
- ✓ **UX-03**: User can see a channel list within a joined swarm — v1.0
- ✓ **UX-04**: App runs as a Tauri desktop application (macOS, Windows, Linux) — v1.0
- ✓ **VOIC-05**: User can mute/unmute microphone during voice — v1.1
- ✓ **VOIC-06**: Mute state visually indicated — v1.1
- ✓ **IDEN-04**: User can assign local petnames to peers — v1.1
- ✓ **IDEN-05**: User can view and manage contacts list — v1.1
- ✓ **CHAN-01–05**: Channel create, rename, delete, CRDT sync, default protection — v1.1
- ✓ **SWRM-01–03**: Swarm rename, leave, regenerate invite — v1.1
- ✓ **UNRD-01–03**: Unread indicators on channels/swarms, clear on view — v1.1
- ✓ **MENT-01–03**: @mention autocomplete, highlight, pubkey-based storage — v1.1
- ✓ **NOTF-01–03**: Desktop notifications for messages and mentions — v1.1
- ✓ **MOD-01–02**: Hide and block peers locally — v1.1
- ✓ **SITE-01–04**: Responsive site, fast LCP, zero tracking, deployed — v2.0
- ✓ **LAND-01–06**: Hero, how-it-works, features, trade-offs, navigation, trust signals — v2.0
- ✓ **DEMO-01–04**: Browser P2P demo with share links and connection status — v2.0
- ✓ **UDOC-01–04**: User docs with sidebar nav, prev/next, full-text search — v2.0
- ✓ **TDOC-01–04**: Architecture overview, protocol docs, SVG diagrams, encryption docs — v2.0

### Active

(No active requirements — next milestone not yet defined)

### Out of Scope

- **Mobile App**: Battery/Background sync is too complex; desktop-first.
- **Images/File Transfer**: Focus on low-latency text/voice first.
- **Offline "Store-and-Forward"**: Requires super-peers or relays; contradicts "Pure Sync".
- **Encryption at Rest**: Local DB is cleartext; transport encryption only.
- **Web of Trust**: Strict verification deferred to future milestone.
- **Global user search**: Requires central index; privacy risk; contradicts sovereign model.
- **Rich text / markdown in messages**: Plain text sufficient for current scope.
- **Built-in community forum**: Community features live in the app itself; site is informational.
- **Analytics / tracking on site**: Any tracking undermines privacy message.
- **Voice demo in browser**: Voice over WebRTC adds significant complexity; text demo sufficient.

---
*Last updated: 2026-03-04 after v2.0 milestone*
