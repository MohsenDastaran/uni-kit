//! A panel anchored to an edge of the surface it covers. Mirrors the Slint
//! `Sheet` beside it: an overlay, a header of title and description with a close
//! control, a body that scrolls, and an optional footer action. It is the
//! supplementary-work surface of the overlay family, so it is built on the same
//! mechanism as [`Popover`](super::popover::Popover) — a `Widget` whose
//! `overlay` returns an `overlay::Element` — and reuses that family's
//! [`Placement`] vocabulary and [`PanelStyle`] surface rather than inventing a
//! second one. It is only the vocabulary: the popover's `Placed` flip-and-clamp
//! does not apply here. A popover is anchored to a trigger it may not fit beside
//! and so flips to the opposite side; a sheet is flush to the edge it names and
//! has no opposite side to flip to.
//!
//! # What covers what
//!
//! The Slint component fills a container and draws the panel over it, so
//! `Sheet::new(page)` wraps the surface the sheet stands in for — a framed
//! preview in the example, the whole window in an application — and the panel
//! is anchored to *that* rectangle, not to the window. The overlay is still laid
//! out against the window and drawn after the whole tree, so the panel is never
//! clipped by a card or covered by what it is beside; its own draw is clipped to
//! the surface, which is the Slint component's `clip: true` and keeps the drop
//! shadow off the page around a preview.
//!
//! # The contract
//!
//! The caller owns `open`, exactly as with the popover: [`Sheet::open`] is what
//! the panel is rendered from and [`Sheet::on_open_change`] is the only way the
//! sheet reports a change — the close control, a click on the overlay, or
//! Escape. A sheet with no `on_open_change` cannot move its own state, because
//! it has none; the caller still renders it open whenever it says so.
//!
//! # The transition
//!
//! The panel travels rather than appearing, which is the `offset` `sheet.slint`
//! animates: from a whole panel away to flush with its edge over [`SLIDE_DURATION`]
//! on Slint's `ease-in-out` — `cubic-bezier(0.42, 0.0, 0.58, 1.0)` — while the
//! scrim fades over [`FADE_DURATION`], whose `animate` block names no curve and
//! so takes Slint's default `linear`. The panel stays in the tree for the whole
//! of both legs, so an interrupted close reverses from the position on screen
//! instead of snapping back to an endpoint.
//!
//! The motion lives in the widget tree, not in the caller's state, and it is
//! driven the way the collapsible's reveal is: `update` samples the instant the
//! frame carries in `RedrawRequested(at)`, and the frame that sampled asks for
//! the next one with `shell.request_redraw()`. There is no interval
//! subscription and none is needed. A sampled frame also asks for a relayout,
//! because the panel's rectangle is part of the overlay's layout and a repaint
//! alone would not move it.
//!
//! # What iced 0.14 cannot express
//!
//! - **A focus trap, and the focus handle to move focus into the panel.** The
//!   overlay is the topmost layer while it is open, so it answers Escape, but it
//!   cannot stop Tab from reaching what is behind it and it cannot move focus
//!   into the panel. A caller that wants the keyboard inside its content gives
//!   that content an `Id` and focuses it. Slint's dialog family has the same
//!   gap.
//! - **An accessible name on the close control.** Slint writes
//!   `accessible-label: "Close"`; iced 0.14 has no accessibility tree to put it
//!   in.
//! - **The close glyph is the chrome's, not the caller's.** The crate's rule is
//!   that icons belong to the application, and the escape hatches around it are
//!   [`Sheet::body`] and the caller's footer; the one control the panel must own
//!   is the close button, so its `x` is the small mark that belongs to the
//!   sheet and the application's icon set is not asked for it.
//! - **`closed()` on a host assignment.** Slint fires `closed` whenever `open`
//!   becomes false, including when the host writes the property itself. iced can
//!   only report the dismissals it sees, so a caller that sets `open(false)`
//!   directly learns nothing; that is the popover's contract too.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{Clipboard, Renderer as _, Shell, overlay, renderer};
use iced::time::{Duration, Instant};
use iced::widget::{Space, column, container, row, svg, text};
use iced::{
    Alignment, Background, Element, Event, Font, Length, Padding, Point, Rectangle, Shadow, Size,
    Theme, Vector,
};

