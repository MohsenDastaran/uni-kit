//! `calendar` — a month grid.
//!
//! Run it with `cargo run --example calendar`, or in the browser as
//! `?story=calendar` in the gallery the website embeds. The application owns the month
//! and the selection; the component reports picks and shift requests.

use iced::widget::{button, column, row, text};
use iced::{Element, Task};

use crate::components::calendar::{Calendar, Date};
use crate::components::theme::ControlSize;

pub fn run(theme: iced::Theme) -> iced::Result {
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
}

#[derive(Debug, Clone)]
enum Message {
    /// A month shift requested by an arrow, in months.
    Shift(i32),
    Pick(Date),
    RangePick(Date),
    ToggleBlockWeekends,
    Columns(usize),
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
    }
    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    let range = app.range.or(app.anchor.map(|anchor| (anchor, anchor)));

    column![
        text(format!("picked {}", app.single)).size(14),
        Calendar::new(app.year, app.month)
            .selected(Some(app.single))
            .columns(app.columns)
            .block_weekends(app.block_weekends)
            .on_select(Message::Pick)
            .on_shift(Message::Shift),
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
        .spacing(8),
        text(match app.range {
            Some((from, to)) => format!("range {from} \u{2013} {to}"),
            None => "range: click two days".into(),
        })
        .size(14),
        Calendar::new(2026, 9)
            .columns(2)
            .range(range)
            .block_weekends(app.block_weekends)
            .on_select(Message::RangePick)
            .on_shift(Message::Shift),
    ]
    .spacing(16)
    .padding(16)
    .into()
}
