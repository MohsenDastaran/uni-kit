<script setup lang="ts">
import { Check, Copy } from "lucide-vue-next";
import { computed, nextTick, onBeforeUnmount, onMounted, ref } from "vue";
import { toggleLogos } from "../lib/toggle-logos.js";
import catalog from "../lib/registry-slugs.json" with { type: "json" };
import FlowText from "./FlowText.vue";

const baseUrl = import.meta.env.BASE_URL;

const MANAGERS = ["npx", "pnpm", "bun"] as const;
type Manager = (typeof MANAGERS)[number];

/**
 * A component page lets the markdown pipeline decide both, so the bar carries
 * the command for that page. A block gallery renders one instance per block, so
 * each passes what to install and the element to render into.
 *
 * `slugs` is keyed by framework because the two agree on neither the names nor
 * their meaning. A Slint block is composed of controls, and every control is a
 * file that `add slint` copies. A GPUI block is one file of usage, because the
 * controls it composes ship in the `gpui-kit` crate.
 */
const props = defineProps<{
  slug?: string;
  slugs?: Partial<Record<"gpui" | "slint", string[]>>;
  host?: string;
}>();

/**
 * Whether the names came from a block rather than from a component page. A
 * block has to say so: the names are the same words either way, and a page that
 * documents a control cannot install one.
 */
const fromBlock = computed(() => Boolean(props.slugs));

const STORAGE_KEY = "selected-package-manager";
// The `storage` event only reaches *other* documents, so it never fires in the
// page that wrote the value. A component page has one command and never noticed;
// a block page has one per block, so the clicked block would update and the rest
// would keep the old manager. This event carries the change within the page.
const MANAGER_EVENT = "package-manager-change";
const PACKAGE = "@dastaran/uni-kit@latest";

// The component names to install. A component page installs one; a block
// installs every name its framework needs (see `slugs`).
const pageNames = ref<string[]>([]);
const targets = computed(() =>
  fromBlock.value ? (props.slugs?.[framework.value] ?? []) : pageNames.value,
);
const manager = ref<Manager>("npx");
const framework = ref<"gpui" | "slint">("gpui");
const copied = ref(false);
const status = ref("");
/**
 * Whether the bar may be teleported yet.
 *
 * Vue's server renderer sends a teleport's content to `ssrContext.teleports`,
 * and Astro's integration does not write those out: the server markup for this
 * component is an empty `<!--teleport start--><!--teleport end-->` pair. The
 * client hydrates the teleport with the command in it, which is a mismatch —
 * and an island that hydrated mismatched stops patching correctly, so a block
 * list that later shrank stayed on screen.
 *
 * Waiting until the component is mounted keeps the server's markup and the
 * client's first pass identical: both render nothing there. The bar appears as
 * the island mounts, which is when it had the content to show anyway.
 */
const clientReady = ref(false);
let timer: ReturnType<typeof setTimeout> | undefined;
let observer: MutationObserver | undefined;

const isManager = (value: unknown): value is Manager =>
  MANAGERS.includes(value as Manager);

const runner = computed(() =>
  manager.value === "pnpm"
    ? "pnpm dlx"
    : manager.value === "bun"
      ? "bunx"
      : "npx",
);

const namesFor = (name: string) =>
  (catalog as Record<string, string[]>)[name] ?? [];

/** A registry entry copies files into the app. uni-kit itself is the crate. */
const packaged = computed(
  () =>
    targets.value.length > 0 &&
    targets.value.every((name) => namesFor(framework.value).includes(name)) &&
    // The GPUI names that reach the registry are block files, so a component
    // page naming the same word (`sidebar`) is not offering to install it.
    (framework.value !== "gpui" || fromBlock.value),
);
const crateInstall = computed(
  () => framework.value === "gpui" && !packaged.value,
);

/**
 * The crate the installed files are written against. A GPUI block imports
 * `gpui_kit`, so the dependency is the other half of the same setup and the bar
 * carries both.
 */
const crateCommand = computed(() =>
  packaged.value && framework.value === "gpui" ? "cargo add gpui-kit" : "",
);

