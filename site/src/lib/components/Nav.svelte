<script lang="ts">
  import { navLinks, siteConfig } from '$lib/constants';
  import NavLink from './NavLink.svelte';
  import MobileMenu from './MobileMenu.svelte';

  const internalLinks = navLinks.filter(l => !l.external);
  const githubLink = navLinks.find(l => l.label === 'GitHub');

  let visible = $state(true);
  let lastScrollY = $state(0);
  let mobileMenuOpen = $state(false);

  function handleScroll() {
    const currentScrollY = window.scrollY;
    visible = currentScrollY < lastScrollY || currentScrollY < 50;
    lastScrollY = currentScrollY;
  }

  function toggleMobileMenu() {
    mobileMenuOpen = !mobileMenuOpen;
  }

  function closeMobileMenu() {
    mobileMenuOpen = false;
  }
</script>

<svelte:window onscroll={handleScroll} />

<nav
  class="fixed top-0 z-50 w-full border-b border-border bg-bg-primary/95 backdrop-blur-sm transition-transform duration-300 {visible
    ? 'translate-y-0'
    : '-translate-y-full'}"
>
  <div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
    <!-- Logo -->
    <a href="/" class="flex items-center gap-2 font-mono text-lg font-bold text-accent-green">
      <span class="text-xl" aria-hidden="true">&gt;_</span>
      <span>{siteConfig.name}</span>
    </a>

    <!-- Desktop Navigation -->
    <div class="hidden items-center gap-6 md:flex">
      {#each internalLinks as link}
        <NavLink href={link.href}>
          {link.label}
        </NavLink>
      {/each}
    </div>

    <!-- Desktop CTAs -->
    <div class="hidden items-center gap-3 md:flex">
      <a
        href="/demo"
        class="rounded-md bg-accent-green px-4 py-2 text-sm font-semibold text-bg-primary transition-colors duration-200 hover:bg-accent-green-dim"
      >
        Try Demo
      </a>
      <a
        href={githubLink?.href}
        target="_blank"
        rel="noopener noreferrer"
        class="rounded-md border border-border-bright px-4 py-2 text-sm font-semibold text-text-secondary transition-colors duration-200 hover:border-text-secondary hover:text-text-primary"
      >
        GitHub
      </a>
    </div>

    <!-- Mobile Hamburger -->
    <button
      onclick={toggleMobileMenu}
      class="flex items-center justify-center md:hidden"
      aria-label={mobileMenuOpen ? 'Close menu' : 'Open menu'}
      aria-expanded={mobileMenuOpen}
    >
      <svg
        class="h-6 w-6 text-text-primary"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
        aria-hidden="true"
      >
        {#if mobileMenuOpen}
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        {:else}
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        {/if}
      </svg>
    </button>
  </div>
</nav>

<MobileMenu open={mobileMenuOpen} onclose={closeMobileMenu} />
