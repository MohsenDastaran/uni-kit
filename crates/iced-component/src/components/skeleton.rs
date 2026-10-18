//! A loading placeholder: a muted, rounded block that pulses while it is
//! visible. Mirrors GPUI Component's `Skeleton` and the Slint version beside
//! it — at least 12 pixels tall, six pixels of radius, and full width unless
//! the caller says otherwise.
//!
//! **The pulse is a clock owned by the widget tree.** Slint computes the block's
//! opacity from `animation-tick()`; iced 0.14 has no interval subscription under
//! this crate's runtime, but a widget keeps state in the tree and can ask for
//! frames itself. While the block is shown it calls `shell.request_redraw()`,
//! and every frame samples the frame's own instant to place the pulse. That is
//! the spinner's shape rather than the collapsible reveal's: the motion never
//! ends while the component is shown, so the demo moves without a `Task`, a
//! subscription or a button.
//!
//! The values are the Slint authority's: one 1400ms cycle, and the alpha
//! `0.55 + 0.35 * abs(sin(phase * 360deg))` over the muted colour. Because the
//! sine is folded through `abs`, the block reaches its dimmest every 700ms and
//! its brightest halfway between — 0.55 to 0.90 and back twice per cycle. The
//! reference has this one animation and no wave: a wave is a travelling offset,
//! a different effect with a different clock, and this component does not sweep
//! one.
//!
//! **The pulse cannot be suppressed.** This crate exposes no reduced-motion
//! preference (the collapsible's reveal and the shimmer have the same note), so
//! the block keeps moving even where the platform asks for reduced motion. The
//! reference says the same: the animation is essential for indicating loading
//! state, and there is no builder that switches the clock off.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{Clipboard, Renderer, Shell};
use iced::time::{Duration, Instant};
use iced::widget::container::Style as ContainerStyle;
use iced::{Border, Element, Event, Length, Rectangle, Size, Theme};

use super::theme::{self, Tokens};

/// One pulse cycle, the Slint authority's `1400ms`.
pub const PULSE_DURATION: Duration = Duration::from_millis(1400);

/// The dimmest alpha the Slint version reaches, its `0.55`.
pub const PULSE_MIN: f32 = 0.55;

/// How far the sine carries the alpha above the dimmest, its `0.35`.
pub const PULSE_SPAN: f32 = 0.35;

/// A skeleton block, built with the methods below. It carries no message, but
/// it is generic over one so a caller can place it in any view without
/// annotating the type.
pub struct Skeleton<Message> {
    width: Length,
    height: Length,
    radius: f32,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Skeleton<Message> {
    pub fn new() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fixed(12.0),
            radius: theme::RADIUS_MD,
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
}

impl<Message> Default for Skeleton<Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The colours a skeleton block draws with.
pub struct Style {
    pub radius: f32,
}

impl Style {
    pub fn new() -> Self {
        Self {
            radius: theme::RADIUS_MD,
        }
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// The alpha at a pulse phase in `0.0..=1.0`, the Slint version's
    /// `0.55 + 0.35 * abs(sin(phase * 360deg))`.
    pub fn alpha(phase: f32) -> f32 {
        let wave = (phase.clamp(0.0, 1.0) * std::f32::consts::TAU).sin().abs();
        (PULSE_MIN + PULSE_SPAN * wave).clamp(0.0, 1.0)
    }

    pub fn resolve(&self, tokens: Tokens, phase: f32) -> ContainerStyle {
        ContainerStyle {
            background: Some(theme::alpha(tokens.muted, Self::alpha(phase)).into()),
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

/// The pulse's clock, held in the widget tree instead of the caller's state.
///
/// iced has no timer to subscribe to, so the motion is not stored as a tick but
/// as the instant the block first drew: any frame can ask where the pulse is,
/// and the phase comes from the frame's own instant rather than from whatever
/// `Instant::now()` says this pass.
#[derive(Debug)]
struct Pulse {
    /// The frame the pulse started on. `None` until the first frame.
    started: Option<Instant>,
    /// The sampled phase, `0.0..=1.0`.
    phase: f32,
}

impl Pulse {
    fn new() -> Self {
        Self {
            started: None,
            phase: 0.0,
        }
    }

    /// Samples the pulse at `now`. The curve is the Slint version's folded sine:
    /// one `PULSE_DURATION` is one turn of the phase.
    fn sample(&mut self, now: Instant) -> f32 {
        let started = *self.started.get_or_insert(now);
        let elapsed = now.saturating_duration_since(started).as_secs_f32();
        self.phase = (elapsed / PULSE_DURATION.as_secs_f32()).fract();
        self.phase
    }
}

/// The element that carries [`Pulse`] in the widget tree and paints the block.
struct Block<Message> {
    width: Length,
    height: Length,
    radius: f32,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Block<Message> {
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<Pulse>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(Pulse::new())
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let frame = match event {
            Event::Window(iced::window::Event::RedrawRequested(at)) => Some(*at),
            _ => None,
        };
        let now = frame.unwrap_or_else(Instant::now);
        tree.state.downcast_mut::<Pulse>().sample(now);

        // The pulse never ends while the block is visible, so every frame asks
        // for the next one. This is the spinner's request, with the block
        // rather than a mark.
        shell.request_redraw();
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }

        // Every token is read when iced draws, so a theme switch recolours the
        // block with the page.
        let resolved = Style::new()
            .radius(self.radius)
            .resolve(Tokens::of(theme), tree.state.downcast_ref::<Pulse>().phase);

        if let Some(background) = resolved.background {
            renderer.fill_quad(
                Quad {
                    bounds,
                    border: resolved.border,
                    ..Quad::default()
                },
                background,
            );
        }
    }
}

impl<'a, Message> From<Skeleton<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Skeleton<Message>) -> Self {
        Element::new(Block {
            width: control.width,
            height: control.height,
            radius: control.radius,
            _message: std::marker::PhantomData,
        })
    }
}
