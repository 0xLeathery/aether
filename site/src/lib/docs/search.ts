import { Index } from 'flexsearch';

export interface SearchEntry {
  title: string;
  href: string;
  content: string;
}

let index: Index | null = null;
let entries: SearchEntry[] = [];

export function createSearchIndex(docs: SearchEntry[]): void {
  entries = docs;
  index = new Index({ tokenize: 'forward' });

  for (let i = 0; i < docs.length; i++) {
    index.add(i, docs[i].title + ' ' + docs[i].content);
  }
}

export function searchDocs(query: string): SearchEntry[] {
  if (!index || !query.trim()) return [];

  const ids = index.search(query, { limit: 10 });
  return ids.map((id) => entries[id as number]);
}
