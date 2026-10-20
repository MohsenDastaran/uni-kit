#!/usr/bin/env node
// Vendor the Native Shell documentation for coding agents.
//
//   node script/vendor-nsh-docs.mjs
//   node script/vendor-nsh-docs.mjs --check   # verify every linked page exists
//
// NSH serves every documentation page as Markdown at its URL plus `.md`, and
// `llms.txt` lists them. The resource is committed so agents can read it
// offline and so a page cannot disappear from the repository unnoticed. The
// pages are third-party prose: they keep their upstream copyright and are not
// relicensed by this repository (see docs/nsh/README.md).
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const dest = path.join(root, 'docs/nsh');
const origin = 'https://nsh.dev';
const check = process.argv.includes('--check');

const urlOf = (file) => `${origin}/${file}`;
const localOf = (url) => {
  const parsed = new URL(url);
  if (parsed.origin !== origin) return null;
  return parsed.pathname.replace(/^\/+/, '');
};

const read = async (url) => {
  const response = await fetch(url, {
    headers: { 'user-agent': 'uni-kit docs vendor' },
    signal: AbortSignal.timeout(20_000),
  });
  if (!response.ok) throw new Error(`HTTP ${response.status} for ${url}`);
  return await response.text();
};

if (!check) fs.mkdirSync(dest, { recursive: true });

let llms;
try {
  llms = check
    ? fs.readFileSync(path.join(dest, 'llms.txt'), 'utf8')
    : await read(urlOf('llms.txt'));
} catch (error) {
  console.error(`nsh docs: ${error.message}`);
  process.exit(1);
}

const links = [...llms.matchAll(/\]\((https:\/\/nsh\.dev\/[^)\s]+\.md)\)/g)]
  .map((match) => match[1])
  .filter((url) => url.startsWith(origin));
const unique = [...new Set(links)].sort();

if (!check) {
  fs.writeFileSync(path.join(dest, 'llms.txt'), llms);
  const full = await read(urlOf('llms-full.txt'));
  fs.writeFileSync(path.join(dest, 'llms-full.txt'), full);
}

let written = 0;
let missing = 0;
const failures = [];

async function vendor(url) {
  const local = localOf(url);
  if (!local) return;
  const target = path.join(dest, local);
  if (check) {
    if (!fs.existsSync(target)) {
      console.error(`  missing ${local}`);
      missing += 1;
    }
    return;
  }
  try {
    const body = await read(url);
    fs.mkdirSync(path.dirname(target), { recursive: true });
    fs.writeFileSync(target, body);
    written += 1;
  } catch (error) {
    // A transient timeout on a page that is already vendored is not a reason to
    // fail the update; a page that has never arrived is.
    if (fs.existsSync(target)) console.warn(`nsh docs: kept the existing ${local} (${error.message})`);
    else failures.push(`${local}: ${error.message}`);
  }
}

// A small pool: the site answers each page quickly, and one slow request must
// not hold the other 180 forever.
for (let at = 0; at < unique.length; at += 8) {
  await Promise.all(unique.slice(at, at + 8).map(vendor));
}
if (failures.length > 0) {
  console.error(`nsh docs: ${failures.length} page(s) failed:`);
  for (const failure of failures) console.error(`  ${failure}`);
  process.exitCode = 1;
}

const source = {
  product: 'Native Shell',
  origin,
  entry: urlOf('llms.txt'),
  version: process.env.NSH_VERSION ?? 'v0.1.1',
  vendored: new Date().toISOString().slice(0, 10),
  pages: unique.length,
};
if (!check) fs.writeFileSync(path.join(dest, 'SOURCE.json'), `${JSON.stringify(source, null, 2)}\n`);

if (check) {
  if (missing > 0) {
    console.error(`nsh docs: ${missing} page(s) missing under docs/nsh`);
    process.exit(1);
  }
  console.log(`nsh docs: ${unique.length} linked pages present under docs/nsh`);
} else {
  console.log(`nsh docs: ${written} pages vendored into docs/nsh (${unique.length} links)`);
}
