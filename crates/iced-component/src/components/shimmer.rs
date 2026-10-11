//! A shimmer: the moving highlight GPUI Component's `ShimmerText` sweeps across
//! a line of loading text.
//!
//! **Determinate by value, not by a timer.** Slint drives its gradient from
//! `animation-tick()` and a `Timer`. iced's timer needs a runtime feature this
//! crate does not enable, so the progress is an `f32` in `0.0..=1.0` that the
//! caller advances from a subscription, and the band's position follows it. The
//! component renders the same at any value.
//!
//! iced cannot paint a gradient through part of a text run, and a canvas cannot
//! draw the caller's text, so what this draws is the band itself rather than the
//! text under it: a track in the base colour with the highlight travelling
//! across it. That is the same motion at the same fractions, and it is the
//! closest an iced component can come without owning the text's shaping. The
//! module doc says so because the shape is a deliberate difference from the
//! Slint original, not an oversight: a caller who wants a shimmering *label*
//! draws its text over the band with `stack`, or uses [`Shimmer::text`], which
//! returns a label the caller can place on top of the same element.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::canvas::{self, Canvas, Frame};
use iced::widget::text;
use iced::{Color, Element, Length, Renderer, Theme};
use iced::{mouse, Rectangle};

use super::theme::{self, Tokens};

/// A shimmer band, built with the methods below. It carries no message, but it
/// is generic over one so a caller can place it in any view without annotating
/// the type.
pub struct Shimmer<Message> {
    progress: f32,
    spread: f32,
    reverse: bool,
    height: f32,
    color: Option<Color>,
    highlight: Option<Color>,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Shimmer<Message> {
    /// A band whose highlight sits at `progress` in `0.0..=1.0`. One full sweep
    /// is one unit; a caller that wants the sweep to repeat takes the fraction
    /// itself.
    pub fn new(progress: f32) -> Self {
        Self {
            progress,
            spread: 0.3,
            reverse: false,
            height: theme::FONT_SIZE_SM,
            color: None,
            highlight: None,
            _message: std::marker::PhantomData,
        }
    }

    /// The highlight's half-width as a fraction of the band.
    pub fn spread(mut self, spread: f32) -> Self {
        self.spread = spread;
        self
    }

    /// Sweeps the other way.
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// The base colour the band sits on. The default is the theme's muted
    /// surface.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// The highlight itself. The default is the base mixed most of the way to
    /// the foreground in the dark theme, and to the page in the light one,
    /// which is the pair the Slint version computes.
    pub fn highlight(mut self, color: Color) -> Self {
        self.highlight = Some(color);
        self
    }

    /// The travelling band's position, in `0.0..=1.0`.
    pub fn band(&self) -> f32 {
        let progress = self.progress.clamp(0.0, 1.0);
        let travel = -self.spread + progress * (1.0 + 2.0 * self.spread);
        if self.reverse {
            1.0 - travel
        } else {
            travel
        }
    }
}

impl<Message> Default for Shimmer<Message> {
    fn default() -> Self {
        Self::new(0.0)
    }
}

/// The colours a shimmer draws its track and its highlight with.
pub struct Style {
    pub color: Option<Color>,
    pub highlight: Option<Color>,
}

impl Style {
    pub fn new() -> Self {
        Self {
            color: None,
            highlight: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn highlight(mut self, color: Color) -> Self {
        self.highlight = Some(color);
        self
    }

    pub fn resolve(&self, tokens: Tokens) -> (Color, Color) {
        let base = self.color.unwrap_or(tokens.muted);
        let highlight = self.highlight.unwrap_or(if tokens.dark {
            theme::mix(tokens.foreground, base, 0.6)
        } else {
            theme::mix(tokens.background, base, 0.7)
        });
        (base, highlight)
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// The band's program: the base track, and the highlight over it.
struct Band {
    color: Option<Color>,
    highlight: Option<Color>,
    band: f32,
    spread: f32,
}

impl<Message> canvas::Program<Message> for Band {
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
        let style = Style::new();
        let style = match self.color {
            Some(color) => style.color(color),
            None => style,
        };
        let style = match self.highlight {
            Some(highlight) => style.highlight(highlight),
            None => style,
        };
        let (base, highlight) = style.resolve(tokens);

        frame.fill_rectangle(iced::Point::ORIGIN, frame.size(), base);

        // The highlight is a gradient in the Slint version: opaque at the band,
        // fading to the base across the spread on each side. Drawing it as a
        // run of one-pixel columns reproduces that fade without depending on a
        // renderer's gradient support.
        let width = frame.width().max(1.0);
        let half = self.spread.clamp(0.05, 1.0);
        let band = self.band * width;
        let span = half * width;
        let columns = width.ceil() as usize;

        for column in 0..columns {
            let x = column as f32;
            let distance = (x - band).abs();
            if distance >= span {
                continue;
            }
            let t = 1.0 - distance / span;
            frame.fill_rectangle(
                iced::Point::new(x, 0.0),
                iced::Size::new(1.0, frame.height()),
                theme::mix(base, highlight, t),
            );
        }

        vec![frame.into_geometry()]
    }
}

impl<'a, Message> From<Shimmer<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Shimmer<Message>) -> Self {
        let height = control.height;
        let program = Band {
            color: control.color,
            highlight: control.highlight,
            band: control.band(),
            spread: control.spread,
        };

        Canvas::new(program)
            .width(Length::Fill)
            .height(Length::Fixed(height))
            .into()
    }
}

/// A label in the shimmer's own colours, for a caller that wants the text as
/// well as the band and places it over [`Shimmer`] with a `stack`.
pub fn text_ink(tokens: Tokens) -> Color {
    tokens.foreground
}

/// A text label the caller can place over a shimmer band.
pub fn label<'a, Message: 'a>(label: impl Into<String>) -> Element<'a, Message> {
    text(label.into())
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| iced::widget::text::Style {
            color: Some(Tokens::of(theme).foreground),
        })
        .into()
}
