<script setup lang="ts">
import {
    ArrowRight,
    Blocks,
    Braces,
    Check,
    Gauge,
    Layers3,
    LayoutDashboard,
    List,
    Palette,
    SquareCode,
    Table2,
} from "lucide-vue-next";

import Hero from "./hero/Hero.vue";

const props = defineProps<{
    starCount: number;
    base: string;
}>();

function url(path: string) {
    const b = props.base.replace(/\/$/, "");
    return `${b}/${path}`.replace(/\/+/g, "/");
}

const stars = props.starCount;

const gettingStartedHref = url("component");
const componentsHref = url("component");
const skillsHref = url("skills");
const llmsHref = url("llms-full.txt");

// Indent depth and token widths (rem) per line
const editorLines = [
    { indent: 0, tokens: [0.45, 1.5, 0.75] },
    { indent: 0, tokens: [0.9, 1.15, 1.8] },
    { indent: 1, tokens: [1.2, 0.8, 1.35] },
    { indent: 2, tokens: [0.7, 1.55] },
    { indent: 2, tokens: [1.05, 0.65, 0.9] },
    { indent: 1, tokens: [0.55, 1.25] },
    { indent: 0, tokens: [0.4] },
];

const frames = [
    72, 78, 74, 81, 76, 84, 79, 73, 88, 77, 82, 75, 86, 80, 71, 83, 76, 90, 74,
    79, 85, 72, 81, 77, 87, 75, 80, 73, 84, 78,
];

const capIcons: Record<string, any> = {
    perf: Gauge,
    table: Table2,
    list: List,
    editor: SquareCode,
    dock: LayoutDashboard,
    theme: Palette,
};

// `gpui-kit` is the one dependency an application needs: it pins GPUI and
// carries every layer. The line on screen is what the clipboard gets, with
// the `[dependencies]` header so it pastes straight into Cargo.toml.
const installCommand = 'gpui-kit = "0.6.0"';

const copy = {
    copyLabel: "Copy install command",
    eyebrow: "Forked from gpui-kit.",
    title: "One component collection for every GUI framework.",
    lead: "uni-kit is a universal, framework-agnostic ecosystem of UI components. with more GUI frameworks on the way.",

    componentsAction: "Browse components",
    baseAction: "Browse components",
    signalStars: "stars on GitHub",
    signalLicense: "Apache-2.0",
    signalPlatforms: "macOS, Windows, Linux",
    capsKicker: "Capabilities",
    capsTitle: "Built for information-dense software.",
    capsDescription:
        "The systems that real desktop applications need are integrated into one framework.",
    caps: [
        {
            icon: "perf",
            title: "120 FPS rendering",
            description:
                "Every frame is drawn by the GPU, so dense interfaces stay smooth instead of dropping frames.",
            apis: ["RenderOnce", "GPU"],
        },
        {
            icon: "table",
            title: "Complex data tables",
            description:
                "Virtual scrolling, fixed and resizable columns, sorting and cell selection across hundreds of thousands of rows.",
            apis: ["Table", "DataTable"],
        },
        {
            icon: "list",
            title: "Virtualized lists",
            description:
                "Only the visible range is rendered, so very long lists keep scrolling smoothly.",
            apis: ["VirtualList", "List"],
        },
        {
            icon: "editor",
            title: "A real code editor",
            description:
                "Rope-backed text that stays stable at 200K lines, with Tree-sitter highlighting and LSP diagnostics, completion and hover.",
            apis: ["Rope", "Tree-Sitter", "LSP", "Highlighter"],
        },
        {
            icon: "dock",
            title: "Freeform dock layout",
            description:
                "Dockable panels with drag-to-rearrange and zooming — all serializable.",
            apis: ["DockArea", "DockLayout", "TabGroup"],
        },
        {
            icon: "theme",
            title: "Multi-theme support",
            description:
                "Light, dark and custom themes driven by semantic tokens instead of endless style fields.",
            apis: ["Theme", "ThemeColor", "ActiveTheme"],
        },
    ],
    chooseKicker: "Two layers. One ecosystem.",
    chooseTitle: "Choose who owns the visual system.",
    chooseDescription:
        "Use gpui-component for a coherent product, or build and own your design system on gpui-base.",
    shipTitle: "Keep the product coherent",
    shipDescription:
        "gpui-component provides a complete, polished visual and interaction system ready to ship.",
    shipPoints: [
        "60+ finished components",
        "Light and dark themes included",
        "Interaction details already handled",
    ],
    startComponent: "Get started",
    ownTitle: "Own the design system",
    ownDescription:
        "Reuse focus, selection, overlay and virtualization behavior while owning every pixel.",
    ownPoints: [
        "Zero-style primitives",
        "Full accessibility behavior",
        "100% visual ownership",
    ],
    startBase: "Browse the components",
    scriptTitle: "Extend it in JavaScript",
    scriptDescription:
        "The host stays Rust and grants what a script may reach, one capability at a time; the script draws real interface in the same process.",
    scriptPoints: [
        "Extend the product without a fork or a release",
        "No capability granted by default",
        "Hot-reload on save, no restart",
    ],
    principleKicker: "Principle",
    principleLead: "Behavior belongs to the foundation.",
    principleTail: "Presentation belongs to the application.",
    principleDetail:
        "gpui-base handles the difficult interaction mechanics — focus, overlay positioning, virtualization and accessibility. Your product decides how they should look and feel.",
    footerPrefix: "Open source under the Apache-2.0 License, developed by",
    footerSuffix: ".",
    footerBuiltOn: "Built on",
    footerAttribution:
        ", the UI framework from Zed Industries, also Apache-2.0.",
    footerNav: "Footer navigation",
    contributors: "Contributors",
    reportBug: "Report Bug",
    discussion: "Discussion",
    iconCredits: "Icons by",
    and: "and",
    period: ".",
};
</script>

