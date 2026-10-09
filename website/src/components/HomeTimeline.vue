<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue";

/**
 * The project's trajectory, as the journey it actually was: a rail that fills
 * as it travels, one stop per milestone, in the order they happened.
 *
 * On a wide screen with motion allowed, page scroll carries the track sideways
 * through a pinned stage — the shape the Hyperiux timeline uses. Everywhere
 * else it stays a track the reader scrolls, or a column on a narrow screen,
 * and every stop is still reachable.
 */

type Milestone = {
    id: string;
    /** Shown above the title, at the size of a caption. */
    period: string;
    title: string;
    body: string;
    /**
     * Marks the stop where this project's own work begins: one emoji, one
     * line, and a dot that announces itself when the rail reaches it.
     */
    origin?: {
        emoji: string;
        label: string;
    };
};

const milestones: Milestone[] = [
    {
        id: "gpui-component",
        period: "2024 · June",
        title: "GPUI Component",
        body: "The library starts on GPUI: one window, one button, and a component model meant to carry a whole application.",
    },
    {
        id: "gpui-kit",
        period: "2026",
        title: "GPUI Kit",
        body: "One dependency instead of five. gpui-kit puts GPUI, Base and the components at the crate root, so an application imports the toolkit and nothing else.",
    },
    {
        id: "fork",
        period: "August 2026",
        title: "The fork",
        body: "uni-kit forks GPUI Kit to carry the same ecosystem beyond a single framework.",
        origin: {
            emoji: "🌱",
            label: "Our work starts here",
        },
    },
    {
        id: "slint",
        period: "September 2026",
        title: "Slint support",
        body: "A second target: the components written for Slint, beside a registry that installs them into a project.",
    },
    {
        id: "blocks",
        period: "October 2026",
        title: "Blocks and starter kit",
        body: "Three installable GPUI screens — sidebar, dock, settings — and forkable Slint templates, so the first change in a project is your own.",
    },
    {
        id: "next",
        period: "Next",
        title: "Every GUI",
        body: "The registry keeps a component's source apart from the framework that draws it. The next targets are the frameworks you ask for.",
    },
];

const section = ref<HTMLElement | null>(null);
const journey = ref<HTMLElement | null>(null);
const track = ref<HTMLElement | null>(null);
/** Dots the rail has reached, by milestone id. */
const lit = ref<Set<string>>(new Set());
/** Cards the reader has arrived at, by milestone id. */
const reached = ref<Set<string>>(new Set());

const REDUCED_MOTION = "(prefers-reduced-motion: reduce)";
const WIDE = "(min-width: 900px)";

let frame = 0;
/** Whether page scroll drives the track through a pinned stage. */
let driven = false;
/** How far the track travels over the pinned range, in pixels. */
let travel = 0;
/** Where each dot lights and each card arrives, as a fraction of the journey. */
let dotStops: number[] = [];
let cardStops: number[] = [];
/** Where the rail starts and how long it is: first stop to last stop. */
let railStart = 0;
let railLength = 1;
/** The column's own height and where it sits on the page, for the scroll map. */
let railHeight = 0;
let railTop = 0;

function clamp(value: number) {
    return Math.min(1, Math.max(0, value));
}

const itemElements = () =>
    Array.from(
        track.value?.querySelectorAll<HTMLElement>(".journey__item") ?? [],
    );

/**
 * Measure the journey: whether it is driven, how long the page must be for it
 * to finish, and where each stop is crossed.
 */
