//! A single expandable panel. Mirrors GPUI Component's `Collapsible` and the
//! Slint version beside it: an optional titled trigger row with a chevron, a
//! content panel underneath, and the caller's `open` state deciding which is
//! drawn.
//!
//! The registry key is `collapsible`; the file and the module share the name.
//!
//! The trigger is 8px of padding all round with an 8px gap, a text-sm semibold
//! title in the foreground and a 14px chevron in the muted foreground. An empty
//! title draws no trigger at all, which is the Slint rule: the caller supplies
//! its own trigger above the panel and this is only the body. The panel insets
//! its children 8px on the left, right and bottom.
//!
//! **The trigger is an element**, so the caller wires the click that toggles:
//! `collapsible::trigger("Details", open, Message::Toggle)` is the built-in
//! ghost-button form, and any other element works in its place. Slint's
//! `Collapsible` reports `toggled(open)` from its own trigger; an iced control
//! carries its message instead, so the message is the caller's.
//!
//! The reveal is animated in Slint and the chevron rotates. iced has no
//! built-in animation driver, so the panel is shown or hidden outright and the
//! chevron is drawn in its end state; a caller that wants the motion
//! interpolates an `f32` progress in its own subscription and passes the
//! measured height.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{button, column, container, row, text};
use iced::widget::container::Style as ContainerStyle;
use iced::{Alignment, Background, Border, Color, Element, Length, Theme};

use super::theme::{self, Tokens};

/// A collapsible panel, built with the methods below.
pub struct Collapsible<'a, Message> {
    trigger: Option<Element<'a, Message>>,
    content: Vec<Element<'a, Message>>,
    open: bool,
    content_inset: f32,
}

impl<'a, Message> Collapsible<'a, Message> {
    /// A panel with no trigger of its own, which is the Slint component with an
    /// empty title: the caller draws the trigger and this is the body.
    pub fn new() -> Self {
        Self {
            trigger: None,
            content: Vec::new(),
            open: false,
            content_inset: 8.0,
        }
    }

    /// The trigger row, drawn above the panel. Use [`trigger`] for the built-in
    /// titled form, or build any element with its own message.
    pub fn trigger(mut self, trigger: impl Into<Element<'a, Message>>) -> Self {
        self.trigger = Some(trigger.into());
        self
    }

    /// Whether the panel is shown. The caller's state decides.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// A panel child. Call it more than once to stack the panel's content.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content.push(content.into());
        self
    }

    /// The panel's inset on the left, right and bottom, 8px by default.
    pub fn content_inset(mut self, inset: f32) -> Self {
        self.content_inset = inset;
        self
    }
}

impl<'a, Message> Default for Collapsible<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The built-in trigger: a text-sm semibold title with a 14px chevron, as a
/// ghost row that fills the width. `on_toggle` is the message the caller wants
/// when the row is clicked.
pub fn trigger<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    open: bool,
    on_toggle: Message,
) -> Element<'a, Message> {
    trigger_with_inset(title, open, 8.0, on_toggle)
}

/// [`trigger`] with a different inset, which is what a panel placed inside a
/// header of its own wants.
pub fn trigger_with_inset<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    open: bool,
    inset: f32,
    on_toggle: Message,
) -> Element<'a, Message> {
    let label = title.into();
    let content: Element<'a, Message> = row![
        text(label)
            .size(theme::FONT_SIZE_SM)
            .width(Length::Fill)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..iced::Font::DEFAULT
            })
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
        column![chevron(open, 14.0)].width(Length::Fixed(14.0)),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into();

    button(content)
        .width(Length::Fill)
        .padding(inset)
        .style(|theme: &Theme, status| {
            let tokens = Tokens::of(theme);
            let hovered = matches!(
                status,
                button::Status::Hovered | button::Status::Pressed
            );
            button::Style {
                background: Some(Background::Color(if hovered {
                    tokens.accent
                } else {
                    Color::TRANSPARENT
                })),
                text_color: tokens.foreground,
                border: Border::default(),
                ..button::Style::default()
            }
        })
        .on_press(on_toggle)
        .into()
}

/// The chevron's end state, drawn from the tokens because iced ships no icon
/// set: a short rule under the open state, a wider one under the closed state.
pub fn chevron<'a, Message: 'a>(open: bool, size: f32) -> Element<'a, Message> {
    let width = if open { size * 0.5 } else { size };
    container(iced::widget::Space::new())
        .width(Length::Fixed(width))
        .height(Length::Fixed(1.0))
        .style(|theme: &Theme| ContainerStyle {
            background: Some(Tokens::of(theme).muted_foreground.into()),
            ..ContainerStyle::default()
        })
        .into()
}

impl<'a, Message: 'a> From<Collapsible<'a, Message>> for Element<'a, Message> {
    fn from(control: Collapsible<'a, Message>) -> Self {
        let open = control.open;
        let inset = control.content_inset;

        let mut sections = column![].width(Length::Fill);

        if let Some(trigger) = control.trigger {
            sections = sections.push(trigger);
        }

        if open && !control.content.is_empty() {
            sections = sections.push(
                column(control.content)
                    .width(Length::Fill)
                    .padding(iced::Padding {
                        top: 0.0,
                        right: inset,
                        bottom: inset,
                        left: inset,
                    }),
            );
        }

        sections.into()
    }
}
