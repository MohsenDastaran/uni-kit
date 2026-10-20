// Copies the NSH catalogue into the npm package before `npm pack`/`publish`.
// The CLI resolves every source from this tree, so an installed command works
// even while the NSH branch is ahead of `main`. `postpack` removes it again.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const repo = path.resolve(here, '..');
const out = path.join(here, 'sources');
const components = path.join(repo, 'crates/nsh-component/src/components');
const demos = path.join(repo, 'crates/nsh-component/src/demos');

if (!fs.existsSync(components)) {
  throw new Error(`Expected NSH components at ${components}`);
}
fs.rmSync(out, { recursive: true, force: true });
fs.cpSync(components, path.join(out, 'crates/nsh-component/src/components'), { recursive: true });
for (const file of fs.readdirSync(demos).filter((name) => name.endsWith('_block.tsx'))) {
  const target = path.join(out, 'crates/nsh-component/src/demos', file);
  fs.mkdirSync(path.dirname(target), { recursive: true });
  fs.copyFileSync(path.join(demos, file), target);
}
console.log('packaged NSH sources for the registry');