function measure() {
    const journeyEl = journey.value;
    const trackEl = track.value;
    if (!journeyEl || !trackEl) return;

    const items = itemElements();
    const itemsEl = trackEl.querySelector<HTMLElement>(".journey__items");
    const viewportWidth = window.innerWidth;
    const viewportHeight = window.innerHeight;
    const reduced = window.matchMedia(REDUCED_MOTION).matches;

    driven = window.matchMedia(WIDE).matches && !reduced;

    // Distance from the track's leading padding to the row of stops, so that
    // layout offsets and the transform are measured against the same origin.
    const padStart =
        itemsEl && trackEl
            ? itemsEl.getBoundingClientRect().left -
              trackEl.getBoundingClientRect().left
            : 0;

    if (!driven) {
        journeyEl.style.removeProperty("height");
        trackEl.style.removeProperty("transform");
        travel = 0;
    } else {
        const last = items[items.length - 1];
        const lastCentre = last
            ? padStart + last.offsetLeft + last.offsetWidth / 2
            : 0;
        // The journey ends with the last stop settled inside the band rather
        // than against the window's edge.
        travel = Math.max(0, lastCentre - viewportWidth * 0.55);
        // The journey owns the page length: the stage inside it pins for as
        // long as the track still has ground to cover.
        journeyEl.style.height = `${Math.round(viewportHeight + travel)}px`;
    }

    // Where the stops sit along the rail, measured from the container that
    // both the rail and the dots are laid out in. The rail spans the first stop
    // to the last, so it neither starts above the journey nor trails past it.
    const railPositions = items.map((item) => {
        const dot = item
            .querySelector<HTMLElement>(".journey__dot")
            ?.getBoundingClientRect();
        if (!dot) return driven ? item.offsetLeft : item.offsetTop;
        return driven
            ? dot.left + dot.width / 2 - (itemsEl?.getBoundingClientRect().left ?? 0)
            : dot.top + dot.height / 2 - (itemsEl?.getBoundingClientRect().top ?? 0);
    });
    railStart = railPositions[0] ?? 0;
    railLength = Math.max(
        1,
        (railPositions[railPositions.length - 1] ?? 0) - railStart,
    );
    journeyEl.style.setProperty("--journey-rail-start", `${railStart.toFixed(2)}px`);
    journeyEl.style.setProperty("--journey-rail-length", `${railLength.toFixed(2)}px`);

    if (driven) {
        dotStops = railPositions.map((position) =>
            clamp((position - railStart) / railLength),
        );
        // A card fades in as its leading edge enters the view, so the reader
        // meets it on the way in rather than as an empty slot.
        cardStops = items.map((item) =>
            clamp(
                (padStart + item.offsetLeft - viewportWidth) /
                    Math.max(1, travel),
            ),
        );
    } else {
        const itemsRect = itemsEl?.getBoundingClientRect();
        railHeight = itemsEl?.offsetHeight ?? 0;
        railTop = (itemsRect?.top ?? 0) + window.scrollY;
        dotStops = railPositions.map((position) => clamp(position / railLength));
        cardStops = items.map((item) =>
            clamp(
                (item.offsetTop + item.offsetHeight - viewportHeight * 0.9) /
                    Math.max(1, railHeight),
            ),
        );
    }

    // The stylesheet holds the cards back only once this has run, so the
    // server-rendered band is readable before — and without — hydration.
    section.value?.setAttribute("data-ready", "true");
    section.value?.toggleAttribute("data-driven", driven);

    paint();
}

/** Follow the scroll: fill the rail, move the track, reveal the stops. */
function paint() {
    const journeyEl = journey.value;
    const trackEl = track.value;
    if (!journeyEl || !trackEl) return;

    if (window.matchMedia(REDUCED_MOTION).matches) {
        // Nothing moves: the rail is drawn in full and every stop is in place.
        journeyEl.style.setProperty("--journey-head", "100%");
        lit.value = new Set(milestones.map((milestone) => milestone.id));
        reached.value = new Set(milestones.map((milestone) => milestone.id));
        return;
    }

    let progress: number;
    if (driven) {
        const top =
            journeyEl.getBoundingClientRect().top + window.scrollY;
        const range = Math.max(1, journeyEl.offsetHeight - window.innerHeight);
        progress = clamp((window.scrollY - top) / range);
        trackEl.style.transform = `translate3d(${Math.round(-progress * travel)}px, 0, 0)`;
    } else {
        // A column: the rail fills as it passes the fold.
        progress = clamp(
            (window.scrollY + window.innerHeight * 0.85 - railTop) /
                Math.max(1, railHeight),
        );
    }

    journeyEl.style.setProperty(
        "--journey-head",
        `${Math.round(progress * railLength)}px`,
    );

    const nextLit = new Set<string>();
    const nextReached = new Set<string>();
    milestones.forEach((milestone, index) => {
        if (progress >= (dotStops[index] ?? 0)) nextLit.add(milestone.id);
        if (progress >= (cardStops[index] ?? 0)) nextReached.add(milestone.id);
    });
    if (nextLit.size !== lit.value.size) lit.value = nextLit;
    if (nextReached.size !== reached.value.size) reached.value = nextReached;
}

function schedule() {
    if (frame) return;
    frame = window.requestAnimationFrame(() => {
        frame = 0;
        paint();
    });
}

function onResize() {
    measure();
}

const queries: MediaQueryList[] = [];

onMounted(() => {
    queries.push(window.matchMedia(REDUCED_MOTION), window.matchMedia(WIDE));
    measure();
    window.addEventListener("scroll", schedule, { passive: true });
    window.addEventListener("resize", onResize);
    for (const query of queries) query.addEventListener("change", onResize);
    // The band is sized by its content, and a late font moves every stop.
    document.fonts?.ready.then(onResize).catch(() => {});
});

