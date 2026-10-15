//! A date field that opens a calendar. Mirrors GPUI Component's `DatePicker`
//! and the Slint `date-picker` beside it, and is the overlay family's last
//! composition: the field's frame, the popover's floating surface, and the
//! `calendar` that is already done.
//!
//! # What it composes
//!
//! The Slint file is the authority, and it is three pieces:
//!
//! - the trigger is a field: `Theme.control-height`, `Theme.control-padding` and
//!   `Theme.control-font-size`, a one-pixel `Theme.input` edge, `radius-md` and
//!   the value on the leading edge with the calendar glyph on the trailing one.
//!   The iced port draws that frame with [`input::Style`], the styled frame
//!   `input.rs` draws its own field with, so the fill and the edge are the
//!   field's rather than a second spelling. Slint's date-picker writes
//!   `Theme.background` where the field writes `input-background`; the field's
//!   token is the one that separates a trigger from the page in the dark theme,
//!   which is why the composition takes the field's frame, exactly as `select`
//!   does for the same reason;
//! - the panel is the popover's floating surface: [`Popover::filled(true)`] draws
//!   the popover fill, a one-pixel edge, `radius-lg` and the shadow, and the
//!   12px inset is the Slint popup's `padding: 12px`. The calendar is asked for
//!   `bordered(false)` so the panel has one surface rather than a border inside a
//!   border — the rule [`Menu`](super::menu::Menu) states, and the only
//!   difference from the Slint popup, which nests the calendar's own edge inside
//!   its own;
//! - the content is [`Calendar`]: `columns` is the Slint `months`, `selected` is
//!   `marked-day`, `range` is `from-key`/`to-key` and `block_weekends` is
//!   `block-weekends`. When `with-time` is set, a one-pixel rule and a time row
//!   follow it, which is the Slint `if root.with-time` block.
//!
//! # The state is the caller's
//!
//! iced 0.14 keeps no state a component can own across frames, so this port has
//! none either. The caller owns:
//!
//! - the month the calendar shows, [`DatePicker::new`]'s `year`/`month`. Slint
//!   keeps it privately as `view-year`/`view-month`; iced cannot, so the caller
//!   holds it and moves it with [`DatePicker::on_shift`], the same arrangement
//!   [`Calendar`] already has. The selected month of a value is not the shown
//!   one: choosing a day reports through [`DatePicker::on_select`] and the caller
//!   decides whether to move the view to it;
//! - the value, [`DatePicker::date`] or [`DatePicker::range`], and the open flag,
//!   [`DatePicker::open`]. [`DatePicker::on_open_change`] reports a trigger click,
//!   an outside click or Escape, because the popover supplies all three; a pick
//!   is reported by [`DatePicker::on_select`] and closes nothing, so the caller
//!   answers with its own `open` — the Slint `choose` closes the popup after a
//!   single date and after the second end of a range, which is the caller's
//!   decision here;
//! - the time text, [`DatePicker::hour`] and [`DatePicker::minute`], and its two
//!   edits through [`DatePicker::on_hour`] and [`DatePicker::on_minute`]. Slint
//!   keeps `hour`/`minute` as integers and a text buffer and clamps in
//!   `commit-time`; with no state to hold either, the port keeps the text the
//!   caller owns and reports it back, and clamping a typed value is the
//!   application's, exactly as parsing is.
//!
//! # What the Slint component has that this does not
//!
//! - **The `changed(year, month, day)` report is one day, not a time.** The
//!   Slint callback carries the day only; the port reports the same
//!   [`Date`] through [`on_select`](Self::on_select), and the time is the two
//!   text callbacks above.
//! - **No year-selection grid.** GPUI's picker has a `set_year_range` and a year
//!   mode; the Slint `Calendar` has neither, only the two month arrows, so this
//!   port has neither either. "A custom year range" is therefore the caller
//!   drawing a year: [`DatePicker::new`] is given whatever year it wants and
//!   every shift is reported, so a caller that wants 1900–2100 clamps
//!   [`on_shift`](Self::on_shift)'s answer in its own `update`.
//! - **No disabled-date matcher.** The Slint `Calendar` has only
//!   `block-weekends`; GPUI's `Matcher` (a range, an interval, a custom
//!   predicate) is not in the authority, so none is invented. Weekends are the
//!   one restriction the port reproduces.
//! - **No built-in presets.** GPUI's `.presets(...)` is not in the Slint
//!   component. The demo's "Preset Ranges" card builds them from ordinary
//!   [`Button`](super::button::Button)s that set the caller's own value, which is
//!   what a preset is.
//! - **`cleanable`, `date_format`, `first_day_of_week`, `time_precision` and
//!   `hour_cycle` are GPUI-only.** The Slint component formats a fixed
//!   `yyyy-mm-dd`, starts the week on Sunday and has no clear control, no date
//!   format string and no 12-hour clock, so none is added here.
//! - **The time row's boxes are the field's size ladder, not a fixed 28px.**
//!   Slint draws each of the hour and minute boxes at a fixed 28px; iced has no
//!   time field, so the port composes the two from [`input::Input`] at the
//!   picker's nearest small size. The structure is Slint's — a "Time" label, a
//!   box, a colon, a box — and the height is the field's rather than a fresh
//!   number.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{column, container, row, rule, svg, text};
use iced::{Alignment, Element, Length, Theme};

