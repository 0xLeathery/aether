<script lang="ts">
  import { page } from '$app/state';
  import type { Snippet } from 'svelte';

  let { href, children }: { href: string; children: Snippet } = $props();

  let isExternal = $derived(href.startsWith('http'));

  let isActive = $derived(
    href === '/'
      ? page.url.pathname === '/'
      : page.url.pathname.startsWith(href)
  );
</script>

{#if isExternal}
  <a
    {href}
    target="_blank"
    rel="noopener noreferrer"
    class="relative py-1 text-text-secondary transition-colors duration-200 hover:text-text-primary"
  >
    {@render children()}
  </a>
{:else}
  <a
    {href}
    class="relative py-1 transition-colors duration-200 {isActive
      ? 'text-accent-green'
      : 'text-text-secondary hover:text-text-primary'}"
  >
    {@render children()}
    {#if isActive}
      <span
        class="absolute bottom-0 left-0 h-0.5 w-full bg-accent-green"
      ></span>
    {/if}
  </a>
{/if}
