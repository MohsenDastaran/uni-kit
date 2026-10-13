//! A command palette: a filtered list of commands, grouped, with a search field,
//! keyboard hints and a scrolling list. Mirrors GPUI Component's `Command` and
//! the Slint `Command` beside it.
//!
//! # What it composes
//!
//! The Slint file is the authority, and it is four pieces this port borrows
//! rather than re-derives:
//!
//! - the query field is [`Input`] with `appearance(false)` — the field's frame
//!   dropped because the panel owns the surface, and the search glyph in its
//!   prefix slot. Slint draws a bare `TextInput` in the row for the same reason;
//! - a row is a [`menu::Item`] for the columns the menu already has — label,
//!   icon, shortcut, disabled, checked, separator — plus the columns a command
//!   adds: the group it belongs to, the keywords it matches on, and the
//!   detail/meta/change a custom or rich row shows;
//! - the shortcuts and the hints band are [`Kbd`], the chip the menu rows and
//!   the kbd page already share;
//! - the surface is [`PanelStyle`]'s, resolved from the tokens inside the style
//!   closure. This file draws no panel of its own: writing a second draw is how
//!   a white table was once painted in the dark theme.
//!
//! # The state is the caller's
//!
//! iced 0.14 keeps no state a component can own across frames, so this port has
//! none either. [`Command::query`] is the free text and [`Command::on_query`] is
//! every edit; the field holds no buffer of its own. [`Command::active`] is the
//! row drawn on the accent surface — the row a key moved to, or the one the
//! pointer is over, and the caller decides which — and [`Command::on_highlight`]
//! and [`Command::on_leave`] are the pointer's half of it. [`Command::on_chosen`]
//! is the click; the palette is never closed by it, because the caller owns
//! whether it is in a dialog or on the page.
//!
//! # Filtering keeps the caller's indices
//!
//! The query matches, case-insensitively, against a row's label, detail,
//! keywords and meta, and filters the list inside the component, the way the
//! Slint `counts()` does. Every index it reports is an index into the **caller's**
//! items, not into the filtered list: a query that hides the first two rows still
//! reports the third row as `2`. [`visible_indices`] is the same reading for a
//! host that binds the traversal keys, so the application's stepping and the
//! panel's drawing cannot disagree. A group heading shows only while one of its
//! rows is visible, and a separator shows only while the query is empty, which is
//! the Slint rule.
//!
//! # The keyboard belongs to the application
//!
//! iced 0.14 has no focus trap and no focus handle a component can take, so
//! `↑`/`↓`/`Home`/`End`/`Enter`/`Space` are bound by the host, which moves
//! [`Command::active`], exactly as they are for the menu, the select and the
//! combobox. Escape is the host's too: the component is an ordinary element, so
//! it is handed a key only by a focused control, and it exposes no
//! `on_cancelled` that could never fire. [`visible_indices`] and
//! [`CommandItem::is_actionable`] are what the host steps with.
//!
//! # What the Slint component has that this does not
//!
//! - **No virtualization.** Slint's list is a plain `VerticalLayout` inside a
//!   `Scrollable` too, so the two agree here; GPUI's `v_virtual_list` is what this
//!   port does not copy, and a palette long enough to need it is the caller's
//!   business.
//! - **No ellipsis.** Slint elides a label wider than its row; iced 0.14 draws
//!   none, so a long label clips at the row's edge — the same gap `menu`,
//!   `select` and `combobox` already name.
//! - **No animated hover and no subtree opacity.** Slint fades a disabled row to
//!   0.45 and lightens a hovered one; iced cannot do either on a subtree, so a
//!   disabled row is drawn muted and a hovered one is the caller's accent row.
//! - **No `cancelled` callback.** See the keyboard section: Escape never reaches
//!   the component.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::widget::{Id, Space, column, container, mouse_area, row, svg, text};
use iced::{Alignment, Border, Color, Element, Length, Padding, Theme};

use super::input::Input;
use super::kbd::Kbd;
use super::menu::Item;
use super::popover::PanelStyle;
use super::scrollable::Scrollable;
use super::theme::{self, ControlSize, Tokens};

