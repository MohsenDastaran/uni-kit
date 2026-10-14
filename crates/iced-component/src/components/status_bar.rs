//! A window footer. Mirrors GPUI Component's `StatusBar` and the Slint
//! `status-bar.slint` beside it: a left region, a centre and a right region, at
//! the bottom of a window or pane, holding short readouts and quick actions.
//!
//! [`StatusBar::left`] and [`StatusBar::right`] pin their items to the two
//! edges; [`StatusBar::child`] adds to the centre, whose alignment follows the
//! pinned ends — centered with both, end-aligned with only a left region,
//! start-aligned otherwise (only a right region, or neither), which is what the
//! Slint component's `space-between` does with the groups that are present, and
//! what GPUI's `has_left`/`has_right` pair does. A bar with no regions behaves
//! like a plain container.
//!
//! A region takes any element. [`StatusItem`] is the translation of Slint's
//! `StatusItem` + `Segment` for the three segments the reference names: a
//! read-only **label** in muted ink, a pressable **button** that is the shared
//! [`Button`](super::button::Button) in its ghost variant, and a **separator**,
//! the vertical rule between runs. A caller that wants a segment this file does
//! not name — a `Popover` around a `Menu`, say — passes the element straight in
//! or wraps it in [`StatusItem::custom`], so nothing here invents a control the
//! catalog already has.
//!
//! # What iced cannot draw, named rather than faked
//!
//! - **The icon is the caller's element.** Slint names a glyph with a string and
//!   resolves it against a shipped set; iced ships none, so [`StatusItem::icon`]
//!   takes the caller's element. A widget cannot recolour a child element, so the
//!   caller also chooses the ink — `foreground` for a button's glyph,
//!   `muted_foreground` for a label's — exactly as the Slint `Segment` colours
//!   its own two cases.
//! - **A label's segment surface is transparent and takes no hover.** Slint's
//!   `TouchArea` is `enabled: interactive`, so only a button segment tints; here
//!   the label draws no surface and the ghost button brings its own accent.
//! - **No rounded clip.** The Slint example clips the bar to the panel's radius;
//!   iced 0.14 has none, so a bar flush to a rounded corner squares that corner.
//! - **No accessible name.** iced exposes no accessibility API, so the bar adds
//!   none; a caller that needs one names the control it composes.
//!
//! The registry key is `status-bar`; this file and its module are `status_bar`,
//! because a hyphen is not a Rust identifier.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one builder
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Color, Element, Length, Theme};

use super::button::{Button, Variant};
use super::theme::{self, ControlSize, Tokens};

/// The bar's height, Slint's `height: 24px`.
pub const BAR_HEIGHT: f32 = 24.0;

/// The inset from the bar's leading and trailing edges, Slint's
/// `padding-left/right: 8px`.
pub const BAR_PADDING: f32 = 8.0;

/// The gap between two segments inside one region, Slint's `spacing: 6px`.
pub const ITEM_SPACING: f32 = 6.0;

/// The gap between the three regions, Slint's outer `spacing: 8px`.
pub const REGION_SPACING: f32 = 8.0;

/// A separator's height, Slint's `height: 12px`.
pub const SEPARATOR_HEIGHT: f32 = 12.0;

/// A label segment's own inset, Slint's `padding-left/right: 6px`.
const LABEL_PADDING: f32 = 6.0;

/// A label segment's height, Slint's `height: 16px`.
const LABEL_HEIGHT: f32 = 16.0;

/// Between a segment's glyph and its word, Slint's `spacing: 4px`.
const ICON_SPACING: f32 = 4.0;

/// One segment of a status bar, built with the methods below and passed to
/// [`StatusBar::left`] or [`StatusBar::right`].
///
/// This is Slint's `StatusItem` and the `Segment` that draws it. The three
/// named shapes are a read-only label, a pressable button and a separator;
/// [`StatusItem::custom`] is the escape hatch for anything else, so a caller
/// composes the catalog's own controls rather than a second shape here. A region
/// also takes a bare element — a [`StatusItem`] whose shape is right, or a
/// [`Container`](iced::widget::container) around a `Popover` the caller built —
/// because both sides convert to an [`Element`].
pub struct StatusItem<'a, Message> {
    icon: Option<Element<'a, Message>>,
    label: String,
    segment: Segment<'a, Message>,
}

