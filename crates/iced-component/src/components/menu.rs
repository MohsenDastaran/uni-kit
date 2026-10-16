//! A list of actions. Mirrors GPUI Component's `PopupMenu` and the Slint
//! `Menu` beside it, and is the item shape `select` and `combobox` compose.
//!
//! # The surface, and what the menu is
//!
//! The Slint `Menu` is the whole panel: `preferred-width: 280px`, a popover
//! fill, a one-pixel border, `radius-lg` and a 4px inset around its rows. This
//! file is that body — the rows, their 32px height floor, the separator's 9px
//! with a one-pixel rule, the columns that stay put when a row has no icon. The
//! floating is [`Popover`](super::popover::Popover)'s job, because a menu with
//! no anchor is not a control: every card in the Slint example puts one inside
//! a `PopupWindow`.
//!
//! This file draws the **body** — the 4px inset and the rows — and never the
//! surface. One container owns the surface, and it is exactly one:
//!
//! - a menu in a [`Popover`](super::popover::Popover) asks the popover for
//!   `filled(true)`, which draws the family's [`PanelStyle`](super::popover::PanelStyle)
//!   surface around the body; [`Menu::into_element`] is what goes in
//!   [`Popover::content`](super::popover::Popover::content);
//! - a caller placing a menu with no popover wraps the body with
//!   [`menu_container`], which draws the same surface from [`MenuStyle`].
//!
//! Never both. [`Menu::into_element`] used to draw a `MenuStyle` of its own as
//! well, so every popover — `filled(true)` in `DropdownButton`, and
//! `panel_container` in the gallery — drew a second rounded border a few pixels
//! inside the first: one menu, painted as two panels.
//!
//! # The item shape
//!
//! [`Item`] carries the columns the Slint `MenuEntry` struct carries — label,
//! icon, shortcut, disabled, checked, separator — and adds the destructive
//! state GPUI's `PopupMenuItem` has and Slint's binding does not. The icon bytes
//! are the *caller's*, because iced applications own their assets and this crate
//! cannot resolve a name to a file the way Slint's build script does; they are
//! `'static` because iced's `svg::Handle` is. The separator is its own
//! constructor rather than a flag on every row, so a menu reads as the list of
//! the things it holds.
//!
//! # Hover, and the highlight the caller owns
//!
//! A row reports the pointer entering it through [`Menu::on_highlight`] and
//! leaving it — or leaving the panel — through [`Menu::on_leave`], and it is
//! drawn on the accent surface when the caller passes an index back to
//! [`Menu::highlighted`]. The highlight cannot live in the menu itself, because
//! the menu becomes an element rather than a widget of its own; the caller
//! keeping it is also iced's own arrangement — `pick_list` holds
//! `hovered_option` in the application's state and its menu reports
//! `on_option_hovered` so the application can follow along.
//!
//! [`Menu::on_highlight`] is a **hover**, and a hover is transient: a pointer
//! that has left leaves no row lit. It is not the selection — that is
//! [`Menu::on_chosen`] and [`Menu::selected`] — and it is not the row a key has
//! moved to either, even though [`Menu::highlighted`] draws both. An
//! application keeps the hover in one index and the key's row in another, and
//! draws whichever is live, because a keyboard highlight has to survive the
//! pointer merely crossing the panel: an application that folds the two into
//! one index loses the key's row when the pointer leaves. The demo's "Keyboard
//! Shortcuts" card shows the arrangement.
//!
//! # A choice is made on the release
//!
//! [`Menu::on_chosen`] reports on the pointer's **release**, not its press. A
//! menu closes on choose, and the press is the last event the panel is
//! guaranteed to still be under: report the choice there and the panel is gone
//! by the release, which then reaches whatever the caller layered behind the
//! menu — the click-through a popover overlay could not stop once its panel had
//! closed. Reporting on the release keeps the panel open for it, and
//! [`Popover`](super::popover::Popover)'s overlay consumes the release while the
//! panel is still there. GPUI's `MenuItem` writes the same rule from the other
//! side: it stops the mouse-down and acts on the click. The row answers a
//! release even when the press began on another row of the same panel; a menu
//! has no drag that would be disturbed by that.
//!
//! # What is not here
//!
//! iced 0.14 has no focus trap and no menu role, so the traversal keys are not
//! the component's: an application binds `↑`/`↓`/`Home`/`End`/`Enter` and moves
//! the highlight itself. `Escape` and a click outside belong to the popover the
//! menu sits in, which owns those two dismissals for every overlay in this
//! crate. `Tab` does not move focus out of a menu.
//!
//! A submenu is an **affordance here and a panel in the caller**: [`Item::submenu`]
//! draws the trailing chevron GPUI draws, and the row reports through
//! [`Menu::on_chosen`] like any other — but the nested items are the caller's, so
//! the caller opens the nested panel. Drawing a `›` that opened nothing would be
//! the lie this file used to refuse; what makes it honest is the caller having
//! nested items to put behind it, which this component cannot know.
//! [`Item::external`] is the same shape: the external-link mark is drawn, and
//! opening the URL is the application's.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{column, container, mouse_area, row, svg, text, Space};
use iced::{Border, Color, Element, Length, Padding, Theme};

