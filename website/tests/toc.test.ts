import assert from 'node:assert/strict';
import test from 'node:test';
import { TOC_PINS, sameTocItems, tocItems, type TocHeading } from '../src/lib/toc.js';

// The outline is built from what the active framework lays out. These cases
// pin the rule the defect report named: a heading the framework hides is not an
// entry, and every entry that survives points at a visible heading.

function heading(overrides: Partial<TocHeading> & { id: string }): TocHeading {
  return { label: overrides.id, depth: 0, visible: true, ...overrides };
}

test('a heading the framework hides is left out of the outline', () => {
  const items = tocItems([
    heading({ id: 'import', label: 'Import', depth: 0 }),
    heading({ id: 'calendar-sizes', label: 'Calendar Sizes', depth: 1, visible: false }),
    heading({ id: 'usage', label: 'Usage', depth: 0 }),
  ]);
  assert.deepEqual(
    items.map((item) => item.id),
    ['import', 'usage'],
  );
});

test('depth keeps the indentation level and href resolves to the heading id', () => {
  const items = tocItems([
    heading({ id: 'usage', depth: 0 }),
    heading({ id: 'two-columns', depth: 1 }),
  ]);
  assert.deepEqual(items, [
    { id: 'usage', label: 'usage', href: '#usage', depth: 0 },
    { id: 'two-columns', label: 'two-columns', href: '#two-columns', depth: 1 },
  ]);
});

test('the install bar and live example lead the outline, and only when shown', () => {
  assert.equal(TOC_PINS[0].id, 'framework-bar');
  assert.equal(TOC_PINS[1].id, 'live-example');
  const items = tocItems([
    heading({ id: 'framework-bar', label: 'Installation', depth: 0 }),
    // The live example has not mounted yet: no entry, not a dead anchor.
    heading({ id: 'live-example', label: 'Live Example', depth: 0, visible: false }),
    heading({ id: 'import', label: 'Import', depth: 0 }),
  ]);
  assert.deepEqual(
    items.map((item) => item.id),
    ['framework-bar', 'import'],
  );
});

test('a framework with nothing visible yields no entries, so no empty rail', () => {
  assert.deepEqual(tocItems([heading({ id: 'a', visible: false })]), []);
});

test('a heading with no id cannot become an entry', () => {
  assert.deepEqual(tocItems([heading({ id: '' })]), []);
});

test('sameTocItems reports a changed visible set', () => {
  const base = tocItems([heading({ id: 'a' }), heading({ id: 'b', depth: 1 })]);
  assert.equal(sameTocItems(base, tocItems([heading({ id: 'a' }), heading({ id: 'b', depth: 1 })])), true);
  assert.equal(sameTocItems(base, tocItems([heading({ id: 'a' })])), false);
  assert.equal(sameTocItems(base, tocItems([heading({ id: 'a' }), heading({ id: 'b', depth: 0 })])), false);
  assert.equal(sameTocItems(base, tocItems([heading({ id: 'a' }), heading({ id: 'c', depth: 1 })])), false);
});