/// The heading band's height, `height: 40px` in the Slint component.
const HEADING_HEIGHT: f32 = 40.0;
/// The search row's height, `height: 36px` in the Slint component.
const SEARCH_HEIGHT: f32 = 36.0;
/// The search glyph's box, `size: 14px`.
const SEARCH_ICON: f32 = 14.0;
/// A row's height, `max(32px, body.preferred-height)` in the Slint component.
const ROW_HEIGHT: f32 = 32.0;
/// The inset around the list's rows, `padding: 4px` in the Slint component.
const LIST_PADDING: f32 = 4.0;
/// The gap between rows, `spacing: 2px`.
const LIST_SPACING: f32 = 2.0;
/// The leading icon slot, `size: 16px`.
const ICON_SLOT: f32 = 16.0;
/// The trailing check slot, `size: 14px`.
const CHECK_SLOT: f32 = 14.0;
/// The palette's preferred width, `preferred-width: 380px`.
const PALETTE_WIDTH: f32 = 380.0;

/// One row of a [`Command`].
///
/// The columns are the Slint `CommandItem` struct's. The label, icon, shortcut,
/// disabled, checked and separator columns are the shared [`menu::Item`]'s — a
/// command row is the same row a menu draws — and the rest are what a command
/// adds: the `group` heading it sits under, the `keywords` the query matches
/// besides the label, and the `detail` (a custom row's second line), `meta` and
/// `change` (a rich row's right-hand column, coloured up or down).
#[derive(Debug, Clone, PartialEq)]
pub struct CommandItem {
    row: Item,
    group: String,
    keywords: String,
    detail: String,
    meta: String,
    change: String,
    up: bool,
}

impl CommandItem {
    /// An actionable row with `label`.
    pub fn new(label: impl Into<String>) -> Self {
        Self::from_row(Item::new(label))
    }

    /// A rule between groups of rows, the Slint `separator: true`. It shows only
    /// while the query is empty and answers no pointer or key.
    pub fn separator() -> Self {
        Self::from_row(Item::separator())
    }

    fn from_row(row: Item) -> Self {
        Self {
            row,
            group: String::new(),
            keywords: String::new(),
            detail: String::new(),
            meta: String::new(),
            change: String::new(),
            up: false,
        }
    }

    /// The SVG drawn in the leading 16px slot. The bytes are the caller's, as
    /// every icon in this crate is.
    pub fn icon(mut self, icon: &'static [u8]) -> Self {
        self.row = self.row.icon(icon);
        self
    }

    /// The shortcut shown at the trailing edge, in a [`Kbd`] chip.
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.row = self.row.shortcut(shortcut);
        self
    }

    /// A row that answers no pointer or key, `disabled: true`.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.row = self.row.disabled(disabled);
        self
    }

    /// The check mark in the trailing slot, `checked: true`.
    pub fn checked(mut self, checked: bool) -> Self {
        self.row = self.row.checked(checked);
        self
    }

    /// The heading this row sits under, the Slint `group`. A heading shows only
    /// while one of its rows is visible, so a group whose rows all filter out
    /// hides it.
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = group.into();
        self
    }

    /// Extra match terms besides the label, the Slint `keywords`. The query
    /// matches the label, the detail, the keywords and the meta together, which
    /// is what the Slint `counts()` does.
    pub fn keywords(mut self, keywords: impl Into<String>) -> Self {
        self.keywords = keywords.into();
        self
    }

    /// The second line of a custom row, drawn muted under the label while
    /// [`Command::custom_rows`] is on. A non-empty detail also gives the row its
    /// taller inset, which is the one variable-height row the Slint example
    /// shows.
    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }

    /// The rich row's right-hand value, drawn bold beside the label. A row with
    /// a meta is drawn in the two-column shape rather than the icon-and-label
    /// one, the Slint `item.meta != ""` branch.
    pub fn meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = meta.into();
        self
    }

    /// The rich row's second right-hand line, drawn in the success colour while
    /// [`up`](Self::up) and the danger colour otherwise.
    pub fn change(mut self, change: impl Into<String>) -> Self {
        self.change = change.into();
        self
    }

    /// Whether [`change`](Self::change) reads as a rise. Slint's `up`.
    pub fn up(mut self, up: bool) -> Self {
        self.up = up;
        self
    }

    /// Whether the row answers a pointer or a key: not a separator and not
    /// disabled.
    pub fn is_actionable(&self) -> bool {
        self.row.is_actionable()
    }
}

