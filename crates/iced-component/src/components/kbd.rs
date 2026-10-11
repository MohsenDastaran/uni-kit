//! A keyboard-shortcut chip. Mirrors GPUI Component's `Kbd` and the Slint
//! version beside it: a monospace glyph in a muted chip with a one-pixel
//! border, at four heights — 16, 18, 20 and 24 pixels.
//!
//! The Slint version also ships `KbdSequence`, a row of chips with a 4px gap;
//! [`sequence`] is the same thing for a caller that has more than one key, and
//! [`Kbd::join`] builds it from a list of keys.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{container, row, text};
use iced::widget::container::Style as ContainerStyle;
use iced::{Border, Element, Length, Theme};

use super::theme::{self, ControlSize, Tokens};

/// A keyboard chip, built with the methods below. It carries no message, but it
/// is generic over one so a caller can place it in any view without annotating
/// the type.
pub struct Kbd<Message> {
    text: String,
    size: ControlSize,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Kbd<Message> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: ControlSize::default(),
            _message: std::marker::PhantomData,
        }
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The chip's height: 16 / 18 / 20 / 24 for xsmall through large.
    pub fn height(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 16.0,
            ControlSize::Small => 18.0,
            ControlSize::Large => 24.0,
            ControlSize::Medium => 20.0,
        }
    }

    /// A row of chips with a 4px gap — the Slint `KbdSequence`.
    pub fn join<I, S>(keys: I) -> Element<'static, Message>
    where
        Message: 'static,
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        sequence(keys)
    }
}

impl<Message> Default for Kbd<Message> {
    fn default() -> Self {
        Self::new("")
    }
}

/// The colours and heights a keyboard chip draws with.
pub struct Style {
    pub size: ControlSize,
}

impl Style {
    pub fn new(size: ControlSize) -> Self {
        Self { size }
    }

    /// The chip's height: 16 / 18 / 20 / 24 for xsmall through large.
    pub fn height(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 16.0,
            ControlSize::Small => 18.0,
            ControlSize::Large => 24.0,
            ControlSize::Medium => 20.0,
        }
    }

    pub fn resolve(&self, tokens: Tokens) -> ContainerStyle {
        ContainerStyle {
            background: Some(tokens.muted.into()),
            text_color: Some(tokens.foreground),
            border: Border {
                color: tokens.border,
                width: 1.0,
                radius: theme::RADIUS_SM.into(),
            },
            ..ContainerStyle::default()
        }
    }
}

impl<'a, Message> From<Kbd<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Kbd<Message>) -> Self {
        let height = control.height();
        let size = control.size;

        container(
            text(control.text)
                .font(theme::MONO)
                .size(size.control_font_size())
                .style(move |theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
        )
        .height(Length::Fixed(height))
        .padding([0.0, 6.0])
        .align_y(iced::Alignment::Center)
        .style(move |theme: &Theme| Style::new(size).resolve(Tokens::of(theme)))
        .into()
    }
}

/// A row of keyboard chips with a 4px gap between them.
pub fn sequence<'a, Message, I, S>(keys: I) -> Element<'a, Message>
where
    Message: 'a,
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut chips = row![].spacing(4.0).align_y(iced::Alignment::Center);
    for key in keys {
        chips = chips.push(Element::<Message>::from(Kbd::<Message>::new(key)));
    }
    chips.into()
}
