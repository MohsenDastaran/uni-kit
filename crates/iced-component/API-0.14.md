# iced 0.14 — API reference

Verified against docs.rs and the iced repository at tag 0.14.0 while writing the
components. Kept here because the difference from the 0.12/0.13 tutorials that
dominate the web is what a component gets wrong. Claims that could not be
verified say so.

---

Reference for writing the components. Every claim below is verified
against `docs.rs/iced/0.14.0`, `docs.rs/iced_widget/0.14.0`, or the iced
repository at tag 0.14.0.

All claims below are verified against `docs.rs/iced/0.14.0`, `docs.rs/iced_widget/0.14.0`, or `raw.githubusercontent.com/iced-rs/iced/0.14.0/{core,runtime,src,widget,futures}/src/*.rs`. Anything I could not verify is marked **UNVERIFIED**. Note: `docs.rs/iced/0.14.0` has several **broken intra-doc links** (`widget::MouseArea`, `Responsive`, `Stack`, `Column`, `Row` 404 on docs.rs); for those I read the 0.14.0 source directly, which is the same authority.

---

## 1. Version choice

- Newest non-yanked on crates.io: **`iced 0.14.0`**, published **2025-12-07T20:51:51Z** (`crates.io/api/v1/crates/iced`: `max_stable_version = "0.14.0"`, `newest_version = "0.14.0"`).
- **No newer major/minor exists as a release candidate.** GitHub releases list only `0.14.0, 0.13.1, 0.13.0, 0.12.1, 0.12.0, 0.10.0…`; none are prereleases. There are no `0.15.0-*` tags. `master` is unreleased dev (CHANGELOG has `## [Unreleased]` and is empty after the 0.14.0 section).
- **Recommendation: target `iced = "0.14.0"`** (not `0.13.1`). 0.14.0 is the only release with the `table`, `grid`, `float`, `pin`, `sensor`, `stack` widgets, the closure/`Catalog` styling model, reactive rendering, and `combo_box`'s stable shape. 0.13.1 is a valid fallback only if you are pinned to edition 2021 / Rust < 1.88, and then most of this reference does not apply.
- **Desktop `Cargo.toml`:**
```toml
[dependencies]
iced = { version = "0.14.0", features = ["svg", "canvas", "qr_code", "image", "markdown"] }
# default = ["wgpu","tiny-skia","crisp","web-colors","thread-pool","linux-theme-detection","x11","wayland"]
```
  Add `"tokio"` (or `"smol"`) if you need `time::every` / `time::repeat` on native — see §11 gotcha 8. Choose exactly **one** of `tokio` / `smol` / `thread-pool`; `thread-pool` is the default and also the default executor.
- **Wasm/web `Cargo.toml`** (mirrors the official `examples/tour/Cargo.toml` at tag 0.14.0):
```toml
[dependencies]
iced = { version = "0.14.0", default-features = false, features = [
  "wgpu",        # WebGPU backend, all default wgpu features
  # or "webgl",  # WebGL2 backend — pick ONE; tour uses "webgl"
  "tiny-skia",
  "crisp",
  "web-colors",
  "fira-sans",   # embeds Fira Sans: needed because there is no system font on the web
  "svg", "canvas", "qr_code", "image",
] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
console_error_panic_hook = "0.1"
console_log = "1.0"
```
  There is **no `web` cargo feature**. Web support is the `wasm32-unknown-unknown` target + the `webgl` (`iced_renderer/webgl`) or `wgpu` renderer + `fira-sans`. `default-features = false` is required so the default `x11`/`wayland`/`linux-theme-detection` winit features are not pulled in.
- Feature list (verbatim from the 0.14.0 root `Cargo.toml`): `wgpu`, `wgpu-bare`, `tiny-skia`, `image`, `image-without-codecs`, `svg`, `canvas`, `qr_code`, `markdown`, `lazy`, `debug`, `time-travel`, `hot`, `tester`, `thread-pool`, `tokio`, `smol`, `sysinfo`, `web-colors`, `crisp`, `webgl`, `highlighter`, `selector`, `advanced`, `fira-sans`, `basic-shaping`, `advanced-shaping`, `strict-assertions`, `unconditional-rendering`, `sipper`, `linux-theme-detection`, `x11`, `wayland`.

## 2. MSRV and toolchain

- **MSRV: `1.88`** — verified twice: crates.io API `"rust_version":"1.88"` for 0.14.0, and the repo root `Cargo.toml` `[workspace.package] rust-version = "1.88"`.
- **Edition: `2024`** (`[workspace.package] edition = "2024"`; crates.io reports `"edition":"2024"`). 0.13.1 was edition 2021 / rust-version 1.80.
- **Nightly:** the library itself requires only stable 1.88. The `hot` (hot-reloading) and `time-travel` features are documented as "very experimental" and pull `iced_devtools`; the repo does not state a nightly requirement. **UNVERIFIED** whether `hot` needs nightly — do not use it in a component library.
- Local toolchain check: `rustc 1.97.0` is installed here, so 0.14.0 compiles toolchain-wise.

## 3. Application skeleton