use super::button::{Button, Variant as ButtonVariant};
use super::popover::PanelStyle;
pub use super::popover::Placement;
use super::scrollable::Scrollable;
use super::theme::{self, ControlSize, Tokens};

/// The header's inset, Slint's `padding-left: 20px`, `padding-right: 16px`,
/// `padding-top: 18px`, `padding-bottom: 18px`.
const HEADER_PADDING: Padding = Padding {
    top: 18.0,
    right: 16.0,
    bottom: 18.0,
    left: 20.0,
};

/// The body's inset, Slint's `padding: 20px`.
const BODY_PADDING: f32 = 20.0;

/// The footer's inset, Slint's `padding-left: 20px`, `padding-right: 20px`,
/// `padding-top: 14px`, `padding-bottom: 14px`.
const FOOTER_PADDING: Padding = Padding {
    top: 14.0,
    right: 20.0,
    bottom: 14.0,
    left: 20.0,
};

/// The gap between the title and the description, Slint's `spacing: 4px`.
const HEADER_GAP: f32 = 4.0;

/// How long the panel takes to reach its edge, or leave it, from
/// `sheet.slint`'s `animate offset { duration: 260ms; easing: ease-in-out; }`.
const SLIDE_DURATION: Duration = Duration::from_millis(260);

/// How long the scrim takes to fade, from `sheet.slint`'s
/// `animate opacity { duration: 200ms; }`. It names no easing, and Slint's
/// default for an `animate` block is linear.
const FADE_DURATION: Duration = Duration::from_millis(200);

/// Slint's `ease-in-out`, which `i-slint-core` defines as
/// `cubic-bezier(0.42, 0.0, 0.58, 1.0)`. The curve is sampled by solving its x
/// component for the time parameter and reading y there; a cubic-bezier's y is
/// not a function of linear time, so `t` itself would be the wrong argument.
/// The y component of this curve reduces to `3t² - 2t³`, and its x is
/// symmetric, which is what makes a bisection over the monotonic x enough.
fn ease_in_out(progress: f32) -> f32 {
    let progress = progress.clamp(0.0, 1.0);
    let x = |t: f32| {
        let inverse = 1.0 - t;
        3.0 * inverse * inverse * t * 0.42 + 3.0 * inverse * t * t * 0.58 + t * t * t
    };
    let (mut low, mut high) = (0.0f32, 1.0f32);
    for _ in 0..20 {
        let middle = (low + high) / 2.0;
        if x(middle) < progress {
            low = middle;
        } else {
            high = middle;
        }
    }
    let t = (low + high) / 2.0;
    3.0 * t * t - 2.0 * t * t * t
}

/// A panel that slides in from an edge of the surface it covers, built with the
/// methods below.
///
/// The panel is an overlay anchored to `page`, the surface the sheet stands in
/// for: a framed preview in the gallery, the window's own content in an
/// application. See the module comment for what an iced sheet does and does not
/// do the way Slint's does.
pub struct Sheet<'a, Message> {
    /// The surface the panel is anchored to and covers.
    page: Element<'a, Message>,
    /// The panel's content — Slint's `@children` — moved into the panel the
    /// first time the overlay is built. `None` after that, for this frame.
    body: Option<Element<'a, Message>>,
    /// The whole panel built around the body, kept for the second time the
    /// runtime asks for the overlay in one frame. See [`Sheet::overlay`].
    panel: Option<Element<'a, Message>>,
    open: bool,
    placement: Placement,
    /// The panel's width for a left or right sheet and its height for a top or
    /// bottom one. `0.0` keeps the default for the edge.
    size: f32,
    title: Option<String>,
    description: Option<String>,
    footer: Option<String>,
    overlay_closable: bool,
    on_open_change: Option<Rc<dyn Fn(bool) -> Message + 'a>>,
    on_footer_press: Option<Message>,
}