/// What a [`StatusItem`] actually draws.
enum Segment<'a, Message> {
    /// Read-only: muted ink, no hover, no surface.
    Label,
    /// Pressable: a ghost [`Button`], so the hover accent and the ink are the
    /// button's own, and the message is what a press reports.
    Button { on_press: Option<Message> },
    /// The vertical rule between two runs.
    Separator,
    /// Anything else the caller built, drawn as it is.
    Custom(Element<'a, Message>),
}

impl<'a, Message> StatusItem<'a, Message> {
    /// A read-only segment, in the bar's muted ink.
    pub fn label(label: impl Into<String>) -> Self {
        Self {
            icon: None,
            label: label.into(),
            segment: Segment::Label,
        }
    }

    /// A pressable segment: a ghost button at the extra-small size, the
    /// `.ghost().xsmall()` the GPUI page tells a caller to pass. Without
    /// [`StatusItem::on_press`] it draws but does not answer, which is iced's
    /// "cannot be pressed".
    pub fn button(label: impl Into<String>) -> Self {
        Self {
            icon: None,
            label: label.into(),
            segment: Segment::Button { on_press: None },
        }
    }

    /// The vertical rule between two runs, the `border` token one pixel wide and
    /// [`SEPARATOR_HEIGHT`] tall.
    pub fn separator() -> Self {
        Self {
            icon: None,
            label: String::new(),
            segment: Segment::Separator,
        }
    }

    /// A segment the caller built: any element, kept as it is.
    pub fn custom(item: impl Into<Element<'a, Message>>) -> Self {
        Self {
            icon: None,
            label: String::new(),
            segment: Segment::Custom(item.into()),
        }
    }

    /// A leading glyph, drawn before the label. The caller owns the element and
    /// therefore its colour — see the module comment.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// What a press of this segment reports. It applies to a button; on the
    /// other shapes it is ignored, because they do not answer input.
    pub fn on_press(mut self, message: Message) -> Self {
        if let Segment::Button { on_press } = &mut self.segment {
            *on_press = Some(message);
        }
        self
    }
}

impl<'a, Message: Clone + 'a> From<StatusItem<'a, Message>> for Element<'a, Message> {
    fn from(item: StatusItem<'a, Message>) -> Self {
        match item.segment {
            // A rule is a one-pixel column, and the region's cross-axis centring
            // places it in the taller row.
            Segment::Separator => container(
                Space::new()
                    .width(Length::Fixed(1.0))
                    .height(Length::Fixed(SEPARATOR_HEIGHT)),
            )
            .style(|theme: &Theme| container::Style {
                background: Some(Tokens::of(theme).border.into()),
                ..container::Style::default()
            })
            .into(),

            Segment::Label => {
                let mut content = row![].spacing(ICON_SPACING).align_y(Alignment::Center);
                if let Some(icon) = item.icon {
                    content = content.push(icon);
                }
                if !item.label.is_empty() {
                    // Resolved inside the closure, never from the theme at build
                    // time: a colour read before the element is drawn would keep
                    // the tokens that were in force when `view` ran.
                    content = content.push(text(item.label).size(theme::FONT_SIZE_XS).style(
                        |theme: &Theme| text::Style {
                            color: Some(Tokens::of(theme).muted_foreground),
                        },
                    ));
                }
                container(content)
                    .height(Length::Fixed(LABEL_HEIGHT))
                    .center_y(Length::Fixed(LABEL_HEIGHT))
                    .padding([0.0, LABEL_PADDING])
                    .into()
            }

            Segment::Button { on_press } => {
                let mut control = Button::new(item.label)
                    .variant(Variant::Ghost)
                    .size(ControlSize::XSmall);
                if let Some(icon) = item.icon {
                    control = control.icon(icon);
                }
                if let Some(message) = on_press {
                    control = control.on_press(message);
                }
                control.into()
            }

            Segment::Custom(element) => element,
        }
    }
}

/// A window footer, built with the methods below. See the module comment for the
/// three regions and how the centre aligns.
pub struct StatusBar<'a, Message> {
    left: Vec<Element<'a, Message>>,
    center: Vec<Element<'a, Message>>,
    right: Vec<Element<'a, Message>>,
    background: Option<Rc<dyn Fn(&Theme) -> Color + 'a>>,
    edge: Option<Rc<dyn Fn(&Theme) -> Color + 'a>>,
}

