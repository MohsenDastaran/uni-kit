import { createHash } from 'node:crypto';
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { isAbsolute, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { glob, type Loader } from 'astro/loaders';

// Component pages are rendered once and cached by the markdown digest. What a
// page shows is not written in the markdown: the Slint samples are cut from
// ui/examples and the iced panels from src/demos, so a change in either would
// stay invisible until the markdown itself changed. This loader drops that
// cache when anything under those samples changes, then renders the pages
// again.

const repoRoot = resolve(fileURLToPath(new URL('.', import.meta.url)), '../../..');

// One entry per framework in registry/components.json: the root its samples are
// read from, and the extensions worth hashing. A framework added there is added
// here, and nothing below needs to learn its name.
const sampleSources = [
  { dir: join(repoRoot, 'crates/slint-component/ui/examples'), extensions: ['.slint'] },
  { dir: join(repoRoot, 'crates/iced-component/src/demos'), extensions: ['.rs'] },
  { dir: join(repoRoot, 'crates/nsh-component/src/demos'), extensions: ['.tsx'] },
];

function samplesStamp() {
  const hash = createHash('sha1');
  for (const source of sampleSources) {
    const walk = (dir: string) => {
      if (!existsSync(dir)) return;
      for (const entry of readdirSync(dir, { withFileTypes: true }).sort((a, b) => a.name.localeCompare(b.name))) {
        const path = join(dir, entry.name);
        if (entry.isDirectory()) walk(path);
        else if (source.extensions.some((extension) => entry.name.endsWith(extension))) {
          hash.update(relative(repoRoot, path));
          hash.update(readFileSync(path));
        }
      }
    };
    walk(source.dir);
  }
  return hash.digest('hex');
}

/** Whether a watched path is a file the stamp covers. */
function isSampleFile(path: string) {
  return sampleSources.some((source) => {
    const inside = relative(source.dir, path);
    return (
      inside !== '' &&
      !inside.startsWith('..') &&
      !isAbsolute(inside) &&
      source.extensions.some((extension) => inside.endsWith(extension))
    );
  });
}

export function componentMarkdownLoader(base: string): Loader {
  const inner = glob({ pattern: '**/*.md', base });
  let timer: ReturnType<typeof setTimeout> | undefined;

  return {
    name: `component-markdown (${base})`,
    async load(context) {
      const next = samplesStamp();
      if (context.meta.get('component-samples') !== next) {
        context.store.clear();
        context.meta.set('component-samples', next);
      }
      await inner.load(context);

      const watcher = context.watcher;
      if (!watcher) return;
      for (const source of sampleSources) {
        if (existsSync(source.dir)) watcher.add(source.dir);
      }
      const reload = () => {
        clearTimeout(timer);
        timer = setTimeout(async () => {
          const updated = samplesStamp();
          if (context.meta.get('component-samples') === updated) return;
          context.store.clear();
          context.meta.set('component-samples', updated);
          await inner.load(context);
        }, 200);
      };
      const onSample = (changedPath: string) => {
        if (isSampleFile(changedPath)) reload();
      };
      watcher.on('add', onSample);
      watcher.on('change', onSample);
      watcher.on('unlink', onSample);
    },
  };
}