/// The indices of `items` a query leaves visible, in order, with the preview
/// rule applied — the reading the component draws from, for a host that binds
/// the traversal keys and has to step over the same rows. Separator indices are
/// included while the query is empty; filter with
/// [`CommandItem::is_actionable`] to step over only the rows that answer.
pub fn visible_indices(items: &[CommandItem], query: &str, preview_count: usize) -> Vec<usize> {
    let needle = query.to_lowercase();
    (0..items.len())
        .filter(|index| shown(items, *index, &needle, preview_count))
        .collect()
}

/// Whether a row matches the query. An empty needle matches everything, which is
/// the preview and the whole-list case.
fn matches(item: &CommandItem, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    let haystack = format!(
        "{} {} {} {}",
        item.row.label(),
        item.detail,
        item.keywords,
        item.meta
    )
    .to_lowercase();
    haystack.contains(needle)
}

/// Whether a row is drawn at all: every separator while the query is empty, the
/// first `preview_count` rows while the query is empty and a preview is set, and
/// otherwise the rows the query matches.
fn shown(items: &[CommandItem], index: usize, needle: &str, preview_count: usize) -> bool {
    let Some(item) = items.get(index) else {
        return false;
    };
    if item.row.is_separator() {
        return needle.is_empty();
    }
    if needle.is_empty() && preview_count > 0 {
        return index < preview_count;
    }
    matches(item, needle)
}

/// Whether `index` opens a group heading: it is a shown, grouped row and no
/// earlier shown row carries the same group. The Slint reads two rows back
/// because its rows are laid out consecutively; asking every earlier row is the
/// same rule and does not depend on the neighbours.
fn needs_header(items: &[CommandItem], index: usize, needle: &str, preview_count: usize) -> bool {
    if !shown(items, index, needle, preview_count) || items[index].row.is_separator() {
        return false;
    }
    let group = &items[index].group;
    if group.is_empty() {
        return false;
    }
    !(0..index).any(|earlier| {
        shown(items, earlier, needle, preview_count) && items[earlier].group == *group
    })
}

/// A command palette, built with the methods below.
///
/// It becomes an [`Element`] through `From`, and what it becomes is the panel:
/// an optional heading band, an optional [`Input`] search row, the filtered rows
/// and an optional hints band, inside [`PanelStyle`]'s surface while
/// [`bordered`](Self::bordered) is on.
pub struct Command<'a, Message> {
    items: Vec<CommandItem>,
    query: String,
    active: Option<usize>,
    placeholder: String,
    searchable: bool,
    bordered: bool,
    heading: String,
    hints: bool,
    preview_count: usize,
    custom_rows: bool,
    max_list_height: f32,
    width: Length,
    /// The name iced keeps the search field's editing state under. It has to be
    /// stable across frames, and two palettes on one page must not share it, so
    /// a caller with more than one gives each its own.
    search_id: Id,
    on_chosen: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    on_highlight: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    on_leave: Option<Rc<dyn Fn(Option<usize>) -> Message + 'a>>,
    on_query: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

