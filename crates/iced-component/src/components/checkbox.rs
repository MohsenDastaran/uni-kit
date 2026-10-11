//! A binary choice. Mirrors GPUI Component's `Checkbox` and the Slint version
//! beside it: a label, an optional description under it, the indeterminate
//! dash, sizes, and a disabled state.
//!
//! The box is built from the primitives iced gives rather than from iced's own
//! `checkbox`, because the Slint control stacks a label and a muted description
//! beside the box and iced's checkbox takes a single text run. The tick is the
//! renderer's own checkmark glyph, drawn from `Iced-Icons`, the only icon font
//! iced ships — Slint names a `"check"` or `"minus"` SVG in a folder its build
//! script resolves, and an iced application owns its assets, so the glyph is
//! the one place this file draws something the Slint version names.
//!
//! **Disabled is the absence of a message**, as everywhere in this crate:
//! `on_toggle(msg)` produces an enabled control, and a checkbox built without
//! one is drawn at the disabled fill and ink. Slint draws the whole control at
//! half opacity; iced's `Disabled` treatment is the lower-contrast fill, which
//! is the same intent.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{checkbox, column, row, text};
use iced::{Border, Color, Element, Theme};

use super::theme::{self, ControlSize, Tokens};

/// A checkbox, built with the methods below.
pub struct Checkbox<'a, Message> {
    checked: bool,
    indeterminate: bool,
    label: String,
    description: String,
    size: ControlSize,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message> Checkbox<'a, Message> {
    pub fn new(checked: bool) -> Self {
        Self {
            checked,
            indeterminate: false,
            label: String::new(),
            description: String::new(),
            size: ControlSize::default(),
            on_toggle: None,
        }
    }

    /// The dash: a filled-looking box for a choice that stands for "some".
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// The smaller muted line under the label.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The value to report when the box is toggled. Without this the control is
    /// disabled, which is how iced spells a control that cannot be pressed.
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// The box's side: 12 / 14 / 16 / 18, the Slint `box`.
    pub fn box_size(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 12.0,
            ControlSize::Small => 14.0,
            ControlSize::Large => 18.0,
            ControlSize::Medium => 16.0,
        }
    }
}

/// The colour a checkbox's label is drawn in.
pub fn label_ink(tokens: Tokens, disabled: bool) -> Color {
    if disabled {
        tokens.muted_foreground
    } else {
        tokens.foreground
    }
}

/// The state and metrics a checkbox draws with. Components pass it to
/// `.style(..)`; it is `pub` so an application can reuse it for its own
/// checkboxes.
pub struct Style {
    pub size: ControlSize,
    pub disabled: bool,
}

impl Style {
    pub fn new(size: ControlSize) -> Self {
        Self {
            size,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn resolve(&self, tokens: Tokens, status: checkbox::Status) -> checkbox::Style {
        let on = match status {
            checkbox::Status::Active { is_checked }
            | checkbox::Status::Hovered { is_checked }
            | checkbox::Status::Disabled { is_checked } => is_checked,
        };
        let hovered = matches!(status, checkbox::Status::Hovered { .. });
        let disabled = self.disabled || matches!(status, checkbox::Status::Disabled { .. });

        let fill = if on {
            if disabled {
                theme::alpha(tokens.primary, 0.5)
            } else if hovered {
                theme::hovered(&tokens, tokens.primary)
            } else {
                tokens.primary
            }
        } else if disabled {
            theme::alpha(tokens.background, 0.5)
        } else if hovered {
            theme::hovered(&tokens, tokens.background)
        } else {
            tokens.background
        };

        let border_color = if on {
            if disabled {
                theme::alpha(tokens.primary, 0.5)
            } else {
                tokens.primary
            }
        } else if disabled {
            theme::alpha(tokens.input, 0.5)
        } else {
            tokens.input
        };

        checkbox::Style {
            background: fill.into(),
            icon_color: if disabled {
                theme::alpha(tokens.primary_foreground, 0.5)
            } else {
                tokens.primary_foreground
            },
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: Some(label_ink(tokens, disabled)),
        }
    }
}

impl<'a, Message> From<Checkbox<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Checkbox<'a, Message>) -> Self {
        let size = control.size;
        let on = control.checked || control.indeterminate;
        let box_size = control.box_size();
        let label = control.label;
        let description = control.description;
        let label_size = size.label_font_size();
        let on_toggle = control.on_toggle;
        let disabled = on_toggle.is_none();

        let box_control = checkbox(on)
            .size(box_size)
            .spacing(0.0)
            .on_toggle_maybe(on_toggle)
            .style(move |theme: &Theme, status| {
                Style::new(size)
                    .disabled(disabled)
                    .resolve(Tokens::of(theme), status)
            });

        if label.is_empty() && description.is_empty() {
            return box_control.into();
        }

        let mut lines = column![].spacing(theme::SPACING_XXS);

        if !label.is_empty() {
            lines = lines.push(
                text(label)
                    .size(label_size)
                    .style(move |theme: &Theme| text::Style {
                        color: Some(label_ink(Tokens::of(theme), disabled)),
                    }),
            );
        }

        if !description.is_empty() {
            lines = lines.push(
                text(description)
                    .size(theme::FONT_SIZE_XS)
                    .style(move |theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
            );
        }

        row![box_control, lines]
            .spacing(theme::SPACING_SM)
            .align_y(iced::Alignment::Center)
            .into()
    }
}