<template>
    <main class="home">
        <Hero
            :copy="copy"
            :getting-started-href="gettingStartedHref"
            :components-href="componentsHref"
            :star-count="stars"
            :install-command="installCommand"
        />

        <section class="band">
            <div class="band__inner">
                <header class="section-head">
                    <span class="section-kicker">{{ copy.capsKicker }}</span>
                    <h2>{{ copy.capsTitle }}</h2>
                    <p>{{ copy.capsDescription }}</p>
                </header>
                <div class="caps__grid">
                    <article
                        v-for="cap in copy.caps"
                        :key="cap.title"
                        class="cap"
                    >
                        <div class="cap__head">
                            <component :is="capIcons[cap.icon]" :size="17" />
                            <h3>{{ cap.title }}</h3>
                        </div>
                        <p>{{ cap.description }}</p>
                        <ul class="cap__api">
                            <li v-for="api in cap.apis" :key="api">
                                {{ api }}
                            </li>
                        </ul>
                        <div
                            class="cap__preview"
                            :class="`cap__preview--${cap.icon}`"
                            aria-hidden="true"
                        >
                            <template v-if="cap.icon === 'perf'">
                                <span class="cap__budget" />
                                <u
                                    v-for="(frame, i) in frames"
                                    :key="i"
                                    :style="{ height: `${frame}%` }"
                                />
                            </template>
                            <template v-else-if="cap.icon === 'table'">
                                <i v-for="n in 5" :key="n"><b /><b /><b /></i>
                            </template>
                            <template v-else-if="cap.icon === 'list'">
                                <i v-for="w in [86, 68, 92, 74, 60]" :key="w"
                                    ><b :style="{ width: `${w}%` }"
                                /></i>
                                <span class="cap__track"
                                    ><span class="cap__thumb"
                                /></span>
                            </template>
                            <template v-else-if="cap.icon === 'editor'">
                                <i
                                    v-for="(line, row) in editorLines"
                                    :key="row"
                                    :style="{
                                        paddingLeft: `${line.indent * 0.6}rem`,
                                    }"
                                >
                                    <s class="cap__gutter" />
                                    <b
                                        v-for="(w, i) in line.tokens"
                                        :key="i"
                                        :style="{ width: `${w}rem` }"
                                    />
                                </i>
                            </template>
                            <template v-else-if="cap.icon === 'dock'">
                                <b /><b /><b />
                            </template>
                            <template v-else>
                                <em v-for="t in 6" :key="t" />
                            </template>
                        </div>
                    </article>
                </div>
            </div>
        </section>

        <section class="band">
            <div class="band__inner">
                <header class="section-head">
                    <span class="section-kicker">{{ copy.chooseKicker }}</span>
                    <h2>{{ copy.chooseTitle }}</h2>
                    <p>{{ copy.chooseDescription }}</p>
                </header>
                <div class="paths__grid">
                    <article class="path path--primary">
                        <div class="path__meta">
                            <Blocks :size="16" />
                            <span>gpui-component</span>
                        </div>
                        <h3>{{ copy.shipTitle }}</h3>
                        <p>{{ copy.shipDescription }}</p>
                        <pre><code><span class="c-type">Button</span>::<span class="c-fn">new</span>(<span class="c-str">"save"</span>)
    .<span class="c-fn">primary</span>()
    .<span class="c-fn">label</span>(<span class="c-str">"Save changes"</span>)
    .<span class="c-fn">on_click</span>(cx.<span class="c-fn">listener</span>(Self::save))</code></pre>
                        <ul>
                            <li v-for="item in copy.shipPoints" :key="item">
                                <Check :size="13" />{{ item }}
                            </li>
                        </ul>
                        <a :href="gettingStartedHref" class="path__link"
                            >{{ copy.startComponent }} <ArrowRight :size="15"
                        /></a>
                    </article>

                    <article class="path">
                        <div class="path__meta">
                            <Layers3 :size="16" /><span>gpui-base</span>
                        </div>
                        <h3>{{ copy.ownTitle }}</h3>
                        <p>{{ copy.ownDescription }}</p>
                        <pre><code><span class="c-type">Button</span>::<span class="c-fn">new</span>(<span class="c-str">"save"</span>)
    .<span class="c-fn">on_click</span>(cx.<span class="c-fn">listener</span>(Self::save))
    .<span class="c-fn">rounded_md</span>()
    .<span class="c-fn">child</span>(<span class="c-str">"Save changes"</span>)</code></pre>
                        <ul>
                            <li v-for="item in copy.ownPoints" :key="item">
                                <Check :size="13" />{{ item }}
                            </li>
                        </ul>
                        <a :href="componentsHref" class="path__link"
                            >{{ copy.startBase }} <ArrowRight :size="15"
                        /></a>
                    </article>
                </div>
            </div>
        </section>

        <section class="band band--principle">
            <div class="principle__grid" aria-hidden="true"></div>
            <div class="band__inner principle">
                <div class="principle__quote">
                    <span class="section-kicker">{{
                        copy.principleKicker
                    }}</span>
                    <blockquote>
                        <span>{{ copy.principleLead }}</span>
                        <span class="principle__accent">{{
                            copy.principleTail
                        }}</span>
                    </blockquote>
                </div>
                <div class="principle__aside">
                    <p>{{ copy.principleDetail }}</p>
                    <a :href="componentsHref" class="btn btn--primary"
                        >{{ copy.baseAction }} <ArrowRight :size="16"
                    /></a>
                </div>
            </div>
        </section>
    </main>
