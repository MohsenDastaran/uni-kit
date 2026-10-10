<script setup lang="ts">
import { onBeforeUnmount, ref } from "vue";
import {
    ArrowRight,
    Check,
    Copy,
    Monitor,
    Scale,
    Zap,
    Blocks,
} from "lucide-vue-next";
import LaserField from "./LaserField.vue";
import DataFeed from "./DataFeed.vue";

/**
 * The hero, re-arranged around two open-source blocks:
 *   - LaserField (ThreeUI "matrix-field" WebGL) behind everything,
 *   - DataFeed (reverseui "DataFeedingIn") against the right edge.
 * The left column keeps the existing title, lead, buttons, signals and install.
 */

const props = defineProps<{
    copy: Record<string, any>;
    gettingStartedHref: string;
    componentsHref: string;
    starterHref: string;
    componentCount: number;
    frameworkCount: number;
    installCommand: string;
}>();

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
            <LaserField :pointer="false" />
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
                    <a :href="starterHref" class="btn btn--primary">
                        {{ copy.ctaPrimary }}
                        <Zap :size="16" class="cta-icon cta-icon--scale" />
                    </a>
                    <a :href="componentsHref" class="btn">
                        {{ copy.ctaSecondary }}
                        <ArrowRight
                            :size="16"
                            class="cta-icon cta-icon--shift"
                        />
                    </a>
                </div>
                <ul class="hero__signals">
                    <li v-if="componentCount > 0">
                        <Blocks :size="14" /><strong>{{
                            componentCount
                        }}</strong>
                        {{ copy.signalComponents }}
                        <strong v-if="frameworkCount > 1">{{
                            frameworkCount
                        }}</strong>
                        {{ copy.signalFrameworks }}
                    </li>

                    <li><Monitor :size="14" /> {{ copy.signalPlatforms }}</li>
                </ul>
            </div>

            <div class="hero__visual">
                <DataFeed />
            </div>
        </div>
    </section>
</template>

<style>
/* The hero follows the page's theme like every other band: the junction draws
   its beams over whatever surface the theme hands it, inking the page in light
   and glowing on it in dark. The `.hero` prefix outranks the theme's
   `.hero__*` rules. */
.hero {
    isolation: isolate;
    /* GPUI's mark in the picker carries blue; the Slint mark is monochrome, so
       the conduit below follows the same two. */
    --feed-accent: #3b82f6;
}

html[data-framework="slint"] .hero {
    --feed-accent: var(--muted-foreground);
}

.hero__laser {
    position: absolute;
    inset: 0;
    z-index: 0;
    background: var(--background);
}

.hero__laser .laser-field {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    display: block;
    /* The junction's centre sits left of middle; nudge the whole field right.
     The strip this leaves on the left is the surface behind it. */
    transform: translateX(4%);
    /* The beams are additive, so over a light surface they are multiplied into
       ink instead of disappearing into it. */
    mix-blend-mode: multiply;
}

html.dark .hero__laser .laser-field {
    mix-blend-mode: screen;
}

.hero__inner {
    z-index: 1;
}

.hero .hero__copy {
    color: var(--foreground);
}

.hero .eyebrow {
    background: var(--secondary);
    border-color: var(--border);
    color: var(--secondary-foreground) !important;
}

.hero .hero h1,
.hero .hero__lead {
    color: var(--foreground);
}

.hero .hero__lead {
    color: var(--muted-foreground);
}

.hero .hero__actions {
    gap: 1rem;
}

.hero .hero__actions .btn {
    min-height: 2.75rem;
    padding: 0 2rem;
    gap: 0.5rem;
    border-radius: var(--radius-control);
    border-color: var(--border);
    background: var(--card);
    color: var(--foreground) !important;
    font-size: 0.875rem;
    font-weight: 500;
    transition:
        background 150ms ease,
        border-color 150ms ease,
        color 150ms ease;
}

.hero .hero__actions .btn:focus-visible {
    outline: 2px solid var(--brand);
    outline-offset: 2px;
}

/* The icon answers the pointer: the sparkle swells, the arrow steps forward. */
.hero .hero__actions .cta-icon {
    transition: transform 160ms ease;
}

