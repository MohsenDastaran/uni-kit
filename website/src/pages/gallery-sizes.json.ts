import type { APIRoute } from 'astro';
import { readdirSync, statSync } from 'node:fs';
import { join, resolve } from 'node:path';

/**
 * What WebAssembly each gallery makes the browser fetch, and which files it
 * comes in.
 *
 * A preview is tens of megabytes, so the loader that waits for one says how
 * much, and — because the names are known here — fetches it with a real
 * percentage instead of a spinner. The site serves every `.wasm` as immutable
 * for a year, so the frame that follows reads the same bytes from the cache
 * rather than paying for them twice.
 *
 * The keys are the frame's path without its query: `/slint-gallery/pages/dock`,
 * `/gallery`, `/examples/base`. Nothing is measured at runtime; the galleries
 * are built before the site, and a checkout without them answers `{}` and the
 * interface leaves the size unsaid.
 */
const MODULE = /\.wasm$/;

type Gallery = { total: number; files: string[] };

function modulesIn(directory: string): Gallery {
  try {
    const files = readdirSync(directory).filter((name) => MODULE.test(name));
    return {
      files,
      total: files.reduce((total, name) => total + statSync(join(directory, name)).size, 0),
    };
  } catch {
    return { files: [], total: 0 };
  }
}

function pagesIn(pages: string): Record<string, Gallery> {
  const found: Record<string, Gallery> = {};
  try {
    for (const slug of readdirSync(pages)) {
      try {
        if (!statSync(join(pages, slug)).isDirectory()) continue;
      } catch {
        continue;
      }
      const gallery = modulesIn(join(pages, slug));
      if (gallery.total > 0) found[`slint-gallery/pages/${slug}`] = gallery;
    }
  } catch {
    // No built pages: nothing to say.
  }
  return found;
}

export const GET: APIRoute = () => {
  const galleries: Record<string, Gallery> = {
    gallery: modulesIn(resolve('../crates/story-web/www/dist')),
    'examples/base': modulesIn(resolve('../crates/base/examples/wasm/www/dist')),
    ...pagesIn(resolve('../crates/slint-component/www/dist/pages')),
  };

  return new Response(JSON.stringify(galleries), {
    headers: { 'Content-Type': 'application/json; charset=utf-8' },
  });
};
