//! `bubble` — a chat surface: a message, the edge it sits on, and the reaction
//! region attached to it.
//!
//! The Slint authority is `crates/slint-component/ui/bubble.slint`, with the
//! example beside it in `ui/examples/bubble.slint`; the GPUI original is
//! `crates/component/src/bubble.rs`. This port follows the Slint rendering,
//! because that is the design the two galleries share:
//!
//! - **[`Bubble`]** is the row. It owns the alignment, the optional avatar slot,
//!   the optional timestamp and the optional reaction region, and it lays the
//!   surface out. Slint defaults the alignment to `start`, which is what an
//!   assistant turn is; [`Bubble::alignment`] moves it to the trailing edge.
//! - **[`Content`]** is the visible surface. It owns the seven variants'
//!   fill/ink/border, the radius and padding, and the callers' per-surface
//!   overrides — the `fill`, `text-color`, `border-color`, `border-width`,
//!   `radius`, `padding-x` and `padding-y` properties the Slint component
//!   exposes, and the `BubbleContent`/`Styled` path the GPUI page documents.
//! - **[`Group`]** is the vertical stack of consecutive bubbles from one sender,
//!   the Slint `BubbleGroup` and GPUI `BubbleGroup`: an 8-pixel gap, aligned as
//!   a whole.
//! - **[`Reactions`]** is the pill under (or above) a bubble edge: the muted
//!   fill, the three-pixel background ring and the full radius of Slint's
//!   `BubbleReactions`, with the leading/trailing 12-pixel inset of
//!   `BubbleReactionRow`.
//!
//! Four deliberate differences from the Slint source, all because iced 0.14 has
//! no equivalent:
//!
//! - **The avatar is the caller's element.** Slint draws its own `Avatar` at
//!   `xsmall` with hard-coded initials; iced exports no avatar from here, so
//!   [`Bubble::avatar`] takes an `Element` — normally
//!   `super::avatar::Avatar` — exactly as GPUI leaves the avatar to `Message`.
//!   The example never switches `show-avatar` on, so nothing in the reference
//!   exercises it.
//! - **The maximum width is a fixed 480 pixels by default.** Slint caps a
//!   regular bubble at `min(480px, viewport * 0.8)`; iced 0.14 has no relative
//!   maximum width, so the viewport half of that is dropped and named here, and
//!   [`Bubble::max_width`] replaces the fixed cap when a caller needs another.
//! - **A reaction region is a sibling, not an overlay.** GPUI positions
//!   `BubbleReactions` absolutely over the bubble edge; Slint stacks it in a
//!   `VerticalLayout` with a 6-pixel gap, and this follows Slint, so iced's
//!   `Stack` is not needed at all.
//! - **The reaction pill's children are plain elements.** GPUI's typed
//!   `.action(Button)` re-rounds the button to join the surface; this port has
//!   no such refinement, so an action is a `super::button::Button` passed to
//!   [`Reactions::child`], which keeps its own radius inside the pill.
//!
//! The content is caller-composed, as the GPUI page says: text, files, buttons,
//! links and overlays are arbitrary elements, and `Bubble` does not own a message
//! record, a collapsed state or a reaction model.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::container::Style as ContainerStyle;
use iced::widget::text::Wrapping;
use iced::widget::{column, container, row, text};
use iced::{
    Alignment as IcedAlignment, Background, Border, Color, Element, Length, Padding, Theme,
};

use super::theme::{self, Tokens};

/// The widest a regular bubble grows by default: Slint's
/// `min(480px, viewport * 0.8)`, with the viewport term dropped because iced
/// 0.14 has no relative maximum. [`Bubble::max_width`] replaces it.
pub const MAX_WIDTH: f32 = 480.0;

/// The vertical padding inside a regular surface, `py_2` in the GPUI original.
const PADDING_Y: f32 = theme::SPACING_SM;

/// The horizontal padding inside a regular surface, `px_3` in the GPUI original.
const PADDING_X: f32 = theme::SPACING_MD;

/// The gap between the surface and its timestamp, which GPUI calls `gap_1`.
const TIME_GAP: f32 = theme::SPACING_XS;

/// The gap between the bubble and a reaction region, Slint's `BubbleWithReactions`
/// `spacing: 6px`; the region's own row adds two more pixels of breathing room.
const REACTION_GAP: f32 = 6.0;

