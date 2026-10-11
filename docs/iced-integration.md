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
screens the Slint view shows), and a wasm gallery for the browser. The components
themselves are the deliverable; the Blocks page has no iced view until a screen
exists to put in it.

## Conventions

Mirroring Slint, with the adaptations Rust and iced require:

- One component per file in `crates/iced-component/src/components/`, named after
  the registry key (`alert-dialog.rs` for `alert-dialog`), so the file name, the
  registry key and the module name agree.
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

## Status

<!-- iced-checklist:start -->
| Component | GPUI ref | Slint ref | iced source | lines | example | registry |
| --- | --- | --- | --- | --- | --- | --- |
| `accordion` | yes | yes | — | — | — | — |
| `alert` | yes | yes | — | — | — | — |
| `alert-dialog` | yes | yes | — | — | — | — |
| `attachment` | yes | yes | — | — | — | — |
| `avatar` | yes | yes | — | — | — | — |
| `badge` | yes | yes | — | — | — | — |
| `bubble` | yes | yes | — | — | — | — |
| `button` | yes | yes | yes | 497 | yes | yes |
| `calendar` | yes | yes | — | — | — | — |
| `card` | yes | yes | — | — | — | — |
| `carousel` | yes | yes | — | — | — | — |
| `chart` | yes | yes | — | — | — | — |
| `checkbox` | yes | yes | — | — | — | — |
| `clipboard` | yes | yes | — | — | — | — |
| `collapsible` | yes | yes | — | — | — | — |
| `color-picker` | yes | yes | — | — | — | — |
| `combobox` | yes | yes | — | — | — | — |
| `command` | yes | yes | — | — | — | — |
| `data-table` | yes | yes | — | — | — | — |
| `date-picker` | yes | yes | — | — | — | — |
| `description-list` | yes | yes | — | — | — | — |
| `dialog` | yes | yes | — | — | — | — |
| `dock` | yes | yes | — | — | — | — |
| `dropdown_button` | yes | yes | — | — | — | — |
| `editor` | yes | yes | — | — | — | — |
| `empty` | yes | yes | — | — | — | — |
| `focus-trap` | yes | yes | — | — | — | — |
| `form` | yes | yes | — | — | — | — |
| `group-box` | yes | yes | — | — | — | — |
| `hover-card` | yes | yes | — | — | — | — |
| `icon` | yes | yes | — | — | — | — |
| `image` | yes | yes | — | — | — | — |
| `input` | yes | yes | — | — | — | — |
| `input-group` | yes | yes | — | — | — | — |
| `kbd` | yes | yes | — | — | — | — |
| `label` | yes | yes | — | — | — | — |
| `list` | yes | yes | — | — | — | — |
| `marker` | yes | yes | — | — | — | — |
| `menu` | yes | yes | — | — | — | — |
| `message` | yes | yes | — | — | — | — |
| `message-role` | — | yes | — | — | — | — |
| `message-scroller` | yes | yes | — | — | — | — |
| `notification` | yes | yes | — | — | — | — |
| `number-input` | yes | yes | — | — | — | — |
| `otp-input` | yes | yes | — | — | — | — |
| `pagination` | yes | yes | — | — | — | — |
| `popover` | yes | yes | — | — | — | — |
| `progress` | yes | yes | — | — | — | — |
| `questionnaire` | yes | yes | — | — | — | — |
| `radio` | yes | yes | — | — | — | — |
| `rating` | yes | yes | — | — | — | — |
| `resizable` | yes | yes | — | — | — | — |
| `root` | yes | yes | — | — | — | — |
| `scrollable` | yes | yes | — | — | — | — |
| `select` | yes | yes | — | — | — | — |
| `settings` | yes | yes | — | — | — | — |
| `sheet` | yes | yes | — | — | — | — |
| `shimmer` | yes | yes | — | — | — | — |
| `sidebar` | yes | yes | — | — | — | — |
| `skeleton` | yes | yes | — | — | — | — |
| `slider` | yes | yes | — | — | — | — |
| `speech` | yes | yes | — | — | — | — |
| `spinner` | yes | yes | — | — | — | — |
| `status-bar` | yes | yes | — | — | — | — |
| `stepper` | yes | yes | — | — | — | — |
| `switch` | yes | yes | — | — | — | — |
| `table` | yes | yes | — | — | — | — |
| `tabs` | yes | yes | — | — | — | — |
| `tag` | yes | yes | — | — | — | — |
| `text-view` | yes | yes | — | — | — | — |
| `textarea` | yes | yes | — | — | — | — |
| `theme` | yes | yes | yes | 406 | — | — |
| `time-field` | yes | yes | — | — | — | — |
| `title-bar` | yes | yes | — | — | — | — |
| `toggle` | yes | yes | — | — | — | — |
| `toolbar` | yes | yes | — | — | — | — |
| `tooltip` | yes | yes | — | — | — | — |
| `tree` | yes | yes | — | — | — | — |
| `virtual-list` | yes | yes | — | — | — | — |

**1 / 79** components have an iced source, an example and a registry entry.

Still to do: `accordion`, `alert`, `alert-dialog`, `attachment`, `avatar`, `badge`, `bubble`, `calendar`, `card`, `carousel`, `chart`, `checkbox`, `clipboard`, `collapsible`, `color-picker`, `combobox`, `command`, `data-table`, `date-picker`, `description-list`, `dialog`, `dock`, `dropdown_button`, `editor`, `empty`, `focus-trap`, `form`, `group-box`, `hover-card`, `icon`, `image`, `input`, `input-group`, `kbd`, `label`, `list`, `marker`, `menu`, `message`, `message-role`, `message-scroller`, `notification`, `number-input`, `otp-input`, `pagination`, `popover`, `progress`, `questionnaire`, `radio`, `rating`, `resizable`, `root`, `scrollable`, `select`, `settings`, `sheet`, `shimmer`, `sidebar`, `skeleton`, `slider`, `speech`, `spinner`, `status-bar`, `stepper`, `switch`, `table`, `tabs`, `tag`, `text-view`, `textarea`, `theme`, `time-field`, `title-bar`, `toggle`, `toolbar`, `tooltip`, `tree`, `virtual-list`.
<!-- iced-checklist:end -->
