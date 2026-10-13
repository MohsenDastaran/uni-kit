//! A selectable list. Mirrors GPUI Component's `List` and the Slint version
//! beside it: rows of a title, an optional description and an optional leading
//! icon, stacked at a two-pixel gap, with the selected row filled by `accent`
//! and the hovered one tinted halfway to it.
//!
//! **The list is rows inside a [`Scrollable`](super::scrollable::Scrollable).**
//! The Slint list stacks its rows and lets the page scroll; the roadmap puts this
//! one on the finished scroll area, so a [`List::height`] bounds the list and the
//! rows move under their own bar. With no height the list sizes to its rows and
//! draws no bar, which is the Slint component's behaviour.
//!
//! The application owns the selection: the list is given the row that is selected
//! and reports a click through [`List::on_select`]. It never holds state of its
//! own, which is the same controlled shape as the table beside it.
//!
//! # What iced cannot draw, named rather than faked
//!
//! - **The icon is the caller's element.** Slint names a glyph with a string and
//!   resolves it against a shipped icon set; iced ships none, exactly as
//!   [`Icon`](super::icon::Icon) explains, so [`ListEntry::icon`] takes the
//!   caller's element.
//! - **There are no sections.** GPUI's `ListDelegate` renders section headers and
//!   footers and hides empty ones; the Slint `List` is one flat run of rows, and
//!   this port is the Slint one.
//! - **There is no search field, no loading row and no infinite scroll.** Those
//!   are `ListState` behaviour in GPUI. The demo's search section mocks the field
//!   above the list rather than pretending the list owns one.
//! - **There is no virtual scrolling.** Every row is a real element; the
//!   `Scrollable` underneath says the same thing.
//! - **There is no disabled row and no keyboard.** A row reports a click and a
//!   selected surface, and the caller decides what a row's state means.
//!
//! Everything is drawn from [`Tokens`], read inside the row's style closure while
//! it renders, so a theme switch repaints it. Nothing here writes a colour of its
//! own, and there is no `Styled` trait to layer one on: a caller that wants
//! another surface wraps the list in a container of its own.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one part gets
// a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{button, column, row, text};
use iced::{Alignment, Border, Color, Element, Font, Length, Shadow, Theme};

use super::scrollable::{Axis, Scrollable};
use super::theme::{self, Tokens};

/// The gap between two rows, `spacing: 2px` in the Slint component.
const ROW_GAP: f32 = 2.0;
/// A row's own inset, `padding: 8px` in the Slint component.
const ROW_PADDING: f32 = 8.0;
/// Between a row's icon and its text, `spacing: 10px`.
const ROW_SPACING: f32 = 10.0;
/// Between a row's title and its description.
const LINE_GAP: f32 = 2.0;
/// The hover tint, `accent.with-alpha(0.5)` in the Slint component.
const HOVER_TINT: f32 = 0.5;

/// One row: a title, an optional description and an optional leading icon.
///
/// The title is required — it is what the row is for. Slint's `ListEntry` names
/// the icon with a string; here it is the caller's [`Element`], because the icon
/// set belongs to the application.
pub struct ListEntry<'a, Message> {
    title: String,
    description: String,
    icon: Option<Element<'a, Message>>,
}

impl<'a, Message: 'a> ListEntry<'a, Message> {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: String::new(),
            icon: None,
        }
    }

    /// A second, muted line under the title. An empty description draws no line,
    /// which is the Slint `if item.description != ""`.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// A leading element, drawn before the text.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// The row's element. `selected` is the list's own choice and `message` is
    /// what a click reports — `None` when the caller registered no handler, which
    /// is the same "cannot be pressed" a button without a message has.
    fn into_element(self, selected: bool, message: Option<Message>) -> Element<'a, Message>
    where
        Message: Clone,
    {
        let mut line = row![]
            .spacing(ROW_SPACING)
            .align_y(Alignment::Center)
            .width(Length::Fill);

        if let Some(icon) = self.icon {
            line = line.push(icon);
        }

        let mut lines = column![].spacing(LINE_GAP).width(Length::Fill);
        lines = lines.push(
            text(self.title)
                .size(theme::FONT_SIZE_SM)
                .font(Font {
                    weight: iced::font::Weight::Medium,
                    ..Font::DEFAULT
                })
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
        );
        if !self.description.is_empty() {
            lines = lines.push(
                text(self.description)
                    .size(theme::FONT_SIZE_XS)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
            );
        }
        line = line.push(lines);

        let mut control = button(line)
            .width(Length::Fill)
            .padding(ROW_PADDING)
            .style(move |theme: &Theme, status| row_style(theme, status, selected));
        if let Some(message) = message {
            control = control.on_press(message);
        }
        control.into()
    }
}

/// The row's surface: `accent` when the list selected it, half of that while
/// the pointer is on it, and nothing otherwise — the Slint ternary, in order.
fn row_style(theme: &Theme, status: button::Status, selected: bool) -> button::Style {
    // Resolved here, inside the closure, and never at build time: a surface read
    // from the light tokens before the row is drawn would paint a white row under
    // the dark theme.
    let tokens = Tokens::of(theme);
    let background = if selected {
        tokens.accent
    } else if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        theme::alpha(tokens.accent, HOVER_TINT)
    } else {
        Color::TRANSPARENT
    };

    button::Style {
        background: (background.a > 0.0).then_some(background.into()),
        text_color: tokens.foreground,
        border: Border {
            radius: theme::RADIUS_MD.into(),
            ..Border::default()
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

/// A selectable list, built with the methods below.
pub struct List<'a, Message> {
    entries: Vec<ListEntry<'a, Message>>,
    selected: Option<usize>,
    height: Length,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message> List<'a, Message> {
    pub fn new(entries: impl IntoIterator<Item = ListEntry<'a, Message>>) -> Self {
        Self {
            entries: entries.into_iter().collect(),
            selected: None,
            height: Length::Shrink,
            on_select: None,
        }
    }

    /// Appends one row.
    pub fn entry(mut self, entry: ListEntry<'a, Message>) -> Self {
        self.entries.push(entry);
        self
    }

    /// Which row is selected. `None`, the default, is the Slint `selected: -1`.
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// Bounds the list, so it scrolls its rows and draws a bar. Without it the
    /// list sizes to its rows, which is the Slint component.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// A row was clicked. The list reports the row's index and leaves the
    /// selection to the caller, so the caller's state is the one shown.
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }
}

impl<'a, Message> Default for List<'a, Message> {
    fn default() -> Self {
        Self::new(std::iter::empty())
    }
}

impl<'a, Message> From<List<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: List<'a, Message>) -> Self {
        let selected = control.selected;
        let on_select = control.on_select;

        let mut rows = column![].spacing(ROW_GAP).width(Length::Fill);
        for (index, entry) in control.entries.into_iter().enumerate() {
            let message = on_select.as_ref().map(|on_select| on_select(index));
            rows = rows.push(entry.into_element(selected == Some(index), message));
        }

        Scrollable::new(rows)
            .axis(Axis::Vertical)
            .height(control.height)
            .into()
    }
}
