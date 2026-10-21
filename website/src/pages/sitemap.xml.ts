import type { APIRoute } from 'astro';
import { getCollection } from 'astro:content';
import { SITE_URL } from '../lib/site';
import {
  FRAMEWORK_KEYS,
  availableFrameworks,
  frameworkLandingPath,
} from '../lib/frameworks';

/** Public static routes. Keep in step with `website/src/pages`. */
const STATIC_PATHS = ['/', '/frameworks', '/skills'] as const;

function escapeXml(value: string) {
  return value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&apos;');
}

/** Always build from the canonical origin; never concatenate origins. */
export function absoluteUrl(path: string): string {
  const pathname = path.replace(/^\/+/, '');
  return new URL(pathname ? `/${pathname}` : '/', `${SITE_URL}/`).href;
}

function urlEntry(path: string) {
  const loc = escapeXml(absoluteUrl(path));
  return `  <url>\n    <loc>${loc}</loc>\n  </url>`;
}

export const GET: APIRoute = async () => {
  const paths = new Set<string>(STATIC_PATHS);

  const entries = await getCollection('component');
  for (const entry of entries) {
    const slug = entry.id.replace(/\.md$/, '');
    if (slug === 'index') continue;
    for (const framework of availableFrameworks(slug, entry.data.example)) {
      paths.add(`${frameworkLandingPath(framework)}/${slug}`);
    }
  }

  for (const framework of FRAMEWORK_KEYS) {
    paths.add(frameworkLandingPath(framework));
    paths.add(`${frameworkLandingPath(framework)}/blocks`);
    paths.add(`${frameworkLandingPath(framework)}/starter`);
  }

  const urls = [...paths]
    .map((path) => path.replace(/\/{2,}/g, '/'))
    .filter((path) => !path.endsWith('.md'))
    .sort()
    .map(urlEntry)
    .join('\n');

  const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls}
</urlset>
`;
  return new Response(xml, {
    headers: {
      'Content-Type': 'application/xml; charset=utf-8',
      'Cache-Control': 'no-cache',
    },
  });
};
