//! A segmented time-of-day field. Mirrors the Slint `time-field` beside it:
//! one rounded frame, one segment per part — hour, minute, optional second and
//! optional AM/PM — and the segment that holds the caret filled with the
//! selection colour, with the caller owning the value.
//!
//! The Slint file is the authority, and it is one frame with a row inside it:
//!
//! - the frame is the field's: `input-background`, a one-pixel `input` edge
//!   that turns `danger` while the value is rejected, `radius-md` and
//!   `control-height` for the size. The iced port draws that frame with
//!   [`input::Style`], the styled frame `input.rs` draws its own field with, so
//!   the fill and the edge are the field's rather than a second spelling;
//! - each numeric segment is iced's text field — the only editable primitive
//!   iced has — fixed at the Slint `segment` width, monospaced, centred, and
//!   filled with `Theme.selection` while it holds the caret, which is the
//!   Slint `background: hour-field.has-focus ? active : transparent`;
//! - the colon between two segments is the muted label the Slint row draws, and
//!   the period is the small semibold `AM`/`PM` text it draws, set off by the
//!   same `ml_1` inset.
//!
//! The value is the caller's, exactly as [`Input`](super::input::Input) takes
//! its text: the caller sets [`hour`](TimeField::hour),
//! [`minute`](TimeField::minute) and [`second`](TimeField::second), and
//! [`on_change`](TimeField::on_change) reports a [`Change`] whenever an edit
//! moves the time. The component clamps a typed value to the segment it was
//! typed in — `0`–`23` for an hour on a 24-hour clock, `1`–`12` plus AM/PM on a
//! 12-hour one, `0`–`59` for minutes and seconds — which is the Slint `min`/
//! `max` pair in its `edited` handlers.
//!
//! **What iced 0.14 cannot express, named rather than faked:**
//!
//! - **No focus or blur event reaches a widget**, so the GPUI model of one Tab
//!   stop with a selected segment inside it is not available: each segment is
//!   its own focusable field and Tab walks them, which [`focus`] can address
//!   programmatically. A segment draws its selection fill from iced's own
//!   focus status, so the fill still marks the segment being edited.
//! - **Arrow keys and `a`/`p` are not reported.** The GPUI field steps a segment
//!   with Up/Down, moves between them with Left/Right, and sets the period with
//!   `a`/`p`; none of those keys is handed to a widget, so none is invented.
//!   The period is therefore the read-only label the Slint component draws, and
//!   the caller changes the cycle or the hour.
//! - **The frame's focus ring has no seam.** A `container` resolves its style
//!   from the theme alone and cannot see whether the field inside has focus, the
//!   same limit `input.rs` names, so the ring stays on the segment's fill and
//!   the frame keeps its resting edge.
//! - **No whole-field opacity.** Slint draws a disabled field at `0.5`; iced has
//!   no subtree opacity, so a disabled field takes the field's muted fill and
//!   muted text instead, as [`Input`](super::input::Input) does.
//! - **The two-digit entry buffer is stateless.** GPUI holds a half-typed first
//!   digit; a stateless port has nowhere to keep it, so a typed value is parsed
//!   from what the segment now holds, preferring a value that fits the segment's
//!   ceiling. Selecting the segment's text first — `Ctrl`/`Cmd`+`A`, or a triple
//!   click, which iced's field already handles — makes a keystroke replace the
//!   value the way Slint's select-on-focus does.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::font::Weight;
use iced::widget::operation;
use iced::widget::{Id, container, row, text, text_input};
use iced::{
    Alignment, Background, Border, Color, Element, Font, Length, Padding, Pixels, Task, Theme,
};

use super::input;
use super::theme::{self, ControlSize, Tokens};

/// The Slint `segment` width: two tabular digits plus the `px_0p5` the GPUI
/// field gives each side of a segment. The same 22px at every size, as Slint
/// sets it.
const SEGMENT: f32 = 22.0;

