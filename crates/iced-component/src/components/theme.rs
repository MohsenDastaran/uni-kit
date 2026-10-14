//! Design tokens for the iced components, mirrored from
//! `crates/slint-component/ui/theme.slint`, which is mapped from
//! `tokens/tokens.json`.
//!
//! Copy this file with any component; every component reads it and nothing else.
//! The palette follows the iced theme in force — [`Tokens::of`] picks the light
//! or dark set from it — so a component looks right in either without being told
//! which is on.
//!
//! The names are the ones the other two frameworks use, so a component can be
//! read beside its GPUI and Slint counterparts: `background`, `foreground`,
//! `primary`, `muted-foreground`, `border`, `ring`, the chart series, and the
//! spacing and radius scales.

// The tokens are the whole scale, and a component reads the part it needs: a
// project that installs one component gets the same file as a project that
// installs twenty, so the unused half is not a mistake to warn about.
#![allow(dead_code)]

use std::cell::Cell;

use iced::{Color, Font, Theme};

// ---------------------------------------------------------------- scales

pub const SPACING_XXS: f32 = 2.0;
pub const SPACING_XS: f32 = 4.0;
pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 12.0;
pub const SPACING_LG: f32 = 16.0;
pub const SPACING_XL: f32 = 24.0;
pub const SPACING_XXL: f32 = 32.0;

pub const RADIUS_SM: f32 = 3.0;
pub const RADIUS_MD: f32 = 6.0;
pub const RADIUS_LG: f32 = 8.0;
pub const RADIUS_XL: f32 = 12.0;

pub const FONT_SIZE_XS: f32 = 12.0;
pub const FONT_SIZE_SM: f32 = 14.0;
pub const FONT_SIZE_MD: f32 = 16.0;
pub const FONT_SIZE_LG: f32 = 18.0;
pub const FONT_SIZE_XL: f32 = 20.0;
pub const FONT_SIZE_MONO: f32 = 13.0;

/// The monospace face the components draw code in.
pub const MONO: Font = Font::MONOSPACE;

// ---------------------------------------------------------------- enums

/// The size step a control is built at. GPUI's `Sizable` and the Slint theme
/// share these four.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ControlSize {
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
}

impl ControlSize {
    /// Padding inside a button, matching GPUI's size_1 through size_4.
    pub fn control_padding(self) -> f32 {
        match self {
            Self::XSmall => 6.0,
            Self::Small => 8.0,
            Self::Large => 16.0,
            Self::Medium => 12.0,
        }
    }

    /// A control's height in a row: 20 / 24 / 32 / 40, GPUI's `h_5`, `h_6`,
    /// `h_8` and `h_11`. Slint spells the same table `control-height(size)` and
    /// the buttons and tabs on this side already repeat it; the toggle is the
    /// first control here to ask for it by name.
    pub fn control_height(self) -> f32 {
        match self {
            Self::XSmall => 20.0,
            Self::Small => 24.0,
            Self::Large => 40.0,
            Self::Medium => 32.0,
        }
    }

    /// Padding inside a text field, which is wider than a button's.
    pub fn input_padding_x(self) -> f32 {
        match self {
            Self::XSmall => 4.0,
            Self::Small => 8.0,
            Self::Large => 12.0,
            Self::Medium => 10.0,
        }
    }

    /// A button label's size: xs is text-xs, small text-sm, the rest text-base.
    pub fn control_font_size(self) -> f32 {
        match self {
            Self::XSmall => FONT_SIZE_XS,
            Self::Small => FONT_SIZE_SM,
            _ => FONT_SIZE_MD,
        }
    }

    /// A checkbox or radio label. Large is text-lg; buttons stay at text-base.
    pub fn label_font_size(self) -> f32 {
        match self {
            Self::Large => FONT_SIZE_LG,
            other => other.control_font_size(),
        }
    }

    /// A text field's height in a row: 20 / 24 / 44 / 32, the Slint theme's
    /// `input-height(size)`, and taller than a button at `large` — a field is a
    /// box a caret has to sit in, not a control a label has to fit. `input.rs`
    /// repeats the table for its own frame; `select` reads it here so its trigger
    /// is the same box as the field beside it.
    pub fn input_height(self) -> f32 {
        match self {
            Self::XSmall => 20.0,
            Self::Small => 24.0,
            Self::Large => 44.0,
            Self::Medium => 32.0,
        }
    }

