<script>
  import { siteConfig } from '$lib/constants';
</script>

<svelte:head>
  <title>Docs - Aether</title>
</svelte:head>

<div class="py-8">
  <h1 class="font-mono text-3xl font-bold text-accent-green sm:text-4xl">Documentation</h1>
  <p class="mt-4 max-w-2xl text-lg leading-relaxed text-text-secondary">
    Comprehensive guides for getting started with Aether, understanding the protocol, and contributing to the project.
  </p>

  <div class="mt-10 grid gap-4 sm:grid-cols-2">
    <a href="/docs/getting-started" class="group block rounded-lg border border-border bg-bg-secondary p-5 no-underline transition-colors hover:border-border-bright">
      <h3 class="font-mono text-base font-semibold text-text-primary group-hover:text-accent-green">Getting Started</h3>
      <p class="mt-2 text-sm leading-relaxed text-text-muted">
        Install Aether, launch your node, and create your Ed25519 identity. Connect to your first peer in minutes.
      </p>
    </a>

    <a href="/docs/guides" class="group block rounded-lg border border-border bg-bg-secondary p-5 no-underline transition-colors hover:border-border-bright">
      <h3 class="font-mono text-base font-semibold text-text-primary group-hover:text-accent-green">User Guides</h3>
      <p class="mt-2 text-sm leading-relaxed text-text-muted">
        Swarms, channels, voice chat, file sharing, and moderation. Learn how to use every feature of Aether.
      </p>
    </a>

    <a href="/docs/architecture" class="group block rounded-lg border border-border bg-bg-secondary p-5 no-underline transition-colors hover:border-border-bright">
      <h3 class="font-mono text-base font-semibold text-text-primary group-hover:text-accent-green">Architecture</h3>
      <p class="mt-2 text-sm leading-relaxed text-text-muted">
        Protocol design, WebRTC networking, CRDT-based state synchronization, and the sovereign identity model.
      </p>
    </a>

    <a href="{siteConfig.github}" target="_blank" rel="noopener noreferrer" class="group block rounded-lg border border-border bg-bg-secondary p-5 no-underline transition-colors hover:border-border-bright">
      <h3 class="font-mono text-base font-semibold text-text-primary group-hover:text-accent-green">Contributing</h3>
      <p class="mt-2 text-sm leading-relaxed text-text-muted">
        How to contribute to Aether. Code conventions, PR workflow, testing guidelines, and the project roadmap.
      </p>
    </a>
  </div>
</div>
