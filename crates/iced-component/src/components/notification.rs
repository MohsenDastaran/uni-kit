//! A toast. Mirrors the Slint `notification.slint` beside GPUI Component's
//! `Notification`: a status glyph, a title, a message and an optional dismiss
//! control, stacked in a corner of whatever frame the caller gives the layer.
//!
//! # The two pieces
//!
//! [`Notification`] is the card and [`NotificationLayer`] places a stack of them
//! at one of six anchors. The layer fills the frame it is put in — the Slint
//! component's own `width: 100%; height: 100%` — and aligns the stack to the
//! corner by a trailing padding, so a caller can drop it into a fixed-height
//! frame or into a [`Stack`](iced::widget::Stack) over a whole page without the
//! layer measuring anything by hand.
//!
//! # The surface, and the two traps the family already paid for
//!
//! The card is the popover's surface, not a second drawing of one: it is
//! [`panel_container`] around the card's row, which is the fill, the edge, the
//! radius and the shadow [`PanelStyle::resolve`] already defines. The style is a
//! closure over the theme — resolved where the container draws, never where the
//! element is built — because a surface resolved from the light tokens at build
//! time is the wrong colour in the dark theme, which is exactly how this family's
//! table once came out white. And because the surface is the container's, no
//! panel node is ever handed to its children: the trap the sheet re-learned by
//! writing its own panel draw is not available here.
//!
//! # What Slint has that this does not
//!
//! - **The entry animation.** Slint's layer animates `rise` from an 18px inset
//!   and its card mounts transparent; iced 0.14 has no layout transition and no
//!   subtree opacity, so an arrival is one frame. The gap is stated rather than
//!   faked with a hand-rolled clock.
//! - **A host that owns the list.** GPUI's `WindowExt::push_notification` mounts
//!   a window-level `NotificationList`; Slint's example hands its layer the
//!   notices and answers `dismissed(index)`. An iced layer is an element the
//!   caller places, so the caller owns the list and every card carries the
//!   message that removes it — there is no second source of truth for what is
//!   showing.
//! - **The data-driven `[Notice]` array and `NotificationStack`.** Slint needs
//!   a value to loop over because it cannot build a child element inside a
//!   handler; iced can, so the layer takes elements and `Notice` has no iced
//!   counterpart. `NotificationStack` is the same column this layer builds, so
//!   it is not a second public type.
//! - **Auto-hide, an action button, custom content, unique ids, system
//!   delivery.** None has a Slint property or an iced equivalent here. The
//!   caller schedules its own timeout by dropping the card from its list, and it
//!   places its own controls beside the card.
//! - **A click handler on the card.** Slint's only callback is `closed`, so the
//!   card reports nothing else; the "Clickable Notifications" example means the
//!   buttons that raise a notice, not a notice that answers a press.
//! - **`LeftCenter` and `RightCenter`.** GPUI's `Anchor` has eight values; the
//!   Slint `NotificationPlacement` this port follows has the six below and no
//!   vertical-centre anchor, so neither does [`Placement`].
//! - **A timer's countdown and the OS notification centre.** iced 0.14 has no
//!   widget-timer, and the platform bridge is out of this component's reach; both
//!   are the application's.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{column, container, row, svg, text};
use iced::{Alignment, Color, Element, Length, Theme};

use super::button::{self, Button};
use super::popover::{PanelStyle, panel_container};
use super::theme::{self, ControlSize, Tokens};

/// The default cap on a card's width, the Slint card's `max-width`.
const MAX_WIDTH: f32 = 320.0;

/// The gap between two stacked cards, the Slint layer's `spacing: 8px`.
const STACK_GAP: f32 = 8.0;

