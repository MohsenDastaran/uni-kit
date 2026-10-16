import assert from 'node:assert/strict';
import test from 'node:test';
import { TOC_PINS, sameTocItems, tocItems, type TocHeading } from '../src/lib/toc.js';

// The outline is built from what the active framework lays out, and its depth
// is derived from the page's regions rather than from the heading tags alone.
// These cases pin the two rules the defect report named: a heading the
// framework hides is not an entry, and the how-to-use region reads as children
// of one Usage category so the rail has levels to bend between.

function heading(overrides: Partial<TocHeading> & { id: string }): TocHeading {
  return { label: overrides.id, level: 2, visible: true, ...overrides };
}

test('a heading the framework hides is left out of the outline', () => {
  const items = tocItems([
    heading({ id: 'import', label: 'Import' }),
    heading({ id: 'calendar-sizes', label: 'Calendar Sizes', level: 3, visible: false }),
    heading({ id: 'usage', label: 'Usage' }),
  ]);
  assert.deepEqual(
    items.map((item) => item.id),
    ['usage', 'import'],
  );
});

test('the Usage category leads Import and owns the samples and their h4 subsections', () => {
  const items = tocItems([
    heading({ id: 'import', label: 'Import' }),
    heading({ id: 'usage', label: 'Usage' }),
    heading({ id: 'variants', label: 'Variants', level: 3 }),
    heading({ id: 'icon-types', label: 'Icon Types', level: 4 }),
    heading({ id: 'date-restrictions', label: 'Date Restrictions' }),
    heading({ id: 'disabled-weekends', label: 'Disabled Weekends', level: 3 }),
    heading({ id: 'api-reference', label: 'API Reference' }),
    heading({ id: 'calendar-state', label: 'CalendarState', level: 3 }),
  ]);
  assert.deepEqual(
    items.map(({ id, depth }) => [id, depth]),
    [
      ['usage', 0],
      ['import', 1],
      ['variants', 1],
      ['icon-types', 2],
      ['date-restrictions', 1],
      ['disabled-weekends', 2],
      ['api-reference', 0],
      ['calendar-state', 1],
    ],
  );
  assert.deepEqual(items[1], { id: 'import', label: 'Import', href: '#import', depth: 1 });
});

// The page's closing sections — the other frameworks and the previous/next
// walk — carry `data-toc-trailing`. They end the usage region the way the API
// reference does, which matters on the 45 component pages that have no API
// reference: without it they would read as more children of Usage, and with it
// they are the top-level rows they look like on the page.
test('a closing section is top-level on a page with no API Reference', () => {
  const items = tocItems([
    heading({ id: 'import', label: 'Import' }),
    heading({ id: 'usage', label: 'Usage' }),
    heading({ id: 'variants', label: 'Variants', level: 3 }),
    heading({ id: 'other-frameworks', label: 'See this component in other frameworks', trailing: true }),
    heading({ id: 'previous-next', label: 'Previous and next component', trailing: true }),
  ]);
  assert.deepEqual(
    items.map(({ id, depth }) => [id, depth]),
    [
      ['usage', 0],
      ['import', 1],
      ['variants', 1],
      ['other-frameworks', 0],
      ['previous-next', 0],
    ],
  );
});

test('closing sections follow the API reference at the same top level', () => {
  const items = tocItems([
    heading({ id: 'usage', label: 'Usage' }),
    heading({ id: 'variants', label: 'Variants', level: 3 }),
    heading({ id: 'api-reference', label: 'API Reference' }),
    heading({ id: 'calendar-state', label: 'CalendarState', level: 3 }),
    heading({ id: 'other-frameworks', label: 'See this component in other frameworks', trailing: true }),
    heading({ id: 'previous-next', label: 'Previous and next component', trailing: true }),
  ]);
  assert.deepEqual(
    items.map(({ id, depth }) => [id, depth]),
    [
      ['usage', 0],
      ['variants', 1],
      ['api-reference', 0],
      ['calendar-state', 1],
      ['other-frameworks', 0],
      ['previous-next', 0],
    ],
  );
});

test('a page without a Usage heading keeps its own tag depth', () => {  const items = tocItems([
    heading({ id: 'getting-started' }),
    heading({ id: 'install', level: 3 }),
    heading({ id: 'manual', level: 4 }),
  ]);
  assert.deepEqual(
    items.map(({ id, depth }) => [id, depth]),
    [
      ['getting-started', 0],
      ['install', 1],
      ['manual', 2],
    ],
  );
});

test('the install bar and live example lead the outline, and only when shown', () => {
  assert.equal(TOC_PINS[0].id, 'framework-bar');
  assert.equal(TOC_PINS[1].id, 'live-example');
  const items = tocItems([
    heading({ id: 'framework-bar', label: 'Installation' }),
    // The live example has not mounted yet: no entry, not a dead anchor.
    heading({ id: 'live-example', label: 'Live Example', visible: false }),
    heading({ id: 'import', label: 'Import' }),
  ]);
  assert.deepEqual(
    items.map(({ id, depth }) => [id, depth]),
    [
      ['framework-bar', 0],
      ['import', 0],
    ],
  );
});

test('a framework with nothing visible yields no entries, so no empty rail', () => {
  assert.deepEqual(tocItems([heading({ id: 'a', visible: false })]), []);
  assert.deepEqual(tocItems([heading({ id: '' })]), []);
});

test('sameTocItems reports a changed visible set or a changed depth', () => {
  const base = tocItems([heading({ id: 'a' }), heading({ id: 'b', level: 3 })]);
  assert.equal(sameTocItems(base, tocItems([heading({ id: 'a' }), heading({ id: 'b', level: 3 })])), true);
  assert.equal(sameTocItems(base, tocItems([heading({ id: 'a' })])), false);
  assert.equal(sameTocItems(base, tocItems([heading({ id: 'a' }), heading({ id: 'b' })])), false);
  assert.equal(sameTocItems(base, tocItems([heading({ id: 'a' }), heading({ id: 'c', level: 3 })])), false);
});
