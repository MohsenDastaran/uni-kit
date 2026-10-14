//! `speech` — dictation, shown the way the Slint example shows it: an input the
//! host can dictates into, the event preview, the session controls, and the
//! button's six states.
//!
//! Run it with `cargo run --example speech`, or in the browser as
//! `?story=speech` in the gallery the website embeds.
//!
//! **iced cannot time a session, so this page plays the script itself.** Slint
//! drives its example from two `Timer`s at 90 ms and 500 ms. `iced::time::every`
//! needs a runtime backend this crate does not enable (see the module note in
//! `components/speech.rs`), so the script advances on the window's frames
//! instead: `iced::window::frames` produces one message per redraw, the gallery
//! advances a level every 90 ms of them, and the waveform asks for those frames
//! while it captures. The two consequences are named rather than hidden: the
//! cadence follows the frame clock rather than a fixed timer, and the final
//! result lands on the first frame after the user stops instead of after a
//! 500 ms wait. An application runs its own recognizer and moves `status`; this
//! is a recognizer that needs no service.
//!
//! The section titles below are the page's own `###` headings — `Dictate into an
//! input`, `Events`, `Session control`, `Button states` — which are the four
//! `GroupBox` titles the Slint example wraps its sections in. The plugin's own
//! `icedSections` reads these markers, so the code on the page is the code that
//! runs here.
//!
//! Each section carries a heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows. Keep them, one per section, in order,
//! and keep the closing `// section: end` after the last one.

use crate::components::button::Button;
use crate::components::group_box::{self, GroupBox};
use crate::components::input::Input;
use crate::components::speech::{SpeechButton, SpeechStatus, SpeechWaveform};
use crate::components::theme::{self, ControlSize, Tokens};
use iced::time::Instant;
use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment as Axis, Color, Element, Length, Subscription, Task, Theme};
use std::time::Duration;

/// The page padding and the gap between sections, which the Slint example sets
/// to 24px.
const PAGE: f32 = theme::SPACING_XL;

/// What a session would recognize, in order.
const SCRIPT: [&str; 8] = [
    "Speech", "input", "turns", "what", "you", "say", "into", "text.",
];

/// How often a level is reported, the Slint example's `level-interval`.
const LEVEL_INTERVAL: Duration = Duration::from_millis(90);

/// How many levels one word takes, the Slint example's three-tick cadence.
const LEVELS_PER_WORD: f32 = 3.0;

/// How many recent levels the waveform window holds, the Slint example's twelve.
const LEVEL_WINDOW: usize = 12;

/// A level that rises and falls like speech, sampled one tick apart: the Slint
/// example's `envelope`, in radians rather than degrees.
fn envelope(tick: f32) -> f32 {
    0.12 + 0.88 * (tick * 37.0_f32.to_radians()).sin().abs()
}

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — speech")
        .theme(theme)
        .subscription(subscription)
        .run()
}

#[derive(Debug)]
struct App {
    /// Where the host's session is, the one thing the component reads.
    status: SpeechStatus,
    /// How many levels have been reported since the session started.
    clock: f32,
    /// How many words of [`SCRIPT`] have been recognized.
    spoken: usize,
    /// The whole transcript of the session so far, the `Partial`/`Final` text.
    transcript: String,
    /// The recent levels, oldest first, in the order the waveform expects.
    levels: Vec<f32>,
    /// The latest event's preview, so the Events section shows what a grant
    /// would have carried.
    last_event: String,
    /// The field the final result is committed to.
    field: String,
    /// When the last level was reported, so the frame clock can be held to
    /// [`LEVEL_INTERVAL`].
    last_level: Option<Instant>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A redraw happened. The script advances on these, because iced has no
    /// timer this crate can subscribe to.
    Frame(Instant),
    /// The microphone was pressed: `true` starts a session, `false` stops it.
    Toggle(bool),
    Start,
    Stop,
    Cancel,
    /// The field reported an edit.
    FieldChanged(String),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot() -> App {
    App {
        status: SpeechStatus::Idle,
        clock: 0.0,
        spoken: 0,
        transcript: String::new(),
        levels: Vec::new(),
        last_event: String::new(),
        field: String::new(),
        last_level: None,
        hovered: None,
        pressed_source: None,
    }
}

impl App {
    /// What the host does to start a session: nothing while one runs, which is
    /// GPUI's `start`.
    fn start(&mut self) {
        if self.status.is_active() {
            return;
        }
        self.clock = 0.0;
        self.spoken = 0;
        self.transcript.clear();
        self.levels.clear();
        self.last_event.clear();
        self.last_level = None;
        self.status = SpeechStatus::Connecting;
    }