use super::theme::{self, Tokens};

/// The rows' height **floor**, `32px` in the Slint component. A one-line row is
/// exactly this; a label long enough to wrap adds a line box and the row grows
/// past it, which the fixed height this replaces could not do.
pub const ROW_HEIGHT: f32 = 32.0;
/// The vertical breathing room every row keeps above and below its content, so
/// a two-line label reads as one row rather than touching the accent fill. The
/// leading slot carries `ROW_HEIGHT - 2 * ROW_PADDING_Y`, which is what holds a
/// one-line row at [`ROW_HEIGHT`] and lets a wrapped one outgrow it.
const ROW_PADDING_Y: f32 = 6.0;
/// The height a separator occupies, `9px` in the Slint component: 4px, the
/// rule, and 4px again.
pub const SEPARATOR_HEIGHT: f32 = 9.0;
/// The inset around the rows, `padding: 4px` in the Slint component.
pub const PANEL_PADDING: f32 = 4.0;
/// The panel's preferred width. The Slint component authors `preferred-width:
/// 280px`, but the labels this catalogue now shows beside their shortcuts
/// ("Export all rows (.csv)") wrapped there, so the panel is wider here.
pub const PANEL_WIDTH: f32 = 320.0;
/// The width of the leading icon slot, `width: 16px` in the Slint component.
const ICON_SLOT: f32 = 16.0;
/// The width of the trailing shortcut column, `width: 48px`.
const SHORTCUT_SLOT: f32 = 48.0;
/// The width of the trailing check slot, `width: 16px`.
const CHECK_SLOT: f32 = 16.0;

/// One row of a menu.
///
/// The columns are the Slint `MenuEntry` struct's, and the destructive state is
/// GPUI's `PopupMenuItem::danger`, which the Slint binding does not carry.
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    label: String,
    /// A second, smaller line under the label, GPUI's `element` row with a
    /// sub-title under its title. Empty means a one-line row.
    subtitle: String,
    icon: Option<&'static [u8]>,
    shortcut: String,
    disabled: bool,
    checked: bool,
    /// A row that opens a nested panel. The component draws the affordance — a
    /// trailing chevron — and reports the choice through [`Menu::on_chosen`];
    /// opening the nested panel is the caller's job, because this component does
    /// not own the caller's nested items.
    submenu: bool,
    /// A row that leaves the application — GPUI's `PopupMenuItem::link`. The
    /// trailing external-link mark is drawn next to the label.
    external: bool,
    danger: bool,
    separator: bool,
}

