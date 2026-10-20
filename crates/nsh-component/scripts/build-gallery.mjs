#!/usr/bin/env node
// Build the NSH gallery the website serves at `/nsh-gallery`.
//
//   node crates/nsh-component/scripts/build-gallery.mjs
//   STORY=button node crates/nsh-component/scripts/build-gallery.mjs
//
// Each `examples/<module>.tsx` is a whole app: `nsh build` compiles it to
// QuickJS bytecode, and this script writes the bundle beside the shared runtime
// and the player page. The website middleware maps `/nsh-gallery` to `www/dist`.
import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const crate = path.resolve(here, '..');
const examples = path.join(crate, 'examples');
const www = path.join(crate, 'www');
const dist = path.join(www, 'dist');
const tmp = path.join(crate, '.tmp', 'gallery');

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
  throw new Error(
    'nsh was not found. Install the pinned CLI with `script/install-nsh.sh`, ' +
      'or set NSH_BIN to its path.',
  );
}

function runtimeDir(nsh) {
  const explicit = process.env.NSH_RUNTIME;
  const candidates = [];
  if (explicit) {
    candidates.push(path.join(explicit, 'web'), explicit);
  }
  candidates.push(path.join(path.dirname(nsh), '..', 'runtime', 'web'));
  candidates.push(path.join(process.env.HOME ?? '', '.nsh', 'runtime', 'web'));
  for (const candidate of candidates) {
    if (fs.existsSync(path.join(candidate, 'nsh.js')) && fs.existsSync(path.join(candidate, 'nsh.wasm'))) {
      return candidate;
    }
  }
  throw new Error(
    'the NSH web runtime was not found. Keep the distribution whole (bin/ and runtime/), ' +
      'or set NSH_RUNTIME to its runtime directory.',
  );
}

function copyFile(from, to) {
  fs.mkdirSync(path.dirname(to), { recursive: true });
  fs.copyFileSync(from, to);
}

function mergeAssets(from, to) {
  if (!fs.existsSync(from)) return;
  for (const entry of fs.readdirSync(from, { withFileTypes: true })) {
    const source = path.join(from, entry.name);
    const target = path.join(to, entry.name);
    if (entry.isDirectory()) mergeAssets(source, target);
    else copyFile(source, target);
  }
}

const nsh = findNsh();
const runtime = runtimeDir(nsh);
const wanted = process.env.STORY
  ? new Set(process.env.STORY.split(',').map((value) => value.trim()).filter(Boolean))
  : null;

if (!fs.existsSync(examples)) throw new Error(`${examples} does not exist`);

const stories = fs
  .readdirSync(examples)
  .filter((file) => file.endsWith('.tsx'))
  .map((file) => file.slice(0, -4))
  .sort()
  .filter((module) => !wanted || wanted.has(module) || wanted.has(module.replace(/_/g, '-')));

if (stories.length === 0) throw new Error('no examples matched STORY');

fs.rmSync(dist, { recursive: true, force: true });
fs.mkdirSync(path.join(dist, 'demos'), { recursive: true });
fs.mkdirSync(tmp, { recursive: true });

copyFile(path.join(runtime, 'nsh.js'), path.join(dist, 'nsh.js'));
copyFile(path.join(runtime, 'nsh.wasm'), path.join(dist, 'nsh.wasm'));
copyFile(path.join(www, 'index.html'), path.join(dist, 'index.html'));

for (const module of stories) {
  const entry = path.join(examples, `${module}.tsx`);
  const out = path.join(tmp, module);
  fs.rmSync(out, { recursive: true, force: true });
  const result = spawnSync(nsh, ['build', '--release', '-o', out, entry], {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  if (result.status !== 0) {
    const detail = (result.stderr || result.stdout || '').trim();
    throw new Error(`nsh build failed for ${module}${detail ? `\n${detail}` : ''}`);
  }
  const bundle = path.join(out, 'bundle.nshc');
  if (!fs.existsSync(bundle)) throw new Error(`nsh build wrote no bundle.nshc for ${module}`);
  copyFile(bundle, path.join(dist, 'demos', `${module.replace(/_/g, '-')}.nshc`));
  mergeAssets(path.join(out, 'assets'), path.join(dist, 'assets'));
  process.stdout.write(`  ${module}\n`);
}

console.log(
  `nsh-gallery: ${stories.length} demo${stories.length === 1 ? '' : 's'} -> ${path.relative(crate, dist)}`,
);