use super::calendar::{self, Calendar, Date};
use super::input;
use super::popover::{Placement, Popover};
use super::theme::{self, ControlSize, Tokens};

/// The popover's inset, the Slint popup's `padding: 12px`.
const PANEL_INSET: f32 = 12.0;

/// The gap between the trigger and the panel, the Slint popup's `y: height + 4px`.
const PANEL_GAP: f32 = 4.0;

/// A date field that opens a calendar, built with the methods below.
///
/// It becomes an [`Element`] through `From`, and what it becomes is a
/// [`Popover`] whose trigger is the field-shaped frame and whose panel holds a
/// [`Calendar`] — which is exactly what the Slint control is: an input-shaped row
/// with a `PopupWindow` holding the grid.
pub struct DatePicker<'a, Message> {
    /// The month the calendar shows. The caller owns it — see the module
    /// comment.
    year: i32,
    month: u32,
    /// The single chosen day, or nothing while the placeholder shows.
    date: Option<Date>,
    /// An inclusive range. When it is set it is what the trigger and the
    /// calendar draw, and `date` is ignored, which is the Slint rule that a
    /// range's endpoints replace `marked-day`.
    range: Option<(Date, Date)>,
    placeholder: String,
    size: ControlSize,
    months: usize,
    block_weekends: bool,
    with_time: bool,
    hour: String,
    minute: String,
    open: bool,
    disabled: bool,
    appearance: bool,
    width: Length,
    /// The base name of the two time fields, so two pickers in one application
    /// do not share iced's editing state. It is the caller's to name.
    id: String,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    on_select: Option<Box<dyn Fn(Date) -> Message + 'a>>,
    on_shift: Option<Box<dyn Fn(i32) -> Message + 'a>>,
    on_hour: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_minute: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

