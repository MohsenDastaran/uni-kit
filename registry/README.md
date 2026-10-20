# uni-kit

Copies catalog components and blocks into the app you run it from. The file list is [`components.json`](components.json) in this folder. Publishing a new component is a new entry in that file, not a new release of this package. Rebuild it with `node build-manifest.mjs` after a Slint import changes or a GPUI block is added.

```bash
npx uni-kit add slint alert-dialog
bunx uni-kit add slint alert-dialog
pnpm dlx uni-kit add slint alert-dialog

npx uni-kit add gpui sidebar
npx uni-kit add gpui sidebar dock settings

npx uni-kit add nsh button

npx uni-kit add egui alert-dialog
npx uni-kit add quickgui button
```

`--dir <path>` overrides the framework's default directory.

## Slint

Slint files land in `ui/components/`. A component that uses icons also copies `icons/`, because `icon.slint` names every SVG. The install adds that directory to the Slint include path in `build.rs`, so an app file can import `alert-dialog.slint` by name.

A screen is several components, so name them all: the files they share are written once.

```bash
npx uni-kit add slint sidebar card button data-table
```

## iced

iced components are Rust source, and they land in `src/components/` with the theme they read. Each one is self-contained: it reaches the theme as `super::theme`, so the directory can be moved or renamed without editing a component. The install adds a `pub mod` line per component to `src/components/mod.rs`.

The components compile against iced and nothing else:

```bash
cargo add iced
npx uni-kit add iced button
```

## NSH

NSH components are TSX source, and they land in `src/components/` with
`theme.ts`, `tokens.ts`, `theme.json`, `icon.tsx` and `icons.ts`. A component
imports its siblings as `./theme`, `./icon` and `./icons`, so the directory can
be moved or renamed without editing anything. There is no module file to
declare: the application imports the component it wants directly.

The runtime provides `nsh` and `nsh/ui/mobile`; there is no dependency line.
Point `nsh.json` at the installed theme so the mobile components share the
tokens (the hand-written components read it through `theme.ts` regardless):

```json
{
  "theme": "src/components/theme.json"
}
```

`nsh run` and `nsh build` read that key; `nsh package` currently rejects it.
Without the key the mobile components use NSH's own default palette.

```bash
npx uni-kit add nsh button

# Finished NSH screens copy the screen and every component it composes.
npx uni-kit add nsh settings-block
npx uni-kit add nsh paginated-table-block
```

## GPUI

GPUI components ship in the `gpui-kit` crate, so what is worth copying for this framework is a composed screen. Each block is one file under [`crates/blocks`](../crates/blocks):

| Block | Writes |
| --- | --- |
| `sidebar` | `src/components/sidebar.rs` |
| `dock` | `src/components/dock.rs` |
| `settings` | `src/components/settings.rs` |

The install adds the matching `pub mod` line to `src/components/mod.rs` and says to declare `mod components;` in the crate root when the app does not already. The block imports `gpui_kit`, so the app needs that dependency:

```bash
cargo add gpui-kit
npx uni-kit add gpui sidebar
```

## Where the manifest comes from

From this repository, the command reads `components.json` and the source files on disk. The published package reads the manifest from [MohsenDastaran/uni-kit](https://github.com/MohsenDastaran/uni-kit) `main` and merges it with the copy shipped in the tarball, so a framework that is ready on a branch still installs while `main` catches up. The NSH sources are copied into the tarball by `prepack`, so installing NSH components and blocks does not depend on that branch being reachable. When the bundled sources are absent, NSH reads from its declared `source_ref` branch.

Publish this folder when the command itself changes. `prepack` bundles the NSH catalogue and `postpack` removes the temporary copy:

```bash
cd registry
npm publish
```