const blockCommand = computed(() => {
  if (targets.value.length === 0) return "";
  // One command for the whole screen: the CLI takes every name and writes the
  // files they share once.
  return `${runner.value} ${PACKAGE} add ${framework.value} ${targets.value.join(" ")}`;
});

const command = computed(() => {
  if (targets.value.length === 0) return "";
  if (packaged.value) return [crateCommand.value, blockCommand.value].filter(Boolean).join("\n");
  if (crateInstall.value) return "cargo add gpui-kit";
  return "";
});

const unavailableLabel = "No files to install for this framework.";
const crateLabel = "Ships in the gpui-kit crate.";
const crateInstalledLabel = "the components come from this crate";
const copyLabel = "Copy command";
const copiedLabel = "Copied";
const managerLabel = "Package manager";

function readManager(): Manager {
  const stored = localStorage.getItem(STORAGE_KEY);
  return isManager(stored) ? stored : "npx";
}

function readFramework() {
  return document.documentElement.dataset.framework === "slint"
    ? "slint"
    : "gpui";
}

function onStorage(event: StorageEvent) {
  if (event.key === STORAGE_KEY && isManager(event.newValue))
    manager.value = event.newValue;
}

function onManagerChange() {
  manager.value = readManager();
}

function mountFrameworkLogos() {
  document
    .querySelectorAll<HTMLButtonElement>("[data-framework-option]")
    .forEach((button) => {
      if (
        button.querySelector("svg") ||
        button.closest("[data-framework-select]")
      )
        return;
      const name = button.dataset.frameworkOption;
      const logo = name
        ? toggleLogos[name as keyof typeof toggleLogos]
        : undefined;
      if (!logo) return;
      const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
      svg.setAttribute("class", "framework-switch__logo");
      svg.setAttribute("viewBox", logo.viewBox);
      svg.setAttribute("aria-hidden", "true");
      for (const path of logo.paths) {
        const el = document.createElementNS(
          "http://www.w3.org/2000/svg",
          "path",
        );
        el.setAttribute("d", path.d);
        el.setAttribute("fill", path.fill);
        if (path.accent)
          el.setAttribute("class", "framework-switch__logo-accent");
        svg.append(el);
      }
      button.prepend(svg);
    });
}

function pageSlug() {
  const match = location.pathname.match(/\/component\/([^/]+)\/?$/);
  const value = match?.[1];
  return !value || value === "index" ? "" : value;
}

onMounted(() => {
  mountFrameworkLogos();
  // A block names what it installs per framework. A component page has no such
  // list, so it derives the one component it installs from the bar the markdown
  // pipeline emitted.
  const page = [props.slug ?? hostSlug()].filter((name) => name && name !== "index");
  if (!fromBlock.value && page.length === 0) return;
  pageNames.value = page;
  manager.value = readManager();
  framework.value = readFramework();
  // The bar itself is client-side only; see `clientReady`.
  clientReady.value = true;
  observer = new MutationObserver(() => {
    framework.value = readFramework();
  });
  observer.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ["data-framework"],
  });
  window.addEventListener("storage", onStorage);
  window.addEventListener(MANAGER_EVENT, onManagerChange);
});

/**
 * The component a page-wide bar installs, read from the host the markdown
 * pipeline emitted. Creates that host when a component page has no bar of its
 * own, which is how the command lands in the framework bar.
 */
function hostSlug(): string {
  let host = document.querySelector<HTMLElement>(".install-command-host");
  if (!host) {
    const bar = document.querySelector(".doc-content .framework-bar");
    const value = pageSlug();
    if (!bar || !value) return "";
    host = document.createElement("div");
    host.className = "install-command-host";
    host.dataset.installSlug = value;
    bar.insertBefore(host, bar.querySelector("[data-framework-status]"));
  }
  return host.dataset.installSlug ?? "";
}

onBeforeUnmount(() => {
  observer?.disconnect();
  window.removeEventListener("storage", onStorage);
  window.removeEventListener(MANAGER_EVENT, onManagerChange);
  clearTimeout(timer);
});

