<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";

// One block: a composed screen shown as the live example. `story` is the GPUI
// gallery's story name, `slug` the Slint gallery's page folder, `doc` the
// registry entry to install. `code` is already-highlighted HTML for each
// framework, produced at build time because `<Code>` renders a component and
// its output cannot be handed to a client-side tab.
export interface Block {
  id: string;
  title: string;
  description: string;
  story: string;
  slug: string;
  doc?: string;
  code?: { gpui?: string; slint?: string };
}

const props = defineProps<{
  blocks: Block[];
  baseUrl: string;
}>();

const base = props.baseUrl.replace(/\/$/, "");

const framework = ref<"gpui" | "slint">("gpui");
const readFramework = () =>
  document.documentElement.dataset.framework === "slint" ? "slint" : "gpui";

// Preview first: the example is what a reader judges a block by.
const tabs = ref<Record<string, "preview" | "code">>({});
const tabFor = (block: Block) => tabs.value[block.id] ?? "preview";
function selectTab(block: Block, tab: "preview" | "code") {
  tabs.value = { ...tabs.value, [block.id]: tab };
}

const MANAGERS = ["npx", "pnpm", "bun"] as const;
type Manager = (typeof MANAGERS)[number];
const MANAGER_KEY = "selected-package-manager";

const manager = ref<Manager>("npx");
function readManager(): Manager {
  const stored = localStorage.getItem(MANAGER_KEY);
  return stored === "pnpm" || stored === "bun" ? stored : "npx";
}
function selectManager(next: Manager) {
  manager.value = next;
  localStorage.setItem(MANAGER_KEY, next);
}

const runner = computed(() =>
  manager.value === "pnpm" ? "pnpm dlx" : manager.value === "bun" ? "bunx" : "npx",
);

// The registry copies Slint component files into an app. GPUI ships as the
// crate, so there is nothing to copy and the crate is the install.
function commandFor(block: Block) {
  if (framework.value === "slint") {
    return block.doc ? `${runner.value} @dastaran/uni-kit@latest add slint ${block.doc}` : "";
  }
  return "cargo add gpui-kit";
}

const copied = ref("");
async function copyCommand(block: Block) {
  try {
    await navigator.clipboard.writeText(commandFor(block));
    copied.value = block.id;
    window.setTimeout(() => {
      if (copied.value === block.id) copied.value = "";
    }, 1600);
  } catch {
    copied.value = "";
  }
}

function onFrameworkChange() {
  const next = readFramework();
  if (next === framework.value) return;
  framework.value = next;
  // Every frame reloads with the other framework, so the chain starts again.
  // Without this the whole set would mount at once on the first switch and the
  // ordering below would only ever hold for the initial load.
  mounted.value = 1;
  loaded.value = {};
}

onMounted(() => {
  framework.value = readFramework();
  manager.value = readManager();
  document.addEventListener("framework-change", onFrameworkChange);
});
onBeforeUnmount(() =>
  document.removeEventListener("framework-change", onFrameworkChange),
);

function srcFor(block: Block) {
  // The same two shapes the component pages use: the gallery takes a story
  // name, and each Slint page is its own folder.
  if (framework.value === "slint") {
    return `${base}/slint-gallery/pages/${block.slug}?component=${block.slug}`;
  }
  return `${base}/gallery?story=${encodeURIComponent(block.story)}`;
}

// Each example is a multi-megabyte wasm. Mounting them together makes the page
// pull every one at once, so only the first is mounted until it has loaded, and
// each loaded frame releases the next. The idle callback keeps a frame from
// starting while the previous one is still settling.
const mounted = ref(1);
const loaded = ref<Record<string, boolean>>({});

type IdleWindow = Window & {
  requestIdleCallback?: (cb: () => void, options?: { timeout: number }) => number;
};

function onFrameLoad(index: number) {
  const block = props.blocks[index];
  if (block) loaded.value[block.id] = true;
  const next = index + 1;
  if (mounted.value !== next) return;
  const advance = () => {
    if (mounted.value === next) mounted.value = next + 1;
  };
  const idle = (window as IdleWindow).requestIdleCallback;
  if (idle) idle(advance, { timeout: 2000 });
  else window.setTimeout(advance, 300);
}

const hasSlint = (block: Block) => Boolean(block.slug);
</script>

