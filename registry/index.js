#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';
import https from 'node:https';
import { fileURLToPath } from 'node:url';

const REGISTRY_URL =
  'https://raw.githubusercontent.com/MohsenDastaran/uni-kit/main/registry/components.json';
const RAW_REPO_BASE = 'https://raw.githubusercontent.com/MohsenDastaran/uni-kit/main';

const here = path.dirname(fileURLToPath(import.meta.url));
const localRegistry = path.resolve(here, 'components.json');
const localRoot = path.resolve(here, '..');

function fetchBuffer(url) {
  return new Promise((resolve, reject) => {
    const request = https.get(url, (response) => {
      if (response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
        response.resume();
        resolve(fetchBuffer(response.headers.location));
        return;
      }
      if (response.statusCode !== 200) {
        response.resume();
        reject(new Error(`HTTP ${response.statusCode} for ${url}`));
        return;
      }
      const chunks = [];
      response.on('data', (chunk) => chunks.push(chunk));
      response.on('end', () => resolve(Buffer.concat(chunks)));
    });
    request.on('error', reject);
  });
}

async function loadRegistry() {
  if (process.env.UNI_KIT_REGISTRY) {
    const given = process.env.UNI_KIT_REGISTRY;
    if (given.startsWith('http://') || given.startsWith('https://')) {
      return JSON.parse((await fetchBuffer(given)).toString('utf8'));
    }
    return JSON.parse(fs.readFileSync(given, 'utf8'));
  }
  if (fs.existsSync(localRegistry)) {
    return JSON.parse(fs.readFileSync(localRegistry, 'utf8'));
  }
  return JSON.parse((await fetchBuffer(REGISTRY_URL)).toString('utf8'));
}

// Reads one source and returns its bytes. Nothing is written here: the caller
// collects every file first, so a missing source stops the install instead of
// leaving half a screen behind.
async function readSource(item) {
  const root = process.env.UNI_KIT_ROOT;
  if (root) return fs.readFileSync(path.join(root, item.src));
  if (fs.existsSync(localRegistry)) return fs.readFileSync(path.join(localRoot, item.src));
  const base = process.env.UNI_KIT_RAW ?? RAW_REPO_BASE;
  return await fetchBuffer(`${base}/${item.src}`);
}

function ensureModule(modFile, moduleName) {
  if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(moduleName)) {
    throw new Error(`Invalid module name "${moduleName}"`);
  }
  const line = `pub mod ${moduleName};`;
  fs.mkdirSync(path.dirname(modFile), { recursive: true });
  const text = fs.existsSync(modFile) ? fs.readFileSync(modFile, 'utf8') : '';
  const already = new RegExp(`^\\s*(pub\\s+)?mod\\s+${moduleName}\\s*;`, 'm').test(text);
  if (already) return false;
  const next = text.length === 0 || text.endsWith('\n') ? `${text}${line}\n` : `${text}\n${line}\n`;
  fs.writeFileSync(modFile, next);
  return true;
}

function buildScriptPath() {
  const cargo = path.join(process.cwd(), 'Cargo.toml');
  if (fs.existsSync(cargo)) {
    const text = fs.readFileSync(cargo, 'utf8');
    const match = text.match(/^\s*build\s*=\s*"([^"]+)"/m);
    if (match) return path.resolve(process.cwd(), match[1]);
  }
  return path.join(process.cwd(), 'build.rs');
}

// Components import each other by file name ("theme.slint"). Those names only
// resolve from a .slint file that lives next to them, so the app's own files
// need the install directory on the Slint include path.
function ensureSlintIncludePath(includeDir) {
  const buildRs = buildScriptPath();
  if (!fs.existsSync(buildRs)) {
    console.log(`  Add "${includeDir}" to the Slint include path so imports like "alert-dialog.slint" resolve.`);
    return;
  }
  const text = fs.readFileSync(buildRs, 'utf8');
  if (text.includes(includeDir)) return;
  const simple = /slint_build::compile\(([^)]*)\)/;
  if (!simple.test(text) || text.includes('compile_with_config')) {
    console.log(`  Add "${includeDir}" to the Slint include path so imports like "alert-dialog.slint" resolve.`);
    return;
  }
  const next = text.replace(
    simple,
    `slint_build::compile_with_config(
        $1,
        slint_build::CompilerConfiguration::new()
            .with_include_paths(vec![std::path::PathBuf::from("${includeDir}")]),
    )`,
  );
  fs.writeFileSync(buildRs, next);
  console.log(`  ${path.relative(process.cwd(), buildRs)} searches ${includeDir} for component imports`);
}

