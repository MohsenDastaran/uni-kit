//! `time-field` — the whole gallery the Slint example shows: the three basic
//! times, the seconds precision, the 12-hour clock, the size ladder, the
//! disabled and invalid fields, and the change each edit reports.
//!
//! Run it with `cargo run --example time_field`, or in the browser as
//! `?story=time-field` in the gallery the website embeds.
//!
//! Each section carries a heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! panels a component page shows, so the code on the page is the code that runs
//! here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! The gallery owns every piece of state the fields draw from, because the
//! component has none: it takes the hour, minute and second the caller sets and
//! reports an edit back through `on_change`. The values start where the Slint
//! example starts them, and the readouts spell what each edit left behind, so a
//! screenshot and a driven keystroke both have something to read.
//!
//! What the page asks for and iced cannot answer is named on the cards rather
//! than approximated: the GPUI field's Up/Down steps, Left/Right movement and
//! `a`/`p` period key are keys iced does not hand to a widget, so the period is
//! the read-only label Slint draws, and each numeric segment is its own
//! focusable field that Tab walks.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, ControlSize, Tokens};
use crate::components::time_field::{Change, HourCycle, TimeField, TimePrecision};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — time-field")
        .theme(theme)
        .run()
}

/// A time of day, the three parts the field draws from.
#[derive(Debug, Clone, Copy, Default)]
struct Time {
    hour: u32,
    minute: u32,
    second: u32,
}

impl Time {
    const fn new(hour: u32, minute: u32, second: u32) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }

    fn hhmm(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    fn hhmmss(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }

    /// The 12-hour clock a person reads: 0 and 12 both show as 12.
    fn h12(&self) -> String {
        let (hour, period) = if self.hour >= 12 {
            (self.hour - 12, "PM")
        } else {
            (self.hour, "AM")
        };
        let hour = if hour == 0 { 12 } else { hour };
        format!("{:02}:{:02} {period}", hour, self.minute)
    }
}

impl From<Change> for Time {
    fn from(change: Change) -> Self {
        Self {
            hour: change.hour(),
            minute: change.minute(),
            second: change.second(),
        }
    }
}

#[derive(Debug)]
struct App {
    /// The three times of the basic card, each a field of its own.
    basic: [Time; 3],
    seconds: Time,
    /// The 12-hour fields: a morning, an afternoon and a midnight.
    h12: [Time; 3],
    /// The size ladder's three fields, small / medium / large.
    sizes: [Time; 3],
    disabled: Time,
    invalid: Time,
    /// The field the `Handle Changes` card drives, and what it last reported.
    changes: Time,
    last: String,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// One variant per field, each carrying the change the component reported.
    Basic { index: usize, change: Change },
    Seconds(Change),
    H12 { index: usize, change: Change },
    Size { index: usize, change: Change },
    Invalid(Change),
    Changes(Change),
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
        basic: [
            Time::new(9, 30, 0),
            Time::new(0, 0, 0),
            Time::new(23, 59, 0),
        ],
        seconds: Time::new(14, 5, 42),
        h12: [
            Time::new(9, 30, 0),
            Time::new(14, 5, 0),
            Time::new(0, 0, 0),
        ],
        sizes: [Time::new(8, 15, 0); 3],
        disabled: Time::new(8, 15, 0),
        invalid: Time::new(8, 15, 0),
        changes: Time::new(9, 30, 0),
        last: "nothing yet".to_string(),
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Basic { index, change } => app.basic[index] = change.into(),
        Message::Seconds(change) => app.seconds = change.into(),
        Message::H12 { index, change } => app.h12[index] = change.into(),
        Message::Size { index, change } => app.sizes[index] = change.into(),
        Message::Invalid(change) => app.invalid = change.into(),
        Message::Changes(change) => {
            app.changes = change.into();
            app.last = app.changes.hhmmss();
        }
        Message::HoverSource(title) => {
            app.hovered = title;
            return Task::none();
        }
        Message::PressSource(title) => {
            app.pressed_source = title;
            return Task::none();
        }
        Message::ShowSource { index, title } => {
            app.pressed_source = None;
            // The page scrolls to the sample; natively this does nothing.
            crate::bridge::show(index, title);
            return Task::none();
        }
    }
    Task::none()
}

