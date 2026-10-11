//! A conversation pinned to its newest turn. Mirrors GPUI Component's
//! `MessageScroller` and the Slint version beside it: a clipped scroll region of
//! bare aligned rows, an "Unread" rule drawn above the first unseen turn, a
//! bottom edge that fades into the surface behind the list, and a round
//! jump-to-newest control that floats over the last row.
//!
//! **iced owns the scroll position, the application owns the meaning.** Slint
//! cannot read a scroll offset, so the Slint component asks the caller for
//! `scrolled-up` and does the jump itself by setting the flickable's offset.
//! Here the offset is iced's: [`MessageScroller::on_scroll`] reports whether the
//! reader is at the tail, and the caller keeps its own `scrolled_up`, feeding it
//! back with [`MessageScroller::scrolled_up`]. The fade and the jump control are
//! drawn from that value, the way the Slint property drives them. The jump
//! control reports [`MessageScroller::on_jump`], and the caller runs
//! [`jump_to_end`] on the scroller's [`Id`] — which is the other reason the
//! constructor takes that id: it names the scroll region for the caller's own
//! jump controls as well as this one.
//!
//! **Rows are bubbles, not messages.** The Slint scroller passes each turn to
//! `Message` with `show-avatar: false` and a bubble variant chosen by role, so
//! the row it actually draws is a bare aligned bubble: no avatar, no author
//! line, no footer. This component draws that row directly — sent turns on the
//! muted surface, received turns as plain ghost text — rather than rendering the
//! whole `Message` to hide its parts. The 480-pixel cap the Slint `Bubble` puts
//! on the muted surface is [`MessageScroller::max_bubble_width`] here.
//!
//! Five things differ from the Slint source, all of them because iced cannot
//! express them:
//!
//! - iced has no opacity widget, so the fade's two states are the two alphas of
//!   its gradient's stops rather than one element whose opacity changes, and the
//!   150 ms transition that would animate between them has nothing to hang on:
//!   the alpha follows `scrolled_up` immediately.
//! - Slint's layout caps a content-sized bubble at a fraction of the viewport,
//!   `min(480px, Theme.viewport * 0.8)`. An iced element cannot read the
//!   viewport during layout, so the cap is the fixed maximum width alone; a
//!   caller that wants it narrower sets [`MessageScroller::max_bubble_width`].
//! - Slint tests for the end of the list with
//!   `viewport-y == viewport-height - height`, an exact comparison because it
//!   owns both numbers. iced reports the viewport as a fraction, so the same
//!   question is whether that fraction has reached `1.0`.
//! - The jump control's circle is drawn by the button's own style rather than by
//!   a wrapper around a ghost button: a wrapper's surface would sit behind the
//!   button's hover fill instead of under it.
//! - The jump control's glyph is a [`Space`](iced::widget::Space) holding an
//!   `svg::Handle` rather than a call into the catalog's `Icon`, because a
//!   component here reads `theme` and nothing else of the crate's — copy this
//!   file on its own and it still compiles. [`MessageScroller::jump_icon`] takes
//!   the application's artwork, and [`ARROW_DOWN`] is the Lucide glyph the Slint
//!   button names.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::cell::Cell;
use std::f32::consts::PI;
use std::rc::Rc;

use iced::widget::container::Style as ContainerStyle;
use iced::widget::operation::snap_to_end;
use iced::widget::scrollable::{Direction, Scrollbar, Viewport};
use iced::widget::text::Wrapping;
use iced::widget::{Id, Space, button, column, container, rule, scrollable, svg, text};
use iced::{
    Background, Border, Color, Element, Gradient, Length, Radians, Task, Theme, gradient::Linear,
};

use super::theme::{self, Tokens};

// ------------------------------------------------------------------ the design

/// The inset around the list, GPUI's `p_5`. The Slint component takes it as a
/// property; [`MessageScroller::padding`] sets it.
pub const LIST_PADDING: f32 = 20.0;

/// The gap between two turns. Both the Slint list and each row of it are spaced
/// 12 px, so consecutive bubbles sit 12 px apart whichever draws the gap.
pub const ROW_SPACING: f32 = theme::SPACING_MD;

/// The fade band's height, measured up from the bottom of the viewport.
pub const FADE_HEIGHT: f32 = 48.0;

/// The jump control's circle.
pub const JUMP_BUTTON_EDGE: f32 = 32.0;

/// How far the jump control sits above the bottom edge.
pub const JUMP_BUTTON_INSET: f32 = 8.0;

/// How much taller the list's bottom padding becomes while the jump control
/// floats over it, so the last row can clear the button.
pub const JUMP_BUTTON_CLEARANCE: f32 = 40.0;

