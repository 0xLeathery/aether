export interface NavItem {
  title: string;
  href: string;
  children?: NavItem[];
}

export const docsNav: NavItem[] = [
  {
    title: 'Getting Started',
    href: '/docs/getting-started'
  },
  {
    title: 'User Guides',
    href: '/docs/guides',
    children: [
      { title: 'Swarms', href: '/docs/guides/swarms' },
      { title: 'Inviting Peers', href: '/docs/guides/inviting-peers' },
      { title: 'Channels', href: '/docs/guides/channels' },
      { title: 'Voice Chat', href: '/docs/guides/voice-chat' },
      { title: 'Moderation', href: '/docs/guides/moderation' }
    ]
  },
  {
    title: 'Architecture',
    href: '/docs/architecture',
    children: [
      { title: 'Networking', href: '/docs/architecture/networking' },
      { title: 'Identity', href: '/docs/architecture/identity' },
      { title: 'CRDTs', href: '/docs/architecture/crdts' },
      { title: 'Voice', href: '/docs/architecture/voice' },
      { title: 'Encryption', href: '/docs/architecture/encryption' }
    ]
  }
];

export function flattenNav(items: NavItem[] = docsNav): { title: string; href: string }[] {
  const result: { title: string; href: string }[] = [];
  for (const item of items) {
    result.push({ title: item.title, href: item.href });
    if (item.children) {
      result.push(...flattenNav(item.children));
    }
  }
  return result;
}
