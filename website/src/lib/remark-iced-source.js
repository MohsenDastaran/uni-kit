import { existsSync, readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

// iced is added beside GPUI and Slint: one Rust source per component in
// `src/components/`, with a usage example in `examples/`, which is also where
// the CLI installs them from. The site asks this module the same question it
// asks `remark-slint-source.js` for Slint — does this page have a view at all? —
// and the answer is read from the sources rather than from the build, so a dev
// server and a published site agree.
//
// A registry key may hold a hyphen and a Rust module may not, so the file for
// `message-role` is `message_role.rs`. That mapping is the one the manifest
// builder and the checklist use.

export const icedRoot = (root = process.cwd()) =>
  resolve(root, '..', 'crates', 'iced-component');

/** The file a registry key lives in, hyphenated key to underscored module. */
export const icedModule = (slug) => slug.replace(/-/g, '_');

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
    // Either an application written here, or a thin `main` running the demo the
    // library holds — the demos moved there so the browser gallery can show the
    // same code the example runs.
    return (
      /fn main\s*\(/.test(code) &&
      (/iced::(application|run)\s*\(/.test(code) || /demos::[a-z_]+::run\s*\(/.test(code))
    );
  }
  return /pub fn/.test(code) && (/Element/.test(code) || path.endsWith('theme.rs'));
}

/**
 * Whether this slug has an iced example to show, and one worth showing. An
 * empty file, a file holding only comments, and a stub that says it is pending
 * all answer no, which is the same reading the checklist uses.
 */
export function hasIcedExample(slug, root = process.cwd()) {
  return implemented(
    join(icedRoot(root), 'examples', `${icedModule(slug)}.rs`),
    'example',
  );
}

/** Whether this slug has an iced component at all, stub or not. */
export function hasIcedComponent(slug, root = process.cwd()) {
  return implemented(
    join(icedRoot(root), 'src', 'components', `${icedModule(slug)}.rs`),
    'component',
  );
}

/**
 * The site has an iced view of this page at all: an iced example to show, on a
 * page that shows one. `example: false` is a page with no example window — a
 * guide — so the iced sidebar has nothing to put beside it and the framework
 * switch has nothing to switch to. The same question is asked of Slint in
 * `remark-slint-source.js`.
 */
export function hasIcedPage(slug, example, root = process.cwd()) {
  // Both halves, because either alone claims more than exists: a component with
  // no example has no view to show, and an example with no component installs
  // nothing.
  return example !== false && hasIcedComponent(slug, root) && hasIcedExample(slug, root);
}

// ---------------------------------------------------------------------------
// The plugin: the iced panel under each usage heading.
//
// The Slint page's samples are cut out of the example file, so what a component
// page shows is the code the gallery runs rather than a second copy of it. A
// Rust file cannot be parsed the way a `.slint` one can, so the iced demos mark
// their sections instead:
//
//     // section: Variants
//     let variants = column![…];
//
// That one comment is the contract. Everything from it to the next marker is the
// section's sample — the same lines the browser runs — and the marker itself is
// not shown.

/** The registry key a page's markdown file belongs to, or nothing. */
function icedSlugOf(file) {
  const path = String(file?.path ?? file?.history?.[0] ?? '');
  const match = /[\\/]component[\\/]([^\\/]+)\.md$/.exec(path);
  return match?.[1] ?? null;
}

/** The same reading the Slint plugin uses: case and punctuation do not matter. */
function sameIcedTitle(a, b) {
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
export function icedSections(slug, root = process.cwd()) {
  const path = join(icedRoot(root), 'src', 'demos', `${icedModule(slug)}.rs`);
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

// The code is Rust, and stays Rust: the site's Shiki registers `rust` and a
// custom `slint` grammar, and a fence in a language it does not know loses the
// whole block rather than falling back. The framework rides on an attribute
// instead, which `rehype-framework-code` reads first.
const icedCode = (value) => ({
  type: 'code',
  // The language is the only thing about a code block that survives the trip to
  // HTML — a `data-*` property set here does not — so iced has a language name,
  // and `src/lib/markdown.js` aliases it to Rust for the highlighter.
  lang: 'iced',
  meta: null,
  value,
  data: { hProperties: { 'data-gallery-framework': 'iced' } },
});

/** The gallery posts the section title; the heading that received the sample
 *  keeps it, so a click scrolls to the sample rather than to whichever block
 *  shares an index. */
function tagIcedHeading(node, title) {
  const data = (node.data ??= {});
  const props = (data.hProperties ??= {});
  const key = 'data-gallery-title';
  const current = String(props[key] ?? '')
    .split('|')
    .filter(Boolean);
  if (current.some((item) => sameIcedTitle(item, title))) return;
  current.push(title);
  props[key] = current.join('|');
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

function icedHeading(title) {
  return {
    type: 'heading',
    depth: 3,
    children: [{ type: 'text', value: title }],
  };
}

/**
 * The heading the plugin writes for a section the page does not have. It is
 * iced's own: the other frameworks leave it out, and the lookup skips it there,
 * the same reading the framework pass gives a heading whose section holds only
 * iced's sample. A created heading whose sample joins the page's own code under
 * it does not get that mark from the pass, so it is said here.
 */
function icedOnlyHeading(title) {
  const node = icedHeading(title);
  const data = (node.data ??= {});
  const props = (data.hProperties ??= {});
  props['data-framework-only'] = 'iced';
  tagIcedHeading(node, title);
  return node;
}

/**
 * Where a section the page has no heading for goes: inside `## Usage`, at the end
 * of it, so an added section still reads as part of the examples.
 */
function usageEnd(children) {
  const start = children.findIndex(
    (node) => node.type === 'heading' && /^usage$/i.test(headingText(node)),
  );
  if (start === -1) return children.length;
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
export function remarkIcedSource() {
  return (tree, file) => {
    const slug = icedSlugOf(file);
    if (!slug || !hasIcedPage(slug, true)) return;
    const sections = icedSections(slug);
    if (sections.length === 0) return;

    const children = tree.children;
    const byTitle = new Map();
    for (const node of children) {
      const title = headingText(node);
      if (title) byTitle.set(title, node);
    }

    // The page's import fence is the GPUI one. A reader who has chosen iced needs
    // the lines they would actually write, and the demo has them: its own `use`
    // statements, which is what the rest of these samples assume.
    const importHeading = children.find(
      (node) => node.type === 'heading' && /^import$/i.test(headingText(node)),
    );
    if (importHeading) {
      const demo = readFileSync(
        join(icedRoot(), 'src', 'demos', `${icedModule(slug)}.rs`),
        'utf8',
      );
      const uses = [];
      for (const line of demo.split('\n')) {
        if (/^use /.test(line)) {
          uses.push(line);
          continue;
        }
        if (uses.length > 0 && line.trim() === '') break;
      }
      if (uses.length > 0) {
        children.splice(children.indexOf(importHeading) + 1, 0, icedCode(uses.join('\n')));
      }
    }

    let anchor = null;
    for (const section of sections) {
      const heading = [...byTitle].find(([title]) => sameIcedTitle(title, section.title))?.[1];
      if (heading) {
        tagIcedHeading(heading, section.title);
        const at = children.indexOf(heading) + 1;
        children.splice(at, 0, icedCode(section.code));
        anchor = children[at];
        continue;
      }
      const at = anchor ? children.indexOf(anchor) + 1 : usageEnd(children);
      // The page has no heading for this section, so the plugin writes one and
      // the sample lives under it. It carries the section title the way a
      // heading the page already had does, so the card's code icon has the
      // sample as a target of its own.
      const created = icedOnlyHeading(section.title);
      const nodes = [created, icedCode(section.code)];
      children.splice(at, 0, ...nodes);
      anchor = nodes[1];
    }
  };
}
