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
- [x] `tooltip` — done, on iced's own hover state machine (`Idle`/`Hovered`/`Open`, a
      delayed redraw) with placement, flip and arrow from `popover::Placed`. Verified by
      measurement rather than by assertion: the 28px trigger was found by sweeping, the
      label is **unchanged at 150 ms and open at 1450 ms**, leaving returns the clip hash
      to the closed one byte for byte, and cancellation holds — a pointer resting 200 ms
      or 350 ms never opens it while 1200 ms does. A preferred-top panel draws above and
      a preferred-bottom one below, and in a 290px-high viewport that bottom one
      **flips above**. The arrow's stepped triangle was measured at all three positions
      (row widths 1,1,3,3,5,5,7 with centroids at start/centre/end). The dark label
      surface is `#f8f8f8` on `#080808` — **19.5:1**, not the white-on-white it could
      have been — and two code icons were clicked to confirm the page scrolls to their
      own samples. Two defects it found are fixed, both by measurement: the arrow's taper was
      **inverted** against `Arrow::draw`'s own doc, so an above-panel arrow pointed up
      into the surface — `abdfb567` now draws it widest where it meets the panel and
      narrowing to the trigger (`8,6,6,4,4,2,2` at Start, `7,7,5,5,3,3,1,1` at Centre
      and End, against `1,1,3,3,5,5,7` before), and renamed `Facing::tip` to
      `Facing::base` with a comment saying the sign is the taper, because that is the
      kind of sign that inverts silently. A demo comment claiming the first card was
      the flip test was wrong — it was measured drawing above at both viewports, and
      the flip is the Underline trigger in a 290px window — corrected in `9fa19e5f`.
      Left/right placements and the native window are unverified; no demo uses them.
      - [x] `menu`, `dropdown_button` — done, one item shape between them (the shape `select`
      composes). Two defects were found by looking rather than reading: the surface was
      drawn **twice** — `Menu::into_element` added one and every caller added another —
      so the menu now returns only its body and exactly one container owns the surface,
      a rule stated in five places; and `build_separator` was **never called**, so a
      separator painted a blank 32px row instead of Slint's 9px band and 1px rule (the
      panel was 167px, it is 145px). The shortcut glyphs were settled by parsing the
      embedded face's cmap rather than guessing: `⇧` and `⌫` are present, `⌘` and `⌥`
      are not, so `⌘N` was painting as `N`, and the demos spell `Cmd`/`Alt` with the rule
      on `Menu::shortcut`. Verified by measurement: every actionable row hovers to the
      accent in both themes (clip hashes per row), selected and highlighted are
      distinguishable, Escape and outside-click return the closed hash exactly, keyboard
      traversal still skips the separator, and the dropdown's label half runs its action
      while only the caret opens the menu. Two things are named and left: the dropdown's
      default width is 240px against Slint's 280px, so a long label wraps where Slint
      elides — iced 0.14 has no ellipsis — and `demos/kbd.rs` still writes `⌘` in the
      same face, which renders as a bare letter. The hover highlight also clears on a **direct jump**
      out of the panel now: `Panel::update` forwarded events only `if inside`, so a
      one-event exit never reached a row's `on_exit` and the accent stuck — a fast flick
      left it behind. It always forwards now, as iced's own overlay does with its list,
      and each widget guards its own `cursor.is_over`. Five commits, one per file, and
      the shared popover change was re-checked against the tooltip, whose closed hash
      returned byte-identical. `dropdown_button` still has no leave path of its own, so
      its caller cannot clear a hover through the new report.

#### Two legibility decisions left open

Both are faithful to the Slint originals and both are visible defects, which is why they
are recorded rather than quietly fixed:

- The **light theme's `danger`** on the page background measures **3.76:1**, under AA for
  12px text, and it colours every component's error line. It is parity-locked to the Slint
  theme, which `check.sh` enforces, so raising it moves both frameworks.
- **`message-role`'s failed state draws its initials in the fill's own colour.** `Role::ink`
  returns `destructive`, the same value as the failed fill, so the initials disappear in
  both themes — reproduced deliberately from `message.slint`, where the tint foreground is
  the destructive token. The one-line fix is `destructive_foreground`.
