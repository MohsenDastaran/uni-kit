//! A rotating loading indicator. Mirrors GPUI Component's `Spinner` and the
//! Slint version beside it: a 12, 14, 16 or 24 pixel arc in the foreground
//! colour or one the caller names.
//!
//! **The turn is the frame's own clock.** Slint rotates the `loader-circle`
//! glyph from `animation-tick()` — `mod(animation-tick(), 800ms) / 800ms *
//! 360deg`, a linear turn with no easing — and GPUI drives a `with_animation`.
//! iced has no interval subscription under this crate's runtime, so the spinner
//! keeps the clock in the widget tree instead: every frame it samples the
//! instant that frame carries in `RedrawRequested(at)`, and while it runs it
//! asks the shell for the next frame. That is `icon`'s spinning turn, with the
//! same [`SPIN_DURATION`] of 800ms and the same `draw_svg` rotation, drawn at
//! the size the caller asks for.
//!
//! [`Spinner::running(false)`] stops the turn. Because the motion comes from the
//! frame clock rather than a timer, a stopped spinner stops asking for frames
//! too — it costs nothing while it holds still. Resuming re-anchors the clock on
//! the frame it resumes on, so the arc continues from the angle on screen rather
//! than snapping back to zero.
//!
//! [`Spinner::progress`] is the determinate form: the caller pins the angle in
//! `0.0..=1.0` and no clock runs, the same split `icon` draws between `spin` and
//! `spinning`. [`Spinner::sweep`] is the arc's share of the ring, and the glyph
//! is built from it — 0.75, the default, is the `loader-circle` shape, and 1.0
//! draws a closed ring.
//!
//! **The turn cannot be suppressed.** This crate exposes no reduced-motion
//! preference (the collapsible's reveal, the skeleton and the shimmer carry the
//! same note), so the arc keeps turning where the platform asks for reduced
//! motion. A caller that needs a still indicator has
//! [`Spinner::progress`] and [`Spinner::running(false)`].
//!
//! The glyph is drawn through the SVG renderer rather than on a `Canvas`. The
//! first build of this animation did draw it on a `Canvas`, and the wasm backend
//! painted one of the eight spinners in the demo and left the rest blank — the
//! catalogue's fourth `Canvas` incident, and the reason `progress` and `speech`
//! draw their own geometry. The SVG renderer is also what the reference draws:
//! its spinner is an `Icon` with `spinning: true`, so the artwork is a glyph and
//! the rotation is a transform on it. iced has no icon set of its own, so the
//! component carries the one glyph it is rather than making an application ship
//! a spinning asset.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::advanced::layout::{self, Layout};
use iced::advanced::svg::Renderer as _;
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{Clipboard, Shell, renderer};
use iced::time::{Duration, Instant};
use iced::widget::svg;
use iced::{
    Color, ContentFit, Element, Event, Length, Point, Radians, Rectangle, Size as BoxSize, Theme,
    Vector, mouse,
};

use super::theme::{ControlSize, Tokens};

/// One full turn, the Slint authority's `800ms`: its spinner hands the rotation
/// to `Icon { spinning: true }`, which writes
/// `mod(animation-tick(), 800ms) / 800ms * 360deg` — elapsed time sampled
/// straight, so the curve is linear.
pub const SPIN_DURATION: Duration = Duration::from_millis(800);

