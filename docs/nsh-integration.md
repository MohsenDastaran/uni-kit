# NSH integration

NSH (Native Shell, <https://nsh.dev>) is the fourth port beside GPUI, Slint and
iced. It is **source-shipped**: `npx @dastaran/uni-kit add nsh <component>`
writes TSX into the application's `src/components/`, with the shared token file,
the mobile theme and the icon map. There is no crate to depend on; the NSH
runtime provides `nsh` and `nsh/ui/mobile`.

## Shape

```text
crates/nsh-component/
  src/components/<module>.tsx   the component an application installs
  src/components/theme.ts       reactive tokens (getters over colorScheme())
  src/components/tokens.ts      the pure light/dark tables and scales
  src/components/theme.json     the same palette for nsh/ui/mobile
  src/components/icon.tsx       Canvas + Path2D icons
  src/components/icons.ts       generated path data from the Slint icon set
  src/demos/<module>.tsx        the whole app the gallery and page show
  examples/<module>.tsx         a thin `mount(Demo)` entry
  examples/nsh.json             the demo app's root, theme path and metadata
  www/index.html                the player page (`?story=`, `?theme=`)
  scripts/                      build, check, status, theme parity, generators
```

The component is a typed function component (`Button(props)`), not a builder:
NSH components run once and follow signals, so the framework's own idiom is
props and callbacks. `nsh/ui/mobile` is reused where its contract matches (`Group`, `ListView`,
`Modal`, `Sheet`, `Stepper`). The catalog controls whose contract is larger are
built from host elements: `Button` (ten variants, sizes, icons, loading),
`Card` (cover, description, inset), `Input` (frame, adornments, clear and
reveal), and the overlay menus (the trigger and filtering are the
component). The reuse decision is recorded in each component file.

## Tokens

The authority chain is unchanged:

```text
tokens/tokens.json -> crates/slint-component/ui/theme.slint
                   -> crates/nsh-component/src/components/tokens.ts
                   -> crates/nsh-component/src/components/theme.json
```

`theme.ts` exposes the tables as one object of getters, so a component writes
`tokens.background` in JSX and follows a light/dark switch. `theme.json` is
generated for `nsh/ui/mobile`; `nsh.json` points at it. `scripts/theme-parity.mjs`
checks every token against the Slint theme, `scripts/check-theme-usage.mjs`
rejects fixed colours and module-level token reads. The 79 demos are checked at
mount in both appearances and by switching the mounted demo live, including
page surface, dark-on-dark text and light-on-light text.
`nsh package` currently rejects the `theme` key, so the gallery uses `nsh build` and the player; that
limitation is documented in `crates/nsh-component/README.md`.

## Gallery

`make dev:website-nsh` builds every demo and serves the site. Each
`examples/<module>.tsx` becomes a `.nshc` bundle under `www/dist/demos/`; the
player at `/nsh-gallery?story=<key>` loads it on the distribution's web runtime.
The site loads the player with `?embed=1`: the player then leaves its document
surface transparent and the iframe's `var(--background)` (the reader's palette)
shows through. `Page` is transparent for the same reason; standalone mode paints
the player's own light/dark document. The player removes its `#loading` element
when the app has drawn its first frame, follows the host page's `dark` class, and
turns the demo's
`nsh-gallery-source:` console line into the `{source:"nsh-component",index,title}`
message the component page's code icon uses. The heading matches the iced
gallery: a 28 pt box around the 16 pt `code-xml` glyph, with 0.12 / 0.22 /
0.34 fill and 0.35 / 0.55 / 0.7 border at rest, hover and press; the title is a
second press target. Extra examples the page has no heading for are added as
NSH-only sections under Usage.

## Checklist

