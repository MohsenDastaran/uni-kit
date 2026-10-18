//! `speech` — dictation controls: the microphone button that starts and stops a
//! session, and the live waveform of the input levels the host pushes.
//!
//! The Slint authority is `crates/slint-component/ui/speech.slint`, with the
//! example beside it in `ui/examples/speech.slint`; the GPUI original is the
//! `speech` module, `crates/component/src/speech/`. This port follows the Slint
//! rendering, because that is the design the two galleries share:
//!
//! - **[`SpeechStatus`]** is where a session is, with the two questions a layout
//!   asks of it: [`SpeechStatus::is_active`] and [`SpeechStatus::is_capturing`],
//!   Slint's `Speech.is-active()` / `Speech.is-capturing()`.
//! - **[`SpeechButton`]** is the control: a ghost icon button — a microphone at
//!   rest, a square while it captures, a disabled button while the final result
//!   is pending — with the label as its tooltip, the way the Slint button carries
//!   its accessible name. It draws nothing while the host reports no recognizer,
//!   so an application can place it unconditionally.
//! - **[`SpeechWaveform`]** is the meter: one bar per level the host pushes, the
//!   newest against the trailing edge, silence as a dot, primary while capture
//!   runs and muted otherwise.
//!
//! **This is not a [`Bubble`](super::bubble), and it is not built on one.** A
//! bubble is a chat surface with an edge, a tail and a reaction region; speech is
//! a control plus a level meter, and it shares none of that surface. `bubble.rs`
//! was read first, as the brief asked, and there is nothing in it to compose
//! here: the two happen to be neighbours in the catalog, not shapes of one
//! another.
//!
//! **The session is the host's, exactly as in Slint.** iced captures no audio and
//! recognizes no speech here, so the component is three inputs and one callback:
//! `status`, `has_recognizer`, `available`, and `on_toggle(active)`. The
//! application runs its own recognizer, moves `status` as the session advances,
//! pushes the levels, and writes the transcript where the text belongs. There is
//! no `SpeechState` on this side; the state a GPUI application gets from the
//! crate is the state an iced application owns itself.
//!
//! Three deliberate differences from the Slint source, all because iced 0.14 has
//! no equivalent:
//!
//! - **The level transition is a step, not a glide.** Slint animates each bar's
//!   height over `interval` with a linear `animate`. iced has no such transition,
//!   so the bar is drawn at the level it was handed. The meter still stays live —
//!   it asks for the next frame while capture runs, the way the GPUI original
//!   redraws every frame while capturing — and the module does not take an
//!   `interval`: the host's push rate is the host's own constant, and there is no
//!   transition for it to time.
//! - **The width is a number, not a laid-out length.** Slint reads `root.width`
//!   to decide how many bars fit, so a caller can size the waveform with any
//!   layout. iced measures an element only after it is built, so
//!   [`SpeechWaveform::width`] is how a caller says how wide it is; it defaults
//!   to Slint's 96 pixels, and the bar count follows it.
//! - **The tooltip carries the label; there is no accessibility tree.** GPUI
//!   passes an `accessibility_label` and Slint passes `accessible-label`; iced
//!   0.14 exposes neither, so the label is drawn as the button's tooltip, composed
//!   from [`Tooltip`](super::tooltip), and the missing accessible name is named
//!   here rather than faked.
//!
//! The two Lucide marks are the ones the Slint icon set names (`mic` and
//! `square`, Lucide, ISC), embedded the way [`Clipboard`](super::clipboard)
//! embeds its pair, so the three galleries draw the same glyphs.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one control
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::marker::PhantomData;
use std::time::Duration;

use iced::advanced::layout;
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{self, Widget};
use iced::advanced::{Clipboard, Renderer as _, Shell};
use iced::widget::{Space, svg, text};
use iced::{Background, Border, Element, Length, Rectangle, Size, Theme};
use iced::{mouse, window};

use super::button::{self, Button, Variant};
use super::theme::{self, ControlSize, Tokens};
use super::tooltip::Tooltip;

/// The width of a waveform the caller does not size, Slint's `96px`.
pub const DEFAULT_WIDTH: f32 = 96.0;

/// How long one level covers, the rate GPUI's meter pushes at. A waveform asks
/// for the next frame this far ahead while it captures.
const LEVEL_INTERVAL: Duration = Duration::from_millis(80);

