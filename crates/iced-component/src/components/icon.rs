//! An icon. Mirrors GPUI Component's `Icon` and the Slint version beside it: an
//! SVG tinted in a theme colour, drawn on a square box at 12, 14, 16 or 24
//! pixels, optionally rotating.
//!
//! **The artwork is the caller's.** Slint ships an `icons/` folder, a `name`
//! property and a chain of `@image-url` arms to pick out of it; GPUI names its
//! glyphs with an `IconName` enum. An iced application embeds its own SVGs, so
//! [`Icon::new`] takes the [`svg::Handle`] it built — a memory handle from
//! `include_bytes!`, or a path when the application ships its assets loose. The
//! set belongs to the application and this component never picks from it.
//!
//! **The rotation is a value, not a clock.** Slint spins the image from
//! `animation-tick()` and GPUI drives a `with_animation`. iced's timer needs a
//! runtime feature this crate does not enable, so [`Icon::spin`] takes the
//! turn's progress in `0.0..=1.0` and the caller advances it from a
//! subscription, exactly as [`Spinner`](super::spinner) does with its
//! `progress`. The icon is drawn at `progress * 2π` radians about its centre,
//! and iced keeps the box the icon had before the turn, so a spinning icon
//! never shifts its neighbours.
//!
//! One Slint property has no iced equivalent: `name`, because the handle
//! already is the artwork.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::svg;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::{container, svg as svg_widget};
use iced::{Color, ContentFit, Element, Length, Rotation, Theme};

use super::theme::{ControlSize, Tokens};

/// How big the icon's square box is.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Size {
    /// One of the four steps the catalog shares: 12, 14, 16 or 24 pixels. The
    /// Slint version defaults to 16, which is [`ControlSize::Medium`].
    #[default]
    Medium,
    XSmall,
    Small,
    Large,
    /// An exact edge length in pixels, for the `size` property Slint lets a
    /// caller set to any length.
    Custom(f32),
}

impl From<ControlSize> for Size {
    fn from(size: ControlSize) -> Self {
        match size {
            ControlSize::XSmall => Self::XSmall,
            ControlSize::Small => Self::Small,
            ControlSize::Medium => Self::Medium,
            ControlSize::Large => Self::Large,
        }
    }
}

impl From<f32> for Size {
    fn from(pixels: f32) -> Self {
        Self::Custom(pixels)
    }
}

impl Size {
    /// The edge length in pixels. Slint's icon box is the glyph's own size, so
    /// the four steps are the same numbers GPUI's `size_3` through `size_6`
    /// give an icon.
    pub fn pixels(self) -> f32 {
        match self {
            Self::XSmall => 12.0,
            Self::Small => 14.0,
            Self::Medium => 16.0,
            Self::Large => 24.0,
            Self::Custom(pixels) => pixels.max(0.0),
        }
    }
}

/// An icon, built with the methods below. It carries no message, but it is
/// generic over one so a caller can place it in any view without annotating the
/// type.
pub struct Icon<Message> {
    handle: svg::Handle,
    size: Size,
    color: Option<Color>,
    spin: Option<f32>,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Icon<Message> {
    /// An icon drawing `handle`, at the default 16 pixels in the foreground
    /// colour.
    pub fn new(handle: impl Into<svg::Handle>) -> Self {
        Self {
            handle: handle.into(),
            size: Size::default(),
            color: None,
            spin: None,
            _message: std::marker::PhantomData,
        }
    }

    /// The edge of the icon's square box: a [`ControlSize`], a [`Size`], or a
    /// pixel count.
    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = size.into();
        self
    }

    /// The glyph's colour. The default is the theme's foreground, which is what
    /// the Slint property defaults to.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Turns the icon `progress` of the way round, in `0.0..=1.0`, about its
    /// centre. The caller advances it; `None`, the default, holds the icon
    /// still. The Slint property of the same name is a flag its clock drives,
    /// so this is that flag plus the value the clock would have produced.
    pub fn spin(mut self, progress: f32) -> Self {
        self.spin = Some(progress);
        self
    }
}

impl<Message> Default for Icon<Message> {
    fn default() -> Self {
        Self::new(svg::Handle::from_memory(Vec::new()))
    }
}

/// The colours and geometry an icon draws with. [`Style::resolve`] fills in the
/// colour from the theme in force, so an icon built without a colour follows a
/// light or dark switch.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub size: Size,
    pub color: Option<Color>,
    pub spin: Option<f32>,
}

impl Style {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            color: None,
            spin: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn spin(mut self, progress: f32) -> Self {
        self.spin = Some(progress);
        self
    }

    /// The colour the glyph is tinted. The Slint property always has a value,
    /// so an icon without a colour of its own takes the theme's foreground in
    /// whichever mode is in force; a colour the caller named wins.
    pub fn resolve(&self, tokens: Tokens) -> svg::Style {
        svg::Style {
            color: Some(self.color.unwrap_or(tokens.foreground)),
        }
    }

    /// The edge length the icon's box takes.
    pub fn pixels(&self) -> f32 {
        self.size.pixels()
    }

    /// The turn, in radians. iced rotates an SVG without moving its layout, so
    /// the box stays square at every angle.
    pub fn rotation(&self) -> Rotation {
        Rotation::Floating(iced::Radians(self.spin.unwrap_or(0.0) * std::f32::consts::TAU))
    }
}

impl<'a, Message> From<Icon<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Icon<Message>) -> Self {
        let style = Style {
            size: control.size,
            color: control.color,
            spin: control.spin,
        };
        let pixels = style.pixels();

        let glyph = svg_widget(control.handle)
            .width(Length::Fixed(pixels))
            .height(Length::Fixed(pixels))
            .content_fit(ContentFit::Contain)
            .rotation(style.rotation())
            .style(move |theme: &Theme, _status| style.resolve(Tokens::of(theme)));

        // The wrapper paints nothing; it fixes the box the icon occupies, so a
        // glyph whose own viewBox is a different shape still lands in a square.
        container(glyph)
            .center(Length::Fixed(pixels))
            .style(|_theme: &Theme| ContainerStyle::default())
            .into()
    }
}
