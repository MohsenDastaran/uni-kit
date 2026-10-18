//! An overlay anchored to the corner of its child: a count, a dot or a small
//! status icon. Mirrors GPUI Component's `Badge` and the Slint version beside
//! it — `count` above zero, `dot`, or an icon element.
//!
//! iced draws overlapping content with `stack`, so the badge lays its child
//! down as the base layer and the overlay on top of it, aligned to the child's
//! top-right corner. The overlay hangs past the child's bounds, which `stack`
//! does not clip.
//!
//! Slint names its icon (`"check"`) and resolves it from its own folder; an
//! iced application owns its assets, so the icon is an element the caller
//! passes in. A count above `max` reads as `max+`.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{container, stack, text};
use iced::{Color, Element, Length, Theme};

use super::theme::{ControlSize, Tokens};

/// The badge, built with the methods below.
pub struct Badge<'a, Message> {
    count: u32,
    max: u32,
    dot: bool,
    size: ControlSize,
    color: Option<Color>,
    icon: Option<Element<'a, Message>>,
    inner_offset: f32,
    content: Element<'a, Message>,
}

impl<'a, Message> Badge<'a, Message> {
    /// Wraps `content`, which the badge anchors to.
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            count: 0,
            max: 99,
            dot: false,
            size: ControlSize::default(),
            color: None,
            icon: None,
            inner_offset: 0.0,
            content: content.into(),
        }
    }

    /// The number, shown while it is above zero. A count above `max` reads as
    /// `max+`.
    pub fn count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    /// The largest count the chip spells out; above it the chip reads `max+`.
    pub fn max(mut self, max: u32) -> Self {
        self.max = max;
        self
    }

    /// A six-pixel dot instead of a number. It takes the place of the count,
    /// so the chip is drawn while `count` is above zero.
    pub fn dot(mut self) -> Self {
        self.dot = true;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The overlay's colour. The default is the theme's danger colour.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// A status icon in a round chip, in place of the count.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self.dot = false;
        self
    }

    /// How far the overlay is pulled back inside the child's top-right corner.
    /// The Slint version anchors the icon chip two pixels inside the corner and
    /// the dot and count flush with it, which is `0.0` here.
    pub fn inner_offset(mut self, offset: f32) -> Self {
        self.inner_offset = offset;
        self
    }

    /// The chip's diameter: 24 for large, 16 for medium, 10 for the rest.
    fn chip(&self) -> f32 {
        match self.size {
            ControlSize::Large => 24.0,
            ControlSize::Small | ControlSize::XSmall => 10.0,
            ControlSize::Medium => 16.0,
        }
    }

    fn number_height(&self) -> f32 {
        match self.size {
            ControlSize::Small | ControlSize::XSmall => 14.0,
            _ => 16.0,
        }
    }

    fn label(&self) -> Option<String> {
        if self.dot || self.icon.is_some() || self.count == 0 {
            return None;
        }
        Some(if self.count > self.max {
            format!("{}+", self.max)
        } else {
            self.count.to_string()
        })
    }

    fn tone(&self, tokens: Tokens) -> Color {
        self.color.unwrap_or(tokens.danger)
    }
}

impl<'a, Message> From<Badge<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Badge<'a, Message>) -> Self {
        let chip = control.chip();
        let number_height = control.number_height();
        let label = control.label();
        let show_dot = control.dot && (control.count > 0 || control.icon.is_none());
        let color = control.color;
        let inner_offset = control.inner_offset;
        let icon = control.icon;

        let mut overlay = stack![control.content];

        if show_dot {
            let dot = container(iced::widget::Space::new())
                .width(Length::Fixed(6.0))
                .height(Length::Fixed(6.0))
                .style(move |theme: &Theme| container::Style {
                    background: Some(tone(Tokens::of(theme), color).into()),
                    border: iced::Border {
                        radius: 3.0.into(),
                        ..iced::Border::default()
                    },
                    ..container::Style::default()
                });
            overlay = overlay.push(
                container(dot)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(iced::Alignment::End)
                    .align_y(iced::Alignment::Start),
            );
        }

        if let Some(label) = label {
            // A count chip is a solid fill, so its ink is the solid-fill
            // foreground every other control in this crate reads. A literal
            // white measured 3.76:1 on the light danger, 2.77:1 on the dark one
            // and 3.90:1 on the page's dark `--destructive`; the token takes the
            // dark pair to 6.48:1 and the page's dark pair to 4.60:1 and leaves
            // the authored light pair where it is.
            let chip = container(
                text(label)
                    .size(10.0)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).primary_foreground),
                    }),
            )
            .height(Length::Fixed(number_height))
            .padding([0.0, 4.0])
            .align_y(iced::Alignment::Center)
            .style(move |theme: &Theme| container::Style {
                background: Some(tone(Tokens::of(theme), color).into()),
                border: iced::Border {
                    radius: (number_height / 2.0).into(),
                    ..iced::Border::default()
                },
                ..container::Style::default()
            });

            overlay = overlay.push(
                container(chip)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(iced::Alignment::End)
                    .align_y(iced::Alignment::Start)
                    .padding(iced::Padding {
                        top: 0.0,
                        right: 6.0 - inner_offset,
                        bottom: 0.0,
                        left: 0.0,
                    }),
            );
        }

        if let Some(icon) = icon {
            let chip = container(container(icon).center(chip * 0.62))
                .width(Length::Fixed(chip))
                .height(Length::Fixed(chip))
                .center(chip)
                .style(move |theme: &Theme| container::Style {
                    background: Some(tone(Tokens::of(theme), color).into()),
                    border: iced::Border {
                        color: Tokens::of(theme).background,
                        width: 1.0,
                        radius: (chip / 2.0).into(),
                    },
                    ..container::Style::default()
                });

            overlay = overlay.push(
                container(chip)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(iced::Alignment::End)
                    .align_y(iced::Alignment::Start)
                    .padding(iced::Padding {
                        top: 0.0,
                        right: 2.0 - inner_offset,
                        bottom: 0.0,
                        left: 0.0,
                    }),
            );
        }

        overlay.into()
    }
}

fn tone(tokens: Tokens, color: Option<Color>) -> Color {
    color.unwrap_or(tokens.danger)
}