impl<'a, Message> Sheet<'a, Message> {
    /// A sheet over `page`, the surface the panel covers. It starts closed,
    /// because the caller's own state says what is open.
    pub fn new(page: impl Into<Element<'a, Message>>) -> Self {
        Self {
            page: page.into(),
            body: None,
            panel: None,
            open: false,
            // Not `Placement::default()`: the shared enum defaults to `Bottom`
            // for the popover, where a panel under a trigger is the common case.
            // The Slint sheet's default is `Right`.
            placement: Placement::Right,
            size: 0.0,
            title: None,
            description: None,
            footer: None,
            overlay_closable: true,
            on_open_change: None,
            on_footer_press: None,
        }
    }

    /// The panel's content — the Slint component's `@children`. The component
    /// lays it out with the body's 20px inset and scrolls it; a caller with
    /// several rows gives it a `column!` with the 8px spacing the Slint body
    /// uses, because iced cannot space children it does not own.
    pub fn body(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.body = Some(content.into());
        self
    }

    /// Whether the panel is showing. The caller owns this: it is set from the
    /// application's state, and [`on_open_change`](Self::on_open_change) is how
    /// the application learns it should change.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// The edge the panel comes from, [`Placement::Right`] by default — Slint's
    /// `placement: Placement.right`.
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// The panel's width for a left or right sheet, its height for a top or
    /// bottom one. `0.0` — the default — keeps the edge's own size: 85% of the
    /// surface's width capped at 360px, or 60% of its height capped at 300px.
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// The panel's title. Empty draws no title line.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// The panel's description, under the title in `muted-foreground`.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// The footer's action label. `None` draws no footer at all, which is
    /// Slint's empty `footer-label`.
    pub fn footer(mut self, label: impl Into<String>) -> Self {
        self.footer = Some(label.into());
        self
    }

    /// The message the footer's primary button publishes.
    pub fn on_footer_press(mut self, message: Message) -> Self {
        self.on_footer_press = Some(message);
        self
    }

    /// Whether a click on the overlay closes the sheet, `true` by default.
    pub fn overlay_closable(mut self, overlay_closable: bool) -> Self {
        self.overlay_closable = overlay_closable;
        self
    }

    /// Report every request to change the open state: the close control, an
    /// overlay click and Escape all want `false`.
    ///
    /// This is the only way the sheet speaks. It never opens or closes itself,
    /// so a caller that wants the panel to move must answer with its own state.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    /// The whole panel around `body`: the header, the rule, the scrolling body
    /// and the footer. Built once per frame, because the runtime asks for the
    /// overlay twice and a body that was moved into the first one would leave
    /// the second with nothing to draw.
    fn build_panel(&self, body: Element<'a, Message>) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let mut heading = column![].spacing(HEADER_GAP);
        if let Some(title) = &self.title {
            heading = heading.push(
                text(title.clone())
                    .size(theme::FONT_SIZE_LG)
                    .font(Font {
                        weight: iced::font::Weight::Semibold,
                        ..Font::DEFAULT
                    })
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).foreground),
                    }),
            );
        }
        if let Some(description) = &self.description {
            heading = heading.push(text(description.clone()).size(theme::FONT_SIZE_SM).style(
                |theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                },
            ));
        }

        let close = {
            let mut control = Button::new("")
                .variant(ButtonVariant::Ghost)
                .size(ControlSize::XSmall)
                .icon(
                    svg(svg::Handle::from_memory(CLOSE_MARK))
                        .width(16.0)
                        .height(16.0)
                        .style(|theme: &Theme, _status| svg::Style {
                            color: Some(Tokens::of(theme).foreground),
                        }),
                );
            if let Some(on_open_change) = &self.on_open_change {
                control = control.on_press(on_open_change(false));
            }
            control
        };

        let header = container(
            row![container(heading).width(Length::Fill), close,]
                .spacing(theme::SPACING_MD)
                .align_y(Alignment::Start)
                .width(Length::Fill),
        )
        .padding(HEADER_PADDING)
        .width(Length::Fill);

        let mut panel = column![header, rule(), Scrollable::new(body).padding(BODY_PADDING)]
            .width(Length::Fill)
            .height(Length::Fill);

        if let Some(label) = &self.footer {
            let mut action = Button::new(label.clone()).variant(ButtonVariant::Primary);
            if let Some(message) = self.on_footer_press.clone() {
                action = action.on_press(message);
            }
            panel = panel.push(rule()).push(
                container(action)
                    .padding(FOOTER_PADDING)
                    .width(Length::Fill)
                    .align_x(Alignment::End),
            );
        }

        panel.into()
    }
}