/// How far a reaction pill is pulled in from the bubble edge, GPUI's `left-3` /
/// `right-3` and Slint's `BubbleReactionRow` padding.
const REACTION_INSET: f32 = 12.0;

/// Which horizontal edge a bubble sits on. Slint's `BubbleAlignment`; GPUI
/// shares `MessageAlignment` with `Message`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    /// The leading edge, a received turn.
    #[default]
    Start,
    /// The trailing edge, a sent turn.
    End,
}

impl Alignment {
    /// The iced alignment this edge means.
    pub fn axis(self) -> IcedAlignment {
        match self {
            Self::Start => IcedAlignment::Start,
            Self::End => IcedAlignment::End,
        }
    }

    /// Whether this is the sent edge, which is what decides whether the avatar
    /// is drawn after the surface rather than before it.
    pub fn is_end(self) -> bool {
        matches!(self, Self::End)
    }
}

/// The visual treatment of the surface. The names and colours are the Slint
/// `BubbleVariant`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// The primary surface, drawn on the primary fill.
    #[default]
    Filled,
    /// The neutral secondary surface.
    Secondary,
    /// The lower-emphasis surface, on the muted fill.
    Muted,
    /// A subtle primary-tinted surface.
    Tinted,
    /// A page-coloured surface with a visible border.
    Outline,
    /// No surface at all: no fill, border, radius or padding, and the full row
    /// width is available.
    Ghost,
    /// A destructive surface for a failed turn.
    Destructive,
}

/// The edge a reaction region is attached to. Slint's `BubbleReactionSide`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReactionSide {
    /// Above the surface.
    Top,
    /// Below the surface, the default in both references.
    #[default]
    Bottom,
}

/// The bubble surface's resolved look: the fill, the ink, and the edge.
///
/// Every token is read inside the style closure when iced renders the element,
/// so a theme switch moves the surface with it. Resolving one of these at
/// view-build time is what once painted a table white in the dark theme.
#[derive(Debug, Clone, Copy)]
struct Surface {
    variant: Variant,
    fill: Option<fn(&Tokens) -> Color>,
    text_color: Option<fn(&Tokens) -> Color>,
    border_color: Option<fn(&Tokens) -> Color>,
    border_width: Option<f32>,
    radius: Option<f32>,
    padding_x: Option<f32>,
    padding_y: Option<f32>,
}

impl Surface {
    /// The fill the variant falls back to.
    fn variant_fill(&self, tokens: Tokens) -> Color {
        match self.variant {
            Variant::Filled => tokens.primary,
            Variant::Secondary | Variant::Muted => tokens.muted,
            // Slint mixes the primary toward the page: 76% in the dark theme,
            // 88% in the light one, which is GPUI's 0.24 / 0.12 the other way.
            Variant::Tinted => theme::mix(
                tokens.primary,
                tokens.background,
                if tokens.dark { 0.76 } else { 0.88 },
            ),
            Variant::Outline => tokens.background,
            Variant::Ghost => Color::TRANSPARENT,
            Variant::Destructive => {
                theme::alpha(tokens.destructive, if tokens.dark { 0.2 } else { 0.1 })
            }
        }
    }

    /// The ink the content sits in. A destructive surface takes the destructive
    /// role itself; the rest pair with their fill's foreground.
    fn ink(&self, tokens: Tokens) -> Color {
        self.text_color
            .map(|color| color(&tokens))
            .unwrap_or(match self.variant {
                Variant::Filled => tokens.primary_foreground,
                Variant::Secondary => tokens.secondary_foreground,
                Variant::Destructive => tokens.destructive,
                _ => tokens.foreground,
            })
    }

    /// The corner: zero for a ghost, the surface tier otherwise.
    fn radius(&self) -> f32 {
        self.radius.unwrap_or(if self.variant == Variant::Ghost {
            0.0
        } else {
            theme::RADIUS_XL
        })
    }

    /// The padding inside the surface, `[vertical, horizontal]`.
    fn padding(&self) -> [f32; 2] {
        let (x, y) = if self.variant == Variant::Ghost {
            (0.0, 0.0)
        } else {
            (PADDING_X, PADDING_Y)
        };
        [self.padding_y.unwrap_or(y), self.padding_x.unwrap_or(x)]
    }