/// The gap between the rows of the unread rule's little stack.
const UNREAD_SPACING: f32 = 6.0;

/// The unread rule's own inset from the row beneath it.
const UNREAD_PADDING_BOTTOM: f32 = 4.0;

/// The muted surface's cap, Slint's `min(480px, Theme.viewport * 0.8)` with the
/// viewport term dropped for the reason in the module docs.
pub const MAX_BUBBLE_WIDTH: f32 = 480.0;

/// The glyph's box on the jump control: the icon catalog's `Medium` step.
pub const JUMP_ICON_EDGE: f32 = 16.0;

/// The Lucide `arrow-down` glyph, the artwork the Slint button names. The icon
/// set belongs to the application and [`MessageScroller::jump_icon`] replaces
/// this; a component that has to draw a control with no glyph at all can still
/// draw the one the design asks for.
pub const ARROW_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14"/><path d="m19 12-7 7-7-7"/></svg>"#;

// ------------------------------------------------------------------ the turns

/// Whether a turn is something the reader sent or something they received. The
/// Slint `MessageRole`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Role {
    /// A received turn: plain ghost text at the leading edge.
    #[default]
    Assistant,
    /// A sent turn: the muted surface, at the trailing edge.
    User,
}

/// One chat turn. The Slint `ChatTurn`: a role and the body, and nothing else —
/// the scroller's rows carry no avatar, author, time or footer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turn {
    pub role: Role,
    pub body: String,
}

impl Turn {
    pub fn new(role: Role, body: impl Into<String>) -> Self {
        Self {
            role,
            body: body.into(),
        }
    }

    /// A turn the reader received.
    pub fn received(body: impl Into<String>) -> Self {
        Self::new(Role::Assistant, body)
    }

    /// A turn the reader sent.
    pub fn sent(body: impl Into<String>) -> Self {
        Self::new(Role::User, body)
    }
}

/// The row a turn draws: a bubble, sized to its content, aligned by its role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BubbleVariant {
    /// The muted surface — GPUI's low-emphasis sent row: a radius, 12 px of
    /// horizontal padding and 8 px of vertical padding.
    #[default]
    Muted,
    /// No surface at all. Text on the page, full width, which is what the Slint
    /// `Bubble` draws when its variant is `ghost`.
    Ghost,
}

// ------------------------------------------------------------------ styles

/// The colours and geometry a turn's row draws with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RowStyle {
    /// Which of the two bubbles the row draws.
    pub variant: BubbleVariant,
}

impl RowStyle {
    pub fn new(variant: BubbleVariant) -> Self {
        Self { variant }
    }

    /// The bubble's fill, radius and padding. A ghost bubble draws no surface,
    /// which is Slint's `transparent` fill with zero radius and zero padding.
    pub fn bubble(&self, tokens: Tokens) -> ContainerStyle {
        match self.variant {
            BubbleVariant::Muted => ContainerStyle {
                background: Some(tokens.muted.into()),
                border: Border {
                    radius: theme::RADIUS_XL.into(),
                    ..Border::default()
                },
                ..ContainerStyle::default()
            },
            BubbleVariant::Ghost => ContainerStyle::default(),
        }
    }

    /// The bubble's text. Both variants take the foreground: a ghost row is
    /// low-emphasis because it carries no surface, not because its ink is dimmed.
    pub fn ink(&self, tokens: Tokens) -> Color {
        tokens.foreground
    }

    /// The padding inside the surface: 8 px above and below, 12 px at the sides.
    pub fn bubble_padding(&self) -> [f32; 2] {
        match self.variant {
            BubbleVariant::Muted => [theme::SPACING_SM, theme::SPACING_MD],
            BubbleVariant::Ghost => [0.0, 0.0],
        }
    }
}

/// The jump control's circle: GPUI's full radius, a one-pixel border, and the
/// page's own colour as the fill, so it reads as a control floating over the
/// transcript rather than a second surface inside it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct JumpButtonStyle;

impl JumpButtonStyle {
    pub fn resolve(&self, tokens: Tokens, status: button::Status) -> button::Style {
        let fill = match status {
            button::Status::Hovered => theme::hovered(&tokens, tokens.background),
            button::Status::Pressed => theme::pressed(&tokens, tokens.background),
            _ => tokens.background,
        };

        button::Style {
            background: Some(Background::Color(fill)),
            text_color: tokens.foreground,
            border: Border {
                color: tokens.border,
                width: 1.0,
                radius: (JUMP_BUTTON_EDGE / 2.0).into(),
            },
            // A round control whose ring is a whole pixel: snapping keeps the
            // border even on every side.
            snap: true,
            ..button::Style::default()
        }
    }
}

