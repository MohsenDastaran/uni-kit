# Roadmap: Native UI Blocks — Website + Copy-Paste Components (GPUI + Slint)

Copy-paste components and blocks on the existing GPUI Kit site. People copy source into their own apps. `gpui-base` stays a crate dependency. Slint and later frameworks are hand-written sources plus a tab on the same pages.

The site is already Astro at [https://gpui-kit.com](https://gpui-kit.com) (`website/`, English). This roadmap adds the gallery to that site. It does not start a second site.

**Current phase = the first empty checkbox in Progress.** Tick a phase only when its **Done when** line is true.

**Progress**

- [x] 1. Restructure folder layout
- [x] 2. GPUI catalog you can copy from the site
- [x] 3. Design tokens (GPUI)
- [x] 4. Framework selector on the website
- [x] 5. Write Slint components (one per GPUI component)
- [x] 6. Slint usage, main controls first
- [x] 7. Add blocks (Login, Dashboard, Settings)
- [x] 8. Starter templates
- [x] 9. Deploy & launch

---

## Target structure

```text
website/                         # existing Astro site
├── component/                   # current GPUI Kit docs
├── blocks/                      # phase 7
├── templates/                   # phase 8 pages
crates/
├── base/                        # gpui-base (real dependency, never copied)
├── component/                   # GPUI copy sources
└── slint-component/             # hand-written Slint catalog + WASM gallery
templates/                       # phase 8 apps people copy
tokens/
├── tokens.json
└── schema.json
registry/                        # uni-kit CLI and components.json
```

`uni-kit` only copies files named in `registry/components.json`. It does not generate components. Adding a toolkit is a new entry in that file, not a new CLI release.

**Do not add:**

- A code-generation pipeline (components, blocks, and Slint files are hand-written)

---

## 1. Restructure folder layout

1. `website/`, `crates/component/`, `crates/base/`, `tokens/`, and `crates/slint-component/` exist.
2. Apache-2.0 attribution stays in `README.md` and `NOTICE` (GPUI Kit / Longbridge, GPUI / Zed).

**Done when:** those folders exist and the workspace still builds.

---

## 2. GPUI catalog you can copy from the site

The component pages are the GPUI copy-paste set: `website/component/`. A person copies a sample into an app that depends on `gpui-kit`. That crate already brings `gpui-component` and `gpui-base`, so the sample is usage, not a second copy of the component source.

`theme.md` is the theme guide and has no component sample. Every other catalog page has Rust you can copy. `CodeBlock` copies the block on the page.

**Done when:** those pages are on the site, and copying a block yields the Rust shown there.

---

## 3. Design tokens (GPUI)

Color, spacing, radius, and typography live in `tokens/tokens.json`, checked by `tokens/schema.json`. `tokens/README.md` maps those tokens onto the GPUI theme (`.theme-schema.json` and `crates/component/src/theme/default-theme.json`). `tokens/gpui/default.json` is the GPUI theme built from that file. The app still loads `default-theme.json` at runtime. Component-specific colors stay in the GPUI theme layer.

The Slint mapping is written in phase 5, with the Slint components.

**Done when:** `tokens/tokens.json`, `tokens/schema.json`, `tokens/gpui/default.json`, and the GPUI mapping note exist.

---

## 4. Framework selector on the website

Built on the existing Astro site. The GPUI Kit docs stay as they are (usage, API).

1. Every catalog component page has a GPUI / Slint selector below the live example (`website/src/lib/rehype-framework-code.js`). The choice is saved in `localStorage` (`selected-framework`) and applied before first paint, so a return visit never flashes the other framework.
2. Each code block on those pages is a pair of panels: `data-framework-panel="gpui"` holds the existing Rust, and `data-framework-panel="slint"` shows “No Slint version of this example yet.” until phase 5 fills it.
3. Switching reloads the page’s code in place (`website/src/lib/framework-switch.ts`): a progress bar runs under the nav, the blocks turn into a shimmering skeleton, then the chosen framework fades in. Reduced motion switches instantly. Arrow keys move between the two options, and the change is announced to screen readers.
4. Each code block has a copy button (`CodeBlock`).

**Done when:** the dev server shows every catalog component with a working selector. GPUI source is visible and copyable. Slint blocks are ready and hidden until Slint is selected.

---

## 5. Write Slint components

`crates/slint-component/` is a standalone crate (excluded from the GPUI Kit workspace) so Slint never enters the GPUI lockfile.

1. Every catalog slug has a hand-written `ui/<slug>.slint` and an interactive `ui/examples/<slug>.slint`. Copy those files into a Slint app; there is no installer.
2. `ui/theme.slint` maps `tokens/tokens.json` (see `tokens/README.md`). Components read `Theme` and nothing else.
3. `scripts/check.sh` type-checks each example with the gallery's Slint version. The gallery window (`ui/gallery.slint`) shows one example, chosen by page slug.
4. The website live example follows the selector (`ComponentExample.vue`): GPUI stays at `/gallery`, Slint loads `/slint-gallery?component=<slug>`. Another framework later adds a source in the same map. Both iframes stay mounted after first open, so switching back is instant.
5. `remark-slint-source.js` fills the Import and the first Usage Slint panel from those files. Phase 6 replaces that with one sample per section.

**Done when:** every catalog page shows Slint source under the selector, and selecting Slint shows the Slint WASM example.

---

## 6. Slint usage, main controls first

Alert Dialog is the pattern. Each Rust sample on its page has its own Slint sample in `crates/slint-component/ui/usage/alert-dialog/`. The import is by file name (`alert-dialog.slint`) from `ui/components`, which is where `uni-kit add` writes the files and what it adds to the Slint include path. Samples import only files that command installs.

Check and finish the main controls before the rest of the catalog:

1. Button, Input, Textarea, Checkbox, Radio, Switch, Select, Label, Icon, Dialog.
2. Every remaining catalog slug.

For each one:

1. Match the GPUI page: one `ui/usage/<slug>/en/*.slint` file per Rust sample, in the same order. When samples are shared, `ui/usage/<slug>/*.slint` is enough.
2. Imports are bare file names. Do not import gallery-only files such as GroupBox.

**Done when:** every catalog page’s Slint panels are separate samples like Alert Dialog, and each sample compiles after that component is installed.

---

## Source icons scroll to the sample

Each gallery card’s code icon (GPUI and Slint) scrolls the docs page to that example’s copyable sample. A card with no section gets a short heading and a fence.

The sample is the gallery example that is already running, not a separately written usage file. The page shows that source, so the block is code that works, and every gallery card has a sample in the usage section. When several cards share a type, the card keeps the type as its title and adds a subtitle so each one scrolls to its own sample.

**Done when:** every catalog component below is ticked. `theme` has no sample.

- [x] Accordion
- [x] Alert
- [x] Alert Dialog
- [x] Attachment
- [x] Avatar
- [x] Badge
- [x] Bubble
- [x] Button
- [x] Calendar
- [x] Card
- [x] Carousel
- [x] Chart
- [x] Checkbox
- [x] Clipboard
- [x] Collapsible
- [x] Color picker
- [x] Combobox
- [x] Command
- [x] Data table
- [x] Date picker
- [x] Description list
- [x] Dialog
- [x] Dock
- [x] Dropdown button
- [x] Editor
- [x] Empty
- [x] Focus trap
- [x] Form
- [x] Group box
- [x] Hover card
- [x] Icon
- [x] Image
- [x] Input
- [x] Input group
- [x] Kbd
- [x] Label
- [x] List
- [x] Marker
- [x] Menu
- [x] Message
- [x] Message scroller
- [x] Notification
- [x] Number input
- [x] OTP input
- [x] Pagination
- [x] Popover
- [x] Progress
- [x] Questionnaire
- [x] Radio
- [x] Rating
- [x] Resizable
- [x] Root
- [x] Scrollable
- [x] Select
- [x] Settings
- [x] Sheet
- [x] Shimmer
- [x] Sidebar
- [x] Skeleton
- [x] Slider
- [x] Spinner
- [x] Status bar
- [x] Stepper
- [x] Switch
- [x] Table
- [x] Tabs
- [x] Tag
- [x] Text view
- [x] Textarea
- [x] Time field
- [x] Title bar
- [x] Toggle
- [x] Toolbar
- [x] Tooltip
- [x] Tree
- [x] Virtual list

---

## 7. Add blocks (Login, Dashboard, Settings)

Build these in both frameworks:

- **Login** — email, password, submit, forgot-password
- **Dashboard** — sidebar, header, card grid, stat cards
- **Settings** — toggles, radio choices, text fields, save

1. Put the GPUI source with the copyable components, and the Slint source in `crates/slint-component/`.
2. Add a Blocks tab: `website/blocks/{name}.md`.
3. Each page uses the same framework selector and copy button.

**Done when:** all three blocks are on the site, with copy-paste source for GPUI and Slint.

**Where this stands.** The Blocks page exists (`website/src/pages/blocks.astro`), with live examples from both
galleries rather than markdown pages, and six blocks instead of the three above. Three of them — sidebar, dock,
settings — are installable for GPUI: the source is `crates/blocks/src/<name>.rs` and `npx uni-kit add gpui <name>`
copies it. That departs from phase 9's "there is no installer": a GPUI application already gets every control from
the `gpui-kit` crate, so the block file is the only thing a command can usefully write. Slint still installs the
controls a block composes, and its blocks are not installable as files.

Login and Dashboard are not built, and the GPUI previews are the gallery stories rather than the finished screen the
installed file draws. Both are open.

---

## 8. Starter templates

Templates are apps a person copies. They are not installed by a command.

1. Add three directories under `templates/`:

- **Blank** — window, `gpui_component::init`, `Root`, default theme
- **Login** — blank plus the Login block
- **Dashboard** — blank plus the Dashboard block

1. Add a Templates tab, in English, with the tree to copy and what the app contains.
2. When Slint blocks exist, add a Slint variant of each template the same way (a directory people copy, listed on the same pages).

**Done when:** each template builds after it is copied into a new directory, and the site links to it.

---

## 9. Deploy & launch

The repository is already public and Apache-2.0. The site is already at [https://gpui-kit.com](https://gpui-kit.com).

1. Publish the selector, the full component catalog, the three blocks, and the templates on that site.
2. Say on the Blocks and component pages that this gallery is source you copy. There is no installer.

**Done when:** those pages are live on [https://gpui-kit.com](https://gpui-kit.com).

---

## What not to do

- Generate components or blocks from a spec. `uni-kit` only copies files listed in `registry/components.json`.
- Use git submodules for framework ports.
- Add QuickGUI until its API is stable.
- Replace the GPUI Kit docs site, or open a second docs site.
- List egui, Iced, or other frameworks in the selector before their source exists.

---

## Risks

| Risk                              | What to do                                                                                                                                    |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| GPUI and Slint drift              | Finish each catalog component in both frameworks before starting the next. The three blocks wait until the components they use exist in both. |
| Slint’s API changes               | Use stable widgets. Skip new Slint APIs in this pass.                                                                                         |
| The website lags the source files | Update the page when the source file changes. A monthly pass is enough.                                                                       |
| Readers look for an installer     | The Blocks pages say to copy the source.                                                                                                      |

---

## iced

The first toolkit after Slint, in progress. The plan, the conventions and the
checklist are in [iced-integration.md](iced-integration.md), and the components
are written against **iced 0.14** in `crates/iced-component`. Coverage is
whatever `crates/iced-component/scripts/status.mjs` prints; it is generated from
the tree rather than written down here, so it cannot go stale.

An iced component is Rust source: it arrives in `src/components/` with the theme
it reads, and the install says `cargo add iced` instead of touching a build
script.

### The gallery

One module answers for every component — `?story=<registry key>` — and the demos
live in the library, at `src/demos/<story>.rs`, so the example binary and the
browser gallery run the same application rather than two copies that drift.

```bash
make dev:website-iced        # build the module, then serve the site
make build:wasm-iced         # the release module
make -C crates/iced-component build-page   # the page and the sources alone
```

The site serves it at `/iced-gallery`, in the release workflow as well. The page
is static — no bundler, and the only generated part is wasm-bindgen's glue — and
it is built **release**: a debug iced module is 90 MB against 7.8 MB. Rendering
needs WebGPU or WebGL2, and the page says so when a browser offers neither.

### A sample per section, like Slint

The Slint button page is ten outlined sections. The iced one is the same ten:
variants, outline, sizes, icons and states, button group, progress, dropdown,
custom size, custom colour, icon only — drawn from the same icon set the Slint
example names (`crates/slint-component/ui/icons`), with the two differences iced
cannot draw stated in the module: a `suffix-icon` (iced's button carries one icon,
before the label) and a squared `ButtonGroup` join (every button rounds its own
corners).

**Done when every component's demo has the sections its Slint example has.**
Button is the first; the others are one section each.

- [x] Button — the ten Slint sections

#### The process, to repeat for every component

1. **Read the Slint example**, `crates/slint-component/ui/examples/<name>.slint`.
   Every `GroupBox` is one section, in order. That list is the target.
2. **Port each section** into `src/demos/<name>.rs`, in the same order, and mark it:

   ```rust
   // section: Variants
   let variants = column![ … ];
   ```

   The markers are a contract, not comments — the website reads them, which is
   what keeps the page's code and the running code the same file. Close the last
   section with `// section: end`, or the lines that assemble the gallery (the
   status line among them) are shown as part of it.

3. **Give each section a heading.** `heading(app, index, title)` draws the title
   and the 28px code box, and posts through `crate::bridge`:

   ```json
   { "source": "gpui-kit", "index": 0, "title": "Variants" }
   ```

   the same payload and the same tag the Slint and GPUI galleries post. Indexes
   count from zero, in the order the sections appear. Both the title and the icon
   are the button, as in Slint's `SourceHeading`, and the icon is the same box:
   `primary` at 0.12 with a 0.35 border, 0.22/0.55 on hover, 0.34/0.7 pressed.

4. **Run it natively first** — `cargo build --examples`, then
   `cargo run --bin iced-gallery -- <name>` — before building any wasm.
5. **Build**: `make -C crates/iced-component build-page` for the page alone,
   `make build:wasm-iced` for the module, `make dev:website-iced` for both and the
   site.

#### Where button stands

**Done, both sides.** The gallery has its ten sections, ten headings, the bridge
and the markers; the page carries an iced panel per section, holding the same Rust
the gallery runs — `let variants = column![ … ]` under "Variants". `FRAMEWORKS` in
`rehype-framework-code` is `gpui`, `slint`, `iced`, and the grouping takes any
number of them: the first block of a run opens the control and a framework that
has not spoken yet joins it.

Four things cost time and are worth knowing before the next component:

- **A framework needs a language Shiki can name.** The iced samples are
  `lang: 'iced'`, aliased to `rust` in `src/lib/markdown.js` for the highlighter.
  A `data-*` property set on a code node does **not** survive the trip to HTML,
  which is why the first attempt put every iced sample in the GPUI panel; the
  language does.
- **`data-*` properties are camel-cased** on the way into hast, so
  `data-gallery-framework` is read as `dataGalleryFramework`.
- **A framework in `FRAMEWORKS` needs everything the switch asks for**, and
  `toggle-logos.js` had no iced mark, so `logo.viewBox` threw and 78 component
  pages came out as error pages. `logoElement` returns nothing for a framework
  without a mark now, and the switch skips it — an official iced mark can be added
  to `toggleLogos` whenever one is sourced.
- **A wasm build needs a font, and font fallback.** A browser gives a canvas no
  system fonts, so `fira-sans` is in the feature list — crate and install hint
  together, about 0.4 MB in the module. It is not enough on its own: that feature
  embeds Fira Sans **Regular**, and the components ask for `Weight::Medium` and
  `Weight::Semibold` in seventeen places, which then match no face and draw
  nothing at all — shapes and icons appeared, every word was missing. Enabling
  `advanced-shaping` is what fixes it: it turns on font fallback, so a weight with
  no face of its own resolves against the one that is embedded. Both features,
  together, are the difference between a gallery with labels and a gallery
  without.

### The plan: the major components first

Button is done end to end, so it is the template rather than the exception: what it
proves is the **route** — a section per example, the markers, the bridge, the page
panel cut from the same file, the tokens. Everything below walks that route again,
in an order chosen so each component can be built out of what is already finished.
`input` is first because half the catalog is a field of some kind.

An `[x]` here means the component satisfies the checklist at the end of this section,
the appearance check included; a component that exists but has not been looked at in
both themes is named plainly rather than ticked.

**The field family, and the one layout piece everything needs**

- [x] `input` — the base: value and placeholder, disabled, invalid, the size ladder,
      prefix and suffix slots. `textarea`, `number-input`, `otp-input`, `combobox`,
      `select` and `date-picker` are built on it, which is why it is first.
- [x] `label` — done. Its demo had no `// section:` markers at all, so no iced sample
  could be cut from it and no card could scroll anywhere: the affordance was added
  with it, and the card-to-sample path is verified in a real browser.
- [x] `scrollable` — the styled scroll area that `table`, `list` and dialog bodies
      need. Cheap, and unblocks three later components.
- [x] `textarea`, `number-input`, `otp-input` — the field, once, in three shapes.

**The overlay family**

- [x] `popover` — the positioning layer the rest of this family sits in, built on
      iced's own overlay mechanism (`Widget::overlay` → `overlay::Element`, the route
      `pick_list` and `combo_box` take) rather than a fake-positioned `stack!`. Click
      opens, outside-click and Escape close — verified by clip-region hash, with the
      three states byte-identical around the open one, and the dark panel surface read
      after a fix: its style had been resolved at view-build time from light tokens,
      so the dark panel was white with near-white content. `panel`/`panel_container`
      take `impl Fn(&Theme) -> PanelStyle` now, which is the shape the family inherits.
      Two traps are written into the comments: the runtime builds a widget's overlay
      twice per frame, so the panel must be borrowed and not taken, and its sub-tree
      must be reconciled in `overlay` or nothing paints.
- [ ] `tooltip` — a popover on hover. Committed and its page is right, but **no one
      has seen the label**: opening, closing, the arrow and the dark label surface are
      written, compile and unverified, because the author concluded this environment
      cannot deliver input to the wasm gallery. It can — four components were driven
      over CDP here (a click, five keystrokes into `input`, `123` into the OTP cells,
      a stepper moving 12→16 with a hover fill) — so the method is what needs fixing:
      hover, then **wait out the 400ms delay** before screenshotting, and sweep for the
      28px hit box. Until that is done, the tooltip is one screenshot short of done.
- [ ] `menu`, `dropdown_button` — a popover with items and keyboard traversal, one
      item shape between them (the shape `select` will compose). Two things are open
      and both are recorded rather than guessed: the **rows do not paint** inside the
      panel while the panel's own surface, height and padding do, and the author
      attributes that to the popover dropping nested children — but the popover demo's
      panel content does paint, so an A/B of the two in one session has to settle
      which it is. And the interaction is unverified for a reason that is itself
      wrong: both this agent and the tooltip's concluded that `Popover::update`
      compares content-space bounds to a window-space cursor. `scrollable` adds the
      scroll offset to the cursor before dispatching (`scrollable.rs:800`), which is
      what `pick_list` relies on, and the popover's author disproved the claim and
      corrected its own commit — the false reading came from a probe that stored the
      trigger rect on every press and from clicking 8px below a 28px button. Keyboard
      traversal is reported through `on_highlight` rather than owned (iced 0.14 has no
      focus trap), so the demo has to demonstrate it for the card to be honest.
- [ ] `select` — input plus popover plus menu.
- [ ] `combobox` — select plus free text.
- [ ] `date-picker` — popover plus the `calendar` that is already done.

**The form family**

None of this waits on `popover`, so it can run beside the overlay family rather than
after it. `form` is the exception and goes last, because it composes the controls
below instead of being one.

- `checkbox` — implemented, and `radio`, `switch` and `toggle` follow its shape. Not
  ticked yet: it has not been looked at in both themes, and like `label` it may have
  no `// section:` markers, which is the same debt and the same fix.
- [ ] `radio`, `switch`, `toggle` — one shape, three controls, so `handle` is built
      once and the three differ in what it draws
- [ ] `slider`, `stepper` — and `number-input`'s stepper behaviour, which is already
      built and should be shared rather than copied
- [ ] `form` — composition and validation over the controls above, not a new control;
      it has no rendering of its own, so what it owns is layout, labels, error
      placement and the state that ties a control to its message

**Surfaces and data**

- [ ] `dialog`, `sheet`, `alert-dialog` — overlays with focus trap and dismissal.
- [ ] `table`, `list`, `pagination` — built on `scrollable`.
- [ ] `notification`, `hover-card`, and the rest of the 79 as they come.

**A component is done when all of this is true**

- [ ] `src/components/<name>.rs`, tokens only — no colour written by hand
- [ ] `src/demos/<name>.rs`, a running example with one `// section:` per example
- [ ] `examples/<name>.rs`, a thin `main` that calls it
- [ ] A registry entry, so `add iced <name>` installs it
- [ ] `./scripts/check.sh` green: module lists, both cargo builds, theme parity,
      feature agreement, the manifest
- [ ] Every card's code icon scrolls to *its* sample, and the sample shown is the
      source of the example that is running
- [ ] Seen in light **and** dark, by looking at a screenshot, not by reading tokens
- [ ] One commit per component

### Themes are part of done

A component seen in one appearance is not finished. Three rules came out of the
button, and everything below inherits them:

- A **solid** control carries its colour in the fill and its label in the
  **foreground**. A hue used as ink on a fill of the same hue is unreadable in the
  dark theme, and in the light theme the same mistake hides inside a pale tint.
- An **outlined** control needs an edge that exists in both appearances. A fixed
  dark grey is an edge on white and nothing on near-black; a proportion of the
  foreground is an edge in both.
- **Hover** moves the fill *toward* the page: 90% of the fill over the background,
  which is shadcn's `bg-primary/90`. Moving it 90% of the way toward the page made
  the control vanish under the pointer instead.

The gallery runs under `theme::iced_theme`, built from the very tokens the
components read, so a window clears to the page's own background and a
default-styled widget agrees with the components beside it.

### Every card scrolls to its own sample

A card's code icon scrolls the page to that card's copyable sample, and the sample
**is the gallery example that is already running** — cut from the same file by the
`// section:` markers, not a usage file written beside it. There is one source of
truth per example: it runs in the gallery, and the page shows that source verbatim.
A sample cannot drift from the behaviour it documents, because there is nothing for
it to drift from. The gallery serves no source files at all now — `www/dist` holds
the module, the glue and the page — and the page carries the samples because the
build read them out of the demos.

Embedded, the gallery's code icon posts `{source: "gpui-kit", index, title}` to the
page hosting it, the same payload Slint and GPUI post. The site accepts that name
and scrolls to the heading that carries the title, which is the heading over the
sample in the usage section.

**Done when** every card has a sample to land on, and each one lands on its own.

- [x] The gallery posts the payload and the site scrolls to the titled heading
- [x] `rehype-framework-code` handles iced panels: `FRAMEWORKS` is `gpui`, `slint`
      and `iced`, and a sample is named by its language (`iced`, aliased to `rust`
      for the highlighter) or by `data-gallery-framework`
- [x] The page's sample is the demo's own source, cut by the section markers
- [ ] Every gallery card has a sample in the usage section, so no card's icon
      scrolls to nothing
- [ ] When several cards share a type — three buttons, two inputs — the card keeps
      the type as its title and adds a **subtitle**, so each card scrolls to its own
      sample instead of all of them landing on the first
- [ ] The scroll is checked by clicking the icon in a browser, not by reading the
      markup

## Later

egui, QuickGUI, or another toolkit can join after iced, one at a time, once its
API is stable: hand-write the full catalog and the 3 blocks, add a value to the
selector, and add the snippets on the English pages.