    /// Whether the surface stretches to the row, which only a ghost does.
    fn is_ghost(&self) -> bool {
        self.variant == Variant::Ghost
    }

    /// The container style: fill, edge and corner, resolved from the tokens the
    /// active theme reports.
    fn resolve(&self, tokens: Tokens) -> ContainerStyle {
        let outline = self.variant == Variant::Outline;
        let border_width = self.border_width.unwrap_or(if outline { 1.0 } else { 0.0 });
        let border_color = self
            .border_color
            .map(|color| color(&tokens))
            .unwrap_or(if outline {
                tokens.border
            } else {
                Color::TRANSPARENT
            });

        ContainerStyle {
            background: Some(Background::Color(
                self.fill
                    .map(|color| color(&tokens))
                    .unwrap_or_else(|| self.variant_fill(tokens)),
            )),
            text_color: Some(self.ink(tokens)),
            border: Border {
                color: border_color,
                width: border_width,
                radius: self.radius().into(),
            },
            ..ContainerStyle::default()
        }
    }
}

/// The visible surface inside a [`Bubble`]: the content and the style overrides
/// a caller can apply to it.
pub struct Content<'a, Message> {
    children: Vec<Element<'a, Message>>,
    fill: Option<fn(&Tokens) -> Color>,
    text_color: Option<fn(&Tokens) -> Color>,
    border_color: Option<fn(&Tokens) -> Color>,
    border_width: Option<f32>,
    radius: Option<f32>,
    padding_x: Option<f32>,
    padding_y: Option<f32>,
}

impl<'a, Message> Content<'a, Message> {
    /// An empty surface that takes its variant from the bubble around it.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            fill: None,
            text_color: None,
            border_color: None,
            border_width: None,
            radius: None,
            padding_x: None,
            padding_y: None,
        }
    }

    /// A child element. The caller colours arbitrary content itself, as the GPUI
    /// page says; text added with [`Bubble::text`] follows the surface's ink.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// A fill that replaces the variant's own, Slint's `fill`. The function
    /// receives the active tokens, so the colour is resolved when iced renders
    /// the bubble and a theme switch moves it; a fixed `Color` would freeze one
    /// appearance. iced's element tree is rebuilt without a theme handle, which
    /// is why the caller supplies a function rather than a value.
    pub fn fill(mut self, fill: fn(&Tokens) -> Color) -> Self {
        self.fill = Some(fill);
        self
    }

    /// An ink that replaces the variant's own, Slint's `text-color`.
    pub fn text_color(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// A border colour that replaces the variant's own, Slint's `border-color`.
    pub fn border_color(mut self, color: fn(&Tokens) -> Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// A border width, Slint's `border-width`.
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }

    /// A corner radius in pixels, Slint's `radius`.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    /// The horizontal padding, Slint's `padding-x`.
    pub fn padding_x(mut self, padding: f32) -> Self {
        self.padding_x = Some(padding);
        self
    }

    /// The vertical padding, Slint's `padding-y`.
    pub fn padding_y(mut self, padding: f32) -> Self {
        self.padding_y = Some(padding);
        self
    }

    /// The surface this content draws on, given the bubble's variant.
    fn surface(&self, variant: Variant) -> Surface {
        Surface {
            variant,
            fill: self.fill,
            text_color: self.text_color,
            border_color: self.border_color,
            border_width: self.border_width,
            radius: self.radius,
            padding_x: self.padding_x,
            padding_y: self.padding_y,
        }
    }
}

impl<'a, Message> Default for Content<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// A chat bubble: a row that places a [`Content`] surface, an optional timestamp
/// and an optional reaction region on one edge.
pub struct Bubble<'a, Message> {
    alignment: Alignment,
    variant: Variant,
    text: String,
    content: Content<'a, Message>,
    time: String,
    max_width: f32,
    avatar: Option<Element<'a, Message>>,
    reactions: Option<Reactions<'a, Message>>,
}

impl<'a, Message> Bubble<'a, Message> {
    /// A filled bubble at the leading edge, Slint's defaults for an assistant
    /// turn.
    pub fn new() -> Self {
        Self {
            alignment: Alignment::default(),
            variant: Variant::default(),
            text: String::new(),
            content: Content::new(),
            time: String::new(),
            max_width: MAX_WIDTH,
            avatar: None,
            reactions: None,
        }
    }

