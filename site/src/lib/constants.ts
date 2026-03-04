interface NavLink {
  href: string;
  label: string;
  external?: boolean;
}

export const navLinks: NavLink[] = [
  { href: '/', label: 'Home' },
  { href: '/docs', label: 'Docs' },
  { href: '/download', label: 'Download' },
  { href: '/demo', label: 'Demo' },
  { href: 'https://github.com/0xLeathery/aether', label: 'GitHub', external: true },
];

export const siteConfig = {
  name: 'Aether',
  tagline: 'Sovereign P2P Communication',
  github: 'https://github.com/0xLeathery/aether',
  version: '0.1.0',
  license: 'Elastic License 2.0',
};