<template>
  <div class="blocks">
    <section
      v-for="(block, index) in blocks"
      :id="block.id"
      :key="block.id"
      class="block"
    >
      <header class="block__head">
        <div
          class="block__tabs framework-switch"
          role="tablist"
          :aria-label="`${block.title} view`"
        >
          <button
            v-for="tab in ['preview', 'code'] as const"
            :key="tab"
            type="button"
            role="tab"
            class="framework-switch__option block__tab capitalize"
            :aria-selected="tabFor(block) === tab"
            :tabindex="tabFor(block) === tab ? 0 : -1"
            @click="selectTab(block, tab)"
          >
            {{ tab }}
          </button>
        </div>
        <h2 class="block__title">{{ block.title }}</h2>
        <p class="block__description">{{ block.description }}</p>
      </header>

      <div v-show="tabFor(block) === 'preview'" class="block__frame">
        <iframe
          v-if="index < mounted"
          :src="srcFor(block)"
          :title="`${block.title} example`"
          class="block__iframe"
          allow="cross-origin-isolated"
          @load="onFrameLoad(index)"
        />
        <div v-else class="block__status" role="status">
          <span class="block__spinner" aria-hidden="true" />
          Waiting for the example above
        </div>

        <div
          v-if="index < mounted && !loaded[block.id]"
          class="block__status"
          role="status"
        >
          <span class="block__spinner" aria-hidden="true" />
          Loading {{ block.title }}
        </div>
      </div>

      <div v-show="tabFor(block) === 'code'" class="block__code">
        <div class="block__install">
          <div
            class="install-command__switch framework-switch"
            role="radiogroup"
            aria-label="Package manager"
            :data-selected="manager"
          >
            <span class="framework-switch__thumb" aria-hidden="true" />
            <button
              v-for="name in MANAGERS"
              :key="name"
              type="button"
              class="framework-switch__option install-command__option capitalize"
              role="radio"
              :aria-checked="manager === name"
              :tabindex="manager === name ? 0 : -1"
              @click="selectManager(name)"
            >
              {{ name }}
            </button>
          </div>
          <code class="block__command">{{ commandFor(block) }}</code>
          <button type="button" class="block__copy" @click="copyCommand(block)">
            {{ copied === block.id ? "Copied" : "Copy" }}
          </button>
        </div>

        <!-- Both frameworks are rendered; the framework panels switch on CSS
             alone, the same way the component pages do. Rendering one and
             swapping it in script would leave the other out of the HTML. -->
        <div v-if="block.code?.gpui || block.code?.slint" class="block__snippets">
          <div class="framework-code__panel" data-framework-panel="gpui">
            <div v-if="block.code?.gpui" class="block__snippet" v-html="block.code.gpui" />
            <p v-else class="block__note">No GPUI source for this block yet.</p>
          </div>
          <div class="framework-code__panel" data-framework-panel="slint">
            <div v-if="block.code?.slint" class="block__snippet" v-html="block.code.slint" />
            <p v-else class="block__note">No Slint source for this block yet.</p>
          </div>
        </div>
        <p v-else class="block__note">No source to copy for this block yet.</p>
      </div>

      <p v-if="!hasSlint(block) && framework === 'slint'" class="block__note">
        This block has no Slint version yet.
      </p>
    </section>
  </div>
</template>

<style scoped>
.blocks {
  display: flex;
  flex-direction: column;
  gap: 4rem;
  margin-block: 2rem;
}

/* The title and description sit above the example, which is what the reader
   scans before deciding whether the screen is worth opening. */
.block__head {
  margin-bottom: 1rem;
}

.block__tabs {
  display: inline-flex;
  margin-bottom: 0.85rem;
}

.block__tab {
  min-width: 4.5rem;
}

.block__title {
  margin: 0;
  font-size: 1.35rem;
  font-weight: 640;
  letter-spacing: -0.022em;
}

.block__description {
  margin: 0.4rem 0 0;
  max-width: 46rem;
  color: var(--muted-foreground);
}

.block__frame {
  position: relative;
  height: clamp(26rem, 62vh, 44rem);
  overflow: hidden;
  border: 1px solid var(--border);
  border-radius: var(--radius-surface);
  background: var(--card);
}

.block__iframe {
  display: block;
  width: 100%;
  height: 100%;
  border: 0;
}

.block__status {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.6rem;
  background: var(--card);
  color: var(--muted-foreground);
  font-size: 0.875rem;
}

.block__spinner {
  width: 0.95rem;
  height: 0.95rem;
  border: 2px solid color-mix(in srgb, var(--foreground) 22%, transparent);
  border-top-color: transparent;
  border-radius: 999px;
  animation: block-spin 700ms linear infinite;
}

@keyframes block-spin {
  to {
    transform: rotate(1turn);
  }
}

@media (prefers-reduced-motion: reduce) {
  .block__spinner {
    animation: none;
  }
}

.block__code {
  overflow: hidden;
  border: 1px solid var(--border);
  border-radius: var(--radius-surface);
  background: var(--card);
}

.block__install {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 0.9rem;
  border-bottom: 1px solid var(--border);
}

.block__command {
  flex: 1;
  min-width: 0;
  overflow-x: auto;
  color: var(--foreground);
  font-family: var(--font-mono);
  font-size: 0.8125rem;
  white-space: nowrap;
}

.block__copy {
  flex: none;
  padding: 0.25rem 0.6rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-control);
  background: transparent;
  color: var(--muted-foreground);
  font-size: 0.75rem;
  cursor: pointer;
}

.block__copy:hover {
  color: var(--foreground);
  border-color: color-mix(in srgb, var(--foreground) 28%, transparent);
}

.block__snippet :deep(pre) {
  margin: 0;
  border-radius: 0;
}

.block__note {
  margin: 0.85rem;
  color: var(--muted-foreground);
  font-size: 0.8125rem;
}
</style>
