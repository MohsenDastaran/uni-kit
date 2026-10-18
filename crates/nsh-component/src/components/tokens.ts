// Design tokens for the NSH components, mirrored from
// `crates/slint-component/ui/theme.slint`, which is mapped from
// `tokens/tokens.json`.
//
// This file is pure data and pure helpers: it imports nothing, so the theme
// generator and the parity check can read it without the NSH runtime. The
// reactive surface is `theme.ts`, which every component imports.
//
// The names are the ones GPUI, Slint and iced use, so a component reads beside
// its counterparts: background, foreground, primary, muted-foreground, border,
// ring, the chart series, and the spacing and radius scales.

export type Tokens = {
  dark: boolean;
  background: string;
  foreground: string;
  surface: string;
  primary: string;
  primaryForeground: string;
  secondary: string;
  secondaryForeground: string;
  muted: string;
  mutedForeground: string;
  link: string;
  inputBackground: string;
  accent: string;
  accentForeground: string;
  destructive: string;
  destructiveForeground: string;
  danger: string;
  border: string;
  input: string;
  ring: string;
  selection: string;
  info: string;
  success: string;
  warning: string;
  chart1: string;
  chart2: string;
  chart3: string;
  chart4: string;
  chart5: string;
  descriptionLabel: string;
  descriptionLabelForeground: string;
  groupBox: string;
  groupBoxForeground: string;
  groupBoxTitleForeground: string;
  popover: string;
  sidebar: string;
  overlay: string;
  shadow: string;
};

// The Slint theme's names, with hyphens as camelCase. `description-label`
// and `group-box` keep the words they have in the authority.
export const LIGHT: Tokens = {
  dark: false,
  background: '#ffffff',
  foreground: '#0a0a0a',
  surface: '#ffffff',
  primary: '#171717',
  primaryForeground: '#fafafa',
  secondary: '#e5e5e5',
  secondaryForeground: '#171717',
  muted: '#f5f5f5',
  mutedForeground: '#737373',
  link: '#2563eb',
  inputBackground: '#ffffff',
  accent: '#f5f5f5',
  accentForeground: '#171717',
  destructive: '#ef4444',
  destructiveForeground: '#fafafa',
  danger: '#ef4444',
  border: '#e5e5e5',
  input: '#e5e5e5',
  ring: '#a3a3a3',
  selection: '#55a0fc',
  info: '#06b6d4',
  success: '#22c55e',
  warning: '#eab308',
  chart1: '#e76e50',
  chart2: '#2a9d90',
  chart3: '#274754',
  chart4: '#e8c468',
  chart5: '#f4a462',
  descriptionLabel: '#ebebeb',
  descriptionLabelForeground: '#171717',
  groupBox: '#f5f5f5',
  groupBoxForeground: '#171717',
  groupBoxTitleForeground: '#737373',
  popover: '#ffffff',
  sidebar: '#fafafa',
  overlay: '#0000004d',
  shadow: '#0000001f',
};

export const DARK: Tokens = {
  dark: true,
  background: '#0a0a0a',
  foreground: '#fafafa',
  surface: '#0a0a0a',
  primary: '#fafafa',
  primaryForeground: '#171717',
  secondary: '#262626',
  secondaryForeground: '#fafafa',
  muted: '#262626',
  mutedForeground: '#a3a3a3',
  link: '#60a5fa',
  inputBackground: '#2f2f2f4d',
  accent: '#262626',
  accentForeground: '#fafafa',
  destructive: '#f87171',
  destructiveForeground: '#dc2626',
  danger: '#f87171',
  border: '#262626',
  input: '#2f2f2f',
  ring: '#737373',
  selection: '#1d4ed8',
  info: '#22d3ee',
  success: '#4ade80',
  warning: '#facc15',
  chart1: '#2662d9',
  chart2: '#2eb88a',
  chart3: '#e88c30',
  chart4: '#af57db',
  chart5: '#e23670',
  descriptionLabel: '#171717',
  descriptionLabelForeground: '#f5f5f5',
  groupBox: '#262626',
  groupBoxForeground: '#fafafa',
  groupBoxTitleForeground: '#a3a3a3',
  popover: '#171717',
  sidebar: '#111111',
  overlay: '#000000a0',
  shadow: '#00000080',
};