/// The status a notification reports. The names and meanings are the Slint
/// theme's, and `Info` is the Slint default here too.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
    Default,
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl Status {
    /// The token this status reports in, or nothing for [`Status::Default`],
    /// which draws in the foreground.
    fn tone(self) -> theme::Status {
        match self {
            Self::Default => theme::Status::Default,
            Self::Info => theme::Status::Info,
            Self::Success => theme::Status::Success,
            Self::Warning => theme::Status::Warning,
            Self::Danger => theme::Status::Danger,
        }
    }

    fn color(self, tokens: Tokens) -> Color {
        tokens.status_color(self.tone())
    }

    /// The status glyph, the same four marks the Slint `Icon` picks between.
    fn glyph(self) -> &'static [u8] {
        match self {
            // `Status::Default` has no mark of its own in Slint either — its
            // glyph expression falls through to `info` — so it draws the info
            // mark in the foreground colour.
            Self::Default | Self::Info => INFO_XML,
            Self::Success => CIRCLE_CHECK_XML,
            Self::Warning => TRIANGLE_ALERT_XML,
            Self::Danger => CIRCLE_X_XML,
        }
    }
}

/// Where a stack sits in its frame. The six values are the Slint
/// `NotificationPlacement` enum's, and the default is the Slint layer's,
/// `TopRight`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Placement {
    TopLeft,
    TopCenter,
    #[default]
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Placement {
    /// The horizontal alignment of a stack at this anchor.
    fn align_x(self) -> Alignment {
        match self {
            Self::TopLeft | Self::BottomLeft => Alignment::Start,
            Self::TopCenter | Self::BottomCenter => Alignment::Center,
            Self::TopRight | Self::BottomRight => Alignment::End,
        }
    }

    /// The vertical alignment of a stack at this anchor. A top anchor packs the
    /// stack at the top edge and a bottom anchor at the bottom edge.
    fn align_y(self) -> Alignment {
        match self {
            Self::TopLeft | Self::TopCenter | Self::TopRight => Alignment::Start,
            Self::BottomLeft | Self::BottomCenter | Self::BottomRight => Alignment::End,
        }
    }
}

/// One toast card, built with the methods below.
pub struct Notification<Message> {
    title: String,
    message: String,
    status: Status,
    closable: bool,
    max_width: f32,
    on_close: Option<Message>,
}

impl<Message> Notification<Message> {
    /// A card with no title, no message and the info status, the Slint card's
    /// own defaults.
    pub fn new() -> Self {
        Self {
            title: String::new(),
            message: String::new(),
            status: Status::default(),
            closable: true,
            max_width: MAX_WIDTH,
            on_close: None,
        }
    }

    /// The heading, drawn semibold in the foreground. Empty draws no title,
    /// which is the Slint `if title != ""`.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// The line under the title, in the muted foreground. Empty draws no line.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// The status, which picks the glyph and the colour it is drawn in.
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    /// Whether the dismiss control is drawn, Slint's `closable`. It is drawn
    /// only when a [`on_close`](Self::on_close) message is also present, because
    /// an iced control that captures a press must carry a message: a card with
    /// `closable(true)` and no message is drawn with no control rather than one
    /// that answers nothing.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }

    /// The card's widest width, 320px by default — the Slint `max-width`.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Send this message when the dismiss control is pressed. The caller owns
    /// the list the card came from, so removing it is the caller's answer to the
    /// message — the same contract the dialog and the alert use.
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }
}

impl<Message> Default for Notification<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Notification<Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(notification: Notification<Message>) -> Self {
        let Notification {
            title,
            message,
            status,
            closable,
            max_width,
            on_close,
        } = notification;

        // The mark is drawn from the status in the theme in force, inside the
        // closure: a colour read when the element was built is the wrong one
        // after a theme switch.
        let mark = svg(svg::Handle::from_memory(status.glyph()))
            .width(16.0)
            .height(16.0)
            .style(move |theme: &Theme, _status| svg::Style {
                color: Some(status.color(Tokens::of(theme))),
            });

        let mut body = column![].spacing(theme::SPACING_XXS).width(Length::Fill);
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
        if !message.is_empty() {
            body = body.push(
                text(message)
                    .size(theme::FONT_SIZE_XS)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
            );
        }

