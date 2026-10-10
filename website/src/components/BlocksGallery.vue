<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { Download, Monitor, Play, RotateCw, Smartphone, Sparkles, Tablet } from "lucide-vue-next";
import InstallCommand from "./InstallCommand.vue";
import PreviewLoader from "./PreviewLoader.vue";
import type { Gallery } from "../lib/gallery-sizes";

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
  /**
   * The components this screen is composed from. For Slint that is also what
   * the install command names: a block is assembled from installed controls
   * rather than imported as a unit, and the CLI writes the files they share
   * once.
   */
  components?: string[];
  /**
   * The registry names this block installs for GPUI, where the block itself is
   * the copyable unit: the controls it composes ship in the `gpui-kit` crate.
   * Absent means no GPUI block file exists yet, and the bar falls back to the
   * crate.
   */
  gpuiInstall?: string[];
  /**
   * Which composition inside the Slint page to show. A component page documents
   * every one it has; a block is a single finished screen, so it asks for the
   * last. Empty shows them all, which is what the component page wants.
   */
  slintExample?: string;
  /**
   * A picture of the finished screen, per framework, shown before anything is
   * downloaded. The reader sees what the block looks like, and the module is
   * fetched only when they ask to run it.
   */
  posters?: { gpui?: string; slint?: string };
  /** Highlighted HTML, for display. */
  code?: { gpui?: string; slint?: string };
  /** The same source as plain text, for the prompt. */
  text?: { gpui?: string; slint?: string };
}

const props = defineProps<{
  blocks: Block[];
  baseUrl: string;
  /** The galleries' sizes and file names, measured at build time. */
  sizes?: Record<string, Gallery>;
}>();

const base = props.baseUrl.replace(/\/$/, "");

const framework = ref<"gpui" | "slint">("slint");
const readFramework = () =>
  document.documentElement.dataset.framework === "gpui" ? "gpui" : "slint";






function onFrameworkChange() {
  const next = readFramework();
  if (next === framework.value) return;
  framework.value = next;
  // Every frame reloads with the other framework, and the new one must be
  // measured rather than fetched unseen: what was asked for by hand stays asked
  // for, and the new first block loads on its own.
  primed.value = {};
  loaded.value = {};
  for (const block of props.blocks) stopWatching(block);
}

onMounted(() => {
  // Registered before anything else can throw: a failure here would otherwise
  // leave the page deaf to the framework switch in the header, and the examples
  // would silently keep showing the framework the page was built with.
  document.addEventListener("framework-change", onFrameworkChange);
  framework.value = readFramework();
  // No frame is in the server's HTML anymore: every preview, the first included,
  // goes through the loader's measured download, so there is nothing to
  // reconcile and no `load` that could have finished before this island woke up.
});
onBeforeUnmount(() => {
  clearTimeout(promptTimer);
  document.removeEventListener("framework-change", onFrameworkChange);
  for (const block of props.blocks) stopWatching(block);
});

/**
 * The blocks for the framework in force. A block is not required to exist in
 * both: only a framework with a file to copy can offer one to install, and a
 * screen worth reading in a gallery is not automatically a screen a command
 * can write.
 */
const visible = computed(() =>
  framework.value === "gpui"
    ? props.blocks.filter((block) => block.gpuiInstall?.length)
    : props.blocks,
);

function srcFor(block: Block) {
  // The same two shapes the component pages use: the gallery takes a story
  // name, and each Slint page is its own folder.
  if (framework.value === "slint") {
    const example = block.slintExample
      ? `&example=${encodeURIComponent(block.slintExample)}`
      : "";
    return `${base}/slint-gallery/pages/${block.slug}?component=${block.slug}${example}`;
  }
  return `${base}/gallery?story=${encodeURIComponent(block.story)}&source=0`;
}

// Every example is a multi-megabyte WebAssembly build with its own renderer. The
// page opens with a picture of each block and downloads nothing; a block's
// module is fetched when the reader asks to run it. Loading them all in
// sequence, which this did two revisions ago, spent the page's whole time and
// memory on previews nobody had reached yet, and loading just the first still
// spent it before the reader had chosen anything.
const requested = ref<Record<string, boolean>>({});
const loaded = ref<Record<string, boolean>>({});

