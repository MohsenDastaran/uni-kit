//! A shimmer: a moving highlight that sweeps across a muted band, the loading
//! treatment GPUI Component's `ShimmerText` and the Slint `ShimmerText` both
//! animate.
//!
//! iced cannot paint a gradient through part of a text run, and a canvas cannot
//! draw the caller's text, so what this draws is the band itself rather than the
//! text under it: a track in the base colour with the highlight travelling
//! across it. That is the same motion at the same fractions as the reference,
//! and it is the closest an iced component can come without owning the text's
//! shaping. A caller who wants the text as well places [`label`] over the band
//! with `stack`, which is what the demo does.
//!
//! **The sweep is a clock owned by the widget tree.** iced 0.14 has no interval
//! subscription under this crate's runtime, but an element that implements
//! `Widget` keeps state in the tree and can ask for frames itself: while the
//! band is visible it calls `shell.request_redraw()`, and every frame samples
//! the frame's own instant to place the highlight. This is the spinner's shape
//! rather than the collapsible reveal's — the motion never ends while the
//! component is shown — and it is why the demo moves without a `Task`, a
//! subscription or a button. One full sweep takes [`DEFAULT_DURATION`], the
//! Slint authority's `2s`, and the curve is linear because Slint's animation
//! clock samples the progress straight from elapsed time.
//!
//! The band advances from left to right by default and [`Shimmer::reverse`]
//! sweeps it the other way, exactly as the Slint `reverse` property does.
//! [`Shimmer::once`] plays a single sweep and stops at the end; the state is
//! keyed by position, so [`Shimmer::restart_on`] carries the caller's replay
//! counter and starts the sweep again when it changes.
//!
//! **The sweep cannot be suppressed.** This crate exposes no reduced-motion
//! preference (the collapsible's reveal has the same note), so the band keeps
//! moving even where the platform asks for reduced motion. A caller that needs
//! a still rendering should draw the band's end state itself — the module has no
//! builder that switches the clock off.

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
use iced::widget::text;
use iced::{Background, Border, Color, Element, Event, Length, Rectangle, Size, Theme};

use super::theme::{self, Tokens};

/// One complete sweep, the Slint authority's `duration: 2s`.
pub const DEFAULT_DURATION: Duration = Duration::from_secs(2);

/// The highlight's relative half-width the caller gets when it sets none,
/// Slint's `spread: 0.3`.
pub const DEFAULT_SPREAD: f32 = 0.3;

/// A shimmer band, built with the methods below. It carries no message, but it
/// is generic over one so a caller can place it in any view without annotating
/// the type.
pub struct Shimmer<Message> {
    duration: Duration,
    spread: f32,
    spread_width: Option<f32>,
    reverse: bool,
    once: bool,
    restart_on: u64,
    height: f32,
    color: Option<fn(&Tokens) -> Color>,
    highlight: Option<fn(&Tokens) -> Color>,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Shimmer<Message> {
    /// A band that starts its sweep when it is first shown. The default is a
    /// looping, two-second, left-to-right sweep, the Slint authority's own.
    pub fn new() -> Self {
        Self {
            duration: DEFAULT_DURATION,
            spread: DEFAULT_SPREAD,
            spread_width: None,
            reverse: false,
            once: false,
            restart_on: 0,
            height: theme::FONT_SIZE_SM,
            color: None,
            highlight: None,
            _message: std::marker::PhantomData,
        }
    }

    /// One complete sweep. Slint clamps this to at least one millisecond, so a
    /// zero does not stop the clock.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration.max(Duration::from_millis(1));
        self
    }

    /// The highlight's half-width as a fraction of the band, clamped to
    /// `0.05..=1.0` as Slint's `spread` is. This is ignored when
    /// [`Shimmer::spread_width`] is set.
    pub fn spread(mut self, spread: f32) -> Self {
        if spread.is_finite() {
            self.spread = spread.clamp(0.05, 1.0);
        }
        self
    }

    /// A fixed half-width in pixels, Slint's `spread-width`: the band keeps the
    /// same physical size across labels of different lengths.
    pub fn spread_width(mut self, width: f32) -> Self {
        if width.is_finite() {
            self.spread_width = Some(width.max(1.0));
        }
        self
    }

    /// Sweeps the other way, Slint's `reverse`.
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// Plays one sweep and stops instead of looping, Slint's `once`.
    pub fn once(mut self, once: bool) -> Self {
        self.once = once;
        self
    }

    /// Starts the sweep again whenever this value changes. The caller keeps a
    /// counter and increments it to replay a `once` sweep; the same value leaves
    /// the motion alone.
    pub fn restart_on(mut self, generation: u64) -> Self {
        self.restart_on = generation;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// The base colour the band sits on. The default is the theme's muted
    /// surface. It is a function of the tokens because the theme is only known
    /// when iced draws.
    pub fn color(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.color = Some(color);
        self
    }

    /// The highlight itself. The default is theme-aware: the track moved 20%
    /// toward the foreground, so the label over it stays readable. Like
    /// [`Shimmer::color`], it is resolved when the band is drawn.
    pub fn highlight(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.highlight = Some(color);
        self
    }
}

impl<Message> Default for Shimmer<Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The colours a shimmer draws its track and its highlight with.
pub struct Style {
    pub color: Option<fn(&Tokens) -> Color>,
    pub highlight: Option<fn(&Tokens) -> Color>,
}

impl Style {
    pub fn new() -> Self {
        Self {
            color: None,
            highlight: None,
        }
    }

