//! A transparent, sizable row of commands. Mirrors GPUI Component's `Toolbar`
//! and the Slint `toolbar.slint` beside it: items render in source order, a
//! separator divides a run, a group holds related controls together, and a
//! spacer pushes a trailing run to the far edge.
//!
//! The bar owns layout and no surface. It draws no background and no border, so
//! the header, tab strip, or custom command surface it drops into supplies them
//! — the Slint `Toolbar` and GPUI's are transparent for the same reason.
//!
//! # The size is the bar's, and the caller sizes the controls
//!
//! Slint and GPUI both hand the bar's size down to the controls inside it, GPUI
//! through a `Sizable` trait that reaches into any child. iced 0.14 has no such
//! seam — a `Button` is an `Element`, not a value the bar can resize — so the
//! bar's [`Toolbar::size`] sets its own row height and [`Toolbar::child`] pins
//! each command's slot to that height, while every control keeps the size its
//! caller gave it. The demo sizes its buttons to the bar's, which is the
//! caller's half of the contract; this is named rather than faked by wrapping
//! every child in a resizer the element does not have.
//!
//! # What iced cannot draw, named rather than faked
//!
//! - **No accessible group name.** GPUI's `ToolbarGroup` and Slint's
//!   `accessible-label` give a run of controls one name for assistive
//!   technology. iced 0.14 exposes no accessibility API, so [`ToolbarGroup`]
//!   holds the run together and invents no label it cannot deliver.
//! - **No roving keyboard focus.** GPUI's toolbar owns arrow-key traversal
//!   along the bar; iced puts keyboard focus on individual widgets and a bare
//!   element has no focus order of its own, so the caller's controls keep their
//!   own keyboard behavior and the bar adds none.
//! - **The overflow menu is the caller's.** The reference ends a bar with a
//!   spacer and an icon button and leaves what that button opens to the
//!   application; a caller composes [`Button`](super::button::Button),
//!   [`Popover`](super::popover::Popover) and [`Menu`](super::menu::Menu) for
//!   it, as `demos/toolbar.rs` does.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one builder
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{Space, container, row};
use iced::{Alignment, Element, Length, Theme};

use super::theme::{self, ControlSize, Tokens};

/// The rule's height in a toolbar, Slint's `bar-height: 18px`. A toolbar's
/// controls are compact, so a full-height divider would read as a column
/// boundary rather than a mark between two runs of the same row.
pub const SEPARATOR_HEIGHT: f32 = 18.0;

/// The gap between a group's controls, Slint's `spacing: 4px` inside a
/// `ToolbarGroup`. The bar's own spacing is wider so the group reads as one run.
pub const GROUP_SPACING: f32 = theme::SPACING_XS;

/// A transparent horizontal container for commands in a header, tab strip, or
/// custom surface, built with the methods below.
///
/// `child` pins a command's slot to the bar's height and centers it there;
/// `content` adds an element that keeps its own dimensions — a
/// [`ToolbarSpacer`], a label, or layout the caller has already built. Items
/// render in source order.
pub struct Toolbar<'a, Message> {
    size: ControlSize,
    spacing: f32,
    inset: f32,
    items: Vec<Element<'a, Message>>,
}

impl<'a, Message: 'a> Toolbar<'a, Message> {
    /// An empty bar at [`ControlSize::Small`], the Slint and GPUI default, with
    /// Slint's 4px gap and no inset.
    pub fn new() -> Self {
        Self {
            size: ControlSize::Small,
            spacing: theme::SPACING_XS,
            inset: 0.0,
            items: Vec::new(),
        }
    }

    /// The bar's height: the size's control height plus the inset twice. The
    /// caller sizes the controls inside it to match — see the module comment.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The gap between items. A separator draws its own rule and wants less
    /// room than a command, which is why Slint's default is one step tighter
    /// than the menu's.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// The inset from the surface. A toolbar drops into a header that already
    /// has its own padding, so this defaults to none.
    pub fn inset(mut self, inset: f32) -> Self {
        self.inset = inset;
        self
    }

    /// Append a command. Its slot is the bar's control height and its content
    /// is centered there, so a short control lines up with the words beside it
    /// — GPUI's `ToolbarItem::into_element`.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        let slot = slot(child.into(), self.size.control_height());
        self.items.push(slot);
        self
    }

    /// Append commands, in source order.
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

    /// Append non-sized content: a [`ToolbarSeparator`], a [`ToolbarSpacer`], a
    /// label, or custom layout that should keep its own dimensions.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.items.push(content.into());
        self
    }

    /// Append non-sized content, in source order.
    pub fn contents<I, E>(mut self, contents: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<Element<'a, Message>>,
    {
        for content in contents {
            self = self.content(content);
        }
        self
    }
}

