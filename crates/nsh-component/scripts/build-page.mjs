#!/usr/bin/env node
// Write the player page and the shared NSH runtime without compiling a demo.
//
//   node crates/nsh-component/scripts/build-page.mjs
//
// This is the fast loop for working on the player page and the theme: the
// bytecode in `www/dist/demos` is whatever the last `build-gallery.mjs` left.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const crate = path.resolve(here, '..');
const www = path.join(crate, 'www');
const dist = path.join(www, 'dist');

const executables = (name) => (process.platform === 'win32' ? [`${name}.exe`, name] : [name]);

function findNsh() {
  const candidates = [];
  if (process.env.NSH_BIN) candidates.push(process.env.NSH_BIN);
  for (const name of executables('nsh')) {
    if (process.env.PATH) {
      for (const dir of process.env.PATH.split(path.delimiter)) {
        if (dir) candidates.push(path.join(dir, name));
      }
    }
    candidates.push(path.join(process.env.HOME ?? '', '.local', 'bin', name));
    candidates.push(path.join(process.env.HOME ?? '', '.nsh', 'bin', name));
  }
  for (const candidate of candidates) {
    try {
      if (candidate && fs.statSync(candidate).isFile()) return fs.realpathSync(candidate);
    } catch {
      // Keep looking.
    }
  }
  throw new Error('nsh was not found; run script/install-nsh.sh or set NSH_BIN.');
}

function runtimeDir(nsh) {
  const candidates = [
    ...(process.env.NSH_RUNTIME
      ? [path.join(process.env.NSH_RUNTIME, 'web'), process.env.NSH_RUNTIME]
      : []),
    path.join(path.dirname(nsh), '..', 'runtime', 'web'),
    path.join(process.env.HOME ?? '', '.nsh', 'runtime', 'web'),
  ];
  for (const candidate of candidates) {
    if (fs.existsSync(path.join(candidate, 'nsh.js')) && fs.existsSync(path.join(candidate, 'nsh.wasm'))) {
      return candidate;
    }
  }
  throw new Error('the NSH web runtime was not found; keep the distribution whole or set NSH_RUNTIME.');
}

const nsh = findNsh();
const runtime = runtimeDir(nsh);
fs.mkdirSync(dist, { recursive: true });
for (const file of ['nsh.js', 'nsh.wasm', 'index.html']) {
  fs.copyFileSync(
    path.join(file === 'index.html' ? www : runtime, file),
    path.join(dist, file),
  );
}
console.log(`nsh-gallery page: ${path.relative(crate, dist)}`);