/// The one-pixel rule Slint draws between the header, the body and the footer,
/// in the `border` token: an edge in both appearances where a fixed grey would
/// vanish on one.
fn rule<'a, Message>() -> Element<'a, Message>
where
    Message: 'a,
{
    container(Space::new().height(1.0).width(Length::Fill))
        .width(Length::Fill)
        .height(1.0)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).border.into()),
            ..container::Style::default()
        })
        .into()
}

/// The panel's rectangle: its size for the edge, and its position along the
/// edge's axis interpolated by `progress`, 0 fully out and 1 flush with `host`.
///
/// This is `sheet.slint`'s `offset: open ? 0px : travel`, where the travel is
/// the panel's own extent: at 0 the panel sits a whole panel outside the host,
/// which the draw's clip turns into "not seen yet", and at 1 it is on its edge.
fn panel_rect(host: Rectangle, placement: Placement, size: f32, progress: f32) -> Rectangle {
    let extent = match placement {
        Placement::Left | Placement::Right => Size::new(
            if size > 0.0 {
                size
            } else {
                (host.width * 0.85).min(360.0)
            },
            host.height,
        ),
        Placement::Top | Placement::Bottom => Size::new(
            host.width,
            if size > 0.0 {
                size
            } else {
                (host.height * 0.6).min(300.0)
            },
        ),
    };
    let travel = match placement {
        Placement::Left | Placement::Right => extent.width,
        Placement::Top | Placement::Bottom => extent.height,
    } * (1.0 - progress.clamp(0.0, 1.0));
    let origin = match placement {
        Placement::Right => Point::new(host.x + host.width - extent.width + travel, host.y),
        Placement::Left => Point::new(host.x - travel, host.y),
        Placement::Top => Point::new(host.x, host.y - travel),
        Placement::Bottom => Point::new(host.x, host.y + host.height - extent.height + travel),
    };
    Rectangle::new(origin, extent)
}

/// The panel's own sub-widget state: the tree the caller's body lives in, kept
/// across frames so a scroll position inside a panel does not reset every time
/// the panel opens, and the motion of the panel's edge.
struct State {
    panel: Tree,
    motion: Motion,
}

impl State {
    fn new(open: bool) -> Self {
        Self {
            panel: Tree::empty(),
            motion: Motion::new(open),
        }
    }
}

/// One animated value. It stores the value on screen as well as the leg's
/// endpoints, because a reversal starts from the sampled value: that continuity
/// is what makes an interrupted close travel back open rather than jump to an
/// endpoint it had already left.
#[derive(Debug)]
struct Track {
    from: f32,
    target: f32,
    value: f32,
}

impl Track {
    /// A value that has just been mounted at its end state: not moving.
    fn settled(value: f32) -> Self {
        Self {
            from: value,
            target: value,
            value,
        }
    }

    /// Points the leg at `target` from wherever the value is now.
    fn retarget(&mut self, target: f32) {
        self.from = self.value;
        self.target = target;
    }

