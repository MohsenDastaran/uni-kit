//! `date-picker` — the whole gallery the Slint example shows: the field-shaped
//! trigger that opens a calendar, a date and a date with a time, the size
//! ladder, blocked weekends, a range, two and three months side by side, and the
//! value each pick reports.
//!
//! Run it with `cargo run --example date-picker`, or in the browser as
//! `?story=date-picker` in the gallery the website embeds.
//!
//! Each section carries the heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! The gallery owns every piece of state a date picker draws from — the month
//! the calendar shows, the chosen date or range, the open flag and the two time
//! strings — because the component has none: it reports a request and the
//! application answers with its own value. The month shown is the caller's, so
//! `on_shift` moves it exactly as it does for the calendar; picking a day also
//! moves the view to that day's month, which is what a person expects after a
//! jump. Escape and a click outside close the panel through the popover, which
//! owns those two dismissals for every overlay in this crate; Escape is also
//! bound here so the key closes one from anywhere.
//!
//! The selections start where the Slint example starts them: 2026-10-02, and
//! 09:30 for the one picker that shows a time. The fixed dates keep the gallery
//! the same every run, which is what a screenshot and a hash compare need.

use iced::keyboard;
use iced::widget::{button, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Subscription, Task, Theme};

use crate::components::calendar::Date;
use crate::components::date_picker::DatePicker;
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The width the gallery fixes its triggers at, so a single-month panel and a
/// two-month one are both deterministic in a screenshot.
const FIELD_WIDTH: f32 = 240.0;

/// The section headings, in the order the component page has them. The index is
/// what the page is asked to scroll to.
const SECTIONS: [&str; 7] = [
    "Usage",
    "Date Restrictions",
    "Custom Year Range",
    "Preset Ranges",
    "Handle Date Selection Events",
    "Multiple Months Display",
    "Advanced Examples",
];

/// Which date picker each piece of state belongs to. A section that shows more
/// than one picker — Usage, Preset Ranges, Multiple Months Display, Advanced
/// Examples — still gets one slot per picker, so opening one panel does not open
/// its neighbours.
mod slot {
    pub const USAGE_DATE: usize = 0;
    pub const USAGE_TIME: usize = 1;
    pub const USAGE_SMALL: usize = 2;
    pub const USAGE_MEDIUM: usize = 3;
    pub const USAGE_LARGE: usize = 4;
    pub const WEEKENDS: usize = 5;
    pub const BIRTHDAY: usize = 6;
    pub const PRESET_SINGLE: usize = 7;
    pub const PRESET_RANGE: usize = 8;
    pub const EVENTS: usize = 9;
    pub const MONTHS_TWO: usize = 10;
    pub const MONTHS_THREE: usize = 11;
    pub const RANGE: usize = 12;
    pub const BUSINESS: usize = 13;
    /// One more than the highest slot: the length of the state arrays.
    pub const COUNT: usize = 14;
}

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — date-picker")
        .theme(theme)
        .subscription(subscription)
        .run()
}

/// A preset the gallery's own buttons set: a single day, or a range. GPUI's
/// date picker takes presets as a list; the Slint component has none, so the
/// gallery composes them from ordinary buttons — see the module comment.
#[derive(Debug, Clone, Copy)]
enum Preset {
    Single(Date),
    Range(Date, Date),
}

