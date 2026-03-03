<script>
  let { children } = $props();
</script>

<div class="flex min-h-[calc(100vh-4rem)]">
  <!-- Sidebar shell - Phase 14 fills this with documentation navigation -->
  <aside class="hidden w-64 shrink-0 border-r border-border md:block">
    <nav class="sticky top-16 p-4">
      <p class="text-sm text-text-muted">Documentation navigation will appear here.</p>
    </nav>
  </aside>

  <!-- Main content area -->
  <div class="flex-1">
    {@render children()}
  </div>
</div>
