<script setup lang="ts">
import { onBeforeUnmount, ref } from "vue";
import { ArrowRight, Check, Copy, Monitor, Scale, Star } from "lucide-vue-next";
import LaserField from "./LaserField.vue";
import AsciiLogos from "./AsciiLogos.vue";
import DataFeed from "./DataFeed.vue";

/**
 * The hero, re-arranged around three open-source blocks:
 *   - LaserField (ThreeUI "matrix-field" WebGL) behind everything,
 *   - DataFeed (reverseui "DataFeedingIn") on the right,
 *   - AsciiLogos (ascii.rest "rust", cycling rust -> gpui -> slint) on top of it.
 * The left column keeps the existing title, lead, buttons, signals and install.
 */

const props = defineProps<{
  copy: Record<string, any>;
  gettingStartedHref: string;
  componentsHref: string;
  starCount: number;
  installCommand: string;
}>();

const stars = props.starCount;
const starLabel = stars >= 1000 ? `${(stars / 1000).toFixed(1)}k` : `${stars}`;

const installSnippet = ["[dependencies]", props.installCommand].join("\n");

const copied = ref(false);
let copyTimer: ReturnType<typeof setTimeout> | undefined;
const copyInstall = async () => {
  try {
    await navigator.clipboard.writeText(installSnippet);
    copied.value = true;
    clearTimeout(copyTimer);
    copyTimer = setTimeout(() => (copied.value = false), 1600);
  } catch {}
};
onBeforeUnmount(() => clearTimeout(copyTimer));
</script>

<template>
  <section class="hero">
    <div class="hero__grid" aria-hidden="true"></div>
    <div class="hero__laser" aria-hidden="true">
      <LaserField />
    </div>

    <div class="hero__inner">
      <div class="hero__copy">
        <span class="eyebrow">
          <span class="eyebrow__pulse" aria-hidden="true"></span>
          {{ copy.eyebrow }}
        </span>
        <h1>{{ copy.title }}</h1>
        <p class="hero__lead">{{ copy.lead }}</p>
        <div class="hero__actions">
          <a :href="gettingStartedHref" class="btn btn--primary">
            {{ copy.startComponent }} <ArrowRight :size="16" />
          </a>
          <a :href="componentsHref" class="btn">{{ copy.componentsAction }}</a>
        </div>
        <ul class="hero__signals">
          <li><Star :size="14" /><strong>{{ starLabel }}</strong> {{ copy.signalStars }}</li>
          <li><Scale :size="14" /> {{ copy.signalLicense }}</li>
          <li><Monitor :size="14" /> {{ copy.signalPlatforms }}</li>
        </ul>
        <div class="hero__install">
          <span class="hero__install-label">Cargo.toml</span>
          <code>{{ installCommand }}</code>
          <button type="button" :aria-label="copy.copyLabel" :data-copied="copied || null" @click="copyInstall">
            <Check v-if="copied" :size="13" />
            <Copy v-else :size="13" />
          </button>
        </div>
      </div>

      <div class="hero__visual">
        <div class="hero__ascii"><AsciiLogos /></div>
        <DataFeed />
      </div>
    </div>
  </section>
</template>

<style>
/* The hero is the one dark section on a themed page: the matrix junction draws
   on black, so the copy and the data card switch to light ink over it. The
   `.hero` prefix outranks the theme's `.hero__*` rules. */
.hero {
  isolation: isolate;
}

.hero__laser {
  position: absolute;
  inset: 0;
  z-index: 0;
  background: #020307;
}

.hero__laser .laser-field {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  display: block;
}

.hero__inner {
  z-index: 1;
}

.hero .hero__copy {
  color: #f5f6f8;
}

.hero .eyebrow {
  background: rgb(255 255 255 / 0.06);
  border-color: rgb(255 255 255 / 0.14);
  color: #e6e9ef !important;
}

.hero .hero h1,
.hero .hero__lead {
  color: #f5f6f8;
}

.hero .hero__lead {
  color: #b9c0cc;
}

.hero .hero__actions .btn {
  border-color: rgb(255 255 255 / 0.14);
  background: rgb(255 255 255 / 0.04);
  color: #e6e9ef !important;
}

.hero .hero__actions .btn--primary {
  background: var(--brand);
  border-color: var(--brand);
  color: var(--brand-contrast) !important;
}

.hero .hero__signals li {
  color: #98a1af;
}

.hero .hero__signals strong {
  color: #e6e9ef;
}

.hero .hero__install {
  background: rgb(255 255 255 / 0.04);
  border-color: rgb(255 255 255 / 0.14);
}

.hero .hero__install code {
  color: #e6e9ef;
}

.hero .hero__install button {
  color: #98a1af;
}

.hero .hero__install button:hover {
  background: rgb(255 255 255 / 0.1);
  color: #e6e9ef;
}

/* Right column: the data card with the ascii logo floating over its corner. */
.hero__visual {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 420px;
}

.hero__ascii {
  position: absolute;
  top: 0;
  right: 0;
  z-index: 2;
  pointer-events: none;
  color: #e8ebf1;
}

.ascii-logos {
  margin: 0;
  font: 400 9px/1 var(--font-mono);
  white-space: pre;
  transition: opacity 300ms ease;
  opacity: 1;
}

@media (max-width: 760px) {
  .hero__visual {
    min-height: 340px;
  }
  .hero__ascii {
    position: static;
    display: flex;
    justify-content: center;
  }
}
</style>
