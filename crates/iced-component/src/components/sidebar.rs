//! An application sidebar. Mirrors GPUI Component's `Sidebar` and is the same
//! shape as the Slint `Sidebar` beside it: a header row with a brand mark, a
//! title and a collapse control; a scrolling body of titled groups of entries;
//! a rule; and a footer with an avatar and a name.
//!
//! **It composes, it does not re-derive.** The body is the finished
//! [`Scrollable`], the footer avatar is the finished [`Avatar`], a row is a
//! [`button`], and every colour is a [`Tokens`] value read *inside the style
//! closure* while the part renders — reading one at build time is how a surface
//! once froze the light palette into the dark theme.
//!
//! [`List`](super::list::List) is deliberately **not** the row. A
//! [`ListEntry`](super::list::ListEntry) carries a title, a description and a
//! leading icon; a sidebar entry carries a label, a trailing figure and a
//! trailing icon, and the list has no section headings at all. Both are the same
//! `button` row on a `Scrollable` underneath, which is the part worth sharing,
//! and the Slint component composes neither `List` nor a list-shaped row.
//!
//! # The controlled shape
//!
//! The sidebar holds no state. The caller owns which row is selected
//! ([`Sidebar::selected`]) and whether the rail is collapsed
//! ([`Sidebar::collapsed`]); a click reports the row through
//! [`Sidebar::on_select`] and the collapse control reports the value it wants
//! through [`Sidebar::on_toggle`], exactly as [`List`](super::list::List) and
//! [`Menu`](super::menu::Menu) report and the caller keeps the state. A sidebar
//! with no `on_toggle` draws no collapse control, for the same reason a
//! [`button`] with no message cannot be pressed.
//!
//! # The collapse control is the caller's
//!
//! Expanded, the header carries a `panel-left` control that asks to collapse.
//! Collapsed, the rail is 56px and holds the brand mark and nothing else, so the
//! sidebar draws **no** expand control: at that width there is no room for one
//! beside the mark, and a control placed outside the rail's bounds is painted
//! before the application's content, which covers it. The application owns the
//! expand trigger and places it beside the rail — the arrangement the Slint
//! component's own comment describes and shadcn uses. `demos/sidebar.rs` shows
//! the 28px `panel-left` trigger it puts there.
//!
//! # Icons
//!
//! An entry's leading icon, its trailing icon and the header mark are the
//! **caller's bytes**, taken as `&'static [u8]` the way [`menu::Item`](super::menu::Item)
//! takes its glyphs. They have to be bytes rather than an [`Element`] because
//! the row colours its glyph from the state it is drawn in — muted, or the
//! ordinary foreground — and an SVG keeps its own black unless the style closure
//! colours it, which is the trap that makes an un-coloured icon vanish in the
//! dark theme. The component ships two glyphs of its own, the `panel-left`
//! collapse control and the footer's `chevrons-up-down`, because those are its
//! chrome rather than the application's content; Slint hardcodes the same two.
//!
//! # Children, badges and the header mark
//!
//! A row may carry children ([`SidebarEntry::children`]) and the caller's
//! `open` decides whether they are disclosed; the row draws a chevron that
//! swaps from `chevron-right` to `chevron-down` and the children are wrapped in
//! [`collapsible::reveal`](super::collapsible::reveal), so the reveal is the
//! accordion's and the tree's own 150ms linear motion rather than a second
//! implementation. A click on that row reports through [`Sidebar::on_select`]
//! exactly like any other row, and the caller decides whether it moves the
//! selection or flips `open` — the one state the sidebar does not own.
//!
//! A row's trailing slot can also hold the finished [`Badge`]
//! ([`SidebarEntry::badge`]) and a [`Toggle`] ([`SidebarEntry::toggle`]); the
//! header can carry a caller-supplied trailing mark ([`Sidebar::header_action`])
//! in place of the collapse control. These are the shapes GPUI's
//! `SidebarMenuItem::suffix` expresses with an element; iced's row is data, so
//! they are named rather than an arbitrary element slot.
//!
//! # What iced cannot draw, named rather than faked
//!
//! - **A muted row keeps its ink, not its opacity.** Slint dims the whole row to
//!   0.45; iced 0.14 has no subtree opacity, so a muted row draws
//!   `muted-foreground` ink and ignores the pointer. Slint also dims a *selected*
//!   muted row's accent backplate; here the backplate stays whole.
//! - **The label clips, it does not elide.** iced 0.14 has no ellipsis, so a long
//!   label is `Wrapping::None` and is cut at the row instead of ending in `…`.
//! - **No keyboard or focus.** A row and the collapse control answer the pointer;
//!   iced 0.14 gives a bare `button` no focus ring, no `Tab` stop and no `Space`
//!   or `Enter`, so the traversal the Slint `TouchArea`s inherit from the window
//!   is the application's.
//! - **A child row is not selectable.** Children are drawn and their own
//!   [`Toggle`] reports, but [`Sidebar::on_select`] addresses a group and a
//!   top-level entry, so a child gets no selection report and no selected
//!   backplate. Reporting one would need a path in the message, which this
//!   component's controlled shape does not carry.
//! - **The rail width snaps.** Slint animates it over 160ms; iced 0.14 has no
//!   width transition, so it changes on the frame after the message. Row
//!   disclosure is animated, because that is a panel height rather than a box
//!   width.
//! - **The footer is not a menu.** Slint draws the `chevrons-up-down` glyph and
//!   opens nothing; a caller that wants an account menu composes
//!   [`Popover`](super::popover::Popover) with [`Menu`](super::menu::Menu) around
//!   its own trigger.
//! - **The narrow-viewport breakpoint is not reproduced.** Slint narrows the rail
//!   to 180px under a 480px viewport; this is a fixed 232px expanded rail.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one part gets
// a warning for each one it does not want.
#![allow(dead_code)]