impl<'a, Message: Clone + 'static> Command<'a, Message> {
    /// A searchable, bordered palette over `items`, with nothing typed and no
    /// row lit.
    ///
    /// The rows are [`CommandItem`]s. A [`CommandItem::separator`] is a rule
    /// between groups; it is not actionable and never carries the accent.
    pub fn new(items: impl IntoIterator<Item = CommandItem>) -> Self {
        Self {
            items: items.into_iter().collect(),
            query: String::new(),
            active: None,
            placeholder: String::from("Type a command or search..."),
            searchable: true,
            bordered: true,
            heading: String::new(),
            hints: false,
            preview_count: 0,
            custom_rows: false,
            max_list_height: 0.0,
            width: Length::Fixed(PALETTE_WIDTH),
            search_id: Id::new("command-search"),
            on_chosen: None,
            on_highlight: None,
            on_leave: None,
            on_query: None,
        }
    }

    /// The free text, the Slint `query`. The caller owns it: it sets this from
    /// its own state and takes the edits back through
    /// [`on_query`](Self::on_query). It is what filters the rows.
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }

    /// The row drawn on the accent surface, by index into `items`. The caller
    /// owns it and decides what it means: the row a key moved to, the row the
    /// pointer is over, or neither. The Slint `active`; `None` lights no row.
    pub fn active(mut self, active: Option<usize>) -> Self {
        self.active = active;
        self
    }

    /// The text the search row shows while nothing is typed, the Slint default
    /// `"Type a command or search..."`.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Whether the search row is drawn and the query filters the rows, `true` by
    /// default — the Slint `searchable`.
    pub fn searchable(mut self, searchable: bool) -> Self {
        self.searchable = searchable;
        self
    }

    /// Whether the panel draws its own border and rounding, `true` by default.
    /// Turn it off when the palette sits inside a frame of its own, such as a
    /// [`Dialog`](super::dialog::Dialog), which is the Slint `bordered` and
    /// GPUI's `.bordered(false)`.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// A heading band above the search field, with the number of matches beside
    /// it — the Slint `heading` and its `"{n} matches"`. Empty draws no band.
    pub fn heading(mut self, heading: impl Into<String>) -> Self {
        self.heading = heading.into();
        self
    }

    /// A footer band of keyboard hints — `↑↓ Navigate`, `Enter Select`,
    /// `Esc Close` — the Slint `hints`.
    pub fn hints(mut self, hints: bool) -> Self {
        self.hints = hints;
        self
    }

    /// Show only this many rows while the query is empty. `0`, the default,
    /// shows every row. The Slint `preview-count`.
    pub fn preview_count(mut self, preview_count: usize) -> Self {
        self.preview_count = preview_count;
        self
    }

    /// Draw each row as the custom shape — its label and its
    /// [`detail`](CommandItem::detail) — rather than the icon-and-label one, the
    /// Slint `custom-rows`.
    pub fn custom_rows(mut self, custom_rows: bool) -> Self {
        self.custom_rows = custom_rows;
        self
    }

    /// Cap the list's height and scroll it, the Slint `max-list-height`. `0`,
    /// the default, draws the whole list in the flow.
    pub fn max_list_height(mut self, max_list_height: f32) -> Self {
        self.max_list_height = max_list_height;
        self
    }

    /// The palette's width, `380px` by default — the Slint component's
    /// `preferred-width` and `max-width`.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// The name iced keeps the search field under. It has to be stable across
    /// frames and distinct per palette, so a page with several gives each its
    /// own — `Id::new(("command", slot))` is the gallery's.
    pub fn search_id(mut self, search_id: impl Into<Id>) -> Self {
        self.search_id = search_id.into();
        self
    }

    /// Report the row that was chosen, by index into `items`, on a click. The
    /// caller owns the items and asks itself what index 2 means; the Slint
    /// `chosen(index)` reports the same number. The palette is not closed by a
    /// choice — the caller answers with its own state.
    pub fn on_chosen(mut self, on_chosen: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_chosen = Some(Rc::new(on_chosen));
        self
    }

    /// Report the row the pointer entered, by index into `items`, the way
    /// [`Menu::on_highlight`](super::menu::Menu::on_highlight) does. The caller
    /// draws it by passing the same index to [`active`](Self::active).
    ///
    /// It is a **hover**, so it is transient: pass the matching
    /// [`on_leave`](Self::on_leave) to clear it. A caller that also owns the row
    /// a key moved to keeps the two apart, or the pointer's leave takes the
    /// keyboard's row with it.
    pub fn on_highlight(mut self, on_highlight: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_highlight = Some(Rc::new(on_highlight));
        self
    }

    /// Report that the pointer left a row, or left the list: `Some(index)` is
    /// the row it left, by index into `items`, and `None` is the list as a
    /// whole. It is the half that clears the hover
    /// [`on_highlight`](Self::on_highlight) sets. See
    /// [`Menu::on_leave`](super::menu::Menu::on_leave) for why the row travels
    /// with the report.
    pub fn on_leave(mut self, on_leave: impl Fn(Option<usize>) -> Message + 'a) -> Self {
        self.on_leave = Some(Rc::new(on_leave));
        self
    }

    /// Every edit of the free text, as the value the field now holds. The
    /// component filters the rows from the value the caller passes back to
    /// [`query`](Self::query). With no callback the field is drawn disabled,
    /// because iced reads a missing `on_input` as the disabled state.
    pub fn on_query(mut self, on_query: impl Fn(String) -> Message + 'a) -> Self {
        self.on_query = Some(Box::new(on_query));
        self
    }

    /// The whole palette, as an element.
    fn into_element(self) -> Element<'a, Message> {
        let Self {
            items,
            query,
            active,
            placeholder,
            searchable,
            bordered,
            heading,
            hints,
            preview_count,
            custom_rows,
            max_list_height,
            width,
            search_id,
            on_chosen,
            on_highlight,
            on_leave,
            on_query,
        } = self;

        let needle = query.to_lowercase();
        let matched_count = (0..items.len())
            .filter(|index| {
                !items[*index].row.is_separator() && shown(&items, *index, &needle, preview_count)
            })
            .count();

        let mut body = column![].width(Length::Fill).spacing(0.0);

        if !heading.is_empty() {
            body = body.push(heading_band(&heading, matched_count));
            body = body.push(rule());
        }
        if searchable {
            body = body.push(search_row(search_id, query, placeholder, on_query));
            body = body.push(rule());
        }

        let mut rows = column![]
            .width(Length::Fill)
            .spacing(LIST_SPACING)
            .padding(LIST_PADDING);
        for index in 0..items.len() {
            if needs_header(&items, index, &needle, preview_count) {
                rows = rows.push(group_heading(&items[index].group));
            }
            if !shown(&items, index, &needle, preview_count) {
                continue;
            }
            if items[index].row.is_separator() {
                rows = rows.push(rule());
                continue;
            }
            rows = rows.push(row_element(
                &items[index],
                index,
                active == Some(index),
                custom_rows,
                &on_chosen,
                &on_highlight,
                &on_leave,
            ));
        }

        let list: Element<'a, Message> = if max_list_height > 0.0 {
            let rows = list_leave(rows, &on_leave);
            Scrollable::new(rows)
                .width(Length::Fill)
                .height(Length::Fixed(max_list_height))
                .into()
        } else {
            list_leave(rows, &on_leave)
        };
        body = body.push(list);

        if !needle.is_empty() && matched_count == 0 {
            body = body.push(
                container(text("No results found.").size(theme::FONT_SIZE_SM).style(
                    |theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    },
                ))
                .width(Length::Fill)
                .padding(theme::SPACING_LG)
                .center_x(Length::Fill),
            );
        }

        if hints {
            body = body.push(rule());
            body = body.push(hints_band());
        }

        // The surface is `PanelStyle`'s, resolved from the tokens inside the
        // closure: the appearance in force is not known where the palette is
        // built, and a fill resolved here for the light theme is the white panel
        // the dark table already paid for. This file draws no panel of its own.
        let inner: Element<'a, Message> = if bordered {
            container(body)
                .width(width)
                .style(|theme: &Theme| PanelStyle::resolve(Tokens::of(theme)).container_style())
                .into()
        } else {
            container(body).width(width).into()
        };
        inner
    }
}

