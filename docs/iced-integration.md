# Adding iced to uni-kit

The plan and the status of the iced integration. The component list below is
generated from the repository by
[`crates/iced-component/scripts/status.mjs`](../crates/iced-component/scripts/status.mjs):

```bash
node crates/iced-component/scripts/status.mjs --write   # update the table
node crates/iced-component/scripts/status.mjs --check   # fail while anything is missing
```

## What is being added

iced as a third framework beside GPUI and Slint, to the same standard as Slint:
every component in the catalog, as Rust source a project copies in with
`npx uni-kit add iced <component>`, with a usage example on its component page.

The **inventory is the Slint registry**. Slint is the framework that lists every
component as copyable source, so its keys are the authoritative set — 79
components. GPUI contributes three composed screens of its own
(`sidebar`, `dock`, `settings`), which are components in the Slint set too, so the
set does not grow. egui and QuickGUI are placeholders with no components and are
not in scope.

Everything is measured against iced **0.14.0** (released 2025-12-07), the current
stable release. The version is pinned in `crates/iced-component/Cargo.toml`.

## Where iced has to be integrated

Following the Slint integration, which is the reference for all of it:

| Where | What changes |
| --- | --- |
| `crates/iced-component/` | The crate: `src/components/` holds the components as source — the same directory the CLI installs them into — `examples/` holds one usage example each, and `src/main.rs` is the gallery the website embeds. |
| `registry/build-manifest.mjs` | Add the `iced` framework to the generated manifest, with each entry naming real files (the builder refuses an entry whose source does not exist). |
| `registry/components.json` | Generated. Gains `iced`, with `default_target_dir: src/components` and `mod_file: src/components/mod.rs`, like GPUI. |
| `website/src/lib/registry-slugs.json` | Generated alongside it; the site reads the framework's component list from here. |
| `registry/index.js` | Help text and the framework-specific hint after installing (Slint's build-script include path; iced's dependency line). |
| `website/src/lib/framework-switch.ts` | `FRAMEWORKS` gains `iced`. The default stays Slint. |
| `website/src/components/FrameworkSelect.astro` | A third option in the selector, and the styles that go with it. |
| `website/src/components/ComponentExample.vue` | The frame for the iced view. |
| `website/src/lib/example-counts.ts` | Example counts per framework, from the iced examples. |
| `website/src/lib/remark-iced-source.js` | Injects each page's iced usage example, as `remark-slint-source.js` does for Slint. |
| `website/src/lib/sidebar.ts` | `hasSlintPage` becomes a question about any framework, so a page with no iced view hides in the iced sidebar. |
| `website/src/pages/component/[...slug].astro` | The frameworks a page has, and its content data. |
| `docs/` and `README.md` | The framework table, the roadmap, and this file. |

Not in scope unless asked: composed iced *blocks* for the Blocks page (the six
screens the Slint view shows). The Blocks page has no iced view until a screen
exists to put in it.

### The gallery, and why the examples are laid out for it

The component pages show a live example, and for GPUI that is one wasm gallery
serving every component rather than a build per page. iced follows that: one
gallery, `src/main.rs`, which the website embeds and which shows the component
named on the command line.

That needs each component's demo to be reachable from the library, and a usage
example is written as a standalone application, so the two are separated at the
end of the work rather than during it:

- `src/examples/<name>.rs` holds the demo's state, message, update and view, as a
  `pub` app the gallery can call;
- `examples/<name>.rs` keeps its `main` and its `fn main` commentary, and does
  nothing but run the one in the library.

Cargo still compiles every example, so the gate does not change, and the gallery
gains every component without a second implementation of it. This is deliberately
*after* the component batches: doing it now would rewrite files five batches are
writing.

Until the gallery exists, a component page shows the iced usage example as source
and the selector does not offer a live iced view for it.

## Conventions

Mirroring Slint, with the adaptations Rust and iced require:

- One component per file in `crates/iced-component/src/components/`. The registry
  key is the site's slug and keeps its hyphens (`alert-dialog`); the file and the
  module are that key with hyphens as underscores (`alert_dialog.rs`,
  `pub mod alert_dialog;`), because Rust does not allow the hyphen. The manifest
  builder does the mapping, so a hyphenated file fails the build rather than
  being missed, and each component's module docs name the key it installs as.
- Every component file is self-contained apart from `theme.rs`, which is shipped
  beside it, exactly as every Slint component ships `theme.slint`. A component
  declares no dependency the caller does not already have.
- The installed target is `src/components/`, and the CLI writes
  `src/components/mod.rs`. A component declares its own submodules, so installing
  two components shares one `theme.rs` rather than duplicating it.
- Icons are the caller's: a component that draws one takes an
  `Option<Element<Message>>` rather than shipping SVG bytes, because the icon set
  is the application's asset, not the component's. Slint ships its icons because
  its components name them in a `ui/icons/` folder the build script resolves.
- Every component has a usage example in `examples/<name>.rs` that the component
  page shows. Cargo compiles them all: `cargo build --examples` is the gate.

## The dependencies the components assume

iced's `svg`, `canvas`, `image-without-codecs`, `qr_code`, `markdown`, `advanced`
and `webgl` features are **not** in its defaults, so `cargo add iced` on its own
produces a project the components do not compile in. The install therefore names
the line instead, and `scripts/check.sh` compares that list — hyphens included,
so `image-without-codecs` is not quietly dropped from both sides — with
`crates/iced-component/Cargo.toml`.

Each of them is there for a reason, and two were learned the hard way:

- **`image-without-codecs`, not `image`.** `image` enables the image crate's
  default codecs, AVIF and `rav1e` among them, and those are what fail a `wasm32`
  build. Nothing in this crate decodes an image file: components display handles
  the application hands them — raw RGBA via `Handle::from_rgba`, or an SVG — so
  the codec-free widget is enough. An application that loads image files adds
  `iced/image` itself.
- **`webgl`.** wgpu on the web asks for WebGPU unless this feature is on, and
  Safari, Firefox and any software-rendered browser have no WebGPU. Without it the
  gallery is an empty frame for those readers.
- **`advanced`** opens the core traits `text_view` implements.

**One component needs a second crate.** `text_view` implements `Widget` itself,
and iced exposes that trait only through `iced_core`, so installing it needs
`iced_core = "0.14"` as well. Which components those are is not a list kept by
hand: `registry/build-manifest.mjs` reads it from the sources into
`needs_extra`, together with the version from this crate's own manifest, and
fails if a source uses a crate the manifest does not declare. The CLI prints the
line for the components that need it and nothing for the ones that do not.

**The wasm-bindgen CLI has to match the module, and there are two of them.** The
iced crate keeps its own lockfile, and its `wasm-bindgen` (0.2.129) is a different
version from the workspace's (0.2.121), which the GPUI and Slint galleries use. A
CLI and a module that disagree are refused at the bindings step, and two versions
cannot share one binary.

Installing the iced version over the shared one is what broke the Slint gallery
once — caught by building one of its pages, not by reading anything. So the iced
CLI lives in a prefix of its own (`make install-iced-bindgen` writes
`~/.wasm-bindgen-iced`), `scripts/build-wasm.sh` prefers it, then `$WASM_BINDGEN`,
then whatever is on `PATH`, and the release workflow does the same with a `--root`.
`build-wasm.sh` prints the version it wants when the bindings are refused, because
the fix is a `cargo install` rather than anything in the repository.

## The gallery

The website's component pages embed the galleries; for iced that is
`/iced-gallery?story=<registry key>`, one module answering for every component the
way the GPUI gallery does, rather than a build per page the way Slint does.

