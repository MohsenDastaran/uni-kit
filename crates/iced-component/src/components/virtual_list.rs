//! A windowed list. Mirrors the Slint `VirtualList` beside it — a flat model, a
//! fixed row height, a selected row, and the range of rows on screen — built on
//! the styled [`Scrollable`](super::scrollable::Scrollable).
//!
//! **This one really does virtualise.** iced 0.14 ships no windowed list, and
//! the [`Scrollable`](super::scrollable::Scrollable) beside this file lays out
//! every child it is given; a list built directly on it is honest but not
//! virtual. What iced does expose is the scroll region's [`Viewport`], through
//! its own `on_scroll` — the offset, the visible bounds and the content bounds,
//! reported whenever the region moves. That is enough to build the window here:
//! this component keeps the caller's offset, derives the rows that offset covers
//! from the list's own fixed height, and hands [`VirtualList`]'s inner column
//! only *those* rows, padding the rest with two spacers. `rows * row_height` of
//! scrollable content is what the scrollbar measures; ten-odd real row elements
//! is what is built. Scrolling 5000 rows never builds 5000 rows.
//!
//! The window is exact because the list owns its height: the region is
//! [`Length::Fixed`], so `ceil(height / row_height)` is the number of rows a
//! viewport can show and no measurement pass is needed before the first frame.
//! [`VirtualList::range`] is that arithmetic, and the demo shows it as the
//! "rows a–b of n are built" line rather than asking anybody to trust a comment.
//!
//! The caller owns the scroll offset, which is the same controlled shape as
//! [`List`](super::list::List)'s selection: the list is handed the offset the
//! application holds and reports the new one through
//! [`VirtualList::on_scroll`], and the application stores it and rebuilds. The
//! offset is the raw `Viewport::absolute_offset().y`, so a programmatic scroll
//! (iced's own `scroll_to`, keyed by [`VirtualList::id`]) and a wheel scroll look
//! the same to the caller.
//!
//! # What iced cannot express, named rather than faked
//!
//! - **One row height per list.** GPUI's `v_virtual_list` takes a size per item
//!   and supports differently sized rows. The Slint component this port answers
//!   to has a single `row-height`, and so does this: the spacer arithmetic needs
//!   a constant pitch, and a title that promised per-index heights would be
//!   lying. The `Variable Item Sizes` example draws the Slint one's shape — a
//!   taller fixed row — not GPUI's per-item sizes.
//! - **No `visible-height` / `viewport-y` properties.** iced has no retained
//!   component object to hold them; the caller's offset and [`VirtualList::range`]
//!   are the same numbers, read where they are needed.
//! - **No `scroll-to` function on the component.** iced scrolls a region by
//!   operation, not by a method on a value element, so [`VirtualList::id`] names
//!   the region and the application issues `scroll_to` itself. The demo's
//!   `Basic Scroll Control` section does exactly that.
//! - **No keyboard.** iced 0.14 reports no widget focus to the application, the
//!   same gap every other port here records; a row is a pointer target.
//!
//! Everything is drawn from [`Tokens`], read inside the row's own style closure,
//! so a theme switch repaints it. Nothing here writes a colour of its own.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one part gets
// a warning for each one it does not want.
#![allow(dead_code)]

use std::ops::Range;

use iced::widget::scrollable::Viewport;
use iced::widget::{column, container, Space};
use iced::{Element, Length};

use super::scrollable::{Axis, Scrollable};

/// A row's height by default, the Slint component's `row-height: 32px`.
pub const ROW_HEIGHT: f32 = 32.0;

/// The area's height by default. A virtual list needs a bounded viewport for the
/// window to mean anything, so it cannot size to its rows the way [`List`] does;
/// a caller that wants another height says so with [`VirtualList::height`].
pub const HEIGHT: f32 = 240.0;

/// How many rows beyond the viewport are built on each side. The spare rows are
/// what keeps a row partially scrolled past the edge from flashing in a frame
/// late; two matches the reference's comfortable margin.
pub const OVERSCAN: usize = 2;

/// A windowed list of `row_count` rows, built with the methods below.
///
/// The rows are the caller's: [`VirtualList::new`] takes a render function that
/// is called once for each row the current window covers, with the row's index
/// and whether the caller has selected it. Returning only the visible rows is
/// the whole point — the function is not called for a row that is off screen.
pub struct VirtualList<'a, Message> {
    row_count: usize,
    row_height: f32,
    height: f32,
    offset: f32,
    overscan: usize,
    selected: Option<usize>,
    id: Option<iced::widget::Id>,
    view: Box<dyn Fn(usize, bool) -> Element<'a, Message> + 'a>,
    on_scroll: Option<Box<dyn Fn(Viewport) -> Message + 'a>>,
}

impl<'a, Message> VirtualList<'a, Message> {
    /// A list of `row_count` rows. `view` is called for each row in the current
    /// window with the row's index and whether it is selected; it is never called
    /// for a row outside the window.
    pub fn new(
        row_count: usize,
        view: impl Fn(usize, bool) -> Element<'a, Message> + 'a,
    ) -> Self {
        Self {
            row_count,
            row_height: ROW_HEIGHT,
            height: HEIGHT,
            offset: 0.0,
            overscan: OVERSCAN,
            selected: None,
            id: None,
            view: Box::new(view),
            on_scroll: None,
        }
    }

