<script lang="ts">
  import { page } from '$app/state';
  import { flattenNav } from '$lib/docs/nav';

  const flat = flattenNav();

  let currentIndex = $derived(flat.findIndex((item) => item.href === page.url.pathname));
  let prev = $derived(currentIndex > 0 ? flat[currentIndex - 1] : null);
  let next = $derived(currentIndex >= 0 && currentIndex < flat.length - 1 ? flat[currentIndex + 1] : null);
</script>

<div class="mt-12 flex items-center justify-between border-t border-border pt-6">
  {#if prev}
    <a href={prev.href} class="font-mono text-sm text-text-secondary transition-colors hover:text-accent-green">
      &larr; {prev.title}
    </a>
  {:else}
    <div></div>
  {/if}
  {#if next}
    <a href={next.href} class="font-mono text-sm text-text-secondary transition-colors hover:text-accent-green">
      {next.title} &rarr;
    </a>
  {:else}
    <div></div>
  {/if}
</div>
