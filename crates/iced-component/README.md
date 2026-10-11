# iced-component

The iced versions of the uni-kit component catalog, and the gallery the website
embeds. An application does not depend on this crate: it copies the component it
wants out of `src/components/`, with `theme.rs`, into its own project — which is
what `npx uni-kit add iced <component>` does.

Targets **iced 0.14**, pinned in `Cargo.toml`.

## Layout

```text
src/components/       One file per registry key, and the theme every one reads.
                      This is the directory the CLI installs into, so the crate
                      is written in the shape an application ends up with.
src/components/theme.rs   The design tokens, mirrored from the Slint theme.
examples/<name>.rs    One usage example per component — what its page shows.
src/main.rs           The gallery, one application serving every component.
scripts/status.mjs    The checklist in docs/iced-integration.md, generated.
```

## The two gates

```bash
cargo build              # the components and the gallery
cargo build --examples   # every usage example
```

Both must be clean: no errors, and no warnings. A warning in an installed file is
a warning in someone's project.

## Writing a component

Read the Slint version in `crates/slint-component/ui/<name>.slint` and the GPUI
sample on the component's page (`website/component/<name>.md`) first. They are the
design and behaviour being reproduced; iced is only the rendering system. Where
they disagree, prefer the Slint one — it is the framework that has every
component.

The shape, with `button.rs` as the worked example:

- **One file per registry key**, named exactly after it, `pub` items only.
- **`#![allow(dead_code)]`** after the module docs. The file is a library that
  arrived by copy: an application uses part of it and gets warnings for the rest.
- **The theme is `super::theme`.** Never `crate::`. The installed directory can be
  moved or renamed, and `super` keeps working.
- **A builder struct** (`Button::new(label).variant(..).on_press(msg)`) and a
  `From<Control> for Element<'a, Message>`. An iced control carries its message,
  so interactions are values the caller passes in.
- **A `Style` type** holding the flags, with `resolve(&self, tokens, status)`
  returning the widget's own `Style` struct. The `From` impl hands iced a closure:
  `.style(move |theme, status| style.resolve(Tokens::of(theme), status))`.
- **Colours come from the theme at render time**, never from a palette captured
  when the element was built. Text takes `.style(move |theme| text::Style { color:
  Some(..) })` for the same reason.
- **Sizes, spacing, radii and font sizes** come from `theme` constants, and the
  per-size metrics from `ControlSize`.
- **Every interaction state is drawn**: rest, hover, press, disabled, and selected
  where the component has one. `theme::hovered` and `theme::pressed` reproduce the
  Slint theme's HSL shifts.
- **`examples/<name>.rs`** is a complete mini-app — state, message, update, view —
  using the component, because that is what a reader copies.

## Where iced needs a different answer

These are deliberate, and a component that needs one says so in its module docs:

- **Icons are the caller's.** A control that draws one takes
  `Option<Element<Message>>`. Slint ships SVGs because its build script resolves a
  folder; an iced application owns its assets.
- **A control's disabled state is the absence of a message.** iced draws a button
  without `on_press` as disabled, so a component does not take a `disabled: bool`.
- **Dialog, popover, tooltip and menu are application-level** in iced (`stack` or
  `iced::widget::overlay`), so those components provide the panel and its styling
  and let the application place it.
- **Animation has no built-in driver.** A component that spins or interpolates
  takes a `f32` progress value, and the application advances it from a
  subscription. It renders the same at any progress.