- **A dialog's auto close glyph is ghost ink on a primary-filled panel**, low contrast in the
  light theme. It mirrors Slint's own ghost button on the same fill, so it is inherited
  rather than introduced.

Three instances now, and they are one shape: an ink or a hue chosen for one surface, used
on another. The theme has no text-safe variant of these pairs, which is why each component
rediscovers it — and why the fix is a theme decision rather than three component patches.
The `check.sh` theme-parity step means any such change moves the Slint theme with it.

`message-role` was also the last component missing from a framework's sidebar: the sidebar
is built by scanning `website/component/*.md`, and it had no page — so a component that
exists, with a working demo, was invisible in every framework's list. Its page and its
section markers both landed in `0523e3be`, and with them the iced sidebar is complete at 45
components. One caveat belongs with it: `html[data-framework]` defaults to `slint`, so a
production first visit to a page whose framework list is iced-only redirects to
`/component` — pre-existing behaviour, and message-role is the only page where it bites.
      - [x] `select` — done, and it composes rather than re-derives: the trigger is
      `input::Style` with the size ladder, the panel is `Popover::filled(true)` around
      `Menu::into_element()` following menu.rs's one-surface rule, and a chosen row is
      the item marked `checked(true)` plus `Menu::selected`. Rows are the menu's own
      `Item`, so `select` inherits icons, shortcuts, disabled, danger and separators.
      The size table it needed (`ControlSize::input_height`) went into `theme.rs` as its
      **own commit first**, which is the shared-file rule applied without being asked.
      Drove it: open, hover to the accent, arrows moving the highlight, choosing row 2
      so the trigger reads `egui`, and Escape and an outside click returning the closed
      frame byte for byte — in both themes, with the dark panel measured at `#171717`
      against a `#0a0a0a` page. Two defects it found and left, both outside its files:
      a hovered menu row's highlight does not clear when the pointer leaves (inherited
      from `Menu` reporting on enter only), and `dropdown_button` is not counted by
      `status.mjs` because its registry key is `dropdown-button` while the Slint
      inventory key is `dropdown_button` — so a landed component still reads as
      outstanding.
- [x] `combobox` — done, and it reuses `menu::Item` rather than inventing a second item
      shape, filtering internally while every index it reports stays into the caller's
      own list. Composed from the field's style, the popover's single surface and the
      menu's body, with a search row, a rule and a caller footer. Verified by driving it
      and by installing it: typing `Sv` filters to SvelteKit, arrows step, Enter chooses
      and clears the query, Escape closes with the query kept, and reopening the panel is
      **byte-identical** to the escape-then-reopen frame; multi-select toggles one row
      without closing; the footer action appends a new value. With the pointer moved
      away, a filtered single row shows **no stray highlight** — the hover-leave fix
      arriving where it matters. Named rather than faked: no groups or badges, pinned
      rows, custom check glyph, trigger variants, `cleanable`, an empty element or
      elision, and the host's arrows act on an open panel because iced 0.14 reports no
      focus to the application.
- [x] `date-picker` — done, the last of this family: the field's frame as the trigger,
      the popover's single surface for the panel, and the calendar inside it. It needed
      one shared-file extension — `Calendar::bordered(false)` — committed **alone,
      first**, so the popover owns the panel edge and no second border appears. Verified
      by driving it rather than reading it: paging months, picking a day so the trigger
      reads `2026-11-10`, and Escape and an outside click each restoring the picked frame
      **byte-identical**; a blocked weekend leaving the frame **unchanged** where the
      Monday beside it selects; presets filling yesterday and a range; a two-click range
      completing; and a two-month panel **flipping above** the trigger near the window
      floor. Contrast measured at ~17.6:1 for the selected day in light and ~15.9:1 in
      dark. The install was run too — `add iced date-picker` writes calendar, date_picker,
      input, popover and theme, and an app using it compiles clean. Named, not faked: no
      year grid or in-component year clamp, no arbitrary disabled-date matcher (only
      weekends), no `cleanable`/`date_format`/`first_day_of_week`/12-hour clock, no
      elision, and time fields use the field size ladder rather than Slint's fixed
      height.