use iced::font::Weight;
use iced::widget::text::Wrapping;
use iced::widget::{Space, button, column, container, row, svg, text};
use iced::{Alignment, Border, Color, Element, Font, Length, Padding, Shadow, Theme};

use super::avatar::{Avatar, AvatarSize};
use super::badge::Badge;
use super::collapsible;
use super::scrollable::{Axis, Scrollable};
use super::theme::{self, Tokens};
use super::toggle::Toggle;

/// The expanded rail, `232px` in the Slint component.
pub const EXPANDED_WIDTH: f32 = 232.0;
/// The collapsed rail, `56px` in the Slint component.
pub const COLLAPSED_WIDTH: f32 = 56.0;

/// The inset around the whole rail, `padding: 12px`.
const RAIL_PADDING: f32 = 12.0;
/// The outer column's gap, `spacing: 4px`.
const RAIL_GAP: f32 = 4.0;
/// The brand mark's box, `width: 28px; height: 28px`.
const BRAND: f32 = 28.0;
/// A row's own inset, `padding: 8px`.
const ROW_PADDING: f32 = 8.0;
/// Between a row's icon and its text, `spacing: 10px`.
const ROW_SPACING: f32 = 10.0;
/// Between two groups in the body, `spacing: 12px`.
const GROUP_GAP: f32 = 12.0;
/// Between the rows of one group, `spacing: 2px`.
const ENTRY_GAP: f32 = 2.0;
/// A row's leading icon, `size: 16px`.
const ICON: f32 = 16.0;
/// A row's trailing icon, `size: 14px`.
const TRAILING_ICON: f32 = 14.0;
/// The hover tint, `accent.with-alpha(0.4)` in the Slint component.
const HOVER_TINT: f32 = 0.4;
/// How far an entry's children are inset past its own row, so a disclosed run
/// reads as belonging to the row above it.
const CHILD_INDENT: f32 = 14.0;

/// The disclosure marks: Lucide's `chevron-right` while a group is closed and
/// `chevron-down` while it is open. Swapped, not turned, because iced 0.14 has
/// no rotation transition for an arbitrary widget — the same rule the
/// accordion's disclosure follows.
const CHEVRON_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#;
const CHEVRON_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#;

/// The `panel-left` glyph the expanded rail's collapse control draws, from
/// `crates/slint-component/ui/icons/panel-left.svg` (Lucide, ISC), so the two
/// galleries show the same control.
const PANEL_LEFT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/></svg>"#;

/// The footer's `chevrons-up-down` glyph, the same Lucide source as above.
const CHEVRONS_UP_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/></svg>"#;

