<script lang="ts">
  import { onMount } from 'svelte';
  import { createSearchIndex, searchDocs, type SearchEntry } from '$lib/docs/search';

  let query = $state('');
  let results = $state<SearchEntry[]>([]);
  let showResults = $state(false);
  let wrapper: HTMLDivElement;

  const filteredResults = $derived.by(() => {
    if (query.length < 2) return [];
    return searchDocs(query);
  });

  $effect(() => {
    results = filteredResults;
    showResults = filteredResults.length > 0 || query.length >= 2;
  });

  onMount(async () => {
    try {
      const res = await fetch('/docs/search.json');
      const data: SearchEntry[] = await res.json();
      createSearchIndex(data);
    } catch (e) {
      console.error('Failed to load search index:', e);
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      showResults = false;
      query = '';
    }
  }

  function handleClickOutside(e: MouseEvent) {
    if (wrapper && !wrapper.contains(e.target as Node)) {
      showResults = false;
    }
  }

  function selectResult() {
    query = '';
    showResults = false;
  }

  $effect(() => {
    document.addEventListener('click', handleClickOutside);
    return () => document.removeEventListener('click', handleClickOutside);
  });
</script>

<div class="relative mb-4" bind:this={wrapper}>
  <input
    type="text"
    placeholder="Search docs..."
    bind:value={query}
    onkeydown={handleKeydown}
    class="w-full rounded-lg border border-border bg-bg-tertiary px-3 py-2 font-mono text-sm text-text-primary placeholder:text-text-muted focus:border-accent-green focus:outline-none"
  />

  {#if showResults}
    <div class="absolute z-50 mt-1 w-full rounded-lg border border-border bg-bg-secondary shadow-lg">
      {#if results.length > 0}
        {#each results as result}
          <a
            href={result.href}
            class="block px-3 py-2 hover:bg-bg-tertiary"
            onclick={selectResult}
          >
            <div class="text-sm font-semibold text-text-primary">{result.title}</div>
            <div class="truncate text-xs text-text-muted">{result.content.slice(0, 80)}</div>
          </a>
        {/each}
      {:else}
        <div class="px-3 py-2 text-sm text-text-muted">No results found</div>
      {/if}
    </div>
  {/if}
</div>
