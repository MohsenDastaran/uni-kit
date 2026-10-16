<script setup lang="ts">
// Animated table of contents: a single SVG rail snakes through the
// indentation levels, and a dot glides along the curve to the active entry
// while the traveled portion of the path darkens.
// Inspired by the "Clean table of contents" by @AliGrids / @raul_dronca.
//
// The entries are not passed in. The docs page renders every framework's
// sections into one document and hides the ones the selected framework has no
// sample for, so the outline is derived from the headings that are actually
// laid out under the framework in force (see `src/lib/toc.ts`). `framework-change`
// and the late mounting of the live example rebuild it, which keeps every entry
// pointing at a section the reader can see.
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import { TOC_PINS, sameTocItems, tocItems, type TocHeading, type TocItem } from '../lib/toc';

const props = withDefaults(
  defineProps<{
    rowHeight?: number;
    indent?: number;
    title?: string;
  }>(),
  {
    rowHeight: 48,
    indent: 24,
    title: 'On this page',
  },
);

const items = ref<TocItem[]>([]);
const active = ref(0);

const navEl = ref<HTMLElement | null>(null);
const listEl = ref<HTMLUListElement | null>(null);

const RAIL_X = 8;
const CORNER = 12;

const xOf = (i: number) => RAIL_X + (items.value[i]?.depth ?? 0) * props.indent;
const yOf = (i: number) => i * props.rowHeight + props.rowHeight / 2;

const svgWidth = computed(
  () => RAIL_X + items.value.reduce((max, item) => Math.max(max, item.depth), 0) * props.indent + 10,
);
const svgHeight = computed(() => items.value.length * props.rowHeight);

// Rail path: vertical runs connected by rounded S-bends where depth changes.
const railPath = computed(() => {
  if (items.value.length === 0) return '';
  let d = `M ${xOf(0)} ${yOf(0)}`;
  for (let i = 1; i < items.value.length; i++) {
    const x0 = xOf(i - 1);
    const x1 = xOf(i);
    const y1 = yOf(i);
    if (x0 === x1) {
      d += ` L ${x1} ${y1}`;
    } else {
      const midY = yOf(i - 1) + props.rowHeight / 2;
      const dir = x1 > x0 ? 1 : -1;
      const sweep0 = dir === 1 ? 0 : 1;
      const sweep1 = dir === 1 ? 1 : 0;
      d += ` L ${x0} ${midY - CORNER}`;
      d += ` A ${CORNER} ${CORNER} 0 0 ${sweep0} ${x0 + CORNER * dir} ${midY}`;
      d += ` L ${x1 - CORNER * dir} ${midY}`;
      d += ` A ${CORNER} ${CORNER} 0 0 ${sweep1} ${x1} ${midY + CORNER}`;
      d += ` L ${x1} ${y1}`;
    }
  }
  return d;
});

const pathEl = ref<SVGPathElement | null>(null);
const totalLen = ref(0);
const rowLens = ref<number[]>([]);

// Map each row to its arc length along the rail (y is monotonic).
function measure() {
  const path = pathEl.value;
  if (!path) return;
  totalLen.value = path.getTotalLength();
  // A framework with nothing to list has no rail to measure, and an empty
  // path throws if it is probed for a point.
  if (items.value.length === 0 || totalLen.value <= 0) {
    rowLens.value = [];
    return;
  }
  rowLens.value = items.value.map((_, i) => {
    const targetY = yOf(i);
    let lo = 0;
    let hi = totalLen.value;
    for (let step = 0; step < 24; step++) {
      const mid = (lo + hi) / 2;
      if (path.getPointAtLength(mid).y < targetY) lo = mid;
      else hi = mid;
    }
    return (lo + hi) / 2;
  });
}

// Spring-glide the dot's arc length toward the active row. `useRafFn` is
// written out here because the site has no @vueuse dependency and one loop does
// not justify adding one: resume on demand, stop once the spring settles.
const dotLen = ref(0);
const dotVel = ref(0);
const dot = ref({ x: RAIL_X, y: props.rowHeight / 2 });
let rafId: number | undefined;
let lastTime = 0;

const reduceMotion = () =>
  typeof window !== 'undefined' && window.matchMedia('(prefers-reduced-motion: reduce)').matches;

function placeDot() {
  const path = pathEl.value;
  if (!path || totalLen.value <= 0) return;
  const point = path.getPointAtLength(Math.min(Math.max(dotLen.value, 0), totalLen.value));
  dot.value = { x: point.x, y: point.y };
}

