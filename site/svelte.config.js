import adapter from '@sveltejs/adapter-vercel';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { mdsvex } from 'mdsvex';
import rehypeSlug from 'rehype-slug';
import rehypeAutolinkHeadings from 'rehype-autolink-headings';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  extensions: ['.svelte', '.svx', '.md'],

  preprocess: [
    vitePreprocess(),
    mdsvex({
      extensions: ['.svx', '.md'],
      rehypePlugins: [
        rehypeSlug,
        [rehypeAutolinkHeadings, { behavior: 'wrap' }]
      ]
    })
  ],

  kit: {
    adapter: adapter({
      runtime: 'nodejs22.x'
    }),
    prerender: {
      handleHttpError: ({ path, message }) => {
        // Doc pages are created incrementally across Phase 14 plans.
        // Warn instead of fail for /docs/* routes that don't exist yet.
        if (path.startsWith('/docs/')) {
          console.warn(`[prerender] Skipping unbuilt doc page: ${path}`);
          return;
        }
        throw new Error(message);
      }
    }
  }
};

export default config;