impl<'a, Message> From<Command<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'static,
{
    fn from(command: Command<'a, Message>) -> Self {
        command.into_element()
    }
}

/// The colours a row's ink is drawn in, before the theme in force is known. A
/// value resolved at build time would be the wrong colour in the other
/// appearance, so the row names the role and the style closure resolves it.
#[derive(Debug, Clone, Copy)]
enum Ink {
    Foreground,
    Muted,
    Success,
    Danger,
}

/// The colour a role takes in the appearance in force. A disabled row's ink is
/// muted whatever the role, which is how this crate draws a disabled control
/// without the subtree opacity Slint uses.
fn ink_color(tokens: Tokens, ink: Ink, disabled: bool) -> Color {
    if disabled {
        return tokens.muted_foreground;
    }
    match ink {
        Ink::Foreground => tokens.foreground,
        Ink::Muted => tokens.muted_foreground,
        Ink::Success => tokens.success,
        Ink::Danger => tokens.danger,
    }
}

/// A one-pixel rule across the panel: the heading's edge, the search row's, a
/// separator and the hints band's. The colour is the `border` token, which is an
/// edge in both appearances; a fixed grey would vanish on one of them.
fn rule<'a, Message: 'a>() -> Element<'a, Message> {
    container(Space::new().height(1.0).width(Length::Fill))
        .width(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).border.into()),
            ..container::Style::default()
        })
        .into()
}

