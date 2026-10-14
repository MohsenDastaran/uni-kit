//! A copy control. Mirrors GPUI Component's `Clipboard` and the Slint version
//! beside it: a ghost icon-only button whose copy mark becomes a check once the
//! caller says the value was copied, and whose press reports the value it holds.
//!
//! iced already ships the clipboard itself — `iced::clipboard::write` is a
//! `Task` an application returns from `update` — but no control to press. What
//! this adds over that call is the button, the two Lucide marks from
//! `crates/slint-component/ui/icons`, and the value a press carries.
//!
//! **The check is the caller's state, and the reset is a helper it wraps that
//! state in.** Slint's `Clipboard` owns `copied` and clears it from a `Timer`,
//! because its button's `clicked` sets both. Here the element cannot clear a
//! flag it does not own — the press travels to the caller as a message — and
//! iced has no interval subscription under this crate's runtime to make a
//! `Timer` of it. What a widget can do is ask the shell for a frame at a
//! deadline, which is [`reset_after`]: while the caller's `copied` is set it
//! wakes once after [`COPIED_DURATION`] — Slint's `2s` — and reports `on_reset`,
//! so the caller clears its own flag and the check goes away on its own. The
//! component never invents a second source of truth for it.
//!
//! **The write is the application's.** An element cannot perform an effect;
//! iced's clipboard API is a `Task` the application returns from `update`. The
//! press reports the value and the application writes it, which is the split
//! Slint draws between `copy-requested` and its host. On wasm that `Task` is a
//! no-op in iced 0.14 — `window_clipboard`'s wasm provider is a dummy that
//! refuses every write — so in the browser gallery the check is the whole of
//! what a press shows.
//!
//! **Two GPUI properties have no iced 0.14 counterpart.** There is no
//! accessibility tree to carry `accessibility_label`, and no tooltip: a caller
//! that wants one wraps this in [`Tooltip`](super::tooltip). Neither is faked.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{renderer, Shell};
use iced::time::{Duration, Instant};
use iced::widget::svg;
use iced::{mouse, Element, Event, Length, Rectangle, Size, Theme, Vector};

use super::button::{self, Button, Variant};
use super::theme::{ControlSize, Tokens};

/// How long the check stays before the caller is asked to clear it, Slint's
/// `Timer { interval: 2s; }`.
pub const COPIED_DURATION: Duration = Duration::from_millis(2000);

/// A copy control, built with the methods below.
pub struct Clipboard<'a, Message> {
    value: String,
    size: ControlSize,
    copied: bool,
    on_copy: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

impl<'a, Message> Clipboard<'a, Message> {
    /// A copy control for `value`: a ghost icon-only button, extra-small by
    /// default, which is the size both references default to.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            size: ControlSize::XSmall,
            copied: false,
            on_copy: None,
        }
    }

    /// The control's size. The icon box follows it: 12 / 14 / 16 / 24 pixels,
    /// the same ladder [`Button`] gives an icon.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Whether the caller has just copied `value`: draws the check instead of
    /// the copy mark. The caller owns it — set it on the press, and wrap the
    /// control in [`reset_after`] so it is cleared again after
    /// [`COPIED_DURATION`].
    pub fn copied(mut self, copied: bool) -> Self {
        self.copied = copied;
        self
    }

    /// What a press reports — the value, so the application can hand it to
    /// `iced::clipboard::write`. Without it the control is drawn but disabled,
    /// the way every control in this crate spells "cannot be pressed".
    pub fn on_copy(mut self, on_copy: impl Fn(String) -> Message + 'a) -> Self {
        self.on_copy = Some(Box::new(on_copy));
        self
    }
}

/// Wraps `content` in the two-second reset the Slint component owns, while
/// leaving the flag itself with the caller.
///
/// iced has no interval subscription under this crate's runtime, so the reset
/// cannot be a `Timer` the component starts, and an element cannot clear a flag
/// the caller owns. So the caller keeps `copied` — the value it hands
/// [`Clipboard::copied`] — and this carries the clock: while `copied` is set it
/// asks the shell for one frame when [`COPIED_DURATION`] has passed, then
/// reports `on_reset` so the caller can clear the flag. Wrap the control, or
/// the field it sits in, and the check comes back on its own the way Slint's
/// does.
pub fn reset_after<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    copied: bool,
    on_reset: Message,
) -> Element<'a, Message> {
    Element::new(Reset {
        content: content.into(),
        copied,
        duration: COPIED_DURATION,
        on_reset,
    })
}