// ------------------------------------------------------------------ the list

/// A list of chat turns that stays pinned to the newest one, built with the
/// methods below.
pub struct MessageScroller<'a, Message> {
    id: Id,
    turns: Vec<Turn>,
    unread_index: Option<usize>,
    scrolled_up: bool,
    jump_button: bool,
    scrollbar: bool,
    padding: f32,
    max_bubble_width: f32,
    jump_icon: svg::Handle,
    on_scroll: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    on_jump: Option<Message>,
}

impl<'a, Message> MessageScroller<'a, Message> {
    /// A scroller over `turns`, named by `id`.
    ///
    /// The id is what the caller's own jump controls pass to [`jump_to_end`],
    /// and it must be unique per scroller on screen: two lists sharing an id
    /// move together.
    pub fn new(id: impl Into<Id>, turns: impl Into<Vec<Turn>>) -> Self {
        Self {
            id: id.into(),
            turns: turns.into(),
            unread_index: None,
            scrolled_up: false,
            jump_button: true,
            scrollbar: true,
            padding: LIST_PADDING,
            max_bubble_width: MAX_BUBBLE_WIDTH,
            jump_icon: svg::Handle::from_memory(ARROW_DOWN),
            on_scroll: None,
            on_jump: None,
        }
    }

    /// Where to draw the "Unread" rule: before the row at this index. `None`
    /// draws none, which is Slint's `-1`.
    pub fn unread_index(mut self, index: impl Into<Option<usize>>) -> Self {
        self.unread_index = index.into();
        self
    }

    /// True while the newest turn is out of view. The fade and the jump control
    /// are drawn from this, so the caller reads the scroll offset in
    /// [`on_scroll`](Self::on_scroll) and feeds the answer back here.
    pub fn scrolled_up(mut self, scrolled_up: bool) -> Self {
        self.scrolled_up = scrolled_up;
        self
    }

    /// Whether the jump control appears at all when the view is scrolled up.
    pub fn jump_button(mut self, jump_button: bool) -> Self {
        self.jump_button = jump_button;
        self
    }

    /// Whether the scrollbar is drawn. GPUI hides it with `scrollbar(false)`.
    pub fn scrollbar(mut self, scrollbar: bool) -> Self {
        self.scrollbar = scrollbar;
        self
    }

    /// The inset around the list. GPUI's story passes `p_5`.
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.0);
        self
    }

    /// The cap on the muted bubble's width. The Slint source computes
    /// `min(480px, Theme.viewport * 0.8)`; see the module docs for why only the
    /// fixed term survives.
    pub fn max_bubble_width(mut self, width: f32) -> Self {
        self.max_bubble_width = width.max(0.0);
        self
    }

    /// The artwork on the jump control, since the icon set belongs to the
    /// application. The default is [`ARROW_DOWN`].
    pub fn jump_icon(mut self, handle: impl Into<svg::Handle>) -> Self {
        self.jump_icon = handle.into();
        self
    }

    /// Called when the scroll position crosses the end of the list, with `true`
    /// once the newest turn is out of view and `false` at the tail, so the
    /// caller can set [`scrolled_up`](Self::scrolled_up). It is called on the
    /// crossing only, not for every scroll event.
    ///
    /// The closure is `Fn`, so a caller that keeps its own flag writes it
    /// through an `Rc<Cell<bool>>` it holds too; see the example.
    pub fn on_scroll(mut self, f: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_scroll = Some(Box::new(f));
        self
    }

    /// Called when the jump control is pressed. The scroller does not move the
    /// viewport on its own here, because iced owns the offset and the caller
    /// owns the screen: run [`jump_to_end`] on the id this scroller was built
    /// with, and clear whatever "new messages" mark goes with it.
    pub fn on_jump(mut self, message: Message) -> Self {
        self.on_jump = Some(message);
        self
    }

    /// The scroll direction, with the scrollbar the caller asked for.
    fn direction(&self) -> Direction {
        let scrollbar = if self.scrollbar {
            Scrollbar::new()
        } else {
            Scrollbar::hidden()
        };

        Direction::Vertical(scrollbar)
    }

    /// The bottom padding: the list's own inset, plus room for the jump control
    /// while it floats over the list.
    fn padding_bottom(&self) -> f32 {
        let clearance = if self.jump_button && self.scrolled_up {
            JUMP_BUTTON_CLEARANCE
        } else {
            0.0
        };

        self.padding + clearance
    }

    /// The list itself, as an element the caller can place.
    fn list(&self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let padding = self.padding;
        let mut list = column![].width(Length::Fill).spacing(ROW_SPACING);

        for index in 0..self.turns.len() {
            let mut rows = column![].width(Length::Fill).spacing(ROW_SPACING);

            // The unread rule sits above the first row the reader has not seen.
            if self.unread_index == Some(index) {
                rows = rows.push(unread_rule());
            }

            if let Some(turn) = self.turns.get(index) {
                rows = rows.push(turn_row(turn, self.max_bubble_width));
            }

            list = list.push(rows);
        }

        container(list)
            .width(Length::Fill)
            .padding(iced::Padding {
                top: padding,
                right: padding,
                bottom: self.padding_bottom(),
                left: padding,
            })
            .into()
    }
}