/// The list's own pointer exit: a row's exit fires when the pointer leaves that
/// row, but the list's fires when it leaves the list, which is the report that
/// clears a hover sitting on a group heading or the list's inset. It draws
/// nothing — a `mouse_area` is a pointer target, not a surface.
fn list_leave<'a, Message: Clone + 'a>(
    rows: impl Into<Element<'a, Message>>,
    on_leave: &Option<Rc<dyn Fn(Option<usize>) -> Message + 'a>>,
) -> Element<'a, Message> {
    match on_leave {
        Some(on_leave) => {
            let on_leave = on_leave.clone();
            mouse_area(rows).on_exit(on_leave(None)).into()
        }
        None => rows.into(),
    }
}

/// The heading band: the title, and the number of rows the query leaves.
fn heading_band<'a, Message: 'a>(heading: &str, matched: usize) -> Element<'a, Message> {
    row![
        text(heading.to_string())
            .size(theme::FONT_SIZE_SM)
            .font(iced::Font {
                weight: iced::font::Weight::Semibold,
                ..iced::Font::DEFAULT
            })
            .width(Length::Fill)
            .wrapping(text::Wrapping::None)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
        text(format!("{matched} matches"))
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    ]
    .spacing(theme::SPACING_SM)
    .align_y(Alignment::Center)
    .padding([0.0, theme::SPACING_MD])
    .height(Length::Fixed(HEADING_HEIGHT))
    .width(Length::Fill)
    .into()
}

/// The search row: the field's frame dropped, because the panel owns the
/// surface, with the search glyph in its prefix slot. The Slint row is a bare
/// `TextInput` with a 14px glyph at its leading edge; [`Input`] is that field
/// plus the size ladder this crate shares.
fn search_row<'a, Message: Clone + 'a>(
    id: Id,
    query: String,
    placeholder: String,
    on_query: Option<Box<dyn Fn(String) -> Message + 'a>>,
) -> Element<'a, Message> {
    let glyph: Element<'a, Message> = svg(svg::Handle::from_memory(SEARCH))
        .width(SEARCH_ICON)
        .height(SEARCH_ICON)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into();

    let mut field = Input::new(id)
        .value(query)
        .placeholder(placeholder)
        .appearance(false)
        .prefix(glyph);
    if let Some(on_query) = on_query {
        field = field.on_input(on_query);
    }

    container(field)
        .height(Length::Fixed(SEARCH_HEIGHT))
        .width(Length::Fill)
        .center_y(SEARCH_HEIGHT)
        .into()
}

/// The hints band: three shortcuts and what they do, the Slint `hints` row.
///
/// The marks are the ones the wasm face can actually draw — the embedded Fira
/// Sans has `↑`, `↓` and `⏎` and **not** the command or option symbols — so a
/// hint that needs one spells it rather than painting a blank. See
/// `demos/menu.rs`.
fn hints_band<'a, Message: 'a>() -> Element<'a, Message> {
    fn hint<'a, Message: 'a>(label: &str) -> Element<'a, Message> {
        text(label.to_string())
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            })
            .into()
    }

    row![
        Kbd::<Message>::new("↑↓").size(ControlSize::XSmall),
        hint("Navigate"),
        Kbd::<Message>::new("Enter").size(ControlSize::XSmall),
        hint("Select"),
        Kbd::<Message>::new("Esc").size(ControlSize::XSmall),
        hint("Close"),
    ]
    .spacing(theme::SPACING_MD)
    .align_y(Alignment::Center)
    .padding([theme::SPACING_SM, theme::SPACING_MD])
    .width(Length::Fill)
    .into()
}

