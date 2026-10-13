//! A list of actions. Mirrors GPUI Component's `PopupMenu` and the Slint
//! `Menu` beside it, and is the item shape `select` and `combobox` compose.
//!
//! # The surface, and what the menu is
//!
//! The Slint `Menu` is the whole panel: `preferred-width: 280px`, a popover
//! fill, a one-pixel border, `radius-lg` and a 4px inset around its rows. This
//! file is that body — the rows, their 32px height, the separator's 9px with a
//! one-pixel rule, the columns that stay put when a row has no icon. The
//! floating is [`Popover`](super::popover::Popover)'s job, because a menu with
//! no anchor is not a control: every card in the Slint example puts one inside
//! a `PopupWindow`.
//!
//! A caller that wants the panel's own surface on its own draws it with
//! [`MenuStyle`], the way [`PanelStyle`](super::popover::PanelStyle) draws a
//! popover's. A menu in a popover asks the popover for `filled(true)` instead,
//! so the body and its surface are one surface rather than two.
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
//! A row reports the pointer entering it through [`Menu::on_highlight`], and it
//! is drawn on the accent surface when the caller passes that index back to
//! [`Menu::highlighted`]. The highlight cannot live in the menu itself, because
//! the menu becomes an element rather than a widget of its own; the caller
//! keeping it is also iced's own arrangement — `pick_list` holds
//! `hovered_option` in the application's state and its menu reports
//! `on_option_hovered` so the application can follow along.
//!
//! So the slider of the menu — which row is lit — is one index the application
//! owns, and both the pointer and any key the application binds move it. The
//! demo's "Keyboard Shortcuts" card shows the arrangement: a row chosen is
//! [`Menu::on_chosen`]'s index, a row hovered is `on_highlight`'s.
//!
//! # What is not here
//!
//! iced 0.14 has no focus trap and no menu role, so the traversal keys are not
//! the component's: an application binds `↑`/`↓`/`Home`/`End`/`Enter` and moves
//! the highlight itself. `Escape` and a click outside belong to the popover the
//! menu sits in, which owns those two dismissals for every overlay in this
//! crate. `Tab` does not move focus out of a menu; there are no submenus and no
//! link rows. Slint's `Menu` has none of those either; GPUI's `PopupMenu` has
//! all of them, and this port names the gap rather than drawing a `›` that
//! opens nothing.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{column, container, mouse_area, row, svg, text, Space};
use iced::{Border, Color, Element, Length, Padding, Theme};

use super::theme::{self, Tokens};

/// The rows' height, `32px` in the Slint component.
pub const ROW_HEIGHT: f32 = 32.0;
/// The height a separator occupies, `9px` in the Slint component: 4px, the
/// rule, and 4px again.
pub const SEPARATOR_HEIGHT: f32 = 9.0;
/// The inset around the rows, `padding: 4px` in the Slint component.
pub const PANEL_PADDING: f32 = 4.0;
/// The panel's preferred width, `preferred-width: 280px`, which is
/// [`Popover::panel_width`](super::popover::Popover::panel_width)'s default too.
pub const PANEL_WIDTH: f32 = 280.0;
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
    icon: Option<&'static [u8]>,
    shortcut: String,
    disabled: bool,
    checked: bool,
    danger: bool,
    separator: bool,
}

impl Item {
    /// An actionable row with `label`.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            shortcut: String::new(),
            disabled: false,
            checked: false,
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

    /// The shortcut shown at the trailing edge, right-aligned in a 48px column
    /// in the monospace face, `⌘N` in the Slint example.
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

