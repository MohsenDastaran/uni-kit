//! A styled scroll area. Mirrors GPUI Component's `Scrollable` and the Slint
//! version beside it: the same three axes — vertical, horizontal and both — a
//! bar that floats over the content and stays thin, and a thumb that darkens
//! under the pointer.
//!
//! **iced owns the scroll region.** `iced::widget::scrollable` measures the
//! content, clips it, answers the wheel and the keys, and draws the bars; this
//! file is the colours and the geometry over it. A caller that wants the offset
//! asks for it with [`Scrollable::on_scroll`], which iced reports as a
//! [`Viewport`]; the Slint component exposes the same numbers as `viewport-x`,
//! `viewport-y` and the two viewport lengths.
//!
//! Everything the bars are drawn with comes from [`Tokens`], read inside the
//! style closure while the region is rendered, so a theme switch repaints them.
//! Nothing here writes a colour of its own: [`Ink`] names the token to draw a
//! part from, and the default is the Slint component's own choice —
//! `muted-foreground` at 0.35, rising to 0.6 while that bar is hovered or
//! dragged.
//!
//! Four things iced cannot do the way the reference does. None are faked:
//!
//! - **The bar cannot widen on hover.** iced fixes a scrollbar's width when the
//!   element is built, and its style function answers with colours only. Slint
//!   animates its 6 px bar to 8 px; here the thumb's colour carries the state,
//!   and [`Scrollable::scrollbar_width`] / [`Scrollable::scroller_width`] pick
//!   the thickness a caller wants instead.
//! - **There is no virtual list.** `scrollable` lays out and draws every child
//!   it is given, on every pass. [`Scrollable::on_scroll`] hands over the
//!   viewport, which is the primitive windowing is built on, but this component
//!   ships no windowing of its own; the gallery's `Virtualization` section says
//!   so rather than inventing an API for it.
//! - **There is no `ScrollbarMode`.** GPUI's *scrolling*, *hover* and *always*
//!   are [`Visibility`], and only the *hover* one is an appearance this port can
//!   honour: the mode chooses whether the thumb is painted while the bar is at
//!   rest. There is no fade, because a style function has no animation to hang
//!   one on.
//! - **The track is not drawn by default.** Slint paints no track behind its bar,
//!   so neither does this; [`Scrollable::track`] asks for one.
//!
//! iced draws a bar only while its content overflows that axis, and a thumb only
//! while the content is longer than the viewport, which is the Slint component's
//! `if overflow > 0` written once in iced instead of twice here.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one axis gets
// a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::scrollable::{Direction, Rail, Scrollbar, Scroller, Viewport};
use iced::widget::{container, scrollable};
use iced::{Background, Border, Color, Element, Length, Shadow, Theme, Vector};

use super::theme::{self, Tokens};

/// The bar's total width. iced centres the thumb in the bar, so 10 − 6 leaves
/// the 2 px gap at the edge that the Slint component draws its bar with.
pub const SCROLLBAR_WIDTH: f32 = 10.0;

/// The thumb's width. Slint draws its resting bar 6 px wide and widens it on
/// hover; iced fixes the width, so the resting width is the only one.
pub const SCROLLER_WIDTH: f32 = 6.0;

/// Which way a scroll area may overflow, the Slint `ScrollAxis`: a vertical area
/// pins its viewport width, a horizontal one its height, and `Both` lets each
/// axis scroll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis {
    /// Content taller than the area scrolls up and down.
    #[default]
    Vertical,
    /// Content wider than the area scrolls left and right.
    Horizontal,
    /// Both viewports may overflow.
    Both,
}

/// When the thumb is shown. The Slint bar is drawn whenever the axis overflows
/// and darkens under the pointer, which is [`Visibility::Always`] here; GPUI's
/// `ScrollbarMode` names the other two.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
    /// The thumb is drawn whenever the axis overflows, at its resting ink.
    #[default]
    Always,
    /// The thumb is drawn only while the bar is hovered or dragged. GPUI's
    /// `ScrollbarMode::Hover`, and the closest iced comes to `Scrolling`: the
    /// paint is gated on the pointer, not on a scroll that has since stopped.
    Hover,
    /// No bar at all. The area still scrolls with the wheel and the keyboard,
    /// which is what iced's zero-width scrollbar keeps.
    Hidden,
}

/// Which token a part of the bar is drawn from. The colour is resolved inside the
/// style closure, out of the tokens in force while the region renders, so an
/// application that names a token still repaints on a theme switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Ink {
    /// Nothing is drawn: a rail that is not painted, or a thumb that is not.
    None,
    /// The Slint component's own choice: `muted-foreground` at 0.35, and at 0.6
    /// while the bar is hovered or dragged.
    #[default]
    Muted,
    /// The page's ink, on the same ladder.
    Foreground,
    /// The primary token, on the same ladder.
    Primary,
    /// The border token, whole. A track that reads as an inset rail.
    Border,
}