    /// A text field's size. Medium is text-sm and large is text-base.
    pub fn input_font_size(self) -> f32 {
        match self {
            Self::XSmall => FONT_SIZE_XS,
            Self::Large => FONT_SIZE_MD,
            _ => FONT_SIZE_SM,
        }
    }

    /// Icon boxes: 12 / 14 / 16 / 24, matching GPUI's size_3 through size_6.
    pub fn icon_size(self) -> f32 {
        match self {
            Self::XSmall => 12.0,
            Self::Small => 14.0,
            Self::Large => 24.0,
            Self::Medium => 16.0,
        }
    }
}

/// The status a control is reporting, which the status colours answer to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Danger,
}

// ---------------------------------------------------------------- palette

/// The colours in force. [`Tokens::of`] returns the set that matches the iced
/// theme, so components never ask whether dark mode is on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tokens {
    pub dark: bool,

    pub background: Color,
    pub foreground: Color,
    pub surface: Color,
    pub primary: Color,
    pub primary_foreground: Color,
    pub secondary: Color,
    pub secondary_foreground: Color,
    pub muted: Color,
    pub muted_foreground: Color,
    pub link: Color,
    pub input_background: Color,
    pub accent: Color,
    pub accent_foreground: Color,
    pub destructive: Color,
    pub destructive_foreground: Color,
    pub danger: Color,
    pub border: Color,
    pub input: Color,
    pub ring: Color,
    pub selection: Color,
    pub info: Color,
    pub success: Color,
    pub warning: Color,
    pub chart_1: Color,
    pub chart_2: Color,
    pub chart_3: Color,
    pub chart_4: Color,
    pub chart_5: Color,
    pub description_label: Color,
    pub description_label_foreground: Color,
    pub group_box: Color,
    pub group_box_foreground: Color,
    pub group_box_title_foreground: Color,
    pub popover: Color,
    pub sidebar: Color,
    pub overlay: Color,
    pub shadow: Color,
}

impl Tokens {
    /// The tokens that match a theme.
    ///
    /// A gallery embedded in a documentation page sets the page's palette once,
    /// at boot ([`set_site_palette`]); from then on that is the palette in
    /// force, because it is the one the page around the frame is drawn in.
    /// Native and standalone runs never set it and keep the built-in tables.
    pub fn of(theme: &Theme) -> Self {
        SITE_PALETTE.with(|palette| palette.get()).unwrap_or_else(|| {
            if theme.extended_palette().is_dark {
                DARK
            } else {
                LIGHT
            }
        })
    }

