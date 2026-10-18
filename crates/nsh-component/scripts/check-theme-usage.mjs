#!/usr/bin/env node
// Guards the port's theme contract: component chrome comes from the shared
// tokens, and a demo must not evaluate a token before the reactive render.
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

const root = new URL('..', import.meta.url).pathname;
const components = join(root, 'src/components');
const demos = join(root, 'src/demos');
const failures = [];

// A colour picker's swatch values are data, not chrome. Everything else in a
// component must come from theme.ts / tokens.ts or from props.
const dataColours = new Set(['color_picker.tsx']);
const colour = /#[0-9a-fA-F]{3,8}\b|\brgba?\(|\bhsla?\(/;

for (const file of readdirSync(components).filter((name) => name.endsWith('.tsx'))) {
  const source = readFileSync(join(components, file), 'utf8');
  if (!dataColours.has(file) && colour.test(source)) {
    failures.push(`${file}: fixed colour; use tokens.* instead`);
  }
  if (/^\s*(?:export\s+)?const\s+[A-Za-z_$][\w$]*\s*=\s*tokens\./m.test(source)) {
    failures.push(`${file}: a token is read into a constant outside JSX`);
  }
}

// Helpers in these two files are components in their own right: their token
// reads happen when they render, not when the module is evaluated.
const helperDemos = new Set(['theme.tsx', 'ui.tsx']);
for (const file of readdirSync(demos).filter((name) => name.endsWith('.tsx'))) {
  const source = readFileSync(join(demos, file), 'utf8');
  const demo = source.indexOf('export function Demo');
  if (demo < 0) continue;
  if (!helperDemos.has(file) && /(?:tokens\.|alpha\(tokens\.|inkOn\()/.test(source.slice(0, demo))) {
    failures.push(`${file}: a token is read before Demo(); the value will freeze at the light palette`);
  }
}

const theme = JSON.parse(readFileSync(join(root, 'src/components/theme.json'), 'utf8'));
const expected = [
  'background',
  'foreground',
  'border',
  'primary.background',
  'primary.active.background',
  'primary.foreground',
  'secondary.background',
  'secondary.active.background',
  'secondary.foreground',
  'danger.background',
  'danger.active.background',
  'danger.foreground',
  'accent.background',
  'muted.background',
  'muted.foreground',
  'group_box.background',
  'group_box.foreground',
  'input.border',
  'ring',
  'list.active.background',
  'overlay',
];
for (const entry of theme.themes) {
  for (const key of ['radius', 'radius.lg', 'font.size']) {
    if (!(key in entry)) failures.push(`theme.json ${entry.mode}: missing ${key}`);
  }
  for (const key of expected) {
    if (!(key in entry.colors)) failures.push(`theme.json ${entry.mode}: missing ${key}`);
  }
}

if (failures.length) {
  for (const failure of failures) console.error(`   fail: ${failure}`);
  process.exit(1);
}
console.log(`   ok: theme usage: ${readdirSync(components).filter((name) => name.endsWith('.tsx')).length} components use tokens, theme.json covers the mobile keys`);
