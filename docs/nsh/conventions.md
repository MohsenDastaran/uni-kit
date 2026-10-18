# Writing NSH code in uni-kit

Native Shell (NSH) apps are TSX modules run by QuickJS on a native Rust runtime.
There is no React and no DOM. The renderer turns JSX into native views, and
signals update the exact expressions that read them. This is the short guide for
working in `crates/nsh-component`; the complete reference is vendored under
`docs/nsh/` (start at [`llms.txt`](llms.txt)).

## The model

- A component is a function that runs **once** and returns JSX. It does not
  re-render. A signal read inside JSX follows the signal; a prop read inside JSX
  follows the prop.
- Props are getters. Read `props.title` where it is displayed, not into a local
  at the top of the component. Destructuring `const { title } = props` freezes
  the first value.
- Never pass a signal setter as a callback (`onChange={count.set}`). The method
  loses its signal and fails at runtime with `not a live signal`. Wrap it:
  `onChange={(next) => count.set(next)}`. Give the parameter a name other than
  the signal (`(value) => value.set(value)` shadows the signal and fails with
  `not a function`).
- `mount(App)` is the entry point. Every `examples/<module>.tsx` is a whole app
  and calls it; `src/demos/<module>.tsx` exports the `Demo` it mounts.
- Elements are PascalCase and imported: `View`, `Text`, `Scroll`, `Input` from
  `nsh`; `Image` from `nsh/image`; `List` from `nsh/list`; `Canvas` and `Path2D`
  from `nsh/canvas`; `Modal` and the mobile set from `nsh/ui/mobile`. Only `If`
  and `Each` are globals.
- Text is only drawn inside `<Text>`. A string directly in a `<View>` is not
  shown.
- There is no `window`, `document`, `localStorage` or DOM. The runtime provides
  `console`, timers, `fetch`, `URL`, streams, `Intl` and `requestAnimationFrame`.

## Theming

- Read `tokens.*` in JSX, a `draw` or an effect. `theme.ts` defines each key as
  a getter over `colorScheme()`, so the expression follows a light/dark change
  without re-running the component. Never destructure `tokens`, spread it, or
  assign one of its values to a module-level constant: that freezes the light
  value.
