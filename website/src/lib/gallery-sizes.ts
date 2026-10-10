import { readdirSync, statSync } from 'node:fs';
import { join, resolve } from 'node:path';

/**
 * What WebAssembly each gallery makes the browser fetch, and which files it
 * comes in — measured at build time, so a page can print it in the HTML it
 * serves rather than discovering it afterwards.
 *
 * The previews are tens of megabytes, and the wait for one deserves a number:
 * the loader says how big the download is, and, because the file names are
 * known, fetches the bytes itself with a real percentage while the frame that
 * follows reads the same bytes from the cache. That reuse is what the site's
 * server must allow; where it does not, the loader stays indeterminate rather
 * than inventing a figure.
 *
 * The gallery's build writes a pre-compressed `.wasm.gz` beside each module
 * (see `scripts/precompress-wasm.mjs`), and the server serves that file for the
 * `.wasm` URL. `total` is therefore the compressed size when one exists —
 * the bytes a reader actually downloads — and the raw size otherwise.
 *
 * `decoded` is the module's own size, and it is what a download's progress is
 * measured against. The two differ whenever the server compresses: a fetch
 * reports `Content-Length` for the compressed transfer, but hands the reader
 * decompressed chunks, so a count of those chunks against that length runs to
 * the end of its range about a third of the way through the transfer — the bar
 * that sat at 99% and never moved.
 *
 * Keys are the frame's path without its query: `gallery`,
 * `slint-gallery/pages/dock`, `examples/base`. A checkout without built
 * galleries answers `{}` and the interface leaves the size unsaid.
 */
export type Gallery = { total: number; files: string[]; decoded: number };

const MODULE = /\.wasm$/;

const sizeOnWire = (file: string): number => {
  for (const variant of [file + '.gz', file + '.br']) {
    try {
      return statSync(variant).size;
    } catch {
      // No pre-compressed variant: the raw module is what goes over the wire.
    }
  }
  return statSync(file).size;
};

function modulesIn(directory: string): Gallery {
  try {
    const files = readdirSync(directory).filter((name) => MODULE.test(name));
    return {
      files,
      total: files.reduce((total, name) => total + sizeOnWire(join(directory, name)), 0),
      decoded: files.reduce(
        (total, name) => total + statSync(join(directory, name)).size,
        0,
      ),
    };
  } catch {
    return { files: [], total: 0, decoded: 0 };
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

/** All the galleries' sizes, keyed by the path their frames ask for. */
export function gallerySizes(repo = resolve(process.cwd(), '..')): Record<string, Gallery> {
  return {
    gallery: modulesIn(join(repo, 'crates/story-web/www/dist')),
    'examples/base': modulesIn(join(repo, 'crates/base/examples/wasm/www/dist')),
    ...pagesIn(join(repo, 'crates/slint-component/www/dist/pages')),
  };
}
