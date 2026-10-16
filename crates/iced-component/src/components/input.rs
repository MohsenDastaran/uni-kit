//! A single-line text field. Mirrors GPUI Component's `Input` and the Slint
//! version beside it: a placeholder, a prefix and a suffix slot, a masked value,
//! the invalid and disabled states and the four sizes, with the same fills and
//! edges.
//!
//! iced's `text_input` carries no adornments of its own, so this component is the
//! frame — a `container` with the fill, the edge and the radius — around a `row`
//! that holds the caller's prefix, the field and the caller's suffix. The field
//! inside is drawn with no fill and no edge, so the frame is the whole control.
//! It also takes the field's text as a `String` the caller owns and reports every
//! edit through `on_input`; iced has no input state to hold it.
//!
//! **Four Slint behaviours have no iced 0.14 counterpart, and none is faked:**
//!
//! - **The frame shows no focus ring and no hover edge.** A container resolves its
//!   style from the theme alone; the field's own status is known only to the
//!   `text_input` inside it, so the port cannot move the focused edge onto the
//!   frame that carries the prefix and suffix. The caret and the selection show
//!   focus instead, and the frame keeps its resting edge.
//! - **`cleanable` and `clean-on-escape` are the application's.** iced's
//!   `text_input` reports text and submit only and handles no key of its own
//!   outside editing, so there is nowhere for an Escape action to live. A clear
//!   control is an ordinary suffix — the gallery builds one — and Escape belongs
//!   to the application's key subscriptions.
//! - **`readonly` is absent.** iced's field has no read-only mode: it either
//!   accepts edits or it is disabled. A disabled field is the closest state, but
//!   it is not selectable and a read-only field is.
//! - **`has-focus` and the inline token support are absent.** iced reports
//!   neither focus nor the keys the reveal toggle needs, and its value is a plain
//!   `String`, so a reference cannot live inside it as a token. `secure` covers
//!   the mask; the reveal toggle is a suffix the caller drives, and the gallery
//!   shows both.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one state
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::container::Style as ContainerStyle;
use iced::widget::{Id, container, row, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Pixels, Theme};

use super::theme::{self, ControlSize, Tokens};

/// A text field, built with the methods below.
///
/// The `id` is what iced keeps the field's editing state under — its caret, its
/// selection and its focus. It has to be stable across renders, so it is the
/// field's name rather than a buffer, and a name that changes with the value
/// would lose the caret on every keystroke.
pub struct Input<'a, Message> {
    id: Id,
    value: String,
    placeholder: String,
    size: ControlSize,
    disabled: bool,
    invalid: bool,
    appearance: bool,
    secure: bool,
    align_x: Alignment,
    prefix: Option<Element<'a, Message>>,
    suffix: Option<Element<'a, Message>>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    width: Length,
}

impl<'a, Message> Input<'a, Message> {
    /// A field iced keeps its state under, named `id` — `Input::new("email")`.
    pub fn new(id: impl Into<Id>) -> Self {
        Self {
            id: id.into(),
            value: String::new(),
            placeholder: String::new(),
            size: ControlSize::default(),
            disabled: false,
            invalid: false,
            appearance: true,
            secure: false,
            align_x: Alignment::Start,
            prefix: None,
            suffix: None,
            on_input: None,
            on_submit: None,
            width: Length::Fill,
        }
    }

