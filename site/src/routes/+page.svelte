<script>
  import { siteConfig } from '$lib/constants';
</script>

<svelte:head>
  <title>Aether — Sovereign P2P Communication</title>
  <meta name="description" content="Own your identity. Communicate directly, peer-to-peer, with no central servers. Aether is a sovereign P2P voice and text app." />
</svelte:head>

<!-- Section 1: Hero -->
<section class="py-24 sm:py-28">
  <div class="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8">
    <div class="text-center">
      <h1 class="font-mono text-5xl font-bold tracking-tight text-accent-green sm:text-6xl lg:text-7xl">
        {siteConfig.name}
      </h1>
      <p class="mx-auto mt-6 max-w-2xl text-xl leading-relaxed text-text-secondary sm:text-2xl">
        Your node is sovereign territory. Aether connects you directly to your
        peers — no accounts, no servers, no company that can delete you. Your
        identity, your data, your rules.
      </p>

      <div class="mt-10 flex flex-col items-center gap-4 sm:flex-row sm:justify-center">
        <a
          href="/demo"
          class="inline-flex items-center rounded-lg bg-accent-green px-6 py-3 font-mono text-sm font-semibold text-bg-primary transition-colors hover:bg-accent-green-dim"
        >
          Try Demo
        </a>
        <a
          href={siteConfig.github}
          target="_blank"
          rel="noopener noreferrer"
          class="inline-flex items-center rounded-lg border border-border px-6 py-3 font-mono text-sm font-semibold text-text-primary transition-colors hover:border-border-bright hover:bg-bg-secondary"
        >
          View on GitHub
        </a>
      </div>
    </div>
  </div>
</section>

<!-- Section 2: How It Works -->
<section class="border-t border-border bg-bg-secondary py-20">
  <div class="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8">
    <div class="text-center">
      <h2 class="font-mono text-3xl font-bold tracking-tight text-text-primary sm:text-4xl">
        How It Works
      </h2>
      <p class="mx-auto mt-4 max-w-2xl text-lg text-text-secondary">
        Three steps to your first sovereign connection.
      </p>
    </div>

    <div class="mt-12 grid gap-8 lg:grid-cols-3">
      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <span class="font-mono text-3xl font-bold text-accent-amber">01</span>
        <h3 class="mt-4 font-mono text-lg font-semibold text-text-primary">Generate Your Identity</h3>
        <p class="mt-3 text-sm leading-relaxed text-text-secondary">
          Aether creates an Ed25519 keypair stored in your system keychain. No
          email. No password. No account. Your cryptographic identity lives only
          on your machine.
        </p>
      </div>

      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <span class="font-mono text-3xl font-bold text-accent-amber">02</span>
        <h3 class="mt-4 font-mono text-lg font-semibold text-text-primary">Share a Secret Code</h3>
        <p class="mt-3 text-sm leading-relaxed text-text-secondary">
          Paste a one-time invite code to your contact — over Signal, in person,
          anywhere you trust. The code is how peers find each other without a
          directory server.
        </p>
      </div>

      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <span class="font-mono text-3xl font-bold text-accent-amber">03</span>
        <h3 class="mt-4 font-mono text-lg font-semibold text-text-primary">Connect Peer-to-Peer</h3>
        <p class="mt-3 text-sm leading-relaxed text-text-secondary">
          WebRTC negotiates a direct connection between your nodes. Once
          connected, voice and text travel straight between you — no relay, no
          middleman, no egress bill.
        </p>
      </div>
    </div>

    <div class="mt-10 text-center">
      <a href="/demo" class="font-mono text-sm font-semibold text-accent-green transition-colors hover:text-accent-green-dim">
        Try it in your browser &rarr;
      </a>
    </div>
  </div>
</section>

