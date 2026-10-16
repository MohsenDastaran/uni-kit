//! The framed region a modal surface puts its controls in, and the trap that
//! keeps `Tab` inside it. Mirrors GPUI Component's `FocusTrap`
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
//! # The trap
//!
//! iced 0.14 binds no key to focus movement: nothing but `combo_box` names
//! `Tab`, and there is no focus scope a container can hold. What it does have
//! is a `focusable` *operation* — `focusable::find_focused`, `focus` and the
//! rest — run from a [`Task`], and a widget `update` that is handed **every**
//! runtime event, not only the keys a focused control passes on: the runtime
//! walks the whole tree for each event, and `text_input` gates on its own
//! `is_focused` rather than on delivery. [`tab`] uses both — it reads the
//! focused id, steps through the region that holds it, and focuses the result,
//! all in one `Task`. The application's whole part is routing `Tab`/`Shift+Tab`
//! to it from a subscription, because iced leaves those keys unbound.
//!
//! [`FocusTrap::focusable`] names the region's members in reading order, and
//! [`tab`] wraps at the ends, so focus cannot walk out of the region. Two
//! regions are passed together and the one holding focus wins, which keeps side
//! by side traps independent. [`enter`] is the move the region makes when it
//! opens; [`focused`] reports what to return focus to when it closes.
//!
//! What is still missing, and named rather than faked:
//!
//! - **No focus ring.** iced 0.14 draws none, so a move is proved by typing:
//!   the text lands in the field that took focus.
//! - **No `escaped` callback.** A widget's `update` cannot publish a focus
//!   operation, only messages and redraw requests, so Escape stays the
//!   application's subscription, exactly as [`dialog`](super::dialog) and
//!   `date_picker` say for the keys their layers cannot take.
//! - **No `enabled` flag.** Slint's `enabled` gates the `FocusScope`; here the
//!   application chooses whether to route `Tab` at all, and a property that drew
//!   the same region either way would be pretending.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::advanced::widget::operation::{Focusable, Operation, Outcome};
use iced::widget::{Id, column, container, operation};
use iced::{Border, Color, Element, Length, Rectangle, Shadow, Task, Theme};

use super::theme::{self, Tokens};

/// The region's inset, the Slint `VerticalLayout`'s 16px padding.
const PADDING: f32 = 16.0;

/// The gap between the region's children, the Slint `VerticalLayout`'s 12px
/// spacing.
const SPACING: f32 = 12.0;

/// The region a focus trap guards, built with the methods below.
///
/// It becomes an [`Element`] through `From`, and what it becomes is the framed
/// column: every child stacked at the reference's 12px gap, inside its 16px
/// inset and one-pixel `border` edge.
pub struct FocusTrap<'a, Message> {
    children: Vec<Element<'a, Message>>,
    focusables: Vec<Id>,
    style: Box<dyn Fn(&Theme) -> Style + 'a>,
}

impl<'a, Message: 'a> FocusTrap<'a, Message> {
    /// An empty region with the reference's frame.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            focusables: Vec::new(),
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

    /// Name one focusable child, in reading order. [`tab`] walks these ids and
    /// wraps at the end, so the region's membership is declared beside its
    /// children. A child whose `Id` is not named here is drawn but is not part
    /// of the trap, and a button is not focusable in iced 0.14 at all.
    pub fn focusable(mut self, id: impl Into<Id>) -> Self {
        self.focusables.push(id.into());
        self
    }

    /// The focusables this region traps, in the order
    /// [`focusable`](Self::focusable) named them.
    pub fn focusables(&self) -> &[Id] {
        &self.focusables
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
        let Self {
            children, style, ..
        } = self;

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

/// The id of the focusable that currently holds focus, or `None`.
///
/// iced's own `find_focused` finishes with nothing when no widget is focused,
/// which a caller cannot tell apart from a query that never ran; this operation
/// always reports, so a trap can open on a page where nothing was focused and
/// still know there is nothing to return to.
pub fn focused() -> Task<Option<Id>> {
    iced::advanced::widget::operate(FindFocused { focused: None })
}

/// The [`focused`] query: reports the focused id even when it is `None`.
struct FindFocused {
    focused: Option<Id>,
}

impl Operation<Option<Id>> for FindFocused {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<Option<Id>>)) {
        operate(self);
    }

    fn focusable(
        &mut self,
        id: Option<&Id>,
        _bounds: Rectangle,
        state: &mut dyn Focusable,
    ) {
        if state.is_focused() {
            if let Some(id) = id {
                self.focused = Some(id.clone());
            }
        }
    }

    fn finish(&self) -> Outcome<Option<Id>> {
        Outcome::Some(self.focused.clone())
    }
}

