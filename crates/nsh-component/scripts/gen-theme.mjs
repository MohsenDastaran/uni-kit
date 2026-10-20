#!/usr/bin/env node
// Generate the NSH mobile-component theme from the shared design tokens.
//
//   node crates/nsh-component/scripts/gen-theme.mjs
//   node crates/nsh-component/scripts/gen-theme.mjs --check
//
// `nsh/ui/mobile` reads a gpui-kit theme file through `nsh.json`'s `theme` key.
// The file is generated, not hand-written, so the mobile components and the
// hand-written components read one palette. `theme-parity.mjs` verifies it
// against `theme.slint` and `tokens.ts`.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, '../../..');
const tokensPath = path.join(here, '../src/components/tokens.ts');
const target = path.join(here, '../src/components/theme.json');
const check = process.argv.includes('--check');

const tokens = await import(pathToFileURL(tokensPath).href);
const { LIGHT, DARK, RADIUS_MD, RADIUS_LG, FONT_SIZE_MD, alpha, pressed } = tokens;

const mobile = (palette, mode) => ({
  name: mode === 'light' ? 'Default Light' : 'Default Dark',
  mode,
  radius: RADIUS_MD,
  'radius.lg': RADIUS_LG,
  'radius.xl': 16,
  'font.size': FONT_SIZE_MD,
  colors: {
    background: palette.background,
    foreground: palette.foreground,
    border: palette.border,
    'primary.background': palette.primary,
    'primary.active.background': pressed(palette.primary, palette.dark),
    'primary.foreground': palette.primaryForeground,
    'secondary.background': palette.secondary,
    'secondary.active.background': pressed(palette.secondary, palette.dark),
    'secondary.foreground': palette.secondaryForeground,
    'danger.background': palette.danger,
    'danger.active.background': pressed(palette.danger, palette.dark),
    'danger.foreground': palette.destructiveForeground,
    'accent.background': palette.accent,
    'accent.foreground': palette.accentForeground,
    'muted.background': palette.muted,
    'muted.foreground': palette.mutedForeground,
    'group_box.background': palette.groupBox,
    'group_box.foreground': palette.groupBoxForeground,
    'group_box.title.foreground': palette.groupBoxTitleForeground,
    'input.border': palette.input,
    ring: palette.ring,
    'list.active.background': alpha(palette.selection, palette.dark ? 0.2 : 0.15),
    overlay: palette.overlay,
    popover: palette.popover,
    sidebar: palette.sidebar,
  },
});

const theme = {
  name: 'uni-kit Default',
  author: 'uni-kit',
  url: 'https://uni-kit.dastaran.com',
  themes: [mobile(LIGHT, 'light'), mobile(DARK, 'dark')],
};
const file = `${JSON.stringify(theme, null, 2)}\n`;

if (check) {
  const current = fs.existsSync(target) ? fs.readFileSync(target, 'utf8') : '';
  if (current !== file) {
    console.error('gen-theme: src/components/theme.json is stale; run node crates/nsh-component/scripts/gen-theme.mjs');
    process.exit(1);
  }
  console.log('gen-theme: theme.json matches the shared tokens');
} else {
  fs.writeFileSync(target, file);
  console.log(`gen-theme: wrote ${path.relative(root, target)}`);
}