impl Item {
    /// An actionable row with `label`.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            subtitle: String::new(),
            icon: None,
            shortcut: String::new(),
            disabled: false,
            checked: false,
            submenu: false,
            external: false,
            danger: false,
            separator: false,
        }
    }

    /// A rule between groups of rows. It draws no label and answers no pointer
    /// or key, which is what `separator: true` does in the Slint example.
    pub fn separator() -> Self {
        Self {
            separator: true,
            ..Self::new("")
        }
    }

    /// The SVG drawn in the leading 16px slot. The bytes are the caller's,
    /// because the icon set belongs to the application; the stroke is drawn in
    /// the row's ink.
    pub fn icon(mut self, icon: &'static [u8]) -> Self {
        self.icon = Some(icon);
        self
    }

    /// A second line under the label, drawn at `font-size-xs` in
    /// `muted-foreground` — the sub-title GPUI's custom-element row puts under
    /// its title. A row with one is taller than the 32px floor, which is what
    /// [`ROW_HEIGHT`] being a floor rather than a fixed height is for.
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    /// A row that opens a nested panel. It draws the trailing chevron GPUI's
    /// submenu row draws; the panel itself is the caller's, because the nested
    /// items are. The row still reports through [`Menu::on_chosen`], so a caller
    /// that has a nested menu reacts to the same index a plain row would.
    pub fn submenu(mut self, submenu: bool) -> Self {
        self.submenu = submenu;
        self
    }

    /// A row that leaves the application — GPUI's `link`. The external-link mark
    /// is drawn in `muted-foreground` at the label's trailing edge, where GPUI
    /// puts it, before the shortcut column.
    pub fn external(mut self, external: bool) -> Self {
        self.external = external;
        self
    }

    /// The shortcut shown at the trailing edge, right-aligned in a 48px column
    /// in the monospace face — `⌘N` in the Slint example.
    ///
    /// The face does not decide what a shortcut says, but it decides what
    /// renders. On wasm the gallery's monospace family resolves to the embedded
    /// Fira Sans, which has `⇧` and `⌫` and **not** `⌘` or `⌥`, so the gallery
    /// spells those two in words (`Cmd N`, `Alt Z`) rather than painting a blank
    /// where a modifier should be. See `demos/menu.rs`; the rule is the caller's,
    /// because the shortcut is the caller's string.
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = shortcut.into();
        self
    }

    /// A row that is never chosen and never highlighted, `disabled: true`.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// The check mark in the trailing 16px slot, `checked: true`.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Draw the row in the destructive colour. GPUI's `PopupMenuItem` has this
    /// as `danger`; the Slint `MenuEntry` has no equivalent, so a menu built
    /// from the Slint example never shows it.
    pub fn danger(mut self, danger: bool) -> Self {
        self.danger = danger;
        self
    }

    pub fn is_separator(&self) -> bool {
        self.separator
    }

    /// Whether the row answers a pointer or a key.
    pub fn is_actionable(&self) -> bool {
        !self.separator && !self.disabled
    }

    /// Whether the row draws its check mark. [`Item::checked`] is the builder;
    /// the boolean reader takes the `is_` prefix, so the two do not collide.
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    /// The SVG bytes drawn in the leading 16px slot, or nothing when the row has
    /// no icon. The builder [`Item::icon`] already owns `icon`, so the reader
    /// names what it hands back.
    pub fn icon_bytes(&self) -> Option<&'static [u8]> {
        self.icon
    }

    /// The shortcut shown at the trailing edge, which [`Item::shortcut`] sets.
    /// Named for the same reason [`Item::icon_bytes`] is: the builder owns the
    /// field name.
    pub fn shortcut_text(&self) -> &str {
        &self.shortcut
    }

    /// The second line under the label, or the empty string. [`Item::subtitle`]
    /// owns the builder name, so the reader names what it hands back.
    pub fn subtitle_text(&self) -> &str {
        &self.subtitle
    }

    /// Whether the row has a second line, which [`Item::subtitle`] sets.
    pub fn has_subtitle(&self) -> bool {
        !self.subtitle.is_empty()
    }

    /// Whether the row draws the trailing submenu chevron.
    pub fn has_submenu(&self) -> bool {
        self.submenu
    }

    /// Whether the row draws the trailing external-link mark. The builder
    /// [`Item::external`] owns that name, so this takes the `is_` prefix.
    pub fn is_external(&self) -> bool {
        self.external
    }
}

