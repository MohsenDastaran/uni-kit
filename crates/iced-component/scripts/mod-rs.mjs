// Generates `src/components/mod.rs` from the directory it declares.
//
// A hand-written module list is the one file every batch of components would
// otherwise have to edit, which serialises the work and — worse — goes stale
// silently: the crate builds, compiles only the modules the list happens to
// name, and every check passes on a tree where half the components are not being
// compiled at all. That happened once already.
//
//   node crates/iced-component/scripts/mod-rs.mjs           # write it
//   node crates/iced-component/scripts/mod-rs.mjs --check    # fail if stale
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const components = path.join(here, '..', 'src', 'components');

const modules = fs
  .readdirSync(components)
  .filter((name) => name.endsWith('.rs') && name !== 'mod.rs' && name !== 'theme.rs')
  .map((name) => name.replace(/\.rs$/, ''))
  .sort();

const text = `//! The components. One file each, named after the registry key with hyphens as
//! underscores, and this list is generated from the directory by
//! \`scripts/mod-rs.mjs\` — never edited by hand, because a module missing from it
//! is a component the crate silently does not compile.
//!
//! \`theme\` is shared: the CLI writes it once however many components are
//! installed, and a component reads it as \`super::theme\`, which keeps the
//! installed directory movable.

pub mod theme;

${modules.map((name) => `pub mod ${name};`).join('\n')}
`;

const file = path.join(components, 'mod.rs');
const current = fs.existsSync(file) ? fs.readFileSync(file, 'utf8') : '';

if (process.argv.includes('--check')) {
  if (current === text) {
    console.log(`mod.rs declares all ${modules.length} components`);
    process.exit(0);
  }
  console.error('src/components/mod.rs is stale — run scripts/mod-rs.mjs');
  process.exit(1);
}

if (current === text) {
  console.log(`mod.rs already declares all ${modules.length} components`);
} else {
  fs.writeFileSync(file, text);
  console.log(`mod.rs now declares ${modules.length} components`);
}