/// A group heading: the first visible row of a group draws it, muted and small,
/// the Slint `padding-left/right: 8px; padding-top: 10px; padding-bottom: 4px`.
fn group_heading<'a, Message: 'a>(group: &str) -> Element<'a, Message> {
    container(
        text(group.to_string())
            .size(theme::FONT_SIZE_XS)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..iced::Font::DEFAULT
            })
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    )
    .width(Length::Fill)
    .padding(Padding {
        top: 10.0,
        right: theme::SPACING_SM,
        bottom: 4.0,
        left: theme::SPACING_SM,
    })
    .into()
}

/// One row of the list, in the shape its item asks for, filled with the accent
/// surface while it is the active row.
///
/// A disabled row is given no pointer target at all: iced's `mouse_area` has no
/// enabled flag, so the reports are simply not attached, which is the same
/// reading the menu takes.
fn row_element<'a, Message: Clone + 'a>(
    item: &CommandItem,
    index: usize,
    active: bool,
    custom_rows: bool,
    on_chosen: &Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    on_highlight: &Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    on_leave: &Option<Rc<dyn Fn(Option<usize>) -> Message + 'a>>,
) -> Element<'a, Message> {
    let disabled = !item.row.is_actionable();
    let lit = active && !disabled;

    let (content, fixed): (Element<'a, Message>, bool) = if custom_rows {
        (custom_body(item, disabled), item.detail.is_empty())
    } else if !item.meta.is_empty() {
        (meta_body(item, disabled), false)
    } else {
        (normal_body(item, disabled), true)
    };

    let body = container(content)
        .width(Length::Fill)
        .height(if fixed {
            Length::Fixed(ROW_HEIGHT)
        } else {
            Length::Shrink
        })
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            container::Style {
                background: Some(
                    if lit {
                        tokens.accent
                    } else {
                        Color::TRANSPARENT
                    }
                    .into(),
                ),
                border: Border {
                    radius: theme::RADIUS_SM.into(),
                    ..Border::default()
                },
                ..container::Style::default()
            }
        });

    if disabled || (on_chosen.is_none() && on_highlight.is_none() && on_leave.is_none()) {
        return body.into();
    }

    let mut area = mouse_area(body);
    if let Some(on_highlight) = on_highlight {
        let on_highlight = on_highlight.clone();
        area = area.on_enter(on_highlight(index));
    }
    if let Some(on_leave) = on_leave {
        let on_leave = on_leave.clone();
        area = area.on_exit(on_leave(Some(index)));
    }
    if let Some(on_chosen) = on_chosen {
        let on_chosen = on_chosen.clone();
        area = area.on_press(on_chosen(index));
    }
    area.into()
}

/// The icon-and-label row: a 16px leading slot, the label, an optional check
/// mark and an optional [`Kbd`] shortcut, the Slint `item.meta == ""` branch.
fn normal_body<'a, Message: Clone + 'a>(
    item: &CommandItem,
    disabled: bool,
) -> Element<'a, Message> {
    let mut line = row![]
        .spacing(theme::SPACING_SM)
        .align_y(Alignment::Center)
        .height(Length::Fill);

    line = line.push(match item.row.icon_bytes() {
        Some(bytes) => glyph::<Message>(bytes, Ink::Muted, disabled),
        None => Space::new().width(ICON_SLOT).height(Length::Shrink).into(),
    });

    line = line.push(
        text(item.row.label().to_string())
            .size(theme::FONT_SIZE_SM)
            .width(Length::Fill)
            // iced 0.14 draws no ellipsis, so a long label clips at the row's
            // edge where Slint elides. See the module comment.
            .wrapping(text::Wrapping::None)
            .style(move |theme: &Theme| text::Style {
                color: Some(ink_color(Tokens::of(theme), Ink::Foreground, disabled)),
            }),
    );

    if item.row.is_checked() {
        line = line.push(glyph::<Message>(CHECK, Ink::Foreground, disabled));
    }
    if !item.row.shortcut_text().is_empty() {
        line = line.push(
            Kbd::<Message>::new(item.row.shortcut_text().to_string()).size(ControlSize::Small),
        );
    }

    container(line)
        .padding([0.0, theme::SPACING_SM])
        .width(Length::Fill)
        .into()
}