impl<'a, Message: Clone + 'static> DatePicker<'a, Message> {
    /// A picker whose calendar shows `year`/`month`, with nothing chosen and the
    /// panel closed.
    ///
    /// The month shown is the caller's, not the value's; Slint keeps it
    /// privately as `view-year`/`view-month`, and iced cannot, so it is an
    /// argument and a report. See the module comment.
    pub fn new(year: i32, month: u32) -> Self {
        Self {
            year,
            month: month.clamp(1, 12),
            date: None,
            range: None,
            placeholder: String::new(),
            size: ControlSize::Medium,
            months: 1,
            block_weekends: false,
            with_time: false,
            hour: String::from("00"),
            minute: String::from("00"),
            open: false,
            disabled: false,
            appearance: true,
            width: Length::Fill,
            id: String::from("date-picker"),
            on_open_change: None,
            on_select: None,
            on_shift: None,
            on_hour: None,
            on_minute: None,
        }
    }

    /// The single chosen day, the Slint `day`/`month`/`year`. `None` — the
    /// default — shows the placeholder, which is the Slint `empty` state.
    pub fn date(mut self, date: Option<Date>) -> Self {
        self.date = date;
        self
    }

    /// An inclusive range: its endpoints are filled and the days between them
    /// are tinted, which is the Slint `range` with `from-key`/`to-key`. When it
    /// is set it is what the trigger draws and what the calendar marks, and
    /// [`date`](Self::date) is ignored.
    ///
    /// A half-made range is the caller's to spell: pass `Some((start, start))`
    /// while only the start is chosen — the calendar marks the one day and the
    /// trigger shows it — and the pair once the end is chosen. The demo's
    /// "Advanced Examples" card does exactly that.
    pub fn range(mut self, range: Option<(Date, Date)>) -> Self {
        self.range = range;
        self
    }

    /// The text shown while nothing is chosen, "" by default. Slint's
    /// `placeholder`.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// The trigger's size, [`ControlSize::Medium`] by default. It picks the
    /// height, the inset and the type together, the way the Slint component's
    /// `size` picks `Theme.control-height`, `control-padding` and
    /// `control-font-size`.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// How many months the calendar shows side by side: 1, or 2–3. This is the
    /// Slint `months`, which is 2 by default in range mode; a caller that wants
    /// that passes `2` beside [`range`](Self::range), and iced has no range flag
    /// to derive it from.
    pub fn months(mut self, months: usize) -> Self {
        self.months = months.clamp(1, 3);
        self
    }

    /// Weekends cannot be picked and are lettered faintly. This is the whole of
    /// the Slint `block-weekends`; the port has no other date restriction. See
    /// the module comment.
    pub fn block_weekends(mut self, block: bool) -> Self {
        self.block_weekends = block;
        self
    }

    /// Whether the picker draws a field frame. `false` drops the fill, the edge
    /// and the radius for a host that supplies the surface — GPUI's
    /// `appearance(false)`, which the Slint component does not have.
    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }

    /// A picker that answers nothing: the trigger is drawn muted and the popover
    /// is told no open state, so it cannot be opened. This is the way this crate
    /// spells a control with no report, and it is GPUI's `disabled`; the Slint
    /// component has no disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Show the time row under the calendar and the time beside the date in the
    /// trigger. This is the Slint `with-time`. The two boxes are text fields the
    /// caller drives — see [`hour`](Self::hour) and [`on_hour`](Self::on_hour).
    pub fn with_time(mut self, with_time: bool) -> Self {
        self.with_time = with_time;
        self
    }

    /// The hour the time row shows, "00" by default. The caller owns the text,
    /// exactly as it owns the date; Slint's `hour-text`.
    pub fn hour(mut self, hour: impl Into<String>) -> Self {
        self.hour = hour.into();
        self
    }

    /// The minute the time row shows, "00" by default. Slint's `minute-text`.
    pub fn minute(mut self, minute: impl Into<String>) -> Self {
        self.minute = minute.into();
        self
    }

    /// Whether the panel is showing. The caller owns it, and
    /// [`on_open_change`](Self::on_open_change) is how the caller learns it
    /// should change.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// The trigger's width, `Length::Fill` by default so the field spans the form
    /// column it is placed in — the Slint `horizontal-stretch: 1`.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// The base name of the two time fields, `"date-picker"` by default. Two
    /// pickers that both show a time need different names, because iced keeps a
    /// text field's caret and focus under its id and would otherwise share them.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Report every request to change the open state: a trigger click wants
    /// `true`, an outside click or Escape wants `false`. A disabled picker is
    /// given no callback, so its trigger cannot open anything.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// A day was picked, the Slint `changed(year, month, day)`. The panel is not
    /// closed; the caller answers with its own `open`, exactly as the Slint
    /// `choose` closes the popup itself.
    pub fn on_select(mut self, on_select: impl Fn(Date) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// The month should shift by this many months, forwarded from the calendar's
    /// two arrows. The caller applies it to its own year and month, exactly as
    /// the Slint `shift` does.
    pub fn on_shift(mut self, on_shift: impl Fn(i32) -> Message + 'a) -> Self {
        self.on_shift = Some(Box::new(on_shift));
        self
    }

    /// The hour field was edited, as the text it now holds.
    pub fn on_hour(mut self, on_hour: impl Fn(String) -> Message + 'a) -> Self {
        self.on_hour = Some(Box::new(on_hour));
        self
    }

    /// The minute field was edited, as the text it now holds.
    pub fn on_minute(mut self, on_minute: impl Fn(String) -> Message + 'a) -> Self {
        self.on_minute = Some(Box::new(on_minute));
        self
    }
}

