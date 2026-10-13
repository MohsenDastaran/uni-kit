//! A colour field that opens a palette and an HSLA panel. Mirrors GPUI
//! Component's `ColorPicker`/`ColorSelect` and the Slint `color-picker` beside
//! them, and is composed from parts that are already done: the field's frame and
//! text field, the popover's floating surface, the slider, the button and the
//! theme.
//!
//! # What it composes
//!
//! `crates/slint-component/ui/color-picker.slint` is the authority, and it is
//! four pieces:
//!
//! - the trigger is a 32px square, or — with [`ColorPicker::field`] — a framed
//!   field with a swatch, the hex value and a caret, which is the Slint `field`
//!   property and GPUI's `ColorSelect`. The field's frame is
//!   [`input::Style`], the same one `input.rs` draws its own field with, so the
//!   fill and the edge are the field's rather than a second spelling;
//! - the panel is the popover's floating surface: [`Popover::filled(true)`] draws
//!   the popover fill, a one-pixel edge, `radius-lg` and the shadow, and the 12px
//!   inset is the Slint popup's `padding: 12px`. There is no second surface in
//!   this file, which is the rule this family learned twice;
//! - the palette is the Slint file's own: a featured row, a rule, and the nine
//!   Tailwind ramps, each cell a 20px chip whose edge is the fill darkened and
//!   whose selected edge is the foreground. Every token is read inside the style
//!   closure, so the panel follows the appearance in force;
//! - the HSLA tab is four rows: a label, a [`Slider`] and the reading. Slint
//!   draws each track as a gradient; iced's slider is one fill, and this port
//!   does not write a second slider to fake it — see below.
//!
//! The hex field at the foot of the panel is [`Input`] — the same field a form
//! uses — so typing a value and pressing Enter is the field's own behaviour.
//!
//! # The state is the caller's
//!
//! iced 0.14 keeps no state a component can own across frames, so this port has
//! none either. The caller owns:
//!
//! - the value, [`ColorPicker::value`], where `None` is the Slint `empty` state
//!   and shows [`ColorPicker::placeholder`] instead of a hex value;
//! - the hex text, [`ColorPicker::hex`], because a field iced recomputed from the
//!   value on every frame would erase a half-typed `#63` before it became a
//!   colour. It is reported back through [`ColorPicker::on_hex_input`] and
//!   committed with [`ColorPicker::on_hex_submit`]; a caller that only sets
//!   `value` still gets a correct reading, because an empty `hex` falls back to
//!   [`to_hex`];
//! - the open flag, [`ColorPicker::open`], and the active tab,
//!   [`ColorPicker::tab`]. [`on_open_change`](ColorPicker::on_open_change)
//!   reports a trigger click, an outside click or Escape, because the popover
//!   supplies all three, and [`on_tab`](ColorPicker::on_tab) reports the two tab
//!   buttons;
//! - the value a palette cell or a slider asks for, through
//!   [`on_change`](ColorPicker::on_change). The component never writes its own
//!   value, exactly as Slint's `changed` leaves the application to answer.
//!
//! # What the Slint component has that this does not
//!
//! - **No gradient slider tracks.** The Slint `ChannelSlider` fills its track
//!   with the hue rainbow, the saturation ramp, a lightness `from`/`mid`/`to`
//!   and the alpha ramp. iced's [`Slider`] draws a fill and a thumb and exposes
//!   no track style, and writing a second slider would be a third implementation
//!   of drawing a control in this crate. The four rows keep the labels, the
//!   ranges and the readings, so the channel is still told apart and still
//!   moved; only the track's colour is the theme's.
//! - **No live alpha composite.** Slint fills the preview with the value,
//!   alpha and all, over the popup. The port does the same and the panel shows
//!   through a translucent swatch; there is no checkerboard behind it, because
//!   the Slint component has none.
//! - **No HSL/hex conversion for a caller outside the component.** The Slint
//!   file's `hex`, `hue-of`, `sat-of` and `light-of` are private pure functions.
//!   [`to_hex`] and [`parse_hex`] are public here because the field's text is the
//!   caller's to hold, which is a difference iced forces rather than a feature
//!   added for its own sake.
//! - **No i18n.** Slint's labels are its own literals; the port's two tab titles
//!   are the same literals, English, like the rest of this catalog.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::widget::{column, container, mouse_area, row, rule, space, svg, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Theme};

