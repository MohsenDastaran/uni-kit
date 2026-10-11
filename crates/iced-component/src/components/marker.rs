//! Highlighted text. Mirrors GPUI Component's `Marker` and the Slint version
//! beside it: text at text-sm, and a highlight that draws a warning-tinted
//! band behind it with a two-pixel inset on each side.
//!
//! The Slint version exports the two as `Marker`, inheriting `Text`, and
//! `MarkerHighlight`, a rectangle around a text. Here they are the two variants
//! of one component, because both are the same three properties.
//!
//! iced cannot draw behind part of a text run, so `Highlight` wraps the text in
//! a container instead — the same shape Slint draws, and the reason a caller
//! who wants only part of a line highlighted passes that part as its own
//! `Marker::highlight`.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{container, text};
use iced::widget::container::Style as ContainerStyle;
use iced::{Border, Element, Theme};

use super::theme::{self, Tokens};

/// Which of the two markers to draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// Plain text at text-sm in the foreground colour.
    #[default]
    Text,
    /// The same text on a warning band, inset by two pixels.
    Highlight,
}

/// A marker, built with the methods below. It carries no message, but it is
/// generic over one so a caller can place it in any view without annotating the
/// type.
pub struct Marker<Message> {
    text: String,
    variant: Variant,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> Marker<Message> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::default(),
            _message: std::marker::PhantomData,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// The shorthand for [`Variant::Highlight`].
    pub fn highlight(self) -> Self {
        self.variant(Variant::Highlight)
    }

    /// The shorthand for [`Variant::Text`].
    pub fn plain(self) -> Self {
        self.variant(Variant::Text)
    }
}

impl<Message> Default for Marker<Message> {
    fn default() -> Self {
        Self::new("")
    }
}

/// The colours a marker draws its band and its text with.
pub struct Style {
    pub variant: Variant,
}

impl Style {
    pub fn new(variant: Variant) -> Self {
        Self { variant }
    }

    pub fn resolve(&self, tokens: Tokens) -> ContainerStyle {
        match self.variant {
            Variant::Text => ContainerStyle::default(),
            Variant::Highlight => ContainerStyle {
                background: Some(theme::alpha(tokens.warning, 0.35).into()),
                text_color: Some(tokens.foreground),
                border: Border {
                    radius: theme::RADIUS_SM.into(),
                    ..Border::default()
                },
                ..ContainerStyle::default()
            },
        }
    }
}

impl<'a, Message> From<Marker<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Marker<Message>) -> Self {
        let variant = control.variant;

        let label = text(control.text)
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            });

        match variant {
            Variant::Text => label.into(),
            Variant::Highlight => container(label)
                .padding([0.0, 2.0])
                .style(move |theme: &Theme| Style::new(variant).resolve(Tokens::of(theme)))
                .into(),
        }
    }
}
