//! A form label. Mirrors GPUI Component's `Label` and the Slint version beside
//! it: the text at the size's control font size and medium weight, an optional
//! required mark in the danger colour, and an optional slot after it for a
//! hint or a control.
//!
//! The label is the text and the mark; anything else a caller wants beside it
//! is [`Label::suffix`], which is the `@children` slot in the Slint version.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{row, text};
use iced::{Element, Theme};

use super::theme::{ControlSize, Tokens};

/// A form label, built with the methods below.
pub struct Label<'a, Message> {
    text: String,
    required: bool,
    size: ControlSize,
    suffix: Option<Element<'a, Message>>,
}

impl<'a, Message> Label<'a, Message> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            required: false,
            size: ControlSize::default(),
            suffix: None,
        }
    }

    /// Draws the asterisk in the danger colour after the label. The Slint
    /// version puts the mark between the label and its children, so a suffix
    /// still follows it.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Content after the label: a hint, a unit, or the control itself.
    pub fn suffix(mut self, suffix: impl Into<Element<'a, Message>>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }
}

impl<'a, Message> Default for Label<'a, Message> {
    fn default() -> Self {
        Self::new("")
    }
}

/// The colours and sizes a label draws with.
pub struct Style {
    pub size: ControlSize,
}

impl Style {
    pub fn new(size: ControlSize) -> Self {
        Self { size }
    }

    /// The text size the label's step uses.
    pub fn font_size(&self) -> f32 {
        self.size.control_font_size()
    }

    pub fn resolve(&self, tokens: Tokens) -> text::Style {
        text::Style {
            color: Some(tokens.foreground),
        }
    }
}

impl<'a, Message> From<Label<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Label<'a, Message>) -> Self {
        let size = Style::new(control.size).font_size();
        let required = control.required;

        let mut content = row![].spacing(4.0).align_y(iced::Alignment::Center);

        content = content.push(
            text(control.text)
                .size(size)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..iced::Font::DEFAULT
                })
                .style(|theme: &Theme| Style::new(ControlSize::Medium).resolve(Tokens::of(theme))),
        );

        if required {
            content = content.push(
                text("*")
                    .size(size)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).danger),
                    }),
            );
        }

        if let Some(suffix) = control.suffix {
            content = content.push(suffix);
        }

        content.into()
    }
}