// ---------------------------------------------------------------- scales

export const SPACING_XXS = 2;
export const SPACING_XS = 4;
export const SPACING_SM = 8;
export const SPACING_MD = 12;
export const SPACING_LG = 16;
export const SPACING_XL = 24;
export const SPACING_XXL = 32;

export const RADIUS_SM = 3;
export const RADIUS_MD = 6;
export const RADIUS_LG = 8;
export const RADIUS_XL = 12;

export const FONT_SIZE_XS = 12;
export const FONT_SIZE_SM = 14;
export const FONT_SIZE_MD = 16;
export const FONT_SIZE_LG = 18;
export const FONT_SIZE_XL = 20;
export const FONT_SIZE_MONO = 13;

/** The monospace face the components draw code in. */
export const MONO = 'monospace';

// ---------------------------------------------------------------- enums

export type ControlSize = 'xsmall' | 'small' | 'medium' | 'large';
export type Status = 'default' | 'info' | 'success' | 'warning' | 'danger';

/** Padding inside a button, matching GPUI's size_1 through size_4. */
export function controlPadding(size: ControlSize = 'medium'): number {
  switch (size) {
    case 'xsmall':
      return 6;
    case 'small':
      return 8;
    case 'large':
      return 16;
    default:
      return 12;
  }
}

/** A control's height in a row: 20 / 24 / 32 / 40. */
export function controlHeight(size: ControlSize = 'medium'): number {
  switch (size) {
    case 'xsmall':
      return 20;
    case 'small':
      return 24;
    case 'large':
      return 40;
    default:
      return 32;
  }
}

/** Padding inside a text field, which is wider than a button's. */
export function inputPaddingX(size: ControlSize = 'medium'): number {
  switch (size) {
    case 'xsmall':
      return 4;
    case 'small':
      return 8;
    case 'large':
      return 12;
    default:
      return 10;
  }
}

/** A text field's height in a row: 20 / 24 / 32 / 44. */
export function inputHeight(size: ControlSize = 'medium'): number {
  switch (size) {
    case 'xsmall':
      return 20;
    case 'small':
      return 24;
    case 'large':
      return 44;
    default:
      return 32;
  }
}

/** A button label's size: xs is text-xs, small text-sm, the rest text-base. */
export function controlFontSize(size: ControlSize = 'medium'): number {
  switch (size) {
    case 'xsmall':
      return FONT_SIZE_XS;
    case 'small':
      return FONT_SIZE_SM;
    default:
      return FONT_SIZE_MD;
  }
}

/** A text field's size. Medium is text-sm and large is text-base. */
export function inputFontSize(size: ControlSize = 'medium'): number {
  switch (size) {
    case 'xsmall':
      return FONT_SIZE_XS;
    case 'large':
      return FONT_SIZE_MD;
    default:
      return FONT_SIZE_SM;
  }
}

/** A checkbox or radio label. Large is text-lg; buttons stay at text-base. */
export function labelFontSize(size: ControlSize = 'medium'): number {
  return size === 'large' ? FONT_SIZE_LG : controlFontSize(size);
}

/** Icon boxes: 12 / 14 / 16 / 24. */
export function iconSize(size: ControlSize = 'medium'): number {
  switch (size) {
    case 'xsmall':
      return 12;
    case 'small':
      return 14;
    case 'large':
      return 24;
    default:
      return 16;
  }
}

// ---------------------------------------------------------------- colour

type Rgba = { r: number; g: number; b: number; a: number };

function parseHex(value: string): Rgba | null {
  const hex = value.trim();
  if (!hex.startsWith('#')) return null;
  const body = hex.slice(1);
  const nibble = (index: number) => parseInt(body.slice(index, index + 1), 16) * 17;
  const byte = (index: number) => parseInt(body.slice(index, index + 2), 16);
  if (body.length === 3 || body.length === 4) {
    return {
      r: nibble(0),
      g: nibble(1),
      b: nibble(2),
      a: body.length === 4 ? nibble(3) / 255 : 1,
    };
  }
  if (body.length === 6 || body.length === 8) {
    return {
      r: byte(0),
      g: byte(2),
      b: byte(4),
      a: body.length === 8 ? byte(6) / 255 : 1,
    };
  }
  return null;
}

