// A gallery card scrolls to the usage section that shows that sample.
// The card's title is matched to the heading that received the sample.
// A shared index into the code list is not a title: sections without a fence
// of their own would land on a different example.

export function exampleWords(value) {
  return String(value ?? '')
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, ' ')
    .trim()
    .split(/\s+/)
    .filter(Boolean)
    .join(' ');
}

export function galleryTitlesOf(value) {
  return String(value ?? '')
    .split('|')
    .map((title) => title.trim())
    .filter(Boolean);
}

/**
 * @param {string} title
 * @param {string} [_period]
 * @param {string} [_note]
 * @param {string} [subtitle]
 */
export function gallerySourceTitle(title, _period, _note, subtitle) {
  const name = String(title ?? '').trim();
  const extra = String(subtitle ?? '').trim();
  return extra ? `${name} — ${extra}` : name;
}

/**
 * @param {{ text: string, titles?: string, visible?: boolean }[]} headings
 * @param {string} title
 */
export function findExampleHeading(headings, title) {
  const wanted = exampleWords(title);
  if (!wanted) return undefined;
  // The page shows one framework at a time, and a heading the framework in
  // force has no sample under is left out of the view. `scrollIntoView` on a
  // `display: none` heading does nothing, so such a heading is not a candidate
  // even when it carries the title. The page reads visibility from the DOM; a
  // heading the caller does not describe is taken as shown.
  const shown = headings.filter((heading) => heading.visible !== false);
  const tagged = shown.find((heading) =>
    galleryTitlesOf(heading.titles).some((item) => exampleWords(item) === wanted),
  );
  if (tagged) return tagged;
  return shown.find((heading) => exampleWords(heading.text) === wanted);
}
