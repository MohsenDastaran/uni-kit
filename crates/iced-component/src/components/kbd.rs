//! A keyboard-shortcut chip. Mirrors GPUI Component's `Kbd` and the Slint
//! version beside it: a monospace glyph in a muted cap with a one-pixel edge,
//! at four heights — 16, 18, 20 and 24 pixels.
//!
//! The cap is drawn as a physical key rather than a flat chip, without copying a
//! key's bevel: the face is one step smaller than the row it sits in — a mono
//! face at 10, 11, 12 and 13 pixels — on the muted surface, and it leaves a
//! single pixel of the cap under itself for the lip. That pixel is the same edge
//! colour the face is outlined in, so the bottom edge carries two pixels to the
//! other three's one and the cap reads as pressed out of the page. The edge is a
//! proportion of the foreground, not the fixed `border` token: `muted` and
//! `border` are the same value in the dark table, so a hairline taken from the
//! token would vanish there.
//!
//! The Slint version also ships `KbdSequence`, a row of chips with a 4px gap;
//! [`sequence`] is the same thing for a caller that has more than one key, and
//! [`Kbd::join`] builds it from a list of keys.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::container::Style as ContainerStyle;
use iced::widget::{container, row, text};
use iced::{Border, Color, Element, Length, Theme};

use super::theme::{self, ControlSize, Tokens};

/// The pixel the cap keeps under its face for the bottom lip. The face loses it,
/// so a `medium` cap is still twenty pixels tall; the lip is what makes the
/// bottom edge heavier than the sides.
const LIP: f32 = 1.0;

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

    /// The cap's height: 16 / 18 / 20 / 24 for xsmall through large.
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

    /// The cap's height: 16 / 18 / 20 / 24 for xsmall through large.
    pub fn height(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 16.0,
            ControlSize::Small => 18.0,
            ControlSize::Large => 24.0,
            ControlSize::Medium => 20.0,
        }
    }

    /// The face of the cap: the muted surface with the hairline edge.
    pub fn resolve(&self, tokens: Tokens) -> ContainerStyle {
        ContainerStyle {
            background: Some(tokens.muted.into()),
            text_color: Some(tokens.foreground),
            border: Border {
                color: edge(tokens),
                width: 1.0,
                radius: theme::RADIUS_SM.into(),
            },
            ..ContainerStyle::default()
        }
    }

    /// The lip under the face, which the outer container fills with.
    pub fn lip(&self, tokens: Tokens) -> ContainerStyle {
        ContainerStyle {
            background: Some(edge(tokens).into()),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: theme::RADIUS_SM.into(),
            },
            ..ContainerStyle::default()
        }
    }
}

/// The cap's edge, hairline and lip alike.
///
/// A proportion of the foreground is an edge in both appearances, which the
/// fixed `border` token is not: it holds the same value as `muted` in the dark
/// table, so a hairline taken from it would be an edge on white and nothing on
/// near-black — the same reading the button's default variant already records.
fn edge(tokens: Tokens) -> Color {
    theme::alpha(tokens.foreground, 0.22)
}

/// The label's size: one step under the control's, because a keycap's face is
/// smaller than the row it sits in. 10, 11, 12 and 13 pixels.
fn key_font_size(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 10.0,
        ControlSize::Small => 11.0,
        ControlSize::Medium => 12.0,
        ControlSize::Large => 13.0,
    }
}

/// The padding inside a cap, scaled with its height rather than fixed at the
/// Slint example's six pixels: a sixteen-pixel cap cannot carry the same inset
/// as a twenty-four-pixel one without the mark crowding its edge.
fn key_padding_x(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 4.0,
        ControlSize::Small => 5.0,
        ControlSize::Medium => 6.0,
        ControlSize::Large => 8.0,
    }
}

impl<'a, Message> From<Kbd<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Kbd<Message>) -> Self {
        let height = control.height();
        let size = control.size;

        // The face leaves `LIP` pixels of the cap to the outer container, whose
        // own fill shows through as the bottom edge. Both carry the hairline
        // edge, so the bottom reads as two pixels against the sides' one — the
        // weight a key's throw has — without a shadow, which would blur and fall
        // outside the cap instead of being part of it.
        let face = container(
            text(control.text)
                .font(theme::MONO)
                .size(key_font_size(size))
                .style(move |theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
        )
        .height(Length::Fixed(height - LIP))
        .padding([0.0, key_padding_x(size)])
        .align_y(iced::Alignment::Center)
        .style(move |theme: &Theme| Style::new(size).resolve(Tokens::of(theme)));

        container(face)
            .height(Length::Fixed(height))
            .padding(iced::Padding {
                top: 0.0,
                right: 0.0,
                bottom: LIP,
                left: 0.0,
            })
            .style(move |theme: &Theme| Style::new(size).lip(Tokens::of(theme)))
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
