import { json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const prerender = true;

function stripMarkdown(raw: string): { title: string; content: string } {
  let text = raw;

  // Remove frontmatter
  text = text.replace(/^---[\s\S]*?---\n*/m, '');

  // Remove svelte:head blocks
  text = text.replace(/<svelte:head>[\s\S]*?<\/svelte:head>\n*/g, '');

  // Extract title from first heading
  const titleMatch = text.match(/^#\s+(.+)$/m);
  const title = titleMatch ? titleMatch[1].trim() : '';

  // Remove headings markers but keep text
  text = text.replace(/^#{1,6}\s+/gm, '');

  // Remove link syntax [text](url) -> text
  text = text.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1');

  // Remove image syntax ![alt](url)
  text = text.replace(/!\[([^\]]*)\]\([^)]+\)/g, '$1');

  // Remove HTML/SVG tags
  text = text.replace(/<[^>]+>/g, '');

  // Remove formatting characters
  text = text.replace(/[*_`~]/g, '');

  // Remove blockquote markers
  text = text.replace(/^>\s*/gm, '');

  // Remove horizontal rules
  text = text.replace(/^---+$/gm, '');

  // Remove list markers
  text = text.replace(/^[\s]*[-*+]\s+/gm, '');
  text = text.replace(/^[\s]*\d+\.\s+/gm, '');

  // Collapse whitespace
  text = text.replace(/\n{2,}/g, ' ');
  text = text.replace(/\s+/g, ' ');
  text = text.trim();

  return { title, content: text.slice(0, 500) };
}

export const GET: RequestHandler = async () => {
  const modules = import.meta.glob('/src/routes/docs/**/+page.md', {
    eager: true,
    query: '?raw',
    import: 'default'
  }) as Record<string, string>;

  const entries: { title: string; href: string; content: string }[] = [];

  for (const [path, raw] of Object.entries(modules)) {
    // Convert file path to route: /src/routes/docs/foo/+page.md -> /docs/foo
    const href = path
      .replace('/src/routes', '')
      .replace('/+page.md', '') || '/docs';

    // Skip the docs index page
    if (href === '/docs') continue;

    const { title, content } = stripMarkdown(raw);

    entries.push({
      title: title || href.split('/').pop() || 'Untitled',
      href,
      content
    });
  }

  return json(entries);
};