/// Where a session is. The four values and their spellings are the Slint
/// `SpeechStatus`, which are GPUI's `SpeechStatus`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpeechStatus {
    /// No session is running.
    #[default]
    Idle,
    /// Audio is captured while the recognizer connects.
    Connecting,
    /// Audio is captured and recognized.
    Recording,
    /// Capture stopped; waiting for the final result.
    Stopping,
}

impl SpeechStatus {
    /// Whether a session is running at all — true for every status but
    /// [`Idle`](Self::Idle), the question Slint's `Speech.is-active` answers.
    pub fn is_active(self) -> bool {
        !matches!(self, Self::Idle)
    }

    /// Whether audio is being captured — true while connecting and while
    /// recording, the question Slint's `Speech.is-capturing` answers.
    pub fn is_capturing(self) -> bool {
        matches!(self, Self::Connecting | Self::Recording)
    }
}

/// The label the Slint button draws when a session can start.
const START_LABEL: &str = "Start dictation";
/// The label the Slint button draws while capture runs.
const STOP_LABEL: &str = "Stop dictation";
/// The label the Slint button draws when the recognizer cannot work.
const UNAVAILABLE_LABEL: &str = "Speech input is not available";

/// The microphone control: starts and stops a dictation session.
///
/// It is a ghost icon button — a microphone at rest, a square while capturing,
/// and disabled while the final result is pending — and it draws **nothing**
/// while [`has_recognizer`](Self::has_recognizer) is false, unless
/// [`show_when_unsupported`](Self::show_when_unsupported) is set, so an
/// application can place it unconditionally.
pub struct SpeechButton<'a, Message> {
    status: SpeechStatus,
    has_recognizer: bool,
    available: bool,
    show_when_unsupported: bool,
    disabled: bool,
    size: ControlSize,
    start_label: String,
    stop_label: String,
    unavailable_label: String,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message> SpeechButton<'a, Message> {
    /// A microphone at the leading defaults: idle, a recognizer present and
    /// available, shown only while one is.
    pub fn new() -> Self {
        Self {
            status: SpeechStatus::default(),
            has_recognizer: true,
            available: true,
            show_when_unsupported: false,
            disabled: false,
            size: ControlSize::default(),
            start_label: START_LABEL.to_string(),
            stop_label: STOP_LABEL.to_string(),
            unavailable_label: UNAVAILABLE_LABEL.to_string(),
            on_toggle: None,
        }
    }

    /// Where the session is. Slint's `status`.
    pub fn status(mut self, status: SpeechStatus) -> Self {
        self.status = status;
        self
    }

    /// Whether this build has a recognizer and an input at all, the question
    /// GPUI's `has_recognizer()` answers. While it is false nothing is drawn,
    /// unless [`show_when_unsupported`](Self::show_when_unsupported) is set.
    pub fn has_recognizer(mut self, has_recognizer: bool) -> Self {
        self.has_recognizer = has_recognizer;
        self
    }

    /// Whether the recognizer works right now, the question GPUI's
    /// `is_available()` answers. The button stays drawn, and is disabled with the
    /// unavailable label.
    pub fn available(mut self, available: bool) -> Self {
        self.available = available;
        self
    }

    /// Draw a disabled button, instead of nothing, when there is no recognizer.
    pub fn show_when_unsupported(mut self, show_when_unsupported: bool) -> Self {
        self.show_when_unsupported = show_when_unsupported;
        self
    }

    /// Disable it for application reasons, such as a readonly field.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The control's size. The icon box follows it, 12 / 14 / 16 / 24, the same
    /// ladder [`Button`] gives an icon.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The tooltip at rest. Slint's `start-label`.
    pub fn start_label(mut self, label: impl Into<String>) -> Self {
        self.start_label = label.into();
        self
    }

    /// The tooltip while capture runs. Slint's `stop-label`.
    pub fn stop_label(mut self, label: impl Into<String>) -> Self {
        self.stop_label = label.into();
        self
    }

    /// The tooltip when the recognizer cannot work. Slint's `unavailable-label`.
    pub fn unavailable_label(mut self, label: impl Into<String>) -> Self {
        self.unavailable_label = label.into();
        self
    }

    /// What a press reports: `true` to start a session, `false` to stop the one
    /// that is running, which is the value of Slint's `toggled(active)`. Without
    /// it the control is drawn disabled, the way every control in this crate
    /// spells "cannot be pressed".
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }
}

