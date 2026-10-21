import assert from 'node:assert/strict';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';
import test from 'node:test';

import { SITE_URL } from '../src/lib/site.ts';

// The expected origin comes from the site's own configuration rather than being
// written out again: these assertions used to name `gpui-kit.com`, so they passed
// while `canonical`, `og:url` and `og:image` all pointed at the old domain.
const origin = new URL(SITE_URL).origin;

const dist = new URL('../dist/', import.meta.url);
const read = (path) => readFileSync(new URL(path, dist), 'utf8');

function htmlFiles(directory) {
  return readdirSync(directory).flatMap((name) => {
    const path = join(directory, name);
    const relative = path.slice(dist.pathname.length);
    // The embedded galleries are separate applications, not site pages. They
    // are copied into dist for deployment and have their own titles and roots.
    if (
      /^(?:gallery|slint-gallery|iced-gallery|nsh-gallery|examples|pagefind)(?:\/|$)/.test(
        relative,
      )
    ) {
      return [];
    }
    return statSync(path).isDirectory() ? htmlFiles(path) : path.endsWith('.html') ? [path] : [];
  });
}

function tag(html, pattern) {
  return html.match(pattern)?.[1]?.replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim() ?? '';
}

test('primary landing pages contain server-rendered headings and copy', () => {
  for (const path of ['index.html', 'skills/index.html']) {
    const html = read(path);
    assert.match(html, /<h1[\s>]/, `${path} must contain an H1 before JavaScript runs`);
    assert.ok(tag(html, /<main[^>]*>([\s\S]*?)<\/main>/).length > 100, `${path} must contain meaningful main content`);
  }
});

test('indexable pages have canonical and English alternates', () => {
  for (const path of ['index.html', 'skills/index.html']) {
    const html = read(path);
    assert.ok(html.includes(`<link rel="canonical" href="${origin}/`), `${path} canonical`);
    assert.match(html, /hreflang="en"/, `${path} English alternate`);
    assert.match(html, /hreflang="x-default"/, `${path} default alternate`);
  }
});

test('titles contain the uni-kit brand exactly once', () => {
  for (const file of htmlFiles(dist.pathname)) {
    if (file.endsWith('/404.html') || file.endsWith('/og-template.html')) continue;
    if (/<meta[^>]+http-equiv="refresh"/i.test(readFileSync(file, 'utf8'))) continue;
    const html = readFileSync(file, 'utf8');
    const title = tag(html, /<title>([\s\S]*?)<\/title>/);
    assert.equal((title.match(/uni-kit/g) ?? []).length, 1, `${file} title: ${title}`);
  }
});

test('titles are unique', () => {
  const seen = new Map();
  for (const file of htmlFiles(dist.pathname)) {
    if (file.endsWith('/404.html') || file.endsWith('/og-template.html')) continue;
    if (/<meta[^>]+http-equiv="refresh"/i.test(readFileSync(file, 'utf8'))) continue;
    const title = tag(readFileSync(file, 'utf8'), /<title>([\s\S]*?)<\/title>/);
    assert.ok(!seen.has(title), `${file} duplicates title from ${seen.get(title)}: ${title}`);
    seen.set(title, file);
  }
});

test('every indexable HTML page has exactly one H1', () => {
  for (const file of htmlFiles(dist.pathname)) {
    if (file.endsWith('/404.html') || file.endsWith('/og-template.html')) continue;
    if (/<meta[^>]+http-equiv="refresh"/i.test(readFileSync(file, 'utf8'))) continue;
    const count = (readFileSync(file, 'utf8').match(/<h1[\s>]/g) ?? []).length;
    assert.equal(count, 1, `${file} has ${count} H1 elements`);
  }
});

test('SEO discovery files and structured data are generated', () => {
  assert.ok(existsSync(new URL('robots.txt', dist)));
  assert.ok(existsSync(new URL('sitemap.xml', dist)));
  assert.match(read('index.html'), /<script type="application\/ld\+json">/);
  assert.match(read('component/button/index.html'), /BreadcrumbList/);
});

