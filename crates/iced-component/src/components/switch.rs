//! An on/off switch. Mirrors GPUI Component's `Switch` and the Slint version
//! beside it: a rounded track with a sliding thumb, a label, the four sizes,
//! and a disabled state.
//!
//! `radio`, `switch` and `toggle` are one shape with three drawings. The shape
//! is [`super::button::handle`]: a control that reports the value it wants to
//! become and cannot be pressed without one. This file draws the track and the
//! thumb and reports the new boolean.
//!
//! The track is content rather than the handle's chrome for the same reason
//! `radio`'s disc is: the handle spans the label too, so a chrome track would
//! stretch under the text. Slint moves the thumb by animating its `x`; iced has
//! no transition on a laid-out child, so the thumb is placed at the end it
//! belongs to and snaps there.
//!
//! **Disabled is the absence of a message**, as everywhere in this crate:
//! `on_toggle(msg)` produces an enabled switch, and one built without it draws
//! the disabled track and ink. Slint fades the whole control to half opacity,
//! which iced 0.14 cannot do to a subtree, so the track and the label take
//! their lower-contrast values instead.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{Space, container, row, text};
use iced::{Alignment, Border, Color, Element, Shadow, Theme};

use super::button::handle;
use super::checkbox::label_ink;
use super::theme::{self, ControlSize, Tokens};
use iced::widget::button as iced_button;

/// A switch, built with the methods below.
pub struct Switch<'a, Message> {
    checked: bool,
    label: String,
    size: ControlSize,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message> Switch<'a, Message> {
    pub fn new(checked: bool) -> Self {
        Self {
            checked,
            label: String::new(),
            size: ControlSize::default(),
            on_toggle: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The value to report when the switch is pressed. Without this the control
    /// is disabled, which is how iced spells a control that cannot be pressed.
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// The track's width and height: 28×16 for the two small steps and 36×20
    /// for the two large ones, Slint's `track-w` and `track-h`.
    pub fn track_size(&self) -> (f32, f32) {
        let (width, height, _) = metrics(self.size);
        (width, height)
    }

    /// The thumb's side: 12 for the two small steps and 16 for the two large
    /// ones, Slint's `thumb`, inset two pixels from the track's edge.
    pub fn thumb_size(&self) -> f32 {
        let (_, _, thumb) = metrics(self.size);
        thumb
    }
}

/// The track's width, height and thumb side for a size step. Slint pairs
/// `xsmall` with `small` and `medium` with `large`.
fn metrics(size: ControlSize) -> (f32, f32, f32) {
    match size {
        ControlSize::XSmall | ControlSize::Small => (28.0, 16.0, 12.0),
        ControlSize::Medium | ControlSize::Large => (36.0, 20.0, 16.0),
    }
}

/// A switch's label: 14px on the two small steps and 16px on the two large
/// ones, which is Slint's `font-size-sm`/`font-size-md` choice rather than the
/// label-size table `checkbox` uses.
fn label_size(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall | ControlSize::Small => theme::FONT_SIZE_SM,
        ControlSize::Medium | ControlSize::Large => theme::FONT_SIZE_MD,
    }
}

/// The track's fill for a state. Unchecked is the secondary surface a shade
/// darker, which is what keeps the near-page thumb visible against it in both
/// appearances.
fn track_fill(tokens: Tokens, checked: bool, disabled: bool) -> Color {
    if disabled {
        theme::alpha(
            if checked {
                tokens.primary
            } else {
                tokens.secondary
            },
            0.5,
        )
    } else if checked {
        tokens.primary
    } else {
        theme::darken(tokens.secondary, 0.1)
    }
}

/// The track, with the thumb at the end it belongs to.
fn track<'a, Message>(checked: bool, size: ControlSize, disabled: bool) -> Element<'a, Message>
where
    Message: 'a,
{
    let (width, height, thumb) = metrics(size);

    let mark = container(Space::new().width(thumb).height(thumb)).style(move |theme: &Theme| {
        container::Style {
            background: Some(Tokens::of(theme).background.into()),
            border: iced::border::rounded(thumb / 2.0),
            ..container::Style::default()
        }
    });

    container(mark)
        .width(width)
        .height(height)
        .padding(2)
        .align_x(if checked {
            Alignment::End
        } else {
            Alignment::Start
        })
        .align_y(Alignment::Center)
        .style(move |theme: &Theme| container::Style {
            background: Some(track_fill(Tokens::of(theme), checked, disabled).into()),
            border: iced::border::rounded(height / 2.0),
            ..container::Style::default()
        })
        .into()
}

/// The state a switch draws with, and the chrome of the handle it sits in.
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

    /// The handle's own chrome. The track is content, so the chrome is
    /// transparent in every state; it is the extension point an application can
    /// give the whole row a surface through.
    pub fn resolve(&self, tokens: Tokens, _status: iced_button::Status) -> iced_button::Style {
        let (_, height, _) = metrics(self.size);
        iced_button::Style {
            background: None,
            text_color: label_ink(tokens, self.disabled),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: (height / 2.0).into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }
}

impl<'a, Message> From<Switch<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Switch<'a, Message>) -> Self {
        let Switch {
            checked,
            label,
            size,
            on_toggle,
        } = control;
        let disabled = on_toggle.is_none();
        let style = Style::new(checked, size).disabled(disabled);

        let mut content = row![track(checked, size, disabled)]
            .spacing(theme::SPACING_SM)
            .align_y(Alignment::Center);

        if !label.is_empty() {
            content = content.push(text(label).size(label_size(size)).style(
                move |theme: &Theme| text::Style {
                    color: Some(label_ink(Tokens::of(theme), disabled)),
                },
            ));
        }

        // The value the switch wants to become, not the one it is.
        let on_press = on_toggle.map(|on_toggle| on_toggle(!checked));

        handle(content, on_press, move |theme: &Theme, status| {
            style.resolve(Tokens::of(theme), status)
        })
    }
}
