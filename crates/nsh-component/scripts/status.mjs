// The NSH integration checklist, generated from the repository rather than
// maintained by hand.
//
// The inventory is the Slint registry: it is the framework that lists every
// component as copyable source, so its keys are the authoritative set. For each
// one this reports whether an NSH implementation exists, whether it has a usage
// example and whether it is wired into the registry.
//
//   node crates/nsh-component/scripts/status.mjs
//   node crates/nsh-component/scripts/status.mjs --check
//   node crates/nsh-component/scripts/status.mjs --write
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, '..', '..', '..');
const components = path.join(root, 'crates/nsh-component/src/components');
const examples = path.join(root, 'crates/nsh-component/examples');
const slintUi = path.join(root, 'crates/slint-component/ui');
const pages = path.join(root, 'website/component');
const doc = path.join(root, 'docs/nsh-integration.md');

const registry = JSON.parse(fs.readFileSync(path.join(root, 'registry/components.json'), 'utf8'));
const inventory = Object.keys(registry.slint.components).sort();
const moduleOf = (name) => name.replace(/-/g, '_');
const wired = new Set(Object.keys(registry.nsh?.components ?? {}).map(moduleOf));

const exists = (file) => fs.existsSync(file);
const read = (file) => (exists(file) ? fs.readFileSync(file, 'utf8') : '');
const lines = (file) => (exists(file) ? read(file).trim().split('\n').length : 0);

/**
 * Whether a file is a component rather than the shape of one. `theme.ts` is the
 * shared token file: it is the NSH counterpart of the Slint theme component and
 * builds no widget, so the JSX test does not apply to it.
 */
function implemented(file, kind) {
  const text = read(file);
  if (!text) return false;
  if (/Implementation pending|not implemented|todo!\(|unimplemented!\(/i.test(text)) return false;
  if (kind === 'example') return /\bmount\s*\(/.test(text);
  if (file.endsWith('theme.ts')) return /export/.test(text) && /tokens/.test(text);
  return /export\s+(function|const)/.test(text) && /</.test(text);
}

const rows = inventory.map((name) => {
  const module = moduleOf(name);
  const source = path.join(components, name === 'theme' ? 'theme.ts' : `${module}.tsx`);
  const example = path.join(examples, `${module}.tsx`);
  const slint = path.join(slintUi, `${name}.slint`);
  const page = path.join(pages, `${name}.md`);
  return {
    name,
    module,
    slint: exists(slint),
    gpui: exists(page),
    source: implemented(source, 'component'),
    lines: lines(source),
    example: implemented(example, 'example'),
    wired: wired.has(module),
  };
});

const done = rows.filter((row) => row.source && row.example && row.wired).length;
const missing = rows.filter((row) => !(row.source && row.example && row.wired)).map((row) => row.name);
const tick = (value) => (value ? 'yes' : '—');
const stubbed = rows.filter((row) => !row.source && row.lines > 0).map((row) => row.name);

const table = [
  '| Component | NSH file | GPUI ref | Slint ref | NSH source | lines | example | registry |',
  '| --- | --- | --- | --- | --- | --- | --- | --- |',
  ...rows.map(
    (row) =>
      `| \`${row.name}\` | \`${row.name === 'theme' ? 'theme.ts' : `${row.module}.tsx`}\` | ${tick(row.gpui)} | ${tick(row.slint)} | ${tick(row.source)} | ${row.lines || '—'} | ${tick(row.example)} | ${tick(row.wired)} |`,
  ),
  '',
  `**${done} / ${rows.length}** components have an NSH source, an example and a registry entry.`,
  '',
  ...(stubbed.length ? [`**Stubs, which do not count:** ${stubbed.map((name) => `\`${name}\``).join(', ')}.`] : []),
  ...(missing.length
    ? [`Still to do: ${missing.map((name) => `\`${name}\``).join(', ')}.`]
    : ['Every component in the Slint registry has an NSH implementation.']),
].join('\n');

if (process.argv.includes('--write')) {
  const text = fs.readFileSync(doc, 'utf8');
  const start = '<!-- nsh-checklist:start -->';
  const end = '<!-- nsh-checklist:end -->';
  const from = text.indexOf(start);
  const to = text.indexOf(end);
  if (from < 0 || to < 0) throw new Error('docs/nsh-integration.md is missing its checklist markers');
  fs.writeFileSync(doc, `${text.slice(0, from + start.length)}\n${table}\n${text.slice(to)}`);
  console.log(`wrote ${done}/${rows.length} into docs/nsh-integration.md`);
} else {
  console.log(table);
}

if (process.argv.includes('--check') && missing.length > 0) {
  console.error(`\nNSH is missing ${missing.length} of ${rows.length}: ${missing.join(', ')}`);
  process.exit(1);
}
