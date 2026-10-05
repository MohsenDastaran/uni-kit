// Vite reads the source theme files at build time. Adding a JSON file or a
// variant to themes/ automatically makes it available on the website.
type Variant = {
  name: string;
  mode: 'light' | 'dark';
  colors: Record<string, string>;
  highlight?: Record<string, unknown>;
};
type ThemeFile = { name: string; themes: Variant[] };

const files = import.meta.glob<ThemeFile>('../../../themes/*.json', { eager: true, import: 'default' });
const slug = (value: string) => value.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '');

export const themeSources = Object.entries(files).map(([path, data]) => ({
  source: path.split('/').pop()!.replace(/\.json$/, ''),
  data,
}));

export const themes = Object.entries(files)
  .flatMap(([path, file]) => file.themes.map((variant) => ({
    id: `${path.split('/').pop()!.replace(/\.json$/, '')}-${slug(variant.name)}`,
    source: path.split('/').pop()!.replace(/\.json$/, ''),
    family: file.name,
    name: variant.name,
    mode: variant.mode,
    colors: variant.colors,
    highlight: variant.highlight,
  })))
  .sort((a, b) => a.name.localeCompare(b.name));

// The band of base colours a palette tile previews, in spectrum order. Every
// theme file carries this family, so the strip is a stable six across themes.
// Dots are the source files' separator, so every token is a flat lookup.
export const swatchTokens = [
  'base.red',
  'base.magenta',
  'base.yellow',
  'base.green',
  'base.cyan',
  'base.blue',
] as const;

// Only literal colours reach CSS. A theme may legitimately hold a gradient or a
// named reference for a token, and neither is paintable.
const paintable = (value: string | undefined): value is string =>
  typeof value === 'string' && (value.startsWith('#') || /^(rgb|hsl|oklch|color)\(/.test(value));

export function themeSwatches(colors: Record<string, string> | undefined): string[] {
  if (!colors) return [];
  return swatchTokens.flatMap((token) => (paintable(colors[token]) ? [colors[token]] : []));
}

// A tile previews a theme the reader is not currently in, so it cannot borrow
// the page's tokens: every colour it paints comes from that theme's own file.
// The stage walks tokens in order and takes the first literal one, which keeps
// a theme with no `popover` from losing its colour entirely.
const surfaceKeys = [
  'card.background',
  'background',
  'popover.background',
  'muted.background',
  'secondary.background',
] as const;

const inkKeys = ['card.foreground', 'foreground', 'popover.foreground', 'secondary.foreground'] as const;

function firstPaintable(colors: Record<string, string>, keys: readonly string[]): string | undefined {
  return keys.map((key) => colors[key]).find(paintable);
}

// The light/dark category a tile is drawn in, which is the mode declared by the
// theme file rather than anything measured from the page.
export function themeSurface(colors: Record<string, string> | undefined, mode: string) {
  const stage = colors ? firstPaintable(colors, surfaceKeys) : undefined;
  const ink = colors ? firstPaintable(colors, inkKeys) : undefined;
  return {
    // A light-category theme opens on white and a dark-category theme on a dark
    // surface, whatever its own background token happens to be.
    stage: stage ?? (mode === 'dark' ? '#0a0a0a' : '#ffffff'),
    ink: ink ?? (mode === 'dark' ? '#fafafa' : '#0a0a0a'),
    // A border is the theme's own, or a faint line of its ink so that the
    // boundary reads on a stage of any brightness.
    border: (colors && firstPaintable(colors, ['border'])) ?? undefined,
  };
}

export type ThemeSurface = ReturnType<typeof themeSurface>;

export const themeInfo = Object.fromEntries(themes.map(({ id, mode, name, source }) => [id, { mode, name, source }]));

// A list names fallbacks in order, matching the component theme's fallbacks.
const tokens: Record<string, string | string[]> = {
  background: 'background',
  foreground: 'foreground',
  card: 'background',
  'card-foreground': 'foreground',
  popover: 'popover.background',
  'popover-foreground': 'popover.foreground',
  primary: 'primary.background',
  'primary-foreground': 'primary.foreground',
  secondary: 'secondary.background',
  'secondary-foreground': 'secondary.foreground',
  muted: 'muted.background',
  'muted-foreground': 'muted.foreground',
  accent: 'accent.background',
  'accent-foreground': 'accent.foreground',
  border: 'border',
  input: 'input.border',
  ring: 'ring',
  titlebar: 'title_bar.background',
  sidebar: 'muted.background',
  'sidebar-foreground': 'foreground',
  'sidebar-primary': 'primary.background',
  'sidebar-primary-foreground': 'primary.foreground',
  'sidebar-accent': 'accent.background',
  'sidebar-accent-foreground': 'accent.foreground',
  'sidebar-border': 'border',
  'sidebar-ring': 'ring',
  brand: 'primary.background',
  'brand-hover': 'primary.hover.background',
  'brand-contrast': 'primary.foreground',
  'brand-subtle': 'muted.background',
  selection: ['selection.background', 'primary.background'],
  success: 'base.green',
  warning: 'base.yellow',
  destructive: 'base.red',
  'scrollbar-track': 'scrollbar.background',
  'scrollbar-thumb': 'scrollbar.thumb.background',
  'data-1': 'base.cyan',
  'data-2': 'base.blue',
  'data-3': 'base.green',
  'data-4': 'base.yellow',
  'data-5': 'base.magenta',
  'code-bg': 'editor.background',
  'code-fg': 'editor.foreground',
  'code-keyword': 'syntax.keyword',
  'code-string': 'syntax.string',
  'code-comment': 'syntax.comment',
  'code-fn': 'syntax.function',
  'code-type': 'syntax.type',
  'code-constant': 'syntax.constant',
  'code-number': 'syntax.number',
  'code-attribute': 'syntax.attribute',
  'code-property': 'syntax.property',
  'code-variable': 'syntax.variable.special',
  'code-link': 'syntax.link_uri',
};

function highlightColor(highlight: Record<string, unknown> | undefined, key: string): string | undefined {
  const value = highlight?.[key] ?? key.split('.').reduce<unknown>((part, segment) =>
    part && typeof part === 'object' ? (part as Record<string, unknown>)[segment] : undefined, highlight);
  return typeof value === 'string' ? value : value && typeof value === 'object'
    ? (value as { color?: string }).color : undefined;
}

export const themeCss = themes.map(({ id, colors, highlight }) => {
  const resolve = (source: string) => source.startsWith('editor.') || source.startsWith('syntax.')
    ? highlightColor(highlight, source) : colors[source];
  const declarations = Object.entries(tokens).flatMap(([token, sources]) => {
    const value = [sources].flat().map(resolve).find(Boolean);
    return value ? [`--${token}:${value};`] : [];
  });
  return `html[data-theme="${id}"]{${declarations.join('')}}`;
}).join('\n');