**Removed in 0.14.0 (and already gone in 0.13.x):**
- `iced::Application` **trait — DOES NOT EXIST** (the *name* `Application` now denotes a builder struct).
- `iced::Sandbox` **trait — DOES NOT EXIST** (zero occurrences in the 0.14.0 all-items index; also absent from the 0.13.1 index).
- `iced::Command` **— DOES NOT EXIST.** Replaced by `iced::Task<Message>`. (`all.html` contains zero `Command` items; the only "Command" strings are lyon's `PathCommands`.)
- `iced::subscription::Subscription` — still exists as `iced::Subscription`.

**Current entry points (exact, verbatim):**
```rust
pub fn application<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Application<impl Program<State = State, Message = Message, Theme = Theme>>
where
    State: 'static,
    Message: Send + 'static,
    Theme: Base,
    Renderer: Renderer,

pub fn run<State, Message, Theme, Renderer>(
    update: impl UpdateFn<State, Message> + 'static,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer> + 'static,
) -> Result
where
    State: Default + 'static,
    Message: Send + MaybeDebug + MaybeClone + 'static,
    Theme: Base + 'static,
    Renderer: Renderer + 'static,
```
```rust
pub trait BootFn<State, Message>   { fn boot(&self) -> (State, Task<Message>); }
pub trait UpdateFn<State, Message> { fn update(&self, state: &mut State, message: Message) -> Task<Message>; }
pub trait ViewFn<'a, State, Message, Theme, Renderer> {
    fn view(&self, state: &'a State) -> Element<'a, Message, Theme, Renderer>;
}
pub trait IntoBoot<State, Message> { fn into_boot(self) -> (State, Task<Message>); }
pub trait ThemeFn<State, Theme> { fn theme(&self, state: &State) -> Option<Theme>; }
pub trait TitleFn<State>        { fn title(&self, state: &State) -> String; }
```
Boot accepts `State` (via `impl IntoBoot`) **or** a closure `Fn() -> (State, Task<Message>)`. `update` may return `()` **or** `Task<Message>` (both are covered by blanket impls). **Note `ThemeFn::theme` takes only `(&State)` — no `window::Id`**, unlike the low-level `Program` trait.

**`Application` builder methods (exact, all on `impl<P: Program> Application<P>`):**
```rust
pub fn run(self) -> Result where Self: 'static, P::Message: MaybeDebug + MaybeClone,
pub fn settings(self, settings: Settings) -> Self
pub fn antialiasing(self, antialiasing: bool) -> Self
pub fn default_font(self, default_font: Font) -> Self
pub fn font(self, font: impl Into<Cow<'static, [u8]>>) -> Self
pub fn window(self, window: Settings) -> Self
pub fn centered(self) -> Self
pub fn exit_on_close_request(self, exit_on_close_request: bool) -> Self
pub fn window_size(self, size: impl Into<Size>) -> Self
pub fn transparent(self, transparent: bool) -> Self
pub fn resizable(self, resizable: bool) -> Self
pub fn decorations(self, decorations: bool) -> Self
pub fn position(self, position: Position) -> Self
pub fn level(self, level: Level) -> Self
pub fn title(self, title: impl TitleFn<P::State>) -> Application<impl Program<State = P::State, Message = P::Message, Theme = P::Theme>>
pub fn subscription(self, f: impl Fn(&P::State) -> Subscription<P::Message>) -> Application<impl Program<...>>
pub fn theme(self, f: impl ThemeFn<P::State, P::Theme>) -> Application<impl Program<...>>
pub fn style(self, f: impl Fn(&P::State, &P::Theme) -> Style) -> Application<impl Program<...>>
pub fn scale_factor(self, f: impl Fn(&P::State) -> f32) -> Application<impl Program<...>>
pub fn executor<E>(self) -> Application<impl Program<...>> where E: Executor,
pub fn presets(self, presets: impl IntoIterator<Item = Preset<P::State, P::Message>>) -> Self
```
There is **no `.maximized()`, `.visible()`, `.icon()`, `.min_size()`, `.max_size()`, `.on_error()`** — use `.window(window::Settings { .. })`. **UNVERIFIED: none found, so treat as non-existent.**

**Complete minimal `main.rs` (compiles against 0.14.0):**
```rust
use iced::widget::{button, column, container, text};
use iced::{Center, Element, Fill, Theme};

pub fn main() -> iced::Result {
    iced::application(Counter::default, Counter::update, Counter::view)
        .title("Counter")
        .theme(|_state: &Counter| Theme::Dark)
        .centered()
        .run()
}

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }

        iced::Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        container(
            column![
                text(self.value).size(48),
                button("+").on_press(Message::Increment),
                button("-").on_press(Message::Decrement),
            ]
            .spacing(12),
        )
        .padding(32)
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}
```
Multi-window / low-level API (only if needed):
```rust
pub trait Program: Sized {
    type State;
    type Message: Send + 'static;
    type Theme: Base;
    type Renderer: Renderer;
    type Executor: Executor;

    fn name() -> &'static str;
    fn settings(&self) -> Settings;
    fn window(&self) -> Option<Settings>;
    fn boot(&self) -> (Self::State, Task<Self::Message>);
    fn update(&self, state: &mut Self::State, message: Self::Message) -> Task<Self::Message>;
    fn view<'a>(&self, state: &'a Self::State, window: Id) -> Element<'a, Self::Message, Self::Theme, Self::Renderer>;

    fn title(&self, _state: &Self::State, _window: Id) -> String { ... }
    fn subscription(&self, _state: &Self::State) -> Subscription<Self::Message> { ... }
    fn theme(&self, _state: &Self::State, _window: Id) -> Option<Self::Theme> { ... }
    fn style(&self, _state: &Self::State, theme: &Self::Theme) -> Style { ... }
    fn scale_factor(&self, _state: &Self::State, _window: Id) -> f32 { ... }
    fn presets(&self) -> &[Preset<Self::State, Self::Message>] { ... }
}
```
`iced::daemon(...)` + `iced::Daemon` is the multi-window builder.

## 4. Widget API reference (0.14.0)

**Existence map for your list:**

| Asked | 0.14.0 status |
|---|---|
| `container`, `column`, `row`, `stack`, `text`, `button`, `checkbox`, `radio`, `toggler`, `pick_list`, `text_input`, `text_editor`, `slider`, `vertical_slider`, `progress_bar`, `scrollable`, `tooltip`, `mouse_area`, `responsive`, `space`, `pane_grid`, `canvas`, `table`, `grid`, `float`, `pin`, `sensor`, `themer`, `combo_box`, `qr_code` | **EXISTS** |
| `rich_text` | **EXISTS** as `iced::widget::rich_text` fn, `widget::text::Rich` struct, `widget::rich_text!` macro |
| `image` | EXISTS but **feature-gated: `image`** |
| `svg` | EXISTS but **feature-gated: `svg`** |
| `canvas` | EXISTS but **feature-gated: `canvas`** |
| `qr_code` | EXISTS but **feature-gated: `qr_code`** |
| `tabs` | **DOES NOT EXIST.** Alternative: `pane_grid` for split layouts; roll your own tab bar from `row` of `button`s + `Stack`/`match` for the body. |
| `horizontal_rule` / `vertical_rule` | **DO NOT EXIST** as those names. Use **`rule::horizontal(height)`** / **`rule::vertical(width)`**. |
| `modal` | **DOES NOT EXIST** as a widget (there is only an `examples/modal` example). Alternatives: `iced::widget::overlay::*` + `iced::overlay::Element`, or `Stack` + `opaque`, or `float`. |
| `overlay` helpers | Modules exist: `iced::overlay` (`Element`, `menu`), `iced::widget::overlay` (`menu`, `Group`, `Nested`), `advanced::overlay::{from_children, Element, Group, Nested}`. No `modal()` helper. |
| `stack` | Exists (fn + `stack!` macro + `Stack` struct) |

**Top-level constructor functions** (all in `iced::widget`, exact from `widget/src/helpers.rs`):
```rust
pub fn container<'a, Message, Theme, Renderer>(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Container<'a, Message, Theme, Renderer> where Theme: container::Catalog + 'a, Renderer: core::Renderer
pub fn button<'a, Message, Theme, Renderer>(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Button<'a, Message, Theme, Renderer> where Theme: button::Catalog + 'a, Renderer: core::Renderer
pub fn text<'a, Theme, Renderer>(text: impl text::IntoFragment<'a>) -> Text<'a, Theme, Renderer> where Theme: text::Catalog + 'a, Renderer: core::text::Renderer
pub fn rich_text<'a, Link, Message, Theme, Renderer>(spans: impl AsRef<[text::Span<'a, Link, Renderer::Font>]> + 'a) -> text::Rich<'a, Link, Message, Theme, Renderer> where Link: Clone + 'static, Theme: text::Catalog + 'a, Renderer: core::text::Renderer, Renderer::Font: 'a
pub fn span<'a, Link, Font>(text: impl text::IntoFragment<'a>) -> text::Span<'a, Link, Font>
pub fn column<'a, Message, Theme, Renderer>(children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>) -> Column<'a, Message, Theme, Renderer>
pub fn row<'a, Message, Theme, Renderer>(children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>) -> Row<'a, Message, Theme, Renderer>
pub fn stack<'a, Message, Theme, Renderer>(children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>) -> Stack<'a, Message, Theme, Renderer>
pub fn grid<'a, Message, Theme, Renderer>(children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>) -> Grid<'a, Message, Theme, Renderer>
pub fn checkbox<'a, Message, Theme, Renderer>(is_checked: bool) -> Checkbox<'a, Message, Theme, Renderer>
pub fn radio<'a, Message, Theme, Renderer, V>(label: impl Into<String>, value: V, selected: Option<V>, on_click: impl FnOnce(V) -> Message) -> Radio<'a, Message, Theme, Renderer> where Message: Clone, V: Copy + Eq
pub fn toggler<'a, Message, Theme, Renderer>(is_checked: bool) -> Toggler<'a, Message, Theme, Renderer>
pub fn pick_list<'a, T, L, V, Message, Theme, Renderer>(options: L, selected: Option<V>, on_selected: impl Fn(T) -> Message + 'a) -> PickList<...> where T: ToString + PartialEq + Clone + 'a, L: Borrow<[T]> + 'a, V: Borrow<T> + 'a
pub fn text_input<'a, Message, Theme, Renderer>(placeholder: &str, value: &str) -> TextInput<...> where Message: Clone
pub fn text_editor<'a, Message, Theme, Renderer>(content: &'a text_editor::Content<Renderer>) -> TextEditor<'a, core::text::highlighter::PlainText, Message, Theme, Renderer>
pub fn slider<'a, T, Message, Theme>(range: RangeInclusive<T>, value: T, on_change: impl Fn(T) -> Message + 'a) -> Slider<'a, T, Message, Theme>
pub fn vertical_slider<'a, T, Message, Theme>(range: RangeInclusive<T>, value: T, on_change: impl Fn(T) -> Message + 'a) -> VerticalSlider<'a, T, Message, Theme>
pub fn progress_bar<'a, Theme>(range: RangeInclusive<f32>, value: f32) -> ProgressBar<'a, Theme>
pub fn scrollable<'a, Message, Theme, Renderer>(content: impl Into<Element<...>>) -> Scrollable<...>
pub fn tooltip<'a, Message, Theme, Renderer>(content: impl Into<Element<...>>, tooltip: impl Into<Element<...>>, position: tooltip::Position) -> Tooltip<...>
pub fn mouse_area<'a, Message, Theme, Renderer>(widget: impl Into<Element<...>>) -> MouseArea<...>
pub fn responsive<'a, Message, Theme, Renderer>(f: impl Fn(Size) -> Element<'a, Message, Theme, Renderer> + 'a) -> Responsive<...>
pub fn space() -> Space
pub fn image<Handle>(handle: impl Into<Handle>) -> crate::Image<Handle>
pub fn svg<'a, Theme>(handle: impl Into<core::svg::Handle>) -> crate::Svg<'a, Theme>
pub fn canvas<P, Message, Theme, Renderer>(program: P) -> crate::Canvas<P, Message, Theme, Renderer>
pub fn qr_code<'a, Theme>(data: &'a crate::qr_code::Data) -> crate::QRCode<'a, Theme>
pub fn combo_box<'a, T, Message, Theme, Renderer>(state: &'a combo_box::State<T>, placeholder: &str, selection: Option<&T>, on_selected: impl Fn(T) -> Message + 'static) -> ComboBox<...> where T: std::fmt::Display + Clone
pub fn pane_grid<'a, T, Message, Theme, Renderer>(state: &'a pane_grid::State<T>, view: impl Fn(pane_grid::Pane, &'a T, bool) -> pane_grid::Content<...>) -> PaneGrid<...>
pub fn float<'a, Message, Theme, Renderer>(content: impl Into<Element<...>>) -> Float<...>
pub fn pin<'a, Message, Theme, Renderer>(content: impl Into<Element<...>>) -> Pin<...>
pub fn sensor<'a, Message, Theme, Renderer>(content: impl Into<Element<...>>) -> Sensor<'a, (), Message, Theme, Renderer>
pub fn themer<'a, Message, Theme, Renderer>(theme: Option<Theme>, content: impl Into<Element<...>>) -> Themer<...>
pub fn opaque<'a, Message, Theme, Renderer>(content: impl Into<Element<...>>) -> Element<...>
pub fn hover<'a, Message, Theme, Renderer>(base: impl Into<Element<...>>, top: impl Into<Element<...>>) -> Element<...>
pub fn center<'a, ...>() / center_x() / center_y() / top() / bottom() / left() / right() ... -> Container<...>
```
Macros: `column![]`, `row![]`, `stack![]`, `grid![]`, `keyed_column![]`, `text![]`, `rich_text![]`.

### Per-widget builder methods (exact)

**container** (`Container<'a, Message, Theme = Theme, Renderer = Renderer>`)
```rust
pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Container<...>
pub fn id(self, id: impl Into<Id>) -> Self
pub fn padding<P>(self, padding: P) -> Self where P: Into<Padding>
pub fn width(self, width: impl Into<Length>) -> Self
pub fn height(self, height: impl Into<Length>) -> Self
pub fn max_width(self, max_width: impl Into<Pixels>) -> Self
pub fn max_height(self, max_height: impl Into<Pixels>) -> Self
pub fn center_x(self, width: impl Into<Length>) -> Self     // NOT Horizontal!
pub fn center_y(self, height: impl Into<Length>) -> Self    // NOT Vertical!
pub fn center(self, length: impl Into<Length>) -> Self
pub fn align_left(self, width: impl Into<Length>) -> Self
pub fn align_right(self, width: impl Into<Length>) -> Self
pub fn align_top(self, height: impl Into<Length>) -> Self
pub fn align_bottom(self, height: impl Into<Length>) -> Self
pub fn align_x(self, alignment: impl Into<Horizontal>) -> Self
pub fn align_y(self, alignment: impl Into<Vertical>) -> Self
pub fn clip(self, clip: bool) -> Self
pub fn style(self, style: impl Fn(&Theme) -> Style + 'a) -> Self where <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(&Theme) -> Style + 'a>>
pub fn class(self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self
```
Style fns: `container::rounded_box(theme)`, `bordered_box(theme)`, `dark(theme)`, `transparent(theme)`, `primary/secondary/success/warning/danger(theme)`, `background(impl Into<Background>)`, `draw_background`, `layout`.

**button**
```rust
pub fn new(content: impl Into<Element<...>>) -> Button<...>
pub fn width(self, width: impl Into<Length>) -> Self
pub fn height(self, height: impl Into<Length>) -> Self
pub fn padding<P>(self, padding: P) -> Self where P: Into<Padding>
pub fn on_press(self, on_press: Message) -> Self
pub fn on_press_with(self, on_press: impl Fn() -> Message + 'a) -> Self
pub fn on_press_maybe(self, on_press: Option<Message>) -> Self
pub fn clip(self, clip: bool) -> Self
pub fn style(self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self where <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(&Theme, Status) -> Style + 'a>>
pub fn class(self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self
pub enum Status { Active, Hovered, Pressed, Disabled }
pub const DEFAULT_PADDING: Padding
// style fns: primary, secondary, success, warning, danger, text, background, subtle
```

**text / rich_text**
```rust
// iced::widget::text::Text is a TYPE ALIAS for iced::advanced::widget::Text:
pub struct Text<'a, Theme = Theme, Renderer = Renderer> { /* private */ }
pub fn new(fragment: impl IntoFragment<'a>) -> Text<'a, Theme, Renderer>
pub fn size(self, size: impl Into<Pixels>) -> Self
pub fn line_height(self, line_height: impl Into<LineHeight>) -> Self
pub fn font(self, font: impl Into<<Renderer as Renderer>::Font>) -> Self
pub fn font_maybe(self, font: Option<impl Into<...>>) -> Self
pub fn width(self, width: impl Into<Length>) -> Self
pub fn height(self, height: impl Into<Length>) -> Self
pub fn center(self) -> Self
pub fn align_x(self, alignment: impl Into<Alignment>) -> Self
pub fn align_y(self, alignment: impl Into<Vertical>) -> Self
pub fn shaping(self, shaping: Shaping) -> Self      // pub enum Shaping { Basic, Advanced, Auto }
pub fn wrapping(self, wrapping: Wrapping) -> Self   // pub enum Wrapping { None, Word, Glyph }
pub fn style(self, style: impl Fn(&Theme) -> Style + 'a) -> Self
pub fn color(self, color: impl Into<Color>) -> Self
pub fn color_maybe(self, color: Option<impl Into<Color>>) -> Self
pub fn class(self, class: impl Into<<Theme as Catalog>::Class<'a>>) -> Self

// Rich (multi-span)
pub struct Rich<'a, Link, Message, Theme = Theme, Renderer = Renderer> where Link: Clone + 'static
pub fn new() -> Rich<...>
pub fn with_spans(spans: impl AsRef<[Span<'a, Link, <Renderer as Renderer>::Font>]> + 'a) -> Rich<...>
pub fn size / line_height / font / width / height / center / align_x / align_y / wrapping
pub fn on_link_click(self, on_link_click: impl Fn(Link) -> Message + 'a) -> Self
pub fn style(self, style: impl Fn(&Theme) -> Style + 'a) -> Self
pub fn color / color_maybe / class
// Span<'a, Link = (), Font = Font> is a plain public-field struct:
pub struct Span<'a, Link = (), Font = Font> {
    pub text: Cow<'a, str>, pub size: Option<Pixels>, pub line_height: Option<LineHeight>,
    pub font: Option<Font>, pub color: Option<Color>, pub link: Option<Link>,
    pub highlight: Option<Highlight>, pub padding: Padding, pub underline: bool, pub strikethrough: bool,
}
// builder: new, size, line_height, font, font_maybe, color, color_maybe, link, link_maybe,
//          background, background_maybe, border, border_maybe, padding, underline, strikethrough (?)
pub enum LineHeight { Relative(f32), Absolute(Pixels) }
// text style fns: base, default, primary, secondary, success, danger, warning
```

**column / row / grid / stack**
```rust
// Column
pub fn new() -> Self ; pub fn with_capacity(usize) -> Self
pub fn with_children(children: impl IntoIterator<Item = Element<...>>) -> Self
pub fn from_vec(children: Vec<Element<...>>) -> Self
pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self
pub fn width(mut self, width: impl Into<Length>) -> Self
pub fn height(mut self, height: impl Into<Length>) -> Self
pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self
pub fn align_x(mut self, align: impl Into<alignment::Horizontal>) -> Self
pub fn clip(mut self, clip: bool) -> Self
pub fn push(mut self, child: impl Into<Element<...>>) -> Self
pub fn extend(self, children: impl IntoIterator<Item = Element<...>>) -> Self
pub fn wrap(self) -> Wrapping<'a, Message, Theme, Renderer>   // NEW in 0.14
// Wrapping: pub fn horizontal_spacing(impl Into<Pixels>) -> Self ; pub fn align_x(impl Into<alignment::Vertical>) -> Self
// NO Column::align_y, NO Column::max_height in 0.14.0

// Row — same shape, but align_y(impl Into<Vertical>), and Wrapping::vertical_spacing + Wrapping::align_x(Horizontal)
// NO Row::max_width in 0.14.0

// Stack
pub fn new() / with_capacity / with_children / from_vec
pub fn width / height (impl Into<Length>)
pub fn push(child) -> Self ; pub fn push_under(child) -> Self ; pub fn extend(children) -> Self
pub fn clip(clip: bool) -> Self

// Grid
pub fn new() / with_capacity / with_children / from_vec
pub fn spacing(mut self, amount: impl Into<Pixels>) -> Self
pub fn width(mut self, width: impl Into<Pixels>) -> Self
pub fn height(mut self, height: impl Into<Sizing>) -> Self
pub fn columns(mut self, column: usize) -> Self
pub fn fluid(mut self, max_width: impl Into<Pixels>) -> Self
pub fn push(child) -> Self ; pub fn push_maybe(Option<impl Into<Element>>) -> Self ; pub fn extend(children) -> Self
pub fn aspect_ratio(width: impl Into<Pixels>, height: impl Into<Pixels>) -> Sizing
pub enum Sizing { AspectRatio(f32), EvenlyDistribute(Length) }
```

**checkbox / radio / toggler / pick_list**
```rust
// Checkbox
pub fn new(is_checked: bool) -> Self
pub fn label(mut self, label: impl text::IntoFragment<'a>) -> Self
pub fn on_toggle<F>(mut self, f: F) -> Self where F: 'a + Fn(bool) -> Message
pub fn on_toggle_maybe<F>(mut self, f: Option<F>) -> Self where F: Fn(bool) -> Message + 'a
pub fn size(mut self, size: impl Into<Pixels>) -> Self
pub fn width(mut self, width: impl Into<Length>) -> Self
pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self
pub fn text_size / text_line_height / text_shaping / text_wrapping
pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self
pub fn icon(mut self, icon: Icon<Renderer::Font>) -> Self
pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self
pub enum Status { Active { is_checked: bool }, Hovered { is_checked: bool }, Disabled { is_checked: bool } }
// style fns: primary, secondary, success, danger

// Radio
pub fn new<F, V>(label: impl Into<String>, value: V, selected: Option<V>, f: F) -> Self where V: Eq + Copy, F: FnOnce(V) -> Message
// + size, width, spacing, text_size, text_line_height, text_shaping, text_wrapping, font, style, class
// style fn: default

// Toggler
pub fn new(is_toggled: bool) -> Self
// + label(IntoFragment), on_toggle(Fn(bool)->Message), on_toggle_maybe, size, width, text_size,
//   text_line_height, text_alignment, text_shaping, text_wrapping, spacing, font, style, class
// style fn: default

// PickList
pub fn new(options: L, selected: Option<V>, on_select: impl Fn(T) -> Message + 'a) -> Self
pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self
pub fn width(mut self, width: impl Into<Length>) -> Self
pub fn menu_height(mut self, menu_height: impl Into<Length>) -> Self
pub fn text_size / text_line_height / text_shaping / font
pub fn handle(mut self, handle: Handle<Renderer::Font>) -> Self
pub fn on_open(mut self, on_open: Message) -> Self
pub fn on_close(mut self, on_close: Message) -> Self
pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
pub fn menu_style(mut self, style: impl Fn(&Theme) -> menu::Style + 'a) -> Self
pub fn class / menu_class
// style fn: default
```

**text_input / text_editor**
```rust
// TextInput
pub fn new(placeholder: &str, value: &str) -> Self
pub fn id(mut self, id: impl Into<widget::Id>) -> Self
pub fn secure(mut self, is_secure: bool) -> Self
pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self
pub fn on_input_maybe(mut self, on_input: Option<impl Fn(String) -> Message + 'a>) -> Self
pub fn on_submit(mut self, message: Message) -> Self
pub fn on_submit_maybe(mut self, on_submit: Option<Message>) -> Self
pub fn on_paste(mut self, on_paste: impl Fn(String) -> Message + 'a) -> Self
pub fn on_paste_maybe(mut self, on_paste: Option<impl Fn(String) -> Message + 'a>) -> Self
pub fn font(mut self, font: Renderer::Font) -> Self
pub fn icon(mut self, icon: Icon<Renderer::Font>) -> Self
pub fn width(mut self, width: impl Into<Length>) -> Self
pub fn size(mut self, size: impl Into<Pixels>) -> Self
pub fn line_height(mut self, line_height: impl Into<text::LineHeight>) -> Self
pub fn align_x(mut self, alignment: impl Into<alignment::Horizontal>) -> Self
pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self
pub enum Status { Active, Hovered, Focused { is_hovered: bool }, Disabled }
// style fn: default ; pub const DEFAULT_PADDING: Padding
// text_input::State (text editing state) has its OWN copy of State; the editor's State is separate.

// TextEditor
pub fn new(content: &'a Content<Renderer>) -> Self
pub fn id / placeholder / width / height / min_height / max_height
pub fn on_action(mut self, on_edit: impl Fn(Action) -> Message + 'a) -> Self
pub fn font / size / line_height / padding / wrapping
pub fn highlight(self, syntax: &str, theme: iced_highlighter::Theme) -> TextEditor<'a, iced_highlighter::Highlighter, ...>   // feature "highlighter"
pub fn highlight_with<H: text::Highlighter>(self, settings: H::Settings, to_format: fn(&H::Highlight, &Theme) -> highlighter::Format<Renderer::Font>) -> TextEditor<'a, H, ...>
pub fn key_binding(mut self, key_binding: impl Fn(KeyPress) -> Option<Binding<Message>> + 'a) -> Self
pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
pub fn class
// text_editor::State: new(), with_text(&str), perform(Action), move_to(Cursor), cursor(), line_count(),
//   line(usize), lines(), text() -> String, selection() -> Option<String>, line_ending(), is_empty(), is_focused()
```

**slider / vertical_slider / progress_bar**
```rust
// Slider (note: NO `length`; `height` takes Pixels)
pub fn new<F>(range: RangeInclusive<T>, value: T, on_change: F) -> Self where F: 'a + Fn(T) -> Message
pub fn default(mut self, default: impl Into<T>) -> Self
pub fn on_release(mut self, on_release: Message) -> Self
pub fn width(mut self, width: impl Into<Length>) -> Self
pub fn height(mut self, height: impl Into<Pixels>) -> Self        // Pixels, not Length
pub fn step(mut self, step: impl Into<T>) -> Self
pub fn shift_step(mut self, shift_step: impl Into<T>) -> Self
pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
pub fn class ; pub fn with_circular_handle(radius: impl Into<Pixels>) -> Self
// VerticalSlider: width(impl Into<Pixels>), height(impl Into<Length>), step(T) — note the swap
// vertical_slider::State has no `.with_circular_handle`? (UNVERIFIED — not present in source scan)

// ProgressBar
pub fn new(range: RangeInclusive<f32>, value: f32) -> Self
pub fn length(mut self, length: impl Into<Length>) -> Self
pub fn girth(mut self, girth: impl Into<Length>) -> Self
pub fn vertical(mut self) -> Self
pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self     // no Status
pub fn class
// style fns: primary, secondary, success, warning, danger
```

**scrollable**
```rust
pub fn new(content: impl Into<Element<...>>) -> Self
pub fn with_direction(content: impl Into<Element<...>>, direction: impl Into<Direction>) -> Self
pub fn horizontal(self) -> Self
pub fn direction(mut self, direction: impl Into<Direction>) -> Self
pub fn id(mut self, id: impl Into<widget::Id>) -> Self
pub fn width(mut self, width: impl Into<Length>) -> Self
pub fn height(mut self, height: impl Into<Length>) -> Self
pub fn on_scroll(mut self, f: impl Fn(Viewport) -> Message + 'a) -> Self
pub fn anchor_top(self) -> Self ; pub fn anchor_bottom(self) -> Self
pub fn anchor_left(self) -> Self ; pub fn anchor_right(self) -> Self
pub fn anchor_x(mut self, alignment: Anchor) -> Self ; pub fn anchor_y(mut self, alignment: Anchor) -> Self
pub fn spacing(mut self, new_spacing: impl Into<Pixels>) -> Self
pub fn auto_scroll(mut self, auto_scroll: bool) -> Self
pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
pub fn class
// enum Direction { Vertical(Scrollbar), Horizontal(Scrollbar), Both { vertical: Scrollbar, horizontal: Scrollbar } }
// enum Anchor { Start, End }
// enum Status { Active { is_horizontal_scrollbar_disabled: bool, is_vertical_scrollbar_disabled: bool },
//               Hovered { is_horizontal_scrollbar_hovered: bool, is_vertical_scrollbar_hovered: bool, + the two disabled },
//               Dragged { is_horizontal_scrollbar_dragged: bool, is_vertical_scrollbar_dragged: bool, + the two disabled } }
// struct Scrollbar { } : new(), hidden(), width(impl Into<Pixels>), margin(impl Into<Pixels>),
//                     scroller_width(impl Into<Pixels>), anchor(Anchor), spacing(impl Into<Pixels>)
// struct Viewport: absolute_offset(), absolute_offset_reversed(), relative_offset(), bounds(), content_bounds(),
//                  y_scroller_grabbed_at(), x_scroller_grabbed_at(), is_mouse_over(Point),
//                  scroll_percentage_y(f32, Point), scroll_percentage_x(f32, Point)
// style fn: default
```

**tooltip / space / rule**
```rust
// Tooltip
pub fn new(content: impl Into<Element<...>>, tooltip: impl Into<Element<...>>, position: Position) -> Self
pub fn gap(mut self, gap: impl Into<Pixels>) -> Self
pub fn padding(mut self, padding: impl Into<Pixels>) -> Self
pub fn delay(mut self, delay: Duration) -> Self
pub fn snap_within_viewport(mut self, snap: bool) -> Self
pub fn style(mut self, style: impl Fn(&Theme) -> container::Style + 'a) -> Self
pub fn class
pub enum Position { Top, Bottom, Left, Right, FollowCursor }

// Space
pub fn new() -> Self ; pub fn horizontal() -> Space ; pub fn vertical() -> Space
pub fn width(mut self, width: impl Into<Length>) -> Self
pub fn height(mut self, height: impl Into<Length>) -> Self

// Rule  (there are NO horizontal_rule/vertical_rule free fns)
pub fn horizontal<'a, Theme>(height: impl Into<Pixels>) -> Rule<'a, Theme> where Theme: Catalog
pub fn vertical<'a, Theme>(width: impl Into<Pixels>) -> Rule<'a, Theme> where Theme: Catalog
pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
pub fn class
pub enum FillMode { Full, Padded, Asymmetric { offset: f32 } }   // UNVERIFIED variant payloads
// style fns: default, weak
```

**image / svg / qr_code**
```rust
// Image  (feature "image")
pub fn new(handle: impl Into<Handle>) -> Self
pub fn width(mut self, width: impl Into<Length>) -> Self
pub fn height(mut self, height: impl Into<Length>) -> Self
pub fn expand(mut self, expand: bool) -> Self
pub fn content_fit(mut self, content_fit: ContentFit) -> Self
pub fn filter_method(mut self, filter_method: FilterMethod) -> Self
pub fn rotation(mut self, rotation: impl Into<Rotation>) -> Self
pub fn opacity(mut self, opacity: impl Into<f32>) -> Self
pub fn scale(mut self, scale: impl Into<f32>) -> Self
pub fn crop(mut self, region: Rectangle<u32>) -> Self
pub fn border_radius(mut self, border_radius: impl Into<border::Radius>) -> Self
pub fn viewer<Handle>(handle: Handle) -> Viewer<Handle>   // free fn: widget::image::viewer

// Svg  (feature "svg")
pub fn new(handle: impl Into<Handle>) -> Self
pub fn from_path(path: impl Into<PathBuf>) -> Self
pub fn width / height (impl Into<Length>)
pub fn content_fit(self, content_fit: ContentFit) -> Self
pub fn rotation(mut self, rotation: impl Into<Rotation>) -> Self
pub fn opacity(mut self, opacity: impl Into<f32>) -> Self
pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
pub fn class
// NOTE: NO `.symbolic(bool)` method in 0.14.0 — svg::Style { color: Option<Color> } does the recolouring.

// QRCode  (feature "qr_code")
pub fn new(data: &'a Data) -> Self
pub fn cell_size(mut self, cell_size: impl Into<Pixels>) -> Self
pub fn total_size(mut self, total_size: impl Into<Pixels>) -> Self
pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
pub fn class
// qr_code::Data: new(data: impl AsRef<[u8]>) -> Result<Self, Error>
//                 with_error_correction(data, ErrorCorrection) -> Result<Self, Error>
//                 with_version(data, Version, ErrorCorrection) -> Result<Self, Error>
```

**mouse_area / responsive / float / stack overlay**
```rust
// MouseArea
pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self
pub fn on_press(mut self, message: Message) -> Self
pub fn on_release(mut self, message: Message) -> Self
pub fn on_double_click(mut self, message: Message) -> Self
pub fn on_right_press(mut self, message: Message) -> Self
pub fn on_right_release(mut self, message: Message) -> Self
pub fn on_middle_press(mut self, message: Message) -> Self
pub fn on_middle_release(mut self, message: Message) -> Self
pub fn on_scroll(mut self, on_scroll: impl Fn(mouse::ScrollDelta) -> Message + 'a) -> Self
pub fn on_enter(mut self, message: Message) -> Self
pub fn on_move(mut self, on_move: impl Fn(Point) -> Message + 'a) -> Self
pub fn on_exit(mut self, message: Message) -> Self
pub fn interaction(mut self, interaction: mouse::Interaction) -> Self

// Responsive
pub fn new(view: impl Fn(Size) -> Element<'a, Message, Theme, Renderer> + 'a) -> Self
pub fn width(mut self, width: impl Into<Length>) -> Self
pub fn height(mut self, height: impl Into<Length>) -> Self

// Float
pub fn new(content: impl Into<Element<...>>) -> Self
pub fn scale(mut self, scale: f32) -> Self
pub fn translate(mut self, translate: impl Fn(Rectangle, Rectangle) -> Vector + 'a) -> Self
pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
pub fn class
```

**pane_grid / table / combo_box / canvas**
```rust
// PaneGrid
pub fn new<T>(state: &'a State<T>, view: impl Fn(Pane, &'a T, bool) -> Content<'a, Message, Theme, Renderer>) -> Self
pub fn width / height (impl Into<Length>) ; pub fn spacing(impl Into<Pixels>) ; pub fn min_size(impl Into<Pixels>)
pub fn on_click<F>(mut self, f: F) -> Self where F: 'a + Fn(Pane) -> Message
pub fn on_drag<F>(mut self, f: F) -> Self where F: 'a + Fn(DragEvent) -> Message
pub fn on_resize<F>(mut self, leeway: impl Into<Pixels>, f: F) -> Self where F: 'a + Fn(ResizeEvent) -> Message
pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
pub fn class
// style fn: default ; pub struct Highlight, pub struct Line

// Table
pub fn table<'a, 'b, T, Message, Theme, Renderer>(columns: impl IntoIterator<Item = Column<'a,'b,T,Message,Theme,Renderer>>, rows: impl IntoIterator<Item = T>) -> Table<'a, Message, Theme, Renderer> where T: Clone
pub fn column<'a, 'b, T, E, Message, Theme, Renderer>(header: impl Into<Element<...>>, view: impl Fn(T) -> E + 'b) -> Column<'a,'b,T,Message,Theme,Renderer>
// Table: width(impl Into<Length>), padding/padding_x/padding_y(impl Into<Pixels>),
//        separator/separator_x/separator_y(impl Into<Pixels>), style(fn(&Theme) -> Style), class
// Column: width(impl Into<Length>), align_x(impl Into<Horizontal>), align_y(impl Into<Vertical>)
// style fn: table::default ; pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

// ComboBox
pub fn new(state: &'a State<T>, placeholder: &str, selection: Option<&T>, on_selected: impl Fn(T) -> Message + 'static) -> Self
pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'static) -> Self
pub fn on_option_hovered(mut self, on_option_hovered: impl Fn(T) -> Message + 'static) -> Self
pub fn on_open / on_close
pub fn padding / font / icon / size / line_height / width / menu_height / text_shaping
pub fn input_style(mut self, style: impl Fn(&Theme, text_input::Status) -> text_input::Style + 'a) -> Self
pub fn menu_style(mut self, style: impl Fn(&Theme) -> menu::Style + 'a) -> Self
pub fn input_class / menu_class
// combo_box::State<T>: new(Vec<T>), with_selection(Vec<T>, Option<&T>), options() -> &[T],
//                      push(&mut self, T), into_options() -> Vec<T>

// Canvas
pub fn new(program: P) -> Self
pub fn width(mut self, width: impl Into<Length>) -> Self
pub fn height(mut self, height: impl Into<Length>) -> Self
```
`canvas::Action` enum exists (`widget::canvas::Action`); `Canvas::new` requires `Renderer: geometry::Renderer`.

## 5. Styling and theming (most important section)

**The styling model in 0.14.0 is closure + `Catalog`, NOT `StyleSheet` traits.**
- `container::StyleSheet`, `button::StyleSheet`, `text_input::StyleSheet`, `scrollable::StyleSheet` — **DO NOT EXIST in 0.14.0.** They were removed (already gone by 0.13.x). You cannot `impl button::StyleSheet for MyTheme`.
- Instead each widget module has a **`Catalog` trait** and a **`StyleFn` type alias**, and `Theme` has a blanket `Catalog` impl whose `Class<'a>` *is* `StyleFn<'a, Theme>`:
```rust
// button.rs
pub trait Catalog {
    type Class<'a>;
    fn default<'a>() -> Self::Class<'a>;
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}
impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;
    fn default<'a>() -> Self::Class<'a> { Box::new(primary) }
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style { class(self, status) }
}
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

// container.rs
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```
This is why `.style(|theme: &Theme, status| ...)` just works: the closure is coerced into `Box<dyn Fn...>` via the `where <Theme as Catalog>::Class<'a>: From<Box<dyn Fn(...) -> Style + 'a>>` bound. There is also `impl<Theme> From<Style> for Box<dyn Fn(&Theme) -> Style + '_>`, so `.style(container::rounded_box)` (a bare `fn`) and `.style(some_style_value)` both work.

**Status-bearing `.style` signatures (full list of `StyleFn` shapes):**
```rust
button, checkbox, radio, toggler, pick_list, text_input, text_editor, scrollable, slider, svg
    -> pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;
container, float, pane_grid, progress_bar, qr_code, rule, table
    -> pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;
```
Per-widget style fn helpers: `container::{rounded_box, bordered_box, dark, transparent, primary, secondary, success, warning, danger, background}`; `button::{primary, secondary, success, warning, danger, text, background, subtle}`; `text::{base, default, primary, secondary, success, danger, warning}`; `checkbox::{primary, secondary, success, danger}`; `progress_bar::{primary, secondary, success, warning, danger}`; `rule::{default, weak}`; plus `default` on `radio`, `toggler`, `pick_list`, `text_input`, `text_editor`, `scrollable`, `slider`, `vertical_slider`, `pane_grid`, `table`, `qr_code`.

**`Theme` / `Palette` / `Extended`:**
```rust
pub enum Theme {
    Light, Dark, Dracula, Nord, SolarizedLight, SolarizedDark, GruvboxLight, GruvboxDark,
    CatppuccinLatte, CatppuccinFrappe, CatppuccinMacchiato, CatppuccinMocha,
    TokyoNight, TokyoNightStorm, TokyoNightLight,
    KanagawaWave, KanagawaDragon, KanagawaLotus, Moonfly, Nightfly, Oxocarbon, Ferra,
    Custom(Arc<Custom>),
}   // = 23 variants, including Custom
pub const ALL: &'static [Theme]
pub fn custom(name: impl Into<Cow<'static, str>>, palette: Palette) -> Theme
pub fn custom_with_fn(name: impl Into<Cow<'static, str>>, palette: Palette, generate: impl FnOnce(Palette) -> Extended) -> Theme
pub fn palette(&self) -> Palette
pub fn extended_palette(&self) -> &Extended
impl Base for Theme { fn default(preference: Mode) -> Theme; fn mode(&self) -> Mode; fn base(&self) -> Style; fn palette(&self) -> Option<Palette>; }

pub struct Palette { pub background: Color, pub text: Color, pub primary: Color, pub success: Color, pub warning: Color, pub danger: Color }
// consts: Palette::LIGHT, DARK, DRACULA, NORD, SOLARIZED_LIGHT, SOLARIZED_DARK, GRUVBOX_LIGHT, GRUVBOX_DARK,
//   CATPPUCCIN_*, TOKYO_NIGHT, TOKYO_NIGHT_STORM, TOKYO_NIGHT_LIGHT, KANAGAWA_*, MOONFLY, NIGHTFLY, OXOCARBON, FERRA

pub struct Extended { pub background: Background, pub primary: Primary, pub secondary: Secondary,
                      pub success: Success, pub warning: Warning, pub danger: Danger, pub is_dark: bool }
pub struct Background { pub base, weakest, weaker, weak, neutral, strong, stronger, strongest: Pair }  // all Pair
pub struct Primary/Secondary/Success/Warning/Danger { pub base: Pair, pub weak: Pair, pub strong: Pair }
pub struct Pair { pub color: Color, pub text: Color }
pub fn generate(palette: Palette) -> Extended
```
**The old `pair.weakest.color` / `pair.strongest.color` field set from 0.12 is renamed.** In 0.14.0 the usable field paths are `palette.background.{base,weakest,weaker,weak,neutral,strong,stronger,strongest}.{color,text}` and `palette.{primary,secondary,success,warning,danger}.{base,weak,strong}.{color,text}`. **Never write `palette.background.weak.color` for `Pair::color` expecting a `Color` — it is a `Color`, correct; but do not use `.weakest_color`-style names.**

There is **no `iced::theme::Catalog` trait** in 0.14.0. The `iced::theme` module contains exactly: modules `palette`; structs `Custom`, `Palette`, `Style`; enums `Mode`, `Theme`; traits `Base`; functions `default`. The per-widget `Catalog` traits live in `iced::widget::{button, container, text, …}::Catalog`.

**Style structs (exact fields):**
```rust
pub struct container::Style { pub text_color: Option<Color>, pub background: Option<Background>, pub border: Border, pub shadow: Shadow, pub snap: bool }
pub struct button::Style { pub background: Option<Background>, pub text_color: Color, pub border: Border, pub shadow: Shadow, pub snap: bool }
   // + pub fn with_background(self, background: impl Into<Background>) -> Style
pub struct text::Style { pub color: Option<Color> }   // (iced::widget::text::Style)
pub struct text_input::Style { pub background: Background, pub border: Border, pub icon: Color, pub placeholder: Color, pub value: Color, pub selection: Color }
pub struct checkbox::Style { pub background: Background, pub icon_color: Color, pub border: Border, pub text_color: Option<Color> }
pub struct radio::Style { pub background: Background, pub dot_color: Color, pub border_width: f32, pub border_color: Color, pub text_color: Option<Color> }
pub struct toggler::Style { pub background: Background, pub background_border_width: f32, pub background_border_color: Color,
                            pub foreground: Background, pub foreground_border_width: f32, pub foreground_border_color: Color,
                            pub text_color: Option<Color>, pub border_radius: Option<Radius>, pub padding_ratio: f32 }
pub struct rule::Style { pub color: Color, pub radius: Radius, pub fill_mode: FillMode, pub snap: bool }
pub struct progress_bar::Style { pub background: Background, pub bar: Background, pub border: Border }
pub struct pick_list::Style { pub text_color: Color, pub placeholder_color: Color, pub handle_color: Color, pub background: Background, pub border: Border }
pub struct slider::Style { pub rail: slider::Rail, pub handle: slider::Handle }  // + with_circular_handle(impl Into<Pixels>)
pub struct slider::Rail { pub backgrounds: (Background, Background), pub width: f32, pub border: Border }
pub struct slider::Handle { pub shape: HandleShape, pub background: Background, pub border_width: f32, pub border_color: Color }
pub struct svg::Style { pub color: Option<Color> }
pub struct scrollable::Style { pub container: container::Style, pub vertical_rail: scrollable::Rail,
                               pub horizontal_rail: scrollable::Rail, pub gap: Option<Background>, pub auto_scroll: scrollable::AutoScroll }
pub struct scrollable::Rail { pub background: Option<Background>, pub border: Border, pub scroller: scrollable::Scroller }
pub struct scrollable::Scroller { pub background: Background, pub border: Border }
pub struct scrollable::AutoScroll { pub background: Background, pub border: Border, pub shadow: Shadow, pub icon: Color }
```
Note `container::Style` and `button::Style` both have a `snap: bool` field — `..Default::default()` is the safe way to fill them.

**Colour with alpha, border, radius, shadow (exact constructors):**
```rust
pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }   // sRGB, 0.0..=1.0
pub const BLACK / WHITE / TRANSPARENT: Color
pub const fn from_rgb(r: f32, g: f32, b: f32) -> Color
pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Color
pub const fn from_rgb8(r: u8, g: u8, b: u8) -> Color
pub const fn from_rgba8(r: u8, g: u8, b: u8, a: f32) -> Color   // NOTE: a is f32, not u8
pub fn from_linear_rgba(r: f32, g: f32, b: f32, a: f32) -> Self
pub fn into_rgba8(self) -> [u8; 4] / into_linear() -> [f32; 4] / scale_alpha(f32) -> Color
impl From<[f32; 3]> / From<[f32; 4]> for Color
impl std::str::FromStr for Color   // "#rrggbb", "rrggbb", "#rgba", 3/4/6/8 hex digits
// There is NO `Color::with_alpha` — use `Color { a: 0.5, ..base }` or `Color::from_rgba(..)`, or `.scale_alpha(f)`.

// color! macro (verbatim arms)
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => { ... };
    ($r:expr, $g:expr, $b:expr, $a:expr) => { ... };
    ($hex:literal) => { ... };
    ($hex:literal, $a:expr) => { ... };
}
// color!(0xffffff), color!(0xffffff, 0.5), color!(0, 0, 0), color!(0, 0, 0, 0.0)
// NOTE: hex shorthand forms were REMOVED in 0.14.0 (changelog "Removed: Short-hand notation support for color! macro")

pub struct Border { pub color: Color, pub width: f32, pub radius: Radius }
pub fn rounded(radius: impl Into<Radius>) -> Border    // free fn `iced::border::rounded`, also `Border::rounded`
pub fn color(color: impl Into<Color>) -> Border        // free fns `iced::border::{color,width,top,bottom,left,right,top_left,top_right,bottom_left,bottom_right,radius}`
pub fn width(width: impl Into<Pixels>) -> Border
pub fn rounded(self, radius) / color(self, color) / width(self, width) -> Self

pub struct Radius { pub top_left: Pixels, pub top_right: Pixels, pub bottom_right: Pixels, pub bottom_left: Pixels }
pub fn radius(value: impl Into<Pixels>) -> Radius ; pub fn top_left / top_right / bottom_right / bottom_left / top / bottom / left / right
pub fn new(value: impl Into<Pixels>) -> Self ; + per-corner setters

pub struct Shadow { pub color: Color, pub offset: Vector, pub blur_radius: f32 }   // all pub fields, derives Default
pub struct Vector { pub x: f32, pub y: f32 }
pub enum Background { Color(Color), Gradient(Gradient) }
impl From<Color> for Background   (so `background: color.into()` or just pass a Color where Into<Background> is expected)
```

**Complete custom-styled container (0.14.0, verified field names):**
```rust
use iced::widget::container;
use iced::{
    border, theme, Background, Border, Color, Element, Padding, Shadow, Theme, Vector,
};

/// Every "card"/"panel"-shaped component in the library is a variation of this.
pub fn card<'a, Message: 'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(Padding::new(16.0))
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();

            container::Style {
                text_color: Some(palette.background.base.text),
                background: Some(Background::Color(
                    Color::from_rgba(0.10, 0.11, 0.13, 0.85), // colour WITH alpha
                )),
                border: Border {
                    color: palette.background.strong.color,
                    width: 1.0,
                    radius: border::radius(10.0),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.35),
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 18.0,
                },
                ..container::Style::default()   // covers the `snap: bool` field
            }
        })
        .into()
}
```
Variants that avoid the struct literal entirely (useful for the 79 components):
```rust
container(content).style(container::rounded_box)                       // fn item
container(content).style(container::bordered_box)
container(content).style(container::background(color!(0x1e1e2e)))
container(content).style(|theme: &Theme| container::Style {
    background: Some(theme.extended_palette().background.weak.color.into()),
    ..container::Style::default()
})
// or the builder methods on Style:
container::Style::default()
    .color(color!(0xffffff))
    .background(color!(0x1e1e2e, 0.9))
    .border(border::rounded(8.0))
    .shadow(Shadow { color: Color::BLACK, offset: Vector::new(0.0, 2.0), blur_radius: 6.0 })
```

**Complete custom-styled button (0.14.0):**
```rust
use iced::widget::button;
use iced::{border, Border, Color, Element, Shadow, Theme, Vector};

pub fn primary_action<'a, Message: Clone + 'a>(
    label: &'a str,
    on_press: Message,
) -> Element<'a, Message> {
    button(iced::widget::text(label))
        .on_press(on_press)
        .padding([10, 18])
        .style(|theme: &Theme, status: button::Status| {
            let palette = theme.extended_palette();

            match status {
                button::Status::Active => button::Style {
                    background: Some(palette.primary.base.color.into()),
                    text_color: palette.primary.base.text,
                    border: Border { radius: border::radius(8.0), ..Border::default() },
                    shadow: Shadow { color: Color::from_rgba(0.0, 0.0, 0.0, 0.25), offset: Vector::new(0.0, 1.0), blur_radius: 4.0 },
                    ..button::Style::default()   // note `snap`
                },
                button::Status::Hovered => button::Style {
                    background: Some(palette.primary.strong.color.into()),
                    text_color: palette.primary.strong.text,
                    border: Border { radius: border::radius(8.0), ..Border::default() },
                    ..button::Style::default()
                },
                button::Status::Pressed => button::Style {
                    background: Some(palette.primary.weak.color.into()),
                    text_color: palette.primary.weak.text,
                    border: Border { radius: border::radius(8.0), ..Border::default() },
                    ..button::Style::default()
                },
                button::Status::Disabled => button::Style {
                    background: Some(palette.background.strong.color.into()),
                    text_color: palette.background.strong.text,
                    border: Border { radius: border::radius(8.0), ..Border::default() },
                    ..button::Style::default()
                },
            }
        })
        .into()
}
```
`button::Style` has no `..Default::default()` trap apart from `snap`, which `Default` covers. To reuse the built-ins inside a closure:
```rust
.style(|theme: &Theme, status| match status {
    button::Status::Active => button::Style { background: Some(color!(0x16a34a).into()), text_color: Color::WHITE, ..button::primary(theme, status) },
    _ => button::primary(theme, status),
})
```
**Programmatic theme:** `.theme(|_state| Theme::Dark)` for a fixed theme, `.theme(|_state| None)` / `.theme(|state| Some(if state.dark { Theme::Dark } else { Theme::Light }))` for dynamic, `.theme(Theme::Light)` also works (`impl<State> ThemeFn<State, Theme> for Theme`). Custom theme:
```rust
const MY: Theme = Theme::custom("Brand", iced::theme::Palette {
    background: color!(0x0f1115), text: color!(0xe6e6e6), primary: color!(0x4f8cff),
    success: color!(0x22c55e), warning: color!(0xf59e0b), danger: color!(0xef4444),
});
// `Theme::custom` is a `fn`, not `const fn` — bind with `.theme(|_| Theme::custom(...))` or a `LazyLock`.
```

## 6. State and events

**Message enum + Task.** `update` may return `()` or `iced::Task<Message>`.
```rust
pub struct Task<T> { /* private */ }
pub fn none() -> Task<T>
pub fn done(value: T) -> Task<T> where T: MaybeSend + 'static
pub fn perform<A>(future: impl Future<Output = A> + MaybeSend + 'static, f: impl FnOnce(A) -> T + MaybeSend + 'static) -> Task<T> where T: MaybeSend + 'static, A: MaybeSend + 'static
pub fn run<A>(stream: impl Stream<Item = A> + MaybeSend + 'static, f: impl Fn(A) -> T + MaybeSend + 'static) -> Task<T> where T: 'static
pub fn future(future: impl Future<Output = T> + MaybeSend + 'static) -> Task<T> where T: 'static
pub fn stream(stream: impl Stream<Item = T> + MaybeSend + 'static) -> Task<T> where T: 'static
pub fn batch(tasks: impl IntoIterator<Item = Task<T>>) -> Task<T> where T: 'static
pub fn map<O>(self, f: impl FnMut(T) -> O + MaybeSend + 'static) -> Task<O> where T: MaybeSend + 'static, O: MaybeSend + 'static
pub fn then<O>(self, f: impl FnMut(T) -> Task<O> + MaybeSend + 'static) -> Task<O>
pub fn chain(self, task: Task<T>) -> Task<T> where T: 'static
pub fn collect(self) -> Task<Vec<T>> ; pub fn discard<O>(self) -> Task<O>
pub fn abortable(self) -> (Task<T>, task::Handle) where T: 'static
pub fn units(&self) -> usize
impl<T> Task<Option<T>> { pub fn and_then<A>(self, f: impl Fn(T) -> Task<A> + MaybeSend + 'static) -> Task<A> }
impl<T, E> Task<Result<T, E>> { pub fn and_then<A>(...); pub fn map_err<E2>(...) }
// task::Handle is `iced::task::Handle`; `Task::sip` and `task::sipper`/`task::stream` need feature "sipper".
```
`iced::Command` does **not** exist. `Command::perform` → `Task::perform`; `Command::batch` → `Task::batch`; `Command::none()` → `Task::none()`.

**Subscriptions:**
```rust
pub struct Subscription<T> { /* private */ }
pub fn none() -> Subscription<T>
pub fn run<S>(builder: fn() -> S) -> Subscription<T>          // fn pointer, NOT a closure
pub fn run_with<D, S>(data: D, builder: fn(&D) -> S) -> Subscription<T>
pub fn batch(subscriptions: impl IntoIterator<Item = Subscription<T>>) -> Subscription<T>
pub fn with<A>(self, value: A) -> Subscription<(A, T)>
pub fn map<F, A>(self, f: F) -> Subscription<A>
pub fn filter_map<F, A>(self, f: F) -> Subscription<A>
pub fn units(&self) -> usize
// NOTE: there is no `Subscription::and`; use `Subscription::batch`.
```

**Passive event listening (exact signatures):**
```rust
// iced::event
pub fn listen() -> Subscription<Event>
pub fn listen_with<Message>(f: fn(Event, event::Status, window::Id) -> Option<Message>) -> Subscription<Message>
    where Message: 'static + MaybeSend
pub fn listen_raw<Message>(f: fn(Event, event::Status, window::Id) -> Option<Message>) -> Subscription<Message>
    where Message: 'static + MaybeSend
// `f` is a FN POINTER (`fn`), not `impl Fn` — a capturing closure will not compile.
pub enum event::Status { Ignored, Captured }   // + pub fn merge(self, b: Self) -> Self

// iced::keyboard
pub fn listen() -> Subscription<keyboard::Event>
// `iced::keyboard::on_key_press` / `on_key_release` DO NOT EXIST in 0.14.0.
// Changelog: "Unified `keyboard` subscriptions into a single `listen` subscription." (#3135)
// Filter manually with event::listen_with, or map keyboard::Event.

// iced::mouse  — has only TYPES in 0.14.0; NO `mouse::on_press` / `on_move` / `on_release` / `on_scroll` fns.
// Use `iced::widget::mouse_area(...)` for pointer interactions on a widget, or event::listen_with.
pub enum mouse::Event {
    CursorEntered, CursorLeft,
    CursorMoved { position: Point },
    ButtonPressed(mouse::Button), ButtonReleased(mouse::Button),
    WheelScrolled { delta: mouse::ScrollDelta },
}
pub enum mouse::Button { Left, Right, Middle, Back, Forward, Other(u16) }
pub enum mouse::Interaction { None, Hidden, Idle, ContextMenu, Help, Pointer, Progress, Wait, Cell, Crosshair,
    Text, Alias, Copy, Move, NoDrop, NotAllowed, Grab, Grabbing, ResizingHorizontally, ResizingVertically,
    ResizingDiagonallyUp, ResizingDiagonallyDown, ResizingColumn, ResizingRow, AllScroll, ZoomIn, ZoomOut }  // 27 variants
pub enum mouse::Cursor { Available(Point), Unavailable }   // UNVERIFIED exact payload — check before use

// iced::keyboard types
pub enum keyboard::Event {
    KeyPressed { key: Key, modified_key: Key, physical_key: Physical, location: Location, modifiers: Modifiers, text: Option<SmolStr>, repeat: bool },
    KeyReleased { key: Key, modified_key: Key, physical_key: Physical, location: Location, modifiers: Modifiers },
    ModifiersChanged(Modifiers),
}
pub enum keyboard::Key<C = SmolStr> { Named(key::Named), Character(C), Unidentified }

// iced::touch — types only (Event, Finger); no on_press/on_move helpers.

// iced::time
pub fn every(duration: Duration) -> Subscription<Instant>            // feature: smol | tokio | wasm32
pub fn repeat<F, T>(f: fn() -> F, interval: Duration) -> Subscription<T> where F: Future<Output = T> + MaybeSend + 'static
pub fn now() -> Task<Instant>
pub fn seconds/minutes/hours/days/milliseconds(u64) -> Duration     // UNVERIFIED exact arg type (u64)
```

**`mouse_area` event API:** see §4 — `on_press`, `on_release`, `on_double_click`, `on_right_press`, `on_right_release`, `on_middle_press`, `on_middle_release`, `on_scroll(impl Fn(mouse::ScrollDelta) -> Message + 'a)`, `on_enter`, `on_move(impl Fn(Point) -> Message + 'a)`, `on_exit`, `interaction(mouse::Interaction)`. **There is no `on_hover`/`on_hover_maybe`** — use `on_enter`/`on_exit`, or read `button::Status::Hovered` in a style closure.

**Window API (`iced::window`, all `Task`-returning unless noted):**
```rust
pub fn events() -> Subscription<(Id, Event)>
pub fn open_events() -> Subscription<Id> ; pub fn close_events() -> Subscription<Id>
pub fn resize_events() -> Subscription<(Id, Size)>
pub fn close_requests() -> Subscription<Id>
pub fn frames() -> Subscription<Instant>
pub fn open(settings: Settings) -> (Id, Task<Id>) ; pub fn close<T>(id: Id) -> Task<T>
pub fn oldest() -> Task<Option<Id>> ; pub fn latest() -> Task<Option<Id>>
pub fn run<T>(...) -> Task<T>       // replaces window::run_with_handle (0.14 change)
pub fn drag<T>(id: Id) -> Task<T> ; pub fn drag_resize<T>(id: Id, direction: Direction) -> Task<T>
pub fn resize<T>(id: Id, new_size: Size) -> Task<T>
pub fn set_resizable<T>(id: Id, resizable: bool) -> Task<T>
pub fn set_max_size<T>(id: Id, size: Option<Size>) -> Task<T>
pub fn set_min_size<T>(id: Id, size: Option<Size>) -> Task<T>
pub fn set_resize_increments<T>(id: Id, increments: Option<Size>) -> Task<T>
pub fn size(id: Id) -> Task<Size> ; pub fn scale_factor(id: Id) -> Task<f32> ; pub fn position(id: Id) -> Task<Option<Point>>
pub fn is_maximized(id: Id) -> Task<bool> ; pub fn maximize<T>(id: Id, maximized: bool) -> Task<T>
pub fn is_minimized(id: Id) -> Task<Option<bool>> ; pub fn minimize<T>(id: Id, minimized: bool) -> Task<T>
pub fn toggle_maximize<T>(id: Id) -> Task<T> ; pub fn toggle_decorations<T>(id: Id) -> Task<T>
pub fn move_to<T>(id: Id, position: Point) -> Task<T> ; pub fn set_mode<T>(id: Id, mode: Mode) -> Task<T> ; pub fn mode(id: Id) -> Task<Mode>
pub fn gain_focus<T>(id: Id) -> Task<T> ; pub fn set_level<T>(id: Id, level: Level) -> Task<T>
pub fn set_icon<T>(id: Id, icon: Icon) -> Task<T> ; pub fn screenshot(id: Id) -> Task<Screenshot>
pub fn request_user_attention<T>(...) ; pub fn show_system_menu<T>(id: Id) -> Task<T>
pub fn enable_mouse_passthrough<Message>(id: Id) -> Task<Message>
pub fn disable_mouse_passthrough<Message>(id: Id) -> Task<Message>
pub fn allow_automatic_tabbing<T>(enabled: bool) -> Task<T>
pub fn monitor_size(id: Id) -> Task<Option<Size>> ; pub fn raw_id<Message>(id: Id) -> Task<u64>
pub fn icon::from_file / from_file_data / from_rgba
pub enum Event {
    Opened { position: Option<Point>, size: Size }, Closed, Moved(Point), Resized(Size), Rescaled(f32),
    RedrawRequested(Instant), CloseRequested, Focused, Unfocused,
    FileHovered(PathBuf), FileDropped(PathBuf), FilesHoveredLeft,
}
```
Typical window subscription:
```rust
fn subscription(_state: &State) -> Subscription<Message> {
    Subscription::batch([
        iced::window::resize_events().map(|(_id, size)| Message::Resized(size)),
        iced::keyboard::listen().map(Message::Keyboard),
    ])
}
```

**Widget operations (`iced::widget::operation`, feature-independent):** `focus`, `focus_next`, `focus_previous`, `is_focused`, `scroll_by`, `scroll_to`, `snap_to`, `snap_to_end`, `select_all`, `select_range`, `move_cursor_to`, `move_cursor_to_front`, `move_cursor_to_end`. Selector API (`iced::widget::selector::{find, find_all, id, is_focused}`) needs feature **`selector`**. `iced::widget::Id`: `Id::new(impl Into<Cow<'static, str>>)` and `Id::unique()`; `From<&str>`/`From<String>` exist.

## 7. Custom widget implementation

**Exact `Widget` trait (verbatim from `core/src/widget.rs` @ 0.14.0):**
```rust
pub trait Widget<Message, Theme, Renderer>
where
    Renderer: crate::Renderer,
{
    fn size(&self) -> Size<Length>;

    fn size_hint(&self) -> Size<Length> { self.size() }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) -> layout::Node;

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    );

    fn tag(&self) -> tree::Tag { tree::Tag::stateless() }
    fn state(&self) -> tree::State { tree::State::None }
    fn children(&self) -> Vec<Tree> { Vec::new() }
    fn diff(&self, tree: &mut Tree) { tree.children.clear(); }

    fn operate(&mut self, _tree: &mut Tree, _layout: Layout<'_>, _renderer: &Renderer, _operation: &mut dyn Operation) {}

    fn update(
        &mut self,
        _tree: &mut Tree,
        _event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {}

    fn mouse_interaction(
        &self, _tree: &Tree, _layout: Layout<'_>, _cursor: mouse::Cursor,
        _viewport: &Rectangle, _renderer: &Renderer,
    ) -> mouse::Interaction { mouse::Interaction::None }

    fn overlay<'a>(
        &'a mut self, _tree: &'a mut Tree, _layout: Layout<'a>, _renderer: &Renderer,
        _viewport: &Rectangle, _translation: Vector,
    ) -> Option<overlay::Element<'a, Message, Theme, Renderer>> { None }
}
```
Only `size`, `layout`, and `draw` are required. **0.14.0 changed `Widget::update` to take `event: &Event` (by reference) and made `layout`/`operate`/`overlay` take `&mut self`** (changelog: "Changed `Widget::update` to take `Event` by reference" #2781; "Mutable `Widget` Methods" #3038; "Inconsistent naming of `tree` parameter in `Widget` trait" #2950). `operate` takes `&mut dyn Operation` (no `Operation<Message>` type parameter in 0.14.0 — do **not** write `&mut dyn Operation<Message>`).

Imports the official `examples/custom_widget` uses:
```rust
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::{border, mouse, Color, Element, Length, Rectangle, Size};
// iced::advanced::widget::Tree, iced::advanced::widget::tree::{Tag, State}
```
Minimal custom widget:
```rust
struct Circle { radius: f32 }

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Circle
where Renderer: renderer::Renderer {
    fn size(&self) -> Size<Length> { Size { width: Length::Shrink, height: Length::Shrink } }

    fn layout(&mut self, _tree: &mut widget::Tree, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
    }

    fn draw(&self, _tree: &widget::Tree, renderer: &mut Renderer, _theme: &Theme,
            _style: &renderer::Style, layout: Layout<'_>, _cursor: mouse::Cursor, _viewport: &Rectangle) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(self.radius),
                ..renderer::Quad::default()
            },
            Color::BLACK,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Circle> for Element<'a, Message, Theme, Renderer>
where Renderer: renderer::Renderer + 'a, Message: 'a, Theme: 'a {
    fn from(circle: Circle) -> Self { Self::new(circle) }
}
```

**`canvas::Program` (verbatim):**
```rust
pub trait Program<Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Renderer: geometry::Renderer,
{
    type State: Default + 'static;

    fn update(
        &self,
        _state: &mut Self::State,
        _event: &Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Option<Action<Message>> { None }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>>;

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction { mouse::Interaction::default() }
}
// canvas::Action (iced::widget::canvas::Action): publish(Message) / request_redraw() / capture()
// canvas::Frame has fill(&Path, Color), stroke, fill_text, draw_image, translate, rotate, scale, etc.
// canvas::Cache is the type alias for the cached geometry; `Cache::new()`, `cache.draw(renderer, bounds, |frame| ...)`,
// and `draw_with_bounds` (new in 0.14).
// Requires the `canvas` cargo feature. `widget::canvas(...)` needs Renderer: graphics::geometry::Renderer.
```
**`shader::Program` (feature `wgpu`) — for GPU-drawn charts:**
```rust
pub trait Program<Message> {
    type State: Default + 'static;
    type Primitive: Primitive + 'static;
    fn update(&self, _state: &mut Self::State, _event: &shader::Event, _bounds: Rectangle, _cursor: mouse::Cursor) -> Option<Action<Message>> { None }
    fn draw(&self, state: &Self::State, cursor: mouse::Cursor, bounds: Rectangle) -> Self::Primitive;
    fn mouse_interaction(&self, _state: &Self::State, _bounds: Rectangle, _cursor: mouse::Cursor) -> mouse::Interaction { mouse::Interaction::default() }
}
```

## 8. Layout primitives

```rust
pub enum Length { Fill, FillPortion(u16), Shrink, Fixed(f32) }
// Convenience re-exports at the crate root: iced::{Fill, FillPortion, Shrink, Center, Left, Right, Top, Bottom}
// NOTE: `Length::Fixed` holds an f32 — `Length::Fixed(300.0)`, NOT `Length::Fixed(Pixels(300.0))`.
// impl From<Pixels> / From<f32> / From<u32> for Length
pub fn fill_factor(&self) -> u16 ; pub fn is_fill(&self) -> bool ; pub fn fluid(&self) -> Self ; pub fn enclose(self, other: Length) -> Self

pub struct Pixels(pub f32);        // impl From<f32>, From<u32> for Pixels; Add/Mul/Div
pub struct Size<T = f32> { pub width: T, pub height: T }   // Size::new(w, h)
pub struct Point<T = f32> { pub x: T, pub y: T }
pub struct Vector { pub x: f32, pub y: f32 }
pub struct Rectangle<T = f32> { pub x: T, pub y: T, pub width: T, pub height: T }
// Rectangle: with_size, with_radius, center, center_x, center_y, position, size, area, contains, distance,
//            offset, is_within, intersection, intersects, union, snap, expand(Padding), shrink(Padding), rotate, zoom, anchor

pub struct Padding { pub top: f32, pub bottom: f32, pub left: f32, pub right: f32 }   // UNVERIFIED whether fields are pub
pub fn all(impl Into<Pixels>) -> Padding ; pub fn top / bottom / left / right / horizontal / vertical(impl Into<Pixels>) -> Padding
pub fn new(impl Into<Pixels>) -> Self   // UNVERIFIED exact name; source shows `all`, `top`, ... and instance `.top(..)`
pub fn x(self) -> f32 ; pub fn y(self) -> f32 ; pub fn fit(self, inner: Size, outer: Size) -> Self
// padding::all / padding::top / padding::bottom / padding::left / padding::right / padding::horizontal / padding::vertical
// Because widget `.padding(..)` takes `impl Into<Padding>`, ALL of these work:
//   .padding(10)  .padding([10, 20])  .padding(Padding::new(10.0))
//   NOTE (corrected in this repository): `.padding([a, b, c, d])` does NOT
//   compile — `Padding` converts from a scalar and from `[vertical, horizontal]`
//   only. For four sides use the struct:
//   .padding(Padding { top: 8.0, right: 16.0, bottom: 8.0, left: 16.0 })

pub enum Alignment { Start, Center, End }
pub enum alignment::Horizontal { Left, Center, Right }
pub enum alignment::Vertical { Top, Center, Bottom }
// re-exported: iced::{Center, Left, Right, Top, Bottom}
// NOTE: `Alignment` is a single-axis enum; widgets take `impl Into<Horizontal>` / `impl Into<Vertical>` for align_x/align_y.

pub struct Border { pub color: Color, pub width: f32, pub radius: Radius }
pub struct Shadow { pub color: Color, pub offset: Vector, pub blur_radius: f32 }
pub enum Background { Color(Color), Gradient(Gradient) }
```
**Sizing a widget to fill its parent:** assign the `Length` explicitly, because container/column/row default to `Shrink`.
```rust
container("x").width(Fill).height(Fill)
column![..].width(Fill).height(Fill)
button("x").width(Fill).height(Length::Fixed(44.0))
image(h).width(Fill).height(Fill).content_fit(ContentFit::Cover)
```
**There are NO `ElementExt`/`Sized` helper methods in 0.14.0.** All of the following are **removed / never existed in 0.14.0**:
```rust
// OLD (0.10–0.12 era tutorials)                → NEW (0.14.0)
element.fill_width()                            → .width(Fill)                // on the concrete widget
element.fill_height()                           → .height(Fill)
element.fill()                                  → .width(Fill).height(Fill)
element.width(Length::Fill)                     // still valid on concrete widgets
element.fixed_width(200)                        → .width(Length::Fixed(200.0))
element.fixed_height(200)                       → .height(Length::Fixed(200.0))
element.shrink_width()                          → .width(Shrink)
// `use iced::widget::Column;` then `Column<Message>` as the return type is still fine.
// `iced::Element` is iced_core's `Element`, and `Element::from(widget)` / `.into()` is the conversion.
```
Layering / overflowing:
```rust
// Layering (replaces the old `iced::widget::Row` stacking hacks and pre-0.14 `overlay` juggling)
stack![background, foreground].width(Fill).height(Fill)
stack![a, b].clip(true)
// Float — translate content outside its layout bounds (tooltips-as-widgets, drag ghosts)
float(content).scale(1.2).translate(|bounds, viewport| Vector::new(10.0, -10.0))
// opaque — stop pointer events passing through in a Stack
stack![base, opaque(menu)]
// hover — show `top` only while `base` is hovered
hover(base, top)
// Modal-ish overlays: `iced::widget::overlay::*` / `iced::overlay::Element` / advanced::overlay::{from_children, Group, Nested}
// `container`/`column`/`row`/`stack`/`scrollable` all expose `.clip(bool)`.
// `opaque(..)` and `hover(..)` return `Element`, not a named widget — do not name their type.
```

## 9. Fonts, icons and assets

```rust
pub struct Font { pub family: Family, pub weight: Weight, pub stretch: Stretch, pub style: Style }  // iced::Font
pub const Font::DEFAULT: Font ; pub const Font::MONOSPACE: Font
pub const fn with_name(name: &'static str) -> Self
// iced::font module: pub use core::font::* (Font, Family, Weight, Stretch, Style)
//                    pub use runtime::font::* (`load`)
pub fn font::load(bytes: impl Into<Cow<'static, [u8]>>) -> Task<Result<(), font::Error>>
```
Registering fonts with the application builder (exact):
```rust
iced::application(boot, update, view)
    .font(include_bytes!("../fonts/Inter-Regular.ttf").as_slice())  // impl Into<Cow<'static, [u8]>>
    .font(include_bytes!("../fonts/Inter-Bold.ttf").as_slice())
    .default_font(Font::with_name("Inter"))
    .run()
// `.font(..)` is additive and repeatable; `.default_font(Font)` sets the default.
// At runtime: `font::load(bytes) -> Task<Result<(), Error>>` — map it into your Message.
```
**`image::Handle` (`iced::advanced::image::Handle` / re-exported under `iced::widget::image`, feature `image`):**
```rust
pub fn from_path<T: Into<PathBuf>>(path: T) -> Handle
pub fn from_bytes(bytes: impl Into<Bytes>) -> Handle           // encoded PNG/JPG/etc.
pub fn from_rgba(width: u32, height: u32, pixels: impl Into<Bytes>) -> Handle   // raw RGBA
pub fn id(&self) -> image::Id
impl<T: Into<PathBuf>> From<T> for Handle                      // so image("path.png") works
```
**`svg::Handle` (`iced::advanced::svg::Handle`, feature `svg`):**
```rust
pub fn from_path(path: impl Into<PathBuf>) -> Handle
pub fn from_memory(bytes: impl Into<Cow<'static, [u8]>>) -> Handle     // for include_bytes! / include_str!
pub fn id(&self) -> u64 ; pub fn data(&self) -> &Data
```
Usage:
```rust
// Raster from disk / embedded
image("assets/logo.png").width(64).height(64).content_fit(ContentFit::Contain)
image(image::Handle::from_bytes(include_bytes!("../assets/logo.png").as_slice()))
image(image::Handle::from_rgba(w, h, rgba_vec))
// SVG from disk / embedded
svg("assets/icon.svg").width(24).height(24)
svg(svg::Handle::from_memory(include_bytes!("../assets/icon.svg").as_slice()))
svg(bytes).style(|theme: &Theme, _status| svg::Style { color: Some(theme.palette().text) })  // recolour
// Zoomable image
image::viewer(image::Handle::from_path("big.jpg"))     // `widget::image::viewer`
```
Icons: iced 0.14.0 ships **no icon set** and no default icon font. The built-in glyphs available without assets are the iced logo (`iced::widget::iced(text_size)`) and the checkbox/toggler/radio/pick_list/scroller glyphs drawn from code. For an icon set you must either register a font (e.g. Lucide/Bootstrap Icons as a TTF, then `text("\u{...}").font(Font::with_name("lucide"))`) or embed SVGs and use `svg(svg::Handle::from_memory(..))`. `iced::widget::image::Handle` and `svg::Handle` are the only asset handles.

## 10. Wasm / web support

- **Supported in 0.14.0, yes.** The winit shell has `#[cfg(target_arch = "wasm32")]` paths using `winit::platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys, WindowExtWebSys}` and `web_sys::HtmlCanvasElement`, and the README lists "[Cross-platform support] (Windows, macOS, Linux, and the Web)". `iced_futures` uses `wasm_bindgen_futures::spawn_local` and a non-`Send` `BoxFuture` on wasm, and `iced::animation` has wasm support.
- **Feature that enables it:** there is **no `web` feature**. You build for `wasm32-unknown-unknown` and enable **`webgl`** (WebGL2) or **`wgpu`** (WebGPU), plus **`fira-sans`** (there is no system font on the web). `iced_futures` on wasm always uses the `wasm_bindgen` executor, so `tokio`/`smol` are unnecessary there, and `time::every` is available on wasm without them.
- **Trunk setup** — the official example is **`examples/tour/`** at tag 0.14.0 (the *only* in-repo web example; `examples/README.md` says the Tour "can run both on native platforms and the web"). Its `index.html` (verbatim):
```html
<!DOCTYPE html>
<html lang="en" style="height: 100%">
<head>
    <meta charset="utf-8" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Tour - Iced</title>
    <base data-trunk-public-url />
</head>
<body style="height: 100%; margin: 0">
<link data-trunk rel="rust" href="Cargo.toml" data-wasm-opt="z" data-bin="tour" />
</body>
</html>
```
  Its `Cargo.toml` gates the web-only features:
```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
iced.workspace = true
iced.features = ["image", "debug", "webgl", "fira-sans"]
console_error_panic_hook = "0.1"
console_log = "1.0"
```
  and `main()` initializes the console logger only on wasm:
```rust
#[cfg(target_arch = "wasm32")]
{
    console_log::init().expect("Initialize logger");
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}
```
  Build/run: `trunk serve` / `trunk build --release` (install with `cargo install trunk && rustup target add wasm32-unknown-unknown`).
- **Known limitations:** no system fonts on web (hence `fira-sans`); `window::*` tasks that manipulate a native window (drag, resize, decorations, mode, icon, system menus, `enable_mouse_passthrough`) are native-only or no-ops; the shell prints/ignores unsupported events. The changelog records real web bugs fixed for 0.14.0 (WebGPU boot failure in Chromium #2686, WebGL binding-alignment crash #2883, panic on a zero-sized canvas #2829) — the WebGL backend is more battle-tested than WebGPU. **UNVERIFIED:** whether file-drop and IME work on wasm.

## 11. Gotcha list — old → new (0.12/0.13-era tutorials vs 0.14.0)

1. **`iced::Application` / `iced::Sandbox` traits** → **removed**. Old code implemented `impl Sandbox for Counter { type Message; fn new(); fn title(); fn update(); fn view(); }` and called `Counter::run(Settings::default())`. New: use the **`iced::application(boot, update, view)` function** builder + `.run()`, or `iced::run(update, view)`. Neither trait exists in 0.14.0 (nor in 0.13.x).
2. **`iced::Command<Message>`** → **`iced::Task<Message>`**. `Command::perform(fut, f)` → `Task::perform(fut, f)`; `Command::batch([..])` → `Task::batch([..])`; `Command::none()` → `Task::none()`. Also, `update` now usually returns `Task<Message>` (returning `()` is still allowed).
3. **`widget::horizontal_rule()` / `vertical_rule()`** → **`widget::rule::horizontal(height)` / `widget::rule::vertical(width)`**, both taking `impl Into<Pixels>` and returning `Rule<'a, Theme>`.
4. **`widget::canvas::Canvas` + `impl canvas::Program`** — still exists, but **needs the `canvas` cargo feature** (default features do NOT include it). Same for **`svg`**, **`image`**, **`qr_code`**, **`markdown`**, **`lazy`**, **`highlighter`**, **`selector`**. Forgetting the feature yields "cannot find function `svg` in module `widget`".
5. **`impl button::StyleSheet for MyTheme { type Style = ...; fn active(&self, style: &Self::Style, ...) }`** → **`StyleSheet` traits are gone.** Use `.style(|theme: &Theme, status| button::Style { .. })`, a `fn(&Theme, Status) -> Style` item, or implement the widget's `Catalog` trait on your theme (`type Class<'a> = button::StyleFn<'a, Self>`). Same for `container::StyleSheet`, `text_input::StyleSheet`, `scrollable::StyleSheet`.
6. **`.fill_width()` / `.fill_height()` / `.width(Length::Fill)` / `.height(Fill)` / `.fixed_width(200)` / `.fixed_height(200)` / `.shrink_width()` on `Element`** → **these `ElementExt`/`Sized` helpers do not exist in 0.14.0**, and `iced::Element` only has `new`, `as_widget`, `as_widget_mut`, `map`, `explain`. Set sizes on the concrete widget: `.width(Fill)`, `.height(Length::Fixed(44.0))`, `.width(Shrink)`. `.fill()`, `.fixed()`, `.shrink()` do **not** exist as `Element` methods.
7. **`Length::Fixed(Pixels(300.0))` / `Length::Units(300)`** → **`Length::Fixed(300.0)`** (payload is `f32`) and **`Pixels`** for all pixel-bearing builders (`impl Into<Pixels>` accepts both `f32` and `u32`). `Length::Units` no longer exists.
8. **`time::every(..)` with default features** → **does not compile unless you enable `tokio` or `smol`** (docs.rs: "Available on WebAssembly or crate feature `smol` or crate feature `tokio` only"). The default `thread-pool` is the *executor*, not a timer. Old tutorials used `iced::time::every` with `iced = "0.12"` defaults, which had `async-std`; add `features = ["tokio"]` (or `["smol"]`) on native.
9. **`iced::keyboard::on_key_press` / `on_key_release` and `iced::mouse::on_press` / `on_move` / `on_release` / `on_scroll`** → **all removed in 0.14.0** ("Unified `keyboard` subscriptions into a single `listen` subscription" #3135). Use `iced::keyboard::listen()`, `iced::event::listen_with(fn(Event, Status, window::Id) -> Option<Message>)`, or `widget::mouse_area(..)` for pointer events. Critical detail: `listen_with`/`listen_raw` take a **`fn` pointer plus a `Status` and a `window::Id`**, so a capturing closure will not compile.
10. **`iced::theme::Catalog`, `Palette::EXTENDED`, `pair.weakest.color`, `0.12` hex shorthand `color!(#fff)`, and `Widget::update(&mut self, .., event: Event, ..)`** → in 0.14.0: the per-widget `Catalog` traits are the styling extension point (there is no `theme::Catalog`); `Extended` is obtained via **`theme.extended_palette()`** and is a struct with `background/primary/secondary/success/warning/danger/is_dark`, addressed as `palette.primary.strong.color` and `palette.background.weak.text` (the 0.12 `weakest`/`strongest` field names are gone); `color!` shorthand notation was **removed** (use `color!(0xffffff)` with a full `0x` literal, or `Color::from_rgb8`); and `Widget::update` now takes **`_event: &Event`** while `layout`/`operate`/`overlay` take **`&mut self`**, and `operate` takes **`&mut dyn Operation`** with no message type parameter.

---

### Files/examples worth pointing the engineer at (official, tag 0.14.0)
- `examples/custom_widget/src/main.rs` — the reference custom `Widget` (exact trait shape used above).
- `examples/tour/` — widget showcase + the only web/Trunk setup (`index.html`, wasm-gated `Cargo.toml`).
- `examples/styling/`, `examples/table/`, `examples/pane_grid/`, `examples/combo_box/`, `examples/qr_code/`, `examples/modal/` (modal is example code, not a widget).
- `docs.rs/iced_widget/0.14.0/iced_widget/index.html` — the per-widget module docs (use `iced_widget` paths, not `iced::widget` paths, for `MouseArea`/`Responsive`/`Stack`/`Column`/`Row`, which are broken on `docs.rs/iced`).

**Two things I could not verify and would flag before codegen:** (a) whether the experimental `hot` / `time-travel` features require nightly; (b) the exact payloads of `rule::FillMode` and `mouse::Cursor`.