/// A menu: a list of [`Item`]s and the report a caller acts on.
///
/// # It becomes an element, not a widget of its own
///
/// The panel is an ordinary iced element — a `container` of rows, each a
/// `mouse_area` — and [`Menu::into_element`] hands it to the
/// [`Popover`](super::popover::Popover) that floats it. That is the same
/// structure the popover's own content uses, and it is what lets the rows
/// measure and keep the per-row widget state a pointer target needs: iced
/// reconciles an element tree against the tree it was built into, and a widget
/// that rebuilds its children from its own `layout` cannot be reconciled with
/// them (the children's trees are never created, so a text row measures at
/// nothing).
///
/// The consequence is named plainly: the highlight is the *caller's*, because an
/// element cannot keep it. [`Menu::highlighted`] is what the row under the
/// pointer is drawn as, and the caller moves it from [`Menu::on_highlight`] and
/// [`Menu::on_leave`] — the entry and exit a `mouse_area` fires. iced cannot put
/// the traversal inside the element without the tree the last paragraph
/// describes, so this component reports and the caller decides, exactly as
/// iced's own `pick_list` keeps its `hovered_option` in the application's state.
pub struct Menu<'a, Message> {
    items: Vec<Item>,
    on_chosen: Option<Callback<'a, Message>>,
    on_highlight: Option<Callback<'a, Message>>,
    /// The pointer's two exits: a row it left, or the panel as a whole. See
    /// [`on_leave`](Menu::on_leave) for why the row travels with the report.
    on_leave: Option<LeaveCallback<'a, Message>>,
    /// The row drawn as chosen — Slint's `checked`, and GPUI's `selected`. It is
    /// the caller's, because the caller's data says which item is current.
    selected: Option<usize>,
    /// The row the caller says is highlighted. The pointer's choice is reported
    /// through [`on_highlight`](Self::on_highlight); drawing it is this.
    highlighted: Option<usize>,
}

/// One report a menu makes, erased so the two are one type. An `Rc` because the
/// builders take and return the menu by value while the element it becomes has
/// to keep its callbacks across frames; the closure is only ever called.
type Callback<'a, Message> = std::rc::Rc<dyn Fn(usize) -> Message + 'a>;

/// The leave report, which carries the row when there was one — see
/// [`Menu::on_leave`].
type LeaveCallback<'a, Message> = std::rc::Rc<dyn Fn(Option<usize>) -> Message + 'a>;

impl<'a, Message> Menu<'a, Message> {
    /// A menu of `items`. An empty menu draws an empty panel rather than
    /// nothing: `select` composes this with a list that may filter to nothing.
    pub fn new(items: impl IntoIterator<Item = Item>) -> Self {
        Self {
            items: items.into_iter().collect(),
            on_chosen: None,
            on_highlight: None,
            on_leave: None,
            selected: None,
            highlighted: None,
        }
    }

    /// Report the row that was chosen, by index, on the pointer's release.
    ///
    /// This is the whole contract with `select`: the caller owns the items and
    /// asks itself what index 2 means. iced's `combo_box` reports `on_select`
    /// the same way, and the Slint component's `chosen(index: int)` is the same
    /// number. The release and not the press is what makes a menu that closes on
    /// choose safe to layer over a page; the module comment has the full
    /// account.
    pub fn on_chosen(mut self, on_chosen: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_chosen = Some(std::rc::Rc::new(on_chosen));
        self
    }

    /// Report the row that was chosen only when the caller has somewhere to put
    /// it, for a `DropdownButton` assembled from a control whose report is
    /// optional. It is the same report as [`on_chosen`](Self::on_chosen); the
    /// `Option` is the caller's.
    pub fn on_chosen_maybe(mut self, on_chosen: impl Fn(usize) -> Option<Message> + 'a) -> Self {
        let on_chosen = std::rc::Rc::new(on_chosen);
        self.on_chosen = Some(std::rc::Rc::new(move |index| {
            on_chosen(index)
                .unwrap_or_else(|| unreachable!("a menu reports a choice only with a callback"))
        }));
        self
    }