    /// Samples the leg at `now`, and answers whether there is more of it to
    /// come. `ease` maps linear time to the reference curve.
    fn sample(
        &mut self,
        now: Instant,
        started: Instant,
        duration: Duration,
        ease: impl Fn(f32) -> f32,
    ) -> bool {
        let elapsed = now.duration_since(started);
        if duration.is_zero() || elapsed >= duration {
            self.value = self.target;
            return false;
        }
        let time = elapsed.as_secs_f32() / duration.as_secs_f32();
        self.value = self.from + (self.target - self.from) * ease(time);
        true
    }
}

/// The panel's position and the scrim's opacity, held in the widget tree. The
/// clock is the frame, not a subscription: any frame can ask where the panel
/// should be at its own instant, so two passes over one frame agree.
#[derive(Debug)]
struct Motion {
    /// The caller's `open` the current leg belongs to.
    open: bool,
    /// When the current leg began, shared by both tracks so a reversal turns
    /// them around together.
    started: Instant,
    /// The panel's travel, 0 out and 1 flush, over [`SLIDE_DURATION`].
    slide: Track,
    /// The scrim's opacity, over [`FADE_DURATION`].
    fade: Track,
    /// The redraw instant whose position is already in the overlay's layout.
    /// Without this every pass over one frame would invalidate its own layout
    /// again and the runtime's three-pass guard would stop the animation.
    framed: Option<Instant>,
}

impl Motion {
    fn new(open: bool) -> Self {
        let value = if open { 1.0 } else { 0.0 };
        Self {
            open,
            started: Instant::now(),
            slide: Track::settled(value),
            fade: Track::settled(value),
            framed: None,
        }
    }

    /// Points both tracks at the caller's `open` from where they are now.
    fn retarget(&mut self, open: bool, now: Instant) {
        if self.open == open {
            return;
        }
        self.open = open;
        let target = if open { 1.0 } else { 0.0 };
        self.slide.retarget(target);
        self.fade.retarget(target);
        self.started = now;
    }

    /// Samples both tracks at `now`, and answers whether either is still
    /// moving.
    fn sample(&mut self, now: Instant) -> bool {
        let sliding = self
            .slide
            .sample(now, self.started, SLIDE_DURATION, ease_in_out);
        let fading = self
            .fade
            .sample(now, self.started, FADE_DURATION, |time| time);
        sliding || fading
    }
}

/// The overlay: the scrim, the panel's surface and the caller's body, anchored
/// to the edge of the host.
///
/// It borrows the panel rather than owning it. The runtime builds a widget's
/// overlay twice per frame — once for the events, once for the paint — so an
/// overlay that took the panel would leave the second call with nothing to
/// draw. See [`Sheet::build_panel`].
struct Panel<'a, 'b, Message> {
    content: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    /// The surface the sheet covers, in window coordinates.
    host: Rectangle,
    /// The panel, in window coordinates.
    panel: Rectangle,
    /// The scrim's opacity this frame.
    fade: f32,
    /// The caller's `open`. A sheet that is leaving is still drawn, but it does
    /// not dismiss again and does not claim the pointer.
    open: bool,
    overlay_closable: bool,
    /// Shared, not taken: the runtime builds the overlay twice per frame.
    on_open_change: Option<Rc<dyn Fn(bool) -> Message + 'a>>,
}

impl<Message> Panel<'_, '_, Message> {
    /// Ask the caller to close, once per request.
    fn dismiss(&self, shell: &mut Shell<'_, Message>) {
        if let Some(on_open_change) = &self.on_open_change {
            shell.publish(on_open_change(false));
        }
    }

