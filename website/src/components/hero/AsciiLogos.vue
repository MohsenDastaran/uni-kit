<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue";

/**
 * The ascii.rest "rust" piece, ported verbatim from its source, extended to
 * cycle rust -> gpui -> slint. The glint -- a slash, leaning like a LEAN-skewed
 * band, crossing the ink every SHINE seconds -- is copied exactly: same
 * START/PASS/HALF/LEAN, same SOLID set, same smoothstep falloff.
 *
 * Rust is the exact art. gpui and slint are wordmarks drawn through a 5x7 block
 * font, because ascii.rest has no such pieces and their logos have no ASCII
 * source to copy.
 */

const COLS = 64;
const ROWS = 32;

// ── exact rust art ────────────────────────────────────────────────────────
const RUST = String.raw`
                         qq_  _8q,  _p,
                   .pq_,_888qp8888qp888,__pq
               p__ q88888888888P888888888888, _pq
              '888888888888888,  )888888888888888
          \qqqp888888888P""'"88qp88"'""Y888888888qppq,
          "888888888P"        "8P"        "Y888888888'
       pqqp888888P"                          "8888888qqqp
       888888888qq________________________     "88888888P
     .__88888888888888888888888888888888888q_    "888888__
    d8888888888888888888888888888888888888888q,   |88888888P
     Y8888888888888888888888888888888888888888b  .88888888"
  ._pp888   88p  8888888888        '"8888888888  p8|  |888qq_
  "888888qqp88P  8888888888        __888888888" '888qpp88888P'
    "88888P"'    888888888888888888888888888P'    '"Y888888"
  _p888888       88888888888888888888888888_         '888888q_
  "8888888       888888888888888888888888888q,     ___888888P"
    _88888,      8888888888      '"8888888888p     88888888_
  \p888888p......8888888888        '8888888888__._p888888888q,
   "Y888888888888888888888888888p   )888888888888888888888PY"
     q88888888888888888888888888b   '888888888888888888888p
    d888888888888888888888888888P    "888888888888888888888b
      '"888888P""""""""""""""""""      """"""""""8888888"'
       q8888888qpppqqp                    pqqqqp88888888p
       "^^Y888888P""88,                  q88""8888888Y^^"
          |888888_ _d8p                  88(  _888888,
          /8P^"888888888qqq__________ppq888888888"YP8"
              .8888888888888888888888888888888888
               ""' )888888888888888888888888| """
                   '8P"'"888PY8888P8888"'"88
                         "P"  "88"  "8"
`;

// ── 5x7 block font, so gpui and slint render as clean wordmarks ───────────
const GLYPH: Record<string, string[]> = {
  g: ["####.", "#...#", "#...#", ".####", "#...#", "#...#", ".####"],
  p: ["####.", "#...#", "#...#", "####.", "#....", "#....", "#...."],
  u: ["#...#", "#...#", "#...#", "#...#", "#...#", "#...#", ".####"],
  i: ["..#..", "..#..", "..#..", "..#..", "..#..", "..#..", "..#.."],
  s: [".####", "#....", "#....", ".####", "....#", "....#", "####."],
  l: ["#....", "#....", "#....", "#....", "#....", "#....", ".####"],
  n: ["#...#", "##..#", "#.#.#", "#..##", "#...#", "#...#", "#...#"],
  t: ["####.", "..#..", "..#..", "..#..", "..#..", "..#..", "..#.."],
};

function wordmark(word: string, scale: number): string {
  const letterW = 5 * scale;
  const gap = 1 * scale;
  const width = word.length * letterW + (word.length - 1) * gap;
  const height = 7 * scale;
  const ox = Math.floor((COLS - width) / 2);
  const oy = Math.floor((ROWS - height) / 2);
  const grid = Array.from({ length: ROWS }, () => " ".repeat(COLS));
  let cursor = ox;
  for (const ch of word) {
    const glyph = GLYPH[ch];
    for (let r = 0; r < 7; r++) {
      for (let c = 0; c < 5; c++) {
        if (glyph[r][c] === "#") {
          for (let dy = 0; dy < scale; dy++) {
            for (let dx = 0; dx < scale; dx++) {
              const y = oy + r * scale + dy;
              const x = cursor + c * scale + dx;
              grid[y] = grid[y].slice(0, x) + "8" + grid[y].slice(x + 1);
            }
          }
        }
      }
    }
    cursor += letterW + gap;
  }
  return "\n" + grid.join("\n") + "\n";
}

const pieces = [
  { name: "rust", mono: RUST },
  { name: "gpui", mono: wordmark("gpui", 2) },
  { name: "slint", mono: wordmark("slint", 2) },
];

// ── the glint, exactly as rust.ts authors it ──────────────────────────────
const START = 0.5;
const PASS = 2;
const HALF = 5;
const LEAN = 0.9;
const SOLID = "8dbqpPYOo0";
const SHINE = 5;

function frameFor(mono: string) {
  const pic = mono
    .slice(1, -1)
    .split("\n")
    .map((line) => line.padEnd(COLS));
  let lo = Infinity;
  let hi = -Infinity;
  pic.forEach((line, y) => {
    for (let x = 0; x < COLS; x++) {
      if (line[x] !== " ") {
        lo = Math.min(lo, x + LEAN * y);
        hi = Math.max(hi, x + LEAN * y);
      }
    }
  });
  const span = hi - lo + 2 * HALF;
  const every = Math.max(SHINE, PASS + 0.5);

  return (t: number) => {
    const since = t - START;
    const at = since >= 0 ? lo - HALF + (span * (since % every)) / PASS : -Infinity;
    const out: string[] = [];
    for (let y = 0; y < pic.length; y++) {
      let line = "";
      for (let x = 0; x < COLS; x++) {
        let ch = pic[y][x];
        if (ch !== " ") {
          const d = Math.abs(x + LEAN * y - at);
          let k = d < HALF ? 1 - d / HALF : 0;
          k = k * k * (3 - 2 * k);
          if (k > 0.55 && SOLID.includes(ch)) ch = "/";
        }
        line += ch;
      }
      out.push(line);
    }
    return out.join("\n");
  };
}

const pre = ref<HTMLElement>();
const current = ref(0);
let render: (t: number) => string = frameFor(pieces[0].mono);
let raf = 0;
let startedAt = 0;
let reducedMotion = false;
let fadeTimer: ReturnType<typeof setTimeout> | undefined;
let rotateTimer: ReturnType<typeof setInterval> | undefined;

function now() {
  return (performance.now() - startedAt) / 1000;
}

function tick() {
  if (pre.value) pre.value.textContent = render(now());
  if (!reducedMotion) raf = requestAnimationFrame(tick);
}

onMounted(() => {
  reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  startedAt = performance.now();
  if (pre.value) pre.value.textContent = render(reducedMotion ? 0 : now());
  if (reducedMotion) return;

  raf = requestAnimationFrame(tick);
  rotateTimer = setInterval(() => {
    const next = (current.value + 1) % pieces.length;
    if (pre.value) pre.value.style.opacity = "0";
    fadeTimer = setTimeout(() => {
      current.value = next;
      render = frameFor(pieces[next].mono);
      if (pre.value) {
        pre.value.textContent = render(now());
        pre.value.style.opacity = "1";
      }
    }, 320);
  }, 6000);
});

onBeforeUnmount(() => {
  cancelAnimationFrame(raf);
  clearInterval(rotateTimer);
  clearTimeout(fadeTimer);
});
</script>

<template>
  <pre ref="pre" class="ascii-logos" aria-hidden="true"></pre>
</template>