    /// Report the row the pointer entered, by index. The caller draws it by
    /// passing the same index to [`highlighted`](Self::highlighted).
    ///
    /// This is a **hover**, not a choice and not the row a key moved to. A hover
    /// is transient — [`on_leave`](Self::on_leave) is its other half — and a
    /// composer that owns both a hover and a keyboard highlight keeps them in
    /// separate indices, because a leave clears only the hover. Folding the two
    /// into one index loses the key's row the moment the pointer crosses the
    /// panel, even though it never stopped over it. See the module comment.
    ///
    /// This is iced `combo_box`'s `on_option_hovered`.
    pub fn on_highlight(mut self, on_highlight: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_highlight = Some(std::rc::Rc::new(on_highlight));
        self
    }

    /// Report that the pointer left a row, or left the panel: `Some(index)` is
    /// the row it left and `None` is the panel as a whole.
    ///
    /// This is the half that clears a hover
    /// ([`on_highlight`](Self::on_highlight) is the half that sets one), and the
    /// row index is part of the report because iced walks the rows top-down:
    /// when the pointer moves from a lower row to an upper one, the upper row's
    /// entry is published **before** the lower row's leave. A composer must
    /// therefore clear only the row it was on, never whatever index is lit:
    ///
    /// ```ignore
    /// Menu::new(items).on_leave(move |left| Message::Left(left))
    /// // in update:
    /// //   Message::Left(Some(index)) if hovered == Some(index) => hovered = None,
    /// //   Message::Left(None) => hovered = None,
    /// ```
    ///
    /// `None` clears the hover whatever row it was on, because the panel is no
    /// longer under the pointer. Neither report touches the row a key moved to
    /// or the selected row — those are the caller's own state, and a keyboard
    /// highlight must outlive the pointer crossing the panel.
    pub fn on_leave(mut self, on_leave: impl Fn(Option<usize>) -> Message + 'a) -> Self {
        self.on_leave = Some(std::rc::Rc::new(on_leave));
        self
    }

    /// The row drawn as chosen, the index of the `checked` item in the Slint
    /// example's "Check state" card.
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// The row drawn on the accent surface, which is the row the pointer is
    /// over or the one a key has moved to. The caller owns it; see the type's
    /// comment.
    pub fn highlighted(mut self, highlighted: Option<usize>) -> Self {
        self.highlighted = highlighted;
        self
    }

    /// The row drawn for a menu with no highlight of its own.
    fn row_element(&self, index: usize) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let item = &self.items[index];
        // A separator is its own shape, not a row: the Slint one is a 9px band
        // with a one-pixel rule, and `build_row` for it drew a 32px blank row
        // with no rule. It answers no pointer and carries no highlight, so it
        // never becomes a `mouse_area`.
        if item.is_separator() {
            return build_separator();
        }
        let highlighted = self.highlighted == Some(index);
        let selected = self.selected == Some(index);

