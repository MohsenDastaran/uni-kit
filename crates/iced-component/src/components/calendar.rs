//! `calendar` — a month grid, mirroring the Slint `Calendar` and GPUI
//! Component's `Calendar`.
//!
//! The application owns the date: the component is given the year and month to
//! show, the selected day, and an optional range, and reports a pick through
//! [`Calendar::on_select`] and a previous/next request through
//! [`Calendar::on_shift`] (a delta in months, `-1` or `1`, exactly the `shift`
//! the Slint file performs). It never advances its own month.
//!
//! Reproduced from the Slint file:
//!
//! - a 252 px month: seven 36 px columns and six 32 px rows of days, weekday
//!   initials in `muted-foreground` above them, the month and year centred in
//!   `font-size-sm` at weight 600 between the two arrows;
//! - `columns` months side by side, `24` apart, inside one bordered panel with
//!   `radius-lg` and `16` of padding; only the first column shows the previous
//!   arrow and only the last shows the next one;
//! - a day that is an endpoint of the range (or the single marked day) is filled
//!   with `primary` and lettered `primary-foreground`; days inside the range are
//!   `primary` at 16%; a hovered day out of the range is `accent`;
//! - blocked weekend days (when `block_weekends`) are lettered at 40% and cannot
//!   be picked;
//! - the leap-year rule and the first-weekday algorithm are copied from the
//!   Slint `days-in-month` and `first-weekday` functions so the grid matches.
//!
//! iced has no icon set of its own, so the two arrows are text chevrons unless
//! the caller supplies their own through [`Calendar::previous`] and
//! [`Calendar::next`]. The button reports hover and press the way every control
//! in this crate does, which is iced's replacement for Slint's `TouchArea`.

#![allow(dead_code)]

use iced::widget::{button, column, container, row, space, text};
use iced::{Background, Border, Color, Element, Length, Shadow, Theme};

use super::theme::{self, Tokens};

/// The size of one day cell, from the Slint file.
pub const CELL_WIDTH: f32 = 36.0;
pub const CELL_HEIGHT: f32 = 32.0;
/// One month is seven cells wide.
pub const MONTH_WIDTH: f32 = CELL_WIDTH * 7.0;
/// The gap between two months, and the panel's own padding.
pub const MONTH_GAP: f32 = 24.0;
pub const PANEL_PADDING: f32 = 16.0;

/// The weekday initials the Slint grid draws, Sunday first.
pub const WEEKDAYS: [&str; 7] = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