#[derive(Debug, Clone)]
struct App {
    /// The month each picker's calendar shows.
    view: [(i32, u32); slot::COUNT],
    /// The chosen day of each single-date picker.
    date: [Option<Date>; slot::COUNT],
    /// The chosen range of each range picker. A half-made range is
    /// `Some((start, start))`, which is what the calendar marks and the trigger
    /// shows.
    range: [Option<(Date, Date)>; slot::COUNT],
    /// The first click of a range, waiting for the second.
    anchor: [Option<Date>; slot::COUNT],
    /// Whether each picker's panel is showing.
    open: [bool; slot::COUNT],
    /// The two time strings of the picker that shows a time.
    hour: [String; slot::COUNT],
    minute: [String; slot::COUNT],
    /// The last day a pick reported, for the status line.
    last: Option<Date>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A picker asked to be opened or closed; the slot says which one.
    Opened { slot: usize, open: bool },
    /// A day was picked, by slot.
    Picked { slot: usize, date: Date },
    /// A month shift the calendar's arrow asked for, in months.
    Shifted { slot: usize, delta: i32 },
    /// The hour field of a picker was edited.
    Hour { slot: usize, text: String },
    /// The minute field of a picker was edited.
    Minute { slot: usize, text: String },
    /// A preset button set a slot's value.
    Preset { slot: usize, value: Preset },
    /// Escape was pressed: close whichever panel is open.
    Escape,
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

/// Whether a slot is a range picker. The component has no range flag — the value
/// says it — but the gallery's own update has to know, so the map lives here.
fn is_range(slot: usize) -> bool {
    matches!(slot, slot::RANGE | slot::PRESET_RANGE)
}

/// Whether a slot shows the time row.
fn with_time(slot: usize) -> bool {
    matches!(slot, slot::USAGE_TIME)
}

/// Whether a slot blocks weekends.
fn blocks_weekends(slot: usize) -> bool {
    matches!(slot, slot::WEEKENDS | slot::BUSINESS)
}

/// How many months a slot's calendar shows.
fn months_of(slot: usize) -> usize {
    match slot {
        slot::MONTHS_TWO | slot::RANGE | slot::PRESET_RANGE => 2,
        slot::MONTHS_THREE => 3,
        _ => 1,
    }
}

/// The size ladder a slot draws at, so the three sizes are one section.
fn size_of(slot: usize) -> ControlSize {
    match slot {
        slot::USAGE_SMALL => ControlSize::Small,
        slot::USAGE_LARGE => ControlSize::Large,
        _ => ControlSize::Medium,
    }
}

fn boot() -> App {
    let mut view = [(2026, 10); slot::COUNT];
    let mut date = [None; slot::COUNT];
    let mut range = [None; slot::COUNT];

    view[slot::BIRTHDAY] = (1990, 1);
    view[slot::MONTHS_THREE] = (2026, 10);

    let start = Date::new(2026, 10, 2);
    date[slot::USAGE_DATE] = Some(start);
    date[slot::USAGE_TIME] = Some(Date::new(2026, 10, 2));
    date[slot::USAGE_SMALL] = Some(start);
    date[slot::USAGE_MEDIUM] = Some(start);
    date[slot::USAGE_LARGE] = Some(start);
    date[slot::WEEKENDS] = Some(start);
    date[slot::BUSINESS] = Some(start);

    range[slot::RANGE] = None;

    let mut hour = std::array::from_fn(|_| String::from("00"));
    let mut minute = std::array::from_fn(|_| String::from("00"));
    hour[slot::USAGE_TIME] = String::from("09");
    minute[slot::USAGE_TIME] = String::from("30");

    App {
        view,
        date,
        range,
        anchor: [None; slot::COUNT],
        open: [false; slot::COUNT],
        hour,
        minute,
        last: None,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Opened { slot, open } => {
            if let Some(flag) = app.open.get_mut(slot) {
                *flag = open;
            }
        }
        Message::Picked { slot, date } => {
            app.last = Some(date);
            app.view[slot] = (date.year, date.month);
            if is_range(slot) {
                // The two clicks of a range: the first anchors it and shows it
                // alone, the second completes it and closes the panel — the
                // Slint `choose` in range mode.
                match app.anchor[slot].take() {
                    None => {
                        app.anchor[slot] = Some(date);
                        app.range[slot] = Some((date, date));
                    }
                    Some(anchor) => {
                        app.range[slot] = Some(if date.key() < anchor.key() {
                            (date, anchor)
                        } else {
                            (anchor, date)
                        });
                        app.open[slot] = false;
                    }
                }
            } else {
                app.date[slot] = Some(date);
                // Slint closes the popup on a single date unless a time is being
                // edited; the caller answers `open`, because the component does
                // not own it.
                if !with_time(slot) {
                    app.open[slot] = false;
                }
            }
        }
        Message::Shifted { slot, delta } => {
            let (year, month) = app.view[slot];
            let absolute = year * 12 + month as i32 - 1 + delta;
            app.view[slot] = (
                absolute.div_euclid(12),
                (absolute.rem_euclid(12) + 1) as u32,
            );
        }
        // Slint's `clamp(floor(text.to-float()), 0, 23)` in `commit-time` is the
        // application's here, because the component holds no text of its own.
        Message::Hour { slot, text } => app.hour[slot] = text,
        Message::Minute { slot, text } => app.minute[slot] = text,
        Message::Preset { slot, value } => {
            match value {
                Preset::Single(date) => {
                    app.date[slot] = Some(date);
                    app.view[slot] = (date.year, date.month);
                }
                Preset::Range(from, to) => {
                    app.range[slot] = Some((from, to));
                    app.open[slot] = false;
                    app.view[slot] = (from.year, from.month);
                }
            }
            app.last = None;
        }
        Message::Escape => app.open = [false; slot::COUNT],
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

/// The keyboard the picker leaves to its host. The popover already answers
/// Escape, and a click outside closes every overlay in this crate; the binding
/// here makes the key close a panel the application owns even when the event
/// never reaches the overlay, which is cheap insurance for one key.
fn subscription(_app: &App) -> Subscription<Message> {
    iced::keyboard::listen().filter_map(|event| match event {
        keyboard::Event::KeyPressed { key, .. } => match key.as_ref() {
            keyboard::Key::Named(keyboard::key::Named::Escape) => Some(Message::Escape),
            _ => None,
        },
        _ => None,
    })
}

/// One date picker in the gallery, wired to the slot its view, value, open flag
/// and time live in.
///
/// A section sample spells the builder out rather than calling this, so the code
/// the component page shows is the code a reader would write; this is the body
/// those samples share where there is more than one picker in a card.
fn picker<'a>(app: &App, slot: usize) -> DatePicker<'a, Message> {
    let (year, month) = app.view[slot];
    let mut picker = DatePicker::new(year, month)
        .open(app.open[slot])
        .size(size_of(slot))
        .months(months_of(slot))
        .block_weekends(blocks_weekends(slot))
        .with_time(with_time(slot))
        .id(format!("date-picker-{slot}"))
        .width(Length::Fixed(FIELD_WIDTH))
        .on_open_change(move |open| Message::Opened { slot, open })
        .on_select(move |date| Message::Picked { slot, date })
        .on_shift(move |delta| Message::Shifted { slot, delta });

    if is_range(slot) {
        picker = picker.range(app.range[slot]);
    } else {
        picker = picker.date(app.date[slot]);
    }

    if with_time(slot) {
        picker = picker
            .hour(app.hour[slot].clone())
            .minute(app.minute[slot].clone())
            .on_hour(move |text| Message::Hour { slot, text })
            .on_minute(move |text| Message::Minute { slot, text });
    }

    picker
}

/// A preset button: a quiet inline control, the way GPUI's own preset list
/// draws one, that sets a slot's value.
fn preset_button<'a>(label: &'static str, value: Preset, slot: usize) -> Element<'a, Message> {
    button(text(label).size(theme::FONT_SIZE_SM))
        .padding([theme::SPACING_XS, theme::SPACING_SM])
        .style(|theme: &Theme, status| {
            let tokens = Tokens::of(theme);
            let fill = match status {
                button::Status::Hovered => tokens.accent,
                button::Status::Pressed => theme::pressed(&tokens, tokens.accent),
                _ => Color::TRANSPARENT,
            };
            button::Style {
                background: (fill.a > 0.0).then(|| Background::Color(fill)),
                text_color: tokens.foreground,
                border: Border {
                    radius: theme::RADIUS_MD.into(),
                    ..Border::default()
                },
                shadow: iced::Shadow::default(),
                snap: false,
            }
        })
        .on_press(Message::Preset { slot, value })
        .into()
}

/// A muted line under an example, carrying what the demo has to say about it.
fn muted_line<'a>(line: impl Into<String>) -> Element<'a, Message> {
    text(line.into())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box the menu gallery draws.
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

    iced::widget::row![label, icon]
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
    // section: Usage
    // The whole of a date picker: the month the calendar shows, the chosen day,
    // the open flag and the three reports. `new` takes the shown month, which the
    // caller owns because iced keeps no state; `on_shift` moves it, `on_select`
    // reports a day, and `on_open_change` reports every dismissal the popover
    // answers — a trigger click, an outside click and Escape. Setting
    // `with_time(true)` adds the time row under the calendar, and `size` and
    // `block_weekends` are the Slint component's own two properties.
    let usage = body_box(
        column![
            DatePicker::new(app.view[slot::USAGE_DATE].0, app.view[slot::USAGE_DATE].1)
                .date(app.date[slot::USAGE_DATE])
                .open(app.open[slot::USAGE_DATE])
                .placeholder("Select a date")
                .width(Length::Fixed(FIELD_WIDTH))
                .on_open_change(|open| Message::Opened {
                    slot: slot::USAGE_DATE,
                    open,
                })
                .on_select(|date| Message::Picked {
                    slot: slot::USAGE_DATE,
                    date,
                })
                .on_shift(|delta| Message::Shifted {
                    slot: slot::USAGE_DATE,
                    delta,
                }),
            picker(app, slot::USAGE_TIME),
            row![
                picker(app, slot::USAGE_SMALL),
                picker(app, slot::USAGE_MEDIUM),
                picker(app, slot::USAGE_LARGE),
            ]
            .spacing(12.0),
            muted_line(concat!(
                "The first field is a date; the second adds the time row. ",
                "Below it are the small, medium and large triggers."
            )),
        ]
        .spacing(12.0),
    );

    // section: Date Restrictions
    // `block_weekends` is the whole of the Slint restriction: a blocked day is
    // lettered faintly and cannot be picked. GPUI's `Matcher` — a range, an
    // interval or a custom predicate — is not in the Slint calendar, so the port
    // does not invent it.
    let restrictions = body_box(
        column![
            picker(app, slot::WEEKENDS),
            muted_line(
                "Weekends are blocked: the grid letters them faintly and a click \
                 does nothing. A caller that needs a range or a predicate keeps \
                 that rule where the value is applied.",
            ),
        ]
        .spacing(8.0),
    );

    // section: Custom Year Range
    // The shown year is the caller's: this picker starts at 1990, and every
    // arrow moves the view through `on_shift`, so a caller that wants a range
    // clamps the shift in its own `update`. The Slint calendar has two month
    // arrows and no year grid, so there is no year-selection mode to configure.
    let years = body_box(
        column![
            picker(app, slot::BIRTHDAY),
            muted_line(
                "A birthday picker: the view starts at 1990 and the caller clamps \
                 `on_shift` to whatever range it allows. There is no year-selection \
                 grid — the Slint calendar has only the two month arrows.",
            ),
        ]
        .spacing(8.0),
    );

    // section: Preset Ranges
    // Presets are the application's. GPUI's `presets(...)` is not in the Slint
    // component, so the gallery builds them from ordinary buttons that set the
    // same value a pick does — the component needs no property for it.
    let presets = body_box(
        column![
            picker(app, slot::PRESET_SINGLE),
            row![
                preset_button(
                    "Yesterday",
                    Preset::Single(Date::new(2026, 10, 1)),
                    slot::PRESET_SINGLE,
                ),
                preset_button(
                    "Last Week",
                    Preset::Single(Date::new(2026, 9, 25)),
                    slot::PRESET_SINGLE,
                ),
                preset_button(
                    "Last Month",
                    Preset::Single(Date::new(2026, 9, 2)),
                    slot::PRESET_SINGLE,
                ),
            ]
            .spacing(8.0),
            picker(app, slot::PRESET_RANGE),
            row![
                preset_button(
                    "Last 7 Days",
                    Preset::Range(Date::new(2026, 9, 26), Date::new(2026, 10, 2)),
                    slot::PRESET_RANGE,
                ),
                preset_button(
                    "Last 30 Days",
                    Preset::Range(Date::new(2026, 9, 3), Date::new(2026, 10, 2)),
                    slot::PRESET_RANGE,
                ),
                preset_button(
                    "Last 90 Days",
                    Preset::Range(Date::new(2026, 7, 5), Date::new(2026, 10, 2)),
                    slot::PRESET_RANGE,
                ),
            ]
            .spacing(8.0),
            muted_line(
                "A preset is a button that writes the value the picker would have \
                 reported, and it is the caller's to build.",
            ),
        ]
        .spacing(8.0),
    );

    // section: Handle Date Selection Events
    // `on_select` reports the day the calendar picked; the line under the field
    // is the report made visible. The panel closes only because the caller
    // answers `open`, which is also where a range's second end closes it.
    let events = body_box(
        column![
            picker(app, slot::EVENTS),
            muted_line(format!(
                "Last pick: {}",
                app.last
                    .map(|date| date.to_string())
                    .unwrap_or_else(|| "nothing yet".to_string())
            )),
        ]
        .spacing(8.0),
    );

    // section: Multiple Months Display
    // `months(2)` and `months(3)` are the Slint `columns`: two or three month
    // grids side by side inside the one popover, which is what a range picker
    // usually shows.
    let months = body_box(
        column![
            picker(app, slot::MONTHS_TWO),
            picker(app, slot::MONTHS_THREE),
        ]
        .spacing(12.0),
    );

    // section: Advanced Examples
    // The range picker reports each end; the caller holds the anchor between the
    // two clicks, so a maximum duration is enforced where the value is set rather
    // than inside the component. The business-day picker is `block_weekends` on
    // the two weekend columns.
    let advanced = body_box(
        column![
            picker(app, slot::RANGE),
            picker(app, slot::BUSINESS),
            muted_line(
                "The range picker shows two months and marks the first click until \
                 the second arrives; a caller that caps the span checks it in \
                 `update`. The second picker blocks weekends.",
            ),
        ]
        .spacing(12.0),
    );

    // section: end
    let status = text(match (
        app.last,
        app.open.iter().position(|open| *open),
        app.range.iter().find_map(|range| *range),
    ) {
        (Some(date), _, _) => format!(
            "The last pick was {date}. A single date closes its panel unless a time \
             is being edited; a range closes on its second end."
        ),
        (None, Some(slot), _) => format!(
            "Date picker {} is open. The arrows move the view through `on_shift`; \
             Escape or a click outside closes it.",
            slot + 1
        ),
        (None, None, Some((from, to))) => format!("A range is set: {from} – {to}."),
        _ => "Open a field and pick a day; the month arrows move the view.".to_string(),
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        usage,
        heading(app, 1, SECTIONS[1]),
        restrictions,
        heading(app, 2, SECTIONS[2]),
        years,
        heading(app, 3, SECTIONS[3]),
        presets,
        heading(app, 4, SECTIONS[4]),
        events,
        heading(app, 5, SECTIONS[5]),
        months,
        heading(app, 6, SECTIONS[6]),
        advanced,
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The page itself is a control, so a press outside a panel lands on a widget
    // rather than on the bare window.
    let page = container(scrollable(gallery).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            ..container::Style::default()
        });
    page.into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