/// The Slint segment's own `radius-sm`, so the selection fill follows the
/// corner of the frame's `radius-md`.
const SEGMENT_RADIUS: f32 = theme::RADIUS_SM;

/// How much of the field is shown: `hours_minutes` or
/// `hours_minutes_seconds`, the Slint `TimePrecision`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimePrecision {
    /// Hours and minutes, e.g. `09:30`.
    #[default]
    HoursMinutes,
    /// Hours, minutes and seconds, e.g. `09:30:15`.
    HoursMinutesSeconds,
}

/// How the field counts the hours of a day, the Slint `HourCycle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HourCycle {
    /// A 24-hour clock from `00` to `23`.
    #[default]
    H24,
    /// A 12-hour clock from `12` to `11`, with an AM/PM label — the Slint `h12`.
    H12,
}

/// One editable part of the field, so an application can address it — [`focus`]
/// is the one that moves the caret.
///
/// The period is not a segment an application can focus: the Slint component
/// draws it as a label rather than an editor, and so does this port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Segment {
    Hour,
    Minute,
    Second,
}

impl Segment {
    /// The segment's name, for the field id.
    fn name(self) -> &'static str {
        match self {
            Self::Hour => "hour",
            Self::Minute => "minute",
            Self::Second => "second",
        }
    }

    /// The largest value a digit typed into this segment may hold, the Slint
    /// `min(23, …)` / `min(59, …)` ceilings. A 12-hour clock tops an hour at
    /// `12`; `0` and `12` both mean 12, which is [`with_value`]'s rule.
    fn max(self, hour_cycle: HourCycle) -> u32 {
        match (self, hour_cycle) {
            (Self::Hour, HourCycle::H24) => 23,
            (Self::Hour, HourCycle::H12) => 12,
            (Self::Minute | Self::Second, _) => 59,
        }
    }
}

/// The time an edit left behind, reported by
/// [`on_change`](TimeField::on_change): the Slint `changed(hour, minute,
/// second)`.
///
/// The fields are private and read through the methods, so a later field can be
/// added without breaking a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Change {
    hour: u32,
    minute: u32,
    second: u32,
}

impl Change {
    /// The hour on a 24-hour clock, whatever cycle the field displays.
    pub fn hour(&self) -> u32 {
        self.hour
    }

    pub fn minute(&self) -> u32 {
        self.minute
    }

    pub fn second(&self) -> u32 {
        self.second
    }
}

/// A segmented time field, built with the methods below.
///
/// The `id` names the field and every segment's `Id` is built from it —
/// `"start"` gives `start-hour`, `start-minute`, `start-second` — so it takes a
/// `String` rather than an `iced::widget::Id`, which does not give its name
/// back. The caller owns the time: it sets the three parts from its own state
/// and takes every edit back through [`on_change`](Self::on_change).
pub struct TimeField<'a, Message> {
    id: String,
    hour: u32,
    minute: u32,
    second: u32,
    precision: TimePrecision,
    hour_cycle: HourCycle,
    size: ControlSize,
    disabled: bool,
    invalid: bool,
    on_change: Option<Rc<dyn Fn(Change) -> Message + 'a>>,
}