/// The month names the Slint header draws.
pub const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// A calendar date. This is a value type: the three fields are the definition,
/// which is why they are public.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    pub const fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// The `yyyymmdd` key the Slint range comparison uses.
    pub const fn key(self) -> i32 {
        self.year * 10000 + self.month as i32 * 100 + self.day as i32
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// The days in a month, with the Slint leap-year rule.
pub fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 => {
            if year.rem_euclid(4) == 0
                && (year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0)
            {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// The Slint `month-offset`: the day-of-week shift that starts each month in
/// Zeller's congruence.
fn month_offset(month: u32) -> i32 {
    match month {
        1 => 0,
        2 => 3,
        3 => 2,
        4 => 5,
        5 => 0,
        6 => 3,
        7 => 5,
        8 => 1,
        9 => 4,
        10 => 6,
        11 => 2,
        _ => 4,
    }
}

/// The weekday the first of `month` falls on, `0` being Sunday. This is the
/// Slint `first-weekday` function, copied so the grid agrees with it.
pub fn first_weekday(year: i32, month: u32) -> u32 {
    let yy = if month < 3 { year - 1 } else { year };
    (yy + yy.div_euclid(4) - yy.div_euclid(100) + yy.div_euclid(400) + month_offset(month) + 1)
        .rem_euclid(7) as u32
}

/// A month grid, built with the methods below.
pub struct Calendar<'a, Message> {
    year: i32,
    month: u32,
    selected: Option<Date>,
    range: Option<(Date, Date)>,
    columns: usize,
    block_weekends: bool,
    bordered: bool,
    previous: Option<Element<'a, Message>>,
    next: Option<Element<'a, Message>>,
    on_select: Option<Box<dyn Fn(Date) -> Message + 'a>>,
    on_shift: Option<Box<dyn Fn(i32) -> Message + 'a>>,
}

impl<'a, Message> Calendar<'a, Message> {
    /// The month to show. The caller owns it and advances it in its own state.
    pub fn new(year: i32, month: u32) -> Self {
        Self {
            year,
            month: month.clamp(1, 12),
            selected: None,
            range: None,
            columns: 1,
            block_weekends: false,
            bordered: true,
            previous: None,
            next: None,
            on_select: None,
            on_shift: None,
        }
    }

    /// The single marked day, filled with `primary`. Ignored while a range is
    /// set, matching the Slint `marked-day` / `from-key` rule.
    pub fn selected(mut self, selected: Option<Date>) -> Self {
        self.selected = selected;
        self
    }

    /// An inclusive range. Its endpoints are filled and the days between them
    /// are tinted.
    pub fn range(mut self, range: Option<(Date, Date)>) -> Self {
        self.range = range;
        self
    }

    /// How many months to show: 1, or 2–3 side by side.
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.clamp(1, 3);
        self
    }

    /// Weekends cannot be picked and are lettered faintly.
    pub fn block_weekends(mut self, block: bool) -> Self {
        self.block_weekends = block;
        self
    }

    /// Whether the calendar draws its own one-pixel border and `radius-lg`.
    /// `true` by default, which is what the Slint `Calendar` draws on its own; a
    /// caller that places it inside a surface it already owns — the date
    /// picker's [`Popover`](super::popover::Popover) — asks for `false`, so the
    /// panel has one surface rather than a border inside a border, the same rule
    /// [`Menu`](super::menu::Menu) follows. The 16px padding stays either way:
    /// it is the grid's inset, not part of the edge.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// The previous arrow. The caller's icon, because the icon set belongs to
    /// the application; the default is a text chevron.
    pub fn previous(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.previous = Some(icon.into());
        self
    }

    /// The next arrow. See [`Self::previous`].
    pub fn next(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.next = Some(icon.into());
        self
    }

    /// A day was picked.
    pub fn on_select(mut self, on_select: impl Fn(Date) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// The month should shift by this many months. The caller applies it to its
    /// own year and month, exactly as the Slint `shift` does.
    pub fn on_shift(mut self, on_shift: impl Fn(i32) -> Message + 'a) -> Self {
        self.on_shift = Some(Box::new(on_shift));
        self
    }
}

/// A ghost arrow button, the `ButtonVariant.ghost` + `ControlSize.xsmall` the
/// Slint header places: 20 px square, no fill at rest, `accent` on hover.
fn arrow<'a, Message: Clone + 'a>(
    icon: Element<'a, Message>,
    message: Option<Message>,
) -> Element<'a, Message> {
    let mut control = button(
        container(icon)
            .width(Length::Fixed(20.0))
            .height(Length::Fixed(20.0))
            .center_x(Length::Fixed(20.0))
            .center_y(Length::Fixed(20.0)),
    )
    .padding(0)
    .width(Length::Fixed(20.0))
    .height(Length::Fixed(20.0))
    .style(|theme: &Theme, status| {
        let tokens = Tokens::of(theme);
        let fill = match status {
            button::Status::Hovered => theme::alpha(tokens.accent, 0.6),
            button::Status::Pressed => tokens.accent,
            _ => Color::TRANSPARENT,
        };
        button::Style {
            background: if fill.a <= 0.0 {
                None
            } else {
                Some(Background::Color(fill))
            },
            text_color: tokens.foreground,
            border: Border {
                radius: theme::RADIUS_MD.into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
            snap: false,
        }
    });
    if let Some(message) = message {
        control = control.on_press(message);
    }
    control.into()
}

/// One day cell. The button's own status supplies hover, so the flags it
/// captures are the model's, not the pointer's.
#[allow(clippy::too_many_arguments)]
fn day_cell<'a, Message: Clone + 'a>(
    day: i32,
    in_month: bool,
    blocked: bool,
    marked: bool,
    endpoint: bool,
    inside: bool,
    message: Option<Message>,
) -> Element<'a, Message> {
    let mut control = button(
        text(if in_month { day.to_string() } else { String::new() })
            .size(theme::FONT_SIZE_SM)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center),
    )
    .padding(0)
    .width(Length::Fixed(CELL_WIDTH))
    .height(Length::Fixed(CELL_HEIGHT))
    .style(move |theme: &Theme, status| {
        let tokens = Tokens::of(theme);
        let pickable = in_month && !blocked;
        let fill = if endpoint || marked {
            tokens.primary
        } else if inside {
            theme::alpha(tokens.primary, 0.16)
        } else if pickable && matches!(status, button::Status::Hovered) {
            tokens.accent
        } else if pickable && matches!(status, button::Status::Pressed) {
            theme::pressed(&tokens, tokens.accent)
        } else {
            Color::TRANSPARENT
        };
        let ink = if blocked {
            theme::alpha(tokens.muted_foreground, 0.4)
        } else if endpoint || marked {
            tokens.primary_foreground
        } else {
            tokens.foreground
        };
        button::Style {
            background: if fill.a <= 0.0 {
                None
            } else {
                Some(Background::Color(fill))
            },
            text_color: ink,
            border: Border {
                radius: theme::RADIUS_MD.into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
            snap: false,
        }
    });
    if let Some(message) = message {
        control = control.on_press(message);
    }
    control.into()
}

/// One month: its header, the weekday row and the six weeks of days.
fn month<'a, Message: Clone + 'a>(
    calendar: &Calendar<'a, Message>,
    show_year: i32,
    show_month: u32,
    previous: Option<Element<'a, Message>>,
    next: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let title = format!("{} {}", MONTHS[(show_month - 1) as usize], show_year);

    let previous = match previous {
        Some(icon) => arrow(icon, calendar.on_shift.as_ref().map(|shift| shift(-1))),
        None => space().width(Length::Fixed(20.0)).into(),
    };
    let next = match next {
        Some(icon) => arrow(icon, calendar.on_shift.as_ref().map(|shift| shift(1))),
        None => space().width(Length::Fixed(20.0)).into(),
    };

    let mut header = row![previous].spacing(0).align_y(iced::Alignment::Center);
    header = header.push(
        container(
            text(title)
                .size(theme::FONT_SIZE_SM)
                .font(iced::Font {
                    weight: iced::font::Weight::Semibold,
                    ..iced::Font::DEFAULT
                })
                .align_x(iced::Alignment::Center)
                .align_y(iced::Alignment::Center),
        )
        .width(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fixed(20.0)),
    );
    header = header.push(next);

    let mut weekdays = row![].spacing(0);
    for name in WEEKDAYS {
        weekdays = weekdays.push(
            container(
                text(name)
                    .size(theme::FONT_SIZE_XS)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    })
                    .align_x(iced::Alignment::Center),
            )
            .width(Length::Fixed(CELL_WIDTH))
            .center_x(Length::Fixed(CELL_WIDTH)),
        );
    }

    let offset = first_weekday(show_year, show_month) as i32;
    let last_day = days_in_month(show_year, show_month) as i32;
    let selected_day = calendar.selected.map(|date| date.day as i32);
    let show_selected = calendar
        .selected
        .is_some_and(|date| date.year == show_year && date.month == show_month)
        && calendar.range.is_none();
    let from = calendar.range.map(|(from, _)| from.key());
    let to = calendar.range.map(|(_, to)| to.key());

    let mut weeks = column![].spacing(0);
    for week in 0..6 {
        let mut cells = row![].spacing(0);
        for col in 0..7 {
            let day = week * 7 + col - offset + 1;
            let in_month = day >= 1 && day <= last_day;
            let key = show_year * 10000 + show_month as i32 * 100 + day;
            let blocked =
                in_month && calendar.block_weekends && (col == 0 || col == 6);
            let marked = in_month && show_selected && Some(day) == selected_day;
            let endpoint = in_month
                && from.is_some()
                && (Some(key) == from || Some(key) == to);
            let inside = in_month
                && match (from, to) {
                    (Some(from), Some(to)) => {
                        key > from.min(to) && key < from.max(to)
                    }
                    _ => false,
                };
            let message = if in_month && !blocked {
                calendar.on_select.as_ref().map(|select| {
                    select(Date::new(show_year, show_month, day as u32))
                })
            } else {
                None
            };
            cells = cells.push(day_cell(
                day, in_month, blocked, marked, endpoint, inside, message,
            ));
        }
        weeks = weeks.push(cells);
    }

    column![header, weekdays, weeks]
        .spacing(8.0)
        .width(Length::Fixed(MONTH_WIDTH))
        .into()
}