    pub fn color(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn highlight(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.highlight = Some(color);
        self
    }

    /// The track and the highlight the band paints with.
    ///
    /// The default highlight moves the track 20% toward the foreground, the
    /// same mix GPUI's `ShimmerText` gives its highlight. Slint can light the
    /// text itself; here the band sits behind the caller's label, so a highlight
    /// as bright as the foreground would erase the label as it passes. Twenty
    /// percent stays visible against the muted track in both appearances while
    /// leaving the foreground readable over it.
    pub fn resolve(&self, tokens: Tokens) -> (Color, Color) {
        let base = self
            .color
            .map(|color| color(&tokens))
            .unwrap_or(tokens.muted);
        let highlight = self
            .highlight
            .map(|color| color(&tokens))
            .unwrap_or_else(|| theme::mix(base, tokens.foreground, 0.2));
        (base, highlight)
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// The sweep's clock, held in the widget tree instead of the caller's state.
///
/// iced has no timer to subscribe to, so the motion is not stored as a tick but
/// as the instant the sweep started: any frame can ask where the highlight
/// should be, and the fraction comes from the frame's own instant rather than
/// from whatever `Instant::now()` says this pass.
#[derive(Debug)]
struct Sweep {
    /// The frame the current sweep started on. `None` until the first frame.
    started: Option<Instant>,
    /// The sampled position, `0.0..=1.0`.
    progress: f32,
    /// A `once` sweep that has reached its end stops asking for frames.
    finished: bool,
    /// The caller's replay counter the current sweep belongs to.
    restart_on: u64,
}

impl Sweep {
    fn new(restart_on: u64) -> Self {
        Self {
            started: None,
            progress: 0.0,
            finished: false,
            restart_on,
        }
    }

    fn start(&mut self, now: Instant) {
        self.started = Some(now);
        self.progress = 0.0;
        self.finished = false;
    }

    /// Samples the sweep at `now`, and answers whether there is more of it to
    /// come. The curve is linear because Slint samples `animation-tick()`
    /// straight into the progress.
    fn sample(&mut self, now: Instant, duration: Duration, once: bool) -> bool {
        let Some(started) = self.started else {
            self.start(now);
            return true;
        };
        let elapsed = now.saturating_duration_since(started).as_secs_f32();
        let seconds = duration.as_secs_f32().max(0.001);
        let turns = elapsed / seconds;
        if once {
            self.progress = turns.min(1.0);
            self.finished = self.progress >= 1.0;
        } else {
            self.progress = turns.fract();
        }
        !self.finished
    }
}

/// The element that carries [`Sweep`] in the widget tree and paints the band.
struct Band<Message> {
    duration: Duration,
    spread: f32,
    spread_width: Option<f32>,
    reverse: bool,
    once: bool,
    restart_on: u64,
    height: f32,
    color: Option<fn(&Tokens) -> Color>,
    highlight: Option<fn(&Tokens) -> Color>,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Band<Message> {
    fn band(&self, progress: f32, width: f32) -> f32 {
        let half = match self.spread_width {
            Some(spread_width) => spread_width / width.max(1.0),
            None => self.spread,
        };
        let travel = -half + progress * (1.0 + 2.0 * half);
        let band = if self.reverse { 1.0 - travel } else { travel };
        band * width
    }

    fn half_width(&self, width: f32) -> f32 {
        match self.spread_width {
            Some(spread_width) => spread_width / width.max(1.0),
            None => self.spread,
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Band<Message> {
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<Sweep>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(Sweep::new(self.restart_on))
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(self.height))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, Length::Fixed(self.height))
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
        let sweep = tree.state.downcast_mut::<Sweep>();
        if sweep.restart_on != self.restart_on {
            sweep.restart_on = self.restart_on;
            sweep.start(now);
        }
        if sweep.sample(now, self.duration, self.once) {
            // The motion never ends while the band is visible, so every frame
            // asks for the next one. This is the spinner's request, with the
            // band rather than a mark.
            shell.request_redraw();
        }
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

        // Every token is read when iced draws, so a theme switch moves the band
        // with the page.
        let tokens = Tokens::of(theme);
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

        renderer.fill_quad(
            Quad {
                bounds,
                border: Border::default(),
                ..Quad::default()
            },
            Background::Color(base),
        );

        // The highlight is a gradient in the Slint version: opaque at the band,
        // fading to the base across the spread on each side. Drawing it as a
        // run of one-pixel columns reproduces that fade without depending on a
        // renderer's gradient support.
        let width = bounds.width;
        let half = self.half_width(width);
        let band = self.band(tree.state.downcast_ref::<Sweep>().progress, width);
        let span = (half * width).max(1.0);
        let columns = width.ceil() as usize;

        for column in 0..columns {
            let x = column as f32;
            let distance = (x - band).abs();
            if distance >= span {
                continue;
            }
            let t = 1.0 - distance / span;
            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: bounds.x + x,
                        y: bounds.y,
                        width: 1.0,
                        height: bounds.height,
                    },
                    border: Border::default(),
                    ..Quad::default()
                },
                Background::Color(theme::mix(base, highlight, t)),
            );
        }
    }
}

impl<'a, Message> From<Shimmer<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Shimmer<Message>) -> Self {
        Element::new(Band {
            duration: control.duration,
            spread: control.spread,
            spread_width: control.spread_width,
            reverse: control.reverse,
            once: control.once,
            restart_on: control.restart_on,
            height: control.height,
            color: control.color,
            highlight: control.highlight,
            _message: std::marker::PhantomData,
        })
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