impl<'a, Message> TimeField<'a, Message> {
    /// A field named `id`, at `00:00`, on a 24-hour clock.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            hour: 0,
            minute: 0,
            second: 0,
            precision: TimePrecision::default(),
            hour_cycle: HourCycle::default(),
            size: ControlSize::default(),
            disabled: false,
            invalid: false,
            on_change: None,
        }
    }

    /// The hour on a 24-hour clock, `0`–`23`, whatever cycle the field shows.
    /// The caller owns it, the way [`Input::value`](super::input::Input::value)
    /// owns its text.
    pub fn hour(mut self, hour: u32) -> Self {
        self.hour = hour;
        self
    }

    /// The minute, `0`–`59`.
    pub fn minute(mut self, minute: u32) -> Self {
        self.minute = minute;
        self
    }

    /// The second, `0`–`59`. Hidden unless the precision shows seconds; the
    /// value is kept either way, as the Slint `second` property is.
    pub fn second(mut self, second: u32) -> Self {
        self.second = second;
        self
    }

    /// Whether a seconds segment is drawn, the Slint `precision`.
    pub fn precision(mut self, precision: TimePrecision) -> Self {
        self.precision = precision;
        self
    }

    /// Whether the hours read `00`–`23` or `12`–`11` with an AM/PM label, the
    /// Slint `hour-cycle`.
    pub fn hour_cycle(mut self, hour_cycle: HourCycle) -> Self {
        self.hour_cycle = hour_cycle;
        self
    }

    /// The size ladder the frame's height and the type take from: `20 / 24 / 32
    /// / 40`, which is the Slint `control-height`, not the field's
    /// `input-height`.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Refuses input and draws the muted field, the way Slint's `enabled: false`
    /// does. A field with no [`on_change`](Self::on_change) is disabled without
    /// this.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Reports the value as rejected: the frame's edge takes the danger colour.
    /// Slint's `invalid`, and edits are still accepted while it is set.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Every edit, as the [`Change`] it leaves behind. Slint's
    /// `changed(hour, minute, second)`.
    pub fn on_change(mut self, on_change: impl Fn(Change) -> Message + 'a) -> Self {
        self.on_change = Some(Rc::new(on_change));
        self
    }
}

/// The `Id` of one segment of the field named `id` — `"start"` with
/// [`Segment::Hour`] gives `start-hour` — so an application can address it.
pub fn segment_id(id: &str, segment: Segment) -> Id {
    Id::from(format!("{id}-{}", segment.name()))
}

/// Puts the caret in one segment and selects what is in it, so the next typed
/// digit replaces the value instead of appending to it — the Slint segment's
/// behaviour when it takes focus, and the reason a two-digit entry does not need
/// a half-typed buffer.
///
/// The Task is returned from `update`; natively and in the browser it is the
/// same widget operation.
pub fn focus<Message: 'static>(id: &str, segment: Segment) -> Task<Message> {
    let segment = segment_id(id, segment);

    Task::batch([
        operation::focus(segment.clone()),
        operation::select_all(segment),
    ])
}

/// Whether an hour is in the afternoon, the Slint `afternoon`.
fn is_afternoon(hour: u32) -> bool {
    hour >= 12
}

/// The value a segment shows: the hour follows the cycle — 0 and 12 read as 12
/// on a 12-hour clock — and the rest are themselves.
fn display_value(
    segment: Segment,
    hour: u32,
    minute: u32,
    second: u32,
    hour_cycle: HourCycle,
) -> u32 {
    match segment {
        Segment::Hour => match hour_cycle {
            HourCycle::H24 => hour,
            HourCycle::H12 => (hour + 11) % 12 + 1,
        },
        Segment::Minute => minute,
        Segment::Second => second,
    }
}

/// The value the time takes when `segment` is set to `value`, with the hour
/// mapped back onto the 24-hour clock a 12-hour field displays — the Slint
/// `display-hour` read in reverse.
fn with_value(segment: Segment, value: u32, hour: u32, hour_cycle: HourCycle) -> u32 {
    match segment {
        Segment::Hour => match hour_cycle {
            HourCycle::H24 => value,
            // 0 and 12 both read as 12, and either keeps the half of the day the
            // hour already belonged to.
            HourCycle::H12 => (value % 12) + (hour / 12) * 12,
        },
        Segment::Minute | Segment::Second => value,
    }
}

/// The value a segment's text now holds, clamped to `max`.
///
/// The digits are read from the end, because iced's field appends to what it
/// holds: a value that fits whole wins, then the last two digits, then the last
/// one, so `018` reads as 18, `091` reads as 1 rather than being clamped from
/// 91, and an empty segment is zero.
fn typed(raw: &str, max: u32) -> u32 {
    let digits: String = raw.chars().filter(|character| character.is_ascii_digit()).collect();
    let count = digits.len();
    if count == 0 {
        return 0;
    }

    let mut lengths = vec![count, count.min(2), 1];
    lengths.dedup();
    for length in lengths {
        if let Ok(value) = digits[count - length..].parse::<u32>() {
            if value <= max {
                return value;
            }
        }
    }
    0
}