/// The month the calendar draws, and the whole panel's width, from how many
/// months it shows: the grid the [`Calendar`] builds plus the popover's inset on
/// each side. A panel narrower than its grid would clip the last month's days.
fn panel_width(months: usize) -> f32 {
    let count = months.clamp(1, 3) as f32;
    let grid = calendar::MONTH_WIDTH * count
        + if count > 1.0 {
            calendar::MONTH_GAP * (count - 1.0)
        } else {
            0.0
        }
        + calendar::PANEL_PADDING * 2.0;
    grid + PANEL_INSET * 2.0
}

/// The time row: the Slint `if root.with_time` block after a one-pixel rule.
///
/// Slint draws a "Time" label, a 44px box around a text field, a colon and
/// another box. iced has no time field, so each box is an [`input::Input`] — the
/// same frame and text field the date's own field would draw — sized at
/// [`ControlSize::Small`] and centred. The caller owns both strings and both
/// edits; clamping a typed value is the application's, as the module comment
/// says.
fn time_row<'a, Message: Clone + 'a>(
    id: &str,
    hour: &str,
    minute: &str,
    on_hour: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_minute: Option<Box<dyn Fn(String) -> Message + 'a>>,
) -> Element<'a, Message> {
    let boxed = |suffix: &str,
                 value: &str,
                 on_input: Option<Box<dyn Fn(String) -> Message + 'a>>|
     -> Element<'a, Message> {
        let mut field = input::Input::new(format!("{id}-{suffix}"))
            .value(value)
            .size(ControlSize::Small)
            .align_x(Alignment::Center)
            .width(Length::Fixed(44.0));
        if let Some(on_input) = on_input {
            field = field.on_input(on_input);
        }
        field.into()
    };

    row![
        text("Time")
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
        boxed("hour", hour, on_hour),
        text(":").size(theme::FONT_SIZE_SM),
        boxed("minute", minute, on_minute),
    ]
    .spacing(theme::SPACING_SM)
    .align_y(Alignment::Center)
    .into()
}

