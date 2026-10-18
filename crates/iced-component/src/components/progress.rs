//! A linear progress bar and a circular one. Mirror GPUI Component's `Progress`
//! and `ProgressCircle` and the Slint versions beside them: a 0–1 value, a
//! status, sizes from 4 to 10 pixels tall for the bar and 8 to 20 across for
//! the circle, a loading form, and the custom height, radius and border GPUI's
//! `Styled` set gives the bar.
//!
//! **Determinate by value, not by a timer.** The Slint versions animate the
//! width and drive their loading form from `animation-tick()`. iced's timer
//! needs a runtime feature this crate does not enable, so the value is an
//! `f32` in `0.0..=1.0` that the caller advances from a subscription, and the
//! loading form takes the same value as its phase. The component renders the
//! same at any value, and at zero it draws only the track.
//!
//! **Both indicators are custom widgets that draw quads, not a `canvas`.** The
//! fill's width is a fraction of whatever space the bar is given, so it needs a
//! laid-out size; a `Quad` is the primitive a container proves on every frame.
//! The circle draws its ring as a rounded `Quad` whose stroke is the border, and
//! its arc as a run of overlapping round quads along the centreline, because
//! iced's `Transformation` has no rotation to turn a stroke with.
//!
//! The `canvas` versions were retired because a `Canvas` inside a region a height
//! animation moves **smears** under the web fallback: animating the collapsible's
//! height relayouts the tree every frame, and the bar came out as a stack of
//! stale rounded bars. It needs all three — the per-frame relayout, a `Canvas`
//! inside the moving region, and the wgpu-to-swiftshader fallback — and the quad
//! path renders correctly in all three.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::marker::PhantomData;

use iced::advanced::Renderer as _;
use iced::advanced::layout;
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{Layout, mouse};
use iced::{Background, Border, Color, Element, Length, Rectangle, Size, Theme};

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
    height: Option<f32>,
    radius: Option<f32>,
    border: f32,
    border_color: Option<Color>,
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
            height: None,
            radius: None,
            border: 0.0,
            border_color: None,
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

    /// A height for the track, replacing the size's. GPUI's `Styled::h`, which
    /// the page's custom-style sample sets to 16 pixels.
    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height.max(0.0));
        self
    }

    /// The corner radius, replacing the capsule of a bar with no custom height.
    /// GPUI's `Styled::rounded`.
    pub fn rounded(mut self, radius: f32) -> Self {
        self.radius = Some(radius.max(0.0));
        self
    }

    /// The outline width. GPUI's `Styled::border_2` and its siblings.
    pub fn border(mut self, width: f32) -> Self {
        self.border = width.max(0.0);
        self
    }

    /// The outline colour. A width with no colour draws no edge, the same way a
    /// GPUI `border_2` with no `border_color` falls back to its theme's.
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// The track height: 4 / 6 / 8 / 10 pixels for xsmall through large, or the
    /// height the caller named.
    pub fn girth(&self) -> f32 {
        self.height.unwrap_or(match self.size {
            ControlSize::XSmall => 4.0,
            ControlSize::Small => 6.0,
            ControlSize::Large => 10.0,
            ControlSize::Medium => 8.0,
        })
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

/// The bar's widget: the tinted track, and the fill over it.
struct Bar<Message> {
    status: Status,
    color: Option<Color>,
    loading: bool,
    value: f32,
    girth: f32,
    radius: Option<f32>,
    border: f32,
    border_color: Option<Color>,
    _message: PhantomData<Message>,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Bar<Message> {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(self.girth))
    }

    fn layout(
        &mut self,
        _tree: &mut widget::Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, Length::Fixed(self.girth))
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }

        // Every token is read when iced draws, so a theme switch moves the bar
        // with the page.
        let tokens = Tokens::of(theme);
        let tone = self.color.unwrap_or(match self.status {
            Status::Default => tokens.primary,
            status => tokens.status_color(status),
        });

        // The track is square and the value is a capsule over it, unless the
        // caller names a radius: then both take it. The defaults are the shape
        // the `canvas` version drew, kept so the two renderings diff to nothing.
        let track_radius = self.radius.unwrap_or(0.0).min(bounds.height / 2.0);
        renderer.fill_quad(
            Quad {
                bounds,
                border: iced::border::rounded(track_radius),
                ..Quad::default()
            },
            Background::Color(theme::alpha(tone, 0.2)),
        );

        let (x, width) = if self.loading {
            let phase = self.value.clamp(0.0, 1.0);
            let head = ease_in_out(((phase - 0.5) / 0.5).clamp(0.0, 1.0));
            let tail = ease_in_out(1.0 - phase);
            (
                bounds.width * head,
                (bounds.width * (1.0 - head - tail)).max(0.0),
            )
        } else {
            (0.0, bounds.width * self.value.clamp(0.0, 1.0))
        };

        if width > 0.0 {
            let fill_radius = self
                .radius
                .unwrap_or(bounds.height / 2.0)
                .min(bounds.height / 2.0);
            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: bounds.x + x,
                        y: bounds.y,
                        width,
                        height: bounds.height,
                    },
                    border: iced::border::rounded(fill_radius),
                    snap: false,
                    ..Quad::default()
                },
                Background::Color(tone),
            );
        }

        // The outline is drawn last so the fill cannot cover it. A width with no
        // colour draws nothing, which is what the default carries.
        if self.border > 0.0 {
            if let Some(color) = self.border_color {
                renderer.fill_quad(
                    Quad {
                        bounds,
                        border: Border {
                            color,
                            width: self.border,
                            radius: track_radius.into(),
                        },
                        snap: false,
                        ..Quad::default()
                    },
                    Background::Color(Color::TRANSPARENT),
                );
            }
        }
    }
}

