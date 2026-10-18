// Checks the NSH token table and mobile theme against the Slint authority.
//
// `theme.slint` is the framework that has every component, and its Theme global
// is the mapping of `tokens/tokens.json`. `tokens.ts` and `theme.json` are the
// NSH transcription; a wrong hex digit there is a control that is slightly the
// wrong colour in a way no compiler mentions. This compares the files directly.
//
//   node crates/nsh-component/scripts/theme-parity.mjs
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, '..', '..', '..');
const slintPath = path.join(root, 'crates/slint-component/ui/theme.slint');
const tokensPath = path.join(here, '../src/components/tokens.ts');
const themeJsonPath = path.join(here, '../src/components/theme.json');

const slint = fs.readFileSync(slintPath, 'utf8');
const tokens = await import(pathToFileURL(tokensPath).href);
const mobile = JSON.parse(fs.readFileSync(themeJsonPath, 'utf8'));

const toCamel = (name) => name.replace(/-([a-z0-9])/g, (_, letter) => letter.toUpperCase());

// Colours the Slint theme computes from another colour rather than stating as a
// pair. They are listed so a new one is noticed, and checked where the
// relationship is simple.
const DERIVED = new Set([
  'inputBackground', // input.with-alpha(...) in dark, background in light
  'danger', // destructive
  'descriptionLabel', // background.mix(border, 0.8) in light
]);

function slintColors() {
  const found = new Map();
  const re = /(?<![-\w])out property <color>\s+([a-z0-9-]+)\s*:\s*([^;]+);/g;
  for (const [, name, expression] of slint.matchAll(re)) {
    found.set(toCamel(name), expression.trim());
  }
  return found;
}

function slintLengths() {
  const found = new Map();
  const re = /(?<![-\w])out property <length>\s+([a-z0-9-]+)\s*:\s*([0-9.]+)px;/g;
  for (const [, name, value] of slint.matchAll(re)) {
    found.set(toCamel(name), Number(value));
  }
  return found;
}

function lightAndDark(expression) {
  const choice = expression.match(/(?:^|\()dark\s*\?\s*(#[0-9a-fA-F]{3,8})\s*:\s*(#[0-9a-fA-F]{3,8})/);
  if (choice) return { light: choice[2], dark: choice[1] };
  const literal = expression.match(/^#[0-9a-fA-F]{3,8}$/);
  if (literal) return { light: literal[0], dark: literal[0] };
  return null;
}

const normalize = (value) => String(value).toLowerCase();
const slintMap = slintColors();
const lengthMap = slintLengths();
const mismatched = [];
const missing = [];
const derived = [];

for (const [name, expression] of slintMap) {
  if (DERIVED.has(name)) {
    derived.push(name);
    continue;
  }
  const pair = lightAndDark(expression);
  if (!pair) {
    derived.push(name);
    continue;
  }
  if (!(name in tokens.LIGHT) || !(name in tokens.DARK)) {
    missing.push(name);
    continue;
  }
  for (const [table, key] of [[tokens.LIGHT, 'light'], [tokens.DARK, 'dark']]) {
    if (normalize(table[name]) !== normalize(pair[key])) {
      mismatched.push(`${name} (${key}): slint ${pair[key]} but tokens.ts ${table[name]}`);
    }
  }
}

// The lengths are literal in both files.
const lengthNames = {
  radiusSm: tokens.RADIUS_SM,
  radiusMd: tokens.RADIUS_MD,
  radiusLg: tokens.RADIUS_LG,
  radiusXl: tokens.RADIUS_XL,
  fontSizeXs: tokens.FONT_SIZE_XS,
  fontSizeSm: tokens.FONT_SIZE_SM,
  fontSizeMd: tokens.FONT_SIZE_MD,
  fontSizeLg: tokens.FONT_SIZE_LG,
  fontSizeXl: tokens.FONT_SIZE_XL,
  fontSizeMono: tokens.FONT_SIZE_MONO,
};
for (const [name, value] of Object.entries(lengthNames)) {
  if (!lengthMap.has(name)) {
    missing.push(`${name} (in theme.slint only)`);
    continue;
  }
  if (lengthMap.get(name) !== value) {
    mismatched.push(`${name}: slint ${lengthMap.get(name)}px but tokens.ts ${value}`);
  }
}

// The mobile theme carries the same palette into nsh/ui/mobile. Compare the
// mapped keys rather than the whole file: NSH ignores the keys it does not use.
for (const [mode, palette] of [['light', tokens.LIGHT], ['dark', tokens.DARK]]) {
  const entry = mobile.themes.find((theme) => theme.mode === mode);
  if (!entry) {
    missing.push(`theme.json ${mode}`);
    continue;
  }
  const expected = {
    background: palette.background,
    foreground: palette.foreground,
    border: palette.border,
    'primary.background': palette.primary,
    'primary.foreground': palette.primaryForeground,
    'secondary.background': palette.secondary,
    'secondary.foreground': palette.secondaryForeground,
    'danger.background': palette.danger,
    'danger.foreground': palette.destructiveForeground,
    'accent.background': palette.accent,
    'muted.background': palette.muted,
    'muted.foreground': palette.mutedForeground,
    'group_box.background': palette.groupBox,
    'group_box.foreground': palette.groupBoxForeground,
    'input.border': palette.input,
    ring: palette.ring,
    overlay: palette.overlay,
  };
  for (const [key, value] of Object.entries(expected)) {
    if (normalize(entry.colors?.[key]) !== normalize(value)) {
      mismatched.push(`theme.json ${mode} ${key}: ${entry.colors?.[key]} but tokens.ts ${value}`);
    }
  }
}

console.log(`slint colours: ${slintMap.size}, tokens.ts tables: ${Object.keys(tokens.LIGHT).length}`);
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