/// The "Unread" rule: a hairline, the label, another hairline, centred.
fn unread_rule<'a, Message>() -> Element<'a, Message>
where
    Message: 'a,
{
    let hairline = || {
        rule::horizontal(1).style(|theme: &Theme| rule::Style {
            color: Tokens::of(theme).border,
            radius: Border::default().radius,
            fill_mode: rule::FillMode::Full,
            snap: false,
        })
    };

    column![
        hairline(),
        text("Unread")
            .size(theme::FONT_SIZE_XS)
            .width(Length::Fill)
            .align_x(iced::Alignment::Center)
            .style(|theme: &Theme| iced::widget::text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
        hairline(),
    ]
    .width(Length::Fill)
    .spacing(UNREAD_SPACING)
    .padding(iced::Padding {
        bottom: UNREAD_PADDING_BOTTOM,
        ..iced::Padding::default()
    })
    .into()
}

/// The bubble a turn draws, aligned by its role.
fn turn_row<'a, Message>(turn: &Turn, max_bubble_width: f32) -> Element<'a, Message>
where
    Message: 'a,
{
    let variant = match turn.role {
        Role::User => BubbleVariant::Muted,
        Role::Assistant => BubbleVariant::Ghost,
    };
    let style = RowStyle::new(variant);
    let role = turn.role;

    let text = text(turn.body.clone())
        .size(theme::FONT_SIZE_SM)
        .wrapping(Wrapping::Word)
        // Filling a bubble that is itself clamped by `max_width` is what makes a
        // short line hug its own width and a long line wrap at the cap.
        .width(Length::Fill)
        .style(move |theme: &Theme| iced::widget::text::Style {
            color: Some(style.ink(Tokens::of(theme))),
        });

    let max_width = match variant {
        BubbleVariant::Muted => max_bubble_width,
        // A ghost surface stretches to the list instead of hugging its content,
        // which is Slint's `horizontal-stretch: ghost ? 1 : 0`.
        BubbleVariant::Ghost => f32::INFINITY,
    };

    let bubble = container(text)
        .max_width(max_width)
        .padding(style.bubble_padding())
        .style(move |theme: &Theme| style.bubble(Tokens::of(theme)));

    row_for(role, bubble.into())
}

/// The row frame: full width, with the bubble pulled to the leading edge for a
/// received turn and to the trailing edge for a sent one, the way Slint's
/// `cross-axis-alignment` and GPUI's `flex_row_reverse` do it.
fn row_for<'a, Message>(role: Role, bubble: Element<'a, Message>) -> Element<'a, Message>
where
    Message: 'a,
{
    let alignment = match role {
        Role::User => iced::Alignment::End,
        Role::Assistant => iced::Alignment::Start,
    };

    column![bubble]
        .width(Length::Fill)
        .align_x(alignment)
        .into()
}

/// The bottom edge, fading the last partly-scrolled row into the surface behind
/// the list. Slint animates this over 150 ms; see the module docs.
fn fade<'a, Message>(scrolled_up: bool) -> Element<'a, Message>
where
    Message: 'a,
{
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(FADE_HEIGHT))
        .style(move |theme: &Theme| {
            let background = Tokens::of(theme).background;
            // Slint's `@linear-gradient(0deg, background, transparent)`: the
            // surface's colour at the bottom edge, fading upwards. iced measures
            // an angle anticlockwise from the positive x axis in a space where y
            // grows downwards, so π points up. iced has no opacity widget, so the
            // two values `scrolled_up` can take become the two alphas.
            let alpha = if scrolled_up { 1.0 } else { 0.0 };

            ContainerStyle {
                background: Some(Background::Gradient(Gradient::Linear(
                    Linear::new(Radians(PI))
                        .add_stop(0.0, theme::alpha(background, 0.0 * alpha))
                        .add_stop(1.0, theme::alpha(background, 1.0 * alpha)),
                ))),
                ..ContainerStyle::default()
            }
        })
        .into()
}