    /// The text the field shows. The caller owns it: it sets this from its own
    /// state and takes the edits back through [`on_input`](Self::on_input).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Refuses input and draws muted, the way Slint's `enabled: false` does. A
    /// field with no [`on_input`](Self::on_input) is disabled without this.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Reports the value as rejected: the edge takes the danger colour.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Draws the frame. `false` drops the fill, the edge and the radius so the
    /// field can sit inside a container that draws its own, which is what Slint's
    /// `appearance: false` is for.
    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }

    /// Masks the value, the way Slint's `masked` does. iced has no reveal toggle
    /// of its own, so one is a suffix the caller drives — see the gallery.
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Where the text sits in the field. `End` is the right-aligned currency
    /// field the Slint example shows.
    pub fn align_x(mut self, align_x: Alignment) -> Self {
        self.align_x = align_x;
        self
    }

    /// A leading element, drawn inside the frame before the text. Any element:
    /// the gallery puts a glyph there, the Slint example a `$`.
    pub fn prefix(mut self, prefix: impl Into<Element<'a, Message>>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// A trailing element, drawn inside the frame after the text — the clear
    /// control, the reveal toggle, a unit.
    pub fn suffix(mut self, suffix: impl Into<Element<'a, Message>>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Every edit, as the value the field now holds.
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// The message Enter sends.
    pub fn on_submit(mut self, on_submit: Message) -> Self {
        self.on_submit = Some(on_submit);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// The height Slint's `input-height` gives a size: 20 / 24 / 32 / 44.
    fn height(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 20.0,
            ControlSize::Small => 24.0,
            ControlSize::Large => 44.0,
            ControlSize::Medium => 32.0,
        }
    }

    /// The gap between the slots: `spacing-xs` at the two small sizes and
    /// `spacing-sm` above them, as the Slint layout sets it.
    fn gap(&self) -> f32 {
        match self.size {
            ControlSize::XSmall | ControlSize::Small => theme::SPACING_XS,
            _ => theme::SPACING_SM,
        }
    }

    /// A field with no way to report an edit is disabled, the way a button with
    /// no message is.
    fn is_disabled(&self) -> bool {
        self.disabled || self.on_input.is_none()
    }

    fn style(&self) -> Style {
        Style::new()
            .invalid(self.invalid)
            .disabled(self.is_disabled())
            .appearance(self.appearance)
    }
}

/// The colours a field draws its frame with. Components pass it to the
/// container's `style`; it is `pub` so an application can draw a matching frame
/// of its own around a borderless field.
pub struct Style {
    pub invalid: bool,
    pub disabled: bool,
    pub appearance: bool,
}

impl Style {
    pub fn new() -> Self {
        Self {
            invalid: false,
            disabled: false,
            appearance: true,
        }
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }

    /// The fill: the page in the light theme, the edge at a third of its weight
    /// over it in the dark one, where a field that filled with the page would be
    /// a hole. A disabled field takes the muted surface instead.
    fn fill(&self, tokens: Tokens) -> Color {
        if !self.appearance {
            Color::TRANSPARENT
        } else if self.disabled {
            tokens.muted
        } else {
            tokens.input_background
        }
    }

    /// The edge: the danger colour while the value is rejected, the input token
    /// otherwise.
    fn edge(&self, tokens: Tokens) -> Color {
        if !self.appearance {
            Color::TRANSPARENT
        } else if self.invalid {
            tokens.danger
        } else {
            tokens.input
        }
    }

    pub fn resolve(&self, tokens: Tokens) -> ContainerStyle {
        ContainerStyle {
            background: Some(self.fill(tokens).into()),
            border: Border {
                color: self.edge(tokens),
                width: if self.appearance { 1.0 } else { 0.0 },
                radius: if self.appearance {
                    theme::RADIUS_MD
                } else {
                    0.0
                }
                .into(),
            },
            text_color: Some(if self.disabled {
                tokens.muted_foreground
            } else {
                tokens.foreground
            }),
            ..ContainerStyle::default()
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Input<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Input<'a, Message>) -> Self {
        let height = control.height();
        let inset = control.size.input_padding_x();
        let gap = control.gap();
        let font_size = control.size.input_font_size();
        let width = control.width;
        let secure = control.secure;
        let align_x = control.align_x;
        let disabled = control.is_disabled();
        let style = control.style();

        let mut field = text_input(&control.placeholder, &control.value)
            .id(control.id.clone())
            // The frame owns the padding and the edge, so the field draws only
            // text and selection inside them.
            .padding(Padding::ZERO)
            .size(font_size)
            // A line box as tall as the control: iced sizes a text input from its
            // line height, and the default would leave the click target one line
            // tall in the middle of the frame. The unit matters — a bare `f32` is
            // a *multiple* of the font size, so `30.0` is a thirty-line box and
            // the text is drawn outside the field. `Pixels` is the absolute
            // height, which is what the size ladder means. It is deliberately
            // taller than the frame's padded inside, so iced clamps it to that
            // inside and the caret comes out inset by the frame's padding instead
            // of running the frame's full height.
            .line_height(Pixels(height - 2.0))
            .secure(secure)
            .align_x(align_x)
            .style(move |theme: &Theme, status| {
                let tokens = Tokens::of(theme);
                let dimmed = disabled || matches!(status, text_input::Status::Disabled);

                text_input::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    border: Border::default(),
                    icon: tokens.muted_foreground,
                    placeholder: tokens.muted_foreground,
                    value: if dimmed {
                        tokens.muted_foreground
                    } else {
                        tokens.foreground
                    },
                    selection: theme::alpha(tokens.selection, 0.35),
                }
            });

        // A disabled field carries no message: iced reads a missing `on_input` as
        // the disabled state, and one it cannot report would be a field that
        // silently drops the edit.
        if !disabled {
            if let Some(on_input) = control.on_input {
                field = field.on_input(on_input);
            }
            if let Some(on_submit) = control.on_submit {
                field = field.on_submit(on_submit);
            }
        }

        let mut contents = row![]
            .spacing(gap)
            .align_y(Alignment::Center)
            .height(Length::Fill);

        if let Some(prefix) = control.prefix {
            contents = contents.push(prefix);
        }
        contents = contents.push(field);
        if let Some(suffix) = control.suffix {
            contents = contents.push(suffix);
        }

        container(contents)
            // The padding is all four sides, `Theme.input-padding-x(size)`, the
            // same inset `textarea.slint` gives its frame — the horizontal inset
            // and the vertical inset are one value in the family's authority.
            // The field's line box below is a line taller than the inside, so
            // iced clamps it to the padded content box: the caret is inset from
            // the frame's edge by this padding instead of running its full
            // height, and the row is still `Fill` and centred, so the text and
            // the placeholder do not move. The outer height is `Fixed` and does
            // not change.
            .padding(inset)
            .height(Length::Fixed(height))
            .width(width)
            .style(move |theme: &Theme| style.resolve(Tokens::of(theme)))
            .into()
    }
}