test('404 is excluded from indexing', () => {
  assert.match(read('404.html'), /<meta name="robots" content="noindex, nofollow"/);
});

test('component pages have independent routes and readable legacy URLs', () => {
  const source = new URL('../component/', import.meta.url);
  for (const name of readdirSync(source).filter(name => name.endsWith('.md'))) {
    const slug = name.slice(0, -3);
    const route = `component${slug === 'index' ? '' : `/${slug}`}`;
    const html = read(`${route}/index.html`);
    assert.match(
      html,
      new RegExp(
        `rel="canonical" href="${origin}/frameworks/(?:gpui|slint|iced|nsh)(?:/[^"']+)?`,
      ),
      route,
    );
    assert.match(html, /hreflang="en"/);
    assert.match(read(`${route}.md`), new RegExp(`^---\nurl: /${route}\\.md\n`));
    assert.match(read(`docs/components/${slug}/index.html`), /http-equiv="refresh"/);
    assert.ok(read(`docs/components/${slug}/index.html`).includes(`url=/${route}`));
  }
  assert.ok(read('docs/components/index.html').includes('url=/component'));
});

test('component pages keep their own sidebar, with no legacy docs links', () => {
  const html = read('component/button/index.html');
  const sidebar = html.match(/<aside class="docs-sidebar"[\s\S]*?<\/aside>/)?.[0] ?? '';
  assert.ok(sidebar.includes('href="/frameworks/nsh/input"'));
  assert.ok(!sidebar.includes('href="/docs/'));
});

test('component links and discovery use canonical routes', () => {
  const sitemap = read('sitemap.xml');
  const index = read('llms.txt');
  const full = read('llms-full.txt');
  assert.ok(sitemap.includes(`${origin}/frameworks/nsh/button`));
  assert.ok(index.includes('/component.md'));
  assert.ok(index.includes('/component/button.md'));
  assert.ok(full.includes('Source: /component/button'));
  assert.ok(read('frameworks/nsh/button/index.html').includes('href="/frameworks/iced/button"'));
  assert.ok(!sitemap.includes('/docs/components'));
  assert.ok(!index.includes('/docs/components'));
  assert.ok(!full.includes('/docs/components'));
});

test('LLM discovery identifies tested consumer recipes', () => {
  const index = read('llms.txt');
  const full = read('llms-full.txt');
  assert.match(index, /Tested consumer recipe: command-control/);
  assert.match(full, /Tested consumer recipe: command-control/);
  assert.match(full, /Source: \/component\/button/);
});

test('primary navigation follows the Kit section order', () => {
  const html = read('index.html');
  const nav = html.match(/<nav class="site-nav"[\s\S]*?<\/nav>/)?.[0] ?? '';
  const links = [...nav.matchAll(/<a\s+href="([^"]+)"\s+class="site-nav__link[^"]*"[^>]*>\s*([^<]+?)\s*<\/a>/g)];
  assert.deepEqual(links.slice(0, 3).map(match => match[2]), ['Component', 'Blocks', 'Starter']);
  assert.deepEqual(links.slice(0, 3).map(match => match[1]), ['/frameworks/nsh', '/frameworks/nsh/blocks', '/frameworks/nsh/starter']);
  assert.doesNotMatch(nav, /site-nav__lang-btn/);
});

test('robots.txt is exact and points at the canonical sitemap', () => {
  assert.equal(
    read('robots.txt'),
    'User-agent: *\nAllow: /\nDisallow: /*.md$\n\nSitemap: https://uni-kit.dastaran.com/sitemap.xml\n',
  );
});

