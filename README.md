<p align="center">
  <img src="https://raw.githubusercontent.com/MohsenDastaran/uni-kit/main/website/public/uni-kit.svg" width="64" alt="uni-kit logo" />
  <br>
  <strong>uni-kit</strong>
</p>

![Publish Website Image](https://github.com/MohsenDastaran/uni-kit/actions/workflows/publish-image.yml/badge.svg)

Copy-paste native UI for Rust desktop toolkits. One catalog of components, written
by hand for each framework, shown as a live WebAssembly example on
[https://uni-kit.dastaran.com](https://uni-kit.dastaran.com), and copied into your
project as source you own — so the first thing you change is the thing you wanted
to change.

uni-kit carries the [GPUI Kit](https://github.com/longbridge/gpui-kit) catalog
past a single framework. GPUI, Slint and iced are all in it today, with the same
pages and the same components; egui, QuickGUI and others join them as their
components are written. The [roadmap](docs/roadmap.md) tracks it.

## Frameworks

| Framework                                     | How an application uses it                                                                                                       |
| --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| [GPUI](https://github.com/zed-industries/zed) | Depend on the `gpui-kit` crate for the controls; `npx uni-kit add gpui <block>` copies a composed screen into `src/components/`. |
| [Slint](https://slint.dev)                    | `npx uni-kit add slint <component>` copies the `.slint` files into `ui/components/`.                                             |
| [iced](https://iced.rs)                      | `npx uni-kit add iced <component>` copies the `.rs` files into `src/components/`.                     |
| egui, QuickGUI, others                        | The same pages, once the source exists. [Roadmap](docs/roadmap.md).                                                              |

iced is a third port of the same catalogue, in
[`crates/iced-component`](crates/iced-component): 76 of the catalog's 79
components, with 80 demos, written against iced 0.14 and sharing the same design
tokens and documentation site. Its source is copied like Slint's, not depended
on.

> **Not published from `main` yet.** iced support lives on the `iced` branch, and
> `main`'s `registry/components.json` has no `iced` key. The CLI reads that
> manifest — the copy bundled in the npm package is only an offline fallback — so
> `npx uni-kit add iced <component>` currently fails with
> `Framework "iced" is not in the registry`. It starts working once the branch is
> merged and the registry is published; until then, copy the source from
> [`crates/iced-component/src/components`](crates/iced-component/src/components).

## What is in this repository

```text
crates/kit              gpui-kit — the one crate an application depends on
crates/base             gpui-base — behavior, state and infrastructure, no styling
crates/component        gpui-component — the styled component library
crates/shell            gpui-shell — JavaScript extensions for a Rust host
crates/component-shell  The styled catalog for that host
crates/blocks           Composed GPUI screens the CLI installs
crates/slint-component  The same catalog, written for Slint 1.17
crates/iced-component   The same catalog, written for iced 0.14
crates/story            The component gallery, as a native application
crates/story-web        The same gallery, built for the browser
registry/               The uni-kit CLI and the manifest it installs from
website/                The documentation site and its live examples
```

`gpui_kit::init` runs before any component is used, and the first view in a window
is a `Root`. Architecture of the GPUI crates is in
[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md). Slint components are source you
copy, not a crate from this repository: take `theme.slint`, the component, the
files it imports, and the `icons/` folder beside `icon.slint`. Adding a framework
port is in [docs/adding-a-framework.md](docs/adding-a-framework.md).

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
- **[iced](https://iced.rs)** — the framework the iced components are written
  against, and the widgets and styling model they are built from.

## License

Apache-2.0. Third-party attribution is in [NOTICE](./NOTICE).

- Built on [GPUI](https://github.com/zed-industries/zed), the UI framework from Zed Industries, also Apache-2.0. The `gpui-pre-*` crates are snapshots of it, published with Zed's license and notices intact.
- The GPUI component library is extracted from [Longbridge Pro](https://longbridge.com/desktop).
