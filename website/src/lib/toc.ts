// The docs page renders every framework's sections into one document and hides
// the ones the selected framework has no sample for (`data-framework-missing`,
// which the stylesheet turns into `display: none`). The outline is therefore
// derived from the headings that are actually laid out under the framework in
// force, never from every heading the page carries: an entry that points at a
// hidden section is a link that does nothing.
//
// The outline is also categorised, not merely indented by heading tag. A
// component page has three regions the reader thinks in — install, the live
// example, and the how-to-use prose that ends at the API reference — so the
// sections between the live example and the API reference read as children of
// one "Usage" category. `## Usage` itself is that category's row; every other
// heading in the region sits one level under it, and an `h4` under one of those
// sits one level under that. The install bar and the live example are injected
// by the layout (`TOC_PINS`), so they are pinned rows rather than headings.
//
// The page's closing sections — the other frameworks and the previous/next
// walk — sit after all of that on every component page, including the 45 pages
// with no `## API Reference` for the usage region to stop at. They carry
// `data-toc-trailing` (`trailing` here), which ends the region the same way the
// API reference does, so they read as top-level rows rather than as more
// children of Usage.
//
// Some component pages never wrote a `## Usage` heading — their samples sit
// under prose `h2`s ("Anatomy and basic usage", "Create state and choose the
// starting position"). The region is still marked: the layout injects the live
// example only on a page that shows one, so the pin says this is a component
// page whose whole content is the usage region, and the API reference or the
// trailing sections still end it. When the mark has no `## Usage` heading, the
// category is synthesised as a label row (no heading to link to) and every
// heading in the region is nested under it. A guide or the component index has
// no live example, so it keeps its own tag depth and is unchanged.

export interface TocHeading {
  id: string;
  label: string;
  /** The heading tag level: 2 for `h2`, 3 for `h3`, and so on. */
  level: number;
  /** Whether the active framework shows this heading. Read from layout, not
   *  from the mark: `getClientRects()` is empty for a `display: none` section,
   *  which is the same question the browser answers. */
  visible: boolean;
  /** A heading that closes the page: it ends the usage region even when the
   *  page has no API reference, so it is a top-level row. */
  trailing?: boolean;
}

export interface TocItem {
  id: string;
  label: string;
  /** The section the row links to. A synthesised category names a region the
   *  mark has no heading for, so it is a label row with no `href`. */
  href?: string;
  depth: number;
}

/** The anchors that are not markdown headings: the install bar and the live
 *  example. Both sit above the first heading, so they lead the outline. */
export const TOC_PINS = [
  { id: 'framework-bar', label: 'Installation' },
  { id: 'live-example', label: 'Live Example' },
] as const;

/** The row that names the usage region on a component page whose mark has no
 *  `## Usage` heading. The id is stable but deliberately not a page anchor:
 *  there is no section to scroll to, only a region to name. */
const USAGE_CATEGORY_ID = 'toc-usage-category';
const USAGE_LABEL = 'Usage';

const PIN_IDS = new Set<string>(TOC_PINS.map((pin) => pin.id));
const USAGE = /^usage$/i;
const API_REFERENCE = /^api reference\b/i;

/** The outline the page's own structure produces. The pins lead, as the layout
 *  renders them above the first heading; the Usage category row leads its own
 *  children; everything else stays in document order. */
export function tocItems(headings: TocHeading[]): TocItem[] {
  const visible = headings.filter((heading) => heading.visible && heading.id.length > 0);
  const pins = visible.filter((heading) => PIN_IDS.has(heading.id));
  const content = visible.filter((heading) => !PIN_IDS.has(heading.id));

  // The how-to-use region runs from the first content heading to the API
  // reference. `## Usage` names that region; where the mark has no such heading
  // the category is synthesised below, so the same nesting still applies. A
  // closing section ends the region as well, so a component page with no API
  // reference still ends its nesting at the two sections that close the page.
  const usageAt = content.findIndex((heading) => heading.level === 2 && USAGE.test(heading.label));
  const apiAt = content.findIndex((heading) => heading.level === 2 && API_REFERENCE.test(heading.label));
  const trailingAt = content.findIndex((heading) => heading.trailing === true);
  const regionEnd = Math.min(
    apiAt >= 0 ? apiAt : content.length,
    trailingAt >= 0 ? trailingAt : content.length,
  );

  // The live example is injected by the layout only where the page shows one,
  // so the pin marks a component page — whose content, from the first heading to
  // the API reference or the closing sections, is the usage region. Where that
  // region's `## Usage` heading was never written, the category is synthesised
  // so its samples still read as its children rather than as top-level rows.
  const hasLiveExample = pins.some((heading) => heading.id === 'live-example');
  const synthesiseUsage = usageAt < 0 && hasLiveExample && regionEnd > 0;

  // A stack of the depth last used at each heading level, so a child sits one
  // deeper than the nearest heading above it — except under the Usage category,
  // whose direct `h3` children belong to the category itself rather than to a
  // nested `h2` section.
  const stack: number[] = [];
  const depths = content.map((heading, index) => {
    const level = Math.min(Math.max(heading.level, 2), 6);
    stack.length = Math.min(stack.length, level - 2);
    let depth: number;
    if (index >= regionEnd || (usageAt < 0 && !synthesiseUsage)) {
      depth = level - 2;
    } else if (level === 2) {
      depth = index === usageAt ? 0 : 1;
    } else {
      let parent = 0;
      for (let above = level - 1; above >= 2; above -= 1) {
        const value = stack[above - 2];
        if (value !== undefined) {
          parent = value;
          break;
        }
      }
      depth = parent + 1;
    }
    stack[level - 2] = depth;
    return depth;
  });

  const item = (heading: TocHeading, depth: number): TocItem => ({
    id: heading.id,
    label: heading.label,
    href: `#${heading.id}`,
    depth,
  });

  // The category row leads its children. The page's own `## Import` section
  // sits above `## Usage` in the mark, so without this the "Usage" row would
  // trail the first child and the indent would read as belonging to the
  // preceding section. Depth is already assigned, so this only orders rows.
  const region = content
    .map((heading, index) => ({ heading, depth: depths[index] }))
    .slice(0, regionEnd);
  if (usageAt >= 0 && usageAt < regionEnd) {
    const category = region[usageAt];
    region.splice(usageAt, 1);
    region.unshift(category);
  }
  const after = content
    .map((heading, index) => ({ heading, depth: depths[index] }))
    .slice(regionEnd);

  // A synthesised category has no heading of its own to anchor, so it is a label
  // row; it still leads the region it names.
  const category: TocItem | null = synthesiseUsage
    ? { id: USAGE_CATEGORY_ID, label: USAGE_LABEL, depth: 0 }
    : null;

  return [
    ...pins.map((heading) => item(heading, 0)),
    ...(category ? [category] : []),
    ...[...region, ...after].map(({ heading, depth }) => item(heading, depth)),
  ];
}

/** Whether two outlines would render identically, so a mutation that left the
 *  visible set unchanged does not rebuild the rail. */
export function sameTocItems(a: TocItem[], b: TocItem[]): boolean {
  return (
    a.length === b.length &&
    a.every(
      (item, index) =>
        item.id === b[index].id &&
        item.label === b[index].label &&
        item.depth === b[index].depth,
    )
  );
}
