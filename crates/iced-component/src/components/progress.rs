//! A linear progress bar and a circular one. Mirror GPUI Component's `Progress`
//! and `ProgressCircle` and the Slint versions beside them: a 0–1 value, a
//! status, sizes from 4 to 10 pixels tall for the bar and 8 to 20 across for
//! the circle, and a loading form.
//!
//! **Determinate by value, not by a timer.** The Slint versions animate the
//! width and drive their loading form from `animation-tick()`. iced's timer
//! needs a runtime feature this crate does not enable, so the value is an
//! `f32` in `0.0..=1.0` that the caller advances from a subscription, and the
//! loading form takes the same value as its phase. The component renders the
//! same at any value, and at zero it draws only the track.
//!
//! The bar is drawn on a `canvas` rather than as two containers, because the
//! fill's width is a fraction of whatever space the bar is given and a widget
//! does not know that width until it is laid out. The circle uses the same
//! `canvas` for its arc.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::canvas::{self, Canvas, Frame, Path};
use iced::widget::canvas::path::arc::Arc;
use iced::{Color, Element, Length, Renderer, Theme};
use iced::{mouse, Rectangle};

use super::theme::{self, ControlSize, Status, Tokens};

/// A linear progress bar, built with the methods below. It carries no message,
/// but it is generic over one so a caller can place it in any view without
/// annotating the type.
pub struct Progress<Message> {
    value: f32,
    loading: bool,
    size: ControlSize,
    status: Status,
    color: Option<Color>,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Progress<Message> {
    /// A bar at `value` in `0.0..=1.0`.
    pub fn new(value: f32) -> Self {
        Self {
            value,
            loading: false,
            size: ControlSize::default(),
            status: Status::default(),
            color: None,
            _message: std::marker::PhantomData,
        }
    }

    /// The indeterminate form: the value is the phase of a band that travels
    /// across the track. The caller advances it.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The status colour the fill takes. A bar with a status ignores the
    /// default, which is the theme's primary colour.
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// The track height: 4 / 6 / 8 / 10 pixels for xsmall through large.
    pub fn girth(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 4.0,
            ControlSize::Small => 6.0,
            ControlSize::Large => 10.0,
            ControlSize::Medium => 8.0,
        }
    }

    fn tone(&self, tokens: Tokens) -> Color {
        self.color.unwrap_or(match self.status {
            Status::Default => tokens.primary,
            status => tokens.status_color(status),
        })
    }

    fn fraction(&self) -> f32 {
        self.value.clamp(0.0, 1.0)
    }

    /// The band the loading form draws, as the Slint version computes it: the
    /// eased phase at each end, so the band grows out of the left edge and is
    /// swallowed by the right one.
    fn band(&self) -> (f32, f32) {
        let phase = self.fraction();
        let head = ease_in_out(((phase - 0.5) / 0.5).clamp(0.0, 1.0));
        let tail = ease_in_out(1.0 - phase);
        (head, (1.0 - head - tail).max(0.0))
    }
}

/// The easing the Slint versions use for their loading forms.
pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (2.0 - 2.0 * t) * (2.0 - 2.0 * t) / 2.0
    }
}

/// The colours a bar or a circle draws with.
pub struct Style {
    pub status: Status,
    pub color: Option<Color>,
    pub girth: f32,
}

impl Style {
    pub fn new() -> Self {
        Self {
            status: Status::default(),
            color: None,
            girth: 8.0,
        }
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn girth(mut self, girth: f32) -> Self {
        self.girth = girth;
        self
    }

    pub fn resolve(&self, tokens: Tokens) -> Color {
        self.color.unwrap_or(match self.status {
            Status::Default => tokens.primary,
            status => tokens.status_color(status),
        })
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// The bar's program: the tinted track, and the fill over it.
struct Bar {
    status: Status,
    color: Option<Color>,
    loading: bool,
    value: f32,
}

impl<Message> canvas::Program<Message> for Bar {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let tokens = Tokens::of(theme);
        let tone = self.color.unwrap_or(match self.status {
            Status::Default => tokens.primary,
            status => tokens.status_color(status),
        });
        let radius = frame.height() / 2.0;

        frame.fill_rectangle(
            iced::Point::ORIGIN,
            frame.size(),
            theme::alpha(tone, 0.2),
        );

        let fill = if self.loading {
            let phase = self.value.clamp(0.0, 1.0);
            let head = ease_in_out(((phase - 0.5) / 0.5).clamp(0.0, 1.0));
            let tail = ease_in_out(1.0 - phase);
            let width = (frame.width() * (1.0 - head - tail)).max(0.0);
            Some((frame.width() * head, width))
        } else {
            Some((0.0, frame.width() * self.value.clamp(0.0, 1.0)))
        };

        if let Some((x, width)) = fill {
            if width > 0.0 {
                let path = Path::rounded_rectangle(
                    iced::Point::new(x, 0.0),
                    iced::Size::new(width, frame.height()),
                    radius.into(),
                );
                frame.fill(&path, tone);
            }
        }

        vec![frame.into_geometry()]
    }
}

impl<'a, Message> From<Progress<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Progress<Message>) -> Self {
        let girth = control.girth();
        let program = Bar {
            status: control.status,
            color: control.color,
            loading: control.loading,
            value: control.fraction(),
        };

        Canvas::new(program)
            .width(Length::Fill)
            .height(Length::Fixed(girth))
            .into()
    }
}