```
crates/iced-component/
  src/demos/<module>.rs   the demo: the whole application, `pub fn run(theme)`
  src/demos/mod.rs        generated: the module list, STORIES, the dispatcher
  src/demos/index.rs      what an unknown story shows: the list of stories
  src/wasm.rs             the browser entry, `#[wasm_bindgen] pub fn run(story, dark)`
  src/main.rs             the native binary, same dispatcher, name from argv
  www/index.html          the page: reads `?story=`, imports the glue, calls run
  www/dist/               the build output the site serves
  scripts/build-wasm.sh   cargo (wasm32, release) -> wasm-bindgen -> www/dist
```

The demos live in the library so that the example binary and the browser gallery
run the same code; two copies would drift. `scripts/mod-rs.mjs` generates the
module list and the dispatcher from the directory, so a demo that exists is a demo
`?story=` can name.

`make dev:website-iced` builds the module and starts the site; the dev middleware
serves `www/dist` at `/iced-gallery`, and the release workflow copies it into the
built site. The page resolves the glue from the directory it is mounted at rather
than from `./`, because `/iced-gallery` is a path with no trailing slash and a
relative import would resolve against the site root — which is exactly how the
first version of this page failed, in the browser, while the module it was
importing was being served correctly.

The build runs inside `script/guard.sh`, the cgroup cap Slint's gallery uses,
moved out of that crate so there is one copy rather than two. It builds
**release**: a debug iced module is 90 MB against 7.8 MB, and a preview nobody
will wait for is not a preview. The 7.8 MB includes wgpu's WebGL backend, which
cost about 2 MB over the WebGPU-only build and is what makes the gallery work in a
browser without WebGPU. `wasm-opt` would shrink it further and is optional:
`script/optimize-wasm.sh` says so and leaves the module alone when the optimizer is
missing, which is the case on this machine.

**What it cannot do.** Rendering needs WebGPU or WebGL2 from the browser; a
browser with neither shows the page's message rather than a blank frame. A theme
change on the host page reloads the frame, because an iced application takes its
theme when it starts.

## How a batch of components is finished

The components are written in batches, and a batch is only done when all of this
holds — the compiler decides most of it, but not all of it:

```bash
node crates/iced-component/scripts/mod-rs.mjs   # the module list follows the directory
cargo build                                     # every component compiles
cargo build --examples                          # every usage example compiles
crates/iced-component/scripts/check.sh          # theme parity, feature agreement, manifest, checklist
node crates/iced-component/scripts/status.mjs --check
```

Then, by hand, because no script sees it:

- no `todo!()`, `unimplemented!()`, placeholder or stub in any component;
- each file draws what its Slint counterpart draws — the states, sizes and
  variants, not a simplified stand-in;
- anything that could not be reproduced says so in the component's module docs;
- the checklist count went up by exactly the number of components in the batch.

## Continuing this work

The integration is finished; what remains is the components themselves. This is
the shape that has actually produced them, after several that did not.

**One agent per component, writing both files, in this order.** An agent that
reads everything first stalls: the API reference is 1,356 lines and every Slint
source is a few hundred more. The order that lands components is

1. read `crates/slint-component/ui/<key>.slint` **only** — it is the authority for
   the design, and it is short;
2. write `src/components/<module>.rs` from it, rough and not yet compiling;
3. iterate on the compiler, grepping `API-0.14.md` for the one symbol each error
   names rather than reading it;
4. write `examples/<module>.rs`, and iterate the same way.

**The gate for one component:**

```bash
cd crates/iced-component
node scripts/mod-rs.mjs && scripts/check-one.sh <module>
```

`mod.rs` is generated from the directory — never edit it by hand. `check-one.sh`
builds your component in a scratch crate, so another agent's half-written file
cannot affect the answer. It must print `clean`: no errors and no warnings, in
the component and in its example.

**Then the crate-wide gate**, which is what decides whether the work is real:

```bash
cargo build && cargo build --examples
crates/iced-component/scripts/check.sh
node crates/iced-component/scripts/status.mjs --check
```

**Leave the agent alone while it works.** The two components that landed on the
first try (`avatar`, `icon`) are the two agents that were never sent a follow-up
message; the ones that were nudged, corrected or interrupted either stalled or had
to be stopped. That is two data points and not a law — but messaging an agent that
is mid-flight has cost more than it has bought here.

**Do not leave a stub.** A five-line file that says it is pending compiles and is
declared, so it satisfies every check that asks whether a file exists; the
checklist now refuses to count one, and names it instead. An absent component is
an honest gap; a stub is a lie the gates cannot see.

**What is left** is in the generated checklist below, and `status.mjs` prints the
same list. At the time of writing it is 21 of 79 complete, with 58 to go.

## Verification log

Kept because the final report has to say what was actually run, and because a
claim that is not written down next to its output tends to grow.

**The chain from manifest to a compiling application.** A clean project with iced
and the features, then:

```bash
UNI_KIT_ROOT=<repo> node registry/index.js add iced button badge card alert
#   src/components/button.rs, badge.rs, card.rs, alert.rs, theme.rs
#   src/components/mod.rs += pub mod button; theme; badge; card; alert;
#   theme.rs written once, not four times

