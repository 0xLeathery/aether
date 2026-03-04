<script>
  import { page } from '$app/state';
  import DocNav from '$lib/components/DocNav.svelte';
  import PrevNext from '$lib/components/PrevNext.svelte';

  let { children } = $props();
  let sidebarOpen = $state(false);

  // Close sidebar on navigation (mobile)
  $effect(() => {
    page.url.pathname;
    sidebarOpen = false;
  });
</script>

<div class="flex min-h-[calc(100vh-4rem)]">
  <!-- Mobile sidebar toggle -->
  <button
    class="fixed bottom-4 right-4 z-40 rounded-lg border border-border bg-bg-secondary p-3 text-text-primary shadow-lg md:hidden"
    onclick={() => sidebarOpen = !sidebarOpen}
    aria-label="Toggle documentation sidebar"
  >
    <svg class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
      <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
    </svg>
  </button>

  <!-- Sidebar -->
  <aside
    class="fixed inset-y-0 left-0 z-30 w-64 shrink-0 transform border-r border-border bg-bg-primary pt-16 transition-transform duration-200 md:static md:translate-x-0 md:pt-0"
    class:-translate-x-full={!sidebarOpen}
    class:translate-x-0={sidebarOpen}
  >
    <div class="sticky top-16 p-6">
      <h2 class="mb-4 font-mono text-sm font-semibold uppercase tracking-wider text-text-muted">Documentation</h2>
      <DocNav />
    </div>
  </aside>

  <!-- Mobile overlay backdrop -->
  {#if sidebarOpen}
    <button
      class="fixed inset-0 z-20 bg-black/50 md:hidden"
      onclick={() => sidebarOpen = false}
      aria-label="Close sidebar"
    ></button>
  {/if}

  <!-- Main content area -->
  <div class="mx-auto w-full max-w-3xl flex-1 px-4 py-8 sm:px-6 lg:px-8">
    <div class="prose">
      {@render children()}
    </div>
    <PrevNext />
  </div>
</div>
