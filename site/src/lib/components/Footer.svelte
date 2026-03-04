<script lang="ts">
  import { navLinks, siteConfig } from '$lib/constants';

  const internalLinks = navLinks.filter(l => !l.external);
  const githubLink = navLinks.find(l => l.label === 'GitHub');
</script>

<footer class="mt-auto border-t border-border bg-bg-secondary">
  <div class="mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
    <div class="grid grid-cols-1 gap-8 md:grid-cols-3">
      <!-- Brand -->
      <div>
        <a href="/" class="font-mono text-lg font-bold text-accent-green">
          <span aria-hidden="true">&gt;_</span> {siteConfig.name}
        </a>
        <p class="mt-2 text-sm text-text-secondary">
          {siteConfig.tagline}
        </p>
        <p class="mt-1 text-sm text-text-muted">
          Own your identity. Own your data.
        </p>
      </div>

      <!-- Navigation -->
      <div>
        <h3 class="font-mono text-sm font-semibold uppercase tracking-wider text-text-muted">
          Navigate
        </h3>
        <ul class="mt-3 flex flex-col gap-2">
          {#each internalLinks as link}
            <li>
              <a
                href={link.href}
                class="text-sm text-text-secondary transition-colors duration-200 hover:text-text-primary"
              >
                {link.label}
              </a>
            </li>
          {/each}
        </ul>
      </div>

      <!-- Open Source -->
      <div>
        <h3 class="font-mono text-sm font-semibold uppercase tracking-wider text-text-muted">
          Open Source
        </h3>
        <ul class="mt-3 flex flex-col gap-2">
          <li>
            <span class="inline-flex items-center gap-2 rounded-full border border-border-bright px-2.5 py-0.5 text-xs font-medium text-text-secondary">
              {siteConfig.license} License
            </span>
          </li>
          <li>
            <a
              href={githubLink?.href}
              target="_blank"
              rel="noopener noreferrer"
              class="text-sm text-text-secondary transition-colors duration-200 hover:text-text-primary"
            >
              View Source on GitHub
            </a>
          </li>
          <li>
            <a
              href="{siteConfig.github}/issues"
              target="_blank"
              rel="noopener noreferrer"
              class="text-sm text-text-secondary transition-colors duration-200 hover:text-text-primary"
            >
              Report an Issue
            </a>
          </li>
        </ul>
      </div>
    </div>

    <!-- Bottom bar -->
    <div class="mt-8 flex flex-col items-center justify-between gap-2 border-t border-border pt-8 text-xs text-text-muted sm:flex-row">
      <span>v{siteConfig.version}</span>
      <span class="font-mono">Built for sovereignty</span>
      <span>Source-available software</span>
    </div>
  </div>
</footer>