    /// Place the bubble at the leading or trailing edge. GPUI leaves this unset
    /// so a parent can supply it; iced has no such propagation, so the default
    /// is the leading edge.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Select the surface treatment.
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// The widest a regular surface grows, replacing [`MAX_WIDTH`]. A ghost
    /// stretches to the row instead, so the cap does not apply to it.
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width.max(0.0);
        self
    }

    /// The message body, drawn in the surface's ink at `text-sm`, before any
    /// children.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Replace the visible content surface. Children already added directly to
    /// the bubble move into the new surface, in front of its own children, so
    /// `.child(...)` composes the same way on either side of this call.
    pub fn content(mut self, mut content: Content<'a, Message>) -> Self {
        let mut children = std::mem::take(&mut self.content.children);
        children.append(&mut content.children);
        content.children = children;
        self.content = content;
        self
    }

    /// Add a child to the surface.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.content.children.push(child.into());
        self
    }

    /// The timestamp below the surface, in the muted ink at 10 pixels.
    pub fn time(mut self, time: impl Into<String>) -> Self {
        self.time = time.into();
        self
    }

    /// The avatar beside the surface: before it at the leading edge, after it at
    /// the trailing edge. iced ships no avatar here, so the element is the
    /// caller's — normally `super::avatar::Avatar`.
    pub fn avatar(mut self, avatar: impl Into<Element<'a, Message>>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    /// Attach a reaction region to the bubble's top or bottom edge.
    pub fn reactions(mut self, reactions: Reactions<'a, Message>) -> Self {
        self.reactions = Some(reactions);
        self
    }
}

impl<'a, Message> Default for Bubble<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Bubble<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(mut control: Bubble<'a, Message>) -> Self {
        let surface = control.content.surface(control.variant);
        let alignment = control.alignment;
        let axis = alignment.axis();
        // The ink is read inside the closure rather than captured here, so a
        // theme switch moves the body with the surface.
        let ink = move |tokens: Tokens| surface.ink(tokens);
        let has_text = !control.text.is_empty();

        // The body column: the reaction region above when it is on the top edge,
        // the surface, the timestamp, and the region below. A ghost surface
        // stretches to the row; the rest hug their content up to `MAX_WIDTH`.
        let mut body = column![].spacing(TIME_GAP).align_x(axis);
        let side = control.reactions.as_ref().map(|item| item.side);

        if side == Some(ReactionSide::Top) {
            if let Some(reactions) = control.reactions.take() {
                body = body.push(reactions);
            }
        }

        let mut inner = column![].spacing(theme::SPACING_SM);
        if has_text {
            let body_text = control.text.clone();
            inner = inner.push(
                text(body_text)
                    .size(theme::FONT_SIZE_SM)
                    .wrapping(Wrapping::Word)
                    .style(move |theme: &Theme| iced::widget::text::Style {
                        color: Some(ink(Tokens::of(theme))),
                    }),
            );
        }

        for child in control.content.children {
            inner = inner.push(child);
        }

        // A ghost has no surface to hug, so its content takes the row width;
        // every other variant keeps the compressed, intrinsic measurement the
        // comment below explains.
        if surface.is_ghost() {
            inner = inner.width(Length::Fill);
        }

        let padding = surface.padding();
        let mut surface_container = container(inner)
            .padding(padding)
            .style(move |theme: &Theme| surface.resolve(Tokens::of(theme)));

        // A regular surface hugs its content and is clamped by `MAX_WIDTH`; a
        // ghost stretches. The inner column stays `Shrink` for the regular
        // surface: a fluid child inside a compressed cross axis is laid out
        // second, and a sole fluid child would resolve against a zero cross
        // size — the whole bubble then draws at zero width. A `Text` survives
        // that case because `Limits::resolve` falls back to its intrinsic size.
        if surface.is_ghost() {
            surface_container = surface_container.width(Length::Fill);
        } else {
            surface_container = surface_container.max_width(control.max_width);
        }

        body = body.push(surface_container);

