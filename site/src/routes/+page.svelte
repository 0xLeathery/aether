<script>
  import { siteConfig } from '$lib/constants';
</script>

<svelte:head>
  <title>Aether - Sovereign P2P Communication</title>
</svelte:head>

<!-- Hero Section -->
<section class="mx-auto max-w-5xl px-4 py-20 sm:px-6 lg:px-8">
  <div class="text-center">
    <h1 class="font-mono text-5xl font-bold tracking-tight text-accent-green sm:text-6xl lg:text-7xl">
      {siteConfig.name}
    </h1>
    <p class="mx-auto mt-6 max-w-2xl text-xl leading-relaxed text-text-secondary sm:text-2xl">
      Own your identity. Own your data. Communicate directly, peer-to-peer, with no central servers standing between you and the people you talk to.
    </p>

    <div class="mt-10 flex flex-col items-center gap-4 sm:flex-row sm:justify-center">
      <a
        href="/demo"
        class="inline-flex items-center rounded-lg bg-accent-green px-6 py-3 font-mono text-sm font-semibold text-bg-primary transition-colors hover:bg-accent-green-dim"
      >
        Try the Demo
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
</section>

<!-- Feature Preview Section -->
<section class="border-t border-border bg-bg-secondary py-20">
  <div class="mx-auto max-w-5xl px-4 sm:px-6 lg:px-8">
    <div class="grid gap-8 sm:grid-cols-2 lg:grid-cols-3">

      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <h3 class="font-mono text-lg font-semibold text-accent-amber">How It Works</h3>
        <p class="mt-3 text-sm leading-relaxed text-text-secondary">
          Generate an Ed25519 identity on your machine. Connect directly to peers using WebRTC. Messages travel point-to-point with no servers in between.
        </p>
      </div>

      <div class="rounded-lg border border-border bg-bg-tertiary p-6">
        <h3 class="font-mono text-lg font-semibold text-accent-amber">Features</h3>
        <p class="mt-3 text-sm leading-relaxed text-text-secondary">
          Text channels, voice chat, file sharing, and swarm-based group communication. All encrypted, all local-first, all under your control.
        </p>
      </div>

      <div class="rounded-lg border border-border bg-bg-tertiary p-6 sm:col-span-2 lg:col-span-1">
        <h3 class="font-mono text-lg font-semibold text-accent-amber">Why Aether</h3>
        <p class="mt-3 text-sm leading-relaxed text-text-secondary">
          No accounts to create. No terms of service to accept. No company that can shut you down. Your node is your sovereign territory on the network.
        </p>
      </div>

    </div>
  </div>
</section>
