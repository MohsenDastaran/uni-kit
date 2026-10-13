import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { hasIcedExample, icedSections } from './remark-iced-source.js';

// Counts the examples the gallery window actually paints, one per framework.
// GPUI stories mark each example with `section(...)`. Slint examples mark each
// one with a GroupBox. Iced demos mark each example with `// section: <title>`,
// which `icedSections` reads -- the same reader the sample panels are cut with
// -- so the count is the number of samples a reader sees on the page. A window
// that is a single example has no markers at all.

const ROOT = process.cwd();
const STORIES = join(ROOT, '../crates/story/src/stories');
const SLINT_EXAMPLES = join(ROOT, '../crates/slint-component/ui/examples');
const ICED_EXAMPLES = join(ROOT, '../crates/iced-component/examples');
const GALLERY = join(ROOT, '../crates/story/src/gallery.rs');
const SANKEY = join(ROOT, '../crates/story/src/fixtures/tsla-income-statement.json');

/** Story title when it is not the PascalCase of the page slug. Mirrors the example window. */
const STORY_TITLES: Record<string, string> = {
  'alert-dialog': 'AlertDialog',
  'color-picker': 'ColorPicker',
  'data-table': 'DataTable',
  'date-picker': 'DatePicker',
  'description-list': 'DescriptionList',
  dropdown_button: 'DropdownButton',
  'focus-trap': 'Dialog',
  'group-box': 'GroupBox',
  'hover-card': 'HoverCard',
  'input-group': 'Input Group',
  'native-menu': 'NativeMenu',
  notification: 'Notification',
  'number-input': 'NumberInput',
  'otp-input': 'OtpInput',
  scrollable: 'Scrollbar',
  'status-bar': 'StatusBar',
  'text-view': 'Editor',
  'title-bar': 'Introduction',
  'virtual-list': 'VirtualList',
};

/**
 * Whether the sources the counts are derived from are on disk.
 *
 * Both live outside `website/`, under `crates/`, so a build that only has the
 * website directory in context finds no stories and no Slint examples. Every
 * count then reads as zero, which is indistinguishable from "this component has
 * no examples" -- and a production build hides those. Callers must treat a
 * missing source tree as "unknown", never as "empty", or a trimmed build
 * context silently empties the whole navigation.
 */
export const exampleSourcesAvailable = existsSync(STORIES) && existsSync(SLINT_EXAMPLES);

if (!exampleSourcesAvailable) {
  console.warn(
    `[example-counts] Story and Slint sources not found under ${ROOT}/../crates. ` +
      'Example counts are unavailable, so no page will be treated as empty. ' +
      'Build from the repository root if the navigation needs its counts.',
  );
}

/**
 * Whether the iced example sources are on disk.
 *
 * The iced crate is being built beside GPUI and Slint, and its `ui/examples/`
 * directory arrives with its first components. Until it does, no iced count can
 * be measured, and every count reading as zero would be indistinguishable from
 * "this component has no iced example" — which a production build hides. A
 * checkout without the directory is "unknown", never "empty"; the same reading
 * guards the Slint tree above.
 */
export const icedExampleSourcesAvailable = existsSync(ICED_EXAMPLES);

export interface GalleryCounts {
  gpui?: number;
  slint?: number;
  iced?: number;
}

function titleCase(value: string): string {
  return value
    .split(/[-_]/)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join('');
}

function stemFromTitle(title: string): string {
  return title
    .replace(/[\s-]+/g, '')
    .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
    .toLowerCase();
}

