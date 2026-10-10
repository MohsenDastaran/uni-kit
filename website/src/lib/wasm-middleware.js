import { createReadStream, existsSync, statSync } from 'node:fs';
import { extname, join, resolve } from 'node:path';

const CONTENT_TYPES = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.wasm': 'application/wasm',
  '.css': 'text/css; charset=utf-8',
  '.svg': 'image/svg+xml',
};

export function wasmExamplesDevServer(base) {
  const prefix = base.replace(/\/$/, '');
  const roots = new Map([
    [`${prefix}/examples/base`, resolve('../crates/base/examples/wasm/www/dist')],
    [`${prefix}/gallery`, resolve('../crates/story-web/www/dist')],
    [`${prefix}/slint-gallery`, resolve('../crates/slint-component/www/dist')],
  ]);

  return {
    name: 'wasm-examples-dev-server',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const pathname = new URL(req.url ?? '/', 'http://localhost').pathname;
        const entry = [...roots].find(
          ([prefix]) => pathname === prefix || pathname.startsWith(`${prefix}/`)
        );
        if (!entry) return next();

        const [prefix, root] = entry;
        const relative = pathname.slice(prefix.length).replace(/^\/+/, '');
        let file = join(root, relative || 'index.html');

        // A folder request answers with its own index.html. The site sets
        // `trailingSlash: 'never'`, so a per-page build is asked for without the
        // slash: `/slint-gallery/pages/<slug>`.
        if (existsSync(file) && statSync(file).isDirectory()) {
          file = join(file, 'index.html');
        }

        // A miss under `pages/` must not fall back to the top-level `index.html`:
        // that page would request `slint_component.js` from the page folder and
        // be handed HTML in its place. Report the missing build instead.
        const page = /^pages\/([^/]+)(?:\/|$)/.exec(relative);
        if (page && !existsSync(file)) {
          res.statusCode = 503;
          res.setHeader('Content-Type', 'text/plain; charset=utf-8');
          res.end(
            `The Slint example for "${page[1]}" is not built.\n` +
              `Run: make build:wasm-slint-dev SLUGS=${page[1]}\n`,
          );
          return;
        }

        if (!existsSync(file) || !statSync(file).isFile()) {
          file = join(root, 'index.html');
        }
        if (!existsSync(file)) {
          res.statusCode = 503;
          res.end('WASM example is not built. Run its Makefile build target first.');
          return;
        }

        const ext = extname(file);
        const stat = statSync(file);
        res.setHeader('Content-Type', CONTENT_TYPES[ext] ?? 'application/octet-stream');
        if (ext === '.wasm') {
          // The preview loader fetches the module itself and counts the bytes,
          // then mounts a frame that must read the same bytes back. `no-cache`
          // lets the browser keep them while revalidating against this ETag on
          // every use: a rebuilt module is never served stale, and the second
          // read costs no transfer. `no-store`, which the other files keep,
          // would turn every preview into a double download.
          res.setHeader('Cache-Control', 'no-cache');
          res.setHeader('Content-Length', stat.size);
          const etag = `"${stat.size}-${Math.floor(stat.mtimeMs)}"`;
          res.setHeader('ETag', etag);
          res.setHeader('Vary', 'Accept-Encoding');
          if (req.method === 'HEAD') {
            res.end();
            return;
          }
          if (req.headers['if-none-match'] === etag) {
            res.statusCode = 304;
            res.end();
            return;
          }
        } else {
          res.setHeader('Cache-Control', 'no-store');
        }
        createReadStream(file).pipe(res);
      });
    },
  };
}
