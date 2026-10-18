//! A searchable list of options with a free-text field. Mirrors GPUI
//! Component's `Combobox` and the Slint `Combobox` beside it, and is the overlay
//! family's second composition: `select`'s field-shaped trigger and floating
//! panel, with a search row in front of the menu's rows.
//!
//! # What it composes
//!
//! The Slint file is the authority, and it is four pieces:
//!
//! - the trigger is a field-shaped row — `Theme.input-height`,
//!   `Theme.radius-md`, a one-pixel `Theme.input` edge, the label and a
//!   `chevrons-up-down` at the trailing edge. That frame is [`input::Style`], the
//!   same frame [`Select`](super::select::Select) draws, resolved from the same
//!   tokens, so a combobox and an input beside it are the same box;
//! - the panel is the popover's one surface — `Theme.radius-lg`, a one-pixel
//!   `Theme.border` edge and the shadow — asked for `filled(true)` and
//!   `padding(0.0)`;
//! - the search row is a 36px band with a 14px search glyph and iced's own
//!   `text_input`, which is the field `input.rs` wraps without the frame: Slint
//!   draws it borderless inside the panel and this does too. A press inside the
//!   field focuses it, as it does anywhere in iced; the component does not move
//!   focus into the panel when the panel opens;
//! - the rows are [`Menu`], the item shape the menu, the dropdown button and the
//!   select already share: label, icon, shortcut, disabled, check mark and
//!   separators. The chosen row is the item marked `checked(true)`, exactly as
//!   the select marks its own choice.
//!
//! # One container owns the surface
//!
//! As [`Menu`]'s module comment requires, `Menu::into_element` hands back the
//! body — the 4px inset and the rows — and draws no surface. The panel is the
//! popover's; the search row, the separator and the footer are ordinary children
//! of that surface and draw no rectangle of their own. Drawing a menu surface
//! here as well would put a second rounded border inside the popover's.
//!
//! # The state is the caller's
//!
//! iced 0.14 keeps no state a component can own across frames, so this port has
//! none either. [`Combobox::open`] is what the panel is rendered from and
//! [`Combobox::on_open_change`] is the only way a trigger click, an outside click
//! or Escape is reported — the popover underneath supplies all three.
//! [`Combobox::query`] is the free text and [`Combobox::on_query`] is every edit;
//! the field holds no buffer of its own. [`Combobox::selected`] is the single
//! choice and [`Combobox::picked`] the multiple one, with
//! [`Combobox::on_chosen`] the click that changes either. The highlight, which a
//! menu cannot keep in the element tree it is rebuilt into, is
//! [`Combobox::highlighted`] and [`Combobox::on_highlight`], and both the pointer
//! and any key the application binds move it; [`Combobox::on_leave`] is the
//! hover's other half.
//!
//! # Filtering keeps the caller's indices
//!
//! The free text filters the list inside the component, the way the Slint
//! `hit()` does, but every index it reports is an index into the **caller's**
//! `items`, not into the filtered list: a query that hides the first two rows
//! still reports the third row as `2`. The component maps a displayed row back
//! to its original index before it publishes anything, and maps
//! [`Combobox::highlighted`] the other way, so the application never has to know
//! which rows the query hid.
//!
//! A row is chosen, not toggled: in single-select the caller answers with its own
//! `selected`, in multiple-select with `picked`, and in either case it decides
//! whether to close. The Slint component closes the popup and clears the query on
//! a single choice and leaves both alone on a multiple one; the gallery does the
//! same, because the component has no state to do it with.
//!
//! # What the Slint component has that this does not
//!
//! - **No groups and no badges.** The Slint `ComboItem` carries a `group` and a
//!   `badge`, and its list inserts a group heading and a trailing pill. [`Item`]
//!   is the menu's row shape and has neither column, so a grouped or badged list
//!   is not drawn here rather than approximated with a second row widget. A
//!   caller that wants headings can put a disabled item between groups, but this
//!   port does not pretend that is the same thing.
//! - **No pinned rows and no custom check glyph.** Slint pins a row (always
//!   checked, never toggled) and lets the caller replace the check icon. `Item`'s
//!   check is the menu's one glyph and it has no pinned state.
//! - **No trigger variants.** Slint's `ComboboxTrigger` picks a text, icon, chip,
//!   badges, overflow or count trigger. This port draws the text trigger, which
//!   is the component's default and the only one the shared field frame
//!   describes; a caller with a different trigger composes one from its own
//!   elements, as the gallery's footer does.
//! - **No `cleanable`, `menu_max_h`, `accessibility_label` or empty element.**
//!   The Slint component has no clear control on the iced side (the trigger is a
//!   container, not a field), no maximum panel height, no accessible name to
//!   attach and no custom empty element; a query that matches nothing draws the
//!   empty panel [`Menu`] draws, which the menu's own comment already promises.
//! - **No ellipsis.** The Slint trigger and every row elide a label wider than
//!   their box; iced 0.14 draws none, so the trigger clips at its edge and a row's
//!   label does the same, exactly as [`Select`](super::select::Select) and
//!   [`DropdownButton`](super::dropdown_button::DropdownButton) already name.
//! - **The keyboard belongs to the application.** iced 0.14 has no focus trap and
//!   no focus handle an overlay can take, so `↑`/`↓`/`Home`/`End`/`Enter`/`Space`
//!   are bound by the host and move [`Combobox::highlighted`], exactly as they are
//!   for the menu and the select. Escape and a click outside are the popover's for
//!   every overlay in this crate.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::widget::{Space, column, container, row, svg, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Pixels, Theme};

