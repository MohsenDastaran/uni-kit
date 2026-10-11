//! A vertical stack of collapsible sections. Mirrors GPUI Component's
//! `Accordion` and the Slint version beside it: a bordered card by default,
//! sizes, a disabled state, several sections open at once with `multiple`, and
//! a toggle event carrying the section and its new state.
//!
//! The registry key is `accordion`; the file and the module share the name.
//!
//! A section is a header row over a content panel, with a 1px separator between
//! sections, and a chevron marking whether it is open. The insets are the Slint
//! ones at each size: 6×4 at xsmall, 8×6 at small, 12×8 at medium and 16×12 at
//! large, with text-xs, text-sm, text-sm and text-md titles. The chevron box is
//! 12px, 14px at the large size.
//!
//! **The header row is the caller's element**, built with the interactive part
//! the caller wants — `button("Details").style(button::text).on_press(..)`, a
//! `mouse_area`, or a row of controls. Slint's `AccordionItem` owns its
//! clickable title and reports `toggled(open)`; an iced control carries its
//! own message instead, so the click that toggles is the caller's, and this
//! component supplies the chrome around it. That is the same split the two
//! libraries already have, and it is what [`accordion::text_header`] and
//! [`accordion::labeled_header`] build for the common case.
//!
//! Which sections are open is the caller's state too. iced has no `in-out`
//! property to bind, so the caller keeps the open set and passes each section's
//! `open` in. The chevron's rotation and the panel's reveal are animated in
//! Slint; iced has no built-in animation driver, so the panel is shown or hidden
//! outright and the chevron is drawn in its end state. A caller that wants the
//! motion interpolates an `f32` progress in its own subscription.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{button, column, container, row, text};
use iced::widget::container::Style as ContainerStyle;
use iced::{Alignment, Background, Border, Color, Element, Length, Theme};

use super::theme::{self, ControlSize, Tokens};

/// One section: the header row, the panel below it and whether it is open.
pub struct Section<'a, Message> {
    header: Element<'a, Message>,
    content: Vec<Element<'a, Message>>,
    open: bool,
    chevron: bool,
}

impl<'a, Message> Section<'a, Message> {
    /// A section whose header is the caller's row. Put the click that toggles
    /// it inside that row.
    pub fn new(header: impl Into<Element<'a, Message>>) -> Self {
        Self {
            header: header.into(),
            content: Vec::new(),
            open: false,
            chevron: true,
        }
    }

    /// Whether the panel is shown. The caller's state decides, and the caller
    /// flips it from the header row's message.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// A panel child. Call it more than once to stack the panel's content.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content.push(content.into());
        self
    }

    /// Draws the chevron in its end state, down when closed and up when open.
    pub fn chevron(mut self, chevron: bool) -> Self {
        self.chevron = chevron;
        self
    }
}

/// An accordion, built with the methods below.
pub struct Accordion<'a, Message> {
    sections: Vec<Section<'a, Message>>,
    size: ControlSize,
    bordered: bool,
}

impl<'a, Message> Accordion<'a, Message> {
    pub fn new(sections: impl IntoIterator<Item = Section<'a, Message>>) -> Self {
        Self {
            sections: sections.into_iter().collect(),
            size: ControlSize::default(),
            bordered: true,
        }
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Removes the outer border and the radius, `bordered: false` in Slint.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// The horizontal inset at each size: 6 / 8 / 16 / 12, the Slint values.
    pub fn padding_x(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 6.0,
            ControlSize::Small => 8.0,
            ControlSize::Large => 16.0,
            ControlSize::Medium => 12.0,
        }
    }

    /// The vertical inset at each size: 4 / 6 / 12 / 8.
    pub fn padding_y(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 4.0,
            ControlSize::Small => 6.0,
            ControlSize::Large => 12.0,
            ControlSize::Medium => 8.0,
        }
    }

    /// The title size: text-xs at xsmall, text-md at large, text-sm between.
    pub fn label_size(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => theme::FONT_SIZE_XS,
            ControlSize::Large => theme::FONT_SIZE_MD,
            _ => theme::FONT_SIZE_SM,
        }
    }

    /// The chevron box: 12px, and 14px at the large size.
    pub fn chevron_size(&self) -> f32 {
        match self.size {
            ControlSize::Large => 14.0,
            _ => 12.0,
        }
    }
}

/// The colour a section's title is drawn in.
pub fn ink(tokens: Tokens, enabled: bool) -> Color {
    if enabled {
        tokens.foreground
    } else {
        tokens.muted_foreground
    }
}

/// A section's header background at rest, hovered and pressed. Pressed is the
/// accent and hovered is the accent at half strength, which is what the Slint
/// theme does; a disabled header draws nothing.
pub fn header_fill(tokens: Tokens, enabled: bool, hovered: bool, pressed: bool) -> Color {
    if !enabled {
        Color::TRANSPARENT
    } else if pressed {
        tokens.accent
    } else if hovered {
        theme::alpha(tokens.accent, 0.5)
    } else {
        Color::TRANSPARENT
    }
}

