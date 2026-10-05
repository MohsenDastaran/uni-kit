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

        res.setHeader('Cache-Control', 'no-store');
        res.setHeader(
          'Content-Type',
          CONTENT_TYPES[extname(file)] ?? 'application/octet-stream'
        );
        createReadStream(file).pipe(res);
      });
    },
  };
}
