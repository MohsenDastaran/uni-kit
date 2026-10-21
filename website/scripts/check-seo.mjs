#!/usr/bin/env node
// Validate the built SEO surface. Runs after `astro build`, so a missing or
// malformed discovery file fails the image instead of reaching production.
import { existsSync, readFileSync, statSync } from 'node:fs';
import { join, resolve } from 'node:path';

const root = resolve(process.cwd(), process.env.SITE_OUT_DIR ?? 'dist');
const origin = 'https://uni-kit.dastaran.com';
const errors = [];

const read = (name) => readFileSync(join(root, name), 'utf8');
const assert = (condition, message) => {
  if (!condition) errors.push(message);
};

const robots = read('robots.txt');
assert(
  robots ===
    'User-agent: *\nAllow: /\nDisallow: /*.md$\n\nSitemap: https://uni-kit.dastaran.com/sitemap.xml\n',
  'robots.txt body does not match the canonical rules',
);
assert(robots.includes(`Sitemap: ${origin}/sitemap.xml`), 'robots.txt sitemap URL is not canonical');

const sitemap = read('sitemap.xml');
assert(sitemap.startsWith('<?xml version="1.0" encoding="UTF-8"?>'), 'sitemap.xml has no XML declaration');
assert(sitemap.includes('<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"'), 'sitemap.xml has no urlset');
assert(sitemap.trimEnd().endsWith('</urlset>'), 'sitemap.xml is truncated');

const locs = [...sitemap.matchAll(/<loc>([^<]+)<\/loc>/g)].map((match) => match[1]);
assert(locs.length > 80, `sitemap has only ${locs.length} URLs`);
assert(new Set(locs).size === locs.length, 'sitemap contains duplicate URLs');

const exists = (pathname) => {
  const clean = decodeURIComponent(pathname).replace(/\/+$/, '');
  const base = join(root, clean);
  return [base, `${base}.html`, join(base, 'index.html')].some(
    (candidate) => existsSync(candidate) && statSync(candidate).isFile(),
  );
};

for (const loc of locs) {
  let url;
  try {
    url = new URL(loc);
  } catch {
    errors.push(`invalid URL: ${loc}`);
    continue;
  }
  assert(url.origin === origin, `wrong origin: ${loc}`);
  assert((loc.match(/https?:\/\//g) ?? []).length === 1, `concatenated origin: ${loc}`);
  assert(!url.search && !url.hash, `query/hash in sitemap: ${loc}`);
  assert(!url.pathname.endsWith('.md'), `markdown URL in sitemap: ${loc}`);
  assert(exists(url.pathname), `sitemap URL has no built page: ${loc}`);
}

for (const path of [
  '/',
  '/blocks',
  '/starter',
  '/skills',
  '/component',
  '/component/button',
  '/frameworks',
  '/frameworks/gpui',
  '/frameworks/slint',
  '/frameworks/iced',
  '/frameworks/nsh',
  '/frameworks/nsh/button',
]) {
  assert(locs.includes(`${origin}${path}`), `expected sitemap URL missing: ${path}`);
}

if (errors.length > 0) {
  console.error(errors.map((error) => `  ✗ ${error}`).join('\n'));
  process.exit(1);
}
console.log(`  ok: ${locs.length} canonical sitemap URLs and robots.txt`);
