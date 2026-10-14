//! A resizable split: two panes and the divider that sets the fixed pane's size.
//! Mirrors the Slint `ResizablePane`/`ResizeHandle` pair beside it and the GPUI
//! base layer's group, panel and handle.
//!
//! Slint packs a pane and its divider as two elements the caller places around
//! each other, and the fixed pane reads the handle's `split` back. This port
//! keeps that shape rather than inventing a group widget: a [`ResizablePane`] is
//! a styled surface, a [`ResizeHandle`] is the divider, and the split is the
//! caller's own value — a pane is given a size, the handle reports what a drag
//! asks for, and the application writes it back. That is the same controlled
//! reading [`Slider`](super::slider) uses for its value, and it is what lets one
//! handle own one edge of a nested workspace without a tree of its own.
//!
//! **The drag is live, and real.** A splitter is where a controlled widget would
//! be expected to lose its drag, so this is worth stating: iced can report one.
//! The handle implements [`iced::advanced::Widget`] directly, exactly as
//! [`Slider`](super::slider) does, and keeps the pointer's press position and the
//! split it started from in its own widget-tree state ([`tree::State`]). A
//! `CursorMoved` while that state says a drag is on reports
//! `start + (position - press)`, clamped — a message per pixel, the same
//! per-pixel reporting iced's own slider makes. What iced has no drag *delta* for
//! is an element that owns no state of its own, such as a table's header cell;
//! a divider is not that, and it is not faked by reporting only on release.
//!
//! **The divider is a 6px strip in the border colour, and its target is wider.**
//! Exactly the Slint original: the strip is 6 pixels, the colour is
//! `Theme.border`, and while the pointer is over or holding it the colour is
//! `Theme.primary` at 45% — the `active` state Slint computes from
//! `has-hover || pressed`. The hit box, like Slint's `TouchArea`, reaches four
//! pixels into each neighbouring pane so a hairline is grabbable without drawing
//! a thicker line; the strip itself never widens.
//!
//! # What iced cannot draw, named rather than faked
//!
//! - **The handle does not animate.** Slint's grip has no transition either, so
//!   nothing is lost; the colour changes with the pointer and nothing eases.
//! - **A clip is rectangular.** Slint's pane sets `clip: true` with a
//!   `border-radius`, so a child is cut to the rounded corner; iced clips to the
//!   pane's rectangle and the radius only rounds the border. A child that reaches
//!   a corner keeps its square one.
//! - **The split is reported in pixels.** Slint's `length` and this `f32` are
//!   both pixels, and neither knows a pane's opposite bound: the caller gives a
//!   [`minimum`](ResizeHandle::minimum) and an optional
//!   [`maximum`](ResizeHandle::maximum), exactly as Slint's handle clamps, so a
//!   pane is never dragged past the numbers the caller named.
//!
//! Everything is drawn from [`Tokens`], read while iced renders, so a theme
//! switch repaints it and no colour is written by hand.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one part gets
// a warning for each one it does not want.
#![allow(dead_code)]

use iced::advanced::Renderer as _;
use iced::advanced::layout::{self, Limits};
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::widget::tree::{self, Tag};
use iced::advanced::{Clipboard, Layout, Shell, Widget, mouse};
use iced::widget::container;
use iced::widget::container::Style as ContainerStyle;
use iced::widget::Space;
use iced::{Border, Element, Event, Length, Rectangle, Size, Theme};
use iced::{touch};

use super::theme::{self, Tokens};

/// How thick the divider is across the axis it separates.
const HANDLE_THICKNESS: f32 = 6.0;
/// How far the grip's hit box reaches into each neighbouring pane, Slint's four
/// pixels of overhang on either side of the six-pixel strip.
const GRIP_OVERHANG: f32 = 4.0;
/// The default floor, Slint's `minimum: 80px`.
const DEFAULT_MINIMUM: f32 = 80.0;
/// The strip's colour while it is hovered or held, `primary.with-alpha(0.45)`.
const ACTIVE_ALPHA: f32 = 0.45;

/// Which way a divider separates two panes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis {
    /// The divider is a vertical strip; the fixed pane's width is the split.
    #[default]
    Horizontal,
    /// The divider is a horizontal strip; the fixed pane's height is the split.
    Vertical,
}

impl Axis {
    /// The coordinate a drag travels along.
    fn position(self, point: iced::Point) -> f32 {
        match self {
            Self::Horizontal => point.x,
            Self::Vertical => point.y,
        }
    }