test('sitemap is valid, unique, absolute and has a built page for every URL', () => {
  const xml = read('sitemap.xml');
  assert.ok(xml.startsWith('<?xml version="1.0" encoding="UTF-8"?>'), 'XML declaration');
  assert.match(xml, /<urlset xmlns="http:\/\/www.sitemaps.org\/schemas\/sitemap\/0.9"/);
  assert.ok(xml.trimEnd().endsWith('</urlset>'), 'closing urlset');

  const locs = [...xml.matchAll(/<loc>([^<]+)<\/loc>/g)].map((match) => match[1]);
  assert.ok(locs.length > 100, `only ${locs.length} sitemap URLs`);
  assert.equal(new Set(locs).size, locs.length, 'duplicate sitemap URLs');

  for (const loc of locs) {
    const url = new URL(loc);
    assert.equal(url.origin, origin, loc);
    assert.equal((loc.match(/https?:\/\//g) ?? []).length, 1, `concatenated origin: ${loc}`);
    assert.equal(url.search, '', `query in sitemap: ${loc}`);
    assert.equal(url.hash, '', `hash in sitemap: ${loc}`);
    assert.doesNotMatch(url.pathname, /\.md$/, `markdown in sitemap: ${loc}`);

    const clean = decodeURIComponent(url.pathname).replace(/\/+$/, '');
    const base = join(dist.pathname, clean);
    assert.ok(
      [base, `${base}.html`, join(base, 'index.html')].some(
        (candidate) => existsSync(candidate) && statSync(candidate).isFile(),
      ),
      `sitemap URL has no built page: ${loc}`,
    );
  }

  for (const path of [
    '/frameworks',
    '/frameworks/gpui',
    '/frameworks/slint',
    '/frameworks/iced',
    '/frameworks/nsh',
    '/frameworks/nsh/blocks',
    '/frameworks/nsh/starter',
    '/frameworks/nsh/button',
  ]) {
    assert.ok(locs.includes(`${origin}${path}`), `missing sitemap route: ${path}`);
  }
});

test('framework pages are server-rendered and canonically addressable', () => {
  for (const [file, framework] of [
    ['frameworks/iced/index.html', 'iced'],
    ['frameworks/slint/button/index.html', 'slint'],
    ['frameworks/nsh/button/index.html', 'nsh'],
  ]) {
    const html = read(file);
    assert.ok(html.includes(`data-framework="${framework}"`), `${file} server framework`);
    assert.ok(html.includes('data-framework-fixed="true"'), `${file} fixed framework`);
    assert.match(html, /<link rel="canonical" href="https:\/\/uni-kit\.dastaran\.com\/frameworks\//);
    assert.match(html, /<h1[\s>]/, `${file} H1`);
    assert.doesNotMatch(html, /rel="canonical"[^>]*\?framework=/, `${file} query canonical`);
  }
});

test('deployment config exposes discovery files and redirects www', () => {
  const nginx = readFileSync(new URL('../../deploy/nginx.conf', import.meta.url), 'utf8');
  assert.match(nginx, /server_name www\.uni-kit\.dastaran\.com/);
  assert.match(nginx, /return 301 https:\/\/uni-kit\.dastaran\.com\$request_uri/);
  assert.match(nginx, /location = \/robots\.txt/);
  assert.match(nginx, /location = \/sitemap\.xml/);
  assert.match(nginx, /default_type application\/xml/);
  assert.match(nginx, /Cache-Control "no-cache"/);
});

test('query-based framework links are removed from generated pages', () => {
  for (const file of ['index.html', 'component/kbd/index.html', 'component/button/index.html']) {
    const html = read(file);
    assert.doesNotMatch(html, /href="[^"]*\?framework=/, `${file} still carries a query framework link`);
  }
});

test('nginx redirects old query framework URLs to clean routes', () => {
  const nginx = readFileSync(new URL('../../deploy/nginx.conf', import.meta.url), 'utf8');
  assert.match(nginx, /\$arg_framework/);
  assert.match(nginx, /\/frameworks\/\$arg_framework/);
});