</template>

<style>
/* Styles from the original index.vue — kept as global since this is the page root */
.home {
    --page: 1280px;
    --gutter: 1.5rem;
    --section-gap: clamp(3.5rem, 6vw, 5.5rem);
    color: var(--foreground);
}

.home > section {
    position: relative;
    border-top: 1px solid var(--border);
}

.home > section:first-child {
    border-top: 0;
}

.band__inner {
    position: relative;
    width: min(100% - 3rem, var(--page));
    margin-inline: auto;
    padding-block: var(--section-gap);
}

.band--principle {
    overflow: hidden;
    background: var(--sidebar);
}

.btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.45rem;
    min-height: 2.6rem;
    padding: 0 1.05rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-control);
    background: var(--background);
    color: var(--foreground) !important;
    font-size: 0.875rem;
    font-weight: 600;
    text-decoration: none !important;
    box-shadow: var(--shadow-raise);
    transition:
        background 150ms ease,
        border-color 150ms ease,
        transform 150ms ease;
}

.btn:hover {
    background: var(--secondary);
}
.btn:active {
    transform: translateY(1px);
}

.btn--primary {
    border-color: var(--brand);
    background: var(--brand);
    color: var(--brand-contrast) !important;
}

.btn--primary:hover {
    border-color: var(--brand-hover);
    background: var(--brand-hover);
}

