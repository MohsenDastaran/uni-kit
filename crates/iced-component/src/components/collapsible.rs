//! A single expandable panel. Mirrors GPUI Component's `Collapsible` and the
//! Slint version beside it: an optional titled trigger row with a chevron, a
//! content panel underneath, and the caller's `open` state deciding which is
//! drawn.
//!
//! The registry key is `collapsible`; the file and the module share the name.
//!
//! The trigger is 8px of padding all round with an 8px gap, a text-sm semibold
//! title in the foreground and a 14px chevron in the muted foreground. An empty
//! title draws no trigger at all, which is the Slint rule: the caller supplies
//! its own trigger above the panel and this is only the body. The panel insets
//! its children 8px on the left, right and bottom.
//!
//! **The trigger is an element**, so the caller wires the click that toggles:
//! `collapsible::trigger("Details", open, Message::Toggle)` is the built-in
//! ghost-button form, and any other element works in its place. Slint's
//! `Collapsible` reports `toggled(open)` from its own trigger; an iced control
//! carries its message instead, so the message is the caller's.
//!
//! The reveal is animated in Slint and the chevron rotates. `animated(true)`
//! measures the panel and moves it from nothing to its full height over
//! [`REVEAL_DURATION`], reversing from wherever it is if the caller toggles
//! mid-flight; without it the panel is shown or hidden outright, which is the
//! behavior every other caller here has. The chevron is drawn in its end state
//! either way: the panel is what carries the motion, and the chevron is a rule
//! in this component rather than a rotating box.
//!
//! **The mechanism is the tooltip's delayed redraw, aimed at the next frame.**
//! iced 0.14 has no interval subscription under this crate's runtime, but an
//! element that implements `Widget` keeps state in the widget tree and can ask
//! for frames itself: while the panel moves it calls `shell.request_redraw()`,
//! and every frame samples the elapsed time to place the height. No timer and
//! no subscription — the same `Shell` the tooltip asks for its delay with, and
//! the route [`reveal`] gives `accordion` and `tree` to reveal their own
//! sections.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{overlay, renderer, Clipboard, Renderer, Shell};
use iced::time::{Duration, Instant};
use iced::widget::{button, column, container, row, text};
use iced::widget::container::Style as ContainerStyle;
use iced::{
    mouse, Alignment, Background, Border, Color, Element, Event, Length, Rectangle, Size, Theme,
    Vector,
};

use super::theme::{self, Tokens};

/// How long a panel takes to reach its target height, Slint's `150ms`.
///
/// The Slint component writes `animate height { duration: 150ms; }` and names
/// no easing, and the Slint reference gives an `animate` block's default easing
/// as `linear`. So the height is a straight line from where it is to where it is
/// going, and this matches the curve exactly. What iced cannot match is honest
/// but different: Slint owns an animation clock and re-evaluates the property
/// every frame, while here a frame *is* the clock — the panel asks for the next
/// one, and the height placed is the line at that frame's instant. Two runs can
/// therefore sample different points of the same line, never different lines.
pub const REVEAL_DURATION: Duration = Duration::from_millis(150);

/// The panel body on its own: `content` is laid out at its full height and
/// clipped to the moving fraction of it that `open` has reached, so the rows
/// under the panel travel with it.
///
/// [`Collapsible`] wraps its panel in this, and it is the seam `accordion` and
/// `tree` compose: wrap a section's content column and drive `open` from the
/// header's message, and the section reveals the way this one does. The
/// content stays mounted while closed, which is what lets it measure and
/// reverse mid-flight. [`REVEAL_DURATION`] is the same duration, so a chevron
/// the caller rotates itself can be timed to it.
pub fn reveal<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    open: bool,
) -> Element<'a, Message> {
    Element::new(Reveal {
        content: content.into(),
        open,
        duration: REVEAL_DURATION,
    })
}

/// A collapsible panel, built with the methods below.
pub struct Collapsible<'a, Message> {
    trigger: Option<Element<'a, Message>>,
    content: Vec<Element<'a, Message>>,
    open: bool,
    content_inset: f32,
    animated: bool,
}

impl<'a, Message> Collapsible<'a, Message> {
    /// A panel with no trigger of its own, which is the Slint component with an
    /// empty title: the caller draws the trigger and this is the body.
    pub fn new() -> Self {
        Self {
            trigger: None,
            content: Vec::new(),
            open: false,
            content_inset: 8.0,
            animated: false,
        }
    }

    /// The trigger row, drawn above the panel. Use [`trigger`] for the built-in
    /// titled form, or build any element with its own message.
    pub fn trigger(mut self, trigger: impl Into<Element<'a, Message>>) -> Self {
        self.trigger = Some(trigger.into());
        self
    }