.hero .hero__actions .btn:hover .cta-icon--scale {
    transform: scale(1.12);
}

.hero .hero__actions .btn:hover .cta-icon--shift {
    transform: translateX(0.25rem);
}

@media (max-width: 640px) {
    .hero .hero__actions {
        flex-direction: column;
        align-items: stretch;
    }
    .hero .hero__actions .btn {
        width: 100%;
    }
}

.hero .hero__actions .btn--primary {
    background: var(--brand);
    border-color: var(--brand);
    color: var(--brand-contrast) !important;
}

.hero .hero__signals li {
    color: var(--muted-foreground);
}

.hero .hero__signals strong {
    color: var(--foreground);
}

.hero .hero__install {
    background: var(--card);
    border-color: var(--border);
}

.hero .hero__install code {
    color: var(--foreground);
}

.hero .hero__install button {
    color: var(--muted-foreground);
}

.hero .hero__install button:hover {
    background: var(--secondary);
    color: var(--foreground);
}

/* Right column: the data card sits against the right edge of the hero. */
.hero__visual {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    /* Tracks the viewport when it is short, so a landscape phone does not
       scroll a full screen of empty column before the card. */
    min-height: min(480px, 66vh);
    /* Centring inside a shorter content box lifts the card, so the conduit
       reaches the navbar the picker sits in instead of stopping short of it. */
    padding-bottom: 44px;
}

/* ── Responsive ──────────────────────────────────────────────────────────
   Three bands.

   Phone (under 768): the copy alone on the dark ground. Stacked, the junction
   runs its beam the width of the screen and the card reads as a fragment.

   Tablet (768 to 1080): the junction still goes, for the same reason, but the
   card stays -- beside the copy as it is on the desktop, and smaller, because
   the column is narrower.

   Desktop (1081 up): both, as authored. */
@media (max-width: 1080px) {
    .hero__laser .laser-field {
        display: none;
    }
    /* A static wash where the junction was, so the hero is not a flat slab.
       It takes the same accent the conduit used, so the stacked hero still
       answers the framework picker. */
    .hero__laser {
        background:
            radial-gradient(
                120% 80% at 12% 0%,
                color-mix(in srgb, var(--feed-accent) 22%, transparent),
                transparent 62%
            ),
            radial-gradient(
                90% 70% at 100% 100%,
                color-mix(in srgb, var(--feed-accent) 12%, transparent),
                transparent 58%
            ),
            var(--background);
    }
}

/* Tablet: two columns again, card to the right, capped so it stays whole in a
   column that is about 44% of the width. The cap carries the height with it,
   since the card is proportional. */
@media (min-width: 768px) and (max-width: 1080px) {
    .hero .hero__inner {
        grid-template-columns: minmax(0, 1fr) minmax(0, 0.8fr);
    }
    .hero__visual {
        min-height: 0;
        padding-top: 0;
        padding-bottom: 0;
        justify-content: flex-end;
    }
    .data-feed,
    .data-feed__card {
        max-width: 300px;
    }
}

/* Phones: the card goes with the junction. */
@media (max-width: 767px) {
    .hero__visual {
        display: none;
    }
}

@media (max-width: 640px) {
    .hero .hero__inner {
        padding-block: 3.25rem;
    }
}

/* The title's clamp bottoms out at 2.2rem, which is five lines in a 288px
   column on the smallest phones. One step down fits it to four. */
@media (max-width: 420px) {
    .hero .hero__copy h1 {
        font-size: 1.95rem;
    }
}

/* A landscape phone, or a laptop window pulled short: the hero should reach
   its actions without a screen of padding above them. */
@media (max-height: 660px) {
    .hero .hero__inner {
        padding-block: 2rem;
    }
    .hero__visual {
        min-height: 0;
        padding-bottom: 0;
    }
}

/* Short but wide, where the two columns are kept and the illustration with
   them. There is no room for the conduit at this height -- it reaches ~122px
   above the card's own top and rode up under the navbar at 1280x500 -- so the
   card keeps the piece and its animation, shortened. */
@media (max-height: 660px) and (min-width: 768px) {
    .data-feed__pulse {
        display: none;
    }
    .data-feed__card {
        max-height: 200px;
    }
}
</style>
