//! A radio: one option in a mutually exclusive set. Mirrors GPUI Component's
//! `Radio` and `RadioGroup` and the Slint version beside it: a disc with a dot,
//! a label, the four sizes, a disabled state, and a group that keeps one
//! selection.
//!
//! `radio`, `switch` and `toggle` are one shape with three drawings. The shape
//! is [`super::button::handle`]: a control that reports the value it wants to
//! become and cannot be pressed without one. This file draws the disc and
//! reports `true` — a radio never turns itself off — and [`RadioGroup`] composes
//! it and reports the index that was chosen.
//!
//! The disc is drawn *inside* the handle rather than as the handle's chrome,
//! because the handle spans the label as well — Slint's `TouchArea` covers the
//! whole control — and a chrome disc would ring the label too.
//!
//! **Disabled is the absence of a message**, as everywhere in this crate:
//! `on_change(msg)` produces an enabled radio, and one built without it draws
//! the disabled disc and ink. Slint fades the whole control to half opacity,
//! which iced 0.14 cannot do to a subtree, so the disc and the label take their
//! lower-contrast values instead — the same reading `checkbox` uses.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::widget::button as iced_button;
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Shadow, Theme};

use super::button::handle;
use super::checkbox::label_ink;
use super::theme::{self, ControlSize, Tokens};

/// A radio, built with the methods below.
pub struct Radio<'a, Message> {
    checked: bool,
    label: String,
    description: String,
    size: ControlSize,
    on_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message> Radio<'a, Message> {
    pub fn new(checked: bool) -> Self {
        Self {
            checked,
            label: String::new(),
            description: String::new(),
            size: ControlSize::default(),
            on_change: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// The smaller muted line under the label, the same second line
    /// [`Checkbox`](super::checkbox::Checkbox) takes. A radio with one aligns its
    /// disc with the label's first line rather than centring against the whole
    /// block, which is the `cross-axis-alignment: start` a row of described
    /// choices needs.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The value to report when the radio is chosen. It only ever reports
    /// `true`, because a radio cannot be turned off from inside its set. Without
    /// this the control is disabled, which is how iced spells a control that
    /// cannot be pressed.
    pub fn on_change(mut self, on_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// The disc's side: 12 / 14 / 16 / 18, Slint's `disc`.
    pub fn disc_size(&self) -> f32 {
        disc_size(self.size)
    }
}

/// The disc's side for a size step.
fn disc_size(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 12.0,
        ControlSize::Small => 14.0,
        ControlSize::Large => 18.0,
        ControlSize::Medium => 16.0,
    }
}

/// The state a radio draws with, and the chrome of the handle it sits in.
///
/// It is `pub` so an application can restyle the whole row the way it restyles
/// a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub checked: bool,
    pub size: ControlSize,
    pub disabled: bool,
}

impl Style {
    pub fn new(checked: bool, size: ControlSize) -> Self {
        Self {
            checked,
            size,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The handle's own chrome.
    ///
    /// The disc is content, so the chrome is transparent in every state; it is
    /// the extension point an application can give the whole row a surface
    /// through, the same way [`Style`](super::button::Style) is for a button.
    pub fn resolve(&self, tokens: Tokens, _status: iced_button::Status) -> iced_button::Style {
        iced_button::Style {
            background: None,
            text_color: label_ink(tokens, self.disabled),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: (disc_size(self.size) / 2.0).into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }
}

/// The disc's fill and its edge. A checked disc keeps the page as its fill and
/// carries the primary colour in its edge and dot, which is the drawing Slint's
/// `Radio` makes.
fn disc_colors(tokens: Tokens, checked: bool, disabled: bool) -> (Color, Color) {
    let border = if checked {
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
    (tokens.background, border)
}

/// The dot's colour: the disc's own edge colour, at the disk's fill weight.
fn dot_color(tokens: Tokens, disabled: bool) -> Color {
    if disabled {
        theme::alpha(tokens.primary, 0.5)
    } else {
        tokens.primary
    }
}

/// The disc, with the checked dot centred in it.
fn disc<'a, Message>(checked: bool, size: ControlSize, disabled: bool) -> Element<'a, Message>
where
    Message: 'a,
{
    let side = disc_size(size);
    let dot = side / 2.0;

    let mark: Element<'a, Message> = if checked {
        container(Space::new().width(dot).height(dot))
            .style(move |theme: &Theme| container::Style {
                background: Some(dot_color(Tokens::of(theme), disabled).into()),
                border: iced::border::rounded(dot / 2.0),
                ..container::Style::default()
            })
            .into()
    } else {
        Space::new().into()
    };

    container(mark)
        .center_x(side)
        .center_y(side)
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            let (fill, border_color) = disc_colors(tokens, checked, disabled);
            container::Style {
                background: Some(fill.into()),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: (side / 2.0).into(),
                },
                ..container::Style::default()
            }
        })
        .into()
}