        if !control.time.is_empty() {
            body = body.push(text(control.time).size(10.0).style(|theme: &Theme| {
                iced::widget::text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }
            }));
        }

        if side == Some(ReactionSide::Bottom) {
            if let Some(reactions) = control.reactions.take() {
                body = body.push(reactions);
            }
        }

        let body = body.width(if surface.is_ghost() {
            Length::Fill
        } else {
            Length::Shrink
        });

        // The avatar sits outside the surface, at the bubble's own edge: before
        // the surface at the leading edge, after it at the trailing one.
        let avatar = control.avatar;
        let mut frame = row![]
            .spacing(theme::SPACING_SM)
            .align_y(IcedAlignment::Start);
        if alignment.is_end() {
            frame = frame.push(body);
            if let Some(avatar) = avatar {
                frame = frame.push(avatar);
            }
        } else {
            if let Some(avatar) = avatar {
                frame = frame.push(avatar);
            }
            frame = frame.push(body);
        }

        container(frame).width(Length::Fill).align_x(axis).into()
    }
}

/// A vertical stack of consecutive bubbles from one sender. It infers no sender
/// identity and removes no headers; the application decides which bubbles group.
pub struct Group<'a, Message> {
    alignment: Alignment,
    children: Vec<Element<'a, Message>>,
}

impl<'a, Message> Group<'a, Message> {
    /// An empty group, aligned to the leading edge.
    pub fn new() -> Self {
        Self {
            alignment: Alignment::default(),
            children: Vec::new(),
        }
    }

    /// Align the whole stack to an edge, Slint's `BubbleGroup.align`.
    pub fn align(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Add a consecutive bubble.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message> Default for Group<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Group<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Group<'a, Message>) -> Self {
        let mut stack = column![]
            .spacing(theme::SPACING_SM)
            .width(Length::Fill)
            .align_x(control.alignment.axis());

        for child in control.children {
            stack = stack.push(child);
        }

        stack.into()
    }
}

/// A reaction region: a pill of controls attached to a bubble edge.
///
/// The children are plain elements. An action is a `super::button::Button`, and
/// any other child is whatever the application wants in the pill — a label, a
/// count, or an overlay trigger. Because there is no typed action here, the
/// compact radius treatment GPUI gives `.action(Button)` has no counterpart.
pub struct Reactions<'a, Message> {
    alignment: Alignment,
    side: ReactionSide,
    children: Vec<Element<'a, Message>>,
}

impl<'a, Message> Reactions<'a, Message> {
    /// A trailing-aligned region below the surface, the default in both
    /// references.
    pub fn new() -> Self {
        Self {
            alignment: Alignment::End,
            side: ReactionSide::default(),
            children: Vec::new(),
        }
    }

    /// Attach the region above or below the surface.
    pub fn side(mut self, side: ReactionSide) -> Self {
        self.side = side;
        self
    }

    /// Align the pill along the bubble edge.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Add a control to the pill.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message> Default for Reactions<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Reactions<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Reactions<'a, Message>) -> Self {
        // The pill: the muted fill, the three-pixel background ring and the full
        // radius Slint derives from the pill's own height. iced clamps a border
        // radius to half the smaller side, so a large value is a pill rather than
        // a value this code has to know the height to compute.
        let pill = container(
            row(control.children)
                .spacing(theme::SPACING_XS)
                .align_y(IcedAlignment::Center),
        )
        .padding([theme::SPACING_XS, theme::SPACING_SM])
        .style(|theme: &Theme| {
            let tokens = Tokens::of(theme);
            ContainerStyle {
                background: Some(Background::Color(tokens.muted)),
                text_color: Some(tokens.foreground),
                border: Border {
                    color: tokens.background,
                    width: 3.0,
                    radius: 999.0.into(),
                },
                ..ContainerStyle::default()
            }
        });

        // The row keeps the pill content-sized, inset twelve pixels from the
        // bubble edge it hangs from. The body column above already contributes a
        // 4-pixel gap, so the row adds the remaining two to reach Slint's 6.
        let pad = REACTION_GAP - TIME_GAP;
        let inset = if control.alignment.is_end() {
            0.0
        } else {
            REACTION_INSET
        };
        let trailing = if control.alignment.is_end() {
            REACTION_INSET
        } else {
            0.0
        };

        container(pill)
            .width(Length::Fill)
            .align_x(control.alignment.axis())
            .padding(Padding {
                top: pad,
                right: trailing,
                bottom: pad,
                left: inset,
            })
            .into()
    }
}
