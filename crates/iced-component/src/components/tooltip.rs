//! A delayed hover label. Mirrors GPUI Component's `Tooltip` and the Slint
//! version beside it: a small surface in the foreground colour, with the ink on
//! it in the page's own colour, shown over a trigger once the pointer has rested
//! on it, on the side of the trigger the caller prefers.
//!
//! # How it is built, and why not on iced's own `tooltip`
//!
//! iced ships a tooltip, and the timing in this file is that widget's: the same
//! three-state machine, the same `shell.request_redraw_at(now + delay)` to wake
//! the frame the delay ends on, and the same `cursor.is_over(layout.bounds())`
//! test. What this port does not take from it is the surface. iced's tooltip
//! draws whatever [`container::Style`](iced::widget::container::Style) the
//! theme's own catalog resolves and clamps the result into the window; the Slint
//! component this catalogue exposes draws its *own* surface — a fill of
//! `Theme.foreground` with the text in `Theme.background`, `radius-md`, no
//! border, an 8px shadow — and it says that a side with no room **flips** to the
//! opposite one. Neither is an argument to iced's `Tooltip`.
//!
//! What this port does take from the popover is the mechanism and only the
//! mechanism: [`Placed::resolve`](super::popover::Placed::resolve) is the
//! popover's own flip-and-clamp rule, [`Arrow::draw`](super::popover::Arrow::draw)
//! is its own triangle, [`PanelStyle`](super::popover::PanelStyle) is its own
//! surface, and the panel is an [`overlay::Element`] exactly as the popover's
//! is — built once for the frame's events and once for the paint, laid out
//! against the window and drawn after the whole tree. That is what keeps a
//! tooltip alive in a [`scrollable`](super::scrollable): no scroll area between
//! the trigger and the window can clip an overlay. A tooltip drawn inside the
//! layout that owns its trigger is a tooltip that disappears at the edge of the
//! list it describes.
//!
//! **Why not the popover's own overlay, which already does all of that.** A
//! [`Popover`](super::popover::Popover) owns its trigger and lends it to the
//! overlay it returns, for as long as that overlay lives; a tooltip also has to
//! lay its trigger out, draw it and update it in the same frame. One element
//! cannot be in two owners, and an iced [`Element`] is not `Clone`, so the
//! tooltip keeps its trigger and borrows the *rules* instead. The cost is one
//! small overlay type in this file; the benefit is that the placement maths and
//! the surface are still written once, in the popover.
//!
//! # The delay
//!
//! Slint's `Tooltip` is a `Timer` of `delay` (400ms) started by `has-hover`,
//! cancelled when the pointer leaves, and it will not open on an empty `text`.
//! The port keeps all three: the timer's two states, the cancellation, and the
//! empty-label refusal. iced has no timer to start, so the deadline is a
//! `RedrawRequest::At`: the frame the delay ends on re-enters
//! [`update`](Widget::update), sees the deadline has passed, and opens. That is
//! `iced_widget::tooltip`'s mechanism line for line, used for the same reason —
//! it is the only one iced 0.14 offers that does not need the `time` runtime
//! feature this crate does not enable.
//!
//! # What Slint has that this does not
//!
//! - **A property-driven `text`.** Slint sets `text` on the `Tooltip` and the
//!   popup reads it. Here the label is an element ([`Tooltip::label`]), because
//!   an iced element is built where its state is and a `String` property would
//!   be a second place for the same state to live. A tooltip with no label never
//!   opens, which is Slint's `text != ""`.
//! - **`close-policy: no-auto-close`.** Slint's popup takes no input at all. An
//!   iced overlay is offered every event before the widgets under it, so the
//!   label here is text and nothing else: the panel holds no control that could
//!   act on a press. See [`popover`](super::popover) for what an interactive
//!   surface does.
//! - **Focus.** GPUI's tooltip system opens on focus as well as hover, and its
//!   page says so. iced 0.14 has no focus handle a widget can take or observe,
//!   so this opens on hover only — stated rather than pretended.
//! - **A shared `Placement`.** [`Side`] is this component's own enum, because a
//!   tooltip's default side is the *opposite* of a popover's: Slint's tooltip
//!   defaults to `Placement.top` while the panel primitive defaults to bottom.
//!   Keeping the types apart keeps both defaults honest.
//!
//! # What this has that Slint does not
//!
//! - **An arrow.** [`Tooltip::arrow`] draws a triangle between the surface and
//!   the trigger, at the start, the centre or the end of the edge they share.
//!   The Slint component has no arrow property and its example never shows one,
//!   so the default is [`Arrow::None`] and Slint's own look is the default look.
//!
//! # Coordinates
//!
//! The hover test is `cursor.is_over(layout.bounds())` — what iced's own tooltip
//! does, and what [`popover`](super::popover) does for its press. Inside a
//! [`scrollable`](super::scrollable) the cursor arrives translated into the
//! content's coordinates while `layout.bounds()` is in the window's, so the two
//! agree while the trigger is near the top of the scroll area and drift apart as
//! it scrolls. That is the one place this port is approximate, and it is the
//! popover's approximation, borrowed with the rest of the mechanism.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{overlay, renderer, Clipboard, Shell};
use iced::time::{Duration, Instant};
use iced::{
    mouse, Element, Event, Length, Padding, Point, Rectangle, Shadow, Size, Theme, Vector,
};

