#!/usr/bin/env node
/**
 * Pre-compress every gallery `.wasm` as `.wasm.gz`, once, at build time.
 *
 * The galleries are tens of megabytes each, and nginx deliberately does not
 * compress wasm on the fly — doing it per request costs more than it saves. A
 * `.wasm.gz` beside the module costs nothing at serve time: `gzip_static` hands
 * it over as-is, so the transfer shrinks ~2.5x while the module itself is
 * untouched. Brotli is smaller still, but the official nginx image has no brotli
 * module, so nothing would serve a `.wasm.br` yet.
 *
 * Run this after the galleries are built and before the site build: the site
 * reads these sizes and prints them in the pages that explain each download.
 * Files whose archives are already newer than the module are left alone, so a
 * rebuild that touches nothing compresses nothing.
 *
 * usage: node scripts/precompress-wasm.mjs <gallery-dist> [...]
 */
import { readFileSync, statSync, writeFileSync } from 'node:fs';
import { readdir } from 'node:fs/promises';
import { join } from 'node:path';
import { cpus } from 'node:os';
import { gzipSync } from 'node:zlib';
import { isMainThread, parentPort, Worker } from 'node:worker_threads';

if (!isMainThread) {
  // The worker half: compress one file per message and report both sizes.
  parentPort.on('message', (job) => {
    if (!job) {
      parentPort.postMessage({ idle: true });
      return;
    }
    try {
      const raw = readFileSync(job.file);
      const gz = gzipSync(raw, { level: 9 });
      writeFileSync(job.file + '.gz', gz);
      parentPort.postMessage({ file: job.file, gz: gz.length });
    } catch (error) {
      parentPort.postMessage({ error: error.message });
    }
  });
} else {
  const roots = process.argv.slice(2);
  if (roots.length === 0) {
    console.error('usage: node scripts/precompress-wasm.mjs <gallery-dist> [...]');
    process.exit(2);
  }

  /** Every .wasm under root that has no newer archives beside it. */
  const collect = async (root) => {
    const found = [];
    const walk = async (directory) => {
      for (const entry of await readdir(directory, { withFileTypes: true })) {
        const file = join(directory, entry.name);
        if (entry.isDirectory()) await walk(file);
        else if (entry.name.endsWith('.wasm')) {
          let current = true;
          for (const variant of [file + '.gz']) {
            try {
              if (statSync(variant).mtimeMs >= statSync(file).mtimeMs) continue;
            } catch {
              // No archive yet: compress.
            }
            current = false;
          }
          if (!current) found.push({ file, raw: statSync(file).size });
        }
      }
    };
    await walk(root);
    return found;
  };

  const jobs = [];
  for (const root of roots) jobs.push(...(await collect(root)));

  if (jobs.length === 0) {
    console.log('precompress-wasm: nothing to compress');
    process.exit(0);
  }

  const bytes = (n) => `${(n / 1024 / 1024).toFixed(1)} MB`;
  const rawTotal = jobs.reduce((sum, job) => sum + job.raw, 0);
  console.log(`precompress-wasm: ${jobs.length} modules, ${bytes(rawTotal)}`);

  // gzip -9 keeps ~2x the file in memory per worker; a handful of threads is
  // enough and leaves the rest of the machine alone.
  const threads = Math.min(cpus().length, jobs.length, 4);
  let cursor = 0;
  let done = 0;
  let gzTotal = 0;

  const workers = Array.from({ length: threads }, () => {
    const worker = new Worker(new URL(import.meta.url), { workerData: 'worker' });
    worker.on('message', (message) => {
      if (message.error) {
        console.error(`precompress-wasm: ${message.error}`);
        process.exit(1);
      }
      if (message.gz) {
        gzTotal += message.gz;
        done += 1;
        if (done === jobs.length) {
          console.log(
            `precompress-wasm: done, ${bytes(rawTotal)} → ${bytes(gzTotal)} (${(rawTotal / gzTotal).toFixed(1)}x)`,
          );
          for (const other of workers) other.terminate();
          process.exit(0);
        }
      }
      if (cursor < jobs.length) worker.postMessage(jobs[cursor++]);
    });
    worker.on('error', (error) => {
      console.error(`precompress-wasm: ${error.message}`);
      process.exit(1);
    });
    return worker;
  });

  for (const worker of workers) worker.postMessage(jobs[cursor++]);
}