    /// Whether the panel is shown. The caller's state decides.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Animates the panel between nothing and its measured height over
    /// [`REVEAL_DURATION`], instead of showing or hiding it outright. A toggle
    /// while it is moving reverses from the height it has reached.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// A panel child. Call it more than once to stack the panel's content.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content.push(content.into());
        self
    }

    /// The panel's inset on the left, right and bottom, 8px by default.
    pub fn content_inset(mut self, inset: f32) -> Self {
        self.content_inset = inset;
        self
    }
}

impl<'a, Message> Default for Collapsible<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The built-in trigger: a text-sm semibold title with a 14px chevron, as a
/// ghost row that fills the width. `on_toggle` is the message the caller wants
/// when the row is clicked.
pub fn trigger<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    open: bool,
    on_toggle: Message,
) -> Element<'a, Message> {
    trigger_with_inset(title, open, 8.0, on_toggle)
}

/// [`trigger`] with a different inset, which is what a panel placed inside a
/// header of its own wants.
pub fn trigger_with_inset<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    open: bool,
    inset: f32,
    on_toggle: Message,
) -> Element<'a, Message> {
    let label = title.into();
    let content: Element<'a, Message> = row![
        text(label)
            .size(theme::FONT_SIZE_SM)
            .width(Length::Fill)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..iced::Font::DEFAULT
            })
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
        column![chevron(open, 14.0)].width(Length::Fixed(14.0)),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into();

    button(content)
        .width(Length::Fill)
        .padding(inset)
        .style(|theme: &Theme, status| {
            let tokens = Tokens::of(theme);
            let hovered = matches!(
                status,
                button::Status::Hovered | button::Status::Pressed
            );
            button::Style {
                background: Some(Background::Color(if hovered {
                    tokens.accent
                } else {
                    Color::TRANSPARENT
                })),
                text_color: tokens.foreground,
                border: Border::default(),
                ..button::Style::default()
            }
        })
        .on_press(on_toggle)
        .into()
}

/// The chevron's end state, drawn from the tokens because iced ships no icon
/// set: a short rule under the open state, a wider one under the closed state.
pub fn chevron<'a, Message: 'a>(open: bool, size: f32) -> Element<'a, Message> {
    let width = if open { size * 0.5 } else { size };
    container(iced::widget::Space::new())
        .width(Length::Fixed(width))
        .height(Length::Fixed(1.0))
        .style(|theme: &Theme| ContainerStyle {
            background: Some(Tokens::of(theme).muted_foreground.into()),
            ..ContainerStyle::default()
        })
        .into()
}

/// The animated reveal on its own. [`reveal`] builds one; it is private because
/// the caller gets an `Element`, and only the panel's own contents need to know
/// what is moving.
struct Reveal<'a, Message> {
    content: Element<'a, Message>,
    open: bool,
    duration: Duration,
}

/// What a [`Reveal`] remembers between frames, held in the widget tree instead
/// of the caller's state.
///
/// iced has no timer to subscribe to, so the motion is not stored as a tick but
/// as the line a panel is on and the instant it started: any frame can ask
/// where it should be, and a toggle re-points the line from the value on screen
/// rather than from an endpoint the panel has already left.
#[derive(Debug)]
struct Motion {
    /// The caller's `open` the current leg belongs to.
    open: bool,
    /// The progress this leg started from and the one it heads for, 0 closed
    /// and 1 open.
    from: f32,
    target: f32,
    started: Instant,
    /// The value sampled for the frame being built, 0..=1.
    progress: f32,
    /// The redraw instant whose height is already in the layout tree. A frame
    /// asks for a new layout only when it has not been laid out yet, because
    /// the runtime runs the update loop more than once per frame while a
    /// layout is invalid — without this, every frame would re-invalidate its
    /// own relayout and trip the runtime's three-pass guard.
    framed: Option<Instant>,
}

impl Motion {
    /// A panel that has just been mounted sits at its own end state: it is not
    /// revealing, it is already there.
    fn new(open: bool) -> Self {
        let progress = if open { 1.0 } else { 0.0 };
        Self {
            open,
            from: progress,
            target: progress,
            started: Instant::now(),
            progress,
            framed: None,
        }
    }

    /// Points the line at the caller's `open`, from wherever the panel is now.
    /// A reversal continues from the sampled value, the continuity the design
    /// guide asks of an interrupted transition.
    fn retarget(&mut self, open: bool, now: Instant) {
        if self.open == open {
            return;
        }
        self.from = self.progress;
        self.target = if open { 1.0 } else { 0.0 };
        self.started = now;
        self.open = open;
    }