use super::popover::{PanelStyle, Placement, Placed};

pub use super::popover::Arrow;
use super::theme::{self, Tokens};

/// How long the pointer rests on the trigger before the label appears — 400ms,
/// the Slint component's `delay`.
pub const DEFAULT_DELAY: Duration = Duration::from_millis(400);

/// The side of the trigger the label prefers.
///
/// The names are the Slint `Placement` enum's, and the default is the one Slint
/// gives a tooltip that sets nothing — [`Side::Top`], the opposite default to a
/// popover's. A side without room flips to the opposite one, and a side that
/// fits nowhere clamps into the window; both are the [popover](super::popover)'s
/// rule, applied to the same trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Side {
    /// Above the trigger. Slint's default.
    #[default]
    Top,
    /// Below the trigger.
    Bottom,
    /// To the left of the trigger.
    Left,
    /// To the right of the trigger.
    Right,
}

impl From<Side> for Placement {
    fn from(side: Side) -> Self {
        match side {
            Side::Top => Self::Top,
            Side::Bottom => Self::Bottom,
            Side::Left => Self::Left,
            Side::Right => Self::Right,
        }
    }
}

/// A delayed hover label over a trigger, built with the methods below.
///
/// The trigger is laid out where the caller puts the tooltip; the label is an
/// overlay, laid out against the window after the whole tree, so a scroll area
/// or a card cannot clip it. See the module comment for the mechanism.
pub struct Tooltip<'a, Message> {
    /// Shared behind an [`Rc`] and lent through a [`RefCell`] because it has two
    /// jobs at once: it is this widget's own layout, drawn every frame, and it
    /// is the anchor the popover places the panel against. An iced [`Element`]
    /// is not `Clone` — it owns a boxed widget — so the same element is lent to
    /// both rather than one of them taking it.
    trigger: Option<Element<'a, Message>>,
    label: Option<Element<'a, Message>>,
    side: Side,
    arrow: Arrow,
    offset: f32,
    delay: Duration,
    style: Box<dyn Fn(&Theme) -> PanelStyle + 'a>,
    /// The appearance the trigger was last drawn with.
    ///
    /// An overlay is built without a theme — [`overlay`](Widget::overlay) is
    /// handed a renderer and nothing else — and the panel's surface is a
    /// function of it, so the theme has to be carried from the one call that
    /// does get it. The base tree is drawn *before* the overlays are built in
    /// `UserInterface::draw`, so [`Widget::draw`] has recorded the theme in
    /// force by the time the panel asks. This is the arrangement the
    /// [popover](super::popover) uses; it is repeated here rather than shared
    /// because the two widgets are built at different points of the frame, and
    /// for the reason the popover's own note gives: a surface resolved for the
    /// wrong appearance is the wrong colour, which is what an earlier version of
    /// the popover did — a white panel with white text in the dark theme.
    theme: Rc<RefCell<Option<Theme>>>,
}