        let row = build_row(item, highlighted, selected, self.items.len(), index);
        // A row is a pointer target only if it has something to report. The
        // three reports are attached independently: a caller that wants only a
        // hover, or only a choice, still gets one, and `on_leave` is what clears
        // the hover when the pointer moves on.
        if self.on_highlight.is_none() && self.on_leave.is_none() && self.on_chosen.is_none() {
            return row;
        }
        let mut area = mouse_area(row);
        if let Some(on_highlight) = &self.on_highlight {
            let on_highlight = on_highlight.clone();
            area = area.on_enter(on_highlight(index));
        }
        if let Some(on_leave) = &self.on_leave {
            let on_leave = on_leave.clone();
            area = area.on_exit(on_leave(Some(index)));
        }
        if let Some(on_chosen) = &self.on_chosen {
            let on_chosen = on_chosen.clone();
            // A choice is made on the pointer's **release**, not its press, and
            // that is what keeps the row's own click out of the page behind it.
            // A menu closes on choose, so the press is the last event the panel
            // is guaranteed to still be under: report there and the panel is
            // already gone by the release, which then lands on whatever the
            // caller layered behind the menu — the press-through one layer down.
            // Waiting for the release keeps the panel alive for it, and the
            // popover's overlay consumes the release while it is. GPUI's
            // `MenuItem` does the same from the other side: it stops the
            // mouse-down and acts on the click. The row answers the release even
            // when the press began elsewhere in the panel; a menu has no drag
            // gesture for that to disturb.
            area = area.on_release(on_chosen(index));
        }
        area.into()
    }

    /// The panel's body, without a surface: the 4px inset and a row per item.
    /// The container draws the surface around it — see the module comment for the
    /// one-owner rule. The body carries the inset, so neither surface asks for
    /// one: [`Popover::filled`](super::popover::Popover::filled) is told
    /// `padding(0.0)`, and [`menu_container`] adds no padding of its own.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let mut rows = column![].spacing(0.0).width(Length::Fill);
        for index in 0..self.items.len() {
            rows = rows.push(self.row_element(index));
        }
        // The body, and only the body. The surface is the container's: the
        // popover's `filled(true)` surface, or `menu_container`'s. Drawing a
        // `MenuStyle` here as well would put a second rounded border a few pixels
        // inside the first — one menu, painted as two panels. The two are never
        // both asked for.
        let body: Element<'a, Message> = container(rows)
            .padding(PANEL_PADDING)
            .width(Length::Fill)
            .into();
        // The panel's own edge: a row's exit fires when the pointer leaves that
        // row, but the panel's fires when it leaves the panel, which is the
        // report that clears a hover sitting on the 4px inset or a separator.
        // It draws nothing — a `mouse_area` is a pointer target, not a surface,
        // so the one-owner rule above still holds.
        match &self.on_leave {
            Some(on_leave) => {
                let on_leave = on_leave.clone();
                mouse_area(body).on_exit(on_leave(None)).into()
            }
            None => body,
        }
    }
}

/// What a row's slot holds when the item leaves it empty. The slot still
/// occupies its width, so labels and shortcuts line up down the menu, which is
/// what the Slint component's empty `Rectangle` is for.
fn empty_slot<'a, Message: 'a>(width: f32) -> Element<'a, Message> {
    Space::new().width(width).height(Length::Shrink).into()
}

/// The leading `ICON_SLOT`-wide column, which also carries the row's height
/// floor. A `Row` is as tall as its tallest child, so the slot's fixed
/// `ROW_HEIGHT - 2 * ROW_PADDING_Y` makes a one-line row exactly
/// [`ROW_HEIGHT`] tall while a label that wraps to a second line grows the row
/// past it. The slot stays centred in the taller row, so the icon, the label
/// and the check keep one centre line.
fn leading_slot<'a, Message>(
    icon: Option<&'static [u8]>,
    danger: bool,
    disabled: bool,
    lit: bool,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let content: Element<'a, Message> = match icon {
        Some(icon) => glyph(icon, danger, disabled, lit),
        None => Space::new().into(),
    };
    container(content)
        .height(ROW_HEIGHT - 2.0 * ROW_PADDING_Y)
        .center_x(ICON_SLOT)
        .into()
}

/// The ink a row's label, icon and check mark are drawn in. A highlighted or
/// selected row takes the accent surface's own foreground — GPUI draws its
/// `MenuItem` with `accent_foreground` over the accent fill — so the row's ink
/// and its fill are a readable pair in both appearances.
fn row_ink(tokens: Tokens, danger: bool, disabled: bool, lit: bool) -> Color {
    if disabled {
        tokens.muted_foreground
    } else if danger {
        tokens.destructive
    } else if lit {
        tokens.accent_foreground
    } else {
        tokens.foreground
    }
}

/// The 14px SVG a row's leading or trailing slot holds.
fn glyph<'a, Message: 'a>(
    bytes: &'static [u8],
    danger: bool,
    disabled: bool,
    lit: bool,
) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(14.0)
        .height(14.0)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(row_ink(Tokens::of(theme), danger, disabled, lit)),
        })
        .into()
}