use super::input;
use super::menu::{self, Item, Menu};
use super::popover::{Placement, Popover};
use super::theme::{self, ControlSize, Tokens};

/// The search row's height, `height: 36px` in the Slint component.
const SEARCH_HEIGHT: f32 = 36.0;
/// The search glyph's box, `size: 14px`.
const SEARCH_ICON: f32 = 14.0;

/// One colour of the trigger's surface, as a function of the theme.
///
/// The appearance in force is not known where the trigger is built — a container
/// resolves its style inside the frame — so a surface colour has to be a function
/// of the theme rather than a value, the same reason [`Popover::panel`] takes one.
/// A value resolved once would be the wrong colour in the other appearance.
type ColorFor<'a> = Box<dyn Fn(&Theme) -> Color + 'a>;

/// A searchable list of options, built with the methods below.
///
/// It becomes an [`Element`] through `From`, and what it becomes is a
/// [`Popover`] whose trigger is the field-shaped frame and whose panel is a
/// search row, a separator and a [`Menu`] — which is what the Slint control is:
/// an input-shaped row with a `PopupWindow` holding a search field and the rows.
pub struct Combobox<'a, Message> {
    items: Vec<Item>,
    /// The single choice, or nothing while the placeholder shows. The Slint
    /// component spells it `selected: -1`.
    selected: Option<usize>,
    /// The multiple choice, as indices into `items`. The Slint component
    /// spells it `picked: [bool]` beside the items; a set of indices is the
    /// same information without a second vector to keep the same length.
    picked: Vec<usize>,
    /// Whether a click toggles into `picked` instead of setting `selected`.
    multiple: bool,
    /// The most rows `picked` may hold, or `0` for no limit — the Slint
    /// `max-selected`.
    max_selected: usize,
    /// The free text. The caller owns it: it sets this from its own state and
    /// takes the edits back through [`on_query`](Self::on_query).
    query: String,
    /// Whether the panel shows the search row. Slint's `searchable: true`.
    searchable: bool,
    placeholder: String,
    search_placeholder: String,
    size: ControlSize,
    disabled: bool,
    appearance: bool,
    open: bool,
    /// The row the caller says is highlighted — the row under the pointer, or the
    /// one a key moved to. See the module comment for why it is the caller's.
    highlighted: Option<usize>,
    width: Length,
    /// The panel's width, or `0` for the trigger's own where it is known.
    menu_width: f32,
    radius: Option<f32>,
    /// The trigger's fill and edge, as functions of the theme, because the
    /// appearance is not known where the trigger is built. [`input::Style`] is
    /// the default.
    background: Option<ColorFor<'a>>,
    border_color: Option<ColorFor<'a>>,
    /// The trailing glyph. The chevron is built in, because a combobox without a
    /// disclosure mark is not recognisable.
    caret: Option<Rc<dyn Fn() -> Element<'a, Message> + 'a>>,
    /// A persistent band under the rows — Slint's `footer: string` — for an
    /// action that belongs to the whole list, such as creating the item that was
    /// typed.
    footer: Option<Element<'a, Message>>,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    on_chosen: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    on_query: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_highlight: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    on_leave: Option<Rc<dyn Fn(Option<usize>) -> Message + 'a>>,
}