impl<'a, Message> From<Radio<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Radio<'a, Message>) -> Self {
        let Radio {
            checked,
            label,
            description,
            size,
            on_change,
        } = control;
        let disabled = on_change.is_none();
        let has_label = !label.is_empty();
        let has_description = !description.is_empty();
        let style = Style::new(checked, size).disabled(disabled);

        // The disc sits against the label's first line, which matters once a
        // description makes the block two lines tall; a single label keeps the
        // centring every other one-line control uses.
        let align_y = if has_description {
            Alignment::Start
        } else {
            Alignment::Center
        };

        let mut content = row![disc(checked, size, disabled)]
            .spacing(theme::SPACING_SM)
            .align_y(align_y);

        let mut lines = column![].spacing(theme::SPACING_XXS);

        if has_label {
            lines = lines.push(text(label).size(size.label_font_size()).style(
                move |theme: &Theme| text::Style {
                    color: Some(label_ink(Tokens::of(theme), disabled)),
                },
            ));
        }

        if has_description {
            lines = lines.push(
                text(description)
                    .size(theme::FONT_SIZE_XS)
                    .style(move |theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
            );
        }

        if has_label || has_description {
            content = content.push(lines);
        }

        // The value the radio wants to become: always `true`, never `!checked`.
        let on_press = on_change.map(|on_change| on_change(true));

        handle(content, on_press, move |theme: &Theme, status| {
            style.resolve(Tokens::of(theme), status)
        })
    }
}

/// A stack of radios that keep one selection. Slint's `RadioGroup` and GPUI's
/// are the same idea: the owner reads the chosen index out of `on_change` and
/// writes it back, which is what makes one selection replace another.
pub struct RadioGroup<'a, Message> {
    options: Vec<String>,
    selected: Option<usize>,
    size: ControlSize,
    disabled: bool,
    horizontal: bool,
    on_change: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message> RadioGroup<'a, Message> {
    /// A vertical group with nothing chosen.
    pub fn new() -> Self {
        Self {
            options: Vec::new(),
            selected: None,
            size: ControlSize::default(),
            disabled: false,
            horizontal: false,
            on_change: None,
        }
    }

    pub fn vertical() -> Self {
        Self::new()
    }

    /// The options side by side, for a short set. Slint's `RadioGroup` is
    /// always vertical; GPUI's `horizontal` is the same set laid out in a row.
    pub fn horizontal() -> Self {
        Self {
            horizontal: true,
            ..Self::new()
        }
    }

    /// The labels, one radio each, in reading order.
    pub fn options(mut self, options: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.options = options.into_iter().map(Into::into).collect();
        self
    }

    /// The chosen index. A radio never clears the group, so `None` shows
    /// nothing chosen until the owner writes one back.
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Disables every radio in the group, GPUI's `disabled(true)`.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The index to report when one is chosen. Without it the group is
    /// disabled, the same as a single radio.
    pub fn on_change(mut self, on_change: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_change = Some(Rc::new(on_change));
        self
    }
}

impl<'a, Message> Default for RadioGroup<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<RadioGroup<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(group: RadioGroup<'a, Message>) -> Self {
        let RadioGroup {
            options,
            selected,
            size,
            disabled,
            horizontal,
            on_change,
        } = group;

        let radios: Vec<Element<'a, Message>> = options
            .into_iter()
            .enumerate()
            .map(|(index, option)| {
                let mut radio = Radio::new(selected == Some(index)).label(option).size(size);
                // A disabled group carries no messages at all, which is what
                // makes every radio in it unable to be pressed.
                if !disabled {
                    if let Some(on_change) = on_change.clone() {
                        radio = radio.on_change(move |_| on_change(index));
                    }
                }
                radio.into()
            })
            .collect();

        if horizontal {
            row(radios).spacing(theme::SPACING_SM).into()
        } else {
            column(radios).spacing(theme::SPACING_SM).into()
        }
    }
}