/// The 14px SVG drawn in `muted-foreground`: the submenu chevron and the
/// external-link mark. GPUI draws both in the muted ink and never in the row's
/// own, so a danger or disabled row does not tint them with its label.
fn muted_glyph<'a, Message: 'a>(bytes: &'static [u8]) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(14.0)
        .height(14.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

fn build_row<'a, Message>(
    item: &Item,
    highlighted: bool,
    selected: bool,
    _items: usize,
    _index: usize,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let danger = item.danger;
    let disabled = item.disabled;
    // A disabled row never lights, so the two are the same surface and the same
    // ink whether the caller says it is highlighted or selected.
    let lit = (highlighted || selected) && !disabled;

    // Every row keeps the same columns — a leading icon slot, the label, the
    // shortcut, and a trailing check slot — so a row that leaves one empty
    // still occupies it and the columns line up down the menu. The label column
    // is the one that grows: a sub-title adds a line under it and the row grows
    // past the 32px floor, and an external-link row keeps that mark at the
    // label's trailing edge, inside the column, so the shortcut does not shift.
    //
    // The row is as tall as its tallest child and the label is the one that can
    // wrap, so a two-line label grows the row instead of being clipped by a
    // fixed 32px frame. The leading slot carries the floor (see
    // [`leading_slot`]) and the row itself stays `Shrink`: a `Fill` child inside
    // a `Shrink` cross axis lays out against a zero cross size, and a `Shrink`
    // row inside a fixed frame rides at the top — this row is neither.
    let mut line = row![]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center);

    line = line.push(leading_slot(item.icon, danger, disabled, lit));

    let mut label = column![].spacing(theme::SPACING_XS).width(Length::Fill);
    label = label.push(
        text(item.label.clone())
            .size(theme::FONT_SIZE_SM)
            .style(move |theme: &Theme| text::Style {
                color: Some(row_ink(Tokens::of(theme), danger, disabled, lit)),
            }),
    );
    if item.has_subtitle() {
        // GPUI's sub-title: `text_xs` in `muted-foreground`, under the title.
        label = label.push(
            text(item.subtitle.clone())
                .size(theme::FONT_SIZE_XS)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        );
    }
    let label: Element<'a, Message> = if item.is_external() {
        row![label, muted_glyph::<Message>(EXTERNAL_LINK)]
            .spacing(theme::SPACING_XS)
            .width(Length::Fill)
            .align_y(iced::Alignment::Center)
            .into()
    } else {
        label.into()
    };
    line = line.push(label);

    line = line.push(if item.shortcut.is_empty() {
        empty_slot(SHORTCUT_SLOT)
    } else {
        // A fixed 48px column, as the Slint row has: the label owns everything
        // left of it. `Length::Fill` here shared the leftover width with the
        // label and wrapped labels that had room to spare.
        container(
            text(item.shortcut.clone())
                .size(theme::FONT_SIZE_XS)
                .font(theme::MONO)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        )
        .width(SHORTCUT_SLOT)
        .align_right(SHORTCUT_SLOT)
        .into()
    });

    // The trailing slot holds the check mark or the submenu chevron — a row is
    // one or the other, never both — and stays the same width either way, so the
    // shortcut column does not move between rows.
    line = line.push(if item.checked {
        container(glyph::<Message>(CHECK, danger, disabled, lit))
            .width(CHECK_SLOT)
            .center_x(CHECK_SLOT)
            .into()
    } else if item.has_submenu() {
        container(muted_glyph::<Message>(CHEVRON_RIGHT))
            .width(CHECK_SLOT)
            .center_x(CHECK_SLOT)
            .into()
    } else {
        empty_slot(CHECK_SLOT)
    });

    container(line)
        .height(Length::Shrink)
        .width(Length::Fill)
        .padding(Padding {
            top: ROW_PADDING_Y,
            right: theme::SPACING_SM,
            bottom: ROW_PADDING_Y,
            left: theme::SPACING_SM,
        })
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            // Hover and the selected state are the same accent surface: the
            // Slint row is `Theme.accent` while hovered, and GPUI's `MenuItem`
            // draws a selected row with the accent fill and `accent_foreground`
            // ink (`row_ink` above). In the dark theme the accent is `#262626`
            // on the `#171717` panel and in the light one `#f5f5f5` on white, so
            // the row lights up in both without either surface changing colour.
            container::Style {
                background: Some(if lit {
                    tokens.accent
                } else {
                    Color::TRANSPARENT
                }
                .into()),
                border: Border {
                    radius: theme::RADIUS_SM.into(),
                    ..Border::default()
                },
                ..container::Style::default()
            }
        })
        .into()
}