    /// The tokens of an embedding page's palette.
    ///
    /// `token` answers with the page's value for one of the theme tokens its
    /// navbar applies — `--background`, `--primary`, and the rest of the scale
    /// on `<html>`. A background and a foreground are required; every other
    /// token falls back to the nearest one that is present, the way the page's
    /// own CSS does, so a page that publishes a shorter scale still yields a
    /// whole palette rather than a half-drawn one. `None` means the page has no
    /// palette at all, and the caller should keep the built-in tables.
    pub fn from_site(dark: bool, token: impl Fn(&str) -> Option<Color>) -> Option<Self> {
        let background = token("--background")?;
        let foreground = token("--foreground")?;
        // The page's mode names which built-in table is the fallback for a
        // token the page itself did not publish.
        let built_in = if dark { DARK } else { LIGHT };
        let or = |value: Option<Color>, fallback: Color| value.unwrap_or(fallback);
        let data = |index: usize| token(&format!("--data-{index}"));

        let surface = or(token("--card"), background);
        let primary = or(token("--primary"), foreground);
        let primary_foreground = or(token("--primary-foreground"), background);
        let secondary = or(token("--secondary"), background);
        let secondary_foreground = or(token("--secondary-foreground"), foreground);
        let muted = or(token("--muted"), background);
        let muted_foreground = or(token("--muted-foreground"), foreground);
        let accent = or(token("--accent"), background);
        let accent_foreground = or(token("--accent-foreground"), foreground);
        let destructive = or(token("--destructive"), built_in.destructive);
        let border = or(token("--border"), foreground);
        let input = or(token("--input"), foreground);
        let ring = or(token("--ring"), foreground);

        Some(Self {
            dark,
            background,
            foreground,
            surface,
            primary,
            primary_foreground,
            secondary,
            secondary_foreground,
            muted,
            muted_foreground,
            // The page publishes no link token. GPUI draws a link in
            // `primary`, so that is the nearest role to take it from.
            link: primary,
            // A text field keeps the border's colour at a third of its weight
            // over the page in dark mode, and sits on the page in light mode,
            // which is what the Slint theme computes.
            input_background: if dark { alpha(input, 0.3) } else { background },
            accent,
            accent_foreground,
            destructive,
            destructive_foreground: if dark {
                destructive
            } else {
                primary_foreground
            },
            danger: destructive,
            border,
            input,
            ring,
            selection: or(token("--selection"), built_in.selection),
            // The page's first data colour is its informational one, and the
            // five are its chart series — the same reading the Slint palette
            // makes of `--data-*`.
            info: or(data(1), built_in.info),
            success: or(token("--success"), built_in.success),
            warning: or(token("--warning"), built_in.warning),
            chart_1: or(data(1), built_in.chart_1),
            chart_2: or(data(2), built_in.chart_2),
            chart_3: or(data(3), built_in.chart_3),
            chart_4: or(data(4), built_in.chart_4),
            chart_5: or(data(5), built_in.chart_5),
            // The dark theme names this surface; the light theme mixes it
            // barely off the border. `mix` weights its second colour, so the
            // background keeps 0.8.
            description_label: if dark {
                muted
            } else {
                mix(background, border, 0.2)
            },
            description_label_foreground: foreground,
            group_box: muted,
            group_box_foreground: secondary_foreground,
            group_box_title_foreground: muted_foreground,
            popover: or(token("--popover"), background),
            sidebar: or(token("--sidebar"), background),
            // A dim and a shadow are translucency, not palette: the page has
            // no token for either, and the built-in weights are already right
            // for a light or a dark surface.
            overlay: if dark {
                rgba(0x00, 0x00, 0x00, 0xa0)
            } else {
                rgba(0x00, 0x00, 0x00, 0x4d)
            },
            shadow: if dark {
                rgba(0x00, 0x00, 0x00, 0x80)
            } else {
                rgba(0x00, 0x00, 0x00, 0x1f)
            },
        })
    }

    /// The colour a status reports in.
    pub fn status_color(&self, status: Status) -> Color {
        match status {
            Status::Default => self.foreground,
            Status::Info => self.info,
            Status::Success => self.success,
            Status::Warning => self.warning,
            Status::Danger => self.danger,
        }
    }
}

thread_local! {
    /// The palette the embedding page asked for, for the life of the process.
    ///
    /// iced's [`Theme`] carries six colours and a component reads the whole
    /// token scale, so the extra roles have nowhere to live in a theme. The
    /// gallery reads the page's tokens once, at boot, and parks them here,
    /// which is the only place [`Tokens::of`] looks for them.
    static SITE_PALETTE: Cell<Option<Tokens>> = const { Cell::new(None) };
}

/// Wear `tokens` as the palette every component reads, until set again.
///
/// The gallery calls this with the embedding page's own palette; `None` returns
/// a process to the built-in light and dark tables.
pub fn set_site_palette(tokens: Option<Tokens>) {
    SITE_PALETTE.with(|palette| palette.set(tokens));
}