use super::button::{Button, Variant};
use super::input::{self, Input};
use super::popover::{Placement, Popover};
use super::slider::Slider;
use super::theme::{self, ControlSize, Tokens};

/// The popover's inset, the Slint popup's `padding: 12px`.
const PANEL_INSET: f32 = 12.0;

/// The gap between the trigger and the panel, the Slint popup's
/// `y: root.height + 4px`.
const PANEL_GAP: f32 = 4.0;

/// The panel's width, the Slint popup's `width: 292px`. The nine eleven-cell
/// ramps need 260px and the inset the rest.
const PANEL_WIDTH: f32 = 292.0;

/// A palette cell's side, the Slint `PaletteCell`'s `20px`.
const CELL: f32 = 20.0;

/// The colours the featured row falls back to, the Slint `featured` property:
/// the eleven-family header of the Tailwind ramps.
const FEATURED: [Color; 11] = [
    hex(0x000000),
    hex(0xef4444),
    hex(0xf97316),
    hex(0xeab308),
    hex(0x84cc16),
    hex(0x22c55e),
    hex(0x06b6d4),
    hex(0x3b82f6),
    hex(0x6366f1),
    hex(0xa855f7),
    hex(0xec4899),
];

/// The nine family ramps, darkest first, exactly as the Slint component lists
/// them. They are the component's data, not a theme decision: the theme carries
/// semantic tokens, not a 99-swatch palette, and the Slint file hard-codes the
/// same values for the same reason.
const PALETTES: [&[Color]; 9] = [
    &STONE, &REDS, &ORANGES, &YELLOWS, &GREENS, &CYANS, &BLUES, &PURPLES, &PINKS,
];

const STONE: [Color; 11] = [
    hex(0x0c0a09),
    hex(0x1c1917),
    hex(0x292524),
    hex(0x44403c),
    hex(0x57534e),
    hex(0x78716c),
    hex(0xa8a29e),
    hex(0xd6d3d1),
    hex(0xe7e5e4),
    hex(0xf5f5f4),
    hex(0xfafaf9),
];
const REDS: [Color; 11] = [
    hex(0x450a0a),
    hex(0x7f1d1d),
    hex(0x991b1b),
    hex(0xb91c1c),
    hex(0xdc2626),
    hex(0xef4444),
    hex(0xf87171),
    hex(0xfca5a5),
    hex(0xfecaca),
    hex(0xfee2e2),
    hex(0xfef2f2),
];
const ORANGES: [Color; 11] = [
    hex(0x431407),
    hex(0x7c2d12),
    hex(0x9a3412),
    hex(0xc2410c),
    hex(0xea580c),
    hex(0xf97316),
    hex(0xfb923c),
    hex(0xfdba74),
    hex(0xfed7aa),
    hex(0xffedd5),
    hex(0xfff7ed),
];
const YELLOWS: [Color; 11] = [
    hex(0x422006),
    hex(0x713f12),
    hex(0x854d0e),
    hex(0xa16207),
    hex(0xca8a04),
    hex(0xeab308),
    hex(0xfacc15),
    hex(0xfde047),
    hex(0xfef08a),
    hex(0xfef9c3),
    hex(0xfefce8),
];
const GREENS: [Color; 11] = [
    hex(0x052e16),
    hex(0x14532d),
    hex(0x166534),
    hex(0x15803d),
    hex(0x16a34a),
    hex(0x22c55e),
    hex(0x4ade80),
    hex(0x86efac),
    hex(0xbbf7d0),
    hex(0xdcfce7),
    hex(0xf0fdf4),
];
const CYANS: [Color; 11] = [
    hex(0x083344),
    hex(0x164e63),
    hex(0x155e75),
    hex(0x0e7490),
    hex(0x0891b2),
    hex(0x06b6d4),
    hex(0x22d3ee),
    hex(0x67e8f9),
    hex(0xa5f3fc),
    hex(0xcffafe),
    hex(0xecfeff),
];
const BLUES: [Color; 11] = [
    hex(0x172554),
    hex(0x1e3a8a),
    hex(0x1e40af),
    hex(0x1d4ed8),
    hex(0x2563eb),
    hex(0x3b82f6),
    hex(0x60a5fa),
    hex(0x93c5fd),
    hex(0xbfdbfe),
    hex(0xdbeafe),
    hex(0xeff6ff),
];
const PURPLES: [Color; 11] = [
    hex(0x3b0764),
    hex(0x581c87),
    hex(0x6b21a8),
    hex(0x7e22ce),
    hex(0x9333ea),
    hex(0xa855f7),
    hex(0xc084fc),
    hex(0xd8b4fe),
    hex(0xe9d5ff),
    hex(0xf3e8ff),
    hex(0xfaf5ff),
];
const PINKS: [Color; 11] = [
    hex(0x500724),
    hex(0x831843),
    hex(0x9d174d),
    hex(0xbe185d),
    hex(0xdb2777),
    hex(0xec4899),
    hex(0xf472b6),
    hex(0xf9a8d4),
    hex(0xfbcfe8),
    hex(0xfce7f3),
    hex(0xfdf2f8),
];