onBeforeUnmount(() => {
    if (frame) window.cancelAnimationFrame(frame);
    window.removeEventListener("scroll", schedule);
    window.removeEventListener("resize", onResize);
    for (const query of queries) query.removeEventListener("change", onResize);
});
</script>

<template>
    <section
        ref="section"
        class="band band--journey"
        aria-labelledby="journey-title"
    >
        <div class="band__inner">
            <header class="section-head journey__head">
                <span class="section-kicker">Trajectory</span>
                <h2 id="journey-title">From one library to every GUI</h2>
                <p>
                    Where the project has been, in the order it happened, and
                    where it goes next.
                </p>
            </header>
        </div>

        <div ref="journey" class="journey">
            <div class="journey__stage">
                <div class="journey__viewport">
                    <div ref="track" class="journey__track">
                        <div class="journey__items">
                            <span class="journey__rail" aria-hidden="true" />
                            <span class="journey__fill" aria-hidden="true" />

                            <article
                                v-for="(milestone, index) in milestones"
                                :key="milestone.id"
                                class="journey__item"
                                :class="{
                                    'is-reached': reached.has(milestone.id),
                                    'is-lit': lit.has(milestone.id),
                                    'is-origin': Boolean(milestone.origin),
                                    'journey__item--after': index % 2 === 1,
                                }"
                            >
                                <span class="journey__dot" aria-hidden="true" />
                                <div class="journey__card">
                                    <p
                                        v-if="milestone.origin"
                                        class="journey__origin"
                                    >
                                        <span aria-hidden="true">{{
                                            milestone.origin.emoji
                                        }}</span>
                                        {{ milestone.origin.label }}
                                    </p>
                                    <p class="journey__period">
                                        {{ milestone.period }}
                                    </p>
                                    <h3>{{ milestone.title }}</h3>
                                    <p class="journey__body">
                                        {{ milestone.body }}
                                    </p>
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>
</template>

<style>
/* The rail is drawn from the page's own tokens, so it follows the theme and
   whichever palette the reader has picked: `--brand` is the theme's primary. */
.band--journey {
    border-top: 1px solid var(--border);
    background: var(--background);
}

.journey__head {
    margin-bottom: 0;
    padding-bottom: var(--section-gap, 4rem);
}

.journey {
    /* How much of the rail has been travelled. Scripting writes it; until then
       the journey has not started. */
    --journey-head: 0px;
    position: relative;
}

.journey__stage {
    display: flex;
    align-items: center;
}

.journey__viewport {
    width: 100%;
    overflow-x: auto;
    scroll-snap-type: x proximity;
    scrollbar-width: none;
}

.journey__viewport::-webkit-scrollbar {
    display: none;
}

.journey__track {
    width: max-content;
    padding-inline: clamp(1.5rem, 7vw, 7rem) clamp(2rem, 24vw, 20rem);
}

.journey__items {
    position: relative;
    display: flex;
    align-items: center;
    gap: clamp(1.75rem, 4vw, 4rem);
}

/* The line the stops sit on: a muted rail, and the length already travelled. */
.journey__rail,
.journey__fill {
    position: absolute;
    top: 50%;
    left: var(--journey-rail-start, 0px);
    height: 1px;
    border-radius: 999px;
}

.journey__rail {
    width: var(--journey-rail-length, 100%);
    background: var(--border);
}

.journey__fill {
    width: min(var(--journey-head), var(--journey-rail-length, 100%));
    background: var(--brand);
}

.journey__item {
    position: relative;
    flex: none;
    width: clamp(15rem, 23vw, 20rem);
    min-height: clamp(19rem, 38vh, 26rem);
    display: grid;
    /* The middle track is the rail: the card above it and the card below it
       both stop short of the line. */
    grid-template-rows: 1fr auto 1fr;
    row-gap: 0.75rem;
    scroll-snap-align: center;
}

/* Every other stop hangs below the rail, so the row reads as a journey rather
   than a table of contents. */
.journey__card {
    grid-row: 1;
    align-self: end;
    padding: clamp(1rem, 1.6vw, 1.35rem);
    border: 1px solid var(--border);
    border-radius: var(--radius-card);
    background: var(--card);
}

.journey__item--after .journey__card {
    grid-row: 3;
    align-self: start;
}

/* Held back only once scripting has measured the journey, so the band reads
   before hydration and would still read without it. */
.band--journey[data-ready] .journey__card {
    opacity: 0;
    translate: 0 1rem;
    transition:
        opacity 420ms ease,
        translate 420ms cubic-bezier(0.22, 1, 0.36, 1);
}