function usage() {
  console.log('Usage: uni-kit add <framework> <component...> [--dir <path>]');
  console.log('Examples:');
  console.log('  npx uni-kit add slint alert-dialog');
  console.log('  npx uni-kit add egui alert-dialog');
  console.log('  npx uni-kit add gpui button');
  console.log('  npx uni-kit add quickgui button');
  console.log('');
  console.log('A screen is usually several components. Name them all and files they');
  console.log('share are written once:');
  console.log('  npx uni-kit add slint sidebar card button table');
}

async function main() {
  const args = process.argv.slice(2);
  if (args[0] === '--help' || args[0] === '-h' || args.length === 0) {
    usage();
    process.exit(args.length === 0 ? 1 : 0);
  }

  const command = args[0];
  const framework = args[1];
  const dirFlag = args.indexOf('--dir');
  const dirOverride = dirFlag === -1 ? undefined : args[dirFlag + 1];
  // Everything after the framework that is not the `--dir` flag or its value.
  const components = args
    .slice(2)
    .filter(
      (value, index, all) =>
        value !== '--dir' && all[index - 1] !== '--dir' && !value.startsWith('--'),
    );

  if (
    command !== 'add' ||
    !framework ||
    components.length === 0 ||
    (dirFlag !== -1 && !dirOverride)
  ) {
    usage();
    process.exit(1);
  }

  const registry = await loadRegistry();
  const frameworkConfig = registry[framework];
  if (!frameworkConfig) {
    console.error(`Framework "${framework}" is not in the registry.`);
    process.exit(1);
  }

  // Validate before writing anything, so a typo in the last name cannot leave
  // half a screen installed.
  for (const component of components) {
    const files = frameworkConfig.components?.[component];
    if (!files || files.length === 0) {
      console.error(`Component "${component}" for ${framework} is not in the registry yet.`);
      process.exit(1);
    }
  }

  const baseTargetDir = dirOverride ?? frameworkConfig.default_target_dir;
  if (!baseTargetDir) {
    console.error(`Framework "${framework}" has no default_target_dir.`);
    process.exit(1);
  }

  // `path.resolve` reads an absolute --dir as itself and a relative one as
  // relative to the app. Joining the raw value under the cwd turned
  // `--dir /tmp/app` into `<cwd>/tmp/app`.
  const targetRoot = path.resolve(process.cwd(), baseTargetDir);

  console.log(`Installing ${components.join(', ')} for ${framework}...`);
  const modules = new Set();
  // Components share files — every one of them imports the theme, and many
  // import the icon set. Written once, whichever name reached it first.
  const written = new Set();
  const plan = [];
  for (const component of components) {
    for (const item of frameworkConfig.components[component]) {
      if (item.target.includes('..') || path.isAbsolute(item.target)) {
        throw new Error(`Refusing to write outside the target directory: ${item.target}`);
      }
      if (written.has(item.target)) continue;
      written.add(item.target);
      plan.push({ item, destination: path.join(targetRoot, item.target) });
      if (item.module) modules.add(item.module);
    }
  }

  // Read every source before writing any of it. A component pulls in the icon
  // set, so a screen is well over a hundred files, and a failure in the middle
  // of that used to leave a half-installed screen with no indication of which
  // files were missing.
  const contents = [];
  for (const { item, destination } of plan) {
    let data;
    try {
      data = await readSource(item);
    } catch (error) {
      throw new Error(`Could not read ${item.src}: ${error.message}`);
    }
    contents.push({ destination, data });
  }

  for (const { destination, data } of contents) {
    fs.mkdirSync(path.dirname(destination), { recursive: true });
    fs.writeFileSync(destination, data);
    console.log(`  ${path.relative(process.cwd(), destination)}`);
  }

  if (modules.size > 0) {
    const modFile = path.resolve(process.cwd(), frameworkConfig.mod_file ?? path.join(baseTargetDir, 'mod.rs'));
    for (const moduleName of modules) {
      const added = ensureModule(modFile, moduleName);
      if (added) console.log(`  ${path.relative(process.cwd(), modFile)} += pub mod ${moduleName};`);
    }
  }

  if (framework === 'slint') ensureSlintIncludePath(baseTargetDir);

  console.log(`Installed ${components.join(', ')} into ${baseTargetDir}/`);
}

main().catch((error) => {
  console.error(error.message);
  process.exit(1);
});
