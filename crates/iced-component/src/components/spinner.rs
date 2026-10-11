//! A rotating loading indicator. Mirrors GPUI Component's `Spinner` and the
//! Slint version beside it: a 12, 14, 16 or 24 pixel arc in the foreground
//! colour or one the caller names.
//!
//! **Determinate by value, not by a timer.** Slint rotates the icon from
//! `animation-tick()`, and GPUI drives a `with_animation`. iced's timer needs a
//! runtime feature this crate does not enable, so the progress is an `f32` in
//! `0.0..=1.0` that the caller advances from a subscription and the arc's
//! starting angle follows it. The spinner renders the same at any value, and it
//! is drawn on a `canvas` rather than as an icon, because iced has no icon set
//! of its own and an application would otherwise have to supply a spinning
//! asset.
//!
//! The Slint version takes the glyph's name, defaulting to `loader-circle`.
//! This draws that glyph's shape — an open ring — directly.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::canvas::{self, Canvas, Frame, Path};
use iced::widget::canvas::path::arc::Arc;
use iced::{Color, Degrees, Element, Length, Radians, Renderer, Theme};
use iced::{mouse, Rectangle};

use super::theme::{ControlSize, Tokens};

/// A spinner, built with the methods below. It carries no message, but it is
/// generic over one so a caller can place it in any view without annotating the
/// type.
pub struct Spinner<Message> {
    size: ControlSize,
    color: Option<Color>,
    diameter: Option<f32>,
    sweep: f32,
    progress: f32,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Spinner<Message> {
    pub fn new() -> Self {
        Self {
            size: ControlSize::default(),
            color: None,
            diameter: None,
            sweep: 0.75,
            progress: 0.0,
            _message: std::marker::PhantomData,
        }
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The arc's colour. The default is the theme's foreground.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// A custom diameter, which the Slint version takes from the element's own
    /// width.
    pub fn diameter(mut self, diameter: f32) -> Self {
        self.diameter = Some(diameter);
        self
    }

    /// How much of the ring the arc covers, in `0.0..=1.0`. The default is
    /// three quarters, which is the shape of the `loader-circle` glyph. A full
    /// turn draws a closed ring, which stops reading as a spinner.
    pub fn sweep(mut self, sweep: f32) -> Self {
        self.sweep = sweep;
        self
    }

    /// The arc's position, in `0.0..=1.0`, which the caller advances. One full
    /// turn is one unit.
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress;
        self
    }

    /// The diameter: 12 / 14 / 16 / 24 pixels for xsmall through large.
    pub fn width(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 12.0,
            ControlSize::Small => 14.0,
            ControlSize::Large => 24.0,
            ControlSize::Medium => 16.0,
        }
    }

    fn stroke(&self) -> f32 {
        (self.width() * 0.15).clamp(1.0, 3.0)
    }
}

impl<Message> Default for Spinner<Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The colours and geometry a spinner draws with.
pub struct Style {
    pub color: Option<Color>,
    pub diameter: f32,
    pub sweep: f32,
}

impl Style {
    pub fn new(diameter: f32) -> Self {
        Self {
            color: None,
            diameter,
            sweep: 0.75,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn sweep(mut self, sweep: f32) -> Self {
        self.sweep = sweep;
        self
    }

    pub fn resolve(&self, tokens: Tokens) -> Color {
        self.color.unwrap_or(tokens.foreground)
    }

    /// The stroke: 15% of the diameter, kept between one and three pixels.
    pub fn stroke(&self) -> f32 {
        (self.diameter * 0.15).clamp(1.0, 3.0)
    }
}

/// The spinner's program: an open arc that starts where the progress puts it.
struct Ring {
    color: Option<Color>,
    sweep: f32,
    progress: f32,
}

impl<Message> canvas::Program<Message> for Ring {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let tokens = Tokens::of(theme);
        let mut frame = Frame::new(renderer, bounds.size());
        let diameter = frame.width().min(frame.height());
        let stroke = (diameter * 0.15).clamp(1.0, 3.0);
        let center = frame.center();
        let radius = ((diameter - stroke) / 2.0).max(0.0);
        let start = self.progress.fract() * 360.0 - 90.0;
        let end = start + self.sweep.clamp(0.05, 1.0) * 360.0;

        let arc = Path::new(|builder| {
            builder.arc(Arc {
                center,
                radius,
                start_angle: Radians::from(Degrees::from(start)),
                end_angle: Radians::from(Degrees::from(end)),
            });
        });

        frame.stroke(
            &arc,
            canvas::Stroke::default()
                .with_color(self.color.unwrap_or(tokens.foreground))
                .with_width(stroke)
                .with_line_cap(canvas::LineCap::Round),
        );

        vec![frame.into_geometry()]
    }
}

impl<'a, Message> From<Spinner<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Spinner<Message>) -> Self {
        let diameter = control.diameter.unwrap_or_else(|| control.width());
        let program = Ring {
            color: control.color,
            sweep: control.sweep,
            progress: control.progress,
        };

        Canvas::new(program)
            .width(Length::Fixed(diameter))
            .height(Length::Fixed(diameter))
            .into()
    }
}
