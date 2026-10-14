//! The framed region a modal surface puts its controls in, and the place where
//! the region's behaviour would live. Mirrors GPUI Component's `FocusTrap`
//! (`FocusTrapContainer` / `FocusTrapElement`, with `FocusTrapManager`
//! installed by `focus_trap::init`) and the Slint `focus-trap.slint` beside it.
//!
//! # What it draws
//!
//! The Slint file is the authority, and its `FocusTrap` is a `FocusScope` around
//! a `Rectangle` with a one-pixel `Theme.border` edge, `radius-lg` corners and a
//! `VerticalLayout` inside it: 16px of inset and a 12px gap between the
//! children. This port draws exactly that region — [`FocusTrap::child`] stacks
//! its children with that gap inside that inset — so a caller wrapping the
//! controls of a custom modal gets the reference's frame, and can leave the fill
//! to the page ([`Style::resolve`] draws no fill, as the Slint `Rectangle` does)
//! or turn the region into a panel with [`FocusTrap::style`].
//!
//! The reference gives the region no margin or placement of its own, and neither
//! does this: a custom modal is a full-size layer in a
//! [`Stack`](iced::widget::Stack) — the mechanism [`dialog`](super::dialog)
//! documents — with the backdrop and the centering the caller writes, and this
//! component is the panel inside it. The gallery's "Custom Modal with Focus
//! Trap" section is that shape.
//!
//! # What iced 0.14 has no way to express
//!
//! The name says what this file would be in GPUI, and the honest thing is to say
//! that the trapped half is missing rather than draw a flag that does nothing:
//!
//! - **No focus scope, and no Tab at all.** GPUI's `FocusTrapContainer` registers
//!   a container with a global manager and `Root` intercepts Tab/Shift-Tab,
//!   cycling within the container. iced has a `focusable` *operation*
//!   (`operation::focusable::focus_next`) but no focus handle a widget can hold,
//!   no way to scope an operation to a subtree, and no runtime key that moves
//!   focus: nothing in iced 0.14 names `Tab` except `combo_box`, which is why
//!   its list closes on one. So a trap cannot be written here, and the
//!   application cannot write one either over this region: a `Stack` layer is
//!   still an ordinary widget, and Tab reaches the page behind it. Focus is not
//!   moved into the region and is not returned to the trigger when it closes.
//! - **No `enabled` flag.** Slint's `enabled` gates the `FocusScope`; there is no
//!   scope to gate. A property that drew the same region either way would be
//!   pretending.
//! - **No `escaped` callback.** The Slint `FocusScope` catches Escape and calls
//!   it. An in-flow widget is handed a key only by a focused control, and iced's
//!   field consumes the keys it handles without bubbling them out, so the
//!   component would never hear the key. Escape stays the application's
//!   subscription, exactly as [`dialog`](super::dialog) and `date_picker` say
//!   for the keys their layers cannot take.
//!
//! The result is the frame without the trap. Where a caller needs containment
//! today, iced's own `text_input` is the only focusable that cycles at all, and a
//! single-field surface is the one case Tab cannot leave; the component page
//! names the gap rather than showing a sample that would mislead.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{column, container};
use iced::{Border, Color, Element, Length, Shadow, Theme};

use super::theme::{self, Tokens};

/// The region's inset, the Slint `VerticalLayout`'s 16px padding.
const PADDING: f32 = 16.0;

/// The gap between the region's children, the Slint `VerticalLayout`'s 12px
/// spacing.
const SPACING: f32 = 12.0;

/// The region a focus trap would guard, built with the methods below.
///
/// It becomes an [`Element`] through `From`, and what it becomes is the framed
/// column: every child stacked at the reference's 12px gap, inside its 16px
/// inset and one-pixel `border` edge.
pub struct FocusTrap<'a, Message> {
    children: Vec<Element<'a, Message>>,
    style: Box<dyn Fn(&Theme) -> Style + 'a>,
}

impl<'a, Message: 'a> FocusTrap<'a, Message> {
    /// An empty region with the reference's frame.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: Box::new(|theme: &Theme| Style::resolve(Tokens::of(theme))),
        }
    }

    /// Add one child, in reading order. The region stacks them itself, so a
    /// caller adds the field, the field and the action the way the Slint
    /// example's `@children` does.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// The region's surface and edge. `Style::resolve` is the default, so a
    /// caller overrides only what it means to.
    ///
    /// It is a function of the theme rather than a value because the appearance
    /// in force is not known where the region is built; a surface resolved for
    /// one appearance is the wrong colour in the other. This crate's
    /// `popover.rs` and `dialog.rs` say the same thing about their own panels.
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self {
        self.style = Box::new(style);
        self
    }

    /// The region, ready to place — over a page, or centered on a modal layer.
    fn into_element(self) -> Element<'a, Message> {
        let Self { children, style } = self;

        let body = column(children).spacing(SPACING).width(Length::Fill);

        container(body)
            .width(Length::Fill)
            .padding(PADDING)
            .style(move |theme: &Theme| style(theme).container_style())
            .into()
    }
}

impl<'a, Message: 'a> Default for FocusTrap<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: 'a> From<FocusTrap<'a, Message>> for Element<'a, Message> {
    fn from(trap: FocusTrap<'a, Message>) -> Self {
        trap.into_element()
    }
}

/// The region's surface and edge.
///
/// [`Style::resolve`] builds one from the theme tokens — a transparent fill, the
/// `border` token as a one-pixel edge and `radius-lg`, which is what the Slint
/// `Rectangle` sets. The fields are private and read through methods, so this
/// can grow without breaking a caller.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    fill: Option<Color>,
    border: Color,
    border_width: f32,
    radius: f32,
    shadow: Shadow,
}

impl Style {
    /// The frame the tokens describe: no fill, the `border` token as a one-pixel
    /// edge, `radius-lg` corners and no shadow.
    ///
    /// The edge is the `border` token and not a fixed grey: the token is
    /// `#e5e5e5` on white and `#262626` on near-black, so the edge exists in
    /// both appearances.
    pub fn resolve(tokens: Tokens) -> Self {
        Self {
            fill: None,
            border: tokens.border,
            border_width: 1.0,
            radius: theme::RADIUS_LG,
            shadow: Shadow::default(),
        }
    }

    /// The region's fill. Without it the region draws none, as Slint's
    /// `Rectangle` does; a caller turning the region into a panel passes the
    /// surface token, `popover` for instance.
    pub fn fill(mut self, fill: Color) -> Self {
        self.fill = Some(fill);
        self
    }

    /// The region's edge. A `width` of 0 draws no edge.
    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border = color;
        self.border_width = width;
        self
    }

    /// The corner radius — `radius-lg` (8px) by default.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// The drop shadow. `Shadow::default()` draws none, which is the reference.
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = shadow;
        self
    }

    /// The region's surface as a container's styling.
    pub fn container_style(&self) -> container::Style {
        container::Style {
            background: self.fill.map(Into::into),
            border: Border {
                color: self.border,
                width: self.border_width,
                radius: self.radius.into(),
            },
            shadow: self.shadow,
            ..container::Style::default()
        }
    }
}

impl Default for Style {
    /// The light theme's resting frame. A caller that wants the appearance in
    /// force builds it with [`Style::resolve`] and [`Tokens::of`].
    fn default() -> Self {
        Self::resolve(Tokens::of(&Theme::Light))
    }
}
