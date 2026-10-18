//! `avatar` — a user portrait. Mirrors the Slint `Avatar` beside it: an image,
//! an initials fallback or an icon fallback inside one clipped rounded box, at
//! five sizes, with an optional custom edge and radius, fill, initials colour,
//! border and drop shadow.
//!
//! The Slint component derives its corner from the edge, so the default is a
//! circle (`edge / 2`) and [`Avatar::radius`] is what makes it a rounded
//! rectangle. The same three layers are drawn here: the box — fill, border,
//! radius, shadow — then its content, clipped to the radius.
//!
//! Two deliberate differences, because iced 0.14 has no equivalent:
//!
//! - **The icon is the caller's element.** Slint ships its own SVG folder and
//!   resolves `icon: "user"` from it; iced 0.14 ships no icon set, so
//!   [`Avatar::icon`] takes an [`Element`] the application supplies (an `svg`,
//!   a registered icon font, or a glyph). Without one the fallback is the
//!   initials, then the fill alone, which is what Slint draws for
//!   `initials: ""` with an icon it cannot resolve.
//! - **The image and the box are one surface.** Slint sets the fill behind the
//!   image; here the image carries the radius and the box behind it carries the
//!   fill, border and shadow, so a partly transparent image still reads over the
//!   fill rather than over the page.
//!
//! Slint pairs this with an `AvatarGroup` that overlaps several avatars and
//! prints an overflow chip. iced has no negative spacing, so an overlapping
//! group is a `stack` the application builds; this file is one avatar.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{container, image, text};
use iced::{Alignment, Background, Border, Color, ContentFit, Element, Length, Shadow, Theme};

use super::theme::Tokens;

/// The size step an avatar is built at: the Slint `AvatarSize`, whose edges are
/// 20 / 24 / 32 / 40 / 56 pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize {
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

impl AvatarSize {
    /// The edge in pixels, which is the Slint size ladder.
    pub fn edge(self) -> f32 {
        match self {
            Self::XSmall => 20.0,
            Self::Small => 24.0,
            Self::Medium => 32.0,
            Self::Large => 40.0,
            Self::XLarge => 56.0,
        }
    }
}

/// What the avatar draws when it has no image. Slint picks between the three by
/// testing `source`, then `initials`; an avatar that has neither draws its fill
/// alone.
pub enum Fallback<'a, Message> {
    /// The initials, centred at 38% of the edge.
    Initials(String),
    /// A caller-supplied icon element, centred at 45% of the edge. iced has no
    /// icon set, so the element is the application's.
    Icon(Element<'a, Message>),
    /// The fill alone, as Slint draws for an empty initials string.
    None,
}

/// An avatar, built with the methods below.
pub struct Avatar<'a, Message> {
    image: Option<image::Handle>,
    fallback: Fallback<'a, Message>,
    size: AvatarSize,
    custom_edge: Option<f32>,
    radius: Option<f32>,
    fill: Option<Color>,
    initials_color: Option<Color>,
    border_width: f32,
    border_color: Option<Color>,
    shadow_blur: f32,
    shadow_offset_y: f32,
    shadow_color: Option<Color>,
}

impl<'a, Message> Avatar<'a, Message> {
    /// An avatar with no image, which draws the icon fallback the caller gives
    /// it; a bare `Avatar::new()` draws its fill alone.
    pub fn new() -> Self {
        Self {
            image: None,
            fallback: Fallback::None,
            size: AvatarSize::default(),
            custom_edge: None,
            radius: None,
            fill: None,
            initials_color: None,
            border_width: 1.0,
            border_color: None,
            shadow_blur: 0.0,
            shadow_offset_y: 1.0,
            shadow_color: None,
        }
    }

    /// The portrait. The handle is the caller's, because iced applications own
    /// their assets; see `crates/iced-component/README.md`.
    pub fn image(mut self, handle: impl Into<image::Handle>) -> Self {
        self.image = Some(handle.into());
        self
    }

    /// The initials shown when there is no image. Anything longer than two
    /// characters is drawn as given, which is the caller's business, not this
    /// component's.
    pub fn initials(mut self, initials: impl Into<String>) -> Self {
        self.fallback = Fallback::Initials(initials.into());
        self
    }

    /// The icon shown when there is neither an image nor initials.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.fallback = Fallback::Icon(icon.into());
        self
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    /// An edge in pixels that overrides [`Avatar::size`], the Slint
    /// `custom-edge`.
    pub fn edge(mut self, edge: f32) -> Self {
        self.custom_edge = Some(edge);
        self
    }