/// A muted line under an example, carrying what the demo has to say about it.
fn muted_line<'a>(line: &str) -> Element<'a, Message> {
    text(line.to_string())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(
            text(title)
                .size(theme::FONT_SIZE_SM)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..iced::Font::DEFAULT
                }),
        )
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
            svg(svg::Handle::from_memory(CLOCK_XML))
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
        .spacing(8.0)
        .align_y(Alignment::Center)
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

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic Time Field
    // Slint's first box is three fields — 09:30, 00:00 and 23:59 — over a
    // 24-hour clock, each one the caller's own value.
    let basic = column![
        TimeField::new("basic-0")
            .hour(app.basic[0].hour)
            .minute(app.basic[0].minute)
            .on_change(|change| Message::Basic { index: 0, change }),
        TimeField::new("basic-1")
            .hour(app.basic[1].hour)
            .minute(app.basic[1].minute)
            .on_change(|change| Message::Basic { index: 1, change }),
        TimeField::new("basic-2")
            .hour(app.basic[2].hour)
            .minute(app.basic[2].minute)
            .on_change(|change| Message::Basic { index: 2, change }),
        muted_line(&format!(
            "The first field reads {}. Each segment is a field of its own, and \
             the caller owns the value.",
            app.basic[0].hhmm()
        )),
    ]
    .spacing(12.0);

    // section: With Seconds
    // `precision(TimePrecision::HoursMinutesSeconds)` draws the third segment,
    // the Slint `with-seconds`.
    let seconds = column![
        TimeField::new("seconds")
            .hour(app.seconds.hour)
            .minute(app.seconds.minute)
            .second(app.seconds.second)
            .precision(TimePrecision::HoursMinutesSeconds)
            .on_change(Message::Seconds),
        muted_line(&format!(
            "With seconds: {} — the third segment appears with the precision.",
            app.seconds.hhmmss()
        )),
    ]
    .spacing(12.0);

    // section: 12-Hour Clock
    // `hour_cycle(HourCycle::H12)` shows 12–11 and the AM/PM label. iced hands a
    // widget no key event, so GPUI's `a`/`p` toggle has nowhere to live and the
    // label is read-only, as it is in the Slint component.
    let h12 = column![
        TimeField::new("h12-0")
            .hour(app.h12[0].hour)
            .minute(app.h12[0].minute)
            .hour_cycle(HourCycle::H12)
            .on_change(|change| Message::H12 { index: 0, change }),
        TimeField::new("h12-1")
            .hour(app.h12[1].hour)
            .minute(app.h12[1].minute)
            .hour_cycle(HourCycle::H12)
            .on_change(|change| Message::H12 { index: 1, change }),
        TimeField::new("h12-2")
            .hour(app.h12[2].hour)
            .minute(app.h12[2].minute)
            .hour_cycle(HourCycle::H12)
            .on_change(|change| Message::H12 { index: 2, change }),
        muted_line(&format!(
            "12-hour clock: {}, {} and {}.",
            app.h12[0].h12(),
            app.h12[1].h12(),
            app.h12[2].h12()
        )),
    ]
    .spacing(12.0);

    // section: Sizes, Disabled and Invalid
    // The size ladder is 20 / 24 / 32 / 40, the Slint `control-height`; a
    // disabled field is the muted fill and text, because iced has no whole-field
    // opacity; and an invalid field keeps the danger edge while it is edited.
    let sizes = column![
        row![
            TimeField::new("size-small")
                .hour(app.sizes[0].hour)
                .minute(app.sizes[0].minute)
                .size(ControlSize::Small)
                .on_change(|change| Message::Size { index: 0, change }),
            TimeField::new("size-medium")
                .hour(app.sizes[1].hour)
                .minute(app.sizes[1].minute)
                .size(ControlSize::Medium)
                .on_change(|change| Message::Size { index: 1, change }),
            TimeField::new("size-large")
                .hour(app.sizes[2].hour)
                .minute(app.sizes[2].minute)
                .size(ControlSize::Large)
                .on_change(|change| Message::Size { index: 2, change }),
        ]
        .spacing(16.0)
        .align_y(Alignment::Center),
        muted_line("The size ladder: 24 / 32 / 40, the Slint theme's control-height."),
        TimeField::new("disabled")
            .hour(app.disabled.hour)
            .minute(app.disabled.minute)
            .disabled(true),
        muted_line("disabled(true) refuses input and draws the muted field."),
        TimeField::new("invalid")
            .hour(app.invalid.hour)
            .minute(app.invalid.minute)
            .invalid(true)
            .on_change(Message::Invalid),
        muted_line("invalid(true): the frame's edge takes the danger colour, and edits are still accepted."),
    ]
    .spacing(12.0);

    // section: Handle Changes
    // `on_change` reports the time an edit left behind — the Slint
    // `changed(hour, minute, second)` — and the caller stores it. Type into a
    // segment and the readout below moves.
    let changes = column![
        TimeField::new("changes")
            .hour(app.changes.hour)
            .minute(app.changes.minute)
            .on_change(Message::Changes),
        muted_line(&format!("on_change reported: {}.", app.last)),
    ]
    .spacing(8.0);

    // section: end
    let gallery = column![
        heading(app, 0, "Basic Time Field"),
        body_box(basic),
        heading(app, 1, "With Seconds"),
        body_box(seconds),
        heading(app, 2, "12-Hour Clock"),
        body_box(h12),
        heading(app, 3, "Sizes, Disabled and Invalid"),
        body_box(sizes),
        heading(app, 4, "Handle Changes"),
        body_box(changes),
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The mark the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the
// two galleries draw the same glyph rather than two lookalikes.

const CLOCK_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>"#;