/** A block is fetched when the reader asks for it, and not before. */
const isRequested = (block: Block) => Boolean(requested.value[block.id]);

/** The picture to show for the framework in force, where one exists. */
const posterFor = (block: Block) => block.posters?.[framework.value];

function request(block: Block) {
  requested.value = { ...requested.value, [block.id]: true };
}

/**
 * The frame's own gallery is the one that knows when the preview is ready: each
 * gallery removes its `#loading` element once the app has drawn its first frame.
 * The overlay stays until then, so the download and the boot read as one wait
 * and the gallery's plain "Loading…" is never what the reader sees. A frame that
 * cannot be inspected — a cross-origin one — falls back to its `load` event, and
 * a boot that never reports in is revealed anyway rather than trapping the page
 * behind an overlay forever.
 */
const readyObservers = new Map<string, MutationObserver>();
const readyTimers = new Map<string, number>();
const BOOT_LIMIT = 120_000;

function stopWatching(block: Block) {
  readyObservers.get(block.id)?.disconnect();
  readyObservers.delete(block.id);
  const timer = readyTimers.get(block.id);
  if (timer !== undefined) window.clearTimeout(timer);
  readyTimers.delete(block.id);
}

function watchFrame(frame: HTMLIFrameElement, block: Block) {
  stopWatching(block);
  const settle = () => {
    stopWatching(block);
    loaded.value = { ...loaded.value, [block.id]: true };
  };

  let document_: Document | null = null;
  try {
    document_ = frame.contentDocument;
  } catch {
    // A cross-origin frame cannot be inspected, so its `load` is all there is.
    settle();
    return;
  }
  if (!document_ || !document_.body) {
    settle();
    return;
  }

  const stillLoading = () => document_?.getElementById("loading") ?? null;
  if (!stillLoading()) {
    settle();
    return;
  }
  // `#loading` is a child of `body`, so watching the body's children is enough
  // and does not walk the whole document on every mutation of the app.
  const observer = new MutationObserver(() => {
    if (!stillLoading()) settle();
  });
  observer.observe(document_.body, { childList: true });
  readyObservers.set(block.id, observer);
  readyTimers.set(
    block.id,
    window.setTimeout(settle, BOOT_LIMIT),
  );
}

function onFrameLoad(event: Event, block: Block) {
  const frame = event.target;
  if (frame instanceof HTMLIFrameElement) watchFrame(frame, block);
  else loaded.value = { ...loaded.value, [block.id]: true };
}

// The galleries' sizes and file names arrive from the page that rendered this
// island, so the server's HTML already says how big each download is. With the
// names, the loader fetches the bytes itself and counts them, and the frame that
// follows reads the same bytes from the cache.
const keyFor = (block: Block) =>
  srcFor(block).split("?")[0].replace(new RegExp(`^${base}/`), "");
const sizeFor = (block: Block) => props.sizes?.[keyFor(block)]?.total;
const filesFor = (block: Block) =>
  props.sizes?.[keyFor(block)]?.files.map((file) => `${base}/${keyFor(block)}/${file}`);

// Every preview is fetched first, so the wait can be counted, and the frame
// mounts once the bytes are here.
const primed = ref<Record<string, boolean>>({});
function prime(block: Block) {
  primed.value = { ...primed.value, [block.id]: true };
}
const frameIsUp = (block: Block) => Boolean(primed.value[block.id]);

const hasSlint = (block: Block) => Boolean(block.slug);

// A block can exist for one framework only: the starter page's introduction is
// the GPUI gallery's opening screen and has no Slint page. Saying so is better
// than pointing the frame at a page that is not there.
const frameUnavailable = (block: Block) =>
  framework.value === "slint" && !hasSlint(block);

// Each block teleports its install bar into its own host, since the shared
// component would otherwise put every one of them in the same place.
const hostFor = (block: Block) => `[data-install-block="${block.id}"]`;