impl<'a, Message: Clone + 'static> Combobox<'a, Message> {
    /// A combobox over `items`, with nothing chosen, nothing typed and the panel
    /// closed.
    ///
    /// The rows are [`Item`]s, the shape the menu, the dropdown button and the
    /// select already use, so an option can carry an icon or a shortcut. A
    /// [`Item::separator`] is a band between groups of options; it is not
    /// actionable and is never the chosen row.
    pub fn new(items: impl IntoIterator<Item = Item>) -> Self {
        Self {
            items: items.into_iter().collect(),
            selected: None,
            picked: Vec::new(),
            multiple: false,
            max_selected: 0,
            query: String::new(),
            searchable: true,
            placeholder: String::from("Select…"),
            search_placeholder: String::from("Search…"),
            size: ControlSize::Medium,
            disabled: false,
            appearance: true,
            open: false,
            highlighted: None,
            width: Length::Fill,
            menu_width: 0.0,
            radius: None,
            background: None,
            border_color: None,
            caret: None,
            footer: None,
            on_open_change: None,
            on_chosen: None,
            on_query: None,
            on_highlight: None,
            on_leave: None,
        }
    }

    /// The single choice, the Slint component's `selected`. `None` shows the
    /// placeholder. It is ignored while [`multiple`](Self::multiple) is on.
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// The multiple choice, as indices into `items` — the Slint component's
    /// `picked`, whose rows carry a check mark while they are on. It is ignored
    /// while [`multiple`](Self::multiple) is off.
    pub fn picked(mut self, picked: impl IntoIterator<Item = usize>) -> Self {
        self.picked = picked.into_iter().collect();
        self
    }

    /// Toggle into [`picked`](Self::picked) instead of setting
    /// [`selected`](Self::selected). The Slint `multiple: true`; the panel stays
    /// open on a click that the caller does not close.
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// The most rows the multiple choice may hold, or `0`, the default, for no
    /// limit. The Slint `max-selected`; a row that would exceed it is drawn
    /// disabled and reports nothing.
    pub fn max_selected(mut self, max_selected: usize) -> Self {
        self.max_selected = max_selected;
        self
    }

    /// The free text, the Slint `query`. The caller owns it: it sets this from
    /// its own state and takes the edits back through
    /// [`on_query`](Self::on_query). It is what filters the rows.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    /// Whether the panel shows its search row, `true` by default — the Slint
    /// `searchable`.
    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    /// The text the trigger shows while nothing is chosen, `"Select…"` by
    /// default, which is the Slint component's own default.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// The text the search row shows while no query is typed, `"Search…"` by
    /// default, the Slint `search-placeholder`.
    pub fn search_placeholder(mut self, search_placeholder: impl Into<String>) -> Self {
        self.search_placeholder = search_placeholder.into();
        self
    }

    /// The size of the trigger, [`ControlSize::Medium`] by default. It picks the
    /// height, the inset, the type and the chevron together, the way the Slint
    /// component's `size` picks `Theme.input-height`, `input-padding-x`,
    /// `input-font-size` and `icon-size` together.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// A combobox that answers nothing: the trigger is drawn muted and the
    /// popover is told no open state, so it cannot be opened. It is the Slint
    /// `enabled: false`, drawn the way this crate draws a disabled field rather
    /// than at half opacity, which iced 0.14 has no subtree control for.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Draw the trigger's frame. `false` drops the fill, the edge and the radius,
    /// which is the Slint `appearance` and is for a host that supplies the
    /// surface itself.
    pub fn appearance(mut self, appearance: bool) -> Self {
        self.appearance = appearance;
        self
    }

    /// Whether the panel is showing. The caller owns it, and
    /// [`on_open_change`](Self::on_open_change) is how the caller learns it
    /// should change.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// The row drawn on the accent surface: the row the pointer is over, or the
    /// one a key moved to, by index into `items`. The caller owns it; see the
    /// module comment.
    pub fn highlighted(mut self, highlighted: Option<usize>) -> Self {
        self.highlighted = highlighted;
        self
    }

    /// The trigger's width, `Length::Fill` by default so the control spans the
    /// form column it is placed in.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// The panel's width. `0`, the default, means the trigger's own width where
    /// it is known — the Slint component's panel is its trigger's width — and a
    /// fixed trigger width is the only case iced can answer; a filling trigger
    /// falls back to the menu's preferred 280px. See
    /// [`Select::menu_width`](super::select::Select::menu_width).
    pub fn menu_width(mut self, width: f32) -> Self {
        self.menu_width = width;
        self
    }

    /// The trigger's corner radius. The default is the field's `radius-md`.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius);
        self
    }

    /// The trigger's fill, as a function of the theme, because the appearance in
    /// force is not known where the trigger is built. The default is the input
    /// frame's fill, `input-background`.
    pub fn background(mut self, background: impl Fn(&Theme) -> Color + 'a) -> Self {
        self.background = Some(Box::new(background));
        self
    }

    /// The trigger's edge, as a function of the theme, for the reason
    /// [`background`](Self::background) takes one. The default is the input
    /// frame's edge, the `input` token.
    pub fn border_color(mut self, border_color: impl Fn(&Theme) -> Color + 'a) -> Self {
        self.border_color = Some(Box::new(border_color));
        self
    }

    /// The disclosure glyph at the trailing edge, replacing the built-in
    /// chevrons.
    pub fn caret(mut self, caret: impl Fn() -> Element<'a, Message> + 'a) -> Self {
        self.caret = Some(Rc::new(caret));
        self
    }

    /// A persistent band under the rows — the Slint `footer` — for an action
    /// that belongs to the whole list, such as creating the item that was typed.
    ///
    /// The content is the caller's, inset and all: the component draws the band's
    /// one-pixel top edge and places the element under it, so the caller owns the
    /// shape and the click. iced has no dialog footers to match here, and a
    /// caller that wants the Slint band writes it — the gallery does — rather
    /// than the component guessing an inset for it.
    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Report every request to change the open state: a trigger click wants
    /// `true`, an outside click or Escape wants `false`. A disabled combobox is
    /// given no callback, so its trigger cannot open anything — this crate's
    /// spelling for a control that answers nothing.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Report the row that was chosen, by index into `items`, on a click. The
    /// caller owns the items and asks itself what index 2 means; the Slint
    /// `changed(index)` and iced's own `combo_box` report the same number.
    ///
    /// In single-select the caller answers with [`selected`](Self::selected), in
    /// multiple-select it adds or removes the index in [`picked`](Self::picked).
    /// The panel is not closed by a choice: the caller answers with its own
    /// `open`, exactly as the Slint example's row sets `selected`, calls
    /// `changed` and closes the popup.
    pub fn on_chosen(mut self, on_chosen: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_chosen = Some(Rc::new(on_chosen));
        self
    }

    /// Every edit of the free text, as the value the field now holds, the Slint
    /// `query`'s binding. The component filters the rows from the value the
    /// caller passes back to [`query`](Self::query).
    pub fn on_query(mut self, on_query: impl Fn(String) -> Message + 'a) -> Self {
        self.on_query = Some(Box::new(on_query));
        self
    }

    /// Report the row the pointer entered, by index into `items`, the way
    /// [`Menu::on_highlight`] does. The caller draws it by passing the same index
    /// to [`highlighted`](Self::highlighted).
    ///
    /// It is a **hover**, as it is on the menu, so it is transient: pass the
    /// matching [`on_leave`](Self::on_leave) to clear it. A composer that also
    /// owns the row a key moved to keeps the two apart, or the pointer's leave
    /// takes the keyboard's row with it.
    pub fn on_highlight(mut self, on_highlight: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_highlight = Some(Rc::new(on_highlight));
        self
    }

    /// Report that the pointer left a row, or left the panel: `Some(index)` is
    /// the row it left, by index into `items`, and `None` is the panel as a
    /// whole. It is the half that clears the hover
    /// [`on_highlight`](Self::on_highlight) sets.
    pub fn on_leave(mut self, on_leave: impl Fn(Option<usize>) -> Message + 'a) -> Self {
        self.on_leave = Some(Rc::new(on_leave));
        self
    }

    /// The whole control: the Popover that floats the search row and the Menu,
    /// with the field-shaped trigger.
    fn into_element(self) -> Element<'a, Message> {
        let Combobox {
            items,
            selected,
            picked,
            multiple,
            max_selected,
            query,
            searchable,
            placeholder,
            search_placeholder,
            size,
            disabled,
            appearance,
            open,
            highlighted,
            width,
            menu_width,
            radius,
            background,
            border_color,
            caret,
            footer,
            on_open_change,
            on_chosen,
            on_query,
            on_highlight,
            on_leave,
        } = self;

        // The query filters without changing what an index means. `origins` is
        // the displayed row's index into the caller's `items`, and every report
        // below crosses back through it. See the module comment.
        let needle = query.to_lowercase();
        let visible: Vec<(usize, Item)> = items
            .iter()
            .enumerate()
            .filter(|(_, item)| needle.is_empty() || item.label().to_lowercase().contains(&needle))
            .map(|(index, item)| (index, item.clone()))
            .collect();
        let origins: Rc<Vec<usize>> = Rc::new(visible.iter().map(|(index, _)| *index).collect());

        // A row is drawn chosen with the menu's own check mark, which is where the
        // Slint row puts `Icon { name: root.check-icon }` when `row-checked` is
        // true. At capacity the rows that could still be picked are dimmed rather
        // than refused silently: the Slint `at-capacity` block is the same.
        let at_capacity = multiple && max_selected > 0 && picked.len() >= max_selected;
        let rows: Vec<Item> = visible
            .iter()
            .map(|(index, item)| {
                let on = if multiple {
                    picked.contains(index)
                } else {
                    selected == Some(*index)
                };
                let mut row = item.clone();
                if on {
                    row = row.checked(true);
                } else if at_capacity {
                    row = row.disabled(true);
                }
                row
            })
            .collect();

        // The highlight travels in the caller's indices too, so a hidden row
        // cannot leave the highlight on a row the caller did not name.
        let displayed_highlight =
            highlighted.and_then(|index| origins.iter().position(|origin| *origin == index));

        let has_value = if multiple {
            !picked.is_empty()
        } else {
            selected.is_some()
        };
        let label = if multiple {
            let joined = picked
                .iter()
                .filter_map(|index| items.get(*index))
                .map(Item::label)
                .collect::<Vec<_>>()
                .join(", ");
            if joined.is_empty() {
                placeholder.clone()
            } else {
                joined
            }
        } else {
            match selected.and_then(|index| items.get(index)) {
                Some(item) => item.label().to_string(),
                None => placeholder.clone(),
            }
        };

        // Slint's panel is its trigger's width, which is a fixed trigger width in
        // the one case iced can answer before layout. See `Select::menu_width`.
        let panel_width = if menu_width > 0.0 {
            menu_width
        } else if let Length::Fixed(fixed) = width {
            fixed
        } else {
            menu::PANEL_WIDTH
        };

        let caret: Element<'a, Message> = match caret {
            Some(caret) => caret(),
            None => svg(svg::Handle::from_memory(CHEVRONS_UP_DOWN))
                .width(size.icon_size())
                .height(size.icon_size())
                .style(move |theme: &Theme, _status| svg::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                })
                .into(),
        };

        // The frame is the field's, so its padding is the field's rule: the
        // horizontal inset is the authority's, and the vertical is what the
        // label's line leaves of the frame (`input::field_inset_y`). Padding all
        // four sides with `input_padding_x` left 8px of a 24px `small` trigger
        // for a 14px label, and iced clipped the label against it.
        let inset_x = size.input_padding_x();
        let inset_y = input::field_inset_y(size.input_height(), size.input_font_size(), inset_x);

        let trigger = container(
            row![
                text(label)
                    .size(size.input_font_size())
                    .width(Length::Fill)
                    // iced 0.14 draws no ellipsis, so the value clips at the
                    // trigger's edge where Slint elides. See the module comment.
                    .wrapping(text::Wrapping::None),
                caret,
            ]
            .spacing(theme::SPACING_SM)
            .align_y(Alignment::Center)
            // The row has to fill the trigger's height: a `Shrink` row is
            // aligned to the top of the fixed-height frame, so `align_y` would
            // centre the label and the caret against the row's own top edge and
            // the row would ride high. `input.rs` fills its row for the same
            // reason.
            .height(Length::Fill)
            .width(Length::Fill),
        )
        .padding([inset_y, inset_x])
        .height(Length::Fixed(size.input_height()))
        .width(width)
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            let mut style = input::Style::new()
                .appearance(appearance)
                .disabled(disabled)
                .resolve(tokens);
            if let Some(radius) = radius {
                style.border.radius = radius.into();
            }
            if let Some(background) = &background {
                style.background = Some(background(theme).into());
            }
            if let Some(border_color) = &border_color {
                style.border.color = border_color(theme);
            }
            // A chosen value reads as a value, a placeholder and a disabled value
            // read as muted, the same reading the select gives the same frame.
            style.text_color = Some(if disabled {
                tokens.muted_foreground
            } else if has_value {
                tokens.foreground
            } else {
                tokens.muted_foreground
            });
            style
        });

        // One surface, and the popover owns it. `Menu::into_element` hands back
        // the body — the 4px inset and the rows — so `filled(true)` here is the
        // panel the eye sees. `padding(0.0)` keeps that surface on the body
        // itself: the search row and the menu carry their own insets, so a
        // nonzero popover inset would put the rectangles at different sizes.
        let mut popover = Popover::new(trigger)
            .open(open && !disabled)
            .placement(Placement::Bottom)
            .offset(4.0)
            .panel_width(panel_width)
            .filled(true)
            .padding(0.0);

        // A disabled control gets no report, so its trigger cannot open anything.
        if !disabled && let Some(on_open_change) = on_open_change {
            popover = popover.on_open_change(on_open_change);
        }

        let mut menu = Menu::new(rows).highlighted(displayed_highlight);
        if let Some(on_chosen) = on_chosen.clone() {
            let origins = origins.clone();
            menu = menu.on_chosen(move |position| on_chosen(origins[position]));
        }
        if let Some(on_highlight) = on_highlight {
            let origins = origins.clone();
            menu = menu.on_highlight(move |position| on_highlight(origins[position]));
        }
        if let Some(on_leave) = on_leave {
            let origins = origins.clone();
            menu = menu.on_leave(move |row| on_leave(row.map(|position| origins[position])));
        }

        let mut body = column![].width(Length::Fill);
        if searchable {
            body = body.push(search_row(query, search_placeholder, on_query));
            body = body.push(rule());
        }
        body = body.push(menu.into_element());
        if let Some(footer) = footer {
            body = body.push(rule());
            body = body.push(footer);
        }

        popover.content(body).into()
    }
}

