<script lang="ts">
  import { page } from '$app/state';
  import { docsNav } from '$lib/docs/nav';
  import type { NavItem } from '$lib/docs/nav';

  function isActive(href: string): boolean {
    return page.url.pathname === href;
  }

  function isSectionActive(item: NavItem): boolean {
    if (isActive(item.href)) return true;
    if (item.children) {
      return item.children.some((child) => isActive(child.href));
    }
    return false;
  }
</script>

<nav class="space-y-2">
  {#each docsNav as item}
    <div>
      <a
        href={item.href}
        class="block font-mono text-sm font-bold transition-colors {isActive(item.href)
          ? 'text-accent-green'
          : 'text-text-secondary hover:text-text-primary'}"
      >
        {item.title}
      </a>
      {#if item.children}
        <div class="mt-1 ml-2 space-y-1 border-l border-border pl-3">
          {#each item.children as child}
            <a
              href={child.href}
              class="block font-mono text-sm transition-colors {isActive(child.href)
                ? 'text-accent-green'
                : 'text-text-secondary hover:text-text-primary'}"
            >
              {child.title}
            </a>
          {/each}
        </div>
      {/if}
    </div>
  {/each}
</nav>
