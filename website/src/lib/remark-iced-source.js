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