impl<'a, Message> Tooltip<'a, Message> {
    /// A tooltip over `trigger` — any element, usually a
    /// [`Button`](super::button::Button), a bare [`Icon`](super::icon::Icon), or
    /// a status dot.
    pub fn new(trigger: impl Into<Element<'a, Message>>) -> Self {
        Self {
            trigger: Some(trigger.into()),
            label: None,
            side: Side::default(),
            arrow: Arrow::default(),
            offset: 8.0,
            delay: DEFAULT_DELAY,
            style: Box::new(surface),
            theme: Rc::new(RefCell::new(None)),
        }
    }

    /// The label, usually one line of [`text`](iced::widget::text). A tooltip
    /// with no label never opens, which is Slint's `text != ""`.
    pub fn label(mut self, label: impl Into<Element<'a, Message>>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// The side of the trigger the label prefers. [`Side::Top`] by default, and
    /// a side without room flips to the opposite one.
    pub fn placement(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    /// An arrow between the surface and the trigger.
    ///
    /// The Slint component draws none, which is why [`Arrow::None`] is the
    /// default and this is the only way to ask for one. iced has no panel
    /// primitive that carries an arrow either, so this is the port's own
    /// addition, drawn by the popover in the panel's own fill.
    pub fn arrow(mut self, arrow: Arrow) -> Self {
        self.arrow = arrow;
        self
    }

    /// The gap between the trigger and the label, 8px by default — the distance
    /// the Slint component uses (`-self.height - 8px`). With an arrow, the gap
    /// is the arrow's length as well as the space it crosses.
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// How long the pointer must rest on the trigger before the label appears,
    /// [`DEFAULT_DELAY`] (400ms) by default. [`iced::time::Duration::ZERO`] shows it at once,
    /// as iced's own tooltip does.
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// The panel's surface, resolved from the tokens by default by [`surface`]:
    /// the foreground as the fill, the background as the ink on it, `radius-md`
    /// and an 8px shadow.
    ///
    /// It is a function of the theme rather than a value because the appearance
    /// is not known where the panel is built — an overlay is constructed without
    /// one — and a surface resolved for one appearance is the wrong colour in
    /// the other. This is the reason [`Popover::panel`] takes a function too. A
    /// caller that wants a warning or a danger tooltip states it here, and
    /// colours the label to match.
    pub fn panel(mut self, style: impl Fn(&Theme) -> PanelStyle + 'a) -> Self {
        self.style = Box::new(style);
        self
    }

}

/// The surface the Slint component draws: `Theme.foreground` as the fill,
/// `Theme.background` as the text on it, `Theme.radius-md`, no border, and an
/// 8px shadow two pixels below.
///
/// The two colours are the point of it. A tooltip is the one surface in this
/// catalogue that inverts the page rather than sitting a step off it, so the ink
/// is read from the tokens in force — the page's own colour — rather than fixed:
/// a hue used as ink on a fill of the same hue is unreadable in the dark theme,
/// and in the light theme the same mistake hides inside a pale tint.
pub fn surface(theme: &Theme) -> PanelStyle {
    let tokens = Tokens::of(theme);
    PanelStyle::resolve(tokens)
        .fill(tokens.foreground)
        .text_color(tokens.background)
        // Slint draws no border here and puts the separation in the shadow. A
        // fixed dark grey is an edge on white and nothing on near-black; no edge
        // with a shadow is an edge in both.
        .border(tokens.foreground, 0.0)
        .radius(theme::RADIUS_MD)
        .shadow(Shadow {
            color: tokens.shadow,
            offset: Vector::new(0.0, 2.0),
            // iced's blur is a Gaussian sigma where Slint's is a spread; 8 at
            // this offset carries the same weight as Slint's 8px, the way the
            // popover's does. See `PanelStyle::resolve`.
            blur_radius: 8.0,
        })
}

/// The three states of a hover timer: resting, waiting the delay out, showing.
/// This is `iced_widget::tooltip`'s machine, so the timing here matches iced's
/// own tooltip to the frame.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum State {
    #[default]
    Idle,
    /// The pointer is on the trigger and the delay is running from `at`.
    Hovered { at: Instant },
    /// The label is showing.
    Open,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Tooltip<'_, Message>
where
    Message: Clone + 'static,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        // One child: the trigger. The label is not in this widget's layout at
        // all — it is measured inside the panel's overlay, on a tree that
        // overlay owns — so there is no second slot to declare.
        let trigger = self
            .trigger
            .as_ref()
            .expect("the trigger is in place until the panel is built");
        vec![Tree::new(trigger.as_widget())]
    }