/// A circular progress indicator, built with the methods below. It carries no
/// message, but it is generic over one so a caller can place it in any view
/// without annotating the type.
pub struct ProgressCircle<Message> {
    value: f32,
    loading: bool,
    size: ControlSize,
    status: Status,
    color: Option<Color>,
    diameter: Option<f32>,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> ProgressCircle<Message> {
    /// A circle at `value` in `0.0..=1.0`. The arc is drawn clockwise from the
    /// top over a tinted track.
    pub fn new(value: f32) -> Self {
        Self {
            value,
            loading: false,
            size: ControlSize::default(),
            status: Status::default(),
            color: None,
            diameter: None,
            _message: std::marker::PhantomData,
        }
    }

    /// The indeterminate form: the value is the phase of an arc that grows and
    /// turns. The caller advances it.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

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

    /// The circle's diameter: 8 / 12 / 16 / 20 for xsmall through large.
    pub fn width(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 8.0,
            ControlSize::Small => 12.0,
            ControlSize::Large => 20.0,
            ControlSize::Medium => 16.0,
        }
    }

    fn tone(&self, tokens: Tokens) -> Color {
        self.color.unwrap_or(match self.status {
            Status::Default => tokens.primary,
            status => tokens.status_color(status),
        })
    }

    /// The stroke: 15% of the diameter, never more than five pixels.
    fn stroke(&self) -> f32 {
        (self.width() * 0.15).min(5.0)
    }
}

/// The circle's program: a stroked track, and the arc over it.
struct Circle {
    status: Status,
    color: Option<Color>,
    loading: bool,
    value: f32,
}

impl<Message> canvas::Program<Message> for Circle {
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
        let tone = self.color.unwrap_or(match self.status {
            Status::Default => tokens.primary,
            status => tokens.status_color(status),
        });
        let mut frame = Frame::new(renderer, bounds.size());
        let diameter = frame.width().min(frame.height());
        let stroke = (diameter * 0.15).min(5.0);
        let center = frame.center();
        let radius = ((diameter - stroke) / 2.0).max(0.0);

        frame.stroke(
            &Path::circle(center, radius),
            canvas::Stroke::default()
                .with_color(theme::alpha(tone, 0.2))
                .with_width(stroke),
        );

        let (start, end) = if self.loading {
            let phase = self.value.clamp(0.0, 1.0);
            (
                ease_in_out(((phase - 0.5) / 0.5).clamp(0.0, 1.0)),
                ease_in_out(phase),
            )
        } else {
            (0.0, self.value.clamp(0.0, 1.0))
        };

        if end > start {
            let arc = Path::new(|builder| {
                builder.arc(Arc {
                    center,
                    radius,
                    // The Slint arc starts at the top and sweeps clockwise.
                    start_angle: iced::Radians::from(iced::Degrees::from(start * 360.0 - 90.0)),
                    end_angle: iced::Radians::from(iced::Degrees::from(end * 360.0 - 90.0)),
                });
            });

            frame.stroke(
                &arc,
                canvas::Stroke::default()
                    .with_color(tone)
                    .with_width(stroke)
                    .with_line_cap(canvas::LineCap::Round),
            );
        }

        vec![frame.into_geometry()]
    }
}

impl<'a, Message> From<ProgressCircle<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: ProgressCircle<Message>) -> Self {
        let diameter = control.diameter.unwrap_or_else(|| control.width());
        let program = Circle {
            status: control.status,
            color: control.color,
            loading: control.loading,
            value: control.value.clamp(0.0, 1.0),
        };

        Canvas::new(program)
            .width(Length::Fixed(diameter))
            .height(Length::Fixed(diameter))
            .into()
    }
}