    /// The child layout the content lives in. The panel node is the surface and
    /// the content was laid out as its single child, and a container zips its
    /// own children against `layout.children()`: handing it the panel node makes
    /// every nested column one level high and paints only its first child, which
    /// is exactly how a sheet's title once drew alone on an empty panel.
    fn content_layout<'c>(layout: Layout<'c>) -> Layout<'c> {
        layout
            .children()
            .next()
            .expect("the panel lays out exactly one child")
    }
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer> for Panel<'_, '_, Message> {
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds)
            .width(Length::Fixed(self.panel.width))
            .height(Length::Fixed(self.panel.height));
        let content = self
            .content
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);
        layout::Node::with_children(self.panel.size(), vec![content.move_to(Point::ORIGIN)])
            .translate(Vector::new(self.panel.x, self.panel.y))
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let panel = layout.bounds();

        // Only an open sheet answers the keyboard and the scrim. While the
        // panel is leaving it is still drawn, but the input that closed it has
        // already been handled and a second close would be a second message for
        // one intention — the Slint component's `TouchArea` is `enabled` on
        // `open` for the same reason.
        if self.open {
            if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),
                ..
            }) = event
            {
                self.dismiss(shell);
                shell.capture_event();
            }

            // A press on the scrim closes the sheet, and one anywhere else on
            // the surface is swallowed either way: what is behind an open sheet
            // is not an interactive target. A press outside the surface belongs
            // to whatever is there — another section's sheet, or the page — so
            // this overlay neither dismisses nor captures it. iced gives an
            // overlay the event before the widgets under it, which is what makes
            // that choice possible.
            if let Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left))
            | Event::Touch(iced::touch::Event::FingerPressed { .. }) = event
                && !cursor.is_over(panel)
                && cursor.is_over(self.host)
            {
                if self.overlay_closable {
                    self.dismiss(shell);
                }
                shell.capture_event();
            }
        }

        // Updated whether or not the pointer is over the panel, for the reason
        // the popover's panel is: a row that left in one move must still learn
        // it, or its hover sticks.
        let content = Self::content_layout(layout);
        self.content.as_widget_mut().update(
            self.tree, event, content, cursor, renderer, clipboard, shell, &panel,
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> iced::mouse::Interaction {
        if !self.open || !cursor.is_over(self.host) {
            return iced::mouse::Interaction::None;
        }
        // A non-`None` interaction tells the runtime the overlay owns the
        // pointer, so the surface under the scrim is not offered the cursor
        // either — the modal half of "the scrim blocks what is behind it". A
        // control in the panel answers with its own interaction; everything else
        // over the surface is idle rather than absent.
        let content = self.content.as_widget().mouse_interaction(
            self.tree,
            Self::content_layout(layout),
            cursor,
            &layout.bounds(),
            renderer,
        );
        if content == iced::mouse::Interaction::None {
            iced::mouse::Interaction::Idle
        } else {
            content
        }
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        inherited: &renderer::Style,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
    ) {
        let tokens = Tokens::of(theme);
        let panel = layout.bounds();
        // Clipped to the surface, which is the Slint example's `clip: true`:
        // without it the panel's shadow falls onto the page around the preview.
        // While the panel is part-way out, that clip is also what turns the
        // overlap into a slide from the edge.
        renderer.with_layer(self.host, |renderer| {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: self.host,
                    ..renderer::Quad::default()
                },
                Background::Color(tokens.overlay.scale_alpha(self.fade)),
            );
            // The family's surface, square-cornered: a sheet is flush to its
            // edge, where the popover's `radius-lg` would round a corner against
            // nothing. The shadow is the stronger one `sheet.slint` and
            // `dialog.slint` share (24px there), scaled the way the popover's
            // comment explains: iced's blur is a Gaussian sigma, not a spread.
            PanelStyle::resolve(tokens)
                .radius(0.0)
                .shadow(Shadow {
                    color: tokens.shadow,
                    offset: Vector::new(0.0, 8.0),
                    blur_radius: if tokens.dark { 12.0 } else { 8.0 },
                })
                .draw(renderer, panel);

            let mut style = *inherited;
            style.text_color = tokens.foreground;
            self.content.as_widget().draw(
                self.tree,
                renderer,
                theme,
                &style,
                Self::content_layout(layout),
                cursor,
                &self.host,
            );
        });
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        operation.container(None, layout.bounds());
        let content = Self::content_layout(layout);
        operation.traverse(&mut |operation| {
            self.content
                .as_widget_mut()
                .operate(self.tree, content, renderer, operation);
        });
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Sheet<'_, Message>
where
    Message: Clone + 'static,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::new(self.open))
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.page.as_widget())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.page.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.page.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.page.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // A layout with no update before it — the first build after the caller
        // changed `open`, a resize — still reconciles the caller's state, but it
        // does not advance the clock: `update` owns the sampling, so the position
        // and the timer never disagree about which frame they are on.
        tree.state
            .downcast_mut::<State>()
            .motion
            .retarget(self.open, Instant::now());
        self.page
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
        cursor: iced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.page.as_widget().draw(
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
        cursor: iced::mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        // The redraw request carries the instant of the frame, and the panel is
        // placed at that instant rather than at whatever `Instant::now()` says
        // this pass — two passes over one frame then agree, which is what lets
        // the second pass see that there is nothing left to lay out.
        let frame = match event {
            Event::Window(iced::window::Event::RedrawRequested(at)) => Some(*at),
            _ => None,
        };
        let now = frame.unwrap_or_else(Instant::now);
        let (moving, stale) = {
            let motion = &mut tree.state.downcast_mut::<State>().motion;
            motion.retarget(self.open, now);
            let moving = motion.sample(now);
            let stale = frame.is_some() && motion.framed != frame;
            if frame.is_some() {
                motion.framed = frame;
            }
            (moving, stale)
        };
        if stale {
            // The panel's rectangle belongs to the overlay's layout, not to the
            // paint, so a sampled frame asks for a new layout.
            shell.invalidate_layout();
        }
        if moving {
            // iced has no interval subscription under this runtime, so the frame
            // that just sampled is what asks for the next one — the collapsible
            // reveal's route, with the compositor's next frame as the target.
            shell.request_redraw();
        }

        self.page.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> iced::mouse::Interaction {
        self.page.as_widget().mouse_interaction(
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
        self.page
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
        let state = tree.state.downcast_mut::<State>();
        let progress = state.motion.slide.value.clamp(0.0, 1.0);
        let fade = state.motion.fade.value.clamp(0.0, 1.0);
        // A closed sheet that has finished leaving is gone; one that is still
        // travelling is drawn, which is what makes the close an animation
        // rather than a removal on the frame the caller cleared `open`.
        if !self.open && progress <= 0.0 {
            return None;
        }
        // The panel is built once and borrowed afterwards: the runtime asks for
        // the overlay twice per frame — once for the events, once for the paint
        // — and the body is not `Clone`. `build_panel` moves it into the panel
        // the first time; the second call finds the panel already built.
        if self.panel.is_none() {
            let body = self.body.take().unwrap_or_else(|| Space::new().into());
            self.panel = Some(self.build_panel(body));
        }
        let content = self.panel.as_mut()?;
        // The body's sub-tree is reconciled here, before the overlay is built
        // from it: without it a text widget has no paragraph to measure with and
        // lays out at zero height.
        state.panel.diff(content.as_widget());
        // The overlay is laid out against the window, so the surface has to be
        // in window coordinates. `translation` is the window offset of the
        // coordinate space the layout position is already in — a scrollable
        // passes `translation - offset` — so the window position is the sum, not
        // the difference. Subtracting it put a panel below its frame by twice
        // the scroll offset: unseen when a preview sits at the top of the page,
        // and offscreen as soon as the section was scrolled into view.
        let host = Rectangle::new(layout.position() + translation, layout.bounds().size());
        let panel = panel_rect(host, self.placement, self.size, progress);
        Some(overlay::Element::new(Box::new(Panel {
            content,
            tree: &mut state.panel,
            host,
            panel,
            fade,
            open: self.open,
            overlay_closable: self.overlay_closable,
            on_open_change: self.on_open_change.clone(),
        })))
    }
}

impl<'a, Message> From<Sheet<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'static,
{
    fn from(sheet: Sheet<'a, Message>) -> Self {
        Element::new(sheet)
    }
}

/// The close mark, the `x` the Slint sheet's close button draws. It is the
/// chrome's own glyph rather than a caller's icon; see the module comment.
const CLOSE_MARK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>"#;
