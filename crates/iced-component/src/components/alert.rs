//! A callout that draws attention to a message. Mirrors GPUI Component's
//! `Alert` and the Slint version beside it: status variants, an optional title,
//! sizes, an optional dismiss control and a banner shape that squares the
//! corners and tightens the padding.
//!
//! The message body is drawn from the callout's message with an optional
//! leading icon, which is the slot the Slint component gives to `@children`.
//!
//! The status the alert reports decides the fill, the edge and the ink: the
//! fill is the status colour at 4% over the page, the edge the same colour at
//! 30%, and the title, message and icon all take the full status colour. The
//! default status is the page's own surface with a plain border.
//!
//! Slint hides the alert by setting `open` to false and reports the dismissal
//! to the caller. An iced control carries its message, so `on_close` is the
//! message the caller wants when the alert is dismissed; the caller owns the
//! `open` flag and stops constructing the alert when it is false.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length, Theme};

use super::theme::{self, ControlSize, Tokens};

/// The status an alert reports. The names and meanings are the Slint theme's,
/// except that `Info` is the Slint default here too.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
    Default,
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl Status {
    fn tone(self) -> Option<theme::Status> {
        match self {
            Self::Default => None,
            Self::Info => Some(theme::Status::Info),
            Self::Success => Some(theme::Status::Success),
            Self::Warning => Some(theme::Status::Warning),
            Self::Danger => Some(theme::Status::Danger),
        }
    }

    fn color(self, tokens: Tokens) -> Color {
        match self.tone() {
            Some(status) => tokens.status_color(status),
            None => tokens.foreground,
        }
    }
}

/// An alert, built with the methods below.
pub struct Alert<'a, Message> {
    title: String,
    message: String,
    status: Status,
    size: ControlSize,
    icon: Option<Element<'a, Message>>,
    banner: bool,
    on_close: Option<Message>,
}

impl<'a, Message> Alert<'a, Message> {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            title: String::new(),
            message: message.into(),
            status: Status::default(),
            size: ControlSize::default(),
            icon: None,
            banner: false,
            on_close: None,
        }
    }

    /// A title above the message, drawn semibold in the status colour. The
    /// Slint banner shape drops it.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// A leading element in place of the status glyph. The icon set belongs to
    /// the application, so Slint's `circle-check` and friends are the caller's.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// The full-width bar shape: square corners, 12×8 padding, a 12px gap and
    /// the icon centred against the message.
    pub fn banner(mut self, banner: bool) -> Self {
        self.banner = banner;
        self
    }

    /// Draws the dismiss control and sends this message when it is pressed.
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// The insets GPUI's sizes use: the message stays at text-sm and only the
    /// padding, the gap and the corner change.
    fn metrics(&self) -> (f32, f32, f32, f32) {
        if self.banner {
            return (12.0, 12.0, 8.0, 12.0);
        }
        let pad_x = match self.size {
            ControlSize::Large => 20.0,
            ControlSize::Medium => 16.0,
            _ => 12.0,
        };
        let pad_y = match self.size {
            ControlSize::XSmall => 6.0,
            ControlSize::Small => 8.0,
            ControlSize::Large => 14.0,
            ControlSize::Medium => 10.0,
        };
        let gap = match self.size {
            ControlSize::XSmall | ControlSize::Small => 6.0,
            _ => 12.0,
        };
        let corner = match self.size {
            ControlSize::Large => theme::RADIUS_LG,
            _ => theme::RADIUS_MD,
        };
        (pad_x, pad_y, gap, corner)
    }
}

/// The colours the alert draws with, resolved from the tokens in force.
pub struct Style {
    pub status: Status,
    pub banner: bool,
    pub size: ControlSize,
}

impl Style {
    pub fn new(status: Status) -> Self {
        Self {
            status,
            banner: false,
            size: ControlSize::default(),
        }
    }

    pub fn banner(mut self, banner: bool) -> Self {
        self.banner = banner;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn resolve(&self, tokens: Tokens) -> container::Style {
        let tone = self.status.tone();
        let ink = self.status.color(tokens);
        let fill = match tone {
            Some(_) => theme::mix(ink, tokens.background, 0.04),
            None => tokens.background,
        };
        let edge = match tone {
            Some(_) => theme::alpha(ink, 0.3),
            None => tokens.border,
        };
        container::Style {
            background: Some(fill.into()),
            text_color: Some(ink),
            border: iced::Border {
                color: edge,
                width: 1.0,
                radius: if self.banner {
                    0.0.into()
                } else {
                    match self.size {
                        ControlSize::Large => theme::RADIUS_LG,
                        _ => theme::RADIUS_MD,
                    }
                    .into()
                },
            },
            ..container::Style::default()
        }
    }
}

impl<'a, Message> From<Alert<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Alert<'a, Message>) -> Self {
        let (pad_x, pad_y, gap, _) = control.metrics();
        let status = control.status;
        let style = Style::new(status)
            .banner(control.banner)
            .size(control.size);
        let ink = status.tone();

        let title = control.title.clone();
        let message = control.message.clone();
        let banner = control.banner;
        let on_close = control.on_close.clone();

        let mut body = column![].spacing(4.0).width(Length::Fill);

        if !title.is_empty() && !banner {
            body = body.push(
                text(title)
                    .size(theme::FONT_SIZE_SM)
                    .font(iced::Font {
                        weight: iced::font::Weight::Semibold,
                        ..iced::Font::DEFAULT
                    })
                    .style(move |theme: &Theme| text::Style {
                        color: Some(status.color(Tokens::of(theme))),
                    }),
            );
        }

        if !message.is_empty() {
            body = body.push(
                text(message)
                    .size(theme::FONT_SIZE_SM)
                    .style(move |theme: &Theme| text::Style {
                        color: Some(status.color(Tokens::of(theme))),
                    }),
            );
        }

        let mut content = row![].spacing(gap).align_y(if banner {
            Alignment::Center
        } else {
            Alignment::Start
        });

        if let Some(icon) = control.icon {
            content = content.push(
                container(icon)
                    .align_y(Alignment::Start)
                    .padding(iced::Padding {
                        top: if banner { 0.0 } else { 2.0 },
                        right: 0.0,
                        bottom: 0.0,
                        left: 0.0,
                    }),
            );
        }

        content = content.push(body);

        if let Some(message) = on_close {
            let close = button(text("×").size(14.0))
                .padding(0.0)
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0))
                .style(move |theme: &Theme, status| {
                    let tokens = Tokens::of(theme);
                    let ink = status_color(tokens, ink);
                    let background = match status {
                        button::Status::Hovered => Some(theme::alpha(ink, 0.1)),
                        button::Status::Pressed => Some(theme::alpha(ink, 0.16)),
                        _ => None,
                    };
                    button::Style {
                        background: background.map(Into::into),
                        text_color: ink,
                        border: iced::Border {
                            radius: theme::RADIUS_SM.into(),
                            ..iced::Border::default()
                        },
                        shadow: iced::Shadow::default(),
                        snap: false,
                    }
                })
                .on_press(message);

            content = content.push(container(close).align_y(if banner {
                Alignment::Center
            } else {
                Alignment::Start
            }));
        }

        container(content)
            .width(Length::Fill)
            .padding([pad_y, pad_x])
            .style(move |theme: &Theme| style.resolve(Tokens::of(theme)))
            .into()
    }
}

fn status_color(tokens: Tokens, status: Option<theme::Status>) -> Color {
    match status {
        Some(status) => tokens.status_color(status),
        None => tokens.foreground,
    }
}
