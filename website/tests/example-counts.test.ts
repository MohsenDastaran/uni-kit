import assert from 'node:assert/strict';
import test from 'node:test';
import { galleryCounts } from '../src/lib/example-counts.js';
import { nshSections } from '../src/lib/remark-nsh-source.js';

// The sidebar count is not a number anyone types: it is the number of
// `// section:` markers in the demo, which is also the number of sample panels
// the page renders. These cases guard the two pages where the old hard-coded
// demo counts were visibly stale.
test('NSH example counts are derived from the demo sections', () => {
  const cases = ['command', 'data-table', 'table', 'combobox', 'color-picker'];
  for (const slug of cases) {
    const sections = nshSections(slug, process.cwd());
    const count = galleryCounts(slug)?.nsh ?? 0;
    assert.ok(sections.length > 0, `${slug} has no parsed sections`);
    assert.equal(
      count,
      sections.length,
      `${slug}: sidebar says ${count} examples, demo has ${sections.length} sections`,
    );
  }
});

test('the previously stale counts now include every demo section', () => {
  // These were shown as 4 and 2 before the demos grew; the count must follow
  // the markers rather than a remembered total.
  assert.ok((galleryCounts('command')?.nsh ?? 0) > 4);
  assert.ok((galleryCounts('data-table')?.nsh ?? 0) > 2);
});