impl<'a, Message> StatusBar<'a, Message> {
    /// An empty bar. Its surface is the `sidebar` token with a `border` edge,
    /// which is the Slint component's own default.
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            center: Vec::new(),
            right: Vec::new(),
            background: None,
            edge: None,
        }
    }

    /// The bar's fill, resolved when it is drawn rather than when it is built —
    /// the Slint `background` property, and GPUI's `bg` style method. `sidebar`
    /// by default.
    pub fn background(mut self, background: impl Fn(&Theme) -> Color + 'a) -> Self {
        self.background = Some(Rc::new(background));
        self
    }

    /// The colour of the bar's top edge, resolved when it is drawn. `border` by
    /// default. The edge is the divider between the bar and the content above
    /// it, so it belongs to the bar and only there — GPUI's `border_t_1`. The
    /// Slint file draws a box on all four sides; a footer does not, because the
    /// window or panel it sits in already owns the other three.
    pub fn border_color(mut self, border_color: impl Fn(&Theme) -> Color + 'a) -> Self {
        self.edge = Some(Rc::new(border_color));
        self
    }
}

impl<'a, Message: Clone + 'a> StatusBar<'a, Message> {
    /// Append an item to the leading region. Call multiple times to add more. A
    /// [`StatusItem`] is the usual argument; any element is kept as it is.
    pub fn left(mut self, item: impl Into<Element<'a, Message>>) -> Self {
        self.left.push(item.into());
        self
    }

    /// Append an item to the trailing region. Call multiple times to add more.
    pub fn right(mut self, item: impl Into<Element<'a, Message>>) -> Self {
        self.right.push(item.into());
        self
    }

    /// Append an item to the centre region, which is where a progress bar or a
    /// custom readout goes. Call multiple times to add more.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.center.push(child.into());
        self
    }

    /// Append centre items, in source order.
    pub fn children<I, E>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Element<'a, Message>>,
    {
        for child in children {
            self = self.child(child);
        }
        self
    }
}

impl<'a, Message> Default for StatusBar<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: Clone + 'a> From<StatusBar<'a, Message>> for Element<'a, Message> {
    fn from(control: StatusBar<'a, Message>) -> Self {
        let has_left = !control.left.is_empty();
        let has_right = !control.right.is_empty();

        // One region is a run with a tighter gap than the bar's own, so a run
        // reads as one unit; a region that holds nothing draws nothing.
        let region = |items: Vec<Element<'a, Message>>| -> Element<'a, Message> {
            let mut run = row![].spacing(ITEM_SPACING).align_y(Alignment::Center);
            for item in items {
                run = run.push(item);
            }
            run.into()
        };

        let left = region(control.left);
        let center = region(control.center);
        let right = region(control.right);
        let gap = || Space::new().width(Length::Fill);

        // The centre's alignment follows the pinned ends: centered with both,
        // end-aligned with only a left region, start-aligned otherwise — the
        // Slint `space-between`, and GPUI's own pair of flags.
        let mut body = row![]
            .spacing(REGION_SPACING)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([0.0, BAR_PADDING]);
        match (has_left, has_right) {
            (true, true) => {
                body = body
                    .push(left)
                    .push(gap())
                    .push(center)
                    .push(gap())
                    .push(right);
            }
            (true, false) => body = body.push(left).push(gap()).push(center),
            (false, true) => body = body.push(center).push(gap()).push(right),
            (false, false) => body = body.push(center),
        }

        let background = control.background;
        let edge = control.edge;

        // The top edge is a one-pixel rule above the row, inside the bar's own
        // height, which is how a footer divides itself from the content above it
        // without boxing the sides the window already owns.
        let rule = {
            let edge = edge.clone();
            container(Space::new().height(Length::Fixed(1.0)))
                .width(Length::Fill)
                // Resolved inside the closure, never at build time: a colour read
                // while `view` ran would keep the tokens in force then.
                .style(move |theme: &Theme| container::Style {
                    background: Some(
                        edge.as_ref()
                            .map_or(Tokens::of(theme).border, |edge| edge(theme))
                            .into(),
                    ),
                    ..container::Style::default()
                })
        };

        container(column![rule, body])
            .width(Length::Fill)
            .height(Length::Fixed(BAR_HEIGHT))
            // The fill is resolved here too, for the same reason.
            .style(move |theme: &Theme| container::Style {
                background: Some(
                    background
                        .as_ref()
                        .map_or(Tokens::of(theme).sidebar, |background| background(theme))
                        .into(),
                ),
                ..container::Style::default()
            })
            .into()
    }
}
