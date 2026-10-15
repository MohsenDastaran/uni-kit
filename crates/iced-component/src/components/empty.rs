//! An empty-state panel. Mirrors GPUI Component's `Empty` and the Slint version
//! beside it: a media slot (a framed icon, a bare icon, or none), a title, a
//! wrapping description, up to two actions, and a bordered surface.
//!
//! The content sits in a column that is at most `text_max_width` wide and is
//! centred or leading-aligned by [`Align`]. Slint has to clamp that column to a
//! fixed width because a wrapping text reports its longest word as its
//! preferred width; iced resolves the same problem with a maximum width and an
//! aligned fill, so the description wraps the same way without a measurement
//! pass.
//!
//! Slint cannot draw a dashed border, so its `bordered` rectangle is a solid
//! one-pixel border in the border colour. iced can draw dashes, but the
//! component keeps Slint's answer, which is also what GPUI draws.
//!
//! The media is an element the caller passes: the icon set belongs to the
//! application. [`Media`] only chooses the frame around it — `Icon` puts it in
//! the muted 32px box and `None` draws no media even when a slot is given. The
//! bare form is the element itself, because the caller already knows how big
//! its own icon is.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{column, container, row, text};
use iced::widget::container::Style as ContainerStyle;
use iced::{Alignment, Border, Color, Element, Length, Theme};

use super::theme::{self, Tokens};

/// What the media slot draws.
///
/// Slint has three treatments, `icon`, `plain` and `none`, and all three are
/// here: `icon` frames the media, `plain` draws the caller's element bare at
/// its own size, and `none` drops it. The bare form is what an avatar or a
/// group of them uses, because iced has no icon set to size them against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Media {
    /// A muted, rounded 32px frame around the media — GPUI's
    /// `EmptyMediaVariant::Icon`.
    #[default]
    Icon,
    /// No frame: the caller's element at its own size, which is how an avatar
    /// or a group of them fills the slot — GPUI's `EmptyMediaVariant::Default`
    /// and Slint's `plain`.
    Plain,
    /// No media at all: the slot is skipped even when it is given.
    None,
}

/// How the content is packed horizontally. Named [`Align`] because
/// `iced::Alignment` is the type a caller already has in scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Center,
    Start,
}

/// An empty state, built with the methods below.
pub struct Empty<'a, Message> {
    media: Media,
    media_slot: Option<Element<'a, Message>>,
    title: String,
    description: String,
    action: Option<Element<'a, Message>>,
    secondary_action: Option<Element<'a, Message>>,
    bordered: bool,
    background: Option<Color>,
    padding: f32,
    text_max_width: f32,
    align: Align,
}

impl<'a, Message> Empty<'a, Message> {
    pub fn new() -> Self {
        Self {
            media: Media::default(),
            media_slot: None,
            title: String::new(),
            description: String::new(),
            action: None,
            secondary_action: None,
            bordered: false,
            background: None,
            padding: 24.0,
            text_max_width: 384.0,
            align: Align::default(),
        }
    }

    /// Which of the three media treatments the panel draws.
    pub fn media(mut self, media: Media) -> Self {
        self.media = media;
        self
    }

    /// The media itself: the icon, or a group of avatars. The Slint version
    /// names its icon and builds avatars from initials; iced applications own
    /// their assets, so the slot takes the element.
    pub fn media_slot(mut self, media: impl Into<Element<'a, Message>>) -> Self {
        self.media_slot = Some(media.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// The primary action.
    pub fn action(mut self, action: impl Into<Element<'a, Message>>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// The outlined action beside the primary one.
    pub fn secondary_action(
        mut self,
        action: impl Into<Element<'a, Message>>,
    ) -> Self {
        self.secondary_action = Some(action.into());
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// The surface fill. The page shows through by default.
    pub fn background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// The widest the title and the description wrap to.
    pub fn text_max_width(mut self, width: f32) -> Self {
        self.text_max_width = width;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    fn centered(&self) -> bool {
        self.align == Align::Center
    }
}

impl<'a, Message> Default for Empty<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The colours an empty state draws its surface with.
pub struct Style {
    pub bordered: bool,
    pub background: Option<Color>,
}

impl Style {
    pub fn new() -> Self {
        Self {
            bordered: false,
            background: None,
        }
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn background(mut self, background: Color) -> Self {
        self.background = Some(background);
        self
    }

    pub fn resolve(&self, tokens: Tokens) -> ContainerStyle {
        ContainerStyle {
            background: self.background.map(Into::into),
            border: Border {
                color: tokens.border,
                width: if self.bordered { 1.0 } else { 0.0 },
                radius: theme::RADIUS_XL.into(),
            },
            ..ContainerStyle::default()
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Empty<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Empty<'a, Message>) -> Self {
        let centered = control.centered();
        let padding = control.padding;
        let text_max_width = control.text_max_width;
        let media = control.media;
        let bordered = control.bordered;
        let background = control.background;
        let title = control.title.clone();
        let description = control.description.clone();
        let horizontal = if centered {
            Alignment::Center
        } else {
            Alignment::Start
        };

        let mut content = column![].spacing(8.0).width(Length::Fill);

        if media != Media::None {
            if let Some(slot) = control.media_slot {
                let slot: Element<'a, Message> = match media {
                    Media::Icon => container(container(slot).center(16.0))
                        .width(Length::Fixed(32.0))
                        .height(Length::Fixed(32.0))
                        .center(32.0)
                        .style(|theme: &Theme| ContainerStyle {
                            background: Some(Tokens::of(theme).muted.into()),
                            border: Border {
                                radius: theme::RADIUS_LG.into(),
                                ..Border::default()
                            },
                            ..ContainerStyle::default()
                        })
                        .into(),
                    _ => container(slot).height(Length::Fixed(32.0)).into(),
                };

                content = content.push(
                    container(slot)
                        .width(Length::Fill)
                        .padding(iced::Padding {
                            top: 0.0,
                            right: 0.0,
                            bottom: 8.0,
                            left: 0.0,
                        })
                        .align_x(horizontal),
                );
            }
        }

        if !title.is_empty() {
            content = content.push(
                text(title)
                    .size(theme::FONT_SIZE_SM)
                    .font(iced::Font {
                        weight: iced::font::Weight::Medium,
                        ..iced::Font::DEFAULT
                    })
                    .width(Length::Fill)
                    .align_x(horizontal)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).foreground),
                    }),
            );
        }

        if !description.is_empty() {
            content = content.push(
                text(description)
                    .size(theme::FONT_SIZE_SM)
                    .width(Length::Fill)
                    .align_x(horizontal)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
            );
        }

        let mut actions = row![].spacing(8.0).align_y(Alignment::Center);
        let mut has_action = false;
        if let Some(action) = control.action {
            actions = actions.push(action);
            has_action = true;
        }
        if let Some(secondary) = control.secondary_action {
            actions = actions.push(secondary);
            has_action = true;
        }
        if has_action {
            content = content.push(
                container(actions)
                    .width(Length::Fill)
                    .align_x(horizontal),
            );
        }

        let column = column![content]
            .width(Length::Fill)
            .max_width(text_max_width)
            .padding(padding)
            .spacing(16.0)
            .align_x(horizontal);

        container(column)
            .width(Length::Fill)
            .align_x(horizontal)
            .style(move |theme: &Theme| {
                let mut style = Style::new().bordered(bordered);
                if let Some(background) = background {
                    style = style.background(background);
                }
                style.resolve(Tokens::of(theme))
            })
            .into()
    }
}