/// One row of a group.
///
/// The columns are the Slint `SidebarEntry` struct's: a required label, an
/// optional leading icon, an optional trailing figure and an optional trailing
/// icon, and the muted state that dims the row to a control that cannot be
/// opened yet. A row may also carry children, which it discloses through
/// [`collapsible::reveal`](super::collapsible::reveal); see
/// [`SidebarEntry::children`].
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarEntry {
    label: String,
    icon: Option<&'static [u8]>,
    suffix: String,
    trailing_icon: Option<&'static [u8]>,
    muted: bool,
    destructive: bool,
    badge: bool,
    toggle: Option<bool>,
    children: Vec<SidebarEntry>,
    open: bool,
}

impl SidebarEntry {
    /// A row labelled `label`.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            suffix: String::new(),
            trailing_icon: None,
            muted: false,
            destructive: false,
            badge: false,
            toggle: None,
            children: Vec::new(),
            open: false,
        }
    }

    /// The SVG drawn in the leading 16px slot. The bytes are the caller's; the
    /// stroke is drawn from the row's ink, so it follows the theme and the muted
    /// state.
    pub fn icon(mut self, icon: &'static [u8]) -> Self {
        self.icon = Some(icon);
        self
    }

    /// A trailing figure, such as `"3"` or `"1,234"`. An empty suffix draws
    /// nothing, which is the Slint `if entry.suffix != ""`.
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = suffix.into();
        self
    }

    /// A trailing icon, such as `chevron-right`. It is drawn in the muted
    /// foreground, as Slint draws it, at 14px.
    pub fn trailing_icon(mut self, icon: &'static [u8]) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    /// A row that cannot be opened yet: muted ink, and no pointer report.
    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    /// Draws the label in the destructive ink, as GPUI's `label_style` does.
    /// The leading icon keeps the ordinary ink.
    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    /// Wraps the trailing icon in the finished [`Badge`] as a dot, so a row
    /// reads as "has a notification". Without a trailing icon the dot has
    /// nothing to anchor to and is not drawn.
    pub fn badge(mut self, badge: bool) -> Self {
        self.badge = badge;
        self
    }

    /// A [`Toggle`] after the label at this checked value. The report is
    /// [`Sidebar::on_entry_toggle`]; without that callback the control draws
    /// disabled.
    pub fn toggle(mut self, checked: bool) -> Self {
        self.toggle = Some(checked);
        self
    }

    /// Appends a child row, drawn indented under this one and disclosed through
    /// [`collapsible::reveal`](super::collapsible::reveal). A row with children
    /// draws its own chevron and reports a press through [`Sidebar::on_select`],
    /// as GPUI's `SidebarMenuItem` does.
    pub fn child(mut self, child: SidebarEntry) -> Self {
        self.children.push(child);
        self
    }

    /// Appends several child rows.
    pub fn children(mut self, children: impl IntoIterator<Item = SidebarEntry>) -> Self {
        self.children.extend(children);
        self
    }

    /// Whether the children are disclosed. The caller owns this, like every
    /// other value here; the row reports a click and this decides what the
    /// reveal shows.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

/// A titled run of entries. An empty title draws no heading, which is the Slint
/// `if group.title != ""`.
#[derive(Debug, Clone, PartialEq)]
pub struct SidebarGroup {
    title: String,
    entries: Vec<SidebarEntry>,
}

impl SidebarGroup {
    /// A group with `title`. Use an empty title for a run with no heading.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            entries: Vec::new(),
        }
    }

    /// Appends one entry.
    pub fn entry(mut self, entry: SidebarEntry) -> Self {
        self.entries.push(entry);
        self
    }

    /// Appends several entries.
    pub fn entries(mut self, entries: impl IntoIterator<Item = SidebarEntry>) -> Self {
        self.entries.extend(entries);
        self
    }
}

/// An application sidebar, built with the methods below.
pub struct Sidebar<'a, Message> {
    groups: Vec<SidebarGroup>,
    /// The selected `(group, entry)`. Slint's defaults are `0` and `0`, so the
    /// first row is current until the caller says otherwise.
    selected: Option<(usize, usize)>,
    collapsed: bool,
    header_icon: Option<&'static [u8]>,
    header_title: String,
    header_subtitle: String,
    footer_initials: String,
    footer_name: String,
    header_action: Option<(&'static [u8], Message)>,
    on_select: Option<Box<dyn Fn(usize, usize) -> Message + 'a>>,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    on_entry_toggle: Option<Box<dyn Fn(usize, usize, usize, bool) -> Message + 'a>>,
}