/// A spinner, built with the methods below. It carries no message, but it is
/// generic over one so a caller can place it in any view without annotating the
/// type.
pub struct Spinner<Message> {
    size: ControlSize,
    color: Option<Color>,
    diameter: Option<f32>,
    sweep: f32,
    progress: Option<f32>,
    running: bool,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Spinner<Message> {
    pub fn new() -> Self {
        Self {
            size: ControlSize::default(),
            color: None,
            diameter: None,
            sweep: 0.75,
            progress: None,
            running: true,
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

    /// Pins the arc at `progress`, in `0.0..=1.0`, where one unit is one full
    /// turn. This is the determinate form: the component's clock does not run,
    /// so the caller owns the angle. Without it the spinner turns by itself,
    /// one turn every [`SPIN_DURATION`].
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = Some(progress);
        self
    }

    /// Whether the turn is running, the flag `icon`'s `spinning` is. `true`, the
    /// default, samples every frame and asks for the next one; `false` holds the
    /// angle on screen and asks for nothing, and turning it back on continues
    /// from that angle.
    pub fn running(mut self, running: bool) -> Self {
        self.running = running;
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

/// The `loader-circle` glyph at `sweep`: an open ring on a 24-unit grid,
/// stroked at Lucide's 2 with round caps, so the artwork scales the way the
/// Slint version's icon does. An arc whose ends meet draws nothing, so a full
/// sweep is a circle instead.
fn ring(sweep: f32) -> svg::Handle {
    let sweep = sweep.clamp(0.05, 1.0);
    let open = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d=""#;
    let close = r#"</svg>"#;
    let markup = if sweep >= 1.0 {
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><circle cx="12" cy="12" r="9"/></svg>"#
            .to_string()
    } else {
        // The arc starts at the top and runs clockwise, the direction Slint's
        // `transform-rotation` turns the glyph in.
        let start = -std::f32::consts::FRAC_PI_2;
        let end = start + sweep * std::f32::consts::TAU;
        let point = |angle: f32| (12.0 + 9.0 * angle.cos(), 12.0 + 9.0 * angle.sin());
        let (x1, y1) = point(start);
        let (x2, y2) = point(end);
        let large = u8::from(sweep > 0.5);
        format!("{open}M {x1:.3} {y1:.3} A 9 9 0 {large} 1 {x2:.3} {y2:.3}\"/>{close}")
    };

    svg::Handle::from_memory(markup.into_bytes())
}

/// The turn's clock, held in the widget tree rather than in the caller's state.
///
/// iced has no timer to subscribe to, so the motion is not stored as a tick but
/// as the angle on screen and the instant the frame that drew it carried: any
/// frame can ask where the arc should be, and the angle comes from the frame's
/// own instant rather than from whatever `Instant::now()` says this pass.
#[derive(Debug, Default)]
struct Rotation {
    /// The angle on screen, `0.0..=1.0`.
    phase: f32,
    /// The frame instant the phase was last sampled at.
    last: Option<Instant>,
    /// Whether the previous frame was running. A flag that turns back on
    /// re-anchors the clock instead of advancing across the pause.
    running: bool,
}

impl Rotation {
    /// Samples the turn at `now`, the clock `animation-tick()` gives Slint. Only
    /// a frame that was already running advances the angle, so a pause costs no
    /// time and resuming continues from the angle on screen.
    fn sample(&mut self, now: Instant, running: bool) {
        if running && self.running {
            if let Some(last) = self.last {
                let elapsed = now.saturating_duration_since(last).as_secs_f32();
                self.phase = (self.phase + elapsed / SPIN_DURATION.as_secs_f32()).fract();
            }
        }
        self.running = running;
        self.last = Some(now);
    }
}

/// The spinner as a widget, because the rotation is a draw-time value the
/// widget tree can carry. [`Spinner`] builds one; it is private because the
/// caller gets an `Element`.
struct Turning {
    glyph: svg::Handle,
    pixels: f32,
    color: Option<Color>,
    progress: Option<f32>,
    running: bool,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Turning {
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<Rotation>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(Rotation::default())
    }

    fn size(&self) -> BoxSize<Length> {
        BoxSize::new(Length::Fixed(self.pixels), Length::Fixed(self.pixels))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // The box is the caller's diameter, so a turn never moves its
        // neighbours and a row of sizes lines up.
        layout::Node::new(limits.resolve(
            Length::Fixed(self.pixels),
            Length::Fixed(self.pixels),
            BoxSize::new(self.pixels, self.pixels),
        ))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        // The determinate form has no clock at all, so a pinned arc asks for
        // nothing and repaints only when something else does.
        if self.progress.is_some() {
            return;
        }

        // The frame carries its own instant, so the angle drawn and the angle
        // sampled are the same frame's.
        if let Event::Window(iced::window::Event::RedrawRequested(at)) = event {
            tree.state
                .downcast_mut::<Rotation>()
                .sample(*at, self.running);
        }

        if self.running {
            // A spinner's motion never ends while it is active, so every frame
            // asks for the next one. This is the icon's `request_redraw`, with
            // the glyph rather than the mark.
            shell.request_redraw();
        }

        // Stopped: the arc holds its angle and no frame is requested, which is
        // what makes a paused spinner free rather than a still one that still
        // repaints.
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let turn = match self.progress {
            Some(progress) => progress,
            None => tree.state.downcast_ref::<Rotation>().phase,
        };
        let bounds = layout.bounds();

        // The icon's own draw: measure the glyph, fit it into the square box and
        // centre it, then rotate it about that centre.
        let image = renderer.measure_svg(&self.glyph);
        let image_size = BoxSize::new(image.width as f32, image.height as f32);
        let fit = ContentFit::Contain.fit(image_size, bounds.size());
        let scale = Vector::new(fit.width / image_size.width, fit.height / image_size.height);
        let final_size = image_size * scale;
        let position = Point::new(
            bounds.center_x() - final_size.width / 2.0,
            bounds.center_y() - final_size.height / 2.0,
        );

        renderer.draw_svg(
            iced::advanced::svg::Svg {
                handle: self.glyph.clone(),
                color: Some(self.color.unwrap_or(Tokens::of(theme).foreground)),
                rotation: Radians(turn * std::f32::consts::TAU),
                opacity: 1.0,
            },
            Rectangle::new(position, final_size),
            bounds,
        );
    }
}

impl<'a, Message> From<Spinner<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Spinner<Message>) -> Self {
        Element::new(Turning {
            glyph: ring(control.sweep),
            pixels: control.diameter.unwrap_or_else(|| control.width()),
            color: control.color,
            progress: control.progress,
            running: control.running,
        })
    }
}