<!-- Section 3: Features -->
<section class="py-20">
  <div class="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8">
    <div class="text-center">
      <h2 class="font-mono text-3xl font-bold tracking-tight text-text-primary sm:text-4xl">
        Built for Sovereignty
      </h2>
      <p class="mx-auto mt-4 max-w-2xl text-lg text-text-secondary">
        Everything you need. Nothing you don't.
      </p>
    </div>

    <div class="mt-12 grid gap-8 sm:grid-cols-2 lg:grid-cols-3">
      <!-- Voice Chat -->
      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <svg class="h-8 w-8 text-accent-green" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 18.75a6 6 0 0 0 6-6v-1.5m-6 7.5a6 6 0 0 1-6-6v-1.5m6 7.5v3.75m-3.75 0h7.5M12 15.75a3 3 0 0 1-3-3V4.5a3 3 0 1 1 6 0v8.25a3 3 0 0 1-3 3Z" />
        </svg>
        <h3 class="mt-4 font-mono text-lg font-semibold text-text-primary">Voice Chat</h3>
        <p class="mt-2 text-sm leading-relaxed text-text-secondary">
          Crystal-clear peer-to-peer voice sessions powered by the Opus codec.
          Sub-50ms latency with no relay servers. Supports up to 8 participants
          in a P2P mesh.
        </p>
      </div>

      <!-- Text Chat -->
      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <svg class="h-8 w-8 text-accent-green" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12c0 4.556-4.03 8.25-9 8.25a9.764 9.764 0 0 1-2.555-.337A5.972 5.972 0 0 1 5.41 20.97a5.969 5.969 0 0 1-.474-.065 4.48 4.48 0 0 0 .978-2.025c.09-.457-.133-.901-.467-1.226C3.93 16.178 3 14.189 3 12c0-4.556 4.03-8.25 9-8.25s9 3.694 9 8.25Z" />
        </svg>
        <h3 class="mt-4 font-mono text-lg font-semibold text-text-primary">Text Chat</h3>
        <p class="mt-2 text-sm leading-relaxed text-text-secondary">
          CRDT-synced messages that survive disconnections. Your conversations
          are stored locally and sync when peers reconnect — no server to go
          down, no timeline to scroll past.
        </p>
      </div>

      <!-- Channels -->
      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <svg class="h-8 w-8 text-accent-green" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12" />
        </svg>
        <h3 class="mt-4 font-mono text-lg font-semibold text-text-primary">Channels</h3>
        <p class="mt-2 text-sm leading-relaxed text-text-secondary">
          Named channels organize conversations within a swarm. CRDT metadata
          ensures channel state is consistent across all peers without a central
          coordinator.
        </p>
      </div>

      <!-- Moderation -->
      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <svg class="h-8 w-8 text-accent-green" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12.75 11.25 15 15 9.75m-3-7.036A11.959 11.959 0 0 1 3.598 6 11.99 11.99 0 0 0 3 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285Z" />
        </svg>
        <h3 class="mt-4 font-mono text-lg font-semibold text-text-primary">Moderation</h3>
        <p class="mt-2 text-sm leading-relaxed text-text-secondary">
          Hide or block any peer — locally, instantly, without reporting to
          anyone. You control your experience. Your decisions stay on your
          machine.
        </p>
      </div>

      <!-- Contacts -->
      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <svg class="h-8 w-8 text-accent-green" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z" />
        </svg>
        <h3 class="mt-4 font-mono text-lg font-semibold text-text-primary">Contacts</h3>
        <p class="mt-2 text-sm leading-relaxed text-text-secondary">
          Assign private petnames to your peers. Your contacts use your names
          for them, not whatever they chose to call themselves. Identity on your
          terms.
        </p>
      </div>
    </div>
  </div>
</section>

