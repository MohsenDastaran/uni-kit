![uni-kit logo](https://raw.githubusercontent.com/MohsenDastaran/uni-kit/main/website/public/uni-kit.svg)  
**GPUI Kit**

![Publish Website Image](https://github.com/MohsenDastaran/uni-kit/actions/workflows/publish-image.yml/badge.svg) ![Docs](https://docs.rs/gpui-kit/badge.svg) ![Crates.io](https://img.shields.io/crates/v/gpui-kit.svg)

Copy-paste native UI for Rust desktop toolkits. One catalog of components, written
by hand for each framework, shown as a live WebAssembly example on
[https://gpui-kit.com](https://gpui-kit.com), and copied into your project as source you own — so the
first thing you change is the thing you wanted to change.

GPUI and Slint are in the catalog. egui, QuickGUI and other toolkits join the same
pages as their components are written; the [roadmap](docs/roadmap.md) tracks it.

## Frameworks

| Framework                                     | How an application uses it                                                                                                       |
| --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| [GPUI](https://github.com/zed-industries/zed) | Depend on the `gpui-kit` crate for the controls; `npx uni-kit add gpui <block>` copies a composed screen into `src/components/`. |
| [Slint](https://slint.dev)                    | `npx uni-kit add slint <component>` copies the `.slint` files into `ui/components/`.                                             |
| egui, QuickGUI, others                        | The same pages, once the source exists. [Roadmap](docs/roadmap.md).                                                              |

```bash
npx uni-kit add slint alert-dialog
npx uni-kit add gpui sidebar dock settings
```

The CLI does not generate components; it copies the files listed in
`[registry/components.json](registry/components.json)`. Slint writes one file per
control, GPUI one file per block, because GPUI controls ship in the crate rather
than as source. `--dir` overrides the destination.

## What is in this repository

```text
crates/kit              gpui-kit — the one crate an application depends on
crates/base             gpui-base — behavior, state and infrastructure, no styling
crates/component        gpui-component — the styled component library
crates/shell            gpui-shell — JavaScript extensions for a Rust host
crates/component-shell  The styled catalog for that host
crates/blocks           Composed GPUI screens the CLI installs
crates/slint-component  The same catalog, written for Slint 1.17
crates/story            The component gallery, as a native application
crates/story-web        The same gallery, built for the browser
registry/               The uni-kit CLI and the manifest it installs from
website/                The documentation site and its live examples
```

`gpui_kit::init` runs before any component is used, and the first view in a window
is a `Root`. Architecture of the GPUI crates is in
[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md). Slint components are source you
copy, not a crate from this repository: take `theme.slint`, the component, the
files it imports, and the `icons/` folder beside `icon.slint`.

## Development

```bash
cargo run              # the GPUI component gallery
make dev:website       # the site, served from a production build
make dev               # the site, rebuilding the Slint gallery first
```

`make dev` is the one for Slint work: it rebuilds the gallery wasm and then starts  
the site, so a refresh shows the change. With no `SLUGS` it rebuilds only the  
pages whose sources changed.

## Credits

This project stands on other people's work, and most of what is good here is
theirs.

- **[GPUI](https://github.com/zed-industries/zed)** — the framework from
  [Zed Industries](https://zed.dev) that everything on the GPUI side renders
  with. Fast, native, and a pleasure to build on. The `gpui-pre-*` crates are
  snapshots of it, published with Zed's license and notices intact.
- **[GPUI Kit](https://github.com/longbridge/gpui-kit)** and
  **[gpui-component](https://longbridge.com/desktop)** — from
  [Longbridge](https://longbridge.com). This repository is a fork of GPUI Kit: the
  component library, the three blocks and the catalog all start there, and the
  copyright notice in [NOTICE](./NOTICE) is theirs.
- **[Slint](https://slint.dev)** and its community — the language, the runtime and
  the tooling behind the second target. A Rust UI toolkit that compiles to native
  and to the browser is a rare thing, and the catalog here is written in their
  language.
- **[shadcn/ui](https://ui.shadcn.com)** and **[Reui](https://reui.io)** — the
  design language the components follow, including the idea that the source is
  yours once it is copied.
- **[Lucide](https://lucide.dev)** — the icons.

## License

Apache-2.0. Third-party attribution is in [NOTICE](./NOTICE).

- Built on [GPUI](https://github.com/zed-industries/zed), the UI framework from Zed Industries, also Apache-2.0. The `gpui-pre-*` crates are snapshots of it, published with Zed's license and notices intact.
- The GPUI component library is extracted from [Longbridge Pro](https://longbridge.com/desktop).
- UI design based on [shadcn/ui](https://ui.shadcn.com), some from [Reui](https://reui.io).
- Icons from [Lucide](https://lucide.dev).