**The form family**

None of this waits on `popover`, so it can run beside the overlay family rather than
after it. `form` is the exception and goes last, because it composes the controls
below instead of being one.

- [x] `checkbox` — done, and it carried exactly the debt suspected: no `// section:`
      markers, so no iced sample could be cut and no card had anywhere to scroll. It
      has nine now, one per page heading, and only **Custom Tab Order** is sample-less
      because iced 0.14 has no tab-order or focus-trap API — named in the module docs
      rather than faked. The appearance check then found a real bug: iced's checkbox
      draws its checkmark whenever the box is filled and has no second glyph, so
      `indeterminate(true)` on an unchecked box drew a **tick** where Slint draws a
      minus. The icon slot takes `U+2212` now, from the face the mandatory `fira-sans`
      feature embeds — the generic sans-serif resolves to no face on wasm, which is the
      same trap that once made every label in the gallery invisible. `radio`, `switch`
      and `toggle` follow its shape.
- [x] `radio`, `switch`, `toggle` — done on one shared handle, as planned:
      `button::handle(content, on_press: Option<Message>, style)` is iced's own button
      with zero padding, a shrinking size and `on_press_maybe`, so disabled is simply
      the absence of a message — the same rule `checkbox` follows. The three differ
      only in what they draw and report: radio draws a disc and a dot and reports
      `true`; switch draws a track with a sliding thumb and reports the new value;
      toggle draws its own pressed fill and border. `RadioGroup` holds the index and
      reports it. Verified by driving real clicks, not by reading: radio exclusivity
      and each control toggling forward and back, in light **and** dark, by
      clip-region hash. Their shared-file extensions were committed separately
      (`ControlSize::control_height` in `theme.rs`, `handle` in `button.rs`) because a
      shared file left dirty across agents is how the popover's arrow was lost.
      Named, not faked: iced 0.14 has no subtree opacity (disabled is fill and ink),
      no layout transition (the thumb snaps), and no focus ring or Space/Enter on a
      bare button; `ToggleGroup` was not built.
- [x] `slider`, `stepper` — done, and the stepper behaviour was shared rather than
      copied: `stepper::step_index` is built on `number_input::step_value` and skips a
      disabled step in the direction of travel, which is Slint's `step-by`; the slider's
      arrow keys call the same function. Clamping is verified at **both** ends in both
      themes — the slider drags to 100% and 0%, the stepper clamps to "Step 1 of 4" with
      Previous disabled and to "Step 4 of 4" with a disabled Done — and the negative
      cases are the good ones: dragging a disabled slider leaves the clip hash
      **byte-identical**, and clicking a disabled step leaves its row unchanged. The
      section titles are the Slint example's GroupBox headings rather than only the
      page's `##` ones, which is what puts a panel under every example and matches the
      button precedent; the author flagged the choice, and it is the right one. Two
      things were left unverified: the scratch install for these two (the manifest
      closure carries `number_input`, `input`, `button` and `theme`, and it compiles,
      but it was not installed and built), and the stepper demo's arrow-key
      subscription, which was wired and native-built but not re-driven in wasm.
- [x] `form` — done, and it is what the plan said it would be: no control of its own,
      but the layout, the labels, the error placement and the state tying a control to
      its message. `LabelLayout::{Above, Beside}` covers Slint's FormField and FormRow,
      fields carry section and separator rows, `col_span`/`col_start` place a field in a
      grid, and the error is application state the field only places — while it is set it
      takes the message line in `danger` and the description steps aside. Verified by
      driving the whole cycle in both themes: submit produces the error lines, and
      correcting the field returns the message clip **byte-identical** to its baseline.
      The install was proven too — `add iced form` writes form, label and theme and the
      scratch crate compiles. Two things are named rather than faked: a column span is a
      `FillPortion` share, exact for single-column and full-width rows and proportional
      for mixed three-column ones, and a hidden field leaves the grid rather than
      holding an empty cell. One real finding is left for a decision rather than fixed:
      the light theme's `danger` on the page is **3.76:1**, under AA for 12px text, but it
      is parity-locked to the Slint theme and shared by every component's error line.