impl<'a, Message> Sidebar<'a, Message> {
    /// A sidebar with Slint's own defaults: the title "Workspace", the footer
    /// "YK" / "You", expanded, with the first row selected and no groups.
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            selected: Some((0, 0)),
            collapsed: false,
            header_icon: None,
            header_title: "Workspace".to_string(),
            header_subtitle: String::new(),
            footer_initials: "YK".to_string(),
            footer_name: "You".to_string(),
            header_action: None,
            on_select: None,
            on_toggle: None,
            on_entry_toggle: None,
        }
    }

    /// Appends one group.
    pub fn group(mut self, group: SidebarGroup) -> Self {
        self.groups.push(group);
        self
    }

    /// Appends several groups.
    pub fn groups(mut self, groups: impl IntoIterator<Item = SidebarGroup>) -> Self {
        self.groups.extend(groups);
        self
    }

    /// Which row is current, as its group and its index in that group. Slint
    /// carries the pair rather than one flat index so a caller does not have to
    /// keep the running total of every earlier group.
    pub fn selected(mut self, group: usize, entry: usize) -> Self {
        self.selected = Some((group, entry));
        self
    }

    /// Draws the rail collapsed: 56px wide, icons only, with no group headings
    /// and no footer name.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// The brand mark's SVG bytes. Without one the 28px primary box is drawn
    /// empty, as Slint draws an icon whose name resolves to nothing.
    pub fn header_icon(mut self, icon: &'static [u8]) -> Self {
        self.header_icon = Some(icon);
        self
    }

    /// The header's title. Slint defaults to "Workspace".
    pub fn header_title(mut self, title: impl Into<String>) -> Self {
        self.header_title = title.into();
        self
    }

    /// The header's second line. Empty, the default, draws no line.
    pub fn header_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.header_subtitle = subtitle.into();
        self
    }

    /// A trailing mark in the header, after the title, and the message it
    /// reports when pressed — the workspace swap GPUI's header carries. It
    /// takes the collapse control's place: a header has room for one trailing
    /// control, and a caller that wants both composes its own.
    pub fn header_action(mut self, icon: &'static [u8], message: Message) -> Self {
        self.header_action = Some((icon, message));
        self
    }

    /// The footer avatar's initials. Slint defaults to "YK".
    pub fn footer_initials(mut self, initials: impl Into<String>) -> Self {
        self.footer_initials = initials.into();
        self
    }

    /// The footer's name. Slint defaults to "You". Collapsed, it is not drawn.
    pub fn footer_name(mut self, name: impl Into<String>) -> Self {
        self.footer_name = name.into();
        self
    }

    /// A row was clicked. The report is the row's group and its index in that
    /// group; the sidebar never moves the selection itself, which is the same
    /// controlled shape as [`List::on_select`](super::list::List::on_select).
    pub fn on_select(mut self, on_select: impl Fn(usize, usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// The collapse control was pressed, with the value it wants the rail to
    /// take (`true` collapses). Without this the control is not drawn and the
    /// application's own trigger is the only way to change the state.
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// A child row's [`Toggle`](SidebarEntry::toggle) was pressed, with the
    /// value it wants (`true` selects). The report carries the row's group and
    /// entry index and the child's index, because a toggle can sit on any
    /// child. Without this the toggle draws disabled.
    pub fn on_entry_toggle(
        mut self,
        on_entry_toggle: impl Fn(usize, usize, usize, bool) -> Message + 'a,
    ) -> Self {
        self.on_entry_toggle = Some(Box::new(on_entry_toggle));
        self
    }

    /// The rail, and nothing else: the caller places it.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let collapsed = self.collapsed;
        let selected = self.selected;
        let width = if collapsed {
            COLLAPSED_WIDTH
        } else {
            EXPANDED_WIDTH
        };

        // The header. Collapsed, only the brand mark is drawn; the title, the
        // subtitle and the collapse control are what leave.
        let mut header = row![]
            .spacing(ROW_SPACING)
            .align_y(Alignment::Center)
            .width(Length::Fill);
        header = header.push(brand(self.header_icon));
        if !collapsed {
            let mut lines = column![header_text(
                self.header_title.clone(),
                theme::FONT_SIZE_SM,
                Weight::Semibold,
                false,
            )]
            .spacing(1.0)
            .width(Length::Fill);
            if !self.header_subtitle.is_empty() {
                lines = lines.push(header_text(
                    self.header_subtitle.clone(),
                    theme::FONT_SIZE_XS,
                    Weight::Normal,
                    true,
                ));
            }
            header = header.push(lines);
            match (self.header_action, &self.on_toggle) {
                (Some((icon, message)), _) => header = header.push(header_action(icon, message)),
                (None, Some(on_toggle)) => header = header.push(collapse_control(on_toggle(true))),
                (None, None) => {}
            }
        }

        // The body. A group is its heading and its rows at a 2px gap; the groups
        // sit at 12px; and the body scrolls under its own bar so the header and
        // footer stay put. `padding-top: 4px` is the Slint inner layout's inset.
        let mut body = column![].spacing(GROUP_GAP).width(Length::Fill);
        for (group_index, group) in self.groups.iter().enumerate() {
            let mut rows = column![].spacing(ENTRY_GAP).width(Length::Fill);
            if !collapsed && !group.title.is_empty() {
                rows = rows.push(group_heading(&group.title));
            }
            for (entry_index, entry) in group.entries.iter().enumerate() {
                let is_selected = selected == Some((group_index, entry_index));
                // A muted row is not given a message, so iced draws it as a
                // disabled control and it answers no pointer — the same rule
                // `list` and `button` follow. A row with children uses the same
                // report: the caller decides whether that click selects it or
                // flips `open`, which is the only state the sidebar does not
                // own.
                let message = self
                    .on_select
                    .as_ref()
                    .filter(|_| !entry.muted)
                    .map(|on_select| on_select(group_index, entry_index));
                let toggle = entry.toggle.and_then(|checked| {
                    self.on_entry_toggle
                        .as_ref()
                        .map(|on_toggle| on_toggle(group_index, entry_index, 0, !checked))
                });
                rows = rows.push(entry_row(
                    entry,
                    is_selected,
                    message,
                    toggle,
                    collapsed,
                    0.0,
                ));

                if !collapsed && !entry.children.is_empty() {
                    let mut children = column![].spacing(ENTRY_GAP).width(Length::Fill);
                    for (child_index, child) in entry.children.iter().enumerate() {
                        let child_toggle = child.toggle.and_then(|checked| {
                            self.on_entry_toggle.as_ref().map(|on_toggle| {
                                on_toggle(group_index, entry_index, child_index, !checked)
                            })
                        });
                        children = children.push(entry_row(
                            child,
                            false,
                            None,
                            child_toggle,
                            collapsed,
                            CHILD_INDENT,
                        ));
                    }
                    // Mounted whether or not it is open: the reveal measures
                    // the children at their full height and clips them, which
                    // is what lets a toggle mid-flight reverse from where the
                    // panel already is.
                    rows = rows.push(collapsible::reveal(children, entry.open));
                }
            }
            body = body.push(rows);
        }
        let body = Scrollable::new(container(body).padding(Padding {
            top: theme::SPACING_XS,
            right: 0.0,
            bottom: 0.0,
            left: 0.0,
        }))
        .axis(Axis::Vertical)
        .height(Length::Fill);

        // The rule, and the footer.
        let rule = container(Space::new())
            .width(Length::Fill)
            .height(Length::Fixed(1.0))
            .style(|theme: &Theme| container::Style {
                background: Some(Tokens::of(theme).border.into()),
                ..container::Style::default()
            });

        let mut footer = row![]
            .spacing(theme::SPACING_SM)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .padding(Padding {
                top: theme::SPACING_XS,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            });
        footer = footer.push(
            Avatar::new()
                .initials(self.footer_initials.clone())
                .size(AvatarSize::Small),
        );
        if !collapsed {
            footer = footer.push(
                text(self.footer_name.clone())
                    .size(theme::FONT_SIZE_SM)
                    .width(Length::Fill)
                    .wrapping(Wrapping::None)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).foreground),
                    }),
            );
            footer = footer.push(glyph(CHEVRONS_UP_DOWN, TRAILING_ICON, |tokens: Tokens| {
                tokens.muted_foreground
            }));
        }

        let rail = column![header, body, rule, footer]
            .spacing(RAIL_GAP)
            .width(Length::Fill);

        container(rail)
            .width(Length::Fixed(width))
            .height(Length::Fill)
            .padding(RAIL_PADDING)
            .style(|theme: &Theme| {
                let tokens = Tokens::of(theme);
                container::Style {
                    background: Some(tokens.sidebar.into()),
                    border: Border {
                        color: tokens.border,
                        width: 1.0,
                        ..Border::default()
                    },
                    ..container::Style::default()
                }
            })
            .into()
    }
}

