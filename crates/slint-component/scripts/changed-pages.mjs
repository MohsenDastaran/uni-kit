#!/usr/bin/env node
// Prints the gallery pages whose sources changed since they were last built.
//
// A page's stamp lives beside its build output, in `www/dist/pages/<slug>/.stamp`,
// and holds a hash of every file the page reaches through imports. When a
// component changes, only the pages that import it are printed, so `make dev`
// rebuilds a handful of pages instead of all of them.
//
// Printing nothing means nothing changed and the caller can skip the build.
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join, normalize, relative, sep } from 'node:path';

const crateDir = join(import.meta.dirname, '..');
const ui = join(crateDir, 'ui');
const distPages = join(crateDir, 'www', 'dist', 'pages');

// Slint resolves an import against the importing file's own folder, so the
// lookup below does the same rather than assuming every `.slint` sits at `ui/`.
function resolveImport(fromDir, spec) {
  return normalize(join(fromDir, spec));
}

function closure(entry) {
  const seen = new Set();
  const pending = [entry];
  while (pending.length) {
    const file = pending.pop();
    if (seen.has(file)) continue;
    if (!existsSync(file)) continue;
    seen.add(file);
    const text = readFileSync(file, 'utf8');
    const dir = join(file, '..');
    for (const m of text.matchAll(/from\s+"([^"]+\.slint)"/g)) {
      pending.push(resolveImport(dir, m[1]));
    }
  }
  return [...seen].sort();
}

function hashOf(paths) {
  const hash = createHash('sha256');
  for (const path of paths) {
    hash.update(relative(ui, path));
    hash.update('\0');
    hash.update(readFileSync(path));
    hash.update('\0');
  }
  return hash.digest('hex');
}

const examplesDir = join(ui, 'examples');

// The hash of everything a page is built from, so a stamp and a later check
// cannot disagree about what counts.
function pageHash(slug) {
  const files = closure(join(examplesDir, `${slug}.slint`));
  // An icon is a name, not an import, so collect the names the closure uses and
  // fold the matching SVGs in. A renamed icon must invalidate the page too.
  const icons = new Set();
  for (const path of files) {
    const source = readFileSync(path, 'utf8');
    for (const m of source.matchAll(/(?:icon|name):\s*"([^"]+)"/g)) {
      const svg = join(ui, 'icons', `${m[1]}.svg`);
      if (existsSync(svg)) icons.add(svg);
    }
  }
  return hashOf([...files, ...[...icons].sort()]);
}

function pages() {
  return readdirSync(examplesDir)
    .filter((f) => f.endsWith('.slint'))
    .filter((f) => /^export component [A-Za-z0-9_]*Example/m.test(readFileSync(join(examplesDir, f), 'utf8')))
    .map((f) => f.replace(/\.slint$/, ''))
    .sort();
}

const [command, argument] = process.argv.slice(2);

if (command === '--write') {
  // Called by build.sh once a page's output exists, so the next check is clean.
  if (!argument) {
    console.error('changed-pages: --write needs a slug');
    process.exit(2);
  }
  mkdirSync(join(distPages, argument), { recursive: true });
  writeFileSync(join(distPages, argument, '.stamp'), `${pageHash(argument)}\n`);
  process.exit(0);
}

const changed = [];
for (const slug of pages()) {
  const stampPath = join(distPages, slug, '.stamp');
  const previous = existsSync(stampPath) ? readFileSync(stampPath, 'utf8').trim() : '';
  if (pageHash(slug) !== previous) changed.push(slug);
}

process.stdout.write(changed.join(' '));