        // The Slint row: the mark, the text column that takes the room, and the
        // dismiss control at the trailing edge, all against the top. The 12px
        // inset is the Slint card's `padding`, applied here because
        // [`panel_container`] deliberately adds no padding of its own.
        let mut content = row![mark]
            .spacing(10.0)
            .padding(theme::SPACING_MD)
            .align_y(Alignment::Start);
        content = content.push(body);
        if closable && let Some(on_close) = on_close {
            content = content.push(container(dismiss(on_close)).align_y(Alignment::Start));
        }

        // The panel's surface around the card, and the card's cap outside it:
        // the surface styling is the popover's alone, the width is layout.
        let card = panel_container(content, |theme: &Theme| {
            PanelStyle::resolve(Tokens::of(theme))
        });

        container(card)
            .width(Length::Fill)
            .max_width(max_width)
            .into()
    }
}

/// A stack of notifications anchored to one corner of its frame, built with the
/// methods below.
///
/// It draws nothing of its own: the frame behind it is the caller's, which is
/// how the Slint example gives each stack a bounded rectangle. A layer with no
/// items is still full-size, so a [`Stack`](iced::widget::Stack) keeps its
/// shape when the last card is dismissed.
pub struct NotificationLayer<'a, Message> {
    items: Vec<Element<'a, Message>>,
    placement: Placement,
    inset: f32,
    stack_width: f32,
}

impl<'a, Message> NotificationLayer<'a, Message> {
    /// An empty layer at the Slint default, the top-right corner, with a 16px
    /// inset and a 320px stack width.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            placement: Placement::default(),
            inset: theme::SPACING_LG,
            stack_width: MAX_WIDTH,
        }
    }

    /// Add one card, in reading order: the first item is nearest the anchored
    /// edge.
    pub fn item(mut self, item: impl Into<Element<'a, Message>>) -> Self {
        self.items.push(item.into());
        self
    }

    /// The corner the stack is anchored to, [`Placement::TopRight`] by default.
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// The distance from the anchored edges, 16px by default — the Slint
    /// `inset`.
    pub fn inset(mut self, inset: f32) -> Self {
        self.inset = inset;
        self
    }

    /// The width of the stack column, 320px by default — the Slint
    /// `stack-width`.
    pub fn stack_width(mut self, stack_width: f32) -> Self {
        self.stack_width = stack_width;
        self
    }
}

impl<Message> Default for NotificationLayer<'_, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<NotificationLayer<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(layer: NotificationLayer<'a, Message>) -> Self {
        let NotificationLayer {
            items,
            placement,
            inset,
            stack_width,
        } = layer;

        let stack = column(items).spacing(STACK_GAP).width(Length::Fixed(stack_width));

        // The layer fills the frame the caller gives it and places the stack by
        // alignment, which is the Slint layer's own rule: it states its size and
        // lets the layouts place the corner.
        container(stack)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(inset)
            .align_x(placement.align_x())
            .align_y(placement.align_y())
            .into()
    }
}

/// The dismiss control: the ghost, extra-small, icon-only button the Slint card
/// draws, with the same stroke every other close control in this crate uses.
fn dismiss<'a, Message>(message: Message) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let glyph = svg(svg::Handle::from_memory(CLOSE_XML))
        .width(16.0)
        .height(16.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(button::ink(
                Tokens::of(theme),
                button::Variant::Ghost,
                false,
            )),
        });

    Button::new("")
        .icon(glyph)
        .variant(button::Variant::Ghost)
        .size(ControlSize::XSmall)
        .on_press(message)
        .into()
}

/// The four status marks, Lucide's strokes at the crate's usual weight.
const INFO_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#;
const CIRCLE_CHECK_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/></svg>"#;
const TRIANGLE_ALERT_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>"#;
const CIRCLE_X_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/></svg>"#;
const CLOSE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>"#;