impl<'a, Message> Default for Sidebar<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Sidebar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(sidebar: Sidebar<'a, Message>) -> Self {
        sidebar.into_element()
    }
}

/// An SVG in a square box, coloured from the tokens in force **inside the style
/// closure**. The ink is a function rather than a colour so a caller can pick a
/// token by the state the part is drawn in without resolving it at build time.
fn glyph<'a, Message: 'a>(
    bytes: &'static [u8],
    size: f32,
    ink: impl Fn(Tokens) -> Color + 'a,
) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(ink(Tokens::of(theme))),
        })
        .into()
}

/// The 28px primary box with the caller's mark centred in it.
fn brand<'a, Message: 'a>(icon: Option<&'static [u8]>) -> Element<'a, Message> {
    let mark: Element<'a, Message> = match icon {
        Some(bytes) => glyph(bytes, ICON, |tokens: Tokens| tokens.primary_foreground),
        None => Space::new().into(),
    };
    container(mark)
        .width(Length::Fixed(BRAND))
        .height(Length::Fixed(BRAND))
        .center_x(Length::Fixed(BRAND))
        .center_y(Length::Fixed(BRAND))
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).primary.into()),
            border: Border {
                radius: theme::RADIUS_SM.into(),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .into()
}

/// The expanded rail's collapse control: the bare `panel-left` glyph, which is
/// what Slint's `TouchArea` wraps. It draws no surface of its own.
fn collapse_control<'a, Message: Clone + 'a>(message: Message) -> Element<'a, Message> {
    iced::widget::mouse_area(glyph(PANEL_LEFT, ICON, |tokens: Tokens| {
        tokens.muted_foreground
    }))
    .on_press(message)
    .into()
}

