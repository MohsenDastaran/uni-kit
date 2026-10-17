//! A shimmer: a moving highlight that sweeps through the label's own ink, the
//! loading treatment GPUI Component's `ShimmerText` and the Slint `ShimmerText`
//! both animate.
//!
//! The text keeps its layout as one run and the colour varies along it: the
//! ink is the label's base colour where the band is not, and the highlight
//! colour where it is, so the highlight travels *through the glyphs* rather than
//! behind them. The run is split into [`SPAN_COUNT`] spans, laid out as one
//! paragraph, and each span's colour is mixed from its own position against the
//! sweep. A single `text` carries one colour, so spans are the only way to
//! colour part of a run without giving up shaping and kerning — splitting the
//! label into a widget per character would lay each glyph out on its own.
//!
//! **The sweep is a clock owned by the widget tree.** iced 0.14 has no interval
//! subscription under this crate's runtime, but an element that implements
//! `Widget` keeps state in the tree and can ask for frames itself: while the
//! label is visible it calls `shell.request_redraw()`, and every frame samples
//! the frame's own instant to place the highlight. This is the spinner's shape
//! rather than the collapsible reveal's — the motion never ends while the
//! component is shown — and it is why the demo moves without a `Task`, a
//! subscription or a button. One full sweep takes [`DEFAULT_DURATION`], the
//! Slint authority's `2s`, and the curve is linear because Slint's animation
//! clock samples the progress straight from elapsed time.
//!
//! The highlight advances from left to right by default and [`Shimmer::reverse`]
//! sweeps it the other way, exactly as the Slint `reverse` property does.
//! [`Shimmer::once`] plays a single sweep and stops at the end; the state is
//! keyed by position, so [`Shimmer::restart_on`] carries the caller's replay
//! counter and starts the sweep again when it changes.
//!
//! **The default highlight moves the ink 70% of the way toward the page**, the
//! distance Slint's light-theme default uses (`background.mix(color, 0.7)`,
//! which weights the receiver). Slint's dark-theme default instead mixes 60%
//! toward the foreground, which is a no-op for a foreground-coloured label and
//! only 1.5:1 against a muted one — present but imperceptible, the failure this
//! catalogue keeps finding. Moving toward the page in both appearances measures
//! **9.8:1 and 3.2:1 in light** and **7.5:1 and 4.7:1 in dark** for the
//! foreground and muted labels the example draws, and a caller can still pass
//! its own [`Shimmer::highlight`] — the accent in the demo's timing sample.
//!
//! **The sweep cannot be suppressed.** This crate exposes no reduced-motion
//! preference (the collapsible's reveal and the skeleton have the same note), so
//! the ink keeps moving even where the platform asks for reduced motion. A
//! caller that needs a still rendering should draw the label's ordinary text
//! instead — the module has no builder that switches the clock off.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::advanced::layout::{self, Layout};
use iced::advanced::text::{self, Paragraph as _, Renderer as _, Span, Text};
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::time::{Duration, Instant};
use iced::{Color, Element, Event, Font, Length, Pixels, Rectangle, Size, Theme, alignment};

use super::theme::{self, Tokens};

/// One complete sweep, the Slint authority's `duration: 2s`.
pub const DEFAULT_DURATION: Duration = Duration::from_secs(2);

/// The highlight's relative half-width the caller gets when it sets none,
/// Slint's `spread: 0.3`.
pub const DEFAULT_SPREAD: f32 = 0.3;

/// How many spans the label is cut into, at most.
///
/// Twelve is the handful that makes the ramp read as a moving band: at the
/// default spread the band covers 60% of the label, so about seven spans sit
/// inside it and each step between neighbours is roughly a quarter of the
/// transition. A label shorter than twelve characters gets one span per
/// character, and the chunks stay inside a single paragraph, so the whole run is
/// still shaped and kerned together.
pub const SPAN_COUNT: usize = 12;