/// A colour from an `0xRRGGBB` literal, for the palette tables above.
const fn hex(value: u32) -> Color {
    Color::from_rgb8(
        ((value >> 16) & 0xff) as u8,
        ((value >> 8) & 0xff) as u8,
        (value & 0xff) as u8,
    )
}

/// The hex text for a colour, `#RRGGBB` uppercase — the Slint `hex` function.
///
/// Alpha is dropped, as it is in the Slint component: the field's text is the
/// value's RGB and the alpha is the fourth slider's business.
pub fn to_hex(color: Color) -> String {
    let byte = |channel: f32| (channel * 255.0).round().clamp(0.0, 255.0) as u8;
    let (r, g, b) = (byte(color.r), byte(color.g), byte(color.b));
    format!("#{r:02X}{g:02X}{b:02X}")
}

/// The colour a hex string names, or `None` when it names nothing.
///
/// `#` is optional, as it is in every hex field a person pastes into, and the
/// digits may be the six of `#RRGGBB` or the eight of `#RRGGBBAA`. This is the
/// reverse of [`to_hex`], and it is public for the same reason: the field's text
/// is the caller's, so parsing it is the caller's too.
pub fn parse_hex(text: &str) -> Option<Color> {
    let digits = text.trim().strip_prefix('#').unwrap_or(text.trim());
    if !digits.is_ascii() {
        return None;
    }
    let pair = |at: usize| u8::from_str_radix(&digits[at..at + 2], 16).ok();
    match digits.len() {
        6 => Some(Color::from_rgb8(pair(0)?, pair(2)?, pair(4)?)),
        8 => Some(Color::from_rgba8(
            pair(0)?,
            pair(2)?,
            pair(4)?,
            pair(6)? as f32 / 255.0,
        )),
        _ => None,
    }
}

/// The hue of a colour in degrees, `0..360` — the Slint `hue-of`.
fn hue_of(color: Color) -> f32 {
    let (r, g, b) = (color.r, color.g, color.b);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    if d <= 0.0001 {
        return 0.0;
    }
    let raw = if max == r {
        (g - b) / d
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    (if raw < 0.0 { raw + 6.0 } else { raw }) * 60.0
}

/// The lightness of a colour, `0..1` — the Slint `light-of`.
fn light_of(color: Color) -> f32 {
    let max = color.r.max(color.g).max(color.b);
    let min = color.r.min(color.g).min(color.b);
    (max + min) / 2.0
}

/// The saturation of a colour, `0..1` — the Slint `sat-of`.
fn sat_of(color: Color) -> f32 {
    let max = color.r.max(color.g).max(color.b);
    let min = color.r.min(color.g).min(color.b);
    let d = max - min;
    let l = (max + min) / 2.0;
    if d <= 0.0001 {
        0.0
    } else if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    }
}

