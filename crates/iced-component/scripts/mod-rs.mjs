// Generates the module lists that have to follow the directory:
// `src/components/mod.rs` and `src/demos/mod.rs`.
//
// A hand-written list is the one file every batch of components would otherwise
// have to edit, which serialises the work and — worse — goes stale silently: the
// crate builds, compiles only the modules the list happens to name, and every
// check passes on a tree where half the components are not being compiled at all.
// That happened once already.
//
// The demos list carries the gallery's dispatcher for the same reason: a demo
// that exists is a demo `?story=` can name, without anybody maintaining a second
// list by hand.
//
//   node crates/iced-component/scripts/mod-rs.mjs           # write them
//   node crates/iced-component/scripts/mod-rs.mjs --check    # fail if stale
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const src = path.join(here, '..', 'src');
const componentsDir = path.join(src, 'components');
const demosDir = path.join(src, 'demos');

/** The module names of a directory's `.rs` files, sorted, minus the given ones. */
function modules(dir, excluded = []) {
  return fs
    .readdirSync(dir)
    .filter((name) => name.endsWith('.rs') && name !== 'mod.rs' && !excluded.includes(name))
    .map((name) => name.replace(/\.rs$/, ''))
    .sort();
}

const components = modules(componentsDir, ['theme.rs']);
const demos = modules(demosDir, ['index.rs']);

const componentsMod = `//! The components. One file each, named after the registry key with hyphens as
//! underscores, and this list is generated from the directory by
//! \`scripts/mod-rs.mjs\` — never edited by hand, because a module missing from it
//! is a component the crate silently does not compile.
//!
//! \`theme\` is shared: the CLI writes it once however many components are
//! installed, and a component reads it as \`super::theme\`, which keeps the
//! installed directory movable.

pub mod theme;

${components.map((name) => `pub mod ${name};`).join('\n')}
`;

const story = (name) => name.replace(/_/g, '-');
const demoArms = demos
  .map((name) => `        "${story(name)}" => ${name}::run(theme),`)
  .join('\n');
const stories = demos.map((name) => `    "${story(name)}",`).join('\n');

const demosMod = `//! The demos the gallery shows: one module each, named after the registry key
//! with hyphens as underscores, and this list is generated from the directory by
//! \`scripts/mod-rs.mjs\` — never edited by hand.
//!
//! A demo is the whole application, not a view: each one calls
//! \`iced::application\` itself, so the native binary in \`examples/\` and the
//! browser gallery run the same code rather than two copies that drift apart.

${demos.map((name) => `pub mod ${name};`).join('\n')}

mod index;

/// Every story the gallery can show, named the way the website names it: the
/// registry key, hyphenated.
pub const STORIES: &[&str] = &[
${stories}
];

/// Run the demo a story names, with the theme the page asked for.
///
/// An unknown story is not an error. The gallery is also how somebody finds out
/// what exists, so a name that matches nothing shows the list rather than a blank
/// window or a panic.
pub fn run(story: &str, theme: iced::Theme) -> iced::Result {
    match story {
${demoArms}
        other => index::run(theme, other.to_string()),
    }
}
`;

const files = [
  [path.join(componentsDir, 'mod.rs'), componentsMod],
  [path.join(demosDir, 'mod.rs'), demosMod],
];

if (process.argv.includes('--check')) {
  const stale = files.filter(
    ([file, text]) => !fs.existsSync(file) || fs.readFileSync(file, 'utf8') !== text,
  );
  if (stale.length === 0) {
    console.log(
      `module lists name all ${components.length} components and ${demos.length} demos`,
    );
    process.exit(0);
  }
  console.error(
    `stale: ${stale.map(([file]) => path.relative(process.cwd(), file)).join(', ')} — run scripts/mod-rs.mjs`,
  );
  process.exit(1);
}

for (const [file, text] of files) {
  const current = fs.existsSync(file) ? fs.readFileSync(file, 'utf8') : '';
  if (current === text) continue;
  fs.writeFileSync(file, text);
}
console.log(`module lists now name ${components.length} components and ${demos.length} demos`);