    /// The divider's size: fixed across, filling along.
    fn size(self) -> Size<Length> {
        match self {
            Self::Horizontal => Size::new(Length::Fixed(HANDLE_THICKNESS), Length::Fill),
            Self::Vertical => Size::new(Length::Fill, Length::Fixed(HANDLE_THICKNESS)),
        }
    }
}

/// A pane of a split: the page's own surface, an optional one-pixel border and
/// the medium radius, with its child packed to the top.
///
/// The size is the caller's: the fixed pane of a split is given the handle's
/// `split`, and the flexible pane keeps the default [`Length::Fill`]. The child
/// is usually a [`column!`](iced::widget::column) whose own padding insets the
/// content, since the pane draws no padding of its own — Slint's pane does not
/// either.
pub struct ResizablePane<'a, Message> {
    bordered: bool,
    width: Length,
    height: Length,
    child: Option<Element<'a, Message>>,
}

impl<'a, Message> ResizablePane<'a, Message> {
    /// A bordered pane that fills the space it is given.
    pub fn new() -> Self {
        Self {
            bordered: true,
            width: Length::Fill,
            height: Length::Fill,
            child: None,
        }
    }

    /// Whether the one-pixel border is drawn. Slint's `bordered`, true by default.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// The pane's size along its split's axis: the handle's split for the fixed
    /// pane, [`Length::Fill`] for the pane that takes what is left.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// The pane's size across its split's axis, filling by default.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// The pane's content.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.child = Some(child.into());
        self
    }
}

impl<'a, Message> Default for ResizablePane<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<ResizablePane<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(pane: ResizablePane<'a, Message>) -> Self {
        let bordered = pane.bordered;
        let content = pane
            .child
            .unwrap_or_else(|| Space::new().into());

        container(content)
            .width(pane.width)
            .height(pane.height)
            // The pane packs its child to the top, the `alignment: start` the
            // Slint pane sets; without it a short child would float.
            .align_y(iced::Alignment::Start)
            .clip(true)
            .style(move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                ContainerStyle {
                    background: Some(tokens.background.into()),
                    text_color: Some(tokens.foreground),
                    border: Border {
                        color: tokens.border,
                        width: if bordered { 1.0 } else { 0.0 },
                        radius: theme::RADIUS_MD.into(),
                    },
                    ..ContainerStyle::default()
                }
            })
            .into()
    }
}

/// The pointer's hold on one handle, kept in the widget tree across the frames a
/// drag spans. The value itself is not here: it belongs to the caller.
#[derive(Debug, Default)]
struct Drag {
    dragging: bool,
    /// Whether the pointer is over the grip. iced draws only when something asks
    /// it to, so the strip's hover colour needs both this and a redraw request
    /// when it changes — without it the strip keeps the hover colour after the
    /// pointer has gone, which is the trap the menu family paid for.
    hovered: bool,
    /// Where the axis coordinate was when the press landed.
    press: f32,
    /// The split the drag started from, so every move is measured from one point
    /// rather than accumulating rounding over the frames in between.
    start_split: f32,
}

/// The divider between two [`ResizablePane`]s.
///
/// It owns no size. [`new`](Self::new) is given the split the caller is showing,
/// [`on_resize`](Self::on_resize) reports every value a drag asks for between
/// [`minimum`](Self::minimum) and [`maximum`](Self::maximum), and the caller
/// writes the split back. A handle built without `on_resize` cannot be moved,
/// which is how iced spells a disabled control.
pub struct ResizeHandle<'a, Message> {
    split: f32,
    minimum: f32,
    /// Zero leaves the split unbounded above, Slint's `maximum: 0px`.
    maximum: f32,
    axis: Axis,
    on_resize: Option<Box<dyn Fn(f32) -> Message + 'a>>,
}

impl<'a, Message> ResizeHandle<'a, Message> {
    /// A horizontal handle holding `split` pixels, between 80 and unbounded,
    /// the Slint defaults.
    pub fn new(split: f32) -> Self {
        Self {
            split,
            minimum: DEFAULT_MINIMUM,
            maximum: 0.0,
            axis: Axis::default(),
            on_resize: None,
        }
    }

    /// The floor the split clamps to, Slint's `minimum`.
    pub fn minimum(mut self, minimum: f32) -> Self {
        self.minimum = minimum;
        self
    }