fn build_separator<'a, Message>() -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    // The rule is the `border` token, which is an edge in both appearances; a
    // fixed grey would vanish on one of them.
    container(
        container(Space::new().height(1.0).width(Length::Fill))
            .width(Length::Fill)
            .style(|theme: &Theme| container::Style {
                background: Some(Tokens::of(theme).border.into()),
                ..container::Style::default()
            }),
    )
    .height(SEPARATOR_HEIGHT)
    .width(Length::Fill)
    .center_y(SEPARATOR_HEIGHT)
    .into()
}

/// The colours a menu panel draws itself with.
///
/// [`MenuStyle::resolve`] builds one from the theme tokens, and
/// [`menu_container`] wraps a menu body in it for a caller that is not using a
/// [`Popover`](super::popover::Popover) to place the menu.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenuStyle {
    fill: Color,
    border: Color,
    border_width: f32,
    radius: f32,
}

impl MenuStyle {
    /// The menu the tokens describe: the popover fill, the border token as a
    /// one-pixel edge, and `radius-lg`. The edge is the token, not a fixed grey
    /// — `#e5e5e5` on white and `#262626` on near-black — so it exists in both
    /// appearances and the surface separates from the page in both.
    pub fn resolve(tokens: Tokens) -> Self {
        Self {
            fill: tokens.popover,
            border: tokens.border,
            border_width: 1.0,
            radius: theme::RADIUS_LG,
        }
    }

    pub fn fill(mut self, fill: Color) -> Self {
        self.fill = fill;
        self
    }

    pub fn border(mut self, border: Color, width: f32) -> Self {
        self.border = border;
        self.border_width = width;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn container_style(&self) -> container::Style {
        container::Style {
            background: Some(self.fill.into()),
            border: Border {
                color: self.border,
                width: self.border_width,
                radius: self.radius.into(),
            },
            ..container::Style::default()
        }
    }
}

impl Default for MenuStyle {
    fn default() -> Self {
        Self::resolve(Tokens::of(&Theme::Light))
    }
}

/// The panel's surface around a menu body, for a caller that is not using a
/// [`Popover`](super::popover::Popover) to place the menu. It draws the surface
/// and nothing else — the body already carries the 4px inset the Slint panel
/// has — so the caller passes [`Menu::into_element`] as `content` and gets one
/// panel, never a body surface inside a container surface.
pub fn menu_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    style: MenuStyle,
) -> Element<'a, Message>
where
    Message: 'a,
{
    container(content)
        .width(Length::Fill)
        .style(move |_theme: &Theme| style.container_style())
        .into()
}

impl<'a, Message> From<Menu<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(menu: Menu<'a, Message>) -> Self {
        menu.into_element()
    }
}

/// The check mark, drawn from `crates/slint-component/ui/icons/check.svg`
/// (Lucide, ISC), so the two galleries draw the same glyph rather than two
/// lookalikes. A row's own icons are the caller's; this one glyph is the
/// exception iced's own `checkbox` also makes.
const CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"/></svg>"#;

/// The submenu chevron, from `crates/slint-component/ui/icons/chevron-right.svg`
/// (Lucide, ISC) — the mark GPUI's `PopupMenuItem::Submenu` draws with
/// `IconName::ChevronRight` at the trailing edge.
const CHEVRON_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#;

/// The external-link mark, from
/// `crates/slint-component/ui/icons/external-link.svg` (Lucide, ISC) — the mark
/// GPUI's `PopupMenuItem::link` draws with `IconName::ExternalLink`.
const EXTERNAL_LINK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/></svg>"#;


