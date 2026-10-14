//! One frame around a field and its addons. Mirrors GPUI Component's
//! `InputGroup` and the Slint version beside it: the frame draws the fill, the
//! edge, the invalid edge and the radius, the field inside drops its own frame,
//! and addons sit before or after it on the same track.
//!
//! This is composition rather than a second field. The group holds an ordinary
//! [`Input`](super::input::Input) and draws its frame with the field's own
//! [`Style`](super::input::Style), so the fill, the edge, the radius and the
//! disabled ink are the ones a standalone field would use; the addons are
//! caller-built elements — a [`Button`](super::button::Button), a glyph, some
//! text — so the group adds only the track and the addon padding. `prefix` and
//! `suffix` stay the field's own slots, inside the frame; an
//! [`InputGroupAddon`] is outside the field and on either side of it.
//!
//! **Four things the references do that iced 0.14 cannot, and none is faked:**
//!
//! - **The frame never learns that the field is focused.** The same limit the
//!   single-line [`Input`](super::input::Input) documents: a container resolves
//!   its style from the theme alone, and the `text_input` inside it is the only
//!   thing that knows the focus state, so the group keeps its resting edge and
//!   the caret and selection carry focus instead.
//! - **Disabled reaches the field and not the addons.** A [`Button`] is disabled
//!   by having no message, and a control the caller already built cannot have
//!   its message taken away by the group. The group's `disabled` style and the
//!   field's refusal are real; an addon button stays live, and the caller builds
//!   it with no message when the whole row is disabled.
//! - **An addon does not focus the field.** Clicking text, an icon or empty
//!   space in an addon focuses the field in GPUI because the group owns the
//!   press; iced reports the press to the element that was pressed, so an addon
//!   that should focus the field has to be the field's own `prefix` slot or a
//!   control that asks for focus itself.
//! - **`BlockStart` and `BlockEnd` are absent.** Slint's group is one horizontal
//!   row and so is this one, so its addons are inline-start and inline-end only.
//!   The GPUI part's two block sides belong to a column frame the Slint
//!   authority does not have; a toolbar above or below the field is the caller's
//!   own column, which is what the Slint textarea section does.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one state
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::container::Style as ContainerStyle;
use iced::widget::{Row, container, row};
use iced::{Alignment, Element, Length, Theme};

use super::input::{self, Input};
use super::theme::{self, ControlSize, Tokens};

/// A shared frame around one field and any number of addons, built with the
/// methods below.
///
/// The caller owns the field: configure an [`Input`](super::input::Input) with
/// its value, placeholder, `prefix`, `suffix` and `on_input`, hand it to
/// [`input`](Self::input), and add caller-built elements with
/// [`addon_start`](Self::addon_start) and [`addon_end`](Self::addon_end). The
/// group draws the control's frame and the field drops its own, so the two read
/// as one control.
pub struct InputGroup<'a, Message> {
    input: Option<Input<'a, Message>>,
    start: Vec<InputGroupAddon<'a, Message>>,
    end: Vec<InputGroupAddon<'a, Message>>,
    size: ControlSize,
    disabled: bool,
    invalid: bool,
    width: Length,
}

impl<'a, Message> InputGroup<'a, Message> {
    /// An empty group. Give it a field with [`input`](Self::input) and the
    /// content around it with the addon builders; a group with neither is just
    /// the frame.
    pub fn new() -> Self {
        Self {
            input: None,
            start: Vec::new(),
            end: Vec::new(),
            size: ControlSize::default(),
            disabled: false,
            invalid: false,
            width: Length::Fill,
        }
    }

    /// The field the group frames, replacing the previous one. The field's own
    /// `appearance` is forced off, because the group is what draws the frame.
    pub fn input(mut self, input: Input<'a, Message>) -> Self {
        self.input = Some(input);
        self
    }

    /// An addon before the field, drawn in the order added.
    pub fn addon_start(mut self, addon: InputGroupAddon<'a, Message>) -> Self {
        self.start.push(addon);
        self
    }

    /// An addon after the field, drawn in the order added.
    pub fn addon_end(mut self, addon: InputGroupAddon<'a, Message>) -> Self {
        self.end.push(addon);
        self
    }