impl<'a, Message> From<Progress<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Progress<Message>) -> Self {
        let girth = control.girth();
        Element::new(Bar {
            status: control.status,
            color: control.color,
            loading: control.loading,
            value: control.fraction(),
            girth,
            radius: control.radius,
            border: control.border,
            border_color: control.border_color,
            _message: PhantomData,
        })
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

/// The circle's widget: a stroked track, and the arc over it.
struct Circle<Message> {
    status: Status,
    color: Option<Color>,
    loading: bool,
    value: f32,
    diameter: f32,
    _message: PhantomData<Message>,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Circle<Message> {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.diameter), Length::Fixed(self.diameter))
    }

    fn layout(
        &mut self,
        _tree: &mut widget::Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(
            limits,
            Length::Fixed(self.diameter),
            Length::Fixed(self.diameter),
        )
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }

        let tokens = Tokens::of(theme);
        let tone = self.color.unwrap_or(match self.status {
            Status::Default => tokens.primary,
            status => tokens.status_color(status),
        });
        let diameter = bounds.width.min(bounds.height);
        let stroke = (diameter * 0.15).min(5.0);
        let center = bounds.center();
        let radius = ((diameter - stroke) / 2.0).max(0.0);

        // The track: a circle whose border is the stroke, the ring `Quad` can
        // draw without a path.
        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: center.x - diameter / 2.0,
                    y: center.y - diameter / 2.0,
                    width: diameter,
                    height: diameter,
                },
                border: Border {
                    color: theme::alpha(tone, 0.2),
                    width: stroke,
                    radius: (diameter / 2.0).into(),
                },
                snap: false,
                ..Quad::default()
            },
            Background::Color(Color::TRANSPARENT),
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
            // The Slint arc starts at the top and sweeps clockwise.
            arc(renderer, center, radius, stroke, start, end, tone);
        }
    }
}

/// One arc of the ring: round quads along the centreline, sampled closely
/// enough that their outer edges read as one stroke and the round end caps the
/// `canvas` version drew come for free. iced's `Transformation` has no
/// rotation, so a turned stroke has no primitive but a dot.
fn arc(
    renderer: &mut iced::Renderer,
    center: iced::Point,
    radius: f32,
    stroke: f32,
    start: f32,
    end: f32,
    color: Color,
) {
    let sweep = (end - start) * std::f32::consts::TAU;
    let step = (stroke * 0.4).max(0.5);
    let samples = ((radius * sweep) / step).ceil().max(1.0) as usize;
    let dot = iced::border::rounded(stroke / 2.0);

    for index in 0..=samples {
        let t = index as f32 / samples as f32;
        let angle = start * std::f32::consts::TAU + t * sweep - std::f32::consts::FRAC_PI_2;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();

        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    x: x - stroke / 2.0,
                    y: y - stroke / 2.0,
                    width: stroke,
                    height: stroke,
                },
                border: dot,
                snap: false,
                ..Quad::default()
            },
            Background::Color(color),
        );
    }
}

impl<'a, Message> From<ProgressCircle<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: ProgressCircle<Message>) -> Self {
        let diameter = control.diameter.unwrap_or_else(|| control.width());
        Element::new(Circle {
            status: control.status,
            color: control.color,
            loading: control.loading,
            value: control.value.clamp(0.0, 1.0),
            diameter,
            _message: PhantomData,
        })
    }
}