impl<'a, Message> From<Combobox<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'static,
{
    fn from(combobox: Combobox<'a, Message>) -> Self {
        combobox.into_element()
    }
}

/// The panel's search row: a 14px glyph and iced's own `text_input` inside the
/// popover's surface, with no frame of its own — the Slint `Rectangle`'s
/// `height: 36px`, `padding-left/right: 12px`, `spacing: 8px`.
///
/// The field is a plain `text_input` rather than [`input::Input`] because an
/// [`Input`](super::input::Input) is keyed by a caller-supplied
/// [`Id`](iced::widget::Id), and a component that may appear several times on one
/// page has no name to give it. Without `.id()` iced keys the field's editing
/// state by its position in the element tree, which is unique where the row is
/// and stable across frames; the styling is the same style closure
/// `input.rs` gives its own field.
fn search_row<'a, Message: Clone + 'a>(
    query: String,
    placeholder: String,
    on_query: Option<Box<dyn Fn(String) -> Message + 'a>>,
) -> Element<'a, Message> {
    let mut field = text_input(&placeholder, &query)
        .padding(Padding::ZERO)
        .size(theme::FONT_SIZE_SM)
        // A line box as tall as the row, the same reason `input.rs` sets one: a
        // bare `f32` is a multiple of the font size, and a missing line box
        // leaves the click target one line tall in the middle of the row.
        .line_height(Pixels(SEARCH_HEIGHT - 2.0))
        .width(Length::Fill)
        .style(|theme: &Theme, status| {
            let tokens = Tokens::of(theme);
            let dimmed = matches!(status, text_input::Status::Disabled);
            text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border: Border::default(),
                icon: tokens.muted_foreground,
                placeholder: tokens.muted_foreground,
                value: if dimmed {
                    tokens.muted_foreground
                } else {
                    tokens.foreground
                },
                selection: theme::alpha(tokens.selection, 0.35),
            }
        });
    if let Some(on_query) = on_query {
        field = field.on_input(on_query);
    }

    let glyph: Element<'a, Message> = svg(svg::Handle::from_memory(SEARCH))
        .width(SEARCH_ICON)
        .height(SEARCH_ICON)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into();

    container(
        row![glyph, field]
            .spacing(theme::SPACING_SM)
            .align_y(Alignment::Center)
            // The row has to fill the search frame's height: a `Shrink` row is
            // aligned to the top of the fixed-height frame, so the glyph and
            // the field ride high. The field's line box is two pixels shorter
            // than the frame, which is exactly what put the row's own centre a
            // pixel above the frame's.
            .height(Length::Fill)
            .width(Length::Fill),
    )
    .height(Length::Fixed(SEARCH_HEIGHT))
    .padding([0.0, theme::SPACING_MD])
    .width(Length::Fill)
    .into()
}

/// A one-pixel rule across the panel: the search row's own edge and the footer's.
/// The colour is the `border` token, which is an edge in both appearances; a
/// fixed grey would vanish on one of them.
fn rule<'a, Message: 'a>() -> Element<'a, Message> {
    container(Space::new().height(1.0).width(Length::Fill))
        .width(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).border.into()),
            ..container::Style::default()
        })
        .into()
}

/// The disclosure mark at the trigger's trailing edge, drawn from
/// `crates/slint-component/ui/icons/chevrons-up-down.svg` (Lucide, ISC), so the
/// two galleries draw the same glyph rather than two lookalikes.
const CHEVRONS_UP_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/></svg>"#;

/// The search glyph, drawn from `crates/slint-component/ui/icons/search.svg`
/// (Lucide, ISC), the same file the Slint combobox names.
const SEARCH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21 21-4.34-4.34"/><circle cx="11" cy="11" r="8"/></svg>"#;
