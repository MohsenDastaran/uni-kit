# NSH component port

The uni-kit catalog written for [Native Shell](https://nsh.dev), the fourth
source-shipped port beside GPUI, Slint and iced. All 79 catalog keys have a
component, a demo, an example and a registry entry.

The component source sits in `src/components/`, where an application copies it.
It is TSX, not a crate: NSH apps run TSX modules on a native Rust runtime, and
the runtime provides `nsh` and `nsh/ui/mobile`. `npx @dastaran/uni-kit add nsh
<component>` writes the component, `theme.ts`, `tokens.ts`, `theme.json`,
`icon.tsx` and `icons.ts` into the app's `src/components/`.

## Run a demo

```sh
script/install-nsh.sh                 # pinned v0.1.1
make -C crates/nsh-component run STORY=button
make dev:website-nsh                  # build the gallery, then serve the site
```

`examples/<module>.tsx` is a whole app (`mount(Demo)`); `nsh run
examples/button.tsx --target web` opens it in the NSH web host. The website's
player serves the compiled bundles at `/nsh-gallery?story=<key>`.

## What is where

| Path | What it is |
| --- | --- |
| `src/components/<module>.tsx` | one component per registry key, in the shape an app installs |
| `src/components/tokens.ts` | the light/dark tables, scales and colour helpers |
| `src/components/theme.ts` | `tokens` as getters over `nsh/device`'s `colorScheme()` |
| `src/components/theme.json` | the same palette for `nsh/ui/mobile` |
| `src/components/icon.tsx` | Canvas + `Path2D` icons |
| `src/components/icons.ts` | generated from `crates/slint-component/ui/icons` |
| `src/demos/<module>.tsx` | the whole gallery app; `// section:` markers feed the website |
| `examples/<module>.tsx` | a thin `mount(Demo)` entry, and the `nsh build` entry |
| `examples/nsh.json` | `"theme": "../src/components/theme.json"` and the app metadata |
| `www/index.html` | the player; `?story=`, `?theme=`, `#loading`, the code-icon bridge |
| `scripts/` | gallery build, gate, checklist, theme parity, generators |

The design authority is `crates/slint-component/ui/theme.slint` and
`ui/<component>.slint`; `website/component/<slug>.md` documents the usage. The
theme chain is `tokens/tokens.json -> theme.slint -> tokens.ts -> theme.json`,
and `scripts/theme-parity.mjs` holds it together.

## Notes

- `nsh check examples/button.tsx` type-checks the whole source tree (the
  `examples/tsconfig.json` includes `../src`).
- `scripts/check.sh --allow-incomplete` is the rollout gate; without the flag
  the generated checklist must be complete.
- `scripts/check-theme-usage.mjs` fails the gate on a fixed component colour,
  a token read outside a reactive render, or an incomplete mobile theme.
- The generator and parity scripts import `tokens.ts`; run them with Node 24 (or Bun), which strips TypeScript types. The gallery build itself needs only Node 18.
- `nsh package` currently rejects the `theme` key.
- The partial ports (editor syntax colours, text-view HTML, speech microphone,
  chart tooltips, dock persistence, web hover) are recorded in
  `docs/nsh-integration.md`. `nsh run` and `nsh build`
  read it, which is how the gallery and the examples get the shared palette.
- The web backend does not raise `onHoverIn`/`onHoverOut`; the demos use
  `hover:` classes for web hover and native press feedback for presses. The
  conventions guide records this and the other platform differences.
- The NSH documentation is vendored under `docs/nsh/`; start at
  `docs/nsh/llms.txt`, and read `docs/nsh/conventions.md` before writing TSX.