impl<'a, Message> From<Calendar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(mut calendar: Calendar<'a, Message>) -> Self {
        let count = calendar.columns.max(1);
        let bordered = calendar.bordered;
        let mut previous = calendar.previous.take();
        let mut next = calendar.next.take();
        let mut months = row![]
            .spacing(MONTH_GAP)
            .align_y(iced::Alignment::Start);

        for offset in 0..count as i32 {
            let absolute = calendar.year * 12 + calendar.month as i32 - 1 + offset;
            let show_year = absolute.div_euclid(12);
            let show_month = (absolute.rem_euclid(12) + 1) as u32;
            let first = offset == 0;
            let last = offset == count as i32 - 1;
            months = months.push(month(
                &calendar,
                show_year,
                show_month,
                if first {
                    previous.take().or_else(|| Some(text("\u{2039}").size(16.0).into()))
                } else {
                    None
                },
                if last {
                    next.take().or_else(|| Some(text("\u{203a}").size(16.0).into()))
                } else {
                    None
                },
            ));
        }

        let width = MONTH_WIDTH * count as f32
            + if count > 1 {
                MONTH_GAP * (count as f32 - 1.0)
            } else {
                0.0
            }
            + PANEL_PADDING * 2.0;

        container(months)
            .padding(PANEL_PADDING)
            .width(Length::Fixed(width))
            .style(move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                container::Style {
                    text_color: Some(tokens.foreground),
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    border: Border {
                        color: tokens.border,
                        width: if bordered { 1.0 } else { 0.0 },
                        radius: if bordered { theme::RADIUS_LG } else { 0.0 }.into(),
                    },
                    shadow: Shadow::default(),
                    ..container::Style::default()
                }
            })
            .into()
    }
}