    /// Stop the running session: capture ends and the final result is pending,
    /// GPUI's `stop`.
    fn stop(&mut self) {
        if self.status.is_capturing() {
            self.status = SpeechStatus::Stopping;
        }
    }

    /// Discard the session at once, GPUI's `cancel`.
    fn cancel(&mut self) {
        if !self.status.is_active() {
            return;
        }
        self.status = SpeechStatus::Idle;
        self.clock = 0.0;
        self.spoken = 0;
        self.transcript.clear();
        self.levels.clear();
        self.last_level = None;
        self.last_event = "Cancelled".to_string();
    }

    /// One level of the script: the recognizer connects on the first, a word
    /// lands every third, and a full script stops the session.
    fn tick(&mut self) {
        if self.status == SpeechStatus::Connecting {
            self.status = SpeechStatus::Recording;
        }
        self.clock += 1.0;

        if self.status == SpeechStatus::Recording {
            let due = (self.clock / LEVELS_PER_WORD).floor() as usize;
            if due > self.spoken && self.spoken < SCRIPT.len() {
                self.transcript = if self.spoken == 0 {
                    SCRIPT[0].to_string()
                } else {
                    format!("{} {}", self.transcript, SCRIPT[self.spoken])
                };
                self.spoken += 1;
                self.last_event = format!("Partial · {}", self.transcript);
            } else if self.spoken >= SCRIPT.len() {
                self.stop();
            }
        }

        // The window of levels the waveform shows, oldest first: the Slint
        // example samples the trail from its clock rather than storing a buffer.
        self.levels = (0..LEVEL_WINDOW)
            .map(|index| envelope(self.clock - (LEVEL_WINDOW as f32 - 1.0) + index as f32))
            .collect();
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Frame(now) => {
            if app.status == SpeechStatus::Stopping {
                // The recognizer's final result, which the application commits
                // where the text belongs. Slint waits 500 ms for it on a second
                // `Timer`; this crate has no timer, so it lands on the first
                // frame after the user stops.
                app.field = app.transcript.clone();
                app.status = SpeechStatus::Idle;
                app.levels.clear();
                app.last_level = None;
                app.last_event = format!("Final · {}", app.transcript);
                return Task::none();
            }
            if !app.status.is_capturing() {
                return Task::none();
            }
            // The frame clock is much faster than the level rate, so a level is
            // reported only once every `LEVEL_INTERVAL`.
            if let Some(last) = app.last_level {
                if now.duration_since(last) < LEVEL_INTERVAL {
                    return Task::none();
                }
            }
            app.last_level = Some(now);
            app.tick();
        }
        Message::Toggle(active) => {
            if active {
                app.start();
            } else {
                app.stop();
            }
        }
        Message::Start => app.start(),
        Message::Stop => app.stop(),
        Message::Cancel => app.cancel(),
        Message::FieldChanged(field) => app.field = field,
        Message::HoverSource(title) => app.hovered = title,
        Message::PressSource(title) => app.pressed_source = title,
        Message::ShowSource { index, title } => {
            app.pressed_source = None;
            // The page scrolls to the sample; natively this does nothing.
            crate::bridge::show(index, title);
        }
    }
    Task::none()
}

/// The frames the script runs on. The waveform asks for one while it captures,
/// and every state change asks for one of its own, so a session advances and then
/// settles when it ends.
fn subscription(_app: &App) -> Subscription<Message> {
    iced::window::frames().map(Message::Frame)
}

/// A line of text whose colour is resolved from the active tokens when iced
/// renders it, so it reads correctly in either theme.
fn line<'a>(
    value: &'a str,
    size: f32,
    tone: fn(&Tokens) -> Color,
) -> iced::widget::Text<'a, Theme, iced::Renderer> {
    text(value)
        .size(size)
        .style(move |theme: &Theme| iced::widget::text::Style {
            color: Some(tone(&Tokens::of(theme))),
        })
}

