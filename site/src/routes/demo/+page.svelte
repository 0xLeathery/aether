<svelte:head>
  <title>Demo - Aether</title>
</svelte:head>

<div class="mx-auto max-w-4xl px-4 py-16 sm:px-6 lg:px-8">
  <h1 class="font-mono text-3xl font-bold text-accent-green sm:text-4xl">Try Aether in Your Browser</h1>
  <p class="mt-4 max-w-2xl text-lg leading-relaxed text-text-secondary">
    Experience peer-to-peer communication without downloading anything. This interactive demo connects you directly to another person using WebRTC.
  </p>

  <!-- Chat window mockup -->
  <div class="mt-10 overflow-hidden rounded-lg border border-border">
    <div class="border-b border-border bg-bg-tertiary px-4 py-3">
      <div class="flex items-center gap-2">
        <span class="h-3 w-3 rounded-full bg-accent-green/30"></span>
        <span class="font-mono text-sm text-text-muted">peer-to-peer channel</span>
      </div>
    </div>
    <div class="flex min-h-[300px] items-center justify-center bg-bg-secondary p-8 sm:min-h-[400px]">
      <div class="text-center">
        <p class="font-mono text-sm text-text-muted">
          The interactive demo is being built.
        </p>
        <p class="mt-2 text-sm text-text-muted">
          WebRTC peer connections will appear here, allowing real-time P2P communication directly in your browser.
        </p>
      </div>
    </div>
  </div>

  <div class="mt-8 text-center">
    <p class="text-sm text-text-secondary">
      For the complete experience, download the full desktop app.
    </p>
    <a
      href="/download"
      class="mt-4 inline-flex items-center rounded-lg border border-accent-green px-5 py-2.5 font-mono text-sm font-semibold text-accent-green transition-colors hover:bg-accent-green-bg"
    >
      Download Aether
    </a>
  </div>
</div>