**Surfaces and data**

- [x] `dialog` — done, and it is deliberately **not** a `Widget::overlay`, which is what
      the brief suggested and what the code proves wrong: iced's `overlay::Group` updates
      every overlay in the group and does not stop on a neighbour's capture, so two open
      overlays would each see the other's press. A modal needs the topmost layer to be the
      only one answered, so it is a full-size in-flow layer for `Stack`, whose `update`
      runs last-first and stops on capture — nested dialogs route correctly. Backdrop is
      the `overlay` token, the panel is the popover's fill/edge/radius/shadow, and a press
      on the panel asks to stay open while a press on the backdrop closes when the caller
      allows it. Verified in proportion: one screenshot per theme with the dialog open
      (panel set off from the dimmed page, no white-on-white), and a driven open, backdrop
      close and Escape each returning the frame byte-identical to the closed baseline.
      Named, not faked: no focus trap, no Escape at the layer itself (the demo binds it, as
      date-picker's does), no scroll lock, no transition, and `content_padding` is a value
      rather than Slint's sub-420px breakpoint.
- [x] `sheet` — done, side-anchored and flush to its edge, covering a page the caller
      passes in. It reuses `popover`'s `PanelStyle` and re-exports its `Placement`, and
      names why the popover's `Placed` flip does not apply to it — a sheet is flush to its
      edge, so it never flips. Two defects were found by **looking at the screenshot**
      rather than by reading: `Placement::default()` is the popover's `Bottom`, so an
      unset sheet anchored to the wrong edge until the default became `Right`; and its
      panel draw handed the content the panel's own layout node, so only the first child
      painted — the same trap `popover` documents, re-learned by a component that wrote
      its own panel draw rather than reusing `popover`'s. That is a design gap worth
      settling: there are now two implementations of "draw a panel", and one of them
      repeated a bug the other had already paid for. Verified in proportion — a
      screenshot per theme with the sheet open, and one driven open with a scrim click
      returning the frame byte-identical.
- [x] `alert-dialog` — done, and it composes `dialog` rather than inventing a third
      mechanism, so it inherits the in-flow `Stack` layer and the dialog's stated gaps (no
      focus trap, no Escape at the layer, no scroll lock, no transition). What it adds is
      the confirmation surface: title and description pass-through, Cancel and confirm
      texts, a `destructive` flag that turns the confirm button into Danger, and a centred
      footer with Cancel first — with a press on Cancel, the backdrop or the close control
      all reporting `false` through the composed dialog, which matches Slint's
      `closed → cancelled`. A caller that passes no `on_open_change` gets no Cancel button
      and nothing captured, which is documented rather than left to be discovered.
      Verified in proportion: one screenshot per theme with the dialog open over the dimmed
      page, and one driven open-then-Cancel returning the frame byte-identical to the
      settled baseline in both themes — the first baseline per theme was discarded because
      the canvas settles a one-pixel edge on the first interaction, which is the kind of
      detail that makes a hash comparison mean something.
- [x] `table` — done, built on the finished `scrollable`, with one type per part the
      reference names (`Table`, `TableHead`, `TableRow`, `TableCell`, header/body/footer,
      caption). It found the **build-time token trap** the hard way — its first dark
      screenshot was a *white table*, because the surface was resolved from the light
      tokens when the element was built rather than inside the style closure. That is the
      second time a documented trap has recurred in a new component, the first being the
      panel layout node that `sheet` re-learned, so the lesson is worth more than the fix:
      a comment in another file does not prevent it, and the family now has two
      implementations of "draw a surface" that each rediscovered one. Verified in
      proportion — a screenshot per theme (header band distinct, badges legible, footer
      present) plus scrollbar crops, and one driven interaction: the striped toggle tints
      the rows and a second click returns the clip byte-identical. Named, not faked: no
      minimum or preferred column width, no elision or wrap, no rounded clip, no `Styled`
      trait (so Custom Styling has no iced panel and the module says why), the table is
      static with no sorting, selection or keyboard, and the scrollbar cannot widen on
      hover.
