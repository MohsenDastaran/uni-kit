//! A pressed-in control. Mirrors GPUI Component's `Toggle` and the Slint version
//! beside it: an optional icon and label, the four sizes, the `default` and
//! `outline` variants, and the pressed/unpressed drawing.
//!
//! `radio`, `switch` and `toggle` are one shape with three drawings. The shape
//! is [`super::button::handle`]: a control that reports the value it wants to
//! become and cannot be pressed without one. This file draws the whole control
//! as the handle's chrome — the pressed fill and its edge — and reports the new
//! boolean.
//!
//! `default` stays flat until it is hovered or pressed; `outline` keeps a frame
//! at rest, which is what a toolbar wants. Slint's selected ink and fill are the
//! `accent` pair, so a pressed toggle carries its colour in the fill and its
//! label in the foreground of that fill.
//!
//! **Disabled is the absence of a message**, as everywhere in this crate:
//! `on_toggle(msg)` produces an enabled toggle, and one built without it draws
//! the disabled fill and ink. Slint fades the whole control to half opacity,
//! which iced 0.14 cannot do to a subtree, so the fill and the ink take their
//! lower-contrast values instead.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow, Theme};

use super::button::handle;
use super::theme::{self, ControlSize, Tokens};
use iced::widget::button as iced_button;

/// How the toggle is drawn. Slint's `ToggleVariant`; GPUI calls the default one
/// `ghost`, and both stay flat until they are hovered or pressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Outline,
}

/// A toggle, built with the methods below.
pub struct Toggle<'a, Message> {
    label: String,
    icon: Option<Element<'a, Message>>,
    selected: bool,
    variant: Variant,
    size: ControlSize,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message> Toggle<'a, Message> {
    pub fn new(selected: bool) -> Self {
        Self {
            label: String::new(),
            icon: None,
            selected,
            variant: Variant::default(),
            size: ControlSize::default(),
            on_toggle: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// A leading element, drawn before the label. The icon set belongs to the
    /// application, so iced's toggle — unlike Slint's, which names an SVG — takes
    /// the element rather than an icon name.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The value to report when the toggle is pressed. Without this the control
    /// is disabled, which is how iced spells a control that cannot be pressed.
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }
}

/// The colour a toggle's label and icon are drawn in: the accent's foreground
/// while pressed, the ordinary foreground at rest, and the muted ink when
/// disabled.
pub fn ink(tokens: Tokens, selected: bool, disabled: bool) -> Color {
    if disabled {
        tokens.muted_foreground
    } else if selected {
        tokens.accent_foreground
    } else {
        tokens.foreground
    }
}

/// The state a toggle draws its chrome with. Components pass it to
/// `.style(..)`; it is `pub` so an application can reuse it for its own
/// toggles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub variant: Variant,
    pub selected: bool,
    pub size: ControlSize,
    pub disabled: bool,
}

impl Style {
    pub fn new(variant: Variant, selected: bool, size: ControlSize) -> Self {
        Self {
            variant,
            selected,
            size,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn resolve(&self, tokens: Tokens, status: iced_button::Status) -> iced_button::Style {
        let disabled = self.disabled || matches!(status, iced_button::Status::Disabled);
        let hovered = matches!(status, iced_button::Status::Hovered);
        let pressed = matches!(status, iced_button::Status::Pressed);
        let outline = self.variant == Variant::Outline;

        // Hover moves the default toggle onto the accent surface, which is what
        // Slint animates to; a press puts the pointer on a shade of the same
        // surface rather than the page. The outline variant keeps its own frame
        // and only changes on selection.
        let fill = if self.selected {
            if disabled {
                theme::alpha(tokens.accent, 0.5)
            } else {
                tokens.accent
            }
        } else if outline {
            if disabled {
                theme::alpha(tokens.background, 0.5)
            } else {
                tokens.background
            }
        } else if disabled {
            Color::TRANSPARENT
        } else if pressed {
            theme::pressed(&tokens, tokens.accent)
        } else if hovered {
            tokens.accent
        } else {
            Color::TRANSPARENT
        };

        let border_color = if self.selected {
            if disabled {
                theme::alpha(tokens.accent, 0.5)
            } else {
                tokens.accent
            }
        } else if outline {
            if disabled {
                theme::alpha(tokens.border, 0.5)
            } else {
                tokens.border
            }
        } else {
            Color::TRANSPARENT
        };

        iced_button::Style {
            background: if fill.a > 0.0 {
                Some(Background::Color(fill))
            } else {
                None
            },
            text_color: ink(tokens, self.selected, disabled),
            border: Border {
                color: border_color,
                width: if self.selected || outline { 1.0 } else { 0.0 },
                radius: theme::RADIUS_MD.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }
}

impl<'a, Message> From<Toggle<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Toggle<'a, Message>) -> Self {
        let Toggle {
            label,
            icon,
            selected,
            variant,
            size,
            on_toggle,
        } = control;
        let disabled = on_toggle.is_none();
        let style = Style::new(variant, selected, size).disabled(disabled);

        let height = size.control_height();
        let icon_only = label.is_empty() && icon.is_some();
        let padding = if icon_only {
            0.0
        } else {
            size.control_padding()
        };

        let mut content = row![].spacing(6.0).align_y(Alignment::Center);
        if let Some(icon) = icon {
            content = content.push(icon);
        }
        if !label.is_empty() {
            content = content.push(
                text(label)
                    .size(size.control_font_size())
                    .font(iced::Font {
                        weight: iced::font::Weight::Medium,
                        ..iced::Font::DEFAULT
                    })
                    .style(move |theme: &Theme| text::Style {
                        color: Some(ink(Tokens::of(theme), selected, disabled)),
                    }),
            );
        }

        // An icon on its own is square, like GPUI's `min_w_8`; a labelled one is
        // as wide as its own content, which is what stops a layout from
        // stretching it across the row.
        let content: Element<'a, Message> = if icon_only {
            container(content)
                .center_x(Length::Fixed(height))
                .center_y(Length::Fixed(height))
                .into()
        } else {
            container(content)
                .width(Length::Shrink)
                .height(height)
                .padding([0.0, padding])
                .align_y(Alignment::Center)
                .into()
        };

        // The value the toggle wants to become, not the one it is.
        let on_press = on_toggle.map(|on_toggle| on_toggle(!selected));

        handle(content, on_press, move |theme: &Theme, status| {
            style.resolve(Tokens::of(theme), status)
        })
    }
}