    /// A corner radius in pixels. The default is half the edge, which is the
    /// Slint circle; set one for a rounded rectangle.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    /// The fill behind the image and the initials. Defaults to the theme's
    /// secondary, as the Slint component does.
    pub fn fill(mut self, fill: Color) -> Self {
        self.fill = Some(fill);
        self
    }

    /// The initials' colour. Defaults to the theme's secondary foreground; a
    /// caller that darkens the fill sets this too, so the initials keep their
    /// contrast.
    pub fn initials_color(mut self, color: Color) -> Self {
        self.initials_color = Some(color);
        self
    }

    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }

    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// The drop shadow's blur radius. Zero, the default, draws none.
    pub fn shadow_blur(mut self, blur: f32) -> Self {
        self.shadow_blur = blur;
        self
    }

    /// The drop shadow's vertical offset, 1 pixel by default.
    pub fn shadow_offset_y(mut self, offset: f32) -> Self {
        self.shadow_offset_y = offset;
        self
    }

    pub fn shadow_color(mut self, color: Color) -> Self {
        self.shadow_color = Some(color);
        self
    }

    /// The edge the avatar draws at: the custom edge when it is above zero, and
    /// otherwise the size ladder.
    pub fn edge_px(&self) -> f32 {
        match self.custom_edge {
            Some(edge) if edge > 0.0 => edge,
            _ => self.size.edge(),
        }
    }

    /// The corner: the custom radius when it is above zero, and otherwise a
    /// circle.
    pub fn radius_px(&self) -> f32 {
        match self.radius {
            Some(radius) if radius > 0.0 => radius,
            _ => self.edge_px() / 2.0,
        }
    }
}

impl<'a, Message> Default for Avatar<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// How an avatar's box is drawn, resolved against the tokens at render time.
struct Shape {
    fill: Color,
    border_color: Color,
    border_width: f32,
    radius: f32,
    shadow: Shadow,
}

impl Shape {
    fn resolve(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.fill)),
            border: Border {
                color: self.border_color,
                width: self.border_width,
                radius: self.radius.into(),
            },
            shadow: self.shadow,
            ..container::Style::default()
        }
    }
}

impl<'a, Message> From<Avatar<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Avatar<'a, Message>) -> Self {
        let edge = control.edge_px();
        let radius = control.radius_px();
        let fill = control.fill;
        let initials_color = control.initials_color;
        let border_color = control.border_color;
        let border_width = control.border_width;
        let shadow_blur = control.shadow_blur;
        let shadow_offset_y = control.shadow_offset_y;
        let shadow_color = control.shadow_color;

        // The content is drawn inside the box, clipped to the same corner. Text
        // takes its colour from the closure rather than from a palette captured
        // here, so a theme switch moves it with everything else.
        let content: Element<'a, Message> = match (control.image, control.fallback) {
            (Some(handle), _) => image(handle)
                .width(Length::Fixed(edge))
                .height(Length::Fixed(edge))
                .content_fit(ContentFit::Cover)
                .border_radius(radius)
                .into(),
            (None, Fallback::Initials(initials)) => container(
                text(initials)
                    .size(edge * 0.38)
                    .font(iced::Font {
                        weight: iced::font::Weight::Semibold,
                        ..iced::Font::DEFAULT
                    })
                    .style(move |theme: &Theme| text::Style {
                        color: Some(
                            initials_color.unwrap_or(Tokens::of(theme).secondary_foreground),
                        ),
                    }),
            )
            .width(Length::Fixed(edge))
            .height(Length::Fixed(edge))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into(),
            (None, Fallback::Icon(icon)) => container(icon)
                .width(Length::Fixed(edge))
                .height(Length::Fixed(edge))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into(),
            (None, Fallback::None) => container(iced::widget::space())
                .width(Length::Fixed(edge))
                .height(Length::Fixed(edge))
                .into(),
        };

        container(content)
            .width(Length::Fixed(edge))
            .height(Length::Fixed(edge))
            .clip(true)
            .style(move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                Shape {
                    fill: fill.unwrap_or(tokens.secondary),
                    border_color: border_color.unwrap_or(tokens.border),
                    border_width,
                    radius,
                    shadow: Shadow {
                        color: shadow_color.unwrap_or(tokens.shadow),
                        offset: iced::Vector::new(0.0, shadow_offset_y),
                        blur_radius: shadow_blur,
                    },
                }
                .resolve()
            })
            .into()
    }
}
