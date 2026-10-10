<script setup lang="ts">
import { computed, onMounted, ref } from "vue";

/**
 * The wait for a WebAssembly preview.
 *
 * The frame fetches its own module, so this side usually has no byte count to
 * report — but the build hands over the module's file names, and the site serves
 * them immutable for a year, so the bytes can be fetched here once and read from
 * the cache by the frame that follows. That is what turns the wait into a real
 * percentage rather than an animation.
 *
 * Where the files are not known, or the server will not let the browser keep
 * them, nothing is fetched twice: the bar stays indeterminate. `Feedback
 * reflects reality` — a number that only reaches 100 when the frame says so
 * would be theatre.
 */
const props = defineProps<{
  /** What is being loaded, e.g. "Loading Sidebar". */
  label: string;
  /** Bytes of the module being fetched, when the build measured it. */
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

/** 0 to 100 while measuring, null when there is nothing honest to show. */
const percent = ref<number | null>(null);

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

async function measure(files: string[]) {
  // Only worth fetching ahead where the frame will reuse the bytes. The metadata
  // is given a moment: a server that does not answer must not hold the preview
  // back, since the frame can always fetch the module itself.
  const heads = await withTimeout(
    Promise.all(files.map((file) => fetch(file, { method: "HEAD" }))),
    6000,
    null,
  );
  if (!heads) return;
  const reusable = heads.every((head) =>
    /max-age=\d+/.test(head.headers.get("cache-control") ?? ""),
  );
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
      // A stream that stops moving is worse than one that is slow: give the
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
    emit("downloaded");
  }
});
</script>

<template>
  <div class="loader" role="status" aria-live="polite">
    <span
      class="loader__bar"
      :class="{ 'loader__bar--measured': percent !== null }"
      aria-hidden="true"
    >
      <i :style="percent !== null ? { width: `${percent}%` } : undefined" />
    </span>
    <p class="loader__text">
      {{ label }}
      <span v-if="percent !== null && percent < 100" class="loader__size">
        · {{ percent }}%
      </span>
      <span v-else-if="sizeLabel" class="loader__size">
        · {{ sizeLabel }} of WebAssembly
      </span>
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