    fn diff(&self, tree: &mut Tree) {
        let trigger = self
            .trigger
            .as_ref()
            .expect("the trigger is in place until the panel is built");
        tree.diff_children(&[trigger.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.trigger
            .as_ref()
            .map(|trigger| trigger.as_widget().size())
            .unwrap_or(Size::new(Length::Fill, Length::Fill))
    }

    fn size_hint(&self) -> Size<Length> {
        self.trigger
            .as_ref()
            .map(|trigger| trigger.as_widget().size_hint())
            .unwrap_or(Size::new(Length::Shrink, Length::Shrink))
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // A tooltip is exactly the thing it describes and takes no room of its
        // own: the label is an overlay, out of the flow.
        self.trigger
            .as_mut()
            .expect("the trigger is in place until the panel is built")
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        // The base tree is drawn before the overlays are built for the same
        // frame, so this is the appearance the panel about to be constructed
        // belongs to.
        *self.theme.borrow_mut() = Some(theme.clone());
        self.trigger
            .as_ref()
            .expect("the trigger is in place until the panel is built")
            .as_widget()
            .draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.trigger
            .as_mut()
            .expect("the trigger is in place until the panel is built")
            .as_widget_mut()
            .update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        // The timer, and the one test that drives it: `layout.bounds()` is the
        // trigger under the pointer. Only a mouse event or the frame the delay
        // was asked for may move a tooltip between states — anything else, and a
        // keyboard event could open a label over a trigger nobody points at.
        if matches!(event, Event::Mouse(_))
            || matches!(
                event,
                Event::Window(iced::window::Event::RedrawRequested(_))
            )
        {
            let over = cursor.is_over(layout.bounds());
            let state = tree.state.downcast_mut::<State>();
            let now = Instant::now();
            // A tooltip with no label is one Slint would not open either
            // (`text != ""`), so it never leaves `Idle`.
            let wanted = over && self.label.is_some();

            match (*state, wanted) {
                (State::Idle, true) => {
                    if self.delay == Duration::ZERO {
                        *state = State::Open;
                        shell.invalidate_layout();
                    } else {
                        *state = State::Hovered { at: now };
                    }
                    shell.request_redraw_at(now + self.delay);
                }
                // Left before the delay was up: the timer Slint stops.
                (State::Hovered { .. }, false) => *state = State::Idle,
                (State::Hovered { at }, true) if now.duration_since(at) < self.delay => {
                    // Not yet: ask for the frame the deadline lands on. A wake-up
                    // that arrives early has to ask again, or the label would
                    // wait for the next unrelated redraw.
                    shell.request_redraw_at(now + (self.delay - now.duration_since(at)));
                }
                (State::Hovered { .. }, true) => {
                    *state = State::Open;
                    shell.invalidate_layout();
                }
                // Left while showing: hidden at once, with no fade. Slint closes
                // its popup on the same event.
                (State::Open, false) => {
                    *state = State::Idle;
                    shell.invalidate_layout();
                }
                (State::Open, true) | (State::Idle, false) => {}
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.trigger
            .as_ref()
            .expect("the trigger is in place until the panel is built")
            .as_widget()
            .mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.trigger
            .as_mut()
            .expect("the trigger is in place until the panel is built")
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        _renderer: &iced::Renderer,
        _viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        if tree.state.downcast_ref::<State>() != &State::Open {
            return None;
        }
        // A closed tooltip builds nothing, and a label-less one has nothing to
        // build — Slint's `text != ""`.
        let label = self.label.as_mut()?;
        // The label's tree is prepared from the immutable borrow, because the
        // overlay that follows borrows the element mutably.
        let mut tree_of_label = Tree::new(label.as_widget());
        tree_of_label.diff(label.as_widget());
        let theme = self.theme.borrow().clone()?;
        let style = (self.style)(&theme);
        // The trigger's own layout, and the offset between it and the window.
        // The overlay is placed against the window while `layout` is in the
        // coordinates of whatever scrolls this tooltip, so the offset is kept
        // rather than folded in here: the placement rule has to see the trigger
        // and the window in one space, and `UserInterface` sizes the overlay
        // against the window.
        let trigger = layout.bounds();
        Some(overlay::Element::new(Box::new(Content {
            label,
            tree: tree_of_label,
            style,
            arrow: self.arrow,
            trigger,
            translation,
            gap: self.offset,
            side: self.side.into(),
            padding: Padding {
                top: theme::SPACING_XS,
                bottom: theme::SPACING_XS,
                left: theme::SPACING_SM,
                right: theme::SPACING_SM,
            },
        })))
    }
}

/// The overlay that draws the label: the [popover](super::popover)'s surface,
/// placed by the [popover](super::popover)'s own rule, with the label measured
/// inside it and an arrow in the same fill between the two.
///
/// The popover is a widget and offers its panel as one; a tooltip's panel is a
/// label and nothing else, so it is drawn here rather than borrowing the
/// popover's own overlay — which would hold the trigger's element borrowed for
/// as long as the overlay lives, and a tooltip has to lay that trigger out
/// itself. What *is* borrowed is the part worth borrowing: [`Placed::resolve`]
/// flips and clamps exactly as the popover's panel does, [`Arrow::draw`] draws
/// the same triangle, and [`PanelStyle`] is the same surface.
struct Content<'a, 'b, Message> {
    label: &'b mut Element<'a, Message>,
    /// The label's own tree. It is measured here, on the same pass that draws,
    /// so there is no second measurement to keep in step with this one.
    tree: Tree,
    style: PanelStyle,
    arrow: Arrow,
    /// The trigger, in the coordinates of whatever scrolls the tooltip.
    trigger: Rectangle,
    /// The offset between those coordinates and the window's, which the runtime
    /// hands an overlay for exactly this. Without it a panel over a scrolled
    /// trigger is placed as if the list had never moved, which puts it at the
    /// top of the window — this port did that until it was driven in a browser.
    translation: Vector,
    gap: f32,
    side: Placement,
    padding: Padding,
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer> for Content<'_, '_, Message> {
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        // The label is laid out at the width it wants, inside the panel's
        // inset, and the panel is the result plus that inset.
        let limits = layout::Limits::new(Size::ZERO, bounds).shrink(self.padding);
        let label = self
            .label
            .as_widget_mut()
            .layout(&mut self.tree, renderer, &limits);
        let size = label.size().expand(self.padding);
        // The placement rule reads the trigger against the window, so it is
        // given the trigger where it is drawn: the scrollable's coordinates
        // plus the offset to the window's.
        let trigger = Rectangle::new(
            self.trigger.position() + self.translation,
            self.trigger.size(),
        );
        let placed = Placed::resolve(trigger, size, bounds, self.gap, self.side);
        let origin = placed.origin();
        layout::Node::with_children(
            size,
            vec![label.move_to(Point::new(
                origin.x + self.padding.left,
                origin.y + self.padding.top,
            ))],
        )
        .translate(Vector::new(origin.x, origin.y))
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        inherited: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();
        self.style.draw(renderer, bounds);
        self.arrow
            .draw(renderer, self.style.background(), bounds, self.trigger, self.gap);
        let mut style = *inherited;
        style.text_color = self.style.content_color();
        self.label.as_widget().draw(
            &self.tree,
            renderer,
            theme,
            &style,
            layout.children().next().expect("the label is the panel's only child"),
            cursor,
            &Rectangle::with_size(Size::INFINITE),
        );
    }

    /// A tooltip takes nothing: it is a label, and every event belongs to the
    /// widgets under it. That is GPUI's tooltip overlay's rule and Slint's
    /// `no-auto-close` popup's — the pointer may pass straight through to the
    /// trigger the label describes.
    fn update(
        &mut self,
        _event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
    ) {
    }

    fn mouse_interaction(
        &self,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::None
    }
}

impl<'a, Message> From<Tooltip<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'static,
{
    fn from(tooltip: Tooltip<'a, Message>) -> Self {
        Self::new(tooltip)
    }
}
