import { existsSync, readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

// NSH is added beside GPUI, Slint and iced: one TSX source per component in
// `src/components/`, with a whole demo in `src/demos/` and a thin runnable
// entry in `examples/`. The site asks this module the same question it asks
// `remark-slint-source.js` for Slint — does this page have a view at all? — and
// the answer is read from the sources rather than from the build, so a dev
// server and a published site agree.
//
// A registry key may hold a hyphen and a module may not, so the file for
// `message-role` is `message_role.tsx`. That mapping is the one the manifest
// builder and the checklist use.

export const nshRoot = (root = process.cwd()) =>
  resolve(root, '..', 'crates', 'nsh-component');

/** The file a registry key lives in, hyphenated key to underscored module. */
export const nshModule = (slug) => slug.replace(/-/g, '_');

function stripComments(source) {
  return source.replace(/\/\*[\s\S]*?\*\//g, '').replace(/^\s*\/\/.*$/gm, '');
}

/**
 * Whether a file is an implementation rather than the shape of one.
 *
 * A stub compiles, is declared, and answers every question that only asks
 * whether a file exists — which is how eight five-line files once counted as
 * done work. An example is an application with a `main`; a component builds an
 * element. A file that says it is pending never counts.
 */
function implemented(path, kind) {
  if (!existsSync(path)) return false;
  let source = '';
  try {
    source = readFileSync(path, 'utf8');
  } catch {
    return false;
  }
  if (/Implementation pending|not implemented|todo!\(|unimplemented!\(/i.test(source)) return false;
  const code = stripComments(source).trim();
  if (code.length === 0) return false;
  if (kind === 'example') {
    // An NSH example is a thin entry that mounts the demo the library holds, so
    // the browser gallery and `nsh run` show the same app.
    return /\bmount\s*\(/.test(code);
  }
  // A component is a function that returns JSX. `theme.ts` is the shared token
  // file and builds no widget, so the JSX test does not apply to it.
  if (path.endsWith('theme.ts')) return /export/.test(code) && /tokens/.test(code);
  return /export\s+(function|const)/.test(code) && /</.test(code);
}

/**
 * Whether this slug has an nsh example to show, and one worth showing. An
 * empty file, a file holding only comments, and a stub that says it is pending
 * all answer no, which is the same reading the checklist uses.
 */
export function hasNshExample(slug, root = process.cwd()) {
  return implemented(
    join(nshRoot(root), 'examples', `${nshModule(slug)}.tsx`),
    'example',
  );
}

/** Whether this slug has an nsh component at all, stub or not. */
export function hasNshComponent(slug, root = process.cwd()) {
  return implemented(
    join(nshRoot(root), 'src', 'components', slug === 'theme' ? 'theme.ts' : `${nshModule(slug)}.tsx`),
    'component',
  );
}

/**
 * The site has an nsh view of this page at all: an nsh example to show, on a
 * page that shows one. `example: false` is a page with no example window — a
 * guide — so the nsh sidebar has nothing to put beside it and the framework
 * switch has nothing to switch to. The same question is asked of Slint in
 * `remark-slint-source.js`.
 */
export function hasNshPage(slug, example, root = process.cwd()) {
  // Both halves, because either alone claims more than exists: a component with
  // no example has no view to show, and an example with no component installs
  // nothing.
  return example !== false && hasNshComponent(slug, root) && hasNshExample(slug, root);
}

// ---------------------------------------------------------------------------
// The plugin: the NSH panel under each usage heading.
//
// The demos are whole applications and the page samples are cut out of them,
// so what a component page shows is the code the gallery runs rather than a
// second copy of it. A TSX file cannot be parsed the way a `.slint` one can, so
// the demos mark their sections instead:
//
//     // section: Variants
//     const variants = (<Row>…</Row>);
//
// That one comment is the contract. Everything from it to the next marker is the
// section's sample — the same lines the browser runs — and the marker itself is
// not shown.

/** The registry key a page's markdown file belongs to, or nothing. */
function nshSlugOf(file) {
  const path = String(file?.path ?? file?.history?.[0] ?? '');
  const match = /[\\/]component[\\/]([^\\/]+)\.md$/.exec(path);
  return match?.[1] ?? null;
}

/** The same reading the Slint plugin uses: case and punctuation do not matter. */
function sameNshTitle(a, b) {
  const clean = (value) =>
    String(value ?? '')
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, ' ')
      .trim();
  return clean(a) === clean(b);
}

/**
 * The demo's sections, in the order they are written: the title from each marker
 * and the code under it, de-indented so it reads as a file rather than as the
 * inside of `view`.
 */
export function nshSections(slug, root = process.cwd()) {
  const path = join(nshRoot(root), 'src', 'demos', `${nshModule(slug)}.tsx`);
  if (!existsSync(path)) return [];
  const lines = readFileSync(path, 'utf8').split('\n');
  const sections = [];
  for (const line of lines) {
    const marker = /^\s*\/\/\s*section:\s*(.+?)\s*$/.exec(line);
    if (marker) {
      // `// section: end` closes the last one, so the lines after it — the status
      // line, the assembly of the gallery — are not shown as part of it.
      if (/^end$/i.test(marker[1])) break;
      sections.push({ title: marker[1], lines: [] });
      continue;
    }
    if (sections.length === 0) continue;
    sections[sections.length - 1].lines.push(line);
  }
  return sections
    .map((section) => {
      // De-indent by the block's own smallest indent, and drop the blank lines
      // the surrounding function adds.
      const body = [...section.lines];
      while (body.length && !body[0].trim()) body.shift();
      while (body.length && !body[body.length - 1].trim()) body.pop();
      const indents = body
        .filter((line) => line.trim())
        .map((line) => line.length - line.trimStart().length);
      const trim = indents.length ? Math.min(...indents) : 0;
      return {
        title: section.title,
        code: body.map((line) => line.slice(trim)).join('\n'),
      };
    })
    .filter((section) => section.code.trim().length > 0);
}

// The code is TSX. The site's Shiki registers `tsx`; `nsh` is an alias so the
// framework named by the fence survives to HTML, which a `data-*` property on a
// code node does not.
const nshCode = (value) => ({
  type: 'code',
  // The language is the only thing about a code block that survives the trip to
  // HTML — a `data-*` property set here does not — so nsh has a language name,
  // and `src/lib/markdown.js` aliases it to TSX for the highlighter.
  lang: 'nsh',
  meta: null,
  value,
  data: { hProperties: { 'data-gallery-framework': 'nsh' } },
});

/** The gallery posts the section title; the heading that received the sample
 *  keeps it, so a click scrolls to the sample rather than to whichever block
 *  shares an index. */
function tagNshHeading(node, title) {
  const data = (node.data ??= {});
  const props = (data.hProperties ??= {});
  const key = 'data-gallery-title';
  const current = String(props[key] ?? '')
    .split('|')
    .filter(Boolean);
  if (current.some((item) => sameNshTitle(item, title))) return;
  current.push(title);
  props[key] = current.join('|');
}

/** The gallery also posts the section's index. Several sections can share a
 *  title — the accordion documents two examples under one heading — so the
 *  index is what tells their headings apart when the title cannot. */
function tagNshIndex(node, index) {
  const data = (node.data ??= {});
  const props = (data.hProperties ??= {});
  props['data-gallery-index'] = String(index);
}

// The heading's readable text, with the literal nodes kept. A title the demo
// writes with backticks — `content` — is an `inlineCode` node, not text, so
// reading only `text` children dropped the word and the heading never matched
// its section. Anything else the page can put in a heading (emphasis, a link,
// math) contributes its children the same way.
function headingText(node) {
  if (node.type !== 'heading') return '';
  const read = (child) =>
    typeof child.value === 'string' ? child.value : (child.children ?? []).map(read).join('');
  return node.children.map(read).join('').trim();
}

function nshHeading(title) {
  return {
    type: 'heading',
    depth: 3,
    children: [{ type: 'text', value: title }],
  };
}

/**
 * The heading the plugin writes for a section the page does not have. It is
 * nsh's own: the other frameworks leave it out, and the lookup skips it there,
 * the same reading the framework pass gives a heading whose section holds only
 * nsh's sample. A created heading whose sample joins the page's own code under
 * it does not get that mark from the pass, so it is said here.
 */
function nshOnlyHeading(title) {
  const node = nshHeading(title);
  const data = (node.data ??= {});
  const props = (data.hProperties ??= {});
  props['data-framework-only'] = 'nsh';
  tagNshHeading(node, title);
  return node;
}

/**
 * The second heading for a title the page already has. The card keeps the type
 * as its title — the same text the gallery card posts — and the subtitle is
 * what tells this repeated sample from the one above it. The index is what the
 * page's lookup uses when both headings carry the same title.
 */
function nshDuplicateHeading(title, subtitle, index) {
  const node = nshOnlyHeading(title);
  node.children.push({ type: 'text', value: ` — ${subtitle}` });
  tagNshIndex(node, index);
  return node;
}

/** Where the heading's section ends: the next heading of the same or shallower
 *  depth. Used to put a repeated sample after the page's own sample instead of
 *  in front of it. */
function sectionEnd(children, heading) {
  const depth = heading.depth ?? 0;
  for (let at = children.indexOf(heading) + 1; at < children.length; at += 1) {
    const node = children[at];
    if (node.type === 'heading' && node.depth <= depth) return at;
  }
  return children.length;
}

/**
 * Where a section the page has no heading for goes: inside `## Usage`, at the end
 * of it, so an added section still reads as part of the examples. A page whose
 * mark has no `## Usage` heading still has the region — it ends at the
 * `## API Reference` — so the fallback stops there rather than at the end of the
 * tree, which would put the sample after the API reference and outside the
 * region the outline nests under Usage.
 */
function usageEnd(children) {
  const start = children.findIndex(
    (node) => node.type === 'heading' && /^usage$/i.test(headingText(node)),
  );
  if (start === -1) {
    const api = children.findIndex(
      (node) =>
        node.type === 'heading' &&
        node.depth === 2 &&
        /^api reference\b/i.test(headingText(node)),
    );
    return api === -1 ? children.length : api;
  }
  for (let i = start + 1; i < children.length; i += 1) {
    const node = children[i];
    if (node.type === 'heading' && node.depth <= 2) return i;
  }
  return children.length;
}

/**
 * The remark plugin: read the component's demo, and place each section's Rust
 * under the heading that names it — creating the heading for a section the page
 * does not have yet, in the demo's order.
 */
export function remarkNshSource() {
  return (tree, file) => {
    const slug = nshSlugOf(file);
    if (!slug || !hasNshPage(slug, true)) return;
    const sections = nshSections(slug);
    if (sections.length === 0) return;

    const children = tree.children;
    const byTitle = new Map();
    for (const node of children) {
      const title = headingText(node);
      if (!title) continue;
      const list = byTitle.get(title) ?? [];
      list.push(node);
      byTitle.set(title, list);
    }

    // The page's import fence is the GPUI one. A reader who has chosen nsh needs
    // the lines they would actually write, and the demo has them: its own `use`
    // statements, which is what the rest of these samples assume.
    const importHeading = children.find(
      (node) => node.type === 'heading' && /^import$/i.test(headingText(node)),
    );
    if (importHeading) {
      const demo = readFileSync(
        join(nshRoot(), 'src', 'demos', `${nshModule(slug)}.tsx`),
        'utf8',
      );
      const imports = [];
      let pending = null;
      for (const line of demo.split('\n')) {
        if (pending) {
          pending.push(line);
          if (/;\s*$/.test(line)) {
            imports.push(pending.join('\n'));
            pending = null;
          }
          continue;
        }
        if (/^import\s/.test(line)) {
          if (/;\s*$/.test(line)) imports.push(line);
          else pending = [line];
          continue;
        }
        if (line.trim() === '' || line.trim().startsWith('//') || line.trim().startsWith('/*')) continue;
        break;
      }
      const readable = imports
        // Demo-only helpers and assets are not part of the installed component.
        .filter((line) => !/from\s+["']\.\/ui["']|from\s+["']\.\/bridge["']|\/assets\//.test(line))
        // The demo imports `../components/x`; an installed app reaches the same
        // file as `./x` beside it.
        .map((line) => line.replace(/from\s+["']\.\.\/components\//g, 'from "./'));
      if (readable.length > 0) {
        children.splice(children.indexOf(importHeading) + 1, 0, nshCode(readable.join('\n')));
      }
    }

    let anchor = null;
    // Which page heading each repeated title has already claimed, and the last
    // heading the plugin wrote for it, so several sections under one title stay
    // in the demo's order rather than stacking back to front.
    const claims = new Map();
    const foreignOnly = (node) => {
      const only = node.data?.hProperties?.['data-framework-only'];
      return only && only !== 'nsh';
    };
    for (const [index, section] of sections.entries()) {
      // A heading that belongs to another framework is not a home for the NSH
      // sample: the stylesheet hides it for every other framework, including
      // nsh. Leave it alone and write an nsh-only heading below.
      const heading = [...byTitle]
        .filter(([title]) => sameNshTitle(title, section.title))
        .flatMap(([, nodes]) => nodes)
        .find((node) => !foreignOnly(node));
      if (heading && !claims.has(heading)) {
        claims.set(heading, { count: 1, tail: null });
        tagNshHeading(heading, section.title);
        tagNshIndex(heading, index);
        const at = children.indexOf(heading) + 1;
        children.splice(at, 0, nshCode(section.code));
        anchor = children[at];
        continue;
      }
      if (heading) {
        // The page heading is taken. The repeated sample gets a heading of its
        // own, after the page's own sample so the panels read in demo order.
        const claim = claims.get(heading);
        claim.count += 1;
        const total = sections.filter((item) => sameNshTitle(item.title, section.title)).length;
        const created = nshDuplicateHeading(section.title, `${claim.count} of ${total}`, index);
        const from = claim.tail ?? heading;
        children.splice(sectionEnd(children, from), 0, created, nshCode(section.code));
        claim.tail = created;
        continue;
      }
      const at = anchor ? children.indexOf(anchor) + 1 : usageEnd(children);
      // The page has no heading for this section, so the plugin writes one and
      // the sample lives under it. It carries the section title the way a
      // heading the page already had does, so the card's code icon has the
      // sample as a target of its own.
      const created = nshOnlyHeading(section.title);
      tagNshIndex(created, index);
      const nodes = [created, nshCode(section.code)];
      children.splice(at, 0, ...nodes);
      anchor = nodes[1];
    }
  };
}
