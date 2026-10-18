//! A container that groups related content under an optional title. Mirrors
//! GPUI Component's `GroupBox` and the Slint version beside it.
//!
//! The title sits outside the surface, so `Normal` reads as a heading over a
//! plain stack, while `Fill` and `Outline` add a surface with its own inset and
//! gap. A footer hangs under the surface as small muted text, with an 8px gap
//! that belongs to the footer rather than to the root spacing.
//!
//! The registry key is `group-box`; this file and its module are `group_box`,
//! because a hyphen is not a Rust identifier.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{column, container, text};
use iced::widget::container::Style as ContainerStyle;
use iced::{Border, Color, Element, Length, Theme};

use super::theme::{self, Tokens};

/// How the group's surface is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// A heading over a plain stack, with no surface at all.
    #[default]
    Normal,
    /// A filled surface in the theme's group-box colour.
    Fill,
    /// A transparent surface with a one-pixel border.
    Outline,
}

/// A group box, built with the methods below.
pub struct GroupBox<'a, Message> {
    title: String,
    variant: Variant,
    footer: String,
    content_padding: f32,
    corner_radius: f32,
    title_color: Option<Color>,
    body: Option<Element<'a, Message>>,
}

impl<'a, Message> GroupBox<'a, Message> {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            variant: Variant::default(),
            footer: String::new(),
            content_padding: 16.0,
            corner_radius: theme::RADIUS_MD,
            title_color: None,
            body: None,
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// Supporting text under the surface.
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = footer.into();
        self
    }

    /// The surface inset. 16px matches the gap between the surface's children.
    pub fn content_padding(mut self, padding: f32) -> Self {
        self.content_padding = padding;
        self
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn title_color(mut self, color: Color) -> Self {
        self.title_color = Some(color);
        self
    }

    /// The grouped content. Each child is stacked with a 16px gap, which is the
    /// spacing the Slint version gives the surface's own layout.
    pub fn body(mut self, body: impl Into<Element<'a, Message>>) -> Self {
        self.body = Some(body.into());
        self
    }
}

impl<'a, Message> Default for GroupBox<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The colours a group box draws its title, surface and footer with.
pub struct Style {
    pub variant: Variant,
    pub corner_radius: f32,
}

impl Style {
    pub fn new() -> Self {
        Self {
            variant: Variant::default(),
            corner_radius: theme::RADIUS_MD,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn resolve(&self, tokens: Tokens) -> ContainerStyle {
        ContainerStyle {
            background: match self.variant {
                Variant::Fill => Some(tokens.group_box.into()),
                _ => None,
            },
            text_color: Some(tokens.group_box_foreground),
            border: Border {
                color: tokens.border,
                width: if self.variant == Variant::Outline {
                    1.0
                } else {
                    0.0
                },
                radius: self.corner_radius.into(),
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

impl<'a, Message> From<GroupBox<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: GroupBox<'a, Message>) -> Self {
        let variant = control.variant;
        let content_padding = control.content_padding;
        let corner_radius = control.corner_radius;
        let title_color = control.title_color;
        let title = control.title.clone();
        let footer = control.footer.clone();
        let inset = if variant == Variant::Normal {
            0.0
        } else {
            content_padding
        };

        let mut surface_children = column![].spacing(16.0).padding(inset);

        if let Some(body) = control.body {
            surface_children = surface_children.push(body);
        }

        let surface = container(surface_children)
            .width(Length::Fill)
            .style(move |theme: &Theme| {
                Style::new()
                    .variant(variant)
                    .corner_radius(corner_radius)
                    .resolve(Tokens::of(theme))
            });

        let mut stack = column![]
            .spacing(if footer.is_empty() { 0.0 } else { 8.0 })
            .push(surface);

        if !footer.is_empty() {
            stack = stack.push(
                text(footer)
                    .size(theme::FONT_SIZE_SM)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
            );
        }

        let mut group = column![]
            .width(Length::Fill)
            .spacing(if variant == Variant::Normal { 16.0 } else { 12.0 });

        if !title.is_empty() {
            group = group.push(
                text(title)
                    .size(theme::FONT_SIZE_SM)
                    .font(iced::Font {
                        weight: iced::font::Weight::Medium,
                        ..iced::Font::DEFAULT
                    })
                    .style(move |theme: &Theme| text::Style {
                        color: Some(match title_color {
                            Some(color) => color,
                            None => Tokens::of(theme).group_box_title_foreground,
                        }),
                    }),
            );
        }

        group = group.push(stack);
        group.into()
    }
}
