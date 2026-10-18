//! `calendar` — a month grid.
//!
//! Run it with `cargo run --example calendar`, or in the browser as
//! `?story=calendar` in the gallery the website embeds. The application owns the month
//! and the selection; the component reports picks and shift requests.
//!
//! The Slint example's three outlined sections — Calendar, Two columns and Three
//! columns — are the three here, in the same order, each under the `###` heading
//! the component page documents it at. The `// section: <title>` comments below
//! are the contract the website reads to cut this file into the usage panels a
//! component page shows, so the code on the page is the code that runs here.
//! Keep them, one per section, in order, and keep the closing `// section: end`
//! after the last one.
//!
//! Each heading carries the title and the code icon that asks the embedding page
//! to scroll to that sample. Both are the button, as in Slint's `SourceHeading`,
//! and a click posts the section's index and title through [`crate::bridge`],
//! the same bridge the Slint and GPUI galleries use.

use iced::widget::{button, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::calendar::{Calendar, Date};
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's `###` headings — have them. The index is what the page is
/// asked to scroll to.
const SECTIONS: [&str; 3] = ["Calendar", "Two columns", "Three columns"];

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — calendar")
        .theme(theme)
        .run()
}

struct App {
    year: i32,
    month: u32,
    single: Date,
    range: Option<(Date, Date)>,
    /// The first click of a range: the second completes it.
    anchor: Option<Date>,
    block_weekends: bool,
    columns: usize,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A month shift requested by an arrow, in months.
    Shift(i32),
    Pick(Date),
    RangePick(Date),
    ToggleBlockWeekends,
    Columns(usize),
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
        year: 2026,
        month: 9,
        single: Date::new(2026, 9, 26),
        range: None,
        anchor: None,
        block_weekends: false,
        columns: 1,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        // The application owns the shown month: the component only asks.
        Message::Shift(delta) => {
            let absolute = app.year * 12 + app.month as i32 - 1 + delta;
            app.year = absolute.div_euclid(12);
            app.month = (absolute.rem_euclid(12) + 1) as u32;
        }
        Message::Pick(date) => app.single = date,
        Message::RangePick(date) => match app.anchor.take() {
            None => app.anchor = Some(date),
            Some(anchor) => {
                app.range = Some(if date.key() < anchor.key() {
                    (date, anchor)
                } else {
                    (anchor, date)
                });
            }
        },
        Message::ToggleBlockWeekends => app.block_weekends = !app.block_weekends,
        Message::Columns(columns) => app.columns = columns,
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

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box the other galleries draw.
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
        .align_y(Alignment::Center)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Calendar
    // One month grid with the 26th picked: the component page's first documented
    // sample, and the Slint example's first `GroupBox`.
    let single = body_box(
        Calendar::new(app.year, app.month)
            .selected(Some(app.single))
            .block_weekends(app.block_weekends)
            .on_select(Message::Pick)
            .on_shift(Message::Shift),
    );

    // section: Two columns
    // The same month twice, side by side: the Slint example's `columns: 2`.
    let two = body_box(
        Calendar::new(app.year, app.month)
            .selected(Some(app.single))
            .columns(2)
            .block_weekends(app.block_weekends)
            .on_select(Message::Pick)
            .on_shift(Message::Shift),
    );

    // section: Three columns
    // The widest the component draws: three months, the Slint example's
    // `columns: 3`.
    let three = body_box(
        Calendar::new(app.year, app.month)
            .selected(Some(app.single))
            .columns(3)
            .block_weekends(app.block_weekends)
            .on_select(Message::Pick)
            .on_shift(Message::Shift),
    );

    // section: end
    // The live controls the demo is driven by, so a pick, a range and a month
    // shift can be seen rather than read: the column buttons above the range
    // calendar, and the weekends switch.
    let range = app.range.or(app.anchor.map(|anchor| (anchor, anchor)));
    let live = column![
        row![
            button(text("1 month").size(ControlSize::Small.control_font_size()))
                .on_press(Message::Columns(1)),
            button(text("2 months").size(ControlSize::Small.control_font_size()))
                .on_press(Message::Columns(2)),
            button(text("3 months").size(ControlSize::Small.control_font_size()))
                .on_press(Message::Columns(3)),
            button(text(if app.block_weekends {
                "Weekends blocked"
            } else {
                "Weekends pickable"
            }))
            .on_press(Message::ToggleBlockWeekends),
        ]
        .spacing(theme::SPACING_SM),
        Calendar::new(app.year, app.month)
            .columns(app.columns)
            .range(range)
            .block_weekends(app.block_weekends)
            .on_select(Message::RangePick)
            .on_shift(Message::Shift),
        text(format!("picked {}", app.single)).size(theme::FONT_SIZE_SM),
        text(match app.range {
            Some((from, to)) => format!("range {from} \u{2013} {to}"),
            None => "range: click two days".into(),
        })
        .size(theme::FONT_SIZE_SM),
    ]
    .spacing(theme::SPACING_MD);

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        single,
        heading(app, 1, SECTIONS[1]),
        two,
        heading(app, 2, SECTIONS[2]),
        three,
        live,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The page itself is a control, so a press outside a panel lands on a widget
    // rather than on the bare window.
    container(scrollable(gallery).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            ..container::Style::default()
        })
        .into()
}

// The Lucide mark the code icon uses, from the icon set this repository already
// ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three galleries
// draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