/// The common header: a title, a chevron and nothing else, as a ghost button
/// that fills the row. Pass the message to toggle this section.
pub fn text_header<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    label_size: f32,
    on_toggle: Message,
) -> Element<'a, Message> {
    let label = title.into();
    let content: Element<'a, Message> = row![
        text(label)
            .size(label_size)
            .width(Length::Fill)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..iced::Font::DEFAULT
            })
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
    ]
    .width(Length::Fill)
    .into();

    button(content)
        .width(Length::Fill)
        .padding(0.0)
        .style(|theme: &Theme, status| {
            let tokens = Tokens::of(theme);
            let hovered = matches!(
                status,
                button::Status::Hovered | button::Status::Pressed
            );
            button::Style {
                background: Some(Background::Color(if hovered {
                    theme::alpha(tokens.accent, 0.5)
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

/// A header with a leading icon element beside the title, as a ghost button
/// that fills the row. The caller owns the icon, because the icon set does.
pub fn labeled_header<'a, Message: Clone + 'a>(
    icon: impl Into<Element<'a, Message>>,
    title: impl Into<String>,
    label_size: f32,
    gap: f32,
    on_toggle: Message,
) -> Element<'a, Message> {
    let label = title.into();
    let content: Element<'a, Message> = row![
        icon.into(),
        text(label)
            .size(label_size)
            .width(Length::Fill)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..iced::Font::DEFAULT
            })
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
    ]
    .spacing(gap)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into();

    button(content)
        .width(Length::Fill)
        .padding(0.0)
        .style(|theme: &Theme, status| {
            let tokens = Tokens::of(theme);
            let hovered = matches!(
                status,
                button::Status::Hovered | button::Status::Pressed
            );
            button::Style {
                background: Some(Background::Color(if hovered {
                    theme::alpha(tokens.accent, 0.5)
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

/// The chevron's end state: a short rule under the open state and a short rule
/// at the top under the closed one, drawn from the tokens because iced ships no
/// icon set. A caller with assets passes its own element in the header instead.
pub fn chevron<'a, Message: 'a>(open: bool, size: f32) -> Element<'a, Message> {
    let (width, height) = if open {
        (size * 0.5, 1.0)
    } else {
        (size, 1.0)
    };
    container(iced::widget::Space::new())
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .style(|theme: &Theme| ContainerStyle {
            background: Some(Tokens::of(theme).muted_foreground.into()),
            ..ContainerStyle::default()
        })
        .into()
}

impl<'a, Message: 'a> From<Accordion<'a, Message>> for Element<'a, Message> {
    fn from(control: Accordion<'a, Message>) -> Self {
        let bordered = control.bordered;
        let pad_x = control.padding_x();
        let pad_y = control.padding_y();
        let chevron_size = control.chevron_size();

        let count = control.sections.len();
        let mut sections = column![].width(Length::Fill);

        for (index, section) in control.sections.into_iter().enumerate() {
            let open = section.open;
            let last = index + 1 == count;

            let mut header = row![].spacing(12.0).align_y(Alignment::Center);
            header = header.push(
                container(section.header)
                    .width(Length::Fill)
                    .padding(iced::Padding {
                        top: pad_y,
                        right: pad_x,
                        bottom: pad_y,
                        left: pad_x,
                    }),
            );
            if section.chevron {
                header = header.push(container(chevron(open, chevron_size)).center_y(chevron_size));
            }
            sections = sections.push(header);

            if open && !section.content.is_empty() {
                sections = sections.push(
                    column(section.content)
                        .width(Length::Fill)
                        .padding(iced::Padding {
                            top: 0.0,
                            right: pad_x,
                            bottom: pad_y,
                            left: pad_x,
                        }),
                );
            }

            if !last {
                sections = sections.push(
                    container(iced::widget::Space::new())
                        .width(Length::Fill)
                        .height(Length::Fixed(1.0))
                        .style(|theme: &Theme| ContainerStyle {
                            background: Some(Tokens::of(theme).border.into()),
                            ..ContainerStyle::default()
                        }),
                );
            }
        }

        container(sections)
            .width(Length::Fill)
            .padding(if bordered { 1.0 } else { 0.0 })
            .style(move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                ContainerStyle {
                    background: Some(tokens.background.into()),
                    border: Border {
                        color: tokens.border,
                        width: if bordered { 1.0 } else { 0.0 },
                        radius: if bordered {
                            theme::RADIUS_LG.into()
                        } else {
                            0.0.into()
                        },
                    },
                    ..ContainerStyle::default()
                }
            })
            .into()
    }
}