/// The colours a segment's field draws with: transparent at rest, the selection
/// fill while it holds the caret. It is iced's field, not a frame around it, so
/// the focus status is visible and the selection can move with it — the reason
/// the Slint row's focused rectangle is a field of its own here.
fn segment_style(disabled: bool) -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    move |theme: &Theme, status| {
        let tokens = Tokens::of(theme);
        let focused = matches!(status, text_input::Status::Focused { .. });

        text_input::Style {
            background: if focused && !disabled {
                Background::Color(tokens.selection)
            } else {
                Background::Color(Color::TRANSPARENT)
            },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: SEGMENT_RADIUS.into(),
            },
            icon: tokens.muted_foreground,
            placeholder: tokens.muted_foreground,
            value: if disabled {
                tokens.muted_foreground
            } else {
                tokens.foreground
            },
            selection: theme::alpha(tokens.selection, 0.35),
        }
    }
}

/// The muted `:` between two segments, the Slint row's colon label.
fn colon<'a, Message: 'a>(font_size: f32) -> Element<'a, Message> {
    text(":")
        .size(font_size)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// The `AM`/`PM` label on a 12-hour clock, set off by the Slint `ml_1` inset.
///
/// It is the read-only label the Slint component draws: iced reports no key
/// event to a widget, so GPUI's `a`/`p` toggle has nowhere to live, and a
/// pointer-only control would be a control without a keyboard path.
fn period<'a, Message: 'a>(hour: u32) -> Element<'a, Message> {
    container(
        text(if is_afternoon(hour) { "PM" } else { "AM" })
            .size(theme::FONT_SIZE_XS)
            .font(Font {
                weight: Weight::Semibold,
                ..Font::DEFAULT
            })
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    )
    .padding(Padding {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: theme::SPACING_XS,
    })
    .into()
}

/// One numeric segment: iced's field, fixed at the Slint width, monospaced,
/// centred, and styled so the focused one carries the selection fill.
///
/// A field with no way to report an edit is left without `on_input`, which is
/// iced's own disabled state, the way a button with no message is disabled.
fn numeric_segment<'a, Message: Clone + 'a>(
    id: &str,
    segment: Segment,
    hour: u32,
    minute: u32,
    second: u32,
    hour_cycle: HourCycle,
    font_size: f32,
    height: f32,
    disabled: bool,
    on_change: &Option<Rc<dyn Fn(Change) -> Message + 'a>>,
) -> Element<'a, Message> {
    let value = display_value(segment, hour, minute, second, hour_cycle);
    let shown = format!("{value:02}");

    let mut field = text_input("", &shown)
        .id(segment_id(id, segment))
        // The frame owns the padding and the edge, so the field draws only its
        // own selection fill and its digits inside them.
        .padding(Padding::ZERO)
        .width(Length::Fixed(SEGMENT))
        .size(font_size)
        // A line box as tall as the field's inside, so the selection fill covers
        // the segment rather than a line in the middle of it, as `input.rs` sizes
        // its own field.
        .line_height(Pixels(height - 2.0))
        .align_x(Alignment::Center)
        .font(theme::MONO)
        .style(segment_style(disabled));

    if !disabled {
        if let Some(on_change) = on_change.clone() {
            let max = segment.max(hour_cycle);
            field = field.on_input(move |raw| {
                let value = typed(&raw, max);
                match segment {
                    Segment::Hour => on_change(Change {
                        hour: with_value(Segment::Hour, value, hour, hour_cycle),
                        minute,
                        second,
                    }),
                    Segment::Minute => on_change(Change {
                        hour,
                        minute: value,
                        second,
                    }),
                    Segment::Second => on_change(Change {
                        hour,
                        minute,
                        second: value,
                    }),
                }
            });
        }
    }

    field.into()
}

