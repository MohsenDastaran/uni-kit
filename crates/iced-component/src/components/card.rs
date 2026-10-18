//! A content container. Mirrors GPUI Component's `Card` and the Slint version
//! beside it: an optional badge above a title and a description, a bordered
//! surface on the page's own background, and a body the caller fills.
//!
//! The inset is the padding and the gap between the sections at once — 16px at
//! the medium size, which the small size tightens to 12px. The title is drawn
//! semibold at text-md (text-sm at the small size) and the description at
//! text-sm in the muted foreground.
//!
//! The body is an element the caller builds. The card's own parts read their
//! colours from the tokens when iced renders them, which is what an iced
//! `style` closure is for; a caller's body does the same with a closure of its
//! own over `Tokens::of(theme)`.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{column, container, text};
use iced::widget::container::Style as ContainerStyle;
use iced::{Border, Element, Length, Theme};

use super::theme::{self, ControlSize, Tokens};

/// A card, built with the methods below.
pub struct Card<'a, Message> {
    title: String,
    description: String,
    badge: String,
    size: ControlSize,
    body: Option<Element<'a, Message>>,
}

impl<'a, Message> Card<'a, Message> {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            badge: String::new(),
            size: ControlSize::default(),
            body: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// A small chip above the title, the label the Slint card draws with its
    /// own `Tag`.
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = badge.into();
        self
    }

    /// Small tightens the inset from 16px to 12px and the title from text-md to
    /// text-sm; the other sizes are the card's.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The card's body, drawn below the header inside the same inset.
    pub fn body(mut self, body: impl Into<Element<'a, Message>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Padding, and the gap between the header, the body and whatever the
    /// caller stacks beside them.
    pub fn inset(&self) -> f32 {
        match self.size {
            ControlSize::Small => 12.0,
            _ => 16.0,
        }
    }
}

impl<'a, Message> Default for Card<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The colours a card draws its surface with.
pub struct Style;

impl Style {
    pub fn resolve(&self, tokens: Tokens) -> ContainerStyle {
        ContainerStyle {
            background: Some(tokens.background.into()),
            text_color: Some(tokens.foreground),
            border: Border {
                color: tokens.border,
                width: 1.0,
                radius: theme::RADIUS_LG.into(),
            },
            ..ContainerStyle::default()
        }
    }
}

impl<'a, Message> From<Card<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Card<'a, Message>) -> Self {
        let inset = control.inset();
        let title_size = match control.size {
            ControlSize::Small => theme::FONT_SIZE_SM,
            _ => theme::FONT_SIZE_MD,
        };
        let badge = control.badge.clone();
        let title = control.title.clone();
        let description = control.description.clone();

        let mut sections = column![].spacing(inset).padding(inset);

        if !badge.is_empty() || !title.is_empty() || !description.is_empty() {
            let mut header = column![].spacing(4.0);

            if !badge.is_empty() {
                header = header.push(
                    container(
                        text(badge)
                            .size(theme::FONT_SIZE_SM)
                            .style(|theme: &Theme| text::Style {
                                color: Some(Tokens::of(theme).foreground),
                            }),
                    )
                    .padding([2.0, 6.0])
                    .style(|theme: &Theme| ContainerStyle {
                        background: Some(Tokens::of(theme).muted.into()),
                        border: Border {
                            radius: theme::RADIUS_MD.into(),
                            ..Border::default()
                        },
                        ..ContainerStyle::default()
                    }),
                );
            }

            if !title.is_empty() {
                header = header.push(
                    text(title)
                        .size(title_size)
                        .font(iced::Font {
                            weight: iced::font::Weight::Semibold,
                            ..iced::Font::DEFAULT
                        })
                        .style(|theme: &Theme| text::Style {
                            color: Some(Tokens::of(theme).foreground),
                        }),
                );
            }

            if !description.is_empty() {
                header = header.push(
                    text(description)
                        .size(theme::FONT_SIZE_SM)
                        .style(|theme: &Theme| text::Style {
                            color: Some(Tokens::of(theme).muted_foreground),
                        }),
                );
            }

            sections = sections.push(header);
        }

        if let Some(body) = control.body {
            sections = sections.push(body);
        }

        container(sections)
            .width(Length::Fill)
            .style(|theme: &Theme| Style.resolve(Tokens::of(theme)))
            .into()
    }
}

/// A card's footer: end-aligned content with an 8px gap, the `CardFooter` the
/// Slint version ships beside the card.
pub fn footer<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(content.into())
        .width(Length::Fill)
        .align_x(iced::Alignment::End)
        .into()
}