function frame(now: number) {
  rafId = undefined;
  const delta = lastTime ? now - lastTime : 16;
  lastTime = now;
  const target = rowLens.value[active.value] ?? 0;
  const dt = Math.min(delta / 1000, 1 / 30);
  const accel = 180 * (target - dotLen.value) - 26 * dotVel.value;
  dotVel.value += accel * dt;
  dotLen.value += dotVel.value * dt;
  placeDot();
  if (Math.abs(dotLen.value - target) < 0.25 && Math.abs(dotVel.value) < 0.5) {
    dotLen.value = target;
    dotVel.value = 0;
    placeDot();
    return;
  }
  rafId = requestAnimationFrame(frame);
}

function resume() {
  const target = rowLens.value[active.value] ?? 0;
  // The design guides make reduced motion a requirement, not a nicety: the
  // active entry still moves, it just arrives without the glide.
  if (reduceMotion()) {
    pause();
    dotVel.value = 0;
    dotLen.value = target;
    placeDot();
    return;
  }
  if (rafId !== undefined) return;
  lastTime = 0;
  rafId = requestAnimationFrame(frame);
}

function pause() {
  if (rafId === undefined) return;
  cancelAnimationFrame(rafId);
  rafId = undefined;
}

const progressOffset = computed(() => Math.max(totalLen.value - dotLen.value, 0));

// ---------------------------------------------------------------------------
// Derive the outline from what the active framework shows.

function headingLabel(el: HTMLElement) {
  const clone = el.cloneNode(true) as HTMLElement;
  // The section link and the copy-prompt button live inside the heading; they
  // are affordances, not part of its name.
  clone.querySelectorAll('.heading-anchor, .example-prompt').forEach((node) => node.remove());
  return clone.textContent?.trim() ?? '';
}

function isVisible(el: Element | null): el is HTMLElement {
  return el instanceof HTMLElement && el.getClientRects().length > 0;
}

function buildHeadings(): TocHeading[] {
  const root = document.querySelector('.doc-content');
  if (!root) return [];
  const headings: TocHeading[] = [];
  for (const pin of TOC_PINS) {
    const el = document.getElementById(pin.id);
    headings.push({ id: pin.id, label: pin.label, depth: 0, visible: isVisible(el) });
  }
  for (const el of root.querySelectorAll<HTMLElement>('h2[id], h3[id]')) {
    headings.push({
      id: el.id,
      label: headingLabel(el),
      depth: Number(el.tagName[1]) - 2,
      visible: isVisible(el),
    });
  }
  return headings;
}

// ---------------------------------------------------------------------------
// Keep the page and the outline in step in both directions. An
// IntersectionObserver is the active rule: the band starts where an anchored
// heading comes to rest (its own `scroll-margin-top`, 76px under the 56px nav)
// so a section counts as current when its heading reaches the reading edge.

let observer: IntersectionObserver | undefined;
let mutations: MutationObserver | undefined;
let scheduled = false;
let offsetPx = 88;

function navOffset() {
  const el = document.querySelector<HTMLElement>('.doc-content h2[id], .doc-content h3[id]');
  if (!el) return 88;
  const value = Number.parseFloat(getComputedStyle(el).scrollMarginTop);
  return Number.isFinite(value) && value > 0 ? value : 88;
}

function track() {
  observer?.disconnect();
  observer = undefined;
  const list = items.value;
  if (list.length === 0) return;
  offsetPx = navOffset();
  const index = new Map(list.map((item, i) => [item.id, i]));
  const elements = list
    .map((item) => document.getElementById(item.id))
    .filter((el): el is HTMLElement => el !== null);
  const seen = new Set<number>();

  observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        const i = index.get(entry.target.id);
        if (i === undefined) continue;
        if (entry.isIntersecting) seen.add(i);
        else seen.delete(i);
      }
      // Document order, so the topmost visible heading wins.
      if (seen.size > 0) {
        setActive(Math.min(...seen));
        return;
      }
      // Nothing in the band — near the foot of the page, or inside a section
      // longer than the viewport. Fall back to the last heading passed.
      let passed: number | undefined;
      for (let i = 0; i < list.length; i += 1) {
        const el = document.getElementById(list[i].id);
        if (el && el.getBoundingClientRect().top <= offsetPx) passed = i;
      }
      if (passed !== undefined) setActive(passed);
    },
    { rootMargin: `-${offsetPx}px 0px -55% 0px`, threshold: 0 },
  );
  elements.forEach((el) => observer?.observe(el));
}