.section-head {
    max-width: 44rem;
    margin-bottom: 2.75rem;
}
.section-head h2 {
    margin: 0.85rem 0 0;
    font-size: clamp(2rem, 3.6vw, 3rem);
    font-weight: 660;
    letter-spacing: -0.045em;
    line-height: 1.04;
}
.section-head p {
    max-width: 38rem;
    margin: 1rem 0 0;
    color: var(--muted-foreground);
    font-size: 1rem;
    line-height: 1.7;
}

.section-kicker {
    display: inline-flex;
    align-items: center;
    color: var(--muted-foreground);
    font: 600 0.68rem/1 var(--font-mono);
    letter-spacing: 0.14em;
    text-transform: uppercase;
}

html[lang^="zh"] .section-kicker {
    letter-spacing: 0.04em;
}

.hero {
    overflow: hidden;
}

.hero__grid {
    position: absolute;
    inset: 0;
    background-image:
        linear-gradient(to right, var(--grid-line) 1px, transparent 1px),
        linear-gradient(to bottom, var(--grid-line) 1px, transparent 1px);
    background-size: 64px 64px;
    mask-image: radial-gradient(115% 85% at 30% 0%, black 15%, transparent 74%);
    pointer-events: none;
}

.hero__inner {
    position: relative;
    display: grid;
    grid-template-columns: minmax(0, 0.93fr) minmax(0, 1.07fr);
    align-items: center;
    gap: clamp(2rem, 4vw, 3.5rem);
    /* Wider than the page's 1280px column, and with a narrower gutter: the
       junction needs the room and the copy caps its own measure anyway. */
    width: min(100% - 2rem, 1520px);
    margin-inline: auto;
    padding-block: clamp(2.75rem, 5vw, 4.25rem);
}

.eyebrow {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.38rem 0.7rem 0.38rem 0.55rem;
    border: 1px solid var(--border);
    border-radius: 999px;
    background: var(--background);
    color: var(--foreground) !important;
    font-size: 0.72rem;
    font-weight: 560;
    text-decoration: none !important;
    transition: border-color 150ms ease;
}

.eyebrow:hover {
    border-color: var(--brand-line);
}

.eyebrow__pulse {
    position: relative;
    width: 0.4rem;
    height: 0.4rem;
    border-radius: 50%;
    background: var(--success);
}

.eyebrow__pulse::after {
    position: absolute;
    inset: -0.15rem;
    border: 1px solid var(--success);
    border-radius: 50%;
    opacity: 0.5;
    content: "";
}

.hero h1 {
    max-width: 18ch;
    margin: 1.25rem 0 0;
    font-size: clamp(2.2rem, 4.3vw, 3.6rem);
    font-weight: 660;
    letter-spacing: -0.042em;
    line-height: 0.98;
}
.hero__lead {
    max-width: min(32rem, 100%);
    margin: 1.25rem 0;
    color: var(--muted-foreground);
    font-size: 1.03rem;
    line-height: 1.7;
}
.hero__actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.65rem;
    margin-top: 1.5rem;
}