    /// The size ladder. It sets the frame's height, the field's size and the
    /// addons' inset together, the way Slint's `ControlSize` does.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Disables the field and draws the frame in the muted fill. An addon the
    /// caller built keeps its own state — see the module docs.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Reports the value as rejected: the frame's edge takes the danger colour,
    /// the way a standalone field does.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// The frame's width. `Fill` by default, the way a field fills its row.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<Message> Default for InputGroup<'_, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// Content on one side of a group: a padded row, centred on the cross axis, for
/// text, a glyph, a [`Button`](super::button::Button) or any other element.
///
/// The addon's ink is the muted foreground, inherited by text that does not set
/// a colour of its own; a glyph is still the caller's to colour, because an SVG
/// keeps its own black unless its style closure says otherwise.
pub struct InputGroupAddon<'a, Message> {
    children: Vec<Element<'a, Message>>,
    divider: bool,
}

impl<'a, Message: 'a> InputGroupAddon<'a, Message> {
    /// An empty addon.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            divider: false,
        }
    }

    /// Append a child. Children appear in the order added.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Append children in order.
    pub fn children(mut self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        self.children.extend(children);
        self
    }

    /// Draws a one-pixel rule on the addon's field-facing edge, so the field and
    /// the addon read as one control with a visible seam rather than as two
    /// controls. Off by default, as in the Slint group, which draws no rule.
    pub fn divider(mut self, divider: bool) -> Self {
        self.divider = divider;
        self
    }

    /// The padded, muted row the group places beside the field.
    fn render(self, disabled: bool) -> Element<'a, Message> {
        container(
            Row::with_children(self.children)
                .spacing(4.0)
                .align_y(Alignment::Center),
        )
        .padding([0.0, 8.0])
        .height(Length::Fill)
        .center_y(Length::Fill)
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            ContainerStyle {
                // A disabled addon takes half the muted ink: iced has no subtree
                // opacity, and the field beside it is already muted, so a
                // full-strength addon would read as the live half of the row.
                text_color: Some(if disabled {
                    theme::alpha(tokens.muted_foreground, 0.5)
                } else {
                    tokens.muted_foreground
                }),
                ..ContainerStyle::default()
            }
        })
        .into()
    }
}

impl<'a, Message: 'a> Default for InputGroupAddon<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The one-pixel rule an addon's `divider` draws, from the same edge token the
/// frame uses so the seam and the frame agree in either theme.
fn addon_divider<'a, Message: 'a>() -> Element<'a, Message> {
    container(row![])
        .width(Length::Fixed(1.0))
        .height(Length::Fill)
        .style(|theme: &Theme| {
            let tokens = Tokens::of(theme);
            ContainerStyle {
                background: Some(tokens.input.into()),
                ..ContainerStyle::default()
            }
        })
        .into()
}

impl<'a, Message> From<InputGroup<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(group: InputGroup<'a, Message>) -> Self {
        let InputGroup {
            input,
            start,
            end,
            size,
            disabled,
            invalid,
            width,
        } = group;
        // The field's height, not the button's: the group has to hold the field
        // it frames, and Slint's `control-height` is two pixels shorter than
        // `input-height` at the large size.
        let height = size.input_height();

        // The group's frame is the field's own style: the same fill, edge,
        // radius and disabled ink. It is resolved **inside** the closure below,
        // so the theme in force when the frame is drawn decides the colours
        // rather than the theme in force when the element was built.
        let style = input::Style::new().invalid(invalid).disabled(disabled);

        let mut children: Vec<Element<'a, Message>> = Vec::new();
        for addon in start {
            let divider = addon.divider;
            children.push(addon.render(disabled));
            if divider {
                children.push(addon_divider());
            }
        }
        if let Some(field) = input {
            children.push(
                field
                    .appearance(false)
                    .size(size)
                    .disabled(disabled)
                    .width(Length::Fill)
                    .into(),
            );
        }
        for addon in end {
            let divider = addon.divider;
            if divider {
                children.push(addon_divider());
            }
            children.push(addon.render(disabled));
        }

        container(
            Row::with_children(children)
                .spacing(0.0)
                .align_y(Alignment::Center)
                .height(Length::Fill),
        )
        // Slint's two-pixel inset, so the field's own padding is not flush
        // against the frame's edge.
        .padding([0.0, 2.0])
        .height(Length::Fixed(height))
        .width(width)
        .style(move |theme: &Theme| style.resolve(Tokens::of(theme)))
        .into()
    }
}