impl<'a, Message> Default for SpeechButton<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<SpeechButton<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'static,
{
    fn from(control: SpeechButton<'a, Message>) -> Self {
        let SpeechButton {
            status,
            has_recognizer,
            available,
            show_when_unsupported,
            disabled,
            size,
            start_label,
            stop_label,
            unavailable_label,
            on_toggle,
        } = control;

        // Slint hides the element, and a hidden element takes no room. iced has
        // no hidden element, so this is a zero-sized space: it draws nothing and
        // measures nothing, which is what `visible: false` is.
        if !has_recognizer && !show_when_unsupported {
            return Space::new()
                .width(Length::Fixed(0.0))
                .height(Length::Fixed(0.0))
                .into();
        }

        let capturing = status.is_capturing();
        // A running session stays stoppable even if the recognizer turns
        // unavailable mid-way, the exception the Slint button makes too.
        let usable = status.is_active() || available;
        let interactive = !disabled && usable && status != SpeechStatus::Stopping;
        let label = if !usable {
            unavailable_label
        } else if capturing {
            stop_label
        } else {
            start_label
        };

        // The ink is resolved inside the draw closure, because the appearance is
        // only known when iced asks. An SVG tinted once at build time keeps its
        // own black — the colour it was born with — through the dark theme. A
        // disabled control dims to the muted ink, the same emphasis the button's
        // own label takes.
        let mark = svg(svg::Handle::from_memory(if capturing {
            SQUARE
        } else {
            MIC
        }))
        .width(size.icon_size())
        .height(size.icon_size())
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(if interactive {
                button::ink(Tokens::of(theme), Variant::Ghost, false)
            } else {
                Tokens::of(theme).muted_foreground
            }),
        });

        let button = Button::new("")
            .variant(Variant::Ghost)
            .size(size)
            .selected(capturing)
            .loading(status == SpeechStatus::Stopping)
            .icon(mark);

        let button = match (interactive, on_toggle) {
            (true, Some(on_toggle)) => button.on_press(on_toggle(!capturing)),
            _ => button,
        };

        // The label is the accessible name in Slint and GPUI, and iced carries
        // no accessibility tree; the tooltip is where a reader sees it here.
        Tooltip::new(button)
            .label(text(label).size(theme::FONT_SIZE_SM))
            .into()
    }
}

/// A live waveform of the input levels the host pushes.
///
/// Each level is a bar that grows up and down from the midline; silence is a
/// dot. The newest level enters at the trailing edge and the older ones sit
/// toward the leading edge, so a short history grows from the right. Before any
/// audio, nothing is drawn.
///
/// Bars thicken with the waveform so a tall one does not read as hairline, and
/// stand apart so a run of silence reads as a row of dots. Far fewer bars fit
/// than levels can be pushed, so hand it a window of the recent levels rather
/// than a whole session's history; only those that fit are drawn, newest last.
///
/// Size it like any element with [`size`](Self::size) and
/// [`width`](Self::width); it is 96 pixels wide by default. While
/// [`capturing`](Self::capturing) is true the waveform asks for the next frame
/// so the newest level is painted as it arrives.
pub struct SpeechWaveform<Message> {
    levels: Vec<f32>,
    capturing: bool,
    size: ControlSize,
    width: f32,
    _message: PhantomData<Message>,
}

impl<Message> SpeechWaveform<Message> {
    /// An empty waveform at the middle size, 96 pixels wide.
    pub fn new() -> Self {
        Self {
            levels: Vec::new(),
            capturing: false,
            size: ControlSize::default(),
            width: DEFAULT_WIDTH,
            _message: PhantomData,
        }
    }

    /// The recent levels, oldest first, in `0.0..=1.0`: the order a session
    /// pushes them in.
    pub fn levels(mut self, levels: impl IntoIterator<Item = f32>) -> Self {
        self.levels = levels.into_iter().collect();
        self
    }

    /// Whether the host is capturing: colours the bars primary, and keeps the
    /// waveform asking for frames. Slint's `capturing`.
    pub fn capturing(mut self, capturing: bool) -> Self {
        self.capturing = capturing;
        self
    }

    /// The waveform's size. The height follows it — 12 / 16 / 20 / 24, GPUI's
    /// four heights.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The waveform's width in pixels. Slint reads its laid-out width; iced
    /// measures after the fact, so the caller states it here. It decides how
    /// many bars fit.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// The bar height for a size, GPUI's ladder.
    fn height(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 12.0,
            ControlSize::Small => 16.0,
            ControlSize::Medium => 20.0,
            ControlSize::Large => 24.0,
        }
    }
}

