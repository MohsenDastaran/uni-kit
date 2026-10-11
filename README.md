![uni-kit logo](https://raw.githubusercontent.com/MohsenDastaran/uni-kit/main/website/public/uni-kit.svg)  
**uni-kit**

![Publish Website Image](https://github.com/MohsenDastaran/uni-kit/actions/workflows/publish-image.yml/badge.svg)

Copy-paste native UI for Rust desktop toolkits. One catalog of components, written
by hand for each framework, shown as a live WebAssembly example on
[https://uni-kit.dastaran.com](https://uni-kit.dastaran.com), and copied into your
project as source you own — so the first thing you change is the thing you wanted
to change.

uni-kit carries the [GPUI Kit](https://github.com/longbridge/gpui-kit) catalog
past a single framework. GPUI and Slint are both in it today, with the same pages
and the same components; egui, QuickGUI and others join them as their components
are written. The [roadmap](docs/roadmap.md) tracks it.

## Frameworks

| Framework                                     | How an application uses it                                                                                                       |
| --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| [GPUI](https://github.com/zed-industries/zed) | Depend on the `gpui-kit` crate for the controls; `npx uni-kit add gpui <block>` copies a composed screen into `src/components/`. |
| [Slint](https://slint.dev)                    | `npx uni-kit add slint <component>` copies the `.slint` files into `ui/components/`.                                             |
| egui, QuickGUI, others                        | The same pages, once the source exists. [Roadmap](docs/roadmap.md).                                                              |

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

- Thanks to **[Zed Industries](https://zed.dev)**, for
  **[GPUI](https://github.com/zed-industries/zed)** — fast, native, and a pleasure
  to build on.
- Thanks to **[Longbridge](https://longbridge.com)**, whose
  **[GPUI Kit](https://github.com/longbridge/gpui-kit)** and
  **[gpui-component](https://longbridge.com/desktop)** this repository forks: the
  component library, the three blocks and the catalog all start there.
- Thanks to the **[Slint](https://slint.dev)** project and its community, for the
  language, the runtime and the tooling behind the second target. A Rust UI toolkit
  that compiles to native and to the browser is a rare thing, and the catalog here
  is written in their language.
- Thanks to **[shadcn/ui](https://ui.shadcn.com)** and
  **[Reui](https://reui.io)**, whose design language the components follow — and
  whose idea it is that the source is yours once it is copied.
- And to **[Lucide](https://lucide.dev)**, for the icons.

What each of those means for licensing and attribution is under
[License](#license) and in [NOTICE](./NOTICE); it is stated once, there.

## License

Apache-2.0. Third-party attribution is in [NOTICE](./NOTICE).

- Built on [GPUI](https://github.com/zed-industries/zed), the UI framework from Zed Industries, also Apache-2.0. The `gpui-pre-*` crates are snapshots of it, published with Zed's license and notices intact.
- The GPUI component library is extracted from [Longbridge Pro](https://longbridge.com/desktop).
- UI design based on [shadcn/ui](https://ui.shadcn.com), some from [Reui](https://reui.io).
- Icons from [Lucide](https://lucide.dev).