/// The header's caller-supplied trailing mark, with no surface of its own — the
/// same bare shape as the collapse control, and the workspace swap GPUI's
/// header draws.
fn header_action<'a, Message: Clone + 'a>(
    icon: &'static [u8],
    message: Message,
) -> Element<'a, Message> {
    iced::widget::mouse_area(glyph(icon, ICON, |tokens: Tokens| tokens.muted_foreground))
        .on_press(message)
        .into()
}

/// A header line. The title is `font-size-sm` at semibold and the subtitle
/// `font-size-xs`; both keep to one line and clip rather than wrap, because
/// iced 0.14 has no ellipsis.
fn header_text<'a, Message: 'a>(
    value: String,
    size: f32,
    weight: Weight,
    subdued: bool,
) -> Element<'a, Message> {
    text(value)
        .size(size)
        .font(Font {
            weight,
            ..Font::DEFAULT
        })
        .width(Length::Fill)
        .wrapping(Wrapping::None)
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            text::Style {
                color: Some(if subdued {
                    tokens.muted_foreground
                } else {
                    tokens.foreground
                }),
            }
        })
        .into()
}

/// A group's heading: `font-size-xs` at medium in the muted foreground, inset
/// 8px from the left with 4px above and 2px below, which is the Slint row.
fn group_heading<'a, Message: 'a>(title: &str) -> Element<'a, Message> {
    container(
        text(title.to_owned())
            .size(theme::FONT_SIZE_XS)
            .font(Font {
                weight: Weight::Medium,
                ..Font::DEFAULT
            })
            .wrapping(Wrapping::None)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    )
    .width(Length::Fill)
    .padding(Padding {
        top: theme::SPACING_XS,
        right: 0.0,
        bottom: theme::SPACING_XXS,
        left: theme::SPACING_SM,
    })
    .into()
}