pub const LIGHT: Tokens = Tokens {
    dark: false,
    background: rgb(0xff, 0xff, 0xff),
    foreground: rgb(0x0a, 0x0a, 0x0a),
    surface: rgb(0xff, 0xff, 0xff),
    primary: rgb(0x17, 0x17, 0x17),
    primary_foreground: rgb(0xfa, 0xfa, 0xfa),
    secondary: rgb(0xe5, 0xe5, 0xe5),
    secondary_foreground: rgb(0x17, 0x17, 0x17),
    muted: rgb(0xf5, 0xf5, 0xf5),
    muted_foreground: rgb(0x73, 0x73, 0x73),
    link: rgb(0x25, 0x63, 0xeb),
    // A text field's own fill is the page's, so it reads as one surface.
    input_background: rgb(0xff, 0xff, 0xff),
    accent: rgb(0xf5, 0xf5, 0xf5),
    accent_foreground: rgb(0x17, 0x17, 0x17),
    destructive: rgb(0xef, 0x44, 0x44),
    destructive_foreground: rgb(0xfa, 0xfa, 0xfa),
    danger: rgb(0xef, 0x44, 0x44),
    border: rgb(0xe5, 0xe5, 0xe5),
    input: rgb(0xe5, 0xe5, 0xe5),
    ring: rgb(0xa3, 0xa3, 0xa3),
    selection: rgb(0x55, 0xa0, 0xfc),
    info: rgb(0x06, 0xb6, 0xd4),
    success: rgb(0x22, 0xc5, 0x5e),
    warning: rgb(0xea, 0xb3, 0x08),
    chart_1: rgb(0xe7, 0x6e, 0x50),
    chart_2: rgb(0x2a, 0x9d, 0x90),
    chart_3: rgb(0x27, 0x47, 0x54),
    chart_4: rgb(0xe8, 0xc4, 0x68),
    chart_5: rgb(0xf4, 0xa4, 0x62),
    // The light theme's description label is the page mixed barely off the
    // border, which is what the Slint theme computes.
    description_label: rgb(0xeb, 0xeb, 0xeb),
    description_label_foreground: rgb(0x17, 0x17, 0x17),
    group_box: rgb(0xf5, 0xf5, 0xf5),
    group_box_foreground: rgb(0x17, 0x17, 0x17),
    group_box_title_foreground: rgb(0x73, 0x73, 0x73),
    popover: rgb(0xff, 0xff, 0xff),
    sidebar: rgb(0xfa, 0xfa, 0xfa),
    overlay: rgba(0x00, 0x00, 0x00, 0x4d),
    shadow: rgba(0x00, 0x00, 0x00, 0x1f),
};

pub const DARK: Tokens = Tokens {
    dark: true,
    background: rgb(0x0a, 0x0a, 0x0a),
    foreground: rgb(0xfa, 0xfa, 0xfa),
    surface: rgb(0x0a, 0x0a, 0x0a),
    primary: rgb(0xfa, 0xfa, 0xfa),
    primary_foreground: rgb(0x17, 0x17, 0x17),
    secondary: rgb(0x26, 0x26, 0x26),
    secondary_foreground: rgb(0xfa, 0xfa, 0xfa),
    muted: rgb(0x26, 0x26, 0x26),
    muted_foreground: rgb(0xa3, 0xa3, 0xa3),
    link: rgb(0x60, 0xa5, 0xfa),
    // A text field keeps the border's colour at a third of its weight over the
    // page, which is what the Slint theme computes.
    input_background: rgba(0x2f, 0x2f, 0x2f, 0x4d),
    accent: rgb(0x26, 0x26, 0x26),
    accent_foreground: rgb(0xfa, 0xfa, 0xfa),
    destructive: rgb(0xf8, 0x71, 0x71),
    destructive_foreground: rgb(0xdc, 0x26, 0x26),
    danger: rgb(0xf8, 0x71, 0x71),
    border: rgb(0x26, 0x26, 0x26),
    input: rgb(0x2f, 0x2f, 0x2f),
    ring: rgb(0x73, 0x73, 0x73),
    selection: rgb(0x1d, 0x4e, 0xd8),
    info: rgb(0x22, 0xd3, 0xee),
    success: rgb(0x4a, 0xde, 0x80),
    warning: rgb(0xfa, 0xcc, 0x15),
    chart_1: rgb(0x26, 0x62, 0xd9),
    chart_2: rgb(0x2e, 0xb8, 0x8a),
    chart_3: rgb(0xe8, 0x8c, 0x30),
    chart_4: rgb(0xaf, 0x57, 0xdb),
    chart_5: rgb(0xe2, 0x36, 0x70),
    description_label: rgb(0x17, 0x17, 0x17),
    description_label_foreground: rgb(0xf5, 0xf5, 0xf5),
    group_box: rgb(0x26, 0x26, 0x26),
    group_box_foreground: rgb(0xfa, 0xfa, 0xfa),
    group_box_title_foreground: rgb(0xa3, 0xa3, 0xa3),
    popover: rgb(0x17, 0x17, 0x17),
    sidebar: rgb(0x11, 0x11, 0x11),
    overlay: rgba(0x00, 0x00, 0x00, 0xa0),
    shadow: rgba(0x00, 0x00, 0x00, 0x80),
};