function select(next: Manager) {
  manager.value = next;
  localStorage.setItem(STORAGE_KEY, next);
  window.dispatchEvent(new Event(MANAGER_EVENT));
}

function onKeydown(event: KeyboardEvent) {
  const step = {
    ArrowRight: 1,
    ArrowDown: 1,
    ArrowLeft: -1,
    ArrowUp: -1,
  }[event.key];
  const edge =
    event.key === "Home"
      ? 0
      : event.key === "End"
        ? MANAGERS.length - 1
        : undefined;
  if (step === undefined && edge === undefined) return;
  event.preventDefault();
  const index = MANAGERS.indexOf(manager.value);
  const next = edge ?? (index + step! + MANAGERS.length) % MANAGERS.length;
  select(MANAGERS[next]);
}

async function copy() {
  const text = command.value;
  if (!text) return;
  try {
    await navigator.clipboard.writeText(text);
  } catch {
    const area = document.createElement("textarea");
    area.value = text;
    area.setAttribute("readonly", "");
    area.style.position = "fixed";
    area.style.left = "-9999px";
    document.body.appendChild(area);
    area.select();
    const ok = document.execCommand("copy");
    area.remove();
    if (!ok) return;
  }
  copied.value = true;
  status.value = "";
  await nextTick();
  status.value = copiedLabel.value;
  clearTimeout(timer);
  timer = setTimeout(() => {
    copied.value = false;
    status.value = "";
  }, 1600);
}
</script>

<template>
  <div class="install-command-mount">
    <span class="sr-only" role="status">{{ status }}</span>
    <Teleport v-if="clientReady && targets.length" :to="props.host ?? '.install-command-host'">
      <div
        v-if="packaged"
        class="install-command__switch framework-switch"
        role="radiogroup"
        :aria-label="managerLabel"
        :data-selected="manager"
        @keydown="onKeydown"
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
          @click="select(name)"
        >
          <!-- Bun's mark is the mascot, so it is an image rather than a path. -->
          <img
            v-if="name === 'bun'"
            class="framework-switch__logo framework-switch__logo--bun"
            :src="`${baseUrl}bun.png`"
            alt=""
            aria-hidden="true"
          />
          <svg
            v-else
            class="framework-switch__logo"
            :viewBox="toggleLogos[name].viewBox"
            aria-hidden="true"
          >
            <path
              v-for="(path, index) in toggleLogos[name].paths"
              :key="index"
              :d="path.d"
              :fill="path.fill"
            />
          </svg>
          {{ name }}
        </button>
      </div>
      <div class="install-command__line">
        <div
          class="install-command__body"
          :class="{ 'install-command__body--stacked': Boolean(crateCommand) }"
        >
          <p v-if="!packaged && !crateInstall" class="install-command__note">
            {{ unavailableLabel }}
          </p>
          <template v-if="packaged">
            <!-- The dependency first: the file below does not compile without
                 it, and the application may not have it yet. -->
            <code v-if="crateCommand">
              <span class="install-command__run">cargo</span>
              <span>add</span>
              <span class="install-command__package">gpui-kit</span>
              <span class="install-command__comment"># {{ crateInstalledLabel }}</span>
            </code>
            <code>
              <FlowText class="install-command__run" :text="runner" />
              <span class="install-command__package">{{ PACKAGE }}</span>
              <span>add</span>
              <FlowText class="install-command__arg" :text="framework" />
              <span class="install-command__arg">{{ targets.join(" ") }}</span>
            </code>
          </template>
          <code v-else-if="crateInstall">
            <span class="install-command__run">cargo</span>
            <span>add</span>
            <span class="install-command__package">gpui-kit</span>
            <span class="install-command__comment"># {{ crateLabel }}</span>
          </code>
        </div>
        <button
          v-if="command"
          type="button"
          class="install-command__copy"
          :aria-label="copied ? copiedLabel : copyLabel"
          :title="copied ? copiedLabel : copyLabel"
          :data-copied="copied || null"
          @click="copy"
        >
          <Check v-if="copied" :size="14" aria-hidden="true" />
          <Copy v-else :size="14" aria-hidden="true" />
        </button>
      </div>
    </Teleport>
  </div>
</template>