/// The colour `h` degrees, `s` saturation, `l` lightness and `a` alpha name —
/// the Slint `hsl` function, with the hue in degrees rather than turns.
fn hsl(h: f32, s: f32, l: f32, a: f32) -> Color {
    if s <= 0.001 {
        return Color::from_rgba(l, l, l, a);
    }
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h / 60.0 - (h / 360.0).floor() * 6.0;
    let x = c * (1.0 - (hp - (hp / 2.0).floor() * 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = if hp < 1.0 {
        (c, x, 0.0)
    } else if hp < 2.0 {
        (x, c, 0.0)
    } else if hp < 3.0 {
        (0.0, c, x)
    } else if hp < 4.0 {
        (0.0, x, c)
    } else if hp < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    Color::from_rgba(r + m, g + m, b + m, a)
}

/// A colour field that opens a palette and an HSLA panel, built with the methods
/// below.
///
/// It becomes an [`Element`] through `From`, and what it becomes is a
/// [`Popover`] whose trigger is the square or the framed field and whose panel
/// holds the tabs, the palette, the four channel rows and the hex field — which
/// is what the Slint control is: a swatch with a `PopupWindow`.
pub struct ColorPicker<'a, Message> {
    value: Option<Color>,
    hex: String,
    placeholder: String,
    featured: Option<Vec<Color>>,
    field: bool,
    size: ControlSize,
    tab: usize,
    open: bool,
    disabled: bool,
    width: Option<Length>,
    panel_width: f32,
    id: String,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    /// Shared, because every palette cell and every slider is a caller of it.
    on_change: Option<Rc<dyn Fn(Option<Color>) -> Message + 'a>>,
    on_hex_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_hex_submit: Option<Message>,
    /// Shared for the same reason: both tab buttons report through it.
    on_tab: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message> ColorPicker<'a, Message> {
    /// A compact picker with nothing chosen and the panel closed.
    pub fn new() -> Self {
        Self {
            value: None,
            hex: String::new(),
            placeholder: String::new(),
            featured: None,
            field: false,
            size: ControlSize::Medium,
            tab: 0,
            open: false,
            disabled: false,
            width: None,
            panel_width: PANEL_WIDTH,
            id: String::from("color-picker"),
            on_open_change: None,
            on_change: None,
            on_hex_input: None,
            on_hex_submit: None,
            on_tab: None,
        }
    }

    /// The chosen colour. `None` — the default — is the Slint `empty` state: the
    /// square falls back to the page and the field shows
    /// [`placeholder`](Self::placeholder).
    pub fn value(mut self, value: Option<Color>) -> Self {
        self.value = value;
        self
    }

    /// The text the hex field shows. The caller owns it; see the module comment.
    /// An empty string falls back to [`to_hex`] of the value, so a caller that
    /// only sets `value` still reads the right colour.
    pub fn hex(mut self, hex: impl Into<String>) -> Self {
        self.hex = hex.into();
        self
    }

    /// The text shown while nothing is chosen, "" by default. Slint's
    /// `placeholder`.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// The colours of the featured row, in order. `None` — the default — uses
    /// the Slint component's own eleven, [`FEATURED`]; GPUI's `featured_colors`.
    pub fn featured(mut self, colors: Vec<Color>) -> Self {
        self.featured = Some(colors);
        self
    }

    /// Draw the trigger as a framed field — a swatch, the hex value and a caret,
    /// sharing the height and frame of the inputs around it — rather than a
    /// square. This is the Slint `field` and GPUI's `ColorSelect`.
    pub fn field(mut self, field: bool) -> Self {
        self.field = field;
        self
    }

    /// The trigger's size, [`ControlSize::Medium`] by default. It picks the
    /// height, the inset and the type together, the way the Slint component's
    /// fixed 32px does at the medium step.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Which panel tab is showing: 0 for Palette, 1 for HSLA. The caller owns
    /// it; [`on_tab`](Self::on_tab) reports the two buttons.
    pub fn tab(mut self, tab: usize) -> Self {
        self.tab = tab;
        self
    }

    /// Whether the panel is showing. The caller owns it, and
    /// [`on_open_change`](Self::on_open_change) is how the caller learns it
    /// should change.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// A picker that answers nothing: the trigger draws muted and the popover is
    /// told no open state, so it cannot be opened — GPUI's `disabled`, which the
    /// Slint component spells by having no callback.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The trigger's width. A square trigger is as wide as it is tall by
    /// default, and a field spans its column, the Slint `horizontal-stretch: 1`.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// The panel's fixed width, 292px by default — the Slint popup's `width`.
    pub fn panel_width(mut self, width: f32) -> Self {
        self.panel_width = width;
        self
    }

    /// The base name of the hex field, `"color-picker"` by default. Two pickers
    /// in one application need different names, because iced keeps a text
    /// field's caret and focus under its id and would otherwise share them.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Report every request to change the open state: a trigger click wants
    /// `true`, an outside click or Escape wants `false`. A disabled picker is
    /// given no callback, so its trigger cannot open anything.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// A colour was asked for: a palette cell was pressed, a channel slider was
    /// moved, or the value was cleared. `None` is not reported by this port —
    /// the Slint component has no clear control — so the value keeps its
    /// `Option` for the caller's own empty state.
    pub fn on_change(mut self, on_change: impl Fn(Option<Color>) -> Message + 'a) -> Self {
        self.on_change = Some(Rc::new(on_change));
        self
    }

    /// Every edit of the hex field, as the text it now holds.
    pub fn on_hex_input(mut self, on_hex_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_hex_input = Some(Box::new(on_hex_input));
        self
    }

    /// The message Enter sends from the hex field. The caller already has the
    /// text from [`on_hex_input`](Self::on_hex_input), so this carries no value.
    pub fn on_hex_submit(mut self, on_hex_submit: Message) -> Self {
        self.on_hex_submit = Some(on_hex_submit);
        self
    }

    /// A tab button was pressed: 0 for Palette, 1 for HSLA.
    pub fn on_tab(mut self, on_tab: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_tab = Some(Rc::new(on_tab));
        self
    }
}

impl<Message> Default for ColorPicker<'_, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// One palette cell: a 20px chip whose edge is its own fill darkened, and whose
/// selected edge is the foreground.
///
/// The trigger is a `mouse_area`, not a `container` response, because iced's
/// containers do not answer the pointer; a cell with no message is drawn but not
/// pressable, which is how this port spells a picker with no `on_change`.
fn swatch_cell<'a, Message: Clone + 'a>(
    fill: Color,
    selected: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let chip = container(space().width(Length::Fill).height(Length::Fill))
        .width(Length::Fixed(CELL))
        .height(Length::Fixed(CELL))
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            container::Style {
                background: Some(fill.into()),
                border: Border {
                    color: if selected {
                        tokens.foreground
                    } else {
                        theme::darken(fill, 0.18)
                    },
                    width: if selected { 2.0 } else { 1.0 },
                    radius: theme::RADIUS_SM.into(),
                },
                ..container::Style::default()
            }
        });
    let area = mouse_area(chip);
    match on_press {
        Some(message) => area.on_press(message).into(),
        None => area.into(),
    }
}

