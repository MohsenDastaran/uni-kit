//! A labeled chip that can be dismissed. Mirrors GPUI Component's `Tag` and
//! the Slint version beside it: four variants, a status, a leading icon, sizes
//! from 16 to 32 pixels tall, and an optional close control.
//!
//! The height is the control height less eight pixels, so the chip sits inside
//! a control-sized row: 16, 20 and 32 pixels for the small, medium and large
//! steps. The fill follows the variant — the theme's muted surface by default,
//! the secondary surface, nothing with a border, or the status colour at the
//! theme's tint — and the ink is the status colour when the tag is a status
//! one and the foreground otherwise.
//!
//! iced has no icon set of its own and the application owns its assets, so the
//! leading icon is an element the caller passes. The close control is drawn as
//! a ghost button in the same shape as [`super::button`], with the Slint "x"
//! glyph as its label.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{button, container, row, text};
use iced::{Border, Color, Element, Length, Theme};

use super::theme::{self, ControlSize, Status, Tokens};

/// How the chip is filled. The names are the Slint `TagVariant`'s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// The muted surface.
    #[default]
    Default,
    /// The secondary surface.
    Secondary,
    /// No fill, with a one-pixel border.
    Outline,
    /// The status colour at the theme's tint, with the status colour for ink.
    Status,
}

/// A tag, built with the methods below.
pub struct Tag<'a, Message> {
    label: String,
    variant: Variant,
    status: Status,
    size: ControlSize,
    icon: Option<Element<'a, Message>>,
    on_close: Option<Message>,
}

impl<'a, Message> Tag<'a, Message> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::default(),
            status: Status::default(),
            size: ControlSize::default(),
            icon: None,
            on_close: None,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// The status a status tag reports. The other variants ignore it.
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self.variant = Variant::Status;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// A leading element at the size's icon box, two pixels smaller than the
    /// box the theme gives icons.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Draws the close control and sends this message when it is pressed.
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// The chip's height: the control height less the eight pixels a tag keeps
    /// for itself — 16, 20 or 32.
    fn height(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 12.0,
            ControlSize::Small => 16.0,
            ControlSize::Large => 32.0,
            ControlSize::Medium => 24.0,
        }
    }

    fn pad_x(&self) -> f32 {
        self.size.control_padding() - 2.0
    }

    fn ink(&self, tokens: Tokens) -> Color {
        match self.variant {
            Variant::Status => tokens.status_color(self.status),
            _ => tokens.foreground,
        }
    }

    fn fill(&self, tokens: Tokens) -> Option<Color> {
        match self.variant {
            Variant::Default => Some(tokens.muted),
            Variant::Secondary => Some(tokens.secondary),
            Variant::Outline => None,
            Variant::Status => {
                Some(theme::alpha(tokens.status_color(self.status), if tokens.dark { 0.18 } else { 0.12 }))
            }
        }
    }
}

/// The colours a tag draws its chip and its ink with.
pub struct Style {
    pub variant: Variant,
    pub status: Status,
}

impl Style {
    pub fn new(variant: Variant) -> Self {
        Self {
            variant,
            status: Status::default(),
        }
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn resolve(&self, tokens: Tokens) -> container::Style {
        let ink = match self.variant {
            Variant::Status => tokens.status_color(self.status),
            _ => tokens.foreground,
        };
        container::Style {
            background: match self.variant {
                Variant::Default => Some(tokens.muted.into()),
                Variant::Secondary => Some(tokens.secondary.into()),
                Variant::Outline => None,
                Variant::Status => Some(
                    theme::alpha(
                        tokens.status_color(self.status),
                        if tokens.dark { 0.18 } else { 0.12 },
                    )
                    .into(),
                ),
            },
            text_color: Some(ink),
            border: Border {
                color: tokens.border,
                width: if self.variant == Variant::Outline { 1.0 } else { 0.0 },
                radius: theme::RADIUS_MD.into(),
            },
            ..container::Style::default()
        }
    }
}

impl<'a, Message> From<Tag<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Tag<'a, Message>) -> Self {
        let variant = control.variant;
        let status = control.status;
        let size = control.size;
        let height = control.height();
        let pad_x = control.pad_x();
        let closable = control.on_close.is_some();
        let label = control.label.clone();

        let mut content = row![]
            .spacing(4.0)
            .align_y(iced::Alignment::Center)
            .height(Length::Fill);

        if let Some(icon) = control.icon {
            content = content.push(icon);
        }

        content = content.push(
            text(label)
                .size(size.control_font_size())
                .style(move |theme: &Theme| text::Style {
                    color: Some(match variant {
                        Variant::Status => Tokens::of(theme).status_color(status),
                        _ => Tokens::of(theme).foreground,
                    }),
                }),
        );

        if let Some(message) = control.on_close {
            let close = button(text("×").size(12.0))
                .padding(0.0)
                .width(Length::Fixed(14.0))
                .height(Length::Fixed(14.0))
                .style(move |theme: &Theme, status| {
                    let tokens = Tokens::of(theme);
                    // The close control is never a status-coloured surface, so
                    // its glyph is the muted foreground rather than the chip's
                    // ink.
                    let ink = tokens.muted_foreground;
                    button::Style {
                        background: match status {
                            button::Status::Hovered | button::Status::Pressed => {
                                Some(theme::alpha(tokens.foreground, 0.1).into())
                            }
                            _ => None,
                        },
                        text_color: ink,
                        border: Border {
                            radius: theme::RADIUS_SM.into(),
                            ..Border::default()
                        },
                        shadow: iced::Shadow::default(),
                        snap: false,
                    }
                })
                .on_press(message);

            content = content.push(close);
        }

        container(content)
            .height(Length::Fixed(height))
            .padding(iced::Padding {
                top: 0.0,
                right: if closable { 4.0 } else { pad_x },
                bottom: 0.0,
                left: pad_x,
            })
            .align_y(iced::Alignment::Center)
            .style(move |theme: &Theme| {
                Style::new(variant)
                    .status(status)
                    .resolve(Tokens::of(theme))
            })
            .into()
    }
}