    /// Samples the line at `now`, and answers whether there is more of it to
    /// come. The curve is linear because Slint's default easing is; see
    /// [`REVEAL_DURATION`].
    fn sample(&mut self, now: Instant, duration: Duration) -> bool {
        let elapsed = now.duration_since(self.started);
        if duration.is_zero() || elapsed >= duration {
            self.progress = self.target;
            return false;
        }
        let t = elapsed.as_secs_f32() / duration.as_secs_f32();
        self.progress = self.from + (self.target - self.from) * t;
        true
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Reveal<'_, Message> {
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<Motion>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(Motion::new(self.open))
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.content]);
    }

    fn size(&self) -> Size<Length> {
        // Full width like the panel it stands in for, and only as tall as the
        // reveal has reached.
        Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // Laid out at its full height first: that is the measurement the reveal
        // moves toward, and there is nothing to measure against until the
        // content has been asked for its size.
        let content = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        let full = content.size();

        let motion = tree.state.downcast_mut::<Motion>();
        // A layout with no update before it — the first build, a resize — still
        // reconciles the caller's `open`, but it does not advance the clock:
        // `update` owns the sampling, so the height and the timer never
        // disagree about which frame they are on.
        motion.retarget(self.open, Instant::now());

        layout::Node::with_children(
            Size::new(full.width, (full.height * motion.progress).max(0.0)),
            vec![content],
        )
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
        let progress = tree.state.downcast_ref::<Motion>().progress;
        if progress <= 0.0 {
            return;
        }
        let Some(content) = layout.children().next() else {
            return;
        };
        let draw = |renderer: &mut iced::Renderer| {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content,
                cursor,
                viewport,
            );
        };

        if progress >= 1.0 {
            draw(renderer);
        } else {
            // A window onto the content, clipped to the height reached so far:
            // the rows below the panel move up with it.
            renderer.with_layer(layout.bounds(), draw);
        }
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
        // The redraw request carries the instant of the frame, and the height
        // is sampled at that instant rather than at whatever `Instant::now()`
        // says this pass; two passes over one frame then agree, which is what
        // lets the second pass see that nothing is left to lay out.
        let frame = match event {
            Event::Window(iced::window::Event::RedrawRequested(at)) => Some(*at),
            _ => None,
        };
        let now = frame.unwrap_or_else(Instant::now);
        let (moving, stale, progress) = {
            let motion = tree.state.downcast_mut::<Motion>();
            motion.retarget(self.open, now);
            let moving = motion.sample(now, self.duration);
            let stale = frame.is_some() && motion.framed != frame;
            if frame.is_some() {
                motion.framed = frame;
            }
            (moving, stale, motion.progress)
        };
        if stale {
            // The height lives in the layout tree — it is what moves the rows
            // below the panel — so a frame asks for a new layout, not only a
            // new paint.
            shell.invalidate_layout();
        }
        if moving {
            // iced has no interval subscription under this runtime, so the
            // frame that just sampled is what asks for the next one. This is
            // the tooltip's `request_redraw_at`, with the compositor's next
            // frame instead of a deadline as the target.
            shell.request_redraw();
        }

        // The content is laid out at its full height even while the panel is
        // closed, so events are trimmed to the window it actually shows: a
        // closed panel takes nothing, and a moving one takes only what is
        // visible. Keyboard and window events do not depend on the cursor and
        // reach it either way.
        if progress <= 0.0 {
            return;
        }
        let Some(content) = layout.children().next() else {
            return;
        };
        let cursor = if cursor.is_over(layout.bounds()) {
            cursor
        } else {
            mouse::Cursor::Unavailable
        };
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            content,
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
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if tree.state.downcast_ref::<Motion>().progress <= 0.0 {
            return mouse::Interaction::None;
        }
        let Some(content) = layout.children().next() else {
            return mouse::Interaction::None;
        };
        let cursor = if cursor.is_over(layout.bounds()) {
            cursor
        } else {
            mouse::Cursor::Unavailable
        };
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            content,
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
        if tree.state.downcast_ref::<Motion>().progress <= 0.0 {
            return;
        }
        if let Some(content) = layout.children().next() {
            self.content
                .as_widget_mut()
                .operate(&mut tree.children[0], content, renderer, operation);
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        if tree.state.downcast_ref::<Motion>().progress <= 0.0 {
            return None;
        }
        let content = layout.children().next()?;
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            content,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a> From<Collapsible<'a, Message>> for Element<'a, Message> {
    fn from(control: Collapsible<'a, Message>) -> Self {
        let open = control.open;
        let inset = control.content_inset;
        let animated = control.animated;

        let mut sections = column![].width(Length::Fill);

        if let Some(trigger) = control.trigger {
            sections = sections.push(trigger);
        }

        if !control.content.is_empty() {
            let body = column(control.content)
                .width(Length::Fill)
                .padding(iced::Padding {
                    top: 0.0,
                    right: inset,
                    bottom: inset,
                    left: inset,
                });

            // An animated panel stays mounted while it is closed, because that
            // is what lets it measure and reverse mid-flight; the immediate one
            // is mounted only when open, as it always was.
            if animated {
                sections = sections.push(reveal(body, open));
            } else if open {
                sections = sections.push(body);
            }
        }

        sections.into()
    }
}
