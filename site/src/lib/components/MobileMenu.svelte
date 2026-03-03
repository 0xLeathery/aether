<script lang="ts">
  import { navLinks, siteConfig } from '$lib/constants';
  import NavLink from './NavLink.svelte';

  let { open, onclose }: { open: boolean; onclose: () => void } = $props();

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && open) {
      onclose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- Backdrop -->
  <div
    class="fixed inset-0 z-40 bg-bg-primary/80 backdrop-blur-sm md:hidden"
    onclick={onclose}
    onkeydown={(e) => e.key === 'Enter' && onclose()}
    role="button"
    tabindex="-1"
    aria-label="Close menu"
  ></div>

  <!-- Menu Panel -->
  <div
    class="fixed inset-y-0 right-0 z-50 w-full max-w-sm border-l border-border bg-bg-primary p-6 pt-20 md:hidden"
    role="dialog"
    aria-modal="true"
    aria-label="Mobile navigation"
  >
    <!-- Close button -->
    <button
      onclick={onclose}
      class="absolute right-4 top-4 flex h-10 w-10 items-center justify-center rounded-md text-text-secondary transition-colors hover:text-text-primary"
      aria-label="Close menu"
    >
      <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>

    <!-- Navigation links -->
    <div class="flex flex-col gap-4">
      {#each navLinks as link}
        <a
          href={link.href}
          onclick={onclose}
          class="block rounded-md px-3 py-3 text-lg font-medium text-text-secondary transition-colors hover:bg-bg-secondary hover:text-text-primary"
        >
          {link.label}
        </a>
      {/each}
    </div>

    <!-- CTAs -->
    <div class="mt-8 flex flex-col gap-3 border-t border-border pt-8">
      <a
        href="/demo"
        onclick={onclose}
        class="block rounded-md bg-accent-green px-4 py-3 text-center text-base font-semibold text-bg-primary transition-colors hover:bg-accent-green-dim"
      >
        Try Demo
      </a>
      <a
        href={siteConfig.github}
        target="_blank"
        rel="noopener noreferrer"
        class="block rounded-md border border-border-bright px-4 py-3 text-center text-base font-semibold text-text-secondary transition-colors hover:border-text-secondary hover:text-text-primary"
      >
        GitHub
      </a>
    </div>
  </div>
{/if}