/// One row of palette cells, spaced the Slint `spacing: 4px`.
fn palette_row<'a, Message: Clone + 'a>(
    colors: &[Color],
    value: Option<Color>,
    on_change: &Option<Rc<dyn Fn(Option<Color>) -> Message + 'a>>,
) -> Element<'a, Message> {
    let mut line = row![].spacing(theme::SPACING_XS);
    for &color in colors {
        let on_press = on_change.as_ref().map(|change| change(Some(color)));
        line = line.push(swatch_cell(color, value == Some(color), on_press));
    }
    line.into()
}

/// One channel row of the HSLA tab: the Slint `84px` label, the track and the
/// `36px` right-aligned reading.
fn channel_row<'a, Message: 'a>(
    label: &'static str,
    reading: String,
    track: Element<'a, Message>,
) -> Element<'a, Message> {
    let muted = |theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    };
    row![
        container(text(label).size(theme::FONT_SIZE_XS).style(muted))
            .width(Length::Fixed(84.0))
            .align_y(Alignment::Center),
        track,
        container(text(reading).size(theme::FONT_SIZE_XS).style(muted))
            .width(Length::Fixed(36.0))
            .align_x(Alignment::End),
    ]
    .spacing(theme::SPACING_SM)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}

impl<'a, Message> ColorPicker<'a, Message>
where
    Message: Clone + 'static,
{
    /// The whole control: the [`Popover`] that floats the panel, with the square
    /// or the field-shaped trigger.
    fn into_element(self) -> Element<'a, Message> {
        let ColorPicker {
            value,
            hex,
            placeholder,
            featured,
            field,
            size,
            tab,
            open,
            disabled,
            width,
            panel_width,
            id,
            on_open_change,
            on_change,
            on_hex_input,
            on_hex_submit,
            on_tab,
        } = self;

        let empty = value.is_none();
        // The caller's text wins; a caller that only set the value still reads
        // the right colour, and an empty state reads as the placeholder.
        let shown = if hex.is_empty() {
            value.map(to_hex).unwrap_or_default()
        } else {
            hex
        };
        // An empty picker still gives the sliders a baseline: black, fully
        // opaque, which is the Slint defaults' shape.
        let (hue, sat, light, alpha) = match value {
            Some(color) => (hue_of(color), sat_of(color), light_of(color), color.a),
            None => (0.0, 0.0, 0.0, 1.0),
        };

        // ---- the tab strip ------------------------------------------------
        let tab_button = |title: &'static str, index: usize| {
            let mut button = Button::new(title)
                .variant(Variant::Ghost)
                .size(ControlSize::Small)
                .selected(tab == index)
                .width(Length::Fill);
            if let Some(on_tab) = &on_tab {
                button = button.on_press(on_tab(index));
            }
            button
        };
        let tab_strip = container(
            row![tab_button("Palette", 0), tab_button("HSLA", 1)]
                .spacing(2.0)
                .width(Length::Fill),
        )
        .padding(2.0)
        .width(Length::Fill)
        .height(Length::Fixed(28.0))
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).muted.into()),
            border: Border {
                radius: theme::RADIUS_MD.into(),
                ..Border::default()
            },
            ..container::Style::default()
        });

        // ---- the palette tab ----------------------------------------------
        let mut palette = column![].spacing(theme::SPACING_SM);
        let featured_colors = featured.unwrap_or_else(|| FEATURED.to_vec());
        palette = palette.push(palette_row(&featured_colors, value, &on_change));
        palette = palette.push(rule::horizontal(1).style(|theme: &Theme| rule::Style {
            color: Tokens::of(theme).border,
            ..rule::default(theme)
        }));
        for ramp in PALETTES {
            palette = palette.push(palette_row(ramp, value, &on_change));
        }

        // ---- the HSLA tab -------------------------------------------------
        // Each row rebuilds its colour from the other three channels, so the
        // sliders stay a view of `value` rather than a second copy of it.
        let mut hsla = column![].spacing(theme::SPACING_SM);

        let mut hue_track = Slider::new(hue)
            .minimum(0.0)
            .maximum(360.0)
            .step(1.0)
            .width(Length::Fill);
        if let Some(change) = &on_change {
            let change = change.clone();
            hue_track =
                hue_track.on_change(move |next| change(Some(hsl(next, sat, light, alpha))));
        }
        hsla = hsla.push(channel_row(
            "Hue",
            format!("{}", hue.round()),
            hue_track.into(),
        ));

        let mut sat_track = Slider::new(sat * 100.0)
            .minimum(0.0)
            .maximum(100.0)
            .step(1.0)
            .width(Length::Fill);
        if let Some(change) = &on_change {
            let change = change.clone();
            sat_track = sat_track
                .on_change(move |next| change(Some(hsl(hue, next / 100.0, light, alpha))));
        }
        hsla = hsla.push(channel_row(
            "Saturation",
            format!("{}", (sat * 100.0).round()),
            sat_track.into(),
        ));

        let mut light_track = Slider::new(light * 100.0)
            .minimum(0.0)
            .maximum(100.0)
            .step(1.0)
            .width(Length::Fill);
        if let Some(change) = &on_change {
            let change = change.clone();
            light_track = light_track
                .on_change(move |next| change(Some(hsl(hue, sat, next / 100.0, alpha))));
        }
        hsla = hsla.push(channel_row(
            "Lightness",
            format!("{}", (light * 100.0).round()),
            light_track.into(),
        ));

        let mut alpha_track = Slider::new(alpha * 100.0)
            .minimum(0.0)
            .maximum(100.0)
            .step(1.0)
            .width(Length::Fill);
        if let Some(change) = &on_change {
            let change = change.clone();
            alpha_track = alpha_track
                .on_change(move |next| change(Some(hsl(hue, sat, light, next / 100.0))));
        }
        hsla = hsla.push(channel_row(
            "Alpha",
            format!("{}", (alpha * 100.0).round()),
            alpha_track.into(),
        ));

        // ---- the hex field ------------------------------------------------
        let preview = container(space().width(Length::Fill).height(Length::Fill))
            .width(Length::Fixed(16.0))
            .height(Length::Fixed(16.0))
            .style(move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                container::Style {
                    background: value.map(Background::Color),
                    border: Border {
                        color: if empty {
                            tokens.input
                        } else {
                            theme::darken(value.unwrap_or(tokens.input), 0.15)
                        },
                        width: 1.0,
                        radius: theme::RADIUS_SM.into(),
                    },
                    ..container::Style::default()
                }
            });

        let mut hex_field = Input::new(format!("{id}-hex"))
            .value(shown.clone())
            .placeholder(placeholder)
            .size(size)
            .prefix(preview);
        if !disabled {
            if let Some(on_hex_input) = on_hex_input {
                hex_field = hex_field.on_input(on_hex_input);
            }
            if let Some(on_hex_submit) = on_hex_submit {
                hex_field = hex_field.on_submit(on_hex_submit);
            }
        }

        let tab_body: Element<'a, Message> = if tab == 1 {
            hsla.into()
        } else {
            palette.into()
        };
        let panel: Element<'a, Message> = column![tab_strip, tab_body, hex_field]
            .spacing(theme::SPACING_SM + 2.0)
            .into();

        // ---- the trigger --------------------------------------------------
        let height = size.control_height();
        let trigger_width = width.unwrap_or(if field {
            Length::Fill
        } else {
            Length::Fixed(height)
        });

        let trigger: Element<'a, Message> = if field {
            let chip = container(space().width(Length::Fill).height(Length::Fill))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0))
                .style(move |theme: &Theme| {
                    let tokens = Tokens::of(theme);
                    container::Style {
                        background: value.map(Background::Color),
                        border: Border {
                            color: if empty {
                                tokens.input
                            } else {
                                theme::darken(value.unwrap_or(tokens.input), 0.15)
                            },
                            width: 1.0,
                            radius: theme::RADIUS_SM.into(),
                        },
                        ..container::Style::default()
                    }
                });
            // The caret is the component's own, the exception `select` and
            // `date-picker` make for the one mark a control cannot be read
            // without. The bytes are
            // `crates/slint-component/ui/icons/chevron-down.svg` (Lucide, ISC).
            let caret = svg(svg::Handle::from_memory(CHEVRON_XML))
                .width(14.0)
                .height(14.0)
                .style(|theme: &Theme, _status| svg::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                });
            container(
                row![
                    chip,
                    text(shown)
                        .size(size.control_font_size())
                        .width(Length::Fill)
                        // iced 0.14 draws no ellipsis, so a long value clips at
                        // the field's edge where Slint elides.
                        .wrapping(text::Wrapping::None)
                        .font(if empty { iced::Font::DEFAULT } else { theme::MONO }),
                    caret,
                ]
                .spacing(theme::SPACING_SM)
                .align_y(Alignment::Center)
                .width(Length::Fill),
            )
            .padding([0.0, size.control_padding()])
            .height(Length::Fixed(height))
            .width(trigger_width)
            .style(move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                let mut style = input::Style::new()
                    .appearance(true)
                    .disabled(disabled)
                    .resolve(tokens);
                // A value reads as a value and a placeholder reads as muted, the
                // Slint `color: empty ? Theme.muted-foreground : Theme.foreground`.
                style.text_color = Some(if disabled || empty {
                    tokens.muted_foreground
                } else {
                    tokens.foreground
                });
                style
            })
            .into()
        } else {
            container(space().width(Length::Fill).height(Length::Fill))
                .width(trigger_width)
                .height(Length::Fixed(height))
                .style(move |theme: &Theme| {
                    let tokens = Tokens::of(theme);
                    container::Style {
                        background: Some(value.unwrap_or(tokens.background).into()),
                        border: Border {
                            color: tokens.input,
                            width: 1.0,
                            radius: theme::RADIUS_MD.into(),
                        },
                        ..container::Style::default()
                    }
                })
                .into()
        };

        let mut popover = Popover::new(trigger)
            .open(open && !disabled)
            .placement(Placement::Bottom)
            .offset(PANEL_GAP)
            .panel_width(panel_width)
            // The popover owns the panel's surface, so there is no second one
            // around the content; the inset is Slint's `padding: 12px`.
            .filled(true)
            .padding(PANEL_INSET);
        if !disabled && let Some(on_open_change) = on_open_change {
            popover = popover.on_open_change(on_open_change);
        }

        popover.content(panel).into()
    }
}

impl<'a, Message> From<ColorPicker<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'static,
{
    fn from(picker: ColorPicker<'a, Message>) -> Self {
        picker.into_element()
    }
}

/// The caret at the field trigger's trailing edge, drawn from
/// `crates/slint-component/ui/icons/chevron-down.svg` (Lucide, ISC).
const CHEVRON_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#;
