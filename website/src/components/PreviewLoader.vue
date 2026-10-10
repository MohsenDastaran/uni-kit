<script setup lang="ts">
import { computed } from "vue";

/**
 * The one loading state for every WebAssembly preview, on every page.
 *
 * It owns no download of its own. The bytes are already on their way — the
 * block's own picture starts fetching them when the pointer arrives, and the
 * frame shares that request — so this only reports what is happening: a ring,
 * the block's name, and either the live percentage of a download under way or
 * the size of the one about to start. It stays up until the preview's gallery
 * removes its `#loading` element, so the download and the boot read as one wait.
 */
const props = defineProps<{
  /** What is being loaded, e.g. "Loading Sidebar". */
  label: string;
  /** Bytes on the wire, when the build measured them — what the reader waits for. */
  size?: number;
  /** Progress of a download already under way, 0–100, when there is one. */
  percent?: number | null;
}>();

const megabytes = (bytes: number) => {
  const mb = bytes / 1024 / 1024;
  return mb >= 1 ? `${mb.toFixed(1)} MB` : `${Math.round(bytes / 1024)} KB`;
};

const sizeLabel = computed(() =>
  props.size && props.size > 0 ? megabytes(props.size) : null,
);
const counting = computed(
  () => typeof props.percent === "number" && props.percent < 100,
);
</script>

<template>
  <div class="loader" role="status" aria-live="polite">
    <span class="loader__spinner" aria-hidden="true" />
    <p class="loader__text">
      {{ label }}
      <span v-if="counting" class="loader__size">
        · {{ percent }}% of {{ sizeLabel }}
      </span>
      <span v-else-if="sizeLabel" class="loader__size"> · {{ sizeLabel }} </span>
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

/* One looping ring for every wait: the number in the label is the reading, so
   the indicator only has to say that work is still happening. */
.loader__spinner {
  display: block;
  width: 1.25rem;
  height: 1.25rem;
  border: 2px solid color-mix(in srgb, var(--foreground) 20%, transparent);
  border-top-color: var(--brand);
  border-radius: 50%;
  animation: loader-spin 700ms linear infinite;
}

@keyframes loader-spin {
  to {
    transform: rotate(1turn);
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
  .loader__spinner {
    animation: none;
  }
}
</style>