impl Ink {
    /// The colour this part is drawn in, for the tokens in force. `lit` is true
    /// while the bar is hovered or dragged, which is the one state iced reports
    /// that a style function can still answer.
    pub fn color(self, tokens: Tokens, lit: bool) -> Option<Color> {
        let weight = if lit { 0.6 } else { 0.35 };
        match self {
            Self::None => None,
            Self::Muted => Some(theme::alpha(tokens.muted_foreground, weight)),
            Self::Foreground => Some(theme::alpha(tokens.foreground, weight)),
            Self::Primary => Some(theme::alpha(tokens.primary, weight)),
            Self::Border => Some(tokens.border),
        }
    }
}

/// The bar's appearance and geometry: everything about how the region looks and
/// scrolls, with the content and the region itself belonging to [`Scrollable`].
///
/// It is `pub` so an application can reuse it for its own `scrollable` and get
/// the same look, the way the button's [`Style`](super::button::Style) is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub axis: Axis,
    pub visibility: Visibility,
    pub track: Ink,
    pub thumb: Ink,
    pub scrollbar_width: f32,
    pub scroller_width: f32,
    /// The rail's inset from the content, when it is embedded rather than
    /// floating. iced honours it for a single axis and ignores it for `Both`.
    pub spacing: Option<f32>,
}

impl Default for Style {
    fn default() -> Self {
        Self::new(Axis::default())
    }
}

impl Style {
    pub fn new(axis: Axis) -> Self {
        Self {
            axis,
            visibility: Visibility::default(),
            track: Ink::None,
            thumb: Ink::Muted,
            scrollbar_width: SCROLLBAR_WIDTH,
            scroller_width: SCROLLER_WIDTH,
            spacing: None,
        }
    }

    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// The token the rail is drawn from. `Ink::None` — the default — paints no
    /// track, which is what the Slint bar does.
    pub fn track(mut self, ink: Ink) -> Self {
        self.track = ink;
        self
    }

    /// The token the thumb is drawn from.
    pub fn thumb(mut self, ink: Ink) -> Self {
        self.thumb = ink;
        self
    }

    /// How much of the area's edge the bar claims, thumb included.
    pub fn scrollbar_width(mut self, width: f32) -> Self {
        self.scrollbar_width = width.max(0.0);
        self
    }

    /// The thumb's own width, at most the bar's width. iced centres it, so the
    /// difference is the gap at the edge.
    pub fn scroller_width(mut self, width: f32) -> Self {
        self.scroller_width = width.max(0.0);
        self
    }

    /// Embeds the rail into the area, with this space between it and the
    /// content, instead of floating it over the content. iced allows it on a
    /// single axis, so `Axis::Both` ignores it.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = Some(spacing.max(0.0));
        self
    }

    /// The scrollbar geometry iced lays the region out with.
    pub fn direction(&self) -> Direction {
        let vertical = self.scrollbar();
        let horizontal = self.scrollbar();

        match self.axis {
            Axis::Vertical => Direction::Vertical(vertical),
            Axis::Horizontal => Direction::Horizontal(horizontal),
            Axis::Both => Direction::Both {
                vertical,
                horizontal,
            },
        }
    }

    /// One bar, with the visibility, the width and the spacing this style asks
    /// for.
    fn scrollbar(&self) -> Scrollbar {
        if self.visibility == Visibility::Hidden {
            // Zero width, so iced reserves no space and draws nothing, but the
            // axis still scrolls.
            return Scrollbar::hidden();
        }

        let width = self.scrollbar_width.max(0.0);
        let bar = Scrollbar::new()
            .width(width)
            .scroller_width(self.scroller_width.max(0.0).min(width));

        match self.spacing {
            Some(spacing) => bar.spacing(spacing),
            None => bar,
        }
    }

    /// The colours iced draws the bars with, resolved from the tokens in force.
    /// This is the body of the style function the region is built with.
    pub fn resolve(&self, tokens: Tokens, status: scrollable::Status) -> scrollable::Style {
        // iced reports each bar's own state; a drag counts as lit too, which is
        // the Slint component's `pressed || has-hover`.
        let (horizontal_lit, vertical_lit) = match status {
            scrollable::Status::Active { .. } => (false, false),
            scrollable::Status::Hovered {
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
                ..
            } => (
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
            ),
            scrollable::Status::Dragged {
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
                ..
            } => (
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
            ),
        };

        scrollable::Style {
            container: container::Style::default(),
            vertical_rail: self.rail(tokens, vertical_lit),
            horizontal_rail: self.rail(tokens, horizontal_lit),
            // The corner where the two bars meet. The Slint component leaves it
            // clear by shortening the vertical bar; iced draws the gap only when
            // a track was asked for, which is the same answer.
            gap: self.track.color(tokens, false).map(Background::Color),
            auto_scroll: scrollable::AutoScroll {
                background: tokens.background.into(),
                border: Border {
                    radius: theme::RADIUS_XL.into(),
                    width: 1.0,
                    color: theme::alpha(tokens.foreground, 0.8),
                },
                shadow: Shadow {
                    color: theme::alpha(tokens.shadow, 1.0),
                    offset: Vector::ZERO,
                    blur_radius: 2.0,
                },
                icon: theme::alpha(tokens.foreground, 0.8),
            },
        }
    }

    /// One rail: the track behind, the thumb in front.
    fn rail(&self, tokens: Tokens, lit: bool) -> Rail {
        Rail {
            background: self.track.color(tokens, lit).map(Background::Color),
            border: Border::default(),
            scroller: Scroller {
                // A pill, the Slint thumb's `border-radius: width / 2`.
                border: Border {
                    radius: (self.scroller_width.max(0.0) / 2.0).into(),
                    ..Border::default()
                },
                background: self
                    .thumb_color(tokens, lit)
                    .map_or(Background::Color(Color::TRANSPARENT), Background::Color),
            },
        }
    }

    /// The thumb's colour, with `Visibility::Hover` withholding it until the bar
    /// is lit.
    fn thumb_color(&self, tokens: Tokens, lit: bool) -> Option<Color> {
        match self.visibility {
            Visibility::Hover if !lit => None,
            _ => self.thumb.color(tokens, lit),
        }
    }
}