<!-- nsh-checklist:start -->
| Component | NSH file | GPUI ref | Slint ref | NSH source | lines | example | registry |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `accordion` | `accordion.tsx` | yes | yes | yes | 257 | yes | yes |
| `alert` | `alert.tsx` | yes | yes | yes | 167 | yes | yes |
| `alert-dialog` | `alert_dialog.tsx` | yes | yes | yes | 77 | yes | yes |
| `attachment` | `attachment.tsx` | yes | yes | yes | 546 | yes | yes |
| `avatar` | `avatar.tsx` | yes | yes | yes | 348 | yes | yes |
| `badge` | `badge.tsx` | yes | yes | yes | 431 | yes | yes |
| `bubble` | `bubble.tsx` | yes | yes | yes | 551 | yes | yes |
| `button` | `button.tsx` | yes | yes | yes | 285 | yes | yes |
| `calendar` | `calendar.tsx` | yes | yes | yes | 526 | yes | yes |
| `card` | `card.tsx` | yes | yes | yes | 218 | yes | yes |
| `carousel` | `carousel.tsx` | yes | yes | yes | 243 | yes | yes |
| `chart` | `chart.tsx` | yes | yes | yes | 576 | yes | yes |
| `checkbox` | `checkbox.tsx` | yes | yes | yes | 211 | yes | yes |
| `clipboard` | `clipboard.tsx` | yes | yes | yes | 49 | yes | yes |
| `collapsible` | `collapsible.tsx` | yes | yes | yes | 224 | yes | yes |
| `color-picker` | `color_picker.tsx` | yes | yes | yes | 77 | yes | yes |
| `combobox` | `combobox.tsx` | yes | yes | yes | 104 | yes | yes |
| `command` | `command.tsx` | yes | yes | yes | 112 | yes | yes |
| `data-table` | `data_table.tsx` | yes | yes | yes | 59 | yes | yes |
| `date-picker` | `date_picker.tsx` | yes | yes | yes | 78 | yes | yes |
| `description-list` | `description_list.tsx` | yes | yes | yes | 92 | yes | yes |
| `dialog` | `dialog.tsx` | yes | yes | yes | 151 | yes | yes |
| `dock` | `dock.tsx` | yes | yes | yes | 54 | yes | yes |
| `dropdown_button` | `dropdown_button.tsx` | yes | yes | yes | 48 | yes | yes |
| `editor` | `editor.tsx` | yes | yes | yes | 86 | yes | yes |
| `empty` | `empty.tsx` | yes | yes | yes | 94 | yes | yes |
| `focus-trap` | `focus_trap.tsx` | yes | yes | yes | 24 | yes | yes |
| `form` | `form.tsx` | yes | yes | yes | 68 | yes | yes |
| `group-box` | `group_box.tsx` | yes | yes | yes | 50 | yes | yes |
| `hover-card` | `hover_card.tsx` | yes | yes | yes | 71 | yes | yes |
| `icon` | `icon.tsx` | yes | yes | yes | 73 | yes | yes |
| `image` | `image.tsx` | yes | yes | yes | 59 | yes | yes |
| `input` | `input.tsx` | yes | yes | yes | 217 | yes | yes |
| `input-group` | `input_group.tsx` | yes | yes | yes | 66 | yes | yes |
| `kbd` | `kbd.tsx` | yes | yes | yes | 79 | yes | yes |
| `label` | `label.tsx` | yes | yes | yes | 73 | yes | yes |
| `list` | `list.tsx` | yes | yes | yes | 46 | yes | yes |
| `marker` | `marker.tsx` | yes | yes | yes | 70 | yes | yes |
| `menu` | `menu.tsx` | yes | yes | yes | 81 | yes | yes |
| `message` | `message.tsx` | yes | yes | yes | 48 | yes | yes |
| `message-role` | `message_role.tsx` | yes | yes | yes | 32 | yes | yes |
| `message-scroller` | `message_scroller.tsx` | yes | yes | yes | 28 | yes | yes |
| `notification` | `notification.tsx` | yes | yes | yes | 97 | yes | yes |
| `number-input` | `number_input.tsx` | yes | yes | yes | 61 | yes | yes |
| `otp-input` | `otp_input.tsx` | yes | yes | yes | 50 | yes | yes |
| `pagination` | `pagination.tsx` | yes | yes | yes | 40 | yes | yes |
| `popover` | `popover.tsx` | yes | yes | yes | 41 | yes | yes |
| `progress` | `progress.tsx` | yes | yes | yes | 116 | yes | yes |
| `questionnaire` | `questionnaire.tsx` | yes | yes | yes | 94 | yes | yes |
| `radio` | `radio.tsx` | yes | yes | yes | 104 | yes | yes |
| `rating` | `rating.tsx` | yes | yes | yes | 52 | yes | yes |
| `resizable` | `resizable.tsx` | yes | yes | yes | 59 | yes | yes |
| `root` | `root.tsx` | yes | yes | yes | 30 | yes | yes |
| `scrollable` | `scrollable.tsx` | yes | yes | yes | 13 | yes | yes |
| `select` | `select.tsx` | yes | yes | yes | 133 | yes | yes |
| `settings` | `settings.tsx` | yes | yes | yes | 94 | yes | yes |
| `sheet` | `sheet.tsx` | yes | yes | yes | 30 | yes | yes |
| `shimmer` | `shimmer.tsx` | yes | yes | yes | 62 | yes | yes |
| `sidebar` | `sidebar.tsx` | yes | yes | yes | 66 | yes | yes |
| `skeleton` | `skeleton.tsx` | yes | yes | yes | 79 | yes | yes |
| `slider` | `slider.tsx` | yes | yes | yes | 103 | yes | yes |
| `speech` | `speech.tsx` | yes | yes | yes | 100 | yes | yes |
| `spinner` | `spinner.tsx` | yes | yes | yes | 105 | yes | yes |
| `status-bar` | `status_bar.tsx` | yes | yes | yes | 53 | yes | yes |
| `stepper` | `stepper.tsx` | yes | yes | yes | 36 | yes | yes |
| `switch` | `switch.tsx` | yes | yes | yes | 65 | yes | yes |
| `table` | `table.tsx` | yes | yes | yes | 70 | yes | yes |
| `tabs` | `tabs.tsx` | yes | yes | yes | 65 | yes | yes |
| `tag` | `tag.tsx` | yes | yes | yes | 119 | yes | yes |
| `text-view` | `text_view.tsx` | yes | yes | yes | 60 | yes | yes |
| `textarea` | `textarea.tsx` | yes | yes | yes | 77 | yes | yes |
| `theme` | `theme.ts` | yes | yes | yes | 74 | yes | yes |
| `time-field` | `time_field.tsx` | yes | yes | yes | 68 | yes | yes |
| `title-bar` | `title_bar.tsx` | yes | yes | yes | 93 | yes | yes |
| `toggle` | `toggle.tsx` | yes | yes | yes | 73 | yes | yes |
| `toolbar` | `toolbar.tsx` | yes | yes | yes | 63 | yes | yes |
| `tooltip` | `tooltip.tsx` | yes | yes | yes | 66 | yes | yes |
| `tree` | `tree.tsx` | yes | yes | yes | 66 | yes | yes |
| `virtual-list` | `virtual_list.tsx` | yes | yes | yes | 31 | yes | yes |