.band--journey[data-ready] .journey__item.is-reached .journey__card {
    opacity: 1;
    translate: 0 0;
}

/* The stop where this project's work begins. Everything else on the rail is
   history it inherited; this one is the line it draws itself, so it carries the
   project's colour and an emoji that says so. */
.journey__item.is-origin .journey__card {
    border-color: color-mix(in srgb, var(--brand) 42%, var(--border));
    background: color-mix(in srgb, var(--brand) 5%, var(--card));
}

.journey__origin {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    margin: 0 0 0.5rem;
    color: var(--brand);
    font-size: 0.75rem;
    font-weight: 620;
    letter-spacing: 0.02em;
    text-transform: uppercase;
}

.journey__origin span {
    font-size: 0.9375rem;
    letter-spacing: 0;
}

.journey__period {
    margin: 0 0 0.4rem;
    color: var(--muted-foreground);
    font-size: 0.75rem;
    font-weight: 560;
    letter-spacing: 0.045em;
    text-transform: uppercase;
}

.journey__card h3 {
    margin: 0 0 0.5rem;
    color: var(--foreground);
    font-size: 1.0625rem;
    font-weight: 640;
    letter-spacing: -0.014em;
}

.journey__body {
    margin: 0;
    color: var(--muted-foreground);
    font-size: 0.875rem;
    line-height: 1.5;
}

.journey__dot {
    position: absolute;
    top: 50%;
    left: 0;
    width: 0.625rem;
    height: 0.625rem;
    translate: -50% -50%;
    border: 1px solid var(--border);
    border-radius: 50%;
    background: var(--background);
    transition:
        background 240ms ease,
        border-color 240ms ease,
        scale 240ms ease;
}

.journey__item.is-lit .journey__dot {
    border-color: var(--brand);
    background: var(--brand);
    scale: 1.15;
}

/* The origin stop is bigger than the rest, keeps a halo once the rail has
   reached it, and announces itself in one movement — not a loop. */
.journey__item.is-origin .journey__dot {
    width: 0.75rem;
    height: 0.75rem;
    border-width: 2px;
}

.journey__item.is-origin.is-lit .journey__dot {
    box-shadow: 0 0 0 0.375rem
        color-mix(in srgb, var(--brand) 18%, transparent);
    animation: journey-origin 560ms cubic-bezier(0.22, 1, 0.36, 1) both;
}

@keyframes journey-origin {
    0% {
        scale: 1;
    }
    45% {
        scale: 1.7;
    }
    100% {
        scale: 1.2;
    }
}

/* Pinned: the stage holds still while page scroll carries the track past it.
   Scripting sets `data-driven` and the journey's own height, so this only
   applies where the page is long enough to drive it. The clipping lives on the
   stage rather than on an ancestor, which would break `position: sticky`. */
.band--journey[data-driven] .journey__viewport {
    overflow: visible;
}

.band--journey[data-driven] .journey__stage {
    position: sticky;
    top: 0;
    height: 100vh;
    overflow: clip;
}

.band--journey[data-driven] .journey__track {
    will-change: transform;
}

/* A narrow screen gets a column instead: the rail runs down the left, and the
   stops follow one another. */
@media (max-width: 899px) {
    .journey__viewport {
        overflow: visible;
    }

    .journey__track {
        width: auto;
        padding-inline: min(1.5rem, 5vw);
    }

    .journey__items {
        flex-direction: column;
        align-items: stretch;
        gap: 0;
    }

    .journey__item,
    .journey__item--after {
        width: 100%;
        min-height: 0;
        padding: 0 0 2rem 1.5rem;
        display: block;
    }

    .journey__item:last-child {
        padding-bottom: 0;
    }

    .journey__card,
    .journey__item--after .journey__card {
        grid-row: auto;
        align-self: stretch;
    }

    .journey__rail,
    .journey__fill {
        top: var(--journey-rail-start, 0px);
        left: 0;
        width: 1px;
        height: var(--journey-rail-length, 100%);
    }

    .journey__fill {
        height: min(var(--journey-head), var(--journey-rail-length, 100%));
    }

    .journey__dot {
        top: 0.45rem;
        left: 0;
    }
}

@media (prefers-reduced-motion: reduce) {
    .band--journey[data-ready] .journey__card,
    .band--journey[data-ready] .journey__item.is-reached .journey__card {
        opacity: 1;
        translate: 0 0;
        transition: none;
    }

    .journey__dot {
        transition: none;
    }

    .journey__item.is-origin.is-lit .journey__dot {
        animation: none;
    }
}
</style>