// The frame narrows to a device width, the way a browser's responsive mode
// does, so a block can be judged at the size it will actually be used at.
const DEVICES = [
  { id: "desktop", label: "Desktop width", icon: Monitor, width: "100%" },
  { id: "tablet", label: "Tablet width", icon: Tablet, width: "834px" },
  { id: "phone", label: "Phone width", icon: Smartphone, width: "390px" },
] as const;
type DeviceId = (typeof DEVICES)[number]["id"];

const device = ref<Record<string, DeviceId>>({});
const deviceFor = (block: Block) => device.value[block.id] ?? "desktop";
const widthFor = (block: Block) =>
  DEVICES.find((entry) => entry.id === deviceFor(block))?.width ?? "100%";
function selectDevice(block: Block, id: DeviceId) {
  device.value = { ...device.value, [block.id]: id };
}

// The same offer the component pages make: hand an assistant the framework, the
// install command and the source, so it can write the screen rather than guess
// at it. The label matches theirs.
const managerRunner = () =>
  localStorage.getItem("selected-package-manager") === "pnpm"
    ? "pnpm dlx"
    : localStorage.getItem("selected-package-manager") === "bun"
      ? "bunx"
      : "npx";

const promptLabel = "Copy Usage Prompt for AI";
const prompted = ref("");
let promptTimer: ReturnType<typeof setTimeout> | undefined;

// The gallery's own source is written for the gallery: imports climb out of
// `examples/`, the tours are guarded by a `GalleryView` flag only the gallery
// sets, and they wrap themselves in a section helper the component library does
// not ship. Handed to an assistant as-is, that produces files whose imports do
// not resolve — so the prompt carries the screen itself instead.
function withoutTours(source: string) {
  const lines = source.split("\n");
  const kept: string[] = [];
  for (let i = 0; i < lines.length; i += 1) {
    if (/^\s*if GalleryView\.example != "last":/.test(lines[i])) {
      // Skip the whole element, counting braces so its body goes with it.
      let depth = 0;
      let opened = false;
      for (; i < lines.length; i += 1) {
        for (const char of lines[i]) {
          if (char === "{") {
            depth += 1;
            opened = true;
          } else if (char === "}") {
            depth -= 1;
          }
        }
        if (opened && depth <= 0) break;
      }
      continue;
    }
    kept.push(lines[i]);
  }
  return kept.join("\n");
}