    /// How tall one row is, the Slint component's `row-height`. Every row is this
    /// tall; the arithmetic that turns the offset into a row range depends on it.
    pub fn row_height(mut self, row_height: f32) -> Self {
        self.row_height = row_height;
        self
    }

    /// The height of the visible box. It bounds the scroll region and it is the
    /// number of rows the window is computed from, so a list without one has no
    /// window to compute.
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(0.0);
        self
    }

    /// How far the list is scrolled, which the caller stores from
    /// [`VirtualList::on_scroll`]. The Slint component exposes the same number as
    /// its `viewport-y`; here it is an input, because iced elements hold no state.
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset.max(0.0);
        self
    }

    /// How many rows beyond the viewport are built on each side. See
    /// [`OVERSCAN`].
    pub fn overscan(mut self, overscan: usize) -> Self {
        self.overscan = overscan;
        self
    }

    /// Which row is selected, the Slint `selected`. `None`, the default, is the
    /// Slint `-1`; the list reports it to `view` and draws nothing itself.
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// Names the scroll region, so the application can move it with iced's own
    /// `scroll_to` and `scroll_by` operations — the `scroll-to` and
    /// `scroll-to-row` functions the Slint component carries.
    pub fn id(mut self, id: impl Into<iced::widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Called with the [`Viewport`] whenever the region scrolls. The application
    /// stores `viewport.absolute_offset().y` and hands it back through
    /// [`VirtualList::offset`]; that round trip is what moves the window.
    pub fn on_scroll(mut self, on_scroll: impl Fn(Viewport) -> Message + 'a) -> Self {
        self.on_scroll = Some(Box::new(on_scroll));
        self
    }

    /// The rows the current offset covers, overscan included — the Slint
    /// component's `first-visible` and `last-visible` in one range, and the exact
    /// set of rows the render function will be called for. A caller that wants to
    /// show which rows are built (the demos do) reads it here rather than
    /// repeating the arithmetic.
    pub fn range(&self) -> Range<usize> {
        // A row height of zero has no rows in it; the guard keeps the division
        // honest rather than producing an infinite window.
        let row_height = if self.row_height > 0.0 {
            self.row_height
        } else {
            1.0
        };

        let first_visible = (self.offset.max(0.0) / row_height).floor() as usize;
        let visible = (self.height / row_height).ceil() as usize + 1;
        let start = first_visible.saturating_sub(self.overscan).min(self.row_count);
        let end = (first_visible + visible + self.overscan).min(self.row_count);

        start..end
    }
}

/// How far a list can scroll: its content height less its viewport, or zero when
/// every row fits. The window helpers below clamp against it, and so must a
/// caller that writes its own offset before asking iced to scroll there.
pub fn max_offset(row_count: usize, row_height: f32, height: f32) -> f32 {
    (row_count as f32 * row_height.max(0.0) - height).max(0.0)
}

/// The offset that puts `index` at the top of the viewport, clamped to the
/// list's travel — the Slint component's `scroll-to-row(index, false)`, and the
/// number iced's `scroll_to` operation takes.
pub fn row_offset(index: usize, row_count: usize, row_height: f32, height: f32) -> f32 {
    (index as f32 * row_height.max(0.0)).min(max_offset(row_count, row_height, height))
}

/// The offset that puts `index` in the middle of the viewport, clamped — the
/// Slint component's `scroll-to-row(index, true)`.
pub fn centered_row_offset(index: usize, row_count: usize, row_height: f32, height: f32) -> f32 {
    (index as f32 * row_height.max(0.0) - (height - row_height) / 2.0)
        .clamp(0.0, max_offset(row_count, row_height, height))
}

impl<'a, Message> From<VirtualList<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: VirtualList<'a, Message>) -> Self {
        let range = control.range();
        let row_height = control.row_height.max(0.0);
        let selected = control.selected;
        let view = control.view;

        // The window: a spacer of the rows above, the rows the offset covers, and
        // a spacer of the rows below. The spacers hold the content height at
        // `row_count * row_height`, so the scrollbar and the scroll position stay
        // those of the whole list even though the great majority of it is never
        // built.
        let mut rows = column![].width(Length::Fill);

        if range.start > 0 {
            rows = rows.push(Space::new().height(Length::Fixed(range.start as f32 * row_height)));
        }

        for index in range.clone() {
            rows = rows.push(
                container(view(index, selected == Some(index)))
                    .width(Length::Fill)
                    .height(Length::Fixed(row_height)),
            );
        }

        if range.end < control.row_count {
            rows = rows.push(
                Space::new()
                    .height(Length::Fixed((control.row_count - range.end) as f32 * row_height)),
            );
        }

        let mut region = Scrollable::new(rows)
            .axis(Axis::Vertical)
            .height(Length::Fixed(control.height));

        if let Some(id) = control.id {
            region = region.id(id);
        }

        if let Some(on_scroll) = control.on_scroll {
            region = region.on_scroll(on_scroll);
        }

        region.into()
    }
}