- [x] `list`, `pagination` — done, both on the finished `scrollable`, and both needing a
      small extension to a landed component that was committed **on its own first**: the
      button gained a trailing icon for pagination's "Next ›". The list's rows are buttons
      in a scroll area with hover and selected states, and pagination draws a five-wide
      window with ellipses, first/last and prev/next, the current page a primary button and
      the rest ghost — sitting in a horizontal scroll area so a wide run scrolls instead of
      clipping. Verified in proportion with pixel samples rather than assertions: clicking a
      row **moved** the selected fill from one row to another by sampled colour, and
      clicking page 3 moved the primary fill with it while the previous arrow became
      enabled. Dark showed no white surface anywhere, including the search field, which is
      the trap the table had just paid for. Named, not faked: no keyboard or accessibility
      layer, no virtual scrolling, no list sections, search or loading state — the search
      field is the reference's mock, not a working filter — and pagination's horizontal
      scrollbar never overflows in these demos, so it was never seen.
- [x] `notification`, `hover-card` — done, **and both reuse `popover`'s surface rather
      than writing a third one**, which is the first time since the popover that a
      component inheriting this family's traps did not re-learn one: the notification
      draws through `panel_container` and `PanelStyle::resolve` with every token resolved
      inside the style closure, and the hover card composes the tooltip's hover machine and
      the popover's `Placed` flip instead of writing its own. The one small extension it
      needed — tooltip padding, for the card's 12px inset — was committed alone first. A
      defect still turned up **by looking**: `panel_container` deliberately adds no
      padding, so the first build had the icon flush against the card edge. Verified in
      proportion and by hash: a notification raises and dismisses with the card clip
      returning byte-identical in both themes, a second anchor proving the layer's
      alignment mapping, and the hover card opening on a held hover past its delay and
      closing on leaving, byte-identical again. Named, not faked: no entry animation or
      auto-hide, no window-level host, no system delivery, no unique ids, four of six
      anchors undriven, and the hover card has fixed placement, one delay and no pointer
      events of its own.

**The family is complete.** Everything the plan named — the field family, the overlay
family, the form family and this one — is built, ticked with its gaps recorded, and
verified in proportion. What remains is the rest of the 79 as they come, and the one piece
of deferred work the recurrences earned: there are still two implementations of "draw a
surface", and consolidating them is what stops a fourth component re-learning what two
already paid for.

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

**The gates check the install, and the check has been shown to fail.** `check.sh --install`
installs **every** component the manifest lists — not a hand-picked five — each into its own
scratch crate so no sibling can be silently supplied, sharing the crate's target dir and
feature set so iced compiles once: **57 of 57 in about 57 seconds warm**. The list is read
from the manifest keys, so a newly registered component is covered without anyone
remembering to add it, and a failure now prints the compiler error instead of discarding it.
It was proven able to fail by feeding a doctored manifest through the CLI's
`UNI_KIT_REGISTRY` override — removing `input.rs` from `command`'s entry produced
`error[E0432]: unresolved import super::input` and 56 of 57, and restoring it produced 57 of
57. That matters more than the coverage number: the earlier version of this check was
vacuous for a while, and a gate nobody has seen fail is a gate nobody can trust.

**Commit with a pathspec — the index is shared.** Agents work in one checkout, so anything
one of them stages is visible to every other agent's plain `git commit`. One commit already
swept up a sibling's staged files this way (caught and re-cut with `reset --soft` and a
pathspec). Two rules follow: stage your own files and nobody else's, and always name the
paths when committing — `git commit -F - -- path/a path/b` — so a commit contains what its
message says it contains regardless of what is sitting in the index.