/// How far the default highlight moves the base ink toward the page: the
/// distance Slint's light-theme default uses, `0.7` of the way. It is measured
/// from the ink rather than from the page, so a muted label gains its band too.
const DEFAULT_HIGHLIGHT: f32 = 0.7;

/// A shimmer label, built with the methods below. It carries no message, but it
/// is generic over one so a caller can place it in any view without annotating
/// the type.
pub struct Shimmer<Message> {
    text: String,
    duration: Duration,
    spread: f32,
    spread_width: Option<f32>,
    reverse: bool,
    once: bool,
    restart_on: u64,
    size: f32,
    font: Font,
    color: Option<fn(&Tokens) -> Color>,
    highlight: Option<fn(&Tokens) -> Color>,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Shimmer<Message> {
    /// A label whose sweep starts when it is first shown. The default is a
    /// looping, two-second, left-to-right sweep, the Slint authority's own.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            duration: DEFAULT_DURATION,
            spread: DEFAULT_SPREAD,
            spread_width: None,
            reverse: false,
            once: false,
            restart_on: 0,
            size: theme::FONT_SIZE_SM,
            font: Font::DEFAULT,
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

    /// The highlight's half-width as a fraction of the label, clamped to
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

    /// The label's size in pixels, Slint's `font-size`.
    pub fn size(mut self, size: f32) -> Self {
        if size.is_finite() && size > 0.0 {
            self.size = size;
        }
        self
    }

    /// The label's weight, Slint's `font-weight`.
    pub fn weight(mut self, weight: iced::font::Weight) -> Self {
        self.font = Font {
            weight,
            ..self.font
        };
        self
    }

    /// The base ink the label draws in where the band is not. The default is
    /// the theme's foreground, Slint's `color`. It is a function of the tokens
    /// because the theme is only known when iced draws.
    pub fn color(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.color = Some(color);
        self
    }

    /// The colour the band's centre takes. The default moves the base ink 70%
    /// of the way toward the page, so the band reads against both a foreground
    /// and a muted label in both appearances. Like [`Shimmer::color`], it is
    /// resolved when the ink is drawn.
    pub fn highlight(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.highlight = Some(color);
        self
    }

    /// The base ink and the highlight this label draws with.
    fn resolve(&self, tokens: Tokens) -> (Color, Color) {
        resolve(self.color, self.highlight, tokens)
    }
}

impl<Message> Default for Shimmer<Message> {
    fn default() -> Self {
        Self::new("")
    }
}

/// The base ink and the highlight, from the caller's choices or the defaults.
fn resolve(
    color: Option<fn(&Tokens) -> Color>,
    highlight: Option<fn(&Tokens) -> Color>,
    tokens: Tokens,
) -> (Color, Color) {
    let base = color
        .map(|color| color(&tokens))
        .unwrap_or(tokens.foreground);
    let highlight = highlight
        .map(|color| color(&tokens))
        .unwrap_or_else(|| default_highlight(tokens, base));
    (base, highlight)
}

/// The base ink brought 70% of the way toward the page background.
///
/// Slint's light arm does exactly this from the page's side
/// (`background.mix(color, 0.7)`); the dark arm mixes toward the foreground,
/// which leaves a foreground label unlit and a muted one at 1.5:1. Moving the
/// ink toward the page reads as a band in both appearances, and it always moves
/// away from the label's own colour rather than toward it.
fn default_highlight(tokens: Tokens, base: Color) -> Color {
    theme::mix(base, tokens.background, DEFAULT_HIGHLIGHT)
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

/// The paragraph this widget draws: the renderer's own type, kept alive from
/// `draw` until it flushes the frame.
type Drawn = <iced::Renderer as text::Renderer>::Paragraph;

/// The element that carries [`Sweep`] in the widget tree and draws the label.
struct Ink<Message> {
    text: String,
    duration: Duration,
    spread: f32,
    spread_width: Option<f32>,
    reverse: bool,
    once: bool,
    restart_on: u64,
    size: f32,
    font: Font,
    color: Option<fn(&Tokens) -> Color>,
    highlight: Option<fn(&Tokens) -> Color>,
    /// The paragraph the last `draw` handed the renderer.
    ///
    /// `fill_paragraph` records a *weak* reference to the paragraph and resolves
    /// it when the frame is flushed, so a paragraph built as a local in `draw`
    /// is gone by then and nothing paints. Keeping it here — the element lives
    /// until the next view rebuild, which is after the flush — is what makes the
    /// ink visible. Forgetting this is a widget that lays out and draws nothing.
    drawn: std::cell::RefCell<Option<Drawn>>,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Ink<Message> {
    /// Where the band's centre sits along the label, as a fraction of its
    /// width: `0.0` is the leading edge, `1.0` the trailing one.
    fn band(&self, progress: f32) -> f32 {
        let half = self.spread;
        let travel = -half + progress * (1.0 + 2.0 * half);
        if self.reverse { 1.0 - travel } else { travel }
    }

    /// The half-width as a fraction of the label, from `spread` or from the
    /// fixed `spread-width` against the measured label width.
    fn half_width(&self, width: f32) -> f32 {
        match self.spread_width {
            Some(spread_width) => spread_width / width.max(1.0),
            None => self.spread,
        }
    }

    /// The label's spans, coloured by where each chunk sits against the band.
    fn spans<'a>(
        &'a self,
        base: Color,
        highlight: Color,
        progress: f32,
        width: f32,
    ) -> Vec<Span<'a, (), Font>> {
        let chunks = chunks(&self.text, SPAN_COUNT);
        let count = chunks.len().max(1) as f32;
        let half = self.half_width(width).max(0.001);
        let band = self.band(progress);

        chunks
            .into_iter()
            .enumerate()
            .map(|(index, chunk)| {
                // The chunk's nearest point to the band, not its centre: the
                // colour a span can show is the strongest the ramp has inside
                // it, so the span the band is crossing takes the highlight
                // itself and its neighbours fade toward the base ink. Sampling
                // chunk centres would leave the peak a fraction short of the
                // highlight whenever the band falls between two centres.
                let start = index as f32 / count;
                let end = (index + 1) as f32 / count;
                let distance = (band.clamp(start, end) - band).abs();
                let t = if distance >= half {
                    0.0
                } else {
                    1.0 - distance / half
                };

                Span::new(chunk)
                    .size(self.size)
                    .font(self.font)
                    .color(theme::mix(base, highlight, t))
            })
            .collect()
    }
}

/// The label cut into at most `count` chunks at character boundaries, spread
/// evenly over the string. Real glyph widths differ, so a chunk's centre is an
/// approximation of its position along the line — which is what a gradient over
/// the run would have said too, and why the band's edges are soft.
fn chunks(text: &str, count: usize) -> Vec<&str> {
    let boundaries: Vec<usize> = text
        .char_indices()
        .map(|(index, _)| index)
        .chain(std::iter::once(text.len()))
        .collect();
    let characters = boundaries.len().saturating_sub(1);
    if characters == 0 {
        return Vec::new();
    }

    let spans = count.clamp(1, characters);
    (0..spans)
        .map(|index| {
            let start = boundaries[index * characters / spans];
            let end = boundaries[(index + 1) * characters / spans];
            &text[start..end]
        })
        .filter(|chunk| !chunk.is_empty())
        .collect()
}

/// One paragraph from `spans`, laid out on one line at the caller's size and
/// font. Drawing it is what carries the per-span colours to the glyphs.
fn paragraph<'a>(
    spans: &[Span<'a, (), Font>],
    size: f32,
    font: Font,
    bounds: Size,
) -> <iced::Renderer as text::Renderer>::Paragraph {
    <iced::Renderer as text::Renderer>::Paragraph::with_spans(Text {
        content: spans,
        bounds,
        size: Pixels(size),
        line_height: text::LineHeight::default(),
        font,
        align_x: alignment::Horizontal::Left.into(),
        align_y: alignment::Vertical::Top,
        shaping: text::Shaping::Advanced,
        // Slint's `ShimmerText` defaults to `no-wrap` with `elide`.
        wrapping: text::Wrapping::None,
    })
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Ink<Message> {
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<Sweep>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(Sweep::new(self.restart_on))
    }

    fn size(&self) -> Size<Length> {
        // The label owns its own size, as Slint's `Text` does; the caller places
        // it rather than stretching a track under it.
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // Colours do not affect metrics, so the measurement is one uncoloured
        // span; `draw` is where the run is cut up and coloured.
        let measured = paragraph(
            &[Span::new(self.text.as_str())
                .size(self.size)
                .font(self.font)],
            self.size,
            self.font,
            limits.max(),
        )
        .min_bounds();

        layout::sized(limits, Length::Shrink, Length::Shrink, |_| measured)
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
            // The motion never ends while the label is visible, so every frame
            // asks for the next one. This is the spinner's request, with the
            // ink rather than a mark.
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _defaults: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.width < 1.0 || bounds.height < 1.0 || !bounds.intersects(viewport) {
            return;
        }

        // Every token is read when iced draws, so a theme switch recolours the
        // ink with the page.
        let tokens = Tokens::of(theme);
        let (base, highlight) = resolve(self.color, self.highlight, tokens);
        let progress = tree.state.downcast_ref::<Sweep>().progress;
        let spans = self.spans(base, highlight, progress, bounds.width);
        let paragraph = paragraph(&spans, self.size, self.font, bounds.size());

        renderer.fill_paragraph(&paragraph, bounds.position(), base, *viewport);
        *self.drawn.borrow_mut() = Some(paragraph);
    }
}

impl<'a, Message> From<Shimmer<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Shimmer<Message>) -> Self {
        Element::new(Ink {
            text: control.text,
            duration: control.duration,
            spread: control.spread,
            spread_width: control.spread_width,
            reverse: control.reverse,
            once: control.once,
            restart_on: control.restart_on,
            size: control.size,
            font: control.font,
            color: control.color,
            highlight: control.highlight,
            drawn: std::cell::RefCell::new(None),
            _message: std::marker::PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::theme::{DARK, LIGHT};

    /// The WCAG relative luminance of an opaque colour.
    fn luminance(color: Color) -> f32 {
        let channel = |value: f32| {
            if value <= 0.03928 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * channel(color.r) + 0.7152 * channel(color.g) + 0.0722 * channel(color.b)
    }

    fn contrast(a: Color, b: Color) -> f32 {
        let (a, b) = (luminance(a), luminance(b));
        (a.max(b) + 0.05) / (a.min(b) + 0.05)
    }

    #[test]
    fn test_shimmer_highlight_reads_in_both_themes() {
        // The band is only present if the eye can see it, and the two labels the
        // example draws in each appearance are the ones that have to read.
        for tokens in [LIGHT, DARK] {
            for base in [tokens.foreground, tokens.muted_foreground] {
                let ratio = contrast(base, default_highlight(tokens, base));
                assert!(ratio >= 3.0, "highlight is {ratio:.2}:1 against {base:?}");
            }
        }
    }

    #[test]
    fn test_shimmer_spans_cover_the_text_once() {
        assert_eq!(chunks("", SPAN_COUNT), Vec::<&str>::new());
        assert_eq!(chunks("Hi", SPAN_COUNT).concat(), "Hi");
        assert_eq!(chunks("Thinking…", SPAN_COUNT).len(), 9);
        assert_eq!(chunks("Thinking…", SPAN_COUNT).concat(), "Thinking…");

        let long = "The assistant is thinking about the open issues…";
        let cut = chunks(long, SPAN_COUNT);
        assert_eq!(cut.len(), SPAN_COUNT);
        assert_eq!(cut.concat(), long);
        // Character boundaries, not byte boundaries, or a multi-byte label
        // would panic on the slice.
        assert_eq!(
            chunks("Läbeln mit Ümlauten", SPAN_COUNT).concat(),
            "Läbeln mit Ümlauten"
        );
    }
}
