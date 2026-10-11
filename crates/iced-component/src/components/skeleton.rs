//! A loading placeholder: a muted, rounded block. Mirrors GPUI Component's
//! `Skeleton` and the Slint version beside it — at least 12 pixels tall, six
//! pixels of radius, and full width unless the caller says otherwise.
//!
//! **A static shape, at a caller-supplied pulse.** The Slint version and GPUI
//! both animate the block's opacity with a sine of the elapsed time. iced's
//! timer needs a runtime feature this crate does not enable, so the pulse is an
//! `f32` in `0.0..=1.0` that the caller advances from a subscription: zero is
//! the Slint animation's dimmest, one is its brightest, and the block renders
//! the same at any value. [`Skeleton::pulse`] maps the value to the alpha the
//! Slint version computes — `0.55 + 0.35 * pulse` over the muted colour.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{container, Space};
use iced::widget::container::Style as ContainerStyle;
use iced::{Border, Element, Length, Theme};

use super::theme::{self, Tokens};

/// A skeleton block, built with the methods below. It carries no message, but
/// it is generic over one so a caller can place it in any view without
/// annotating the type.
pub struct Skeleton<Message> {
    width: Length,
    height: Length,
    radius: f32,
    pulse: f32,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Skeleton<Message> {
    pub fn new() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fixed(12.0),
            radius: theme::RADIUS_MD,
            pulse: 1.0,
            _message: std::marker::PhantomData,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// The corner radius. Six pixels by default, `Length::Fill`-sized blocks
    /// often want half their height.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// How bright the block is, in `0.0..=1.0`, which the caller advances. One
    /// is the resting block and zero the dimmest the Slint animation reaches.
    pub fn pulse(mut self, pulse: f32) -> Self {
        self.pulse = pulse;
        self
    }

    /// The alpha the Slint version's sine produces at this pulse:
    /// `0.55 + 0.35 * pulse`.
    pub fn alpha(&self) -> f32 {
        (0.55 + 0.35 * self.pulse.clamp(0.0, 1.0)).clamp(0.0, 1.0)
    }
}

impl<Message> Default for Skeleton<Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The colours a skeleton block draws with.
pub struct Style {
    pub pulse: f32,
    pub radius: f32,
}

impl Style {
    pub fn new() -> Self {
        Self {
            pulse: 1.0,
            radius: theme::RADIUS_MD,
        }
    }

    pub fn pulse(mut self, pulse: f32) -> Self {
        self.pulse = pulse;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// The alpha at this pulse: `0.55 + 0.35 * pulse`.
    pub fn alpha(&self) -> f32 {
        (0.55 + 0.35 * self.pulse.clamp(0.0, 1.0)).clamp(0.0, 1.0)
    }

    pub fn resolve(&self, tokens: Tokens) -> ContainerStyle {
        ContainerStyle {
            background: Some(theme::alpha(tokens.muted, self.alpha()).into()),
            border: Border {
                radius: self.radius.into(),
                ..Border::default()
            },
            ..ContainerStyle::default()
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Skeleton<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Skeleton<Message>) -> Self {
        let radius = control.radius;
        let pulse = control.pulse;
        let width = control.width;
        let height = control.height;

        container(Space::new())
            .width(width)
            .height(height)
            .style(move |theme: &Theme| {
                Style::new()
                    .radius(radius)
                    .pulse(pulse)
                    .resolve(Tokens::of(theme))
            })
            .into()
    }
}
