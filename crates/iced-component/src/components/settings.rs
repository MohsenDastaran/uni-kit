//! A settings page: a stack of labelled rows, each carrying a control the
//! caller supplies. Mirrors GPUI Component's `Settings` → `SettingGroup` →
//! `SettingItem` and the Slint `Settings` and `SettingRow` beside it.
//!
//! This file draws no control of its own, the same division [`super::form`]
//! makes: a row is a title, an optional description, and **any** [`Element`] the
//! caller builds from the landed controls. A switch for a setting that takes
//! effect at once, a checkbox for an independent choice, a radio group for a
//! small visible set, a select for a longer one, an input or a number input for
//! a value — the row names none of them and installs none of them.
//!
//! # The row is its own two columns, not a `form::Field`
//!
//! A [`super::form::Field`] puts its description under the *control* and its
//! label in a fixed-width column. A settings row is the other way round: the
//! title and its description lead as one block and the control sits at the
//! trailing edge at its own width, which is Slint's `Settings` row and GPUI's
//! `SettingItem` in horizontal layout. The Design Guides say not to force long
//! labels into a narrow fixed column, so the row keeps the title flexible rather
//! than reusing the field's geometry. [`Label`] draws the title and the tokens
//! colour the description.
//!
//! # Controlled, like the rest of the crate
//!
//! A row holds no value. Its control reports the value it wants through the
//! caller's own message and the application stores it — the same controlled
//! shape [`super::switch`], [`super::list`] and [`super::sidebar`] use.
//!
//! # The surface belongs to the caller
//!
//! Slint's example wraps `Settings` in an outlined `GroupBox` for the basic
//! section and in bare rectangles for the variant one; GPUI's `Settings` owns a
//! `GroupBoxVariant`. Here the surface is the caller's, so the same rows can sit
//! on the page, on a filled group or on an outlined one without this file
//! choosing. `demos/settings.rs` shows both variants.
//!
//! # What iced cannot draw, named rather than faked
//!
//! - **Disabled is not a row state.** GPUI dims the whole row and disables the
//!   built-in field; iced 0.14 has no subtree opacity and the row does not reach
//!   into its child. A caller that wants a disabled row builds the control
//!   without a message, which is how iced spells "cannot be pressed", and the
//!   control's own disabled ink is what dims it. There is no `disabled` builder
//!   rather than one that only tints the text.
//! - **No search.** GPUI's `Settings` filters pages and items by title,
//!   description and keywords; this port is the rows the example shows, so it has
//!   no query to match and no keywords to carry.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Padding, Theme};

use super::label::Label;
use super::theme::{self, ControlSize, Tokens};

/// A settings page: a vertical run of [`SettingRow`]s, built with the methods
/// below.
pub struct Settings<'a, Message> {
    rows: Vec<SettingRow<'a, Message>>,
    gap: f32,
}

impl<'a, Message> Settings<'a, Message> {
    /// An empty page. Rows stack with no gap by default: a row carries its own
    /// 12px above and below, which is the spacing Slint's `Settings` sets.
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            gap: 0.0,
        }
    }

    /// The gap between rows, 0 by default.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Add a row.
    pub fn row(mut self, row: SettingRow<'a, Message>) -> Self {
        self.rows.push(row);
        self
    }

    /// Add rows in order.
    pub fn rows(mut self, rows: impl IntoIterator<Item = SettingRow<'a, Message>>) -> Self {
        self.rows.extend(rows);
        self
    }

    /// The rows, and nothing else: the caller places the page and its surface.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        column(self.rows.into_iter().map(SettingRow::into_element))
            .spacing(self.gap)
            .width(Length::Fill)
            .into()
    }
}

impl<'a, Message> Default for Settings<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Settings<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(settings: Settings<'a, Message>) -> Self {
        settings.into_element()
    }
}

/// One labelled row of a [`Settings`] page, built with the methods below.
///
/// It places the title, the description and the caller's control; it paints no
/// control and owns no value.
pub struct SettingRow<'a, Message> {
    title: String,
    description: String,
    field: Option<Element<'a, Message>>,
}

impl<'a, Message> SettingRow<'a, Message> {
    /// An empty row titled `title`.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: String::new(),
            field: None,
        }
    }

    /// The row's title, in the foreground at 14px medium — Slint's
    /// `font-size-sm` at weight 500. Calling this again replaces the one the
    /// constructor took.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// The supporting line under the title, in the muted colour at 12px — Slint's
    /// `font-size-xs`, which wraps. An empty description, the default, draws no
    /// line.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// The control at the trailing edge: a switch, a checkbox, a radio group, a
    /// select, an input, a number input, or a stack of them. Its width is its
    /// own; a control that asks for `Length::Fill` competes with the title for
    /// the row, so a settings row is where a control states the width it wants.
    pub fn field(mut self, field: impl Into<Element<'a, Message>>) -> Self {
        self.field = Some(field.into());
        self
    }

    /// Draw the title and its description leading, the control trailing, with
    /// Slint's 12px above and below.
    fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let mut lines = column![Label::new(self.title.clone()).size(ControlSize::Small)]
            .spacing(theme::SPACING_XS)
            .width(Length::Fill);

        if !self.description.is_empty() {
            lines = lines.push(
                text(self.description)
                    .size(theme::FONT_SIZE_XS)
                    .width(Length::Fill)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
            );
        }

        // The title column fills, so the control that follows it lands against
        // the trailing edge at its own width — the row shape, not a
        // `justify_between` that would hide what is in the middle.
        let mut line = row![lines]
            .spacing(theme::SPACING_LG)
            .align_y(Alignment::Center)
            .width(Length::Fill);

        if let Some(field) = self.field {
            line = line.push(field);
        }

        container(line)
            .width(Length::Fill)
            .padding(Padding {
                top: theme::SPACING_MD,
                right: 0.0,
                bottom: theme::SPACING_MD,
                left: 0.0,
            })
            .into()
    }
}