impl<'a, Message> DatePicker<'a, Message>
where
    Message: Clone + 'static,
{
    /// The whole control: the [`Popover`] that floats the [`Calendar`], with the
    /// field-shaped trigger.
    fn into_element(self) -> Element<'a, Message> {
        let DatePicker {
            year,
            month,
            date,
            range,
            placeholder,
            size,
            months,
            block_weekends,
            with_time,
            hour,
            minute,
            open,
            disabled,
            appearance,
            width,
            id,
            on_open_change,
            on_select,
            on_shift,
            on_hour,
            on_minute,
        } = self;

        let has_range = range.is_some();
        // The Slint `label`: a completed range joins its ends with an en dash, a
        // half-made one shows its start, a single date shows the date, and time
        // follows whatever the date is. Nothing chosen shows the placeholder.
        let labelled = match range {
            Some((from, to)) if from != to => Some(format!("{from} \u{2013} {to}")),
            Some((from, _)) => Some(from.to_string()),
            None => date.map(|date| date.to_string()),
        };
        let label = match labelled {
            Some(value) if with_time => format!("{value} {hour}:{minute}"),
            Some(value) => value,
            None => placeholder,
        };
        let empty = !has_range && date.is_none();

        // The calendar glyph on the trailing edge. It is the one icon the control
        // needs, so it draws it rather than asking the caller, the same exception
        // `select` makes for its disclosure mark. The bytes are
        // `crates/slint-component/ui/icons/calendar.svg` (Lucide, ISC).
        let glyph = svg(svg::Handle::from_memory(CALENDAR_XML))
            .width(16.0)
            .height(16.0)
            .style(move |theme: &Theme, _status| svg::Style {
                color: Some(if disabled {
                    Tokens::of(theme).muted_foreground.scale_alpha(0.5)
                } else {
                    Tokens::of(theme).muted_foreground
                }),
            });

        let trigger = container(
            row![
                text(label)
                    .size(size.control_font_size())
                    .width(Length::Fill)
                    // iced 0.14 draws no ellipsis, so a long value clips at the
                    // trigger's edge where Slint elides.
                    .wrapping(text::Wrapping::None),
                glyph,
            ]
            .spacing(theme::SPACING_SM)
            .align_y(Alignment::Center)
            // The row has to fill the trigger's height: a `Shrink` row is
            // aligned to the top of the fixed-height frame, so `align_y` would
            // centre the label and the calendar glyph against the row's own top
            // edge and the row would ride high. `input.rs` fills its row for the
            // same reason.
            .height(Length::Fill)
            .width(Length::Fill),
        )
        .padding([0.0, size.control_padding()])
        .height(Length::Fixed(size.control_height()))
        .width(width)
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            let mut style = input::Style::new()
                .appearance(appearance)
                .disabled(disabled)
                .resolve(tokens);
            // A value reads as a value and a placeholder reads as muted, the
            // Slint `color: empty ? Theme.muted-foreground : Theme.foreground`,
            // plus this crate's disabled ink.
            style.text_color = Some(if disabled || empty {
                tokens.muted_foreground
            } else {
                tokens.foreground
            });
            style
        });

        let mut calendar = Calendar::new(year, month)
            .columns(months)
            .block_weekends(block_weekends)
            // The popover owns the panel's surface; see the module comment.
            .bordered(false)
            .selected(date)
            .range(range);
        if let Some(on_shift) = on_shift {
            calendar = calendar.on_shift(move |delta| on_shift(delta));
        }
        if let Some(on_select) = on_select {
            calendar = calendar.on_select(move |date| on_select(date));
        }

        let mut content = column![calendar].spacing(theme::SPACING_MD);
        if with_time {
            content = content.push(
                rule::horizontal(1).style(|theme: &Theme| rule::Style {
                    color: Tokens::of(theme).border,
                    ..rule::default(theme)
                }),
            );
            content = content.push(time_row(&id, &hour, &minute, on_hour, on_minute));
        }

        let mut popover = Popover::new(trigger)
            .open(open && !disabled)
            .placement(Placement::Bottom)
            .offset(PANEL_GAP)
            .panel_width(panel_width(months))
            .filled(true)
            .padding(PANEL_INSET);
        if !disabled && let Some(on_open_change) = on_open_change {
            popover = popover.on_open_change(on_open_change);
        }

        popover.content(content).into()
    }
}

impl<'a, Message> From<DatePicker<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'static,
{
    fn from(picker: DatePicker<'a, Message>) -> Self {
        picker.into_element()
    }
}

/// The calendar glyph at the trigger's trailing edge, drawn from
/// `crates/slint-component/ui/icons/calendar.svg` (Lucide, ISC), so the two
/// galleries draw the same mark rather than two lookalikes.
const CALENDAR_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 2v3"/><path d="M16 2v3"/><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18"/></svg>"#;
