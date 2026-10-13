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
 * @param {{ text: string, titles?: string, visible?: boolean, only?: string }[]} headings
 * @param {string} title
 * @param {string} [framework] The framework in force, as `html[data-framework]` names it.
 */
export function findExampleHeading(headings, title, framework) {
  const wanted = exampleWords(title);
  if (!wanted) return undefined;
  // The page shows one framework at a time, and a heading the framework in
  // force has no sample under is left out of the view. `scrollIntoView` on a
  // `display: none` heading does nothing, so such a heading is not a candidate
  // even when it carries the title. The page reads visibility from the DOM; a
  // heading the caller does not describe is taken as shown.
  const shown = headings.filter((heading) => heading.visible !== false);
  const carries = (heading) =>
    galleryTitlesOf(heading.titles).some((item) => exampleWords(item) === wanted);
  // A heading whose section holds one framework's sample alone belongs to that
  // framework, and its title is its own. A shared heading can carry another
  // framework's alias for the same word — iced's `Progress` is also the Slint
  // title on `Icons and states` — so the framework in force claims its own
  // section first and only then takes a heading it shares.
  if (framework) {
    const own = shown.find((heading) => heading.only === framework && carries(heading));
    if (own) return own;
  }
  const tagged = shown.find(carries);
  if (tagged) return tagged;
  return shown.find((heading) => exampleWords(heading.text) === wanted);
}