// ---------------------------------------------------------------- colour

/// A colour from 8-bit channels, for the token tables above.
const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

/// A colour from 8-bit channels and an 8-bit alpha, for the token tables above.
const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: a as f32 / 255.0,
    }
}

/// The same colour at a different opacity.
pub fn alpha(color: Color, a: f32) -> Color {
    Color { a, ..color }
}

/// `a` brought `t` of the way to `b`: `a` keeps `1 - t`, `b` takes `t`.
///
/// This is **not** Slint's reading. Slint's `a.mix(b, factor)` weights the
/// receiver, so it is this call with `t = 1 - factor` — `mix(a, b, 1.0 - factor)`.
/// A call site that transcribes a Slint factor without inverting it inverts the
/// weight, which is how the alert's fill became the status colour at 96% instead
/// of a 4% tint over the page; the call sites that do read a Slint `mix` on
/// purpose pass the inversion, such as the bubble's `0.24` becoming `0.76`.
pub fn mix(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

/// Slint's `brighter`: lightness raised in HSL, not a wash of white.
pub fn lighten(color: Color, amount: f32) -> Color {
    with_lightness(color, |l| l + (1.0 - l) * amount)
}

/// Slint's `darker`: lightness lowered in HSL.
pub fn darken(color: Color, amount: f32) -> Color {
    with_lightness(color, |l| l * (1.0 - amount))
}

/// The fill a control takes when the pointer is over it.
pub fn hovered(tokens: &Tokens, fill: Color) -> Color {
    if tokens.dark {
        lighten(fill, 0.15)
    } else {
        darken(fill, 0.06)
    }
}

/// The fill a control takes while it is held.
pub fn pressed(tokens: &Tokens, fill: Color) -> Color {
    if tokens.dark {
        lighten(fill, 0.3)
    } else {
        darken(fill, 0.12)
    }
}

fn with_lightness(color: Color, adjust: impl Fn(f32) -> f32) -> Color {
    let (h, s, l) = to_hsl(color);
    from_hsl(h, s, adjust(l).clamp(0.0, 1.0), color.a)
}

fn to_hsl(color: Color) -> (f32, f32, f32) {
    let max = color.r.max(color.g).max(color.b);
    let min = color.r.min(color.g).min(color.b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < f32::EPSILON {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == color.r {
        ((color.g - color.b) / d + if color.g < color.b { 6.0 } else { 0.0 }) / 6.0
    } else if max == color.g {
        ((color.b - color.r) / d + 2.0) / 6.0
    } else {
        ((color.r - color.g) / d + 4.0) / 6.0
    };
    (h, s, l)
}

fn from_hsl(h: f32, s: f32, l: f32, a: f32) -> Color {
    if s <= 0.0 {
        return Color { r: l, g: l, b: l, a };
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    Color {
        r: hue(p, q, h + 1.0 / 3.0),
        g: hue(p, q, h),
        b: hue(p, q, h - 1.0 / 3.0),
        a,
    }
}

fn hue(p: f32, q: f32, t: f32) -> f32 {
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

/// The theme the gallery runs under.
///
/// iced's own light and dark palettes are greys of its choosing, and a window
/// clears to its palette's background — so an embedded gallery sat a shade lighter
/// than the page around it however carefully the page was coloured. This palette is
/// the same tokens every component reads, so the clear is the page's own background
/// in either appearance.
pub fn iced_theme(dark: bool) -> iced::Theme {
    let tokens = if dark { DARK } else { LIGHT };
    theme_for(if dark { "uni-kit dark" } else { "uni-kit light" }, &tokens)
}

/// The iced theme a token set describes, named for the palette it came from.
///
/// iced draws its own widgets from the six-colour [`iced::theme::Palette`],
/// while the components draw from the whole [`Tokens`] scale. This is the
/// bridge: a page's tokens become the theme's own six core colours, so a window
/// clears to the page's background and iced's built-in drawing sits in the same
/// palette the components do.
pub fn theme_for(name: &str, tokens: &Tokens) -> iced::Theme {
    iced::Theme::custom(
        name.to_string(),
        iced::theme::Palette {
            background: tokens.background,
            text: tokens.foreground,
            primary: tokens.primary,
            success: tokens.success,
            warning: tokens.warning,
            danger: tokens.danger,
        },
    )
}