function stripComments(source: string): string {
  return source.replace(/\/\*[\s\S]*?\*\//g, '').replace(/^\s*\/\/.*$/gm, '');
}

function wasmStoryStems(): Set<string> {
  let source = '';
  try {
    source = readFileSync(GALLERY, 'utf8');
  } catch {
    return new Set();
  }
  source = source.replace(
    /#\[cfg\(not\(target_family = "wasm"\)\)\]\s*StoryContainer::panel::<\w+>\([^)]*\),?/g,
    '',
  );
  const stems = new Set<string>();
  for (const match of source.matchAll(/StoryContainer::panel::<(\w+)Story>/g)) {
    stems.add(stemFromTitle(match[1]));
  }
  return stems;
}

const wasmStories = wasmStoryStems();

function readStory(stem: string): string {
  const parts: string[] = [];
  const file = join(STORIES, `${stem}_story.rs`);
  const dir = join(STORIES, `${stem}_story`);
  if (existsSync(file)) parts.push(readFileSync(file, 'utf8'));
  if (existsSync(dir) && statSync(dir).isDirectory()) {
    for (const name of readdirSync(dir)) {
      if (name.endsWith('.rs')) parts.push(readFileSync(join(dir, name), 'utf8'));
    }
  }
  return parts.join('\n');
}

function countSections(source: string): number {
  return stripComments(source).match(/(^|[^.\w])section\s*\(/g)?.length ?? 0;
}

function countChartCards(source: string): number {
  const start = source.search(/fn sections\s*\(/);
  if (start < 0) return 0;
  const end = source.indexOf('\n}', start);
  const body = source.slice(start, end < 0 ? undefined : end);
  let sankey = 0;
  if (/0\s*\.\.\s*sankey_count/.test(body)) {
    try {
      const fixture = JSON.parse(readFileSync(SANKEY, 'utf8')) as { list?: unknown[] };
      sankey = Array.isArray(fixture.list) ? fixture.list.length : 0;
    } catch {
      sankey = 0;
    }
  }
  const withoutSankey = body.replace(/\(0\s*\.\.\s*sankey_count\)\.map\([^)]*\)/g, '');
  let cards = 0;
  for (const array of withoutSankey.matchAll(/\[([\s\S]*?)\]/g)) {
    const names = array[1].match(/\b[A-Z][A-Za-z0-9]*\b/g) ?? [];
    cards += names.filter((name) => name !== 'ChartSection').length;
  }
  return cards + sankey;
}

function countSlint(slug: string): number {
  const path = join(SLINT_EXAMPLES, `${slug}.slint`);
  if (!existsSync(path)) return 0;
  const source = stripComments(readFileSync(path, 'utf8'));
  const boxes = source.match(/\bGroupBox\s*\{/g)?.length ?? 0;
  if (boxes > 0) return boxes;
  const cards = source.match(/\bChartCard\s*\{/g)?.length ?? 0;
  if (cards > 0) return cards;
  const sections = source.match(/\bExampleSection\s*\{/g)?.length ?? 0;
  if (sections > 0) return sections;
  return /\bexport\s+component\b/.test(source) ? 1 : 0;
}

function countIced(slug: string): number {
  if (!icedExampleSourcesAvailable) return 0;
  // Two files answer two questions, so both are read. `hasIcedExample` says
  // whether the component has a gallery window at all -- an example that is a
  // stub or absent counts zero, never one. The number of examples inside that
  // window comes from `icedSections`, which the remark plugin already uses to
  // cut the sample panels, so this module reuses that reader instead of
  // repeating the `// section:` contract and counting markers itself.
  if (!hasIcedExample(slug)) return 0;
  const sections = icedSections(slug, ROOT);
  // A demo with no markers is not zero examples -- `calendar` paints one whole
  // example and just does not divide it. That is the same reading `countGpui`
  // gives a story with no `section(...)` calls.
  return sections.length > 0 ? sections.length : 1;
}

function countGpui(component: string): number {
  const title = STORY_TITLES[component] ?? titleCase(component);
  const stem = title === 'Introduction' ? 'welcome' : stemFromTitle(title);
  if (!wasmStories.has(stem)) return 0;
  const source = readStory(stem);
  if (!source) return 0;
  const count = stem === 'chart' ? countChartCards(source) : countSections(source);
  return count > 0 ? count : 1;
}

/**
 * Examples painted in the gallery window for this component page.
 * `example` is the page's frontmatter override; `false` means the page has no window.
 */
export function galleryCounts(
  slug: string,
  example?: string | false,
): GalleryCounts | undefined {
  if (example === false) return undefined;
  const gpui = countGpui(example ?? slug);
  const slint = countSlint(slug);
  const iced = countIced(slug);
  if (!gpui && !slint && !iced) return undefined;
  return {
    ...(gpui > 0 ? { gpui } : {}),
    ...(slint > 0 ? { slint } : {}),
    ...(iced > 0 ? { iced } : {}),
  };
}
