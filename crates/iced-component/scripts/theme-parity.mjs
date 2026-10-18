// Checks the iced theme against the Slint theme it was transcribed from.
//
// `theme.rs` is the one file every component reads, and it was copied by hand
// from 326 lines of `theme.slint`. A wrong hex digit there is a component that is
// slightly the wrong colour in a way no compiler will mention, so this compares
// the two files directly.
//
// The Slint theme writes each colour as `palette ? palette-x : (dark ? A : B)`;
// only the `A`/`B` pair is compared, because the palette overrides exist for the
// website gallery filling in the page's own theme, which iced does not have. The
// few colours computed from other colours are listed as derived rather than
// compared.
//
//   node crates/iced-component/scripts/theme-parity.mjs
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, '..', '..', '..');
const slintTheme = fs.readFileSync(path.join(root, 'crates/slint-component/ui/theme.slint'), 'utf8');
const rustTheme = fs.readFileSync(path.join(here, '..', 'src/components/theme.rs'), 'utf8');

// Colours the Slint theme computes rather than states. Kept in the comparison as
// named exceptions so a new one is noticed.
const DERIVED = new Set(['input_background', 'description_label']);

const toRust = (name) => name.replace(/-/g, '_');

/**
 * Every `out property <color> name: <expression>` in the Slint theme — the
 * resolved tokens, not the `in-out` palette slots the gallery fills from the
 * page's own theme.
 */
function slintColors() {
  const found = new Map();
  // `(?<![-\w])out` so an `in-out property` is not read as a token: those are
  // the gallery's palette overrides, which iced does not have.
  const re = /(?<![-\w])out property <color>\s+([a-z0-9-]+)\s*:\s*([^;]+);/g;
  for (const [, name, expression] of slintTheme.matchAll(re)) {
    found.set(toRust(name), expression.trim());
  }
  return found;
}

/** `dark ? A : B`, or the literal when the expression is not a choice. */
function lightAndDark(expression) {
  const choice = expression.match(/(?:^|\()dark\s*\?\s*(#[0-9a-fA-F]{3,8})\s*:\s*(#[0-9a-fA-F]{3,8})/);
  if (choice) return { light: choice[2], dark: choice[1] };
  const literal = expression.match(/^#[0-9a-fA-F]{3,8}$/);
  if (literal) return { light: literal[0], dark: literal[0] };
  return null;
}

/** The `LIGHT` and `DARK` tables in theme.rs, name to 8-bit channels. */
function rustTable(which) {
  const start = rustTheme.indexOf(`pub const ${which}: Tokens = Tokens {`);
  if (start < 0) throw new Error(`theme.rs has no ${which} table`);
  const body = rustTheme.slice(start, rustTheme.indexOf('\n};', start));
  const found = new Map();
  for (const [, name, kind, args] of body.matchAll(
    /^\s{4}([a-z0-9_]+):\s*(rgb|rgba)\(([^)]*)\)/gm,
  )) {
    // `rgba` is the cases the tokens carry alpha for, which the Slint theme
    // computes; the channel comparison does not apply to them.
    found.set(name, kind === 'rgba'
      ? 'alpha'
      : args.split(',').map((n) => parseInt(n.trim(), 16)));
  }
  return found;
}

const slint = slintColors();
const light = rustTable('LIGHT');
const dark = rustTable('DARK');
const hex = (value) => {
  const digits = value.replace('#', '');
  const full = digits.length <= 4
    ? digits.split('').map((d) => d + d).join('')
    : digits;
  return [0, 2, 4].map((i) => parseInt(full.slice(i, i + 2), 16));
};

const mismatched = [];
const missing = [];
const derived = [];
for (const [name, expression] of slint) {
  if (DERIVED.has(name)) {
    derived.push(name);
    continue;
  }
  const pair = lightAndDark(expression);
  if (!pair) {
    derived.push(name);
    continue;
  }
  if (!light.has(name) || !dark.has(name)) {
    missing.push(name);
    continue;
  }
  for (const [table, key] of [[light, 'light'], [dark, 'dark']]) {
    const mine = table.get(name);
    if (mine === 'alpha') continue;
    const theirs = hex(pair[key]);
    if (mine.join() !== theirs.join()) {
      mismatched.push(`${name} (${key}): slint ${pair[key]} = ${theirs.join()} but theme.rs = ${mine.join()}`);
    }
  }
}

for (const name of light.keys()) {
  if (![...slint.keys()].some((key) => key === name)) missing.push(`${name} (in theme.rs only)`);
}

console.log(`slint colours: ${slint.size}, theme.rs light: ${light.size}, dark: ${dark.size}`);
console.log(`derived or computed, checked by hand: ${derived.length ? derived.join(', ') : 'none'}`);
if (mismatched.length) {
  console.log(`\nMISMATCHED (${mismatched.length}):`);
  for (const line of mismatched) console.log(`  ${line}`);
}
if (missing.length) {
  console.log(`\nNOT IN BOTH (${missing.length}):`);
  for (const line of missing) console.log(`  ${line}`);
}
if (!mismatched.length && !missing.length) console.log('\nthe two themes agree, colour for colour');

process.exit(mismatched.length || missing.length ? 1 : 0);