**79 / 79** components have an NSH source, an example and a registry entry.

Every component in the Slint registry has an NSH implementation.
<!-- nsh-checklist:end -->

## Partial ports and limits

The checklist is complete: every Slint key has an NSH source, an example and a
registry entry. The following are honest partial ports, not stubs:

| Area | What is complete | What is not |
| --- | --- | --- |
| `editor` | Frame, line numbers, toolbar, controlled value, read-only | Syntax colours: NSH's native `Input` cannot style spans. |
| `text-view` | Headings, paragraphs, lists, quotes, fenced code | HTML rendering, inline emphasis, links with a click target. |
| `speech` | Dictation UI, listening state, app-provided recognizer interface, scripted demo | A built-in microphone/recognizer: NSH has no such API; the host app supplies one. |
| `chart` | Line, area, bar, pie, radar, candlestick and a simplified Sankey on Canvas | Tooltips, legends, and Sankey links (the flows are stacked bars). |
| `dock` | Tabbed group and resizable split | Panel drag-and-drop, persistence and the full tree model. |
| `resizable` | Two panes and a draggable divider | Constraint solvers and nested resizable trees. |
| `tooltip`, `hover-card` | Hover on desktop, press/long press on the web | Web hover: the web backend does not raise `onHoverIn`/`onHoverOut`. |
| `title-bar` | Desktop custom bar via `nsh/desktop`, preview elsewhere | Web/mobile window controls are the platform's, not drawn. |
| `theme` | The shared token files and a swatch demo | The page is a guide (`example: false`), so the site shows its source panels but no live frame. |
| `notification` | In-app toast host, variants, auto-dismiss | The OS notification centre. |
| Packaging | `nsh build`/`nsh run` | `nsh package` currently rejects the `theme` key; a packaged app can omit it. |

## Portability

`nsh trace` and the web player verify every demo on this machine. iOS, Android,
macOS, Windows and GPUI windows cannot be run here; the source avoids
platform-only APIs except in `title-bar` (which uses `nsh/desktop` behind
`isAvailable`) and uses only elements and modules available on every NSH
backend. The final section records any component that is partial or absent.
