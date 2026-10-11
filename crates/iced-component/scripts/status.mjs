// The iced integration's checklist, generated from the repository rather than
// maintained by hand.
//
// The inventory is the Slint registry: it is the framework that lists every
// component as copyable source, so its keys are the authoritative set. For each
// one this reports whether an iced implementation exists, whether it is wired
// into the registry, and whether it has a usage example — the three things that
// have to be true before a component counts as done.
//
// Writes the table into docs/iced-integration.md between the markers, and with
// `--check` exits non-zero when something is missing, so CI can hold the line.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, '..', '..', '..');
// The component sources sit where the CLI installs them, `src/components/`, so
// the directory an application ends up with is the one this crate is written in.
const ui = path.join(root, 'crates/iced-component/src/components');
const examples = path.join(root, 'crates/iced-component/examples');
const slintUi = path.join(root, 'crates/slint-component/ui');
const pages = path.join(root, 'website/component');
const doc = path.join(root, 'docs/iced-integration.md');

const registry = JSON.parse(fs.readFileSync(path.join(root, 'registry/components.json'), 'utf8'));
const inventory = Object.keys(registry.slint.components).sort();
const wired = new Set(Object.keys(registry.iced?.components ?? {}));

const exists = (file) => fs.existsSync(file);
const read = (file) => (exists(file) ? fs.readFileSync(file, 'utf8') : '');
const lines = (file) => (exists(file) ? read(file).trim().split('\n').length : 0);

/**
 * Whether a file is a component rather than the shape of one.
 *
 * A stub compiles, is declared, and passes every check that only asks whether a
 * file exists — which is how eight five-line files once counted as done work. A
 * component has to build an `Element` and expose something to call.
 */
function implemented(file, kind) {
  const text = read(file);
  if (!text) return false;
  if (/Implementation pending|not implemented|todo!\(|unimplemented!\(/i.test(text)) return false;
  if (kind === 'example') {
    // An example is an application: it either builds one itself or runs the demo
    // the library holds for it. The demo moved into `src/demos/` so the browser
    // gallery can show the very same code, which leaves the example a thin
    // `main` that calls it — still an application, just not one written here.
    return (
      /fn main\s*\(/.test(text) &&
      (/iced::(application|run)\s*\(/.test(text) || /demos::[a-z_]+::run\s*\(/.test(text))
    );
  }
  // A component builds an element. `theme.rs` is the shared token file: it is
  // the iced counterpart of the Slint theme component and builds no widget, so
  // the element test does not apply to it.
  const buildsElement = /Element/.test(text) || file.endsWith('theme.rs');
  return /pub fn/.test(text) && buildsElement;
}

// The registry key is the site's slug and may hold a hyphen; the file and the
// module cannot, so the key is the module with underscores back to hyphens.
const moduleOf = (name) => name.replace(/-/g, '_');

const rows = inventory.map((name) => {
  const source = path.join(ui, `${moduleOf(name)}.rs`);
  const example = path.join(examples, `${moduleOf(name)}.rs`);
  const slint = path.join(slintUi, `${name}.slint`);
  const page = path.join(pages, `${name}.md`);
  const size = lines(source);
  return {
    name,
    module: moduleOf(name),
    slint: exists(slint),
    gpui: exists(page),
    source: implemented(source, 'component'),
    lines: size,
    example: implemented(example, 'example'),
    wired: wired.has(name),
  };
});

const done = rows.filter((r) => r.source && r.example && r.wired).length;
const missing = rows.filter((r) => !(r.source && r.example && r.wired)).map((r) => r.name);

const tick = (value) => (value ? 'yes' : '—');
const stubbed = rows.filter((r) => !r.source && r.lines > 0).map((r) => r.name);
const table = [
  '| Component | iced file | GPUI ref | Slint ref | iced source | lines | example | registry |',
  '| --- | --- | --- | --- | --- | --- | --- | --- |',
  ...rows.map(
    (r) =>
      `| \`${r.name}\` | \`${r.module}.rs\` | ${tick(r.gpui)} | ${tick(r.slint)} | ${tick(r.source)} | ${r.lines || '—'} | ${tick(r.example)} | ${tick(r.wired)} |`,
  ),
  '',
  `**${done} / ${rows.length}** components have an iced source, an example and a registry entry.`,
  '',
  ...(stubbed.length
    ? [`**Stubs, which do not count:** ${stubbed.map((n) => `\`${n}\``).join(', ')}.`]
    : []),
  ...(missing.length
    ? [`Still to do: ${missing.map((n) => `\`${n}\``).join(', ')}.`]
    : ['Every component in the Slint registry has an iced implementation.']),
].join('\n');

if (process.argv.includes('--write')) {
  const text = fs.readFileSync(doc, 'utf8');
  const start = '<!-- iced-checklist:start -->';
  const end = '<!-- iced-checklist:end -->';
  const from = text.indexOf(start);
  const to = text.indexOf(end);
  if (from < 0 || to < 0) throw new Error(`docs/iced-integration.md is missing its checklist markers`);
  fs.writeFileSync(doc, `${text.slice(0, from + start.length)}\n${table}\n${text.slice(to)}`);
  console.log(`wrote ${done}/${rows.length} into docs/iced-integration.md`);
} else {
  console.log(table);
}

if (process.argv.includes('--check') && missing.length > 0) {
  console.error(`\niced is missing ${missing.length} of ${rows.length}: ${missing.join(', ')}`);
  process.exit(1);
}
