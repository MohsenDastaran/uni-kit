// The docs page renders every framework's sections into one document and hides
// the ones the selected framework has no sample for (`data-framework-missing`,
// which the stylesheet turns into `display: none`). The outline is therefore
// derived from the headings that are actually laid out under the framework in
// force, never from every heading the page carries: an entry that points at a
// hidden section is a link that does nothing.

export interface TocHeading {
  id: string;
  label: string;
  depth: number;
  /** Whether the active framework shows this heading. Read from layout, not
   *  from the mark: `getClientRects()` is empty for a `display: none` section,
   *  which is the same question the browser answers. */
  visible: boolean;
}

export interface TocItem {
  id: string;
  label: string;
  href: string;
  depth: number;
}

/** The anchors that are not markdown headings: the install bar and the live
 *  example. Both sit above the first heading, so they lead the outline. */
export const TOC_PINS = [
  { id: 'framework-bar', label: 'Installation' },
  { id: 'live-example', label: 'Live Example' },
] as const;

/** The visible candidates, in document order, as outline entries. */
export function tocItems(headings: TocHeading[]): TocItem[] {
  return headings
    .filter((heading) => heading.visible && heading.id.length > 0)
    .map((heading) => ({
      id: heading.id,
      label: heading.label,
      href: `#${heading.id}`,
      depth: heading.depth,
    }));
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
