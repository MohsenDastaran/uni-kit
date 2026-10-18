//! `image` — a framed picture. Mirrors the Slint `ImageView` beside it: a
//! source, a fit, a corner radius, and a placeholder for the frame that has no
//! source.
//!
//! iced already draws an image, so this is not a re-wrap of the widget: what it
//! adds is the **frame** the Slint component draws around one. A muted,
//! bordered, rounded box clips the source to a corner radius, and a frame with
//! no source keeps its size and centres the placeholder instead of collapsing
//! to nothing. `ContentFit` is the Slint `ImageFit` renamed, limited to the
//! three arms Slint has: `cover` crops, `contain` letterboxes, `fill` stretches.
//!
//! Two things are deliberately not here:
//!
//! - **The placeholder is the caller's element.** Slint names an `image` icon in
//!   its own `icons/` folder; iced ships no icon set, so [`ImageView::placeholder`]
//!   takes an [`Element`] the application supplies. Without one the empty frame
//!   draws its fill and border alone, which is the same box Slint would draw if
//!   its icon could not resolve.
//! - **There is no loading or error state.** A Slint `image` property is a
//!   handle to bytes, and so is `iced::widget::image::Handle` — a source that
//!   cannot be decoded never reaches the widget. An application that fetches or
//!   decodes asynchronously swaps the source when the bytes arrive; the frame
//!   keeps its size in the meantime, which is what the placeholder is for.
//!
//! The geometry Slint gives its frame is kept as closely as iced allows: a fill
//! width, a 160-pixel preferred height (the caller sets both), and a
//! medium-radius corner by default.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{container, image};
use iced::{Alignment, Background, Border, Element, Length, Theme};

use super::theme::{self, Tokens};

/// How a source is fitted into its frame, the Slint `ImageFit` with Slint's
/// three arms. iced's [`iced::ContentFit`] has two more (`None`, `ScaleDown`);
/// they are left out because the component's frame is a fixed box and neither
/// is what this component is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFit {
    /// Fill the frame, cropping the overflow. The Slint default.
    #[default]
    Cover,
    /// Fit inside the frame, letterboxing the remainder over the frame's fill.
    Contain,
    /// Stretch to the frame, distorting the aspect ratio.
    Fill,
}

impl ImageFit {
    fn content_fit(self) -> iced::ContentFit {
        match self {
            Self::Cover => iced::ContentFit::Cover,
            Self::Contain => iced::ContentFit::Contain,
            Self::Fill => iced::ContentFit::Fill,
        }
    }
}

/// A framed picture, built with the methods below. The frame is drawn even when
/// there is no source, so a missing or not-yet-loaded image never leaves an
/// empty hole in a layout.
pub struct ImageView<'a, Message> {
    source: Option<image::Handle>,
    fit: ImageFit,
    radius: Option<f32>,
    width: Length,
    height: Length,
    placeholder: Option<Element<'a, Message>>,
}

impl<'a, Message> ImageView<'a, Message> {
    /// A frame with no source: the muted, bordered box and the placeholder.
    pub fn new() -> Self {
        Self {
            source: None,
            fit: ImageFit::default(),
            radius: None,
            width: Length::Fill,
            height: Length::Fixed(160.0),
            placeholder: None,
        }
    }

    /// The picture. The handle is the caller's, because iced applications own
    /// their assets; see `crates/iced-component/README.md`.
    pub fn source(mut self, handle: impl Into<image::Handle>) -> Self {
        self.source = Some(handle.into());
        self
    }

    /// How the source is fitted into the frame.
    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    /// The corner radius in pixels, which rounds both the frame and the source
    /// inside it. Defaults to the theme's medium radius, the Slint
    /// `Theme.radius-md`.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    /// The frame's width. It fills its parent by default.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// The frame's height. 160 pixels by default, the Slint frame's preferred
    /// height; the caller sets it when the frame is a thumbnail or a banner.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// What the frame centres when it has no source. The element is the
    /// caller's, because the icon set belongs to the application; a bare frame
    /// with nothing to show draws its fill and border alone.
    pub fn placeholder(mut self, placeholder: impl Into<Element<'a, Message>>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }
}

impl<'a, Message> Default for ImageView<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<ImageView<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: ImageView<'a, Message>) -> Self {
        let ImageView {
            source,
            fit,
            radius,
            width,
            height,
            placeholder,
        } = control;
        let radius = match radius {
            Some(radius) if radius > 0.0 => radius,
            _ => theme::RADIUS_MD,
        };

        // The source is clipped to the same corner as the frame; a contain fit
        // leaves the frame's fill visible in the letterbox, which is what makes
        // the frame readable without the picture.
        let content: Element<'a, Message> = match source {
            Some(handle) => image(handle)
                .width(Length::Fill)
                .height(Length::Fill)
                .content_fit(fit.content_fit())
                .border_radius(radius)
                .into(),
            None => container(placeholder.unwrap_or_else(|| iced::widget::space().into()))
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into(),
        };

        // Every token is resolved inside the closure, so a theme switch repaints
        // the frame: resolving at build time is what once painted a dark panel
        // white.
        container(content)
            .width(width)
            .height(height)
            .clip(true)
            .style(move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                container::Style {
                    background: Some(Background::Color(tokens.muted)),
                    border: Border {
                        color: tokens.border,
                        width: 1.0,
                        radius: radius.into(),
                    },
                    ..container::Style::default()
                }
            })
            .into()
    }
}