.hero__install {
    display: inline-flex;
    align-items: center;
    gap: 0.6rem;
    max-width: 100%;
    margin-top: 1.25rem;
    padding: 0.4rem 0.4rem 0.4rem 0.6rem;
    border: 1px solid var(--border);
    border-radius: var(--radius-control);
    background: var(--sidebar);
}

.hero__install code {
    min-width: 0;
    overflow-x: auto;
    color: var(--foreground);
    font: 0.73rem/1.6 var(--font-mono);
    mask-image: linear-gradient(
        to right,
        black calc(100% - 1.25rem),
        transparent
    );
    scrollbar-width: none;
    white-space: nowrap;
}

.hero__install button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.7rem;
    height: 1.7rem;
    border-radius: 0.3rem;
    color: var(--muted-foreground);
    cursor: pointer;
    transition:
        background 140ms ease,
        color 140ms ease;
}

.hero__install button:hover {
    background: var(--secondary);
    color: var(--foreground);
}
.hero__install button[data-copied] {
    color: var(--foreground);
}

.hero__code {
    min-width: 0;
}

.hero__code pre {
    margin: 0;
    padding: 1.05rem 1.25rem 1.25rem;
    overflow-x: auto;
    background: var(--code-bg);
    font: 0.72rem/1.7 var(--font-mono);
    scrollbar-width: thin;
    tab-size: 4;
}

.hero__code code {
    color: var(--code-fg);
}

.hero__install-label {
    flex-shrink: 0;
    padding-right: 0.6rem;
    border-right: 1px solid var(--border);
    color: var(--muted-foreground);
    font: 0.66rem/1.6 var(--font-mono);
    letter-spacing: 0.04em;
}

.hero__signals {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 1.5rem;
    margin: 1.5rem 0 0;
    padding: 0;
    list-style: none;
    color: var(--muted-foreground);
    font-size: 0.8rem;
}

.hero__signals li {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
}
.hero__signals strong {
    color: var(--foreground);
    font-weight: 620;
    font-variant-numeric: tabular-nums;
}

.caps__grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1px;
    border: 1px solid var(--border);
    border-radius: var(--radius-surface);
    background: var(--border);
    overflow: hidden;
}

.cap {
    display: flex;
    flex-direction: column;
    padding: 1.75rem;
    background: var(--background);
    transition: background 160ms ease;
}

.cap:hover {
    background: var(--sidebar);
}

.cap__head {
    display: flex;
    align-items: center;
    gap: 0.6rem;
}
.cap__head h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 620;
    letter-spacing: -0.015em;
}

.cap p {
    margin: 0.7rem 0 auto;
    color: var(--muted-foreground);
    font-size: 0.875rem;
    line-height: 1.65;
}

.cap__api {
    display: flex;
    flex-wrap: wrap;
    gap: 0.3rem;
    margin: 1.25rem 0 0;
    padding: 0;
    list-style: none;
}

.cap__api li {
    padding: 0.18rem 0.42rem;
    border: 1px solid var(--border);
    border-radius: 0.3rem;
    background: var(--secondary);
    color: var(--muted-foreground);
    font: 0.7rem/1.5 var(--font-mono);
}

.cap__preview {
    position: relative;
    display: flex;
    align-items: stretch;
    gap: 0.32rem;
    height: 6rem;
    margin-top: 1.1rem;
    padding: 0.75rem;
    overflow: hidden;
    border: 1px solid var(--border);
    border-radius: var(--radius-card);
    background: var(--sidebar);
}

.cap__preview--table,
.cap__preview--list,
.cap__preview--editor {
    flex-direction: column;
    justify-content: center;
    gap: 0.42rem;
}

.cap__preview--table i,
.cap__preview--list i,
.cap__preview--editor i {
    display: flex;
    align-items: center;
    gap: 0.32rem;
}

.cap__preview--table b,
.cap__preview--list b,
.cap__preview--editor b {
    height: 0.3rem;
    border-radius: 999px;
    background: var(--input);
}