/// The rich row: the keywords and the label on the left, the meta and the change
/// on the right, the Slint `item.meta != ""` branch.
fn meta_body<'a, Message: Clone + 'a>(item: &CommandItem, disabled: bool) -> Element<'a, Message> {
    let left = column![
        text(item.keywords.clone())
            .size(theme::FONT_SIZE_SM)
            .style(move |theme: &Theme| text::Style {
                color: Some(ink_color(Tokens::of(theme), Ink::Foreground, disabled)),
            }),
        text(item.row.label().to_string())
            .size(theme::FONT_SIZE_XS)
            .style(move |theme: &Theme| text::Style {
                color: Some(ink_color(Tokens::of(theme), Ink::Muted, disabled)),
            }),
    ]
    .spacing(theme::SPACING_XXS)
    .width(Length::Fill);

    let change_ink = if item.up { Ink::Success } else { Ink::Danger };
    let right = column![
        text(item.meta.clone())
            .size(theme::FONT_SIZE_SM)
            .style(move |theme: &Theme| text::Style {
                color: Some(ink_color(Tokens::of(theme), Ink::Foreground, disabled)),
            }),
        text(item.change.clone())
            .size(theme::FONT_SIZE_XS)
            .style(move |theme: &Theme| text::Style {
                color: Some(ink_color(Tokens::of(theme), change_ink, disabled)),
            }),
    ]
    .spacing(theme::SPACING_XXS)
    .align_x(Alignment::End);

    row![left, right]
        .spacing(theme::SPACING_MD)
        .align_y(Alignment::Start)
        .padding(theme::SPACING_SM)
        .width(Length::Fill)
        .into()
}

/// The custom row: the label, and the detail under it when there is one. The
/// detail also gives the row its taller inset, which is what makes the two rows
/// in the Slint example different heights.
fn custom_body<'a, Message: Clone + 'a>(
    item: &CommandItem,
    disabled: bool,
) -> Element<'a, Message> {
    let vertical = if item.detail.is_empty() { 6.0 } else { 12.0 };

    let mut body = column![
        text(item.row.label().to_string())
            .size(theme::FONT_SIZE_SM)
            .style(move |theme: &Theme| text::Style {
                color: Some(ink_color(Tokens::of(theme), Ink::Foreground, disabled)),
            })
    ]
    .spacing(theme::SPACING_XXS)
    .width(Length::Fill);

    if !item.detail.is_empty() {
        body = body.push(
            text(item.detail.clone())
                .size(theme::FONT_SIZE_XS)
                .wrapping(text::Wrapping::Word)
                .style(move |theme: &Theme| text::Style {
                    color: Some(ink_color(Tokens::of(theme), Ink::Muted, disabled)),
                }),
        );
    }

    container(body)
        .padding(Padding {
            top: vertical,
            right: theme::SPACING_SM,
            bottom: vertical,
            left: theme::SPACING_SM,
        })
        .width(Length::Fill)
        .into()
}

/// The 14px SVG a row's leading or trailing slot holds. The colour is resolved
/// inside the style closure, so the icon follows the appearance in force.
fn glyph<'a, Message: 'a>(bytes: &'static [u8], ink: Ink, disabled: bool) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(ICON_SLOT)
        .height(ICON_SLOT)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(ink_color(Tokens::of(theme), ink, disabled)),
        })
        .into()
}

/// The search glyph, drawn from `crates/slint-component/ui/icons/search.svg`
/// (Lucide, ISC), the same file the Slint component names.
const SEARCH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21 21-4.34-4.34"/><circle cx="11" cy="11" r="8"/></svg>"#;

/// The check mark a `checked` row draws, from
/// `crates/slint-component/ui/icons/check.svg` (Lucide, ISC), the same file the
/// menu's check and the Slint command's both name.
const CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"/></svg>"#;