/// A scroll area, built with the methods below.
///
/// The content is whatever the caller places inside it; the region fills its
/// parent by default, which is the Slint component's `horizontal-stretch` and
/// `preferred-height: 100%`. A scroll area therefore needs a bounded parent — an
/// explicit [`Scrollable::height`] for a vertical one, [`Scrollable::width`] for
/// a horizontal one — or there is nothing for the content to overflow.
pub struct Scrollable<'a, Message> {
    content: Element<'a, Message>,
    style: Style,
    id: Option<iced::widget::Id>,
    width: Length,
    height: Length,
    padding: f32,
    on_scroll: Option<Box<dyn Fn(Viewport) -> Message + 'a>>,
}

impl<'a, Message> Scrollable<'a, Message> {
    /// A vertical scroll area over `content`.
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
            id: None,
            width: Length::Fill,
            height: Length::Fill,
            padding: 0.0,
            on_scroll: None,
        }
    }

    /// Replaces the appearance and the geometry wholesale.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn axis(mut self, axis: Axis) -> Self {
        self.style.axis = axis;
        self
    }

    /// Names the region, so an application can move it with iced's own
    /// `scroll_to` and `scroll_by` operations. Two areas sharing an id move
    /// together.
    pub fn id(mut self, id: impl Into<iced::widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// The inset around the content, inside the scroll region. The content
    /// scrolls with its padding, the way the Slint examples put a layout's own
    /// padding inside the `Scrollable`.
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding.max(0.0);
        self
    }

    /// When the thumb is drawn. See [`Visibility`].
    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.style.visibility = visibility;
        self
    }

    /// The token the rail is drawn from. See [`Ink`].
    pub fn track(mut self, ink: Ink) -> Self {
        self.style.track = ink;
        self
    }

    /// The token the thumb is drawn from. See [`Ink`].
    pub fn thumb(mut self, ink: Ink) -> Self {
        self.style.thumb = ink;
        self
    }

    /// How much of the area's edge the bar claims, thumb included.
    pub fn scrollbar_width(mut self, width: f32) -> Self {
        self.style.scrollbar_width = width.max(0.0);
        self
    }

    /// The thumb's own width, at most the bar's width.
    pub fn scroller_width(mut self, width: f32) -> Self {
        self.style.scroller_width = width.max(0.0);
        self
    }

    /// Embeds the rail with this space between it and the content, instead of
    /// floating it over the content. iced honours it on a single axis only.
    pub fn scrollbar_spacing(mut self, spacing: f32) -> Self {
        self.style.spacing = Some(spacing.max(0.0));
        self
    }

    /// Called with the [`Viewport`] whenever the region scrolls. This is the
    /// scroll tracking the Slint component exposes as its `viewport-*`
    /// properties, and the primitive a windowed list is built on.
    pub fn on_scroll(mut self, f: impl Fn(Viewport) -> Message + 'a) -> Self {
        self.on_scroll = Some(Box::new(f));
        self
    }
}

impl<'a, Message> From<Scrollable<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Scrollable<'a, Message>) -> Self {
        let style = control.style;
        let direction = style.direction();

        let content: Element<'a, Message> = if control.padding > 0.0 {
            container(control.content).padding(control.padding).into()
        } else {
            control.content
        };

        let mut region = scrollable(content)
            .direction(direction)
            .width(control.width)
            .height(control.height)
            .style(move |theme: &Theme, status| style.resolve(Tokens::of(theme), status));

        if let Some(id) = control.id {
            region = region.id(id);
        }

        if let Some(on_scroll) = control.on_scroll {
            region = region.on_scroll(on_scroll);
        }

        region.into()
    }
}
