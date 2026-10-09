# uni-kit

Copies catalog components and blocks into the app you run it from. The file list is [`components.json`](components.json) in this folder. Publishing a new component is a new entry in that file, not a new release of this package. Rebuild it with `node build-manifest.mjs` after a Slint import changes or a GPUI block is added.

```bash
npx uni-kit add slint alert-dialog
bunx uni-kit add slint alert-dialog
pnpm dlx uni-kit add slint alert-dialog

npx uni-kit add gpui sidebar
npx uni-kit add gpui sidebar dock settings

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

From this repository, the command reads `components.json` and the source files on disk. The published package reads the manifest from [MohsenDastaran/uni-kit](https://github.com/MohsenDastaran/uni-kit) `main`, and falls back to the copy shipped in the tarball when the network is unavailable. The manifest is in the tarball, but it is not authoritative there — landing a new entry on `main` is enough to publish it to `npx` users, and the sources it names are fetched from `main` too.

Publish this folder when the command itself changes:

```bash
cd registry
npm publish
```