/// The reset on its own. [`reset_after`] builds one; it is private because the
/// caller gets an `Element`.
struct Reset<'a, Message> {
    content: Element<'a, Message>,
    copied: bool,
    duration: Duration,
    on_reset: Message,
}

/// What a [`Reset`] remembers between frames: which side of the caller's flag
/// the wait belongs to, when that wait started, and whether it has already
/// spoken — so one active period reports once, even if the caller leaves the
/// flag set.
#[derive(Debug)]
struct Wait {
    copied: bool,
    started: Instant,
    reported: bool,
}

impl Wait {
    fn new(copied: bool) -> Self {
        Self {
            copied,
            started: Instant::now(),
            reported: false,
        }
    }
}

impl<'a, Message: Clone + 'a> Widget<Message, Theme, iced::Renderer> for Reset<'a, Message> {
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<Wait>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(Wait::new(self.copied))
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.content]);
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // The content keeps its own node as this wrapper's only child, so the
        // wrapper adds no layout of its own and the caller's field is unmoved.
        let content = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        let size = content.size();
        layout::Node::with_children(size, vec![content])
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        // The frame carries the instant, so the deadline is measured against
        // the frame that wakes it rather than against whatever `Instant::now()`
        // says this pass.
        let frame = match event {
            Event::Window(iced::window::Event::RedrawRequested(at)) => Some(*at),
            _ => None,
        };
        let now = frame.unwrap_or_else(Instant::now);
        let report = {
            let wait = tree.state.downcast_mut::<Wait>();
            // The flag changing starts a new wait, which is what gives a second
            // copy its own two seconds.
            if wait.copied != self.copied {
                wait.copied = self.copied;
                wait.started = now;
                wait.reported = false;
            }
            if !self.copied || wait.reported {
                false
            } else if now.duration_since(wait.started) >= self.duration {
                wait.reported = true;
                true
            } else {
                // One wake at the deadline is enough; a frame each frame would
                // spend the window redrawing a check that is not moving. Same
                // `request_redraw_at` the tooltip's delay uses.
                shell.request_redraw_at(wait.started + self.duration);
                false
            }
        };
        if report {
            shell.publish(self.on_reset.clone());
        }

        if let Some(content) = layout.children().next() {
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
        if let Some(content) = layout.children().next() {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content,
                cursor,
                viewport,
            );
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
        layout.children().next().map_or(
            mouse::Interaction::None,
            |content| {
                self.content.as_widget().mouse_interaction(
                    &tree.children[0],
                    content,
                    cursor,
                    viewport,
                    renderer,
                )
            },
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn widget::Operation,
    ) {
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
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, iced::Renderer>> {
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

impl<'a, Message> From<Clipboard<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Clipboard<'a, Message>) -> Self {
        let Clipboard {
            value,
            size,
            copied,
            on_copy,
        } = control;

        let artwork: &'static [u8] = if copied { CHECK } else { COPY };
        // The ink is resolved inside the draw closure, because the appearance is
        // only known when iced asks. An SVG tinted once at build time keeps its
        // own black — the colour it was born with — through the dark theme.
        let mark = svg(svg::Handle::from_memory(artwork))
            .width(size.icon_size())
            .height(size.icon_size())
            .style(|theme: &Theme, _status| svg::Style {
                color: Some(button::ink(Tokens::of(theme), Variant::Ghost, false)),
            });

        let mut button = Button::new("")
            .variant(Variant::Ghost)
            .size(size)
            .icon(mark);

        if let Some(on_copy) = on_copy {
            button = button.on_press(on_copy(value));
        }

        button.into()
    }
}

// The two Lucide marks the Slint icon set names (`copy` and `check`, Lucide,
// ISC), one line each, so the embedded artwork is the same the Slint and GPUI
// galleries draw.

const COPY: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>"#;

const CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"/></svg>"#;