/// The id `Tab` moves to inside one region, or `None` for an empty region.
///
/// `focused` is the id that currently holds focus. An id that is not in
/// `focusables` enters the region at its first focusable — or its last, when
/// `backwards` is `Shift+Tab` — and at either end the walk wraps. That wrap is
/// the whole of the containment.
pub fn step(focusables: &[Id], focused: Option<&Id>, backwards: bool) -> Option<Id> {
    if focusables.is_empty() {
        return None;
    }

    let index = focused
        .and_then(|focused| focusables.iter().position(|id| id == focused));

    let next = match index {
        None if backwards => focusables.len() - 1,
        None => 0,
        Some(0) if backwards => focusables.len() - 1,
        Some(index) if backwards => index - 1,
        Some(index) if index + 1 == focusables.len() => 0,
        Some(index) => index + 1,
    };

    Some(focusables[next].clone())
}

/// The whole `Tab` handler: read the focused id from the widget tree, step
/// through the region that holds it, and focus the result.
///
/// The read and the move are one widget operation, so the application keeps no
/// focus state of its own and needs no message round trip — it returns this
/// from `update` when `Tab` (or `Shift+Tab`) arrives. `regions` is every active
/// trap in reading order: the region holding the focused id wins, so two traps
/// side by side stay independent, and a focused id in no region enters the
/// first region. One region is the ordinary case.
///
/// iced 0.14 binds no key to focus movement, so the key itself is the
/// application's subscription; what lives here is the trap.
pub fn tab<Message: Send + 'static>(regions: &[&[Id]], backwards: bool) -> Task<Message> {
    iced::advanced::widget::operate(Tab {
        regions: regions.iter().map(|region| region.to_vec()).collect(),
        backwards,
        focused: None,
    })
    .discard()
}

/// The [`tab`] operation: collects the focused id, then chains the focus move
/// that [`step`] chooses. It reports no output of its own, which is why `Tab`
/// is one tree walk rather than a query and a second round trip.
struct Tab {
    regions: Vec<Vec<Id>>,
    backwards: bool,
    focused: Option<Id>,
}

impl Operation<()> for Tab {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<()>)) {
        operate(self);
    }

    fn focusable(
        &mut self,
        id: Option<&Id>,
        _bounds: Rectangle,
        state: &mut dyn Focusable,
    ) {
        if state.is_focused() {
            if let Some(id) = id {
                self.focused = Some(id.clone());
            }
        }
    }

    fn finish(&self) -> Outcome<()> {
        let region = self
            .regions
            .iter()
            .find(|region| self.focused.as_ref().is_some_and(|id| region.contains(id)))
            .or_else(|| self.regions.first());

        match region.and_then(|region| step(region, self.focused.as_ref(), self.backwards)) {
            Some(id) => Outcome::Chain(Box::new(
                iced::advanced::widget::operation::focusable::focus(id),
            )),
            None => Outcome::None,
        }
    }
}

/// Focus the first focusable of `focusables` — the move a trap makes when its
/// region opens, so the next `Tab` is already inside it.
pub fn enter<Message: 'static>(focusables: &[Id]) -> Task<Message> {
    match focusables.first() {
        Some(id) => operation::focus(id.clone()),
        None => Task::none(),
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