// What the install command actually produces: one flat directory of files that
// import each other by bare name.
function installableSource(source: string) {
  return (
    withoutTours(source)
      // The screen, not the gallery's branch that selects it.
      .replace(/^(\s*)if GalleryView\.example == "last": /gm, "$1")
      // Every component lands beside the app's own files.
      .replace(/from "\.\.\//g, 'from "')
      // GroupBox belonged to the tours; with them gone nothing imports it.
      .replace(/^import \{[^}]*\} from "section\.slint";\n/gm, "")
      .replace(/import \{ Theme, GalleryView, /, "import { Theme, ")
      .replace(/import \{ Theme, GalleryView \}/, "import { Theme }")
  );
}

function buildPrompt(block: Block) {
  const source = block.text?.[framework.value] ?? "";
  const library = framework.value === "slint" ? "Slint" : "gpui-component";
  const live = framework.value === "slint" ? "Rust, Slint & WASM" : "Rust, GPUI & WASM";
  const slint = framework.value === "slint";
  const names = block.components?.length
    ? block.components
    : [block.doc ?? block.slug].filter(Boolean);
  // Slint installs the controls the screen is built from. GPUI installs the
  // screen itself, and the crate it is written against alongside it.
  const gpuiNames = block.gpuiInstall ?? [];
  const install = slint
    ? `${managerRunner()} @dastaran/uni-kit@latest add slint ${names.join(" ")}`
    : gpuiNames.length > 0
      ? [
          "cargo add gpui-kit",
          `${managerRunner()} @dastaran/uni-kit@latest add gpui ${gpuiNames.join(" ")}`,
        ].join("\n")
      : "cargo add gpui-kit";
  return [
    `Use this ${slint ? "Slint" : "GPUI"} example in my application. Keep the framework, file layout, and API below; do not invent a different component or asset path.`,
    "",
    "## Framework",
    `${slint ? "Slint" : "GPUI"} (${live}). Library: ${library}.`,
    "",
    "## Block",
    block.title,
    ...(block.components?.length
      ? [`Composed from: ${block.components.join(", ")}`]
      : []),
    `Page: ${window.location.href.split("#")[0]}`,
    "",
    "## Install",
    install,
    "",
    "## Files",
    slint
      ? "Every file lands in ui/components/ beside the others, and the command adds that directory to the Slint include path in build.rs. Imports between them are by bare file name, as below."
      : gpuiNames.length > 0
        ? `The command writes src/components/${gpuiNames[0]}.rs and adds \`pub mod ${gpuiNames[0]};\` to src/components/mod.rs. Declare \`mod components;\` in the crate root so it is compiled. That file is the screen: the controls it composes come from the gpui-kit crate through \`gpui_kit::component\`, and it needs no asset file of its own.`
        : "The screen is written against the gpui-kit crate; the controls it composes come from `gpui_kit::component`.",
    "",
    "## Source",
    slint ? installableSource(source) : source,
  ].join("\n");
}

async function copyUsagePrompt(block: Block) {
  try {
    await navigator.clipboard.writeText(buildPrompt(block));
  } catch {
    return;
  }
  prompted.value = block.id;
  clearTimeout(promptTimer);
  promptTimer = setTimeout(() => {
    if (prompted.value === block.id) prompted.value = "";
  }, 1600);
}

// Reloading replaces the frame: the nonce is part of its key, so Vue tears the
// old one down and the example starts from the beginning again.
const reloads = ref<Record<string, number>>({});
function reload(block: Block) {
  stopWatching(block);
  loaded.value = { ...loaded.value, [block.id]: false };
  reloads.value = { ...reloads.value, [block.id]: (reloads.value[block.id] ?? 0) + 1 };
}
</script>

<template>
  <div class="blocks">
    <section
      v-for="block in visible"
      :id="block.id"
      :key="block.id"
      class="block"
    >
      <!-- The same bar the component pages use: a label, the switcher, and the
           install command on one row. Here the switcher picks Preview or Code,
           the title takes the flexible column, and the actions sit before the
           command. -->
      <header class="framework-bar block__bar">
        <h2 class="block__title">{{ block.title }}</h2>

        <div class="block__actions">
          <button
            v-for="entry in DEVICES"
            :key="entry.id"
            type="button"
            class="block__action"
            :class="{ 'is-active': deviceFor(block) === entry.id }"
            :aria-pressed="deviceFor(block) === entry.id"
            :disabled="!primed[block.id]"
            :title="entry.label"
            :aria-label="entry.label"
            @click="selectDevice(block, entry.id)"
          >
            <component :is="entry.icon" :size="15" aria-hidden="true" />
          </button>
          <button
            type="button"
            class="block__action"
            :disabled="!primed[block.id]"
            title="Reload example"
            aria-label="Reload example"
            @click="reload(block)"
          >
            <RotateCw :size="15" aria-hidden="true" />
          </button>
        </div>

        <div class="block__install" :data-install-block="block.id" />
      </header>

      <!-- Outside the bar, as on a component page. Inside it, the component's
           own wrapper becomes another grid cell: the row shifts, and the command
           is measured against a grid area instead of the row it sits in. -->
      <InstallCommand
        :slug="block.doc"
        :slugs="{ gpui: block.gpuiInstall ?? [block.id], slint: block.components }"
        :host="hostFor(block)"
      />

      <!-- Under the bar: the sentence says what the button is for, and the
           button sits at the trailing edge of the row. -->
      <div class="block__prompt-row">
        <p class="block__prompt-note">
          Installed and ran the command? To build this block yourself, hand this
          prompt to your AI.
        </p>
        <button
          type="button"
          class="block__prompt"
          :title="promptLabel"
          :aria-label="promptLabel"
          :data-copied="prompted === block.id || null"
          @click="copyUsagePrompt(block)"
        >
          <Sparkles :size="13" aria-hidden="true" />
          {{ prompted === block.id ? "Copied" : "Usage prompt for this block" }}
        </button>
      </div>

      <div class="block__frame">
        <div class="block__sizer" :style="{ maxWidth: widthFor(block) }">
          <!-- The frame boots behind the picture, out of sight, so the
               gallery's own plain "Loading…" is never what the reader sees. -->
          <iframe
            v-if="frameIsUp(block) && !frameUnavailable(block)"
            :key="`${srcFor(block)}:${reloads[block.id] ?? 0}`"
            :src="srcFor(block)"
            :title="`${block.title} example`"
            class="block__iframe"
            :class="{ 'block__iframe--live': loaded[block.id] }"
            allow="cross-origin-isolated"
            @load="onFrameLoad($event, block)"
          />

          <div
            v-if="frameUnavailable(block)"
            class="block__status block__status--note"
            role="status"
          >
            This example runs in the GPUI gallery only.
          </div>

          <!-- What the block looks like, until the reader chooses to run it.
               The whole picture is the button; the call to action appears when
               the pointer or the keyboard is on it. -->
          <div
            v-else-if="!loaded[block.id]"
            class="block__status block__status--offer"
          >
            <button
              v-if="posterFor(block)"
              type="button"
              class="block__poster"
              :disabled="isRequested(block)"
              :aria-label="`Show the live preview of ${block.title}`"
              @click="request(block)"
            >
              <img
                class="block__poster-image"
                :src="posterFor(block)"
                alt=""
                loading="lazy"
                decoding="async"
              />
              <span class="block__poster-cta">
                <Play :size="14" aria-hidden="true" />
                Show live preview
                <span v-if="sizeFor(block)" class="block__poster-size">
                  · {{ (sizeFor(block)! / 1024 / 1024).toFixed(1) }} MB
                </span>
              </span>
            </button>
            <template v-else>
              <p class="block__offer-text">
                This preview is a running WebAssembly build<template
                  v-if="sizeFor(block)"
                >
                  — a {{ (sizeFor(block)! / 1024 / 1024).toFixed(1) }} MB
                  download</template
                >. It fetches only when you ask for it.
              </p>
              <button type="button" class="block__offer" @click="request(block)">
                <Download :size="14" aria-hidden="true" />
                Download &amp; show preview
              </button>
            </template>
          </div>

          <div
            v-if="isRequested(block) && !loaded[block.id] && !frameUnavailable(block)"
            :key="`loader:${block.id}:${framework}`"
            class="block__status block__status--loading"
          >
            <PreviewLoader
              :label="`Loading ${block.title}…`"
              :size="sizeFor(block)"
              :files="primed[block.id] ? undefined : filesFor(block)"
              @downloaded="prime(block)"
            />
          </div>
        </div>
      </div>

    </section>
  </div>
</template>

<style scoped>
.blocks {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  margin-block: 2rem;
}

/* One hairline per boundary, drawn by the block below it: two neighbours must
   not both draw the same line. The spacing is split across it, so the rule sits
   in the middle of the gap rather than against either block's first row. */
.block + .block {
  padding-top: 2rem;
  border-top: 1px solid var(--border);
}

/* The bar the component pages use, with the four columns read as
   tabs | title | actions | install. The global rule puts a `.framework-switch`
   in column three, which is right for the component page's bar and wrong here.

   Row one is stated rather than inferred. Measured in a browser it came out
   46.39px against 32px controls, because the title's line box is taller than the
   controls beside it, and centring three equal controls against a taller row put
   the middle one 7.2px below the others. Pinning the row and the alignment
   removes the ambiguity: the controls are all the control height, so they share
   one top edge. */
.block__bar {
  margin-block: 0 0.75rem;
  grid-template-rows: 2rem auto;
  align-items: center;
}




.block__title {
  grid-area: 1 / 1 / 2 / 3;
  min-width: 0;
  margin: 0;
  overflow: hidden;
  font-size: 0.9375rem;
  font-weight: 560;
  letter-spacing: -0.012em;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* `contents` lets the teleported parts join this bar's grid, which is how the
   component page's bar places them: the manager switch in column four of the
   first row, the command across the second. */
.block__install {
  display: contents;
}

/* The command sits directly under the bar here. The component page draws a rule
   above it, which reads as a second header on a page of previews. */
.block__bar :deep(.install-command__line) {
  border-top: 0;
}

/* `margin: 0` matters more than it looks. A prose-spacing rule in the docs
   layout gives this box `margin-top: 14.4px`; with `align-self: center` half of
   that margin pushes it down, which measured as the whole row sitting 7.2px
   below the title and the switch. */
.block__actions {
  grid-area: 1 / 3 / 2 / 4;
  margin: 0;
  display: flex;
  align-items: center;
  gap: 0.125rem;
}

/* Stated per item rather than inherited, for the same reason. */
.block__bar > .block__title,
.block__bar > .block__actions,
.block__bar :deep(.install-command__switch) {
  align-self: center;
}

/* The title is an `h2` inside `.doc-content`, so it inherits the documentation
   rule that draws a rule under a heading, and its line box grew row one to
   46.4px -- taller than the 32px controls in it, which is what left them unable
   to agree on a top edge. Dropping the rule and pinning the line box to the
   control height makes row one exactly one row of controls. */
.block__bar > .block__title {
  border-bottom: 0;
  padding-block: 0;
  line-height: 2rem;
}

.block__bar :deep(.install-command__switch) {
  grid-area: 1 / 4 / 2 / 5;
  height: 2rem;
}

/* A row of its own under the bar: the sentence explains the button, the button
   sits at the trailing edge. */
.block__prompt-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  margin-block: 0 0.75rem;
}

.block__prompt-note {
  min-width: 0;
  max-width: 46rem;
  margin: 0;
  color: var(--muted-foreground);
  font-size: 0.8125rem;
  line-height: 1.5;
}

/* Same treatment as the install command's copy button — hairline border and
   `--radius-control` — so the two read as the same family of controls. */
.block__prompt {
  display: inline-flex;
  flex: none;
  align-items: center;
  gap: 0.4rem;
  height: 2rem;
  padding: 0 0.85rem;
  border: 1px solid color-mix(in srgb, var(--brand) 55%, var(--border));
  border-radius: var(--radius-control);
  background: color-mix(in srgb, var(--brand) 8%, var(--background));
  color: var(--foreground);
  font: 600 0.78rem/1 var(--font-sans);
  letter-spacing: -0.011em;
  cursor: pointer;
  transition:
    background 140ms ease,
    border-color 140ms ease,
    color 140ms ease;
}

/* The sparkle is the one accent. It is the brand mixed towards the label's ink:
   the brand alone is a surface colour, and measured against the button's own
   background it drops to 2.2:1 on the light palettes while the chart set drops to
   2.0:1 on the dark ones. Mixed with the ink it keeps the palette's hue and stays
   legible on every theme. */
.block__prompt svg {
  color: color-mix(in srgb, var(--brand) 55%, var(--foreground));
}

.block__prompt:hover {
  border-color: var(--brand);
  background: color-mix(in srgb, var(--brand) 16%, var(--background));
}

/* Copied is a result, so it says so in the accent rather than a wash of it. */
.block__prompt[data-copied] {
  border-color: var(--brand);
  background: var(--brand);
  color: var(--brand-contrast);
}

.block__prompt[data-copied] svg {
  color: var(--brand-contrast);
}

.block__action {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 2rem;
  height: 2rem;
  border-radius: var(--radius-control);
  color: var(--muted-foreground);
  cursor: pointer;
}

.block__action:hover,
.block__action.is-active {
  background: var(--secondary);
  color: var(--foreground);
}


.block__frame {
  height: clamp(26rem, 62vh, 44rem);
  overflow: hidden;
  border: 1px solid var(--border);
  border-radius: var(--radius-surface);
  background: var(--card);
}

/* Centred, so a narrow device width reads as a device rather than as a frame
   that failed to fill its container. */
.block__sizer {
  position: relative;
  width: 100%;
  height: 100%;
  margin-inline: auto;
}

.block__iframe {
  display: block;
  width: 100%;
  height: 100%;
  border: 0;
}

/* The frame boots out of sight behind the picture: it is loaded and running, but
   the reader keeps looking at the block until the gallery reports its first
   frame, which is what the loader is waiting for. */
.block__iframe:not(.block__iframe--live) {
  visibility: hidden;
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

/* A frame with nothing to load. The status styling already reads as a note, so
   this only keeps a sentence from running the full width of the frame. */
.block__status--note {
  padding-inline: 2rem;
  text-align: center;
}

/* Disabled means the frame is not there yet, so the control has nothing to act
   on and must not pretend otherwise. */
.block__action:disabled {
  opacity: 0.4;
  cursor: default;
}

.block__action:disabled:hover {
  background: transparent;
  color: var(--muted-foreground);
}

/* The offer in place of a preview nobody asked for. It is the same box as every
   other frame state, so asking for the preview moves nothing. With a picture it
   carries no padding: the picture is the box. */
.block__status--offer {
  flex-direction: column;
  gap: 1rem;
  padding: 1.5rem;
  text-align: center;
}

.block__status--offer:has(.block__poster) {
  padding: 0;
}

/* The whole picture is the control. It shows what the block is; the pointer or
   the keyboard brings up the call to action over it. */
.block__poster {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  padding: 0;
  border: 0;
  background: var(--card);
  color: inherit;
  cursor: pointer;
}

.block__poster-image {
  display: block;
  width: 100%;
  height: 100%;
  object-fit: contain;
  transition: filter 180ms ease;
}

.block__poster:hover .block__poster-image,
.block__poster:focus-visible .block__poster-image {
  filter: brightness(0.5);
}

.block__poster:focus-visible {
  outline: 2px solid var(--brand);
  outline-offset: -2px;
}

/* The call to action, over the picture, only while the pointer or the keyboard
   is on it. `:disabled` is the moment after the click, when the loader owns the
   box. */
.block__poster-cta {
  position: absolute;
  display: inline-flex;
  align-items: center;
  gap: 0.45rem;
  min-height: 2.5rem;
  padding: 0 1.1rem;
  border: 1px solid color-mix(in srgb, var(--foreground) 30%, transparent);
  border-radius: var(--radius-control);
  background: color-mix(in srgb, var(--background) 88%, transparent);
  color: var(--foreground);
  font-size: 0.875rem;
  font-weight: 560;
  opacity: 0;
  transform: translateY(0.25rem);
  transition:
    opacity 160ms ease,
    transform 160ms ease;
  backdrop-filter: blur(3px);
}

.block__poster:hover .block__poster-cta,
.block__poster:focus-visible .block__poster-cta {
  opacity: 1;
  transform: none;
}

.block__poster-size {
  color: var(--muted-foreground);
  font-weight: 460;
}

.block__poster:disabled {
  cursor: default;
}

/* The wait sits over the picture rather than replacing it, so the reader keeps
   seeing what they asked for while it arrives. The scrim is opaque enough to
   hide the frame booting behind it — that is what keeps the gallery's own
   "Loading…" out of sight. */
.block__status--loading {
  background: color-mix(in srgb, var(--card) 94%, transparent);
}

.block__offer-text {
  max-width: 36ch;
  margin: 0;
  line-height: 1.6;
}

.block__offer {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  min-height: 2.25rem;
  padding: 0 1rem;
  border: 1px solid var(--border);
  border-radius: var(--radius-control);
  background: var(--background);
  color: var(--foreground);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition:
    background 150ms ease,
    border-color 150ms ease;
}

.block__offer:hover {
  border-color: var(--brand);
  background: var(--secondary);
}

.block__offer:focus-visible {
  outline: 2px solid var(--brand);
  outline-offset: 2px;
}
</style>