// The column scrolls its own entries; without this the dot glides to a row the
// reader cannot see once an outline is longer than the viewport.
function revealRow(index: number) {
  const container = navEl.value;
  const row = listEl.value?.children[index] as HTMLElement | undefined;
  if (!container || !row) return;
  const top = row.offsetTop;
  const bottom = top + row.offsetHeight;
  if (top < container.scrollTop) container.scrollTop = top;
  else if (bottom > container.scrollTop + container.clientHeight) {
    container.scrollTop = bottom - container.clientHeight;
  }
}

function setActive(index: number) {
  if (index < 0 || index >= items.value.length || active.value === index) return;
  active.value = index;
  revealRow(index);
}

function scrollToItem(index: number) {
  const item = items.value[index];
  if (!item) return;
  const el = document.getElementById(item.id);
  if (!el) return;
  // `scrollIntoView` honours the heading's own `scroll-margin-top`, which is the
  // same offset the observer band uses — so a selected heading comes to rest
  // just under the bar, not underneath it.
  el.scrollIntoView({ behavior: reduceMotion() ? 'auto' : 'smooth', block: 'start' });
  history.replaceState(null, '', `#${item.id}`);
}

function onSelect(event: MouseEvent, index: number) {
  // Let a modified click do what the browser would: open the anchor elsewhere.
  if (
    event.defaultPrevented ||
    event.button !== 0 ||
    event.metaKey ||
    event.ctrlKey ||
    event.shiftKey ||
    event.altKey
  ) {
    return;
  }
  event.preventDefault();
  setActive(index);
  scrollToItem(index);
}

function refresh() {
  const next = tocItems(buildHeadings());
  if (sameTocItems(items.value, next)) return;
  items.value = next;
  // The active entry may not exist under the framework that just came in.
  if (active.value >= next.length) active.value = Math.max(next.length - 1, 0);
  nextTick(() => {
    measure();
    track();
  });
}

function schedule() {
  if (scheduled) return;
  scheduled = true;
  // A microtask is enough: the live example mounts during this task, so its
  // heading is in the document by the time the outline is rebuilt.
  queueMicrotask(() => {
    scheduled = false;
    refresh();
  });
}

onMounted(() => {
  measure();
  refresh();
  document.addEventListener('framework-change', refresh);
  const root = document.querySelector('.doc-content');
  if (root) {
    mutations = new MutationObserver(schedule);
    mutations.observe(root, { childList: true, subtree: true });
  }
});

onBeforeUnmount(() => {
  pause();
  observer?.disconnect();
  mutations?.disconnect();
  document.removeEventListener('framework-change', refresh);
});

watch([railPath, () => props.rowHeight], () => measure(), { flush: 'post' });
watch([active, rowLens], () => resume(), { flush: 'post' });
</script>

<template>
  <nav
    ref="navEl"
    v-show="items.length >= 2"
    class="toc toc--animated"
    :aria-label="props.title"
  >
    <p v-if="title" class="toc__label">{{ title }}</p>

    <div class="toc__body">
      <svg
        class="toc__rail-svg"
        :width="svgWidth"
        :height="svgHeight"
        :viewBox="`0 0 ${svgWidth} ${svgHeight}`"
        aria-hidden="true"
      >
        <!-- Base rail -->
        <path
          ref="pathEl"
          :d="railPath"
          class="toc__rail"
          fill="none"
          stroke-width="2"
          stroke-linecap="round"
        />
        <!-- Traveled portion -->
        <path
          :d="railPath"
          class="toc__rail-traveled"
          fill="none"
          stroke-width="2"
          stroke-linecap="round"
          :stroke-dasharray="totalLen"
          :stroke-dashoffset="progressOffset"
        />
        <!-- Traveling dot -->
        <circle class="toc__dot" :cx="dot.x" :cy="dot.y" r="5" />
      </svg>

      <ul ref="listEl" class="toc__list">
        <li
          v-for="(item, i) in items"
          :key="item.id"
          :style="{ height: `${rowHeight}px` }"
        >
          <a
            :href="item.href"
            class="toc__entry"
            :class="{ 'toc__entry--active': i === active }"
            :style="{ paddingLeft: `${16 + item.depth * indent}px` }"
            :aria-current="i === active ? 'true' : undefined"
            @click="onSelect($event, i)"
          >
            <span class="toc__entry-label">{{ item.label }}</span>
          </a>
        </li>
      </ul>
    </div>
  </nav>
</template>