impl<Message> Default for SpeechWaveform<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<SpeechWaveform<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: SpeechWaveform<Message>) -> Self {
        let height = control.height();
        Element::new(Bars {
            levels: control.levels,
            capturing: control.capturing,
            width: control.width,
            height,
            _message: PhantomData,
        })
    }
}

/// The bars the waveform draws.
///
/// It is a widget rather than a row of containers because the count and the
/// height of every bar depend on the width the caller gave it, which a row of
/// explicitly sized containers cannot express without the caller repeating the
/// arithmetic. It draws each bar as a quad — the same primitive a container
/// uses — rather than through iced's `canvas`, and asks for the next frame while
/// capture runs so the newest level is painted as it arrives.
///
/// The quad is deliberate. A `canvas` version of this meter drew nothing once it
/// sat in the field's suffix, where the demo needs it: the same `Bars` program
/// rendered in the section below and produced no geometry in the row above, at a
/// size the layout had already resolved. A quad is the path a container proves
/// on every frame, and it draws the same bars in both places.
struct Bars<Message> {
    levels: Vec<f32>,
    capturing: bool,
    width: f32,
    height: f32,
    _message: PhantomData<Message>,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Bars<Message> {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
    }

    fn layout(
        &mut self,
        _tree: &mut widget::Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(
            limits,
            Length::Fixed(self.width),
            Length::Fixed(self.height),
        )
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: iced::advanced::Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.width < 1.0 || bounds.height < 1.0 {
            return;
        }

        // Every token is read when iced draws, so a theme switch moves the
        // waveform with the page.
        let tokens = Tokens::of(theme);
        let tone = if self.capturing {
            tokens.primary
        } else {
            tokens.muted_foreground
        };

        // Bars thicken with the waveform so a tall one does not read as
        // hairline, and stand apart so a run of silence reads as a row of dots.
        let bar = (self.height * 0.125).clamp(2.0, 4.0);
        let gap = bar * 2.5;
        let pitch = bar + gap;
        // How many bars the width holds, and never more than there are levels,
        // Slint's `limit` and `bar-count`.
        let limit = (((bounds.width + gap) / pitch).floor()).max(0.0) as usize;
        let count = self.levels.len().min(limit);
        if count == 0 {
            return;
        }

        // The bars hang off the trailing edge, so a short history grows from the
        // right, the way the Slint row's `alignment: end` places them.
        let total = count as f32 * bar + (count as f32 - 1.0) * gap;
        let start = bounds.width - total;

        for index in 0..count {
            // The window of levels that fits: the newest sits against the
            // trailing edge.
            let level = self.levels[self.levels.len() - count + index].clamp(0.0, 1.0);
            let height = (self.height * level).max(bar);
            let x = bounds.x + start + index as f32 * pitch;
            let y = bounds.y + (bounds.height - height) / 2.0;

            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x,
                        y,
                        width: bar,
                        height,
                    },
                    border: Border {
                        color: iced::Color::TRANSPARENT,
                        width: 0.0,
                        radius: (bar / 2.0).into(),
                    },
                    snap: false,
                    ..Quad::default()
                },
                Background::Color(tone),
            );
        }
    }

    fn update(
        &mut self,
        _tree: &mut widget::Tree,
        event: &iced::Event,
        _layout: iced::advanced::Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        // A live meter stays awake while audio runs, the way the GPUI original
        // redraws every frame while capturing, so the newest level is painted as
        // it arrives rather than waiting for an unrelated event. iced has no
        // transition between levels, so the bar steps; the request keeps the
        // meter current, and it is also what the gallery's script advances on.
        if !self.capturing {
            return;
        }
        if matches!(
            event,
            iced::Event::Window(window::Event::RedrawRequested(_))
        ) {
            shell.request_redraw_at(iced::time::Instant::now() + LEVEL_INTERVAL);
        }
    }
}

// The two Lucide marks the Slint icon set names (`mic` and `square`, Lucide,
// ISC), one line each, so the embedded artwork is the same the Slint and GPUI
// galleries draw.

const MIC: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 19v3"/><path d="M19 10v2a7 7 0 0 1-14 0v-2"/><rect x="9" y="2" width="6" height="13" rx="3"/></svg>"#;

const SQUARE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2"/></svg>"#;
