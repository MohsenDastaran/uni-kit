//! A delayed card of rich content over a trigger. Mirrors the Slint
//! `hover-card.slint` beside GPUI Component's `HoverCard`: a title, a
//! description and a delay, shown under whatever the pointer rests on.
//!
//! # Why it composes rather than re-derives
//!
//! A hover card is a [tooltip](super::tooltip) with more substance: the same
//! three-state hover timer, the same `RedrawRequest::At` deadline, the same
//! overlay that no scroll area can clip, and the same
//! [`Placed`](super::popover::Placed) flip-and-clamp rule. This port therefore
//! builds a [`Tooltip`] and gives it the card's [panel](Self::panel_surface) —
//! the Slint file's own look — instead of writing a second hover machine that
//! could drift from the first. The one thing [`Tooltip`] did not offer was its
//! inset, so it grew a `padding` setter, landed on its own before this file.
//!
//! # The surface, and the trap it avoids
//!
//! The panel is the popover's surface: [`PanelStyle::resolve`]'s fill, edge,
//! radius and shadow, resolved inside the closure the tooltip calls at draw
//! time. Nothing here draws a surface of its own, and no token is read while the
//! element is built — a surface resolved from the light tokens at build time is
//! the wrong colour in the dark theme, which is how this family's table once
//! came out white.
//!
//! # What Slint has that this does not
//!
//! - **A card that answers the pointer.** Slint sets `close-policy:
//!   no-auto-close` and lets the pointer cross into the card; an iced overlay's
//!   content is the caller's and this port's panel is text, so the card takes no
//!   event and the pointer passes through it, exactly as the tooltip's does. A
//!   card that must hold a control can be built on [`Tooltip`] directly with the
//!   same surface and delay.
//! - **A custom content slot and a styling API.** The Slint component exposes
//!   `title` and `description` and nothing else, so neither does this; the
//!   website's "Custom Content Builder" and "Styling" examples describe GPUI's
//!   `content` closure, which this port names rather than fakes.
//! - **Separate open and close delays.** GPUI has both; Slint has one `delay`,
//!   so this has one, and the close is immediate — the delay is cancelled on the
//!   event the pointer leaves, which is what the tooltip's state machine does.
//! - **A placement property with Slint's four sides, not GPUI's six anchors.**
//!   The Slint component pins the card below its host (`y: host.height + 8px`)
//!   and no property changes it; this port exposes the tooltip's [`Side`] so a
//!   caller can pin it above, below, left or right, which is what the page's
//!   Positioning example needs. `Side` is the tooltip's own four-value enum —
//!   the sides, not GPUI's six corner anchors — and a side without room flips
//!   to the opposite one by the shared rule.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::time::Duration;
use iced::widget::{column, container, text};
use iced::{Element, Length, Theme};

use super::popover::PanelStyle;
use super::theme::{self, Tokens};
use super::tooltip::Tooltip;

/// The side of the trigger the card prefers, re-exported from the
/// [tooltip](super::tooltip) whose placement rule the card shares.
pub use super::tooltip::Side;

/// How long the pointer rests on the trigger before the card appears — 280ms,
/// the Slint component's `delay`. The tooltip's own default is 400ms; a card is
/// larger, so it arrives a little sooner.
pub const DEFAULT_DELAY: Duration = Duration::from_millis(280);

/// The card's width, the Slint `width: 240px`.
const WIDTH: f32 = 240.0;

/// The gap between the title and the description, the Slint `spacing: 6px`.
const GAP: f32 = 6.0;

/// A hover card over a trigger, built with the methods below.
///
/// The trigger is laid out where the caller puts the card; the card is an
/// overlay, laid out against the window after the whole tree, so a scroll area
/// or a card cannot clip it. See the module comment for the mechanism.
pub struct HoverCard<'a, Message> {
    trigger: Element<'a, Message>,
    title: String,
    description: String,
    side: Side,
    delay: Duration,
}

impl<'a, Message> HoverCard<'a, Message> {
    /// A card over `trigger` — any element: a
    /// [`Button`](super::button::Button), a link, a whole row. The Slint example
    /// uses the whole of each host, so a row is a valid trigger.
    pub fn new(trigger: impl Into<Element<'a, Message>>) -> Self {
        Self {
            trigger: trigger.into(),
            title: String::new(),
            description: String::new(),
            side: Side::Bottom,
            delay: DEFAULT_DELAY,
        }
    }

    /// The card's heading, drawn semibold in the foreground. Empty draws no
    /// heading, which is the Slint `if title != ""`.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// The line under the heading, in the muted foreground and wrapped to the
    /// card's width. Empty draws no line.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// The side of the trigger the card prefers. [`Side::Bottom`] by default —
    /// where the Slint component pins it — and a side without room flips to the
    /// opposite one by the shared [popover](super::popover) rule.
    pub fn placement(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    /// How long the pointer must rest on the trigger before the card appears,
    /// [`DEFAULT_DELAY`] (280ms) by default. [`Duration::ZERO`] shows it at
    /// once, which is the Slint example's "Immediate".
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }
}

/// The surface the Slint card draws — the popover's own, since a hover card sits
/// on the page the way a popover does.
///
/// It is a function of the theme rather than a value because the appearance is
/// not known where the panel is built; the tooltip calls it inside the closure
/// that draws the panel, so the tokens are read in the theme in force.
pub fn panel_surface(theme: &Theme) -> PanelStyle {
    PanelStyle::resolve(Tokens::of(theme))
}

impl<'a, Message> From<HoverCard<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'static,
{
    fn from(card: HoverCard<'a, Message>) -> Self {
        let HoverCard {
            trigger,
            title,
            description,
            side,
            delay,
        } = card;

        let mut body = column![].spacing(GAP);
        let has_content = !title.is_empty() || !description.is_empty();
        if !title.is_empty() {
            body = body.push(
                text(title)
                    .size(theme::FONT_SIZE_SM)
                    .font(iced::Font {
                        weight: iced::font::Weight::Semibold,
                        ..iced::Font::DEFAULT
                    })
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).foreground),
                    }),
            );
        }
        if !description.is_empty() {
            body = body.push(
                text(description)
                    .size(theme::FONT_SIZE_XS)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
            );
        }

        // The card is drawn on the requested side of the trigger at the Slint
        // width, with the popover's inset; a card with neither heading nor
        // description has no label and never opens — the tooltip's `text != ""`.
        let mut tooltip = Tooltip::new(trigger)
            .placement(side)
            .delay(delay)
            .padding(theme::SPACING_MD)
            .panel(panel_surface);
        if has_content {
            tooltip = tooltip.label(container(body).width(Length::Fixed(WIDTH)));
        }
        tooltip.into()
    }
}