- Do not write a colour literal in a component. Status chips derive their label
  ink with `inkOn(fill)`; data swatches (the colour picker's palette) are the
  one exception.
- `tokens.ts` is the shared palette, `theme.json` is the same palette for
  `nsh/ui/mobile` (`Group`, `ListView`, `Modal`, `Sheet`, `Stepper`), and
  `scripts/theme-parity.mjs` checks both against the Slint theme.
- `scripts/check-theme-usage.mjs` is part of the gate: it rejects fixed colours,
  module-level token reads, and a missing mobile theme key.
- Every demo wraps its content in `Page`, which paints `tokens.background`; a
  demo must not put text on the page without a token colour (`NSH`'s `Text`
  defaults to black).

## Styling

- `style` is an object of camelCase properties, or an array merged left to
  right. Numbers are points. `style` wins over `class`.
- Layout is flexbox with **column** as the default; `flexShrink` defaults to 0.
  Use `flexDirection: "row"` for a row, `gap` for spacing, `flex: 1` to share
  space.
- Colors are CSS strings. Read them from `./theme`:

  ```tsx
  import { tokens, RADIUS_MD, controlHeight } from "./theme";
  <View style={{ backgroundColor: tokens.background, borderRadius: RADIUS_MD }} />
  ```

  `tokens` is a single object of getters over `colorScheme()`, so a light/dark
  switch updates every expression that reads it. Do not spread it (`{...tokens}`
  reads once) and do not move it into a module-level constant.
- `nsh/ui/mobile` reads `nsh.json`'s `"theme"`. The app points at
  `src/components/theme.json`; `theme.json` is generated from `tokens.ts` and
  checked against `theme.slint` by `scripts/theme-parity.mjs`.
- Hover: `onHoverIn`/`onHoverOut` fire on macOS, Linux and GPUI but **not on the
  web**. Use `class="hover:opacity-90"` (or another `hover:` utility) for web
  hover, and `activeOpacity`/`underlayColor`/`active:` for presses. A control
  that reveals content on hover needs a press/focus trigger on the web.
- Press feedback is native: `activeOpacity`, `underlayColor`, `active:` classes.
  `disabled` removes the press listeners.
- `Canvas` has no content size; always give it `width`/`height`/`flex`/
  `aspectRatio`. Import `Path2D` from `nsh/canvas`; it is not a global (a
  `ReferenceError: Path2D is not defined` inside `draw` is this).

## The gallery contract

- `src/components/<module>.tsx` is the component an application installs. It
  imports only `./theme`, `./tokens`, `./icon`, `./icons`, `nsh` and
  `nsh/...`; the registry computes that closure.
- `src/demos/<module>.tsx` is the whole app the page shows. Mark each documented
  sample with a statement-level comment:

  ```tsx
  // section: Variants
  const variants = (<Row>…</Row>);
  // section: end
  ```

  One marker per sample, and `// section: end` after the last one. A marker
  titled like a page heading fills that heading; a title the page does not have
  becomes an NSH-only heading under `## Usage`. The marker titles are lookup
  keys for the page headings.
- `examples/<module>.tsx` is a thin `mount(Demo)`. It is the `nsh build` entry,
  the thing `nsh run` opens, and the file the website counts as a real example.
- `ExampleSection` (in `src/demos/ui.tsx`) draws the heading and the code chip.
  The chip is the iced gallery's: a 28pt box with the 16pt `code-xml` glyph,
  8pt radius, 0.12 / 0.22 / 0.34 primary fill and 0.35 / 0.55 / 0.7 border at
  rest, hover and press. Pressing the title or the chip calls
  `showSource(index, title)`, which writes
  `nsh-gallery-source:{"index":…,"title":…}` to `console`. The player turns that
  into the component page's postMessage. The app has no `window.parent`.
- A demo section whose title the page does not have becomes an NSH-only heading
  under `## Usage`; it is not placed under a heading owned by another framework.
  Keep a `// section: Usage` in the shared components so that section is real
  NSH code, not another framework's sample.
- The player removes `#loading` when the app paints its first child, follows the
  host page's `dark` class, and resolves `nsh.js`/`nsh.wasm`/`demos/*.nshc`
  from the directory the page is mounted at. The site passes `?embed=1`; the
  player then leaves the page surface transparent, so the iframe's
  `--background` palette shows through. `Page` is transparent for that reason;
  the standalone gallery paints the player's document background instead. `/nsh-gallery` has no trailing
  slash, so the player computes that base explicitly; `assetBase` must be built
  from that base too, or `asset:/assets/...` resolves against the site root.
- Overlays use `Modal` (transparent) when they must escape a scrolling page:
  Select, Combobox, DatePicker, DropdownButton and Popover. Tooltip and
  HoverCard keep an absolute layer beside their trigger. An absolute panel
  inside a `Scroll` can be painted under the next section on the web, which is
  why the clipped cases are modals.

## Commands

```sh
script/install-nsh.sh                          # pinned v0.1.1
nsh check examples/button.tsx                  # type-checks src too
nsh trace examples/button.tsx --taps 0         # headless render
make -C crates/nsh-component run STORY=button  # nsh run --target web
make dev:website-nsh                           # build the gallery, serve the site
node crates/nsh-component/scripts/theme-parity.mjs
node crates/nsh-component/scripts/status.mjs --check
```

`nsh package` currently rejects the `theme` key. Use `nsh build`/`nsh run` (the
gallery does); a packaged app can omit the theme key and use NSH's built-in
palette.

## Porting notes

- The authority is `crates/slint-component/ui/<key>.slint` and
  `crates/slint-component/ui/theme.slint`; `website/component/<slug>.md`
  documents the usage. Read the Slint file first, then the iced demo for
  behaviour, then write the NSH component.
- Reuse `nsh/ui/mobile` where its contract matches: `Group`, `ListView`,
  `Modal`, `Sheet`, `Stepper`. Where the catalog contract is larger (Button's
  variants/sizes/icons/loading, Card's cover/description/inset, Input's frame
  and adornments, the trigger+filter of Select/Combobox), build from host
  elements and record the decision.
- Every catalog key needs a component, an example and a registry entry. A stub
  is worse than an explicit exemption: `status.mjs` refuses "not implemented"
  and placeholder-only files.
- Platform-specific modules (`nsh/desktop`, `nsh/ios`, `nsh/android`) are
  imported only where the component is platform-specific and guarded with
  `isAvailable`; the rest of the catalog uses cross-platform elements.
