<script setup lang="ts">
import { onBeforeUnmount, ref } from "vue";
import {
    ArrowRight,
    Check,
    Copy,
    Monitor,
    Scale,
    Zap,
    Star,
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
                    <li>
                        <Star :size="14" /><strong>{{ starLabel }}</strong>
                        {{ copy.signalStars }}
                    </li>
                    <li><Scale :size="14" /> {{ copy.signalLicense }}</li>
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
/* The hero is the one dark section on a themed page: the matrix junction draws
   on black, so the copy and the data card switch to light ink over it. The
   `.hero` prefix outranks the theme's `.hero__*` rules. */
.hero {
    isolation: isolate;
    /* GPUI's mark in the picker carries blue; the Slint mark is monochrome, so
       the conduit below follows the same two. */
    --feed-accent: #3b82f6;
}

html[data-framework="slint"] .hero {
    --feed-accent: #cbd5e1;
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
    /* The junction's centre sits left of middle; nudge the whole field right.
     The strip this leaves on the left is the same near-black behind it. */
    transform: translateX(4%);
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

.hero .hero__actions {
    gap: 1rem;
}

.hero .hero__actions .btn {
    min-height: 2.75rem;
    padding: 0 2rem;
    gap: 0.5rem;
    border-radius: var(--radius-control);
    border-color: rgb(255 255 255 / 0.14);
    background: rgb(255 255 255 / 0.04);
    color: #e6e9ef !important;
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
   The hero stacks into one column at 1080px (its grid lives in HomeApp).
   Stacked, the conduit no longer lines up under the navbar picker, so the
   card centres and the lift that closed that gap is dropped with it. */
@media (max-width: 1080px) {
    .hero__visual {
        min-height: 0;
        padding-bottom: 0;
        padding-top: clamp(1.5rem, 4vw, 2.5rem);
        justify-content: center;
    }
}

@media (max-width: 640px) {
    .hero .hero__inner {
        padding-block: 2.75rem;
    }
    .hero__visual {
        padding-top: 1rem;
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

/* Short but wide -- a phone on its side. Two columns fit, and stacking there
   makes the hero twice the height of the screen, so it keeps them. The `.hero`
   prefix outranks the collapse rule in HomeApp. */
@media (max-height: 660px) and (min-width: 760px) {
    .hero .hero__inner {
        grid-template-columns: minmax(0, 0.93fr) minmax(0, 1.07fr);
    }
    .hero__visual {
        padding-top: 0;
        justify-content: flex-end;
    }
    /* The conduit is 202px of the column's height and reaches ~122px above the
       card's own top. There is no room for that here: it rode up under the
       navbar. The card keeps the piece and its animation, shortened. */
    .data-feed__pulse {
        display: none;
    }
    .data-feed__card {
        max-height: 200px;
    }
}
</style>