function toHex({ r, g, b, a }: Rgba): string {
  const channel = (value: number) => Math.round(Math.max(0, Math.min(255, value))).toString(16).padStart(2, '0');
  return `#${channel(r)}${channel(g)}${channel(b)}${a < 1 ? channel(a * 255) : ''}`;
}

/** A translucency of a hex colour, as the theme's `with-alpha` does. */
export function alpha(color: string, value: number): string {
  const parsed = parseHex(color);
  if (!parsed) return color;
  return toHex({ ...parsed, a: Math.max(0, Math.min(1, value)) });
}

/** RGB interpolation. The factor weights `first`, as Slint's `mix` does. */
export function mix(first: string, second: string, factor: number): string {
  const a = parseHex(first);
  const b = parseHex(second);
  if (!a || !b) return first;
  const t = Math.max(0, Math.min(1, factor));
  return toHex({
    r: a.r * t + b.r * (1 - t),
    g: a.g * t + b.g * (1 - t),
    b: a.b * t + b.b * (1 - t),
    a: a.a * t + b.a * (1 - t),
  });
}

function rgbToHsl({ r, g, b }: Rgba): { h: number; s: number; l: number } {
  const rn = r / 255;
  const gn = g / 255;
  const bn = b / 255;
  const max = Math.max(rn, gn, bn);
  const min = Math.min(rn, gn, bn);
  const l = (max + min) / 2;
  if (max === min) return { h: 0, s: 0, l };
  const d = max - min;
  const s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
  let h = 0;
  if (max === rn) h = (gn - bn) / d + (gn < bn ? 6 : 0);
  else if (max === gn) h = (bn - rn) / d + 2;
  else h = (rn - gn) / d + 4;
  return { h: h / 6, s, l };
}

function hslToRgb(h: number, s: number, l: number, a: number): Rgba {
  if (s === 0) {
    const value = l * 255;
    return { r: value, g: value, b: value, a };
  }
  const hue = (p: number, q: number, t: number) => {
    let value = t;
    if (value < 0) value += 1;
    if (value > 1) value -= 1;
    if (value < 1 / 6) return p + (q - p) * 6 * value;
    if (value < 1 / 2) return q;
    if (value < 2 / 3) return p + (q - p) * (2 / 3 - value) * 6;
    return p;
  };
  const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
  const p = 2 * l - q;
  return {
    r: hue(p, q, h + 1 / 3) * 255,
    g: hue(p, q, h) * 255,
    b: hue(p, q, h - 1 / 3) * 255,
    a,
  };
}

function shiftLightness(color: string, amount: number): string {
  const parsed = parseHex(color);
  if (!parsed) return color;
  const hsl = rgbToHsl(parsed);
  return toHex(hslToRgb(hsl.h, hsl.s, Math.max(0, Math.min(1, hsl.l + amount)), parsed.a));
}

/** Hover is a lighter fill in dark mode and a darker one in light mode. */
export function hovered(fill: string, dark: boolean): string {
  return shiftLightness(fill, dark ? 0.15 : -0.06);
}

/** Press is a stronger version of the same shift. */
export function pressed(fill: string, dark: boolean): string {
  return shiftLightness(fill, dark ? 0.3 : -0.12);
}

/** The tinted background of a status-coloured surface. */
export function tint(fill: string, dark: boolean): string {
  return alpha(fill, dark ? 0.18 : 0.12);
}

/**
 * An ink that reads on a filled colour: the palette's light foreground on a
 * dark fill, its dark foreground on a light fill. Status chips keep a readable
 * label in either appearance without a fixed white.
 */
export function inkOn(fill: string): string {
  const parsed = parseHex(fill);
  if (!parsed) return DARK.foreground;
  const luminance =
    (0.2126 * parsed.r + 0.7152 * parsed.g + 0.0722 * parsed.b) / 255;
  return luminance > 0.55 ? LIGHT.foreground : DARK.foreground;
}

/** A 24-hour clock label, used by time-field and calendar demos. */
export function pad(value: number, width = 2): string {
  return String(Math.trunc(Math.abs(value))).padStart(width, '0');
}