/// One entry, as the button the shared row shape is built on.
///
/// `message` is `None` for a muted entry, which is how iced spells "cannot be
/// pressed"; the row then takes no hover either. `toggle_message` drives the
/// entry's own [`Toggle`], and `indent` is where a child row starts inside the
/// row's own inset.
fn entry_row<'a, Message: Clone + 'a>(
    entry: &SidebarEntry,
    selected: bool,
    message: Option<Message>,
    toggle_message: Option<Message>,
    collapsed: bool,
    indent: f32,
) -> Element<'a, Message> {
    let muted = entry.muted;
    let destructive = entry.destructive;

    let mut line = row![]
        .spacing(ROW_SPACING)
        .align_y(Alignment::Center)
        .width(Length::Fill);
    line = line.push(match entry.icon {
        Some(bytes) => glyph(bytes, ICON, move |tokens: Tokens| {
            if muted {
                tokens.muted_foreground
            } else {
                tokens.foreground
            }
        }),
        // Slint draws an icon even when its name resolves to nothing, and the
        // 16px slot is what keeps every label lined up; an entry without one
        // keeps the slot.
        None => Space::new().width(ICON).into(),
    });

    if !collapsed {
        let ink = move |tokens: Tokens| {
            if muted {
                tokens.muted_foreground
            } else if destructive {
                tokens.danger
            } else {
                tokens.foreground
            }
        };
        line = line.push(
            text(entry.label.clone())
                .size(theme::FONT_SIZE_SM)
                .width(Length::Fill)
                .wrapping(Wrapping::None)
                .style(move |theme: &Theme| text::Style {
                    color: Some(ink(Tokens::of(theme))),
                }),
        );
        if !entry.suffix.is_empty() {
            line = line.push(
                text(entry.suffix.clone())
                    .size(theme::FONT_SIZE_XS)
                    .wrapping(Wrapping::None)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
            );
        }
        if let Some(checked) = entry.toggle {
            let mut control = Toggle::new(checked).size(theme::ControlSize::XSmall);
            if let Some(message) = toggle_message {
                control = control.on_toggle(move |_| message.clone());
            }
            line = line.push(control);
        }
        if let Some(bytes) = entry.trailing_icon {
            let icon = glyph(bytes, TRAILING_ICON, |tokens: Tokens| {
                tokens.muted_foreground
            });
            let trailing: Element<'a, Message> = if entry.badge {
                Badge::new(icon).dot().count(1).into()
            } else {
                icon
            };
            line = line.push(trailing);
        }
        if !entry.children.is_empty() {
            line = line.push(disclosure(entry.open));
        }
    }

    let mut control = button(line)
        .width(Length::Fill)
        .padding(Padding {
            top: ROW_PADDING,
            right: ROW_PADDING,
            bottom: ROW_PADDING,
            left: ROW_PADDING + indent,
        })
        .style(move |theme: &Theme, status| row_style(theme, status, selected, muted));
    if let Some(message) = message {
        control = control.on_press(message);
    }
    control.into()
}

/// The disclosure mark a row with children draws, in the open or closed end
/// state, tinted in its own style closure — an SVG keeps its own black otherwise.
fn disclosure<'a, Message: 'a>(open: bool) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(if open {
        CHEVRON_DOWN
    } else {
        CHEVRON_RIGHT
    }))
    .width(Length::Fixed(TRAILING_ICON))
    .height(Length::Fixed(TRAILING_ICON))
    .style(|theme: &Theme, _status| svg::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    })
    .into()
}

/// The row's surface, resolved inside the closure: `accent` when the caller
/// selected it, 0.4 of that while the pointer is on it and it is not muted, and
/// nothing otherwise — the Slint ternary, in order.
fn row_style(theme: &Theme, status: button::Status, selected: bool, muted: bool) -> button::Style {
    let tokens = Tokens::of(theme);
    let lit = matches!(status, button::Status::Hovered | button::Status::Pressed);
    let background = if selected {
        tokens.accent
    } else if lit && !muted {
        theme::alpha(tokens.accent, HOVER_TINT)
    } else {
        Color::TRANSPARENT
    };

    button::Style {
        background: (background.a > 0.0).then_some(background.into()),
        text_color: if muted {
            tokens.muted_foreground
        } else {
            tokens.foreground
        },
        border: Border {
            radius: theme::RADIUS_MD.into(),
            ..Border::default()
        },
        shadow: Shadow::default(),
        snap: false,
    }
}
