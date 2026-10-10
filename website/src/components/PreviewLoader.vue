<script setup lang="ts">
import { computed, onMounted, ref } from "vue";

/**
 * The one loading state for every WebAssembly preview, on every page.
 *
 * The frame fetches its own module, so this side usually has no byte count to
 * report — but the build hands over the module's file names, and the site
 * serves them in a way the browser will reuse, so the bytes can be fetched here
 * once, counted, and read from the cache by the frame that follows. That is what
 * turns the wait into a real percentage: `N% of S`.
 *
 * Three phases, one honest rule each:
 *
 * - `waiting` — the download is about to be measured, or cannot be (no files
 *   known, or a server that will not let the browser keep them). An
 *   indeterminate bar and the size, where it is known. `Feedback reflects
 *   reality`: a number this side cannot compute is not printed.
 * - `measured` — the bytes are being fetched and counted.
 * - `starting` — the download is done and the frame is booting from the cache.
 */
const props = defineProps<{
  /** What is being loaded, e.g. "Loading Sidebar". */
  label: string;
  /** The same wait once the download has finished, e.g. "Starting Sidebar". */
  startLabel?: string;
  /** Bytes on the wire, when the build measured them. */
  size?: number;
  /** The module's files, relative to the site root, when the build found them. */
  files?: string[];
}>();

const emit = defineEmits<{ downloaded: [] }>();

const megabytes = (bytes: number) => {
  const mb = bytes / 1024 / 1024;
  return mb >= 1 ? `${mb.toFixed(1)} MB` : `${Math.round(bytes / 1024)} KB`;
};

const sizeLabel = computed(() =>
  props.size && props.size > 0 ? megabytes(props.size) : null,
);
const startingLabel = computed(
  () => props.startLabel ?? props.label.replace(/^Loading/, "Starting"),
);

/** 0–100 while measuring, null when there is nothing honest to count. */
const percent = ref<number | null>(null);
/** The measured download finished (or was never measurable); the frame boots. */
const started = ref(false);

const barWidth = computed(() => {
  if (percent.value !== null) return `${percent.value}%`;
  if (started.value) return "100%";
  return undefined;
});
const barMeasured = computed(() => percent.value !== null || started.value);

/** Resolves to `fallback` if the work has not finished in time. */
function withTimeout<T>(work: Promise<T>, ms: number, fallback: T) {
  return new Promise<T>((resolve) => {
    const timer = window.setTimeout(() => resolve(fallback), ms);
    work.then(
      (value) => {
        window.clearTimeout(timer);
        resolve(value);
      },
      () => {
        window.clearTimeout(timer);
        resolve(fallback);
      },
    );
  });
}

/**
 * Fetches the module once and counts it. A server that does not answer, or will
 * not let the bytes be kept, must not hold the preview back: the frame can
 * always fetch the module itself.
 */
async function measure(files: string[]) {
  const heads = await withTimeout(
    Promise.all(files.map((file) => fetch(file, { method: "HEAD" }))),
    6000,
    null,
  );
  if (!heads) return;

  // Reuse is what makes a preflight free: the frame's request must read the
  // same bytes from the cache rather than pay for them a second time.
  const reusable = heads.every((head) => {
    const control = head.headers.get("cache-control") ?? "";
    return control !== "" && !/no-store/.test(control);
  });
  const total = heads.reduce(
    (sum, head) => sum + Number(head.headers.get("content-length") ?? 0),
    0,
  );
  if (!reusable || total <= 0) return;

  percent.value = 0;
  let received = 0;
  for (const file of files) {
    const response = await fetch(file);
    const reader = response.body?.getReader();
    if (!reader) break;
    for (;;) {
      // A stream that stops moving is worse than one that is slow: hand the
      // preview back to the frame rather than counting forever.
      const chunk = await withTimeout(reader.read(), 8000, null);
      if (!chunk) return;
      if (chunk.done) break;
      received += chunk.value.byteLength;
      // Held below 100: the frame still has to boot.
      percent.value = Math.min(99, Math.round((received / total) * 100));
    }
  }
  percent.value = 100;
}

onMounted(async () => {
  const files = props.files ?? [];
  try {
    if (files.length > 0) await measure(files);
  } catch {
    // A failed read is not a failed preview: the frame fetches it itself.
  } finally {
    // Always, and whatever happened above: the frame starts now. Waiting for a
    // manifest that arrives late would leave the preview unmounted for good.
    started.value = true;
    emit("downloaded");
  }
});
</script>

<template>
  <div class="loader" role="status" aria-live="polite">
    <span
      class="loader__bar"
      :class="{ 'loader__bar--measured': barMeasured }"
      aria-hidden="true"
    >
      <i :style="barWidth ? { width: barWidth } : undefined" />
    </span>
    <p class="loader__text">
      <template v-if="started && percent !== null">{{ startingLabel }}</template>
      <template v-else>
        {{ label }}
        <span v-if="percent !== null && percent < 100" class="loader__size">
          · {{ percent }}% of {{ sizeLabel }}
        </span>
        <span v-else-if="sizeLabel" class="loader__size">
          · {{ sizeLabel }} download
        </span>
      </template>
    </p>
  </div>
</template>

<style scoped>
.loader {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.7rem;
  padding: 1.25rem;
  text-align: center;
}

.loader__bar {
  position: relative;
  display: block;
  overflow: hidden;
  width: min(14rem, 60%);
  height: 3px;
  border-radius: 999px;
  background: color-mix(in srgb, var(--foreground) 12%, transparent);
}

.loader__bar i {
  position: absolute;
  inset: 0;
  width: 40%;
  border-radius: 999px;
  background: var(--brand);
  animation: loader-sweep 1.15s ease-in-out infinite;
}

/* Measured: the bar is the reading, so it holds still. */
.loader__bar--measured i {
  left: 0;
  animation: none;
  transition: width 220ms ease-out;
}

@keyframes loader-sweep {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(250%);
  }
}

.loader__text {
  margin: 0;
  color: var(--muted-foreground);
  font-size: 0.84rem;
  line-height: 1.5;
}

.loader__size {
  color: color-mix(in srgb, var(--muted-foreground) 80%, transparent);
}

@media (prefers-reduced-motion: reduce) {
  .loader__bar i {
    animation: none;
    width: 100%;
    opacity: 0.5;
  }

  .loader__bar--measured i {
    opacity: 1;
    transition: none;
  }
}
</style>