/// The jump control: a 32-pixel circle, a one-pixel border, and the arrow-down
/// glyph, centred over the list.
fn jump_button<'a, Message>(icon: svg::Handle, on_jump: Option<Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let glyph = svg(icon)
        .width(Length::Fixed(JUMP_ICON_EDGE))
        .height(Length::Fixed(JUMP_ICON_EDGE));

    let control = button(glyph)
        .width(Length::Fixed(JUMP_BUTTON_EDGE))
        .height(Length::Fixed(JUMP_BUTTON_EDGE))
        .padding(0.0)
        .style(|theme: &Theme, status| JumpButtonStyle.resolve(Tokens::of(theme), status));

    let control = match on_jump {
        Some(message) => control.on_press(message),
        None => control,
    };

    control.into()
}

/// Whether the reader is away from the newest turn, remembered between scroll
/// events for one scroller. A [`Cell`] is all this needs: the closure iced calls
/// is `Fn`, and one bit of state outside it is what keeps a scroll event that has
/// not crossed the end from reporting again.
struct TailCrossing {
    last: Rc<Cell<Option<bool>>>,
}

impl TailCrossing {
    fn new() -> Self {
        Self {
            last: Rc::new(Cell::new(None)),
        }
    }

    /// The crossing behind this viewport, if there is one. `None` before the
    /// first event: the list has not reported anything yet, and the viewport
    /// starts wherever iced put it.
    fn crossed(&self, viewport: Viewport) -> Option<bool> {
        // Slint compares the flickable's offset with
        // `viewport-height - height` exactly, because it owns both numbers; iced
        // hands over the viewport as a fraction, so the same question is whether
        // that fraction has reached the end.
        let scrolled_up = viewport.relative_offset().y < 1.0;
        let changed = self.last.get() != Some(scrolled_up);
        self.last.set(Some(scrolled_up));

        changed.then_some(scrolled_up)
    }
}

/// The fade laid against the bottom of the frame.
fn bottom_fade<'a, Message>(scrolled_up: bool) -> Element<'a, Message>
where
    Message: 'a,
{
    // `Pin` takes a position from the frame's top-left, and a `Length` cannot be
    // subtracted from another; a container aligned to the bottom of the same
    // space says the same thing without the arithmetic.
    container(fade(scrolled_up))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(iced::Alignment::End)
        .into()
}

/// The jump control laid eight pixels above the bottom of the frame.
fn floating_jump<'a, Message>(icon: svg::Handle, on_jump: Option<Message>) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    container(jump_button(icon, on_jump))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        // The control's box runs from eight pixels above the bottom edge to
        // eight pixels below it, so centring it that far above the edge lands
        // the button where the design puts it.
        .align_bottom(Length::Fixed(JUMP_BUTTON_EDGE + 2.0 * JUMP_BUTTON_INSET))
        .into()
}

impl<'a, Message> From<MessageScroller<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(mut control: MessageScroller<'a, Message>) -> Self {
        let scrolled_up = control.scrolled_up;
        let show_jump = control.jump_button && scrolled_up;
        let on_jump = control.on_jump.clone();
        let icon = control.jump_icon.clone();
        let on_scroll = control.on_scroll.take();

        // The list is built before the frame it sits in, because its bottom
        // padding answers to the jump control that floats over it.
        let list = control.list();

        let region = scrollable(list)
            .id(control.id.clone())
            .direction(control.direction())
            .width(Length::Fill)
            .height(Length::Fill)
            // Keep the viewport against the end while the content grows, which
            // is what following the live tail means when iced owns the offset.
            .anchor_bottom();

        let region = match on_scroll {
            Some(report) => {
                let crossing = TailCrossing::new();
                let at_tail = control.scrolled_up;

                region.on_scroll(move |viewport: Viewport| {
                    crossing
                        .crossed(viewport)
                        .map_or_else(|| report(at_tail), &report)
                })
            }
            None => region,
        };

        let region: Element<'a, Message> = region.into();
        let mut layers = iced::widget::stack![region];

        if scrolled_up {
            layers = layers.push(bottom_fade(scrolled_up));
        }

        if show_jump {
            layers = layers.push(floating_jump(icon, on_jump));
        }

        container(layers)
            .width(Length::Fill)
            .height(Length::Fill)
            .clip(true)
            .into()
    }
}

/// Move a scroller to its newest turn. The caller runs this from its own update:
/// iced owns the scroll position, so the component reports that the jump was
/// asked for and the application decides what the viewport does.
pub fn jump_to_end<Message>(id: impl Into<Id>) -> Task<Message> {
    snap_to_end(id)
}