**Verify in proportion — do not over-engineer, do not over-test.** The list above says
what must be true, not how hard to look. In practice: one screenshot per theme, read with
your eyes; one driven interaction where the component's whole point is interaction; and
the component built as the honest minimum that matches the reference. Clip-hash matrices,
coordinate sweeps and before/after triples are tools for chasing a *suspected* defect.
They earned their place on the field and overlay families because the panel content, the
hover exit and the tooltip delay were each wrong in a way a screenshot could not show —
but they are not the default, and nothing should be engineered just to make it easier to
measure.

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

#### What the wasm face can and cannot draw

Four separate components lost a round to the same thing, so here is the answer. On wasm
the only embedded faces are Fira Sans Regular (`fira-sans`) and Iced Icons, so a
symbol the regular face lacks draws **nothing at all** rather than falling back to
anything. Parsing that face's cmap settles which marks are safe:

- **Present:** `⇧` `⌫` `↑` `↓` `←` `→` `⌦` `⇪` `⏎`
- **Absent:** `⌘` `⌥` `⌃` `↵` `⎋` `⇥` `⌤` `␣` `⎇`

A shortcut written with an absent mark reads wrong instead of not at all — `⌘N` paints
as a bare `N` — which is how the menu, kbd and tooltip demos each shipped a shortcut
that lied. The rule the demos now follow: keep the marks above, spell the rest
(`Cmd`, `Alt`), and check the cmap rather than assuming. This is the same failure that
once made every label in the gallery invisible: a glyph with no face resolves to
nothing, silently.

#### Driving the wasm gallery from a script

Verifying these components means driving a canvas, and three agents have lost a round
each to the same handful of traps. What works:

- **Coordinates are CSS pixels.** The canvas bitmap is CSS-sized and iced paints at
  `devicePixelRatio`, so a screenshot at DSF 2 is a scale-up of the same UI and input
  coordinates are unaffected — drive with `deviceScaleFactor: 1` and read at 2.
- **Sweep for the hit box.** These controls are as small as 28px, and a click 8px low
  produces a confident wrong conclusion rather than an error.
- **Wait out any delay, and hold the hover.** The tooltip opens after 400ms; a
  screenshot taken straight after a `mouseMoved` is byte-identical by construction,
  which is exactly how one agent decided the environment could not deliver input at all.
- **Compare clip-region hashes.** A before/after/dismiss triple that returns to the
  original hash is stronger evidence than an image someone has to squint at, and it is
  cheap.
- **Expect three caches.** The page stamp in `component-loader.ts`, Astro's
  `node_modules/.astro/data-store.json`, and a long-running dev server's own HTML. A
  change that "isn't showing" is usually one of them — restart with `--force`.
- **Do not `pkill -f` a pattern that matches your own command line.** It has killed
  this session's shell three times; bracket a character or kill by PID.
- **Click the canvas before sending keys.** Keyboard-driven state needs the headless
  canvas focused first; without that one click every keystroke is silently ignored, and
  the state looks like it never changes.
- **A screenshot preview is not the coordinate space.** Read a 1:1
  `Page.captureScreenshot` clip to derive coordinates; an estimate taken from a scaled
  preview was 50px low and looked like a dead control.
- **Make the clip cover where the text lands.** These fields left-align, so a clip
  starting mid-width misses typed text entirely and reports "the field never took a
  keystroke" — a false negative that reads exactly like a broken input.
- **Click, then type.** With focus emulation on, the first click after a navigation does
  not always register; the reliable sequence is to click the field and then send keys.
- **To enter a value: click, End, then keys.** Triple-click-and-type produced `08:05` where
  `18:45` was intended, and `Input.insertText` was ignored outright — the reliable sequence
  is a click on the segment, `End` to move the caret to the end, then the keystrokes.

#### A `Stack` sizes itself from its first child

iced's `Stack` defaults to `Shrink` and takes its size from the first child, so a layer
meant to cover a frame — which is how `dialog` and everything built on it are drawn —
ends up as a fragment of whatever sits behind it. The command palette's dialog sections
rendered as a tiny rectangle until both stacks were given `Width::Fill`/`Height::Fill`.
Nobody reads that from the type signature, and it cost a screenshot to notice, which is
the third time in this family that looking at a picture beat reading the code.

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