CARGO_TARGET_DIR=<crate>/target cargo build
#   Finished `dev` profile [unoptimized + debuginfo] target(s) in 27.91s
```

No errors and no warnings, with `src/main.rs` written against the components' real
public APIs. This is the strongest check available of the registry, the CLI, the
shared theme and the component source together, because it is exactly what a
reader does.

**Fidelity, coarsely.** The compiler says a port builds; it does not say the port
is complete. As a first pass, every ported component was compared against the
Slint source it came from, by size:

```
component                iced  slint  ratio
theme                     406    326   1.25
description-list          441    336   1.31
accordion                 366    219   1.67
...
skeleton                  152     15  10.13
22 components ported; median ratio 3.08
```

No component is thinner than its source, and the median is three times it, which
is what iced's builder style costs against Slint's declarative syntax. A ratio
below 0.45 on a source over 40 lines would have been worth reading closely; none
is. This is a smoke test, not a substitute for reading a port beside its Slint
original — but it is cheap, it runs over every component at once, and it would
have caught the failure this task cares about, a component that compiles while
drawing almost nothing.

**The whole browser chain, as far as this machine allows.** `make
dev:website-iced` builds the module and serves the site; then, through the dev
server:

```
/iced-gallery?story=button              200  text/html            4,594 B
/iced-gallery/iced_component.js         200  text/javascript    149,994 B
/iced-gallery/iced_component_bg.wasm    200  application/wasm  8,151,057 B  (7.77 MB)
```

The module loads and instantiates in Chrome, and the page's failure path renders
its message instead of a blank rectangle. The component page's iframe asks for
`/iced-gallery?story=button`, and availability is gated on a real component *and*
a real example: `chart` has neither and `tabs` has no example, and neither page
offers iced — which is the check that keeps the site from claiming a component
that is not there.

**What could not be verified here: painted pixels.** Headless Chrome on this
machine reports `webgl2=true webgl1=true webgpu=true`, but wgpu finds no usable
adapter: `No available adapters`, then a trap inside the module. Chrome's flags
change which failure appears — `--use-webgpu-adapter=swiftshader` gets an adapter
and then fails at the device — never to success. So "the gallery renders a
component in a browser" is unproven on this machine, and it is recorded as
unproven rather than assumed from the HTTP responses being correct.

**Installing the components an application actually builds.** A clean project
with only what the CLI names, then `cargo build`:

```
iced = { version = "0.14", features = ["svg", "canvas", "image-without-codecs",
          "qr_code", "markdown", "advanced", "webgl"] }