/// A muted line for a value built with `format!` rather than borrowed from the
/// application state. It owns its text, so it does not tie the view's lifetime to
/// a local.
fn owned_muted<'a>(value: String, size: f32) -> Element<'a, Message> {
    text(value)
        .size(size)
        .style(|theme: &Theme| iced::widget::text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// The status as the Slint example spells it.
fn status_text(status: SpeechStatus) -> &'static str {
    match status {
        SpeechStatus::Idle => "idle",
        SpeechStatus::Connecting => "connecting",
        SpeechStatus::Recording => "recording",
        SpeechStatus::Stopping => "stopping",
    }
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(text(title).size(theme::FONT_SIZE_SM).font(iced::Font {
            weight: iced::font::Weight::Medium,
            ..iced::Font::DEFAULT
        }))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .style(|theme: &Theme| container::Style {
            text_color: Some(Tokens::of(theme).muted_foreground),
            ..container::Style::default()
        }),
    )
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    let icon = mouse_area(
        container(
            svg(svg::Handle::from_memory(CODE_XML))
                .width(16.0)
                .height(16.0)
                .style(|theme: &Theme, _status| svg::Style {
                    color: Some(Tokens::of(theme).primary),
                }),
        )
        .width(28.0)
        .height(28.0)
        .center_x(28.0)
        .center_y(28.0)
        .style(move |theme: &Theme| {
            let primary = Tokens::of(theme).primary;
            let (fill, line) = if pressed {
                (0.34, 0.7)
            } else if hovered {
                (0.22, 0.55)
            } else {
                (0.12, 0.35)
            };
            container::Style {
                background: Some(primary.scale_alpha(fill).into()),
                border: iced::border::rounded(theme::RADIUS_LG)
                    .color(primary.scale_alpha(line))
                    .width(1.0),
                ..container::Style::default()
            }
        }),
    )
    .on_enter(Message::HoverSource(Some(title)))
    .on_exit(Message::HoverSource(None))
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    row![label, icon]
        .spacing(theme::SPACING_SM)
        .align_y(Axis::Center)
        .into()
}

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A session control: an extra-small button that is disabled when the session
/// says its action does not apply.
fn control<'a>(label: &'static str, enabled: bool, message: Message) -> Button<'a, Message> {
    let button = Button::new(label).size(ControlSize::XSmall);
    if enabled {
        button.on_press(message)
    } else {
        button
    }
}