    pub fn label(&self) -> &str {
        &self.label
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
/// pointer is drawn as, and the caller moves it from [`Menu::on_highlight`] —
/// which a `mouse_area` fires on entry. iced cannot put the traversal inside the
/// element without the tree the last paragraph describes, so this component
/// reports and the caller decides, exactly as iced's own `pick_list` keeps its
/// `hovered_option` in the application's state.
pub struct Menu<'a, Message> {
    items: Vec<Item>,
    on_chosen: Option<Callback<'a, Message>>,
    on_highlight: Option<Callback<'a, Message>>,
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

impl<'a, Message> Menu<'a, Message> {
    /// A menu of `items`. An empty menu draws an empty panel rather than
    /// nothing: `select` composes this with a list that may filter to nothing.
    pub fn new(items: impl IntoIterator<Item = Item>) -> Self {
        Self {
            items: items.into_iter().collect(),
            on_chosen: None,
            on_highlight: None,
            selected: None,
            highlighted: None,
        }
    }

    /// Report the row that was chosen, by index, on a click.
    ///
    /// This is the whole contract with `select`: the caller owns the items and
    /// asks itself what index 2 means. iced's `combo_box` reports `on_select`
    /// the same way, and the Slint component's `chosen(index: int)` is the same
    /// number.
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
    /// This is iced `combo_box`'s `on_option_hovered`.
    pub fn on_highlight(mut self, on_highlight: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_highlight = Some(std::rc::Rc::new(on_highlight));
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
        let highlighted = self.highlighted == Some(index);
        let selected = self.selected == Some(index);

        let row = build_row(item, highlighted, selected, self.items.len(), index);
        let Some(on_chosen) = &self.on_chosen else {
            return row;
        };
        let on_chosen = on_chosen.clone();
        let Some(on_highlight) = &self.on_highlight else {
            return row;
        };
        let on_highlight = on_highlight.clone();
        mouse_area(row)
            .on_enter(on_highlight(index))
            .on_press(on_chosen(index))
            .into()
    }

    /// The panel's body, without a surface: the 4px inset and a row per item.
    /// The [`Popover`](super::popover::Popover) draws the surface around it.
    pub fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let mut rows = column![].spacing(0.0).width(Length::Fill);
        for index in 0..self.items.len() {
            rows = rows.push(self.row_element(index));
        }
        // The panel's own surface travels with the menu body, so a caller that
        // drops the menu into a bare popover — `Popover::filled(false)` — still
        // gets the popover fill, the border token as a one-pixel edge and
        // `radius-lg`, which is what the Slint `Menu` draws around its rows.
        // `Popover::filled(true)` draws the same surface and this one over it,
        // which is why the two are never both asked for.
        container(container(rows).padding(PANEL_PADDING).width(Length::Fill))
            .width(Length::Fill)
            .style(|theme: &Theme| MenuStyle::resolve(Tokens::of(theme)).container_style())
            .into()
    }
}

/// What a row's slot holds when the item leaves it empty. The slot still
/// occupies its width, so labels and shortcuts line up down the menu, which is
/// what the Slint component's empty `Rectangle` is for.
fn empty_slot<'a, Message: 'a>(width: f32) -> Element<'a, Message> {
    Space::new().width(width).height(Length::Shrink).into()
}

/// The ink a row's label, icon and check mark are drawn in.
fn row_ink(tokens: Tokens, danger: bool, disabled: bool) -> Color {
    if disabled {
        tokens.muted_foreground
    } else if danger {
        tokens.destructive
    } else {
        tokens.foreground
    }
}

/// The 14px SVG a row's leading or trailing slot holds.
fn glyph<'a, Message: 'a>(bytes: &'static [u8], danger: bool, disabled: bool) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(14.0)
        .height(14.0)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(row_ink(Tokens::of(theme), danger, disabled)),
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

    // Every row keeps the same four columns — a leading icon slot, the label,
    // the shortcut, and a trailing check slot — so a row that leaves one empty
    // still occupies it and the columns line up down the menu.
    let mut line = row![]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center);

    line = line.push(match item.icon {
        Some(icon) => container(glyph::<Message>(icon, danger, disabled))
            .width(ICON_SLOT)
            .center_x(ICON_SLOT)
            .into(),
        None => empty_slot(ICON_SLOT),
    });

    line = line.push(
        text(item.label.clone())
            .size(theme::FONT_SIZE_SM)
            .width(Length::Fill)
            .style(move |theme: &Theme| text::Style {
                color: Some(row_ink(Tokens::of(theme), danger, disabled)),
            }),
    );

    line = line.push(if item.shortcut.is_empty() {
        empty_slot(SHORTCUT_SLOT)
    } else {
        container(
            text(item.shortcut.clone())
                .size(theme::FONT_SIZE_XS)
                .font(theme::MONO)
                .align_x(iced::Alignment::End)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        )
        .width(SHORTCUT_SLOT)
        .align_right(Length::Fill)
        .into()
    });

    line = line.push(if item.checked {
        container(glyph::<Message>(CHECK, danger, disabled))
            .width(CHECK_SLOT)
            .center_x(CHECK_SLOT)
            .into()
    } else {
        empty_slot(CHECK_SLOT)
    });

    container(line)
        .height(ROW_HEIGHT)
        .width(Length::Fill)
        .padding(Padding {
            top: 0.0,
            right: theme::SPACING_SM,
            bottom: 0.0,
            left: theme::SPACING_SM,
        })
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            // Hover and the selected state are the same accent surface: the
            // Slint row is `Theme.accent` while hovered, and GPUI's `MenuItem`
            // draws a selected row with the accent fill and its foreground too.
            // In the dark theme the accent is `#262626` on the `#171717` panel
            // and in the light one `#f5f5f5` on white, so the row lights up in
            // both without either surface changing colour.
            let fill = if (highlighted || selected) && !disabled {
                tokens.accent
            } else {
                Color::TRANSPARENT
            };
            container::Style {
                background: Some(fill.into()),
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
/// [`Popover`](super::popover::Popover) to place the menu. It adds the 4px
/// inset the Slint panel has, so the caller passes the menu itself as `content`.
pub fn menu_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    style: MenuStyle,
) -> Element<'a, Message>
where
    Message: 'a,
{
    container(content)
        .padding(PANEL_PADDING)
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