.cap__preview--table b:first-child {
    flex: 1.6;
}
.cap__preview--table b:nth-child(2) {
    flex: 1;
}
.cap__preview--table b:last-child {
    flex: 0.55;
}
.cap__preview--table i:first-child b {
    background: color-mix(in srgb, var(--foreground) 40%, transparent);
}
.cap__preview--table i:nth-child(3) b:first-child {
    background: var(--data-2);
}

.cap__preview--list {
    padding-right: 1.4rem;
}
.cap__preview--list i:nth-child(2) b {
    background: var(--data-2);
}

.cap__track {
    position: absolute;
    top: 0.75rem;
    right: 0.7rem;
    bottom: 0.75rem;
    width: 0.28rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--foreground) 8%, transparent);
}
.cap__thumb {
    display: block;
    width: 100%;
    height: 42%;
    border-radius: 999px;
    background: color-mix(in srgb, var(--foreground) 30%, transparent);
}

.cap__preview--editor {
    gap: 0.3rem;
}
.cap__preview--editor i:nth-child(1) b:nth-child(2) {
    background: var(--data-2);
}
.cap__preview--editor i:nth-child(3) b:last-child {
    background: color-mix(in srgb, var(--success) 60%, transparent);
}
.cap__preview--editor i:nth-child(5) b:first-child {
    background: color-mix(in srgb, var(--data-2) 55%, transparent);
}

.cap__gutter {
    flex-shrink: 0;
    width: 0.5rem;
    height: 0.28rem;
    border-radius: 999px;
    background: color-mix(in srgb, var(--foreground) 12%, transparent);
    text-decoration: none;
}

.cap__preview--perf {
    align-items: flex-end;
    gap: 0.1rem;
}
.cap__preview--perf u {
    flex: 1;
    min-width: 0;
    border-radius: 0.08rem 0.08rem 0 0;
    background: linear-gradient(
        to top,
        color-mix(in srgb, var(--data-2) 35%, transparent),
        var(--data-2)
    );
}

.cap__budget {
    position: absolute;
    top: 40%;
    right: 0.75rem;
    left: 0.75rem;
    border-top: 1px dashed
        color-mix(in srgb, var(--foreground) 28%, transparent);
}

.cap__preview--dock b {
    border: 1px solid var(--border);
    border-radius: 0.3rem;
    background: var(--background);
}
.cap__preview--dock b:first-child {
    flex: 0.55;
}
.cap__preview--dock b:nth-child(2) {
    flex: 1.3;
    border-color: color-mix(in srgb, var(--data-2) 45%, var(--border));
    background: color-mix(in srgb, var(--data-2) 10%, transparent);
}
.cap__preview--dock b:last-child {
    flex: 0.8;
}

.cap__preview--theme em {
    flex: 1;
    border: 1px solid var(--border);
    border-radius: 0.3rem;
}
.cap__preview--theme em:nth-child(1) {
    background: #0a0a0a;
}
.cap__preview--theme em:nth-child(2) {
    background: #404040;
}
.cap__preview--theme em:nth-child(3) {
    background: #a3a3a3;
}
.cap__preview--theme em:nth-child(4) {
    background: #f5f5f5;
}
.cap__preview--theme em:nth-child(5) {
    background: var(--background);
}
.cap__preview--theme em:nth-child(6) {
    background: var(--data-2);
}

.paths__grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 1.25rem;
}

.path {
    display: flex;
    flex-direction: column;
    padding: clamp(1.75rem, 3vw, 2.5rem);
    border: 1px solid var(--border);
    border-radius: var(--radius-surface);
    background: var(--background);
    transition:
        border-color 180ms ease,
        box-shadow 180ms ease;
}

/* These are three large cards side by side, so hovering one is read against
   the two beside it. The shadow carries the lift; the border only needs to
   acknowledge the pointer, hence `--border-hover` rather than the stronger
   `--brand-line` the smaller App Stories cards use. */
.path:hover {
    border-color: var(--border-hover);
    box-shadow: var(--shadow-panel);
}