/// One cell of the button-states grid: the control, and its caption under it.
/// The caption is wider than the control, so the cross-axis alignment centres
/// the button over it — which is what the Slint example's `cross-axis-alignment:
/// center` does.
fn state_cell<'a>(
    button: SpeechButton<'a, Message>,
    caption: &'static str,
) -> Element<'a, Message> {
    column![
        // The control sits in a slot of its own height, so the cell whose button
        // draws nothing keeps its caption on the same line as every other cell.
        container(button).center_y(Length::Fixed(ControlSize::XSmall.control_height())),
        line(caption, theme::FONT_SIZE_XS, |tokens| tokens
            .muted_foreground),
    ]
    .spacing(theme::SPACING_XS)
    .align_x(Axis::Center)
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Dictate into an input
    // The waveform and the microphone are the field's suffix. The waveform is
    // always there — an empty history draws nothing, so no condition is needed
    // to keep it out of the way — and the application starts and stops its own
    // session on the button's callback.
    let dictate = column![
        Input::new("speech-field")
            .value(&app.field)
            .placeholder("Type or dictate")
            .on_input(Message::FieldChanged)
            .suffix(
                row![
                    SpeechWaveform::new()
                        .levels(app.levels.iter().copied())
                        .capturing(app.status.is_capturing())
                        .size(ControlSize::XSmall)
                        .width(48.0),
                    SpeechButton::new()
                        .status(app.status)
                        .size(ControlSize::XSmall)
                        .on_toggle(Message::Toggle),
                ]
                .spacing(theme::SPACING_SM)
                .align_y(Axis::Center),
            ),
        line(&app.transcript, theme::FONT_SIZE_SM, |tokens| tokens
            .muted_foreground,),
    ]
    .spacing(theme::SPACING_SM);

    // section: Events
    // Every event carries the whole transcript, so the latest one is the preview.
    // The events themselves are the application's: the component reports nothing,
    // and this demo shows what the host would have carried.
    let events = column![
        line(
            "Every event carries the whole transcript, so the latest one is the preview.",
            theme::FONT_SIZE_SM,
            |tokens| tokens.muted_foreground,
        ),
        row![
            SpeechWaveform::new()
                .levels(app.levels.iter().copied())
                .capturing(app.status.is_capturing())
                .size(ControlSize::Small),
            if app.last_event.is_empty() {
                line("Nothing yet", theme::FONT_SIZE_SM, |tokens| {
                    tokens.muted_foreground
                })
            } else {
                line(&app.last_event, theme::FONT_SIZE_SM, |tokens| {
                    tokens.foreground
                })
            },
        ]
        .spacing(theme::SPACING_MD)
        .align_y(Axis::Center),
    ]
    .spacing(theme::SPACING_SM);

    // section: Session control
    // The state the component reads, and the three calls an application makes to
    // it: start, stop, cancel. iced's element cannot own the session, so the
    // buttons move the one this demo holds.
    let session_status = format!("status: {}", status_text(app.status));
    let session = column![
        row![
            control("Start", !app.status.is_active(), Message::Start),
            control("Stop", app.status.is_capturing(), Message::Stop),
            control("Cancel", app.status.is_active(), Message::Cancel),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(Axis::Center),
        owned_muted(session_status, theme::FONT_SIZE_SM),
    ]
    .spacing(theme::SPACING_SM);

    // section: Button states
    // The same control in its six states: idle, capturing, stopping, disabled,
    // unavailable, and hidden when the host has no recognizer at all.
    let states = row![
        state_cell(
            SpeechButton::new()
                .status(SpeechStatus::Idle)
                .size(ControlSize::XSmall),
            "Idle",
        ),
        state_cell(
            SpeechButton::new()
                .status(SpeechStatus::Recording)
                .size(ControlSize::XSmall),
            "Capturing",
        ),
        state_cell(
            SpeechButton::new()
                .status(SpeechStatus::Stopping)
                .size(ControlSize::XSmall),
            "Stopping",
        ),
        state_cell(
            SpeechButton::new().disabled(true).size(ControlSize::XSmall),
            "Disabled",
        ),
        state_cell(
            SpeechButton::new()
                .available(false)
                .size(ControlSize::XSmall),
            "Unavailable",
        ),
        state_cell(
            SpeechButton::new()
                .has_recognizer(false)
                .size(ControlSize::XSmall),
            "Hidden",
        ),
    ]
    .spacing(theme::SPACING_LG)
    .align_y(Axis::Start);

    // section: end
    let summary = format!(
        "{} · {} · field: {}",
        status_text(app.status),
        if app.levels.is_empty() {
            "no levels"
        } else {
            "levels running"
        },
        if app.field.is_empty() {
            "empty"
        } else {
            app.field.as_str()
        },
    );
    let status = owned_muted(summary, theme::FONT_SIZE_SM);

    let gallery = column![
        heading(app, 0, "Dictate into an input"),
        body_box(dictate),
        heading(app, 1, "Events"),
        body_box(events),
        heading(app, 2, "Session control"),
        body_box(session),
        heading(app, 3, "Button states"),
        body_box(states),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The Lucide mark the code icon uses, from the icon set this repository already
// ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three galleries
// draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