iced_core = "0.14"                        # text-view only, as the CLI says
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 22.53s
```

**The crate's own gates**, run in `crates/iced-component`:

```bash
node scripts/mod-rs.mjs        # mod.rs declares every component in the directory
cargo build                    # every component compiles
cargo build --examples         # every usage example compiles
scripts/check-one.sh <name>    # one component, isolated from the others
scripts/check.sh               # theme parity, feature agreement, manifest, checklist
```

## Status

<!-- iced-checklist:start -->
| Component | iced file | GPUI ref | Slint ref | iced source | lines | example | registry |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `accordion` | `accordion.rs` | yes | yes | yes | 393 | yes | yes |
| `alert` | `alert.rs` | yes | yes | yes | 319 | yes | yes |
| `alert-dialog` | `alert_dialog.rs` | yes | yes | yes | 242 | yes | yes |
| `attachment` | `attachment.rs` | yes | yes | yes | 1169 | yes | yes |
| `avatar` | `avatar.rs` | yes | yes | yes | 320 | yes | yes |
| `badge` | `badge.rs` | yes | yes | yes | 237 | yes | yes |
| `bubble` | `bubble.rs` | yes | yes | yes | 712 | yes | yes |
| `button` | `button.rs` | yes | yes | yes | 586 | yes | yes |
| `calendar` | `calendar.rs` | yes | yes | yes | 507 | yes | yes |
| `card` | `card.rs` | yes | yes | yes | 198 | yes | yes |
| `carousel` | `carousel.rs` | yes | yes | yes | 529 | yes | yes |
| `chart` | `chart.rs` | yes | yes | yes | 1258 | yes | yes |
| `checkbox` | `checkbox.rs` | yes | yes | yes | 269 | yes | yes |
| `clipboard` | `clipboard.rs` | yes | yes | yes | 127 | yes | yes |
| `collapsible` | `collapsible.rs` | yes | yes | yes | 567 | yes | yes |
| `color-picker` | `color_picker.rs` | yes | yes | yes | 936 | yes | yes |
| `combobox` | `combobox.rs` | yes | yes | yes | 738 | yes | yes |
| `command` | `command.rs` | yes | yes | yes | 990 | yes | yes |
| `data-table` | `data_table.rs` | yes | yes | yes | 804 | yes | yes |
| `date-picker` | `date_picker.rs` | yes | yes | yes | 539 | yes | yes |
| `description-list` | `description_list.rs` | yes | yes | yes | 441 | yes | yes |
| `dialog` | `dialog.rs` | yes | yes | yes | 620 | yes | yes |
| `dock` | `dock.rs` | yes | yes | yes | 1520 | yes | yes |
| `dropdown_button` | `dropdown_button.rs` | yes | yes | yes | 357 | yes | — |
| `editor` | `editor.rs` | yes | yes | yes | 514 | yes | yes |
| `empty` | `empty.rs` | yes | yes | yes | 330 | yes | yes |
| `focus-trap` | `focus_trap.rs` | yes | yes | yes | 217 | yes | yes |
| `form` | `form.rs` | yes | yes | yes | 593 | yes | yes |
| `group-box` | `group_box.rs` | yes | yes | yes | 227 | yes | yes |
| `hover-card` | `hover_card.rs` | yes | yes | yes | 177 | yes | yes |
| `icon` | `icon.rs` | yes | yes | yes | 214 | yes | yes |
| `image` | `image.rs` | yes | yes | yes | 196 | yes | yes |
| `input` | `input.rs` | yes | yes | yes | 373 | yes | yes |
| `input-group` | `input_group.rs` | yes | yes | yes | 294 | yes | yes |
| `kbd` | `kbd.rs` | yes | yes | yes | 141 | yes | yes |
| `label` | `label.rs` | yes | yes | yes | 122 | yes | yes |
| `list` | `list.rs` | yes | yes | yes | 241 | yes | yes |
| `marker` | `marker.rs` | yes | yes | yes | 123 | yes | yes |
| `menu` | `menu.rs` | yes | yes | yes | 670 | yes | yes |
| `message` | `message.rs` | yes | yes | yes | 490 | yes | yes |
| `message-role` | `message_role.rs` | yes | yes | yes | 235 | yes | yes |
| `message-scroller` | `message_scroller.rs` | yes | yes | yes | 676 | yes | yes |
| `notification` | `notification.rs` | yes | yes | yes | 418 | yes | yes |
| `number-input` | `number_input.rs` | yes | yes | yes | 524 | yes | yes |
| `otp-input` | `otp_input.rs` | yes | yes | yes | 522 | yes | yes |
| `pagination` | `pagination.rs` | yes | yes | yes | 276 | yes | yes |
| `popover` | `popover.rs` | yes | yes | yes | 1117 | yes | yes |
| `progress` | `progress.rs` | yes | yes | yes | 413 | yes | yes |
| `questionnaire` | `questionnaire.rs` | yes | yes | yes | 538 | yes | yes |
| `radio` | `radio.rs` | yes | yes | yes | 387 | yes | yes |
| `rating` | `rating.rs` | yes | yes | yes | 197 | yes | yes |
| `resizable` | `resizable.rs` | yes | yes | yes | 459 | yes | yes |
| `root` | `root.rs` | yes | yes | — | — | — | — |
| `scrollable` | `scrollable.rs` | yes | yes | yes | 468 | yes | yes |
| `select` | `select.rs` | yes | yes | yes | 507 | yes | yes |
| `settings` | `settings.rs` | yes | yes | yes | 211 | yes | yes |
| `sheet` | `sheet.rs` | yes | yes | yes | 726 | yes | yes |
| `shimmer` | `shimmer.rs` | yes | yes | yes | 249 | yes | yes |
| `sidebar` | `sidebar.rs` | yes | yes | yes | 675 | yes | yes |
| `skeleton` | `skeleton.rs` | yes | yes | yes | 152 | yes | yes |
| `slider` | `slider.rs` | yes | yes | yes | 492 | yes | yes |
| `speech` | `speech.rs` | yes | yes | yes | 562 | yes | yes |
| `spinner` | `spinner.rs` | yes | yes | yes | 209 | yes | yes |
| `status-bar` | `status_bar.rs` | yes | yes | yes | 385 | yes | yes |
| `stepper` | `stepper.rs` | yes | yes | yes | 642 | yes | yes |
| `switch` | `switch.rs` | yes | yes | yes | 235 | yes | yes |
| `table` | `table.rs` | yes | yes | yes | 753 | yes | yes |
| `tabs` | `tabs.rs` | yes | yes | yes | 323 | yes | yes |
| `tag` | `tag.rs` | yes | yes | yes | 257 | yes | yes |
| `text-view` | `text_view.rs` | yes | yes | yes | 919 | yes | yes |
| `textarea` | `textarea.rs` | yes | yes | yes | 425 | yes | yes |
| `theme` | `theme.rs` | yes | yes | yes | 455 | — | — |
| `time-field` | `time_field.rs` | yes | yes | yes | 608 | yes | yes |
| `title-bar` | `title_bar.rs` | yes | yes | — | — | — | — |
| `toggle` | `toggle.rs` | yes | yes | yes | 268 | yes | yes |
| `toolbar` | `toolbar.rs` | yes | yes | yes | 328 | yes | yes |
| `tooltip` | `tooltip.rs` | yes | yes | yes | 718 | yes | yes |
| `tree` | `tree.rs` | yes | yes | yes | 430 | yes | yes |
| `virtual-list` | `virtual_list.rs` | yes | yes | yes | 265 | yes | yes |

**75 / 79** components have an iced source, an example and a registry entry.

Still to do: `dropdown_button`, `root`, `theme`, `title-bar`.
<!-- iced-checklist:end -->