    /// The ceiling the split clamps to; zero means none, Slint's `maximum`.
    pub fn maximum(mut self, maximum: f32) -> Self {
        self.maximum = maximum;
        self
    }

    /// Which way the divider separates its panes. Horizontal by default.
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// The split a drag asks for, already clamped. Slint's `resized`, with iced's
    /// caller writing the value back rather than an in-out property.
    pub fn on_resize(mut self, on_resize: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_resize = Some(Box::new(on_resize));
        self
    }

    /// Whether the handle answers the pointer.
    fn is_enabled(&self) -> bool {
        self.on_resize.is_some()
    }

    /// The split `next` clamps to, exactly the Slint `resize-to` arithmetic: a
    /// floor that always applies and a ceiling that applies only when set.
    fn clamp(&self, next: f32) -> f32 {
        let low = self.minimum.max(next);
        if self.maximum > 0.0 {
            self.maximum.min(low)
        } else {
            low
        }
    }

    /// The hit box: the strip plus four pixels into each neighbouring pane.
    fn grip(&self, bounds: Rectangle) -> Rectangle {
        bounds.expand(GRIP_OVERHANG)
    }

    /// Report `next` when it differs from the split already shown. A drag reports
    /// every pixel of travel, so an unchanged value is not a change.
    fn report(&self, next: f32, shell: &mut Shell<'_, Message>) {
        if (next - self.split).abs() > f32::EPSILON
            && let Some(on_resize) = &self.on_resize
        {
            shell.publish(on_resize(next));
        }
    }
}

impl<'a, Message> Widget<Message, Theme, iced::Renderer> for ResizeHandle<'a, Message>
where
    Message: 'a,
{
    fn tag(&self) -> Tag {
        Tag::of::<Drag>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Drag::default())
    }

    fn size(&self) -> Size<Length> {
        self.axis.size()
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &Limits,
    ) -> layout::Node {
        let size = self.axis.size();
        layout::atomic(limits, size.width, size.height)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let drag = tree.state.downcast_mut::<Drag>();
        let bounds = layout.bounds();
        let grip = self.grip(bounds);
        let enabled = self.is_enabled();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if enabled && let Some(position) = cursor.position_over(grip) {
                    drag.dragging = true;
                    drag.press = self.axis.position(position);
                    drag.start_split = self.split;
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                drag.dragging = false;
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                // iced repaints on request, not on every move, so the hover
                // colour is tracked here and a change asks for the frame that
                // draws it.
                let over = cursor.is_over(grip);
                if over != drag.hovered {
                    drag.hovered = over;
                    shell.request_redraw();
                }
                if drag.dragging && enabled {
                    if let Some(position) = cursor.land().position() {
                        let delta = self.axis.position(position) - drag.press;
                        self.report(self.clamp(drag.start_split + delta), shell);
                    }
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorLeft) => {
                if drag.hovered {
                    drag.hovered = false;
                    shell.request_redraw();
                }
            }
            Event::Touch(touch::Event::FingerPressed { position, .. }) => {
                if enabled && grip.contains(*position) {
                    drag.dragging = true;
                    drag.press = self.axis.position(*position);
                    drag.start_split = self.split;
                    shell.capture_event();
                }
            }
            Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                drag.dragging = false;
            }
            Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if drag.dragging && enabled {
                    let delta = self.axis.position(*position) - drag.press;
                    self.report(self.clamp(drag.start_split + delta), shell);
                    shell.capture_event();
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let tokens = Tokens::of(theme);
        let drag = tree.state.downcast_ref::<Drag>();
        // Slint's `active`: the pointer over the grip, or holding it.
        let active = drag.dragging || drag.hovered;
        let color = if active {
            tokens.primary.scale_alpha(ACTIVE_ALPHA)
        } else {
            tokens.border
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                ..renderer::Quad::default()
            },
            color,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let dragging = tree.state.downcast_ref::<Drag>().dragging;
        if self.is_enabled() && (dragging || cursor.is_over(self.grip(layout.bounds()))) {
            match self.axis {
                Axis::Horizontal => mouse::Interaction::ResizingHorizontally,
                Axis::Vertical => mouse::Interaction::ResizingVertically,
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message> From<ResizeHandle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(handle: ResizeHandle<'a, Message>) -> Self {
        Element::new(handle)
    }
}