impl<'a, Message: 'a> Default for Toolbar<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// A run of related controls, held together with a tighter gap than the bar's
/// own spacing so it reads as one unit.
///
/// The group carries no accessible name: GPUI's `ToolbarGroup` and Slint's
/// `accessible-label` name the run for assistive technology, and iced 0.14 has
/// no accessibility API for that name to reach. Named, not invented.
pub struct ToolbarGroup<'a, Message> {
    size: ControlSize,
    spacing: f32,
    items: Vec<Element<'a, Message>>,
}

impl<'a, Message: 'a> ToolbarGroup<'a, Message> {
    /// An empty group at the bar's default size.
    pub fn new() -> Self {
        Self {
            size: ControlSize::Small,
            spacing: GROUP_SPACING,
            items: Vec::new(),
        }
    }

    /// The size the group's command slots take. A group sits inside a bar, so
    /// the caller matches this to the bar's own size.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The gap between the group's controls, [`GROUP_SPACING`] by default.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Append a control, as [`Toolbar::child`] does.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        let slot = slot(child.into(), self.size.control_height());
        self.items.push(slot);
        self
    }

    /// Append controls, in source order.
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

    /// Append non-sized content, as [`Toolbar::content`] does.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.items.push(content.into());
        self
    }
}

impl<'a, Message: 'a> Default for ToolbarGroup<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The vertical rule between command groups, Slint's `ToolbarSeparator`.
///
/// The rule is the `border` token, which is an edge in both themes; a fixed
/// grey would vanish in one of them.
pub struct ToolbarSeparator {
    height: f32,
}

impl ToolbarSeparator {
    /// A rule at [`SEPARATOR_HEIGHT`].
    pub fn new() -> Self {
        Self {
            height: SEPARATOR_HEIGHT,
        }
    }

    /// The rule's height, `18px` by default.
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl Default for ToolbarSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: 'a> From<ToolbarSeparator> for Element<'a, Message> {
    fn from(separator: ToolbarSeparator) -> Self {
        // The rule is a one-pixel column, and the bar's own cross-axis
        // alignment centers it in a taller row.
        container(Space::new().width(Length::Fixed(1.0)).height(Length::Fixed(separator.height)))
            .style(|theme: &Theme| container::Style {
                background: Some(Tokens::of(theme).border.into()),
                ..container::Style::default()
            })
            .into()
    }
}

/// A pushed group: everything after it sits at the trailing edge, Slint's
/// `ToolbarSpacer`. A layout with unclaimed space spreads its children apart,
/// so a spacer is what keeps the leading commands together while a trailing
/// run still reaches the far end.
pub struct ToolbarSpacer;

impl ToolbarSpacer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ToolbarSpacer {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: 'a> From<ToolbarSpacer> for Element<'a, Message> {
    fn from(_: ToolbarSpacer) -> Self {
        Space::new().width(Length::Fill).height(Length::Shrink).into()
    }
}

/// A command's slot: a fixed-height box with its content centered in it. GPUI's
/// `ToolbarItem::into_element` does the same, which is what lines an icon up
/// with the words beside it instead of letting each control find its own
/// vertical position.
fn slot<'a, Message: 'a>(item: Element<'a, Message>, height: f32) -> Element<'a, Message> {
    container(item)
        .height(Length::Fixed(height))
        .center_y(Length::Fixed(height))
        .into()
}

impl<'a, Message: 'a> From<Toolbar<'a, Message>> for Element<'a, Message> {
    fn from(control: Toolbar<'a, Message>) -> Self {
        let inset = control.inset;
        let height = control.size.control_height() + inset * 2.0;

        let mut bar = row![]
            .spacing(control.spacing)
            .align_y(Alignment::Center)
            .width(Length::Fill);
        for item in control.items {
            bar = bar.push(item);
        }

        container(bar)
            .width(Length::Fill)
            .height(Length::Fixed(height))
            .padding(inset)
            .into()
    }
}

impl<'a, Message: 'a> From<ToolbarGroup<'a, Message>> for Element<'a, Message> {
    fn from(group: ToolbarGroup<'a, Message>) -> Self {
        let mut run = row![]
            .spacing(group.spacing)
            .align_y(Alignment::Center)
            .width(Length::Shrink);
        for item in group.items {
            run = run.push(item);
        }
        run.into()
    }
}