impl<'a, Message> From<TimeField<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: TimeField<'a, Message>) -> Self {
        let TimeField {
            id,
            hour,
            minute,
            second,
            precision,
            hour_cycle,
            size,
            disabled,
            invalid,
            on_change,
        } = control;

        // A field with no way to report an edit is disabled, the way a button
        // with no message is: iced would otherwise accept a digit and drop it.
        let disabled = disabled || on_change.is_none();
        let height = size.control_height();
        let font_size = size.control_font_size();
        let frame = input::Style::new().invalid(invalid).disabled(disabled);
        let with_seconds = precision == TimePrecision::HoursMinutesSeconds;
        let with_period = hour_cycle == HourCycle::H12;

        let mut segments = row![]
            .spacing(1.0)
            .align_y(Alignment::Center)
            .height(Length::Fill);

        segments = segments.push(numeric_segment(
            &id,
            Segment::Hour,
            hour,
            minute,
            second,
            hour_cycle,
            font_size,
            height,
            disabled,
            &on_change,
        ));
        segments = segments.push(colon(font_size));
        segments = segments.push(numeric_segment(
            &id,
            Segment::Minute,
            hour,
            minute,
            second,
            hour_cycle,
            font_size,
            height,
            disabled,
            &on_change,
        ));
        if with_seconds {
            segments = segments.push(colon(font_size));
            segments = segments.push(numeric_segment(
                &id,
                Segment::Second,
                hour,
                minute,
                second,
                hour_cycle,
                font_size,
                height,
                disabled,
                &on_change,
            ));
        }
        if with_period {
            segments = segments.push(period(hour));
        }

        // `px_1` in the GPUI field: the segments sit just inside the frame — on
        // all four sides, the same inset `textarea.slint` gives its frame. Each
        // segment's line box is a line taller than this padded inside, so iced
        // clamps it there and the caret is inset from the frame's edge instead of
        // running its full height. The segments row is `Fill` and centred, so the
        // digits do not move, and the outer height is `Fixed`. The width is the
        // content's — iced does not stretch a container the way Slint's layout
        // would, so the field is as wide as its segments and stays that width
        // because each segment is padded to two digits.
        container(segments)
            .padding(theme::SPACING_XS)
            .height(Length::Fixed(height))
            .width(Length::Shrink)
            .style(move |theme: &Theme| frame.resolve(Tokens::of(theme)))
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_typed_value_is_clamped_to_the_segment() {
        // The whole text when it fits, so a padded value that was appended to
        // reads as itself.
        assert_eq!(typed("18", 23), 18);
        assert_eq!(typed("018", 23), 18);
        assert_eq!(typed("59", 59), 59);
        // A value over the ceiling falls back to the last digit rather than
        // being clamped from the whole text.
        assert_eq!(typed("091", 23), 1);
        assert_eq!(typed("095", 59), 5);
        // Anything that is not a digit is dropped, and an empty segment is zero.
        assert_eq!(typed("", 23), 0);
        assert_eq!(typed("x", 23), 0);
        assert_eq!(typed("1x2", 23), 12);
    }

    #[test]
    fn a_twelve_hour_clock_maps_the_hour_both_ways() {
        assert_eq!(display_value(Segment::Hour, 0, 0, 0, HourCycle::H12), 12);
        assert_eq!(display_value(Segment::Hour, 9, 0, 0, HourCycle::H12), 9);
        assert_eq!(display_value(Segment::Hour, 14, 0, 0, HourCycle::H12), 2);
        // 12 AM is 0 and 12 PM is 12; a morning hour stays in the morning.
        assert_eq!(with_value(Segment::Hour, 12, 3, HourCycle::H12), 0);
        assert_eq!(with_value(Segment::Hour, 12, 15, HourCycle::H12), 12);
        assert_eq!(with_value(Segment::Hour, 9, 15, HourCycle::H12), 21);
        // A 24-hour clock takes the value itself.
        assert_eq!(with_value(Segment::Hour, 9, 15, HourCycle::H24), 9);
    }
}