.path h3 {
    margin: 1rem 0 0;
    font-size: 1.35rem;
    font-weight: 640;
    letter-spacing: -0.03em;
}
.path p {
    min-height: 3.05rem;
    margin: 0.65rem 0 0;
    color: var(--muted-foreground);
    font-size: 0.92rem;
    line-height: 1.65;
}

.path pre {
    margin: 1.5rem 0 0;
    padding: 1rem 1.1rem;
    overflow-x: auto;
    border: 1px solid var(--border);
    border-radius: var(--radius-card);
    background: var(--code-bg);
    color: var(--code-fg);
    font: 0.78rem/1.75 var(--font-mono);
}

.path ul {
    margin: 1.25rem 0 0;
    padding: 0;
    list-style: none;
}

.path li {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.3rem 0;
    color: var(--muted-foreground);
    font-size: 0.84rem;
}

.path__meta {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--muted-foreground);
    font: 0.72rem/1 var(--font-mono);
}

.path__link {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    margin-top: auto;
    padding-top: 1.5rem;
    color: var(--brand) !important;
    font-size: 0.84rem;
    font-weight: 620;
    text-decoration: none !important;
    transition: color 140ms ease;
}

.c-type {
    color: var(--code-type);
}
.c-fn {
    color: var(--code-fn);
}
.c-str {
    color: var(--code-string);
}
.c-kw {
    color: var(--code-keyword);
}
.c-mod {
    color: var(--code-fg);
}

.principle {
    display: grid;
    grid-template-columns: minmax(0, 1.05fr) minmax(0, 0.95fr);
    align-items: end;
    gap: clamp(2rem, 5vw, 4rem);
}

.principle blockquote {
    margin: 1.1rem 0 0;
    border: 0;
    padding: 0;
    font-size: clamp(1.5rem, 2.8vw, 2.25rem);
    font-weight: 640;
    letter-spacing: -0.04em;
    line-height: 1.14;
}

.principle blockquote span {
    display: block;
}
.principle p {
    margin: 0;
    color: var(--muted-foreground);
    line-height: 1.7;
}
.principle .btn {
    margin-top: 1.5rem;
}
.principle__aside {
    padding-bottom: 0.3rem;
}
.principle__accent {
    color: var(--brand);
}

.principle__grid {
    position: absolute;
    inset: 0;
    background-image:
        linear-gradient(to right, var(--grid-line) 1px, transparent 1px),
        linear-gradient(to bottom, var(--grid-line) 1px, transparent 1px);
    background-size: 48px 48px;
    mask-image: radial-gradient(90% 90% at 100% 50%, black, transparent 68%);
    pointer-events: none;
}

.principle > *:not(.principle__grid) {
    position: relative;
}

@media (max-width: 1080px) {
    .hero__inner {
        grid-template-columns: minmax(0, 1fr);
    }
    .hero__code {
        display: none;
    }
    .caps__grid {
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }
}

@media (max-width: 1180px) {
    .paths__grid {
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }
}

@media (max-width: 860px) {
    .paths__grid {
        grid-template-columns: minmax(0, 1fr);
    }
    .principle {
        grid-template-columns: 1fr;
        align-items: start;
    }
}

@media (max-width: 640px) {
    .hero__inner,
    .band__inner {
        width: calc(100% - 2rem);
    }
    .caps__grid {
        grid-template-columns: minmax(0, 1fr);
    }
}

@media (prefers-reduced-motion: no-preference) {
    .hero__inner > * {
        animation: rise 620ms cubic-bezier(0.16, 1, 0.3, 1) both;
    }
    .hero__inner > :nth-child(2) {
        animation-delay: 70ms;
    }
    .eyebrow__pulse::after {
        animation: ping 2.4s ease-out infinite;
    }
}

@keyframes rise {
    from {
        opacity: 0;
        transform: translateY(0.85rem);
    }
    to {
        opacity: 1;
        transform: none;
    }
}

@keyframes ping {
    0% {
        opacity: 0.55;
        transform: scale(0.8);
    }
    70%,
    100% {
        opacity: 0;
        transform: scale(1.7);
    }
}
</style>