<!-- Section 4: Trade-offs We Chose -->
<section class="border-t border-border bg-bg-secondary py-20">
  <div class="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8">
    <div class="text-center">
      <h2 class="font-mono text-3xl font-bold tracking-tight text-text-primary sm:text-4xl">
        Trade-offs We Chose
      </h2>
      <p class="mx-auto mt-4 max-w-2xl text-lg text-text-secondary">
        Radical honesty about our architectural decisions. These are constraints
        we chose deliberately.
      </p>
    </div>

    <div class="mt-12 grid gap-6 sm:grid-cols-2">
      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <h3 class="font-mono text-base font-semibold text-text-primary">Simultaneous Presence Required</h3>
        <p class="mt-3 font-mono text-xs uppercase tracking-wider text-text-muted">In practice:</p>
        <p class="mt-1 text-sm leading-relaxed text-text-secondary">
          Both peers must be online at the same time to exchange messages. There
          is no inbox, no store-and-forward, no offline delivery.
        </p>
        <p class="mt-3 font-mono text-xs uppercase tracking-wider text-text-muted">Why we chose it:</p>
        <p class="mt-1 text-sm leading-relaxed text-text-secondary">
          Pure peer-to-peer sync eliminates relay servers entirely. No egress
          costs, no central point of failure, no company holding your messages
          in transit.
        </p>
      </div>

      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <h3 class="font-mono text-base font-semibold text-text-primary">Transport-Only Encryption</h3>
        <p class="mt-3 font-mono text-xs uppercase tracking-wider text-text-muted">In practice:</p>
        <p class="mt-1 text-sm leading-relaxed text-text-secondary">
          Messages are encrypted in transit between peers. Data at rest on your
          disk is not encrypted by Aether — your OS and disk encryption handle
          that.
        </p>
        <p class="mt-3 font-mono text-xs uppercase tracking-wider text-text-muted">Why we chose it:</p>
        <p class="mt-1 text-sm leading-relaxed text-text-secondary">
          Simplifies the v1 architecture and preserves local search and indexing
          performance. Full at-rest encryption is a planned future improvement.
        </p>
      </div>

      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <h3 class="font-mono text-base font-semibold text-text-primary">Hardware-Bound Identity</h3>
        <p class="mt-3 font-mono text-xs uppercase tracking-wider text-text-muted">In practice:</p>
        <p class="mt-1 text-sm leading-relaxed text-text-secondary">
          Your identity keypair lives in your system keychain and cannot be
          exported or transferred. Losing your device means starting fresh with
          a new identity.
        </p>
        <p class="mt-3 font-mono text-xs uppercase tracking-wider text-text-muted">Why we chose it:</p>
        <p class="mt-1 text-sm leading-relaxed text-text-secondary">
          System keychain security (Secure Enclave on macOS, TPM on Windows)
          makes private keys non-extractable. This is a stronger security
          guarantee than seed phrases.
        </p>
      </div>

      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <h3 class="font-mono text-base font-semibold text-text-primary">8-Peer Voice Limit</h3>
        <p class="mt-3 font-mono text-xs uppercase tracking-wider text-text-muted">In practice:</p>
        <p class="mt-1 text-sm leading-relaxed text-text-secondary">
          Voice sessions support a maximum of 8 simultaneous participants.
          Beyond 8, connection quality degrades due to mesh topology bandwidth
          requirements.
        </p>
        <p class="mt-3 font-mono text-xs uppercase tracking-wider text-text-muted">Why we chose it:</p>
        <p class="mt-1 text-sm leading-relaxed text-text-secondary">
          WebRTC P2P mesh requires each peer to maintain N-1 direct connections.
          8 is the practical limit before latency and bandwidth costs outpace
          the benefit of avoiding a server.
        </p>
      </div>
    </div>
  </div>
</section>

<!-- Section 5: Trust Signals -->
<section class="border-t border-border py-20">
  <div class="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8">
    <div class="mx-auto max-w-2xl text-center">
      <p class="font-mono text-sm font-semibold uppercase tracking-widest text-accent-green">Open Source</p>
      <h2 class="mt-3 font-mono text-3xl font-bold text-text-primary">
        Built in the open. Always will be.
      </h2>
      <p class="mt-4 text-lg leading-relaxed text-text-secondary">
        Aether is {siteConfig.license}-licensed and developed entirely in
        public. Read the source, audit the protocol, build on it, fork it.
        Sovereignty includes the software itself.
      </p>
      <div class="mt-8 flex flex-wrap items-center justify-center gap-4">
        <a
          href={siteConfig.github}
          target="_blank"
          rel="noopener noreferrer"
          class="inline-flex items-center rounded-lg border border-border px-6 py-3 font-mono text-sm font-semibold text-text-primary transition-colors hover:border-border-bright hover:bg-bg-secondary"
        >
          View Source on GitHub
        </a>
        <span class="inline-flex items-center gap-2 rounded-full border border-border-bright px-4 py-2 font-mono text-sm text-text-secondary">
          {siteConfig.license} License
        </span>
      </div>
    </div>
  </div>
</section>
