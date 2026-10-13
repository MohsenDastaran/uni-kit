//! A data table: the [`Table`](super::table) surface with the interaction the
//! table deliberately lacks. Mirrors GPUI Component's `DataTable` and the Slint
//! version beside it — typed columns and rows, a fixed header over a scrolling
//! body, a header click that cycles the sort, row selection, stripes, borders
//! and the size ladder.
//!
//! **The application owns the state, the table only draws it.** It is given the
//! sort ([`DataTable::sort`]) and the selected row ([`DataTable::selected`]), and
//! reports a header press through [`DataTable::on_sort`] and a row press through
//! [`DataTable::on_select`]. It never reorders the rows and never moves the
//! selection, which is the same controlled shape as [`List`](super::list) and
//! the Slint component: the sort of the rows on screen is the caller's own.
//! Without an `on_sort` a sortable header draws its indicator and reports
//! nothing; without an `on_select` no row can be selected, which is iced's
//! "cannot be pressed".
//!
//! A header press reports the direction the cycle lands on, not the one it left:
//! a new column starts at descending, then ascending, then default, which is the
//! Slint `toggle-sort` and GPUI's `ColumnSort` ladder.
//!
//! **The body is a [`Scrollable`](super::scrollable::Scrollable).** A
//! [`DataTable::height`] bounds it so the rows move under the fixed header;
//! without one the body sizes to its rows and draws no bar.
//!
//! # What iced cannot draw, named rather than faked
//!
//! - **There is no virtual scrolling.** Slint's body instantiates only the
//!   visible rows, so thousands stay cheap; `scrollable` lays out and draws every
//!   row it is given, on every pass, exactly as it says of itself. The demo's
//!   large-dataset section shows a bounded body scrolling and names the gap.
//! - **A column cannot be resized or moved.** Slint drags a handle on the header
//!   and writes the new width back; iced has no drag delta to hand a controlled
//!   widget, and header reordering needs a drop target the component does not
//!   own. The width a [`Column`] is given is the width it is drawn at.
//! - **The component has no keyboard.** Slint's `FocusScope` moves the selection
//!   with the arrows, Home/End, PageUp/PageDown and Escape. iced 0.14 reports no
//!   widget focus to the application, so the host binds those keys; the
//!   component's selection is written by the caller either way.
//! - **A cell does not elide.** Slint asks for `overflow: elide`; iced 0.14 has
//!   neither elision nor a wrapping text widget, so a long value is clipped by
//!   its cell. The widths a caller gives are the ones that keep it readable.
//! - **The header does not follow a horizontal scroll.** Slint pins the header
//!   to the body's viewport; the body here is a vertical scroll area, and a
//!   caller that overflows it horizontally wraps the whole table instead.
//!
//! Everything is drawn from [`Tokens`], read inside a style closure while the
//! table renders, so a theme switch repaints it. Nothing here writes a colour of
//! its own, and there is no `Styled` trait to layer one on.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one part gets
// a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::button;
use iced::widget::{column, container, row, svg, text, Space};
use iced::{Alignment, Border, Color, Element, Font, Length, Theme};

use super::scrollable::{Axis, Scrollable};
use super::theme::{self, ControlSize, Status, Tokens};

/// The header band's tint, `Theme.muted.with-alpha(0.5)` in the Slint component.
const BAND_TINT: f32 = 0.5;
/// The header cell under the pointer, `Theme.muted.with-alpha(0.8)`.
const HEAD_HOVER_TINT: f32 = 0.8;
/// A hovered row, `Theme.muted.with-alpha(0.8)`.
const ROW_HOVER_TINT: f32 = 0.8;
/// A striped row, `Theme.muted.with-alpha(0.5)`.
const STRIPE_TINT: f32 = 0.5;
/// The selected row's tint in the light theme, `Theme.selection.with-alpha(0.15)`.
const SELECT_TINT_LIGHT: f32 = 0.15;
/// The selected row's tint in the dark theme, `Theme.selection.with-alpha(0.2)`.
const SELECT_TINT_DARK: f32 = 0.2;
/// A body row's separator, `Theme.border.with-alpha(0.7)`.
const RULE_TINT: f32 = 0.7;

/// `arrow-up`, from `crates/slint-component/ui/icons/` (Lucide, ISC).
const ARROW_UP: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m5 12 7-7 7 7"/><path d="M12 19V5"/></svg>"#;

/// `arrow-down`, from the same set.
const ARROW_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14"/><path d="m19 12-7 7-7-7"/></svg>"#;

/// `chevrons-up-down`, from the same set: a sortable column that is not sorted.
const CHEVRONS_UP_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/></svg>"#;

/// Where a cell's content sits across its width. The table family's own
/// [`Align`](super::table::Align), shared here rather than derived a second time:
/// a data-table column and a table cell line up by the same value.
pub use super::table::Align;

/// Which way a column is sorted, the Slint `ColumnSort`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColumnSort {
    /// Natural order: the rows are drawn as they were given. The indicator is
    /// the two-headed chevron.
    #[default]
    Default,
    Ascending,
    Descending,
}

impl ColumnSort {
    /// The step a header press makes, the Slint `toggle-sort` ladder: default to
    /// descending, descending to ascending, ascending back to default.
    pub fn next(self) -> Self {
        match self {
            Self::Default => Self::Descending,
            Self::Descending => Self::Ascending,
            Self::Ascending => Self::Default,
        }
    }

    /// Whether the rows run the other way from the natural order.
    pub fn is_sorted(self) -> bool {
        !matches!(self, Self::Default)
    }
}

/// The direction a press on `column` reports, given where the sort stands. A
/// press on a column that is not the sorted one starts at descending, which is
/// the Slint `toggle-sort` first step.
fn next_sort(current: Option<(usize, ColumnSort)>, column: usize) -> ColumnSort {
    match current {
        Some((index, sort)) if index == column => sort.next(),
        _ => ColumnSort::Descending,
    }
}

/// A row's height at a size: 26 / 30 / 32 / 40, the Slint `row-height` ladder.
fn row_height(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 26.0,
        ControlSize::Small => 30.0,
        ControlSize::Medium => 32.0,
        ControlSize::Large => 40.0,
    }
}

/// A cell's horizontal inset at a size: 4 / 6 / 8 / 12, the Slint
/// `cell-padding` ladder. It is the data table's own, so it lives here rather
/// than on [`ControlSize`].
fn pad_x(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 4.0,
        ControlSize::Small => 6.0,
        ControlSize::Medium => 8.0,
        ControlSize::Large => 12.0,
    }
}

/// A column: the key the caller sorts by, the title the header draws, an optional
/// fixed width, the alignment of every cell under it, and whether its header is
/// interactive.
pub struct Column {
    key: String,
    title: String,
    width: Option<f32>,
    align: Align,
    sortable: bool,
}

impl Column {
    /// A column of `width` fill and start alignment, not sortable.
    pub fn new(key: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            title: title.into(),
            width: None,
            align: Align::default(),
            sortable: false,
        }
    }

    /// A fixed width. Without one the column takes an equal share of what the
    /// fixed columns leave, which is the Slint `width: 0` and `horizontal-stretch`.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(0.0));
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Draw the sort indicator and make the header a button, the Slint
    /// `sortable: true`.
    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    /// The key the caller sorts by. The component itself never reads it: sorting
    /// is the application's, exactly as the Slint file leaves it.
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    /// The column's share of the row: a fixed width, or one fill portion.
    fn length(&self) -> Length {
        match self.width {
            Some(width) => Length::Fixed(width),
            None => Length::FillPortion(1),
        }
    }
}

impl From<(&str, &str)> for Column {
    fn from((key, title): (&str, &str)) -> Self {
        Self::new(key, title)
    }
}

/// A cell: the text, and the status that colours it — `Theme.status_color` in the
/// Slint component, which is how a gain and a loss read apart.
pub struct Cell {
    text: String,
    status: Status,
}

impl Cell {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            status: Status::default(),
        }
    }

    /// Colour the text by a status rather than the row foreground.
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl From<&str> for Cell {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl From<String> for Cell {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

/// A row: an id the caller keys it by, and one cell per column.
pub struct Row {
    id: String,
    cells: Vec<Cell>,
}

impl Row {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            cells: Vec::new(),
        }
    }

    /// Appends one cell. A `&str` is a [`Cell`] with the default status.
    pub fn cell(mut self, cell: impl Into<Cell>) -> Self {
        self.cells.push(cell.into());
        self
    }

    pub fn cells(mut self, cells: impl IntoIterator<Item = Cell>) -> Self {
        self.cells.extend(cells);
        self
    }

    /// The id the caller gave the row. The component reports a display index;
    /// this is how a caller maps it back to its own row.
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// A one-pixel rule across the surface, drawn from the border token at 0.7.
fn rule<'a, Message: 'a>() -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(|theme: &Theme| container::Style {
            background: Some(theme::alpha(Tokens::of(theme).border, RULE_TINT).into()),
            ..container::Style::default()
        })
        .into()
}

/// The sort indicator, at 12px the way the Slint header draws it. `None` for a
/// column that is not sortable.
fn indicator<'a, Message: 'a>(
    sortable: bool,
    current: Option<(usize, ColumnSort)>,
    index: usize,
) -> Option<Element<'a, Message>> {
    if !sortable {
        return None;
    }

    let sort = match current {
        Some((column, sort)) if column == index => sort,
        _ => ColumnSort::Default,
    };
    // A column that is sorted carries the page's ink; an unsorted one is the
    // muted mark at 0.7, which is the Slint header's own ladder.
    let active = sort.is_sorted() && current.map(|(column, _)| column) == Some(index);
    let handle = match sort {
        ColumnSort::Ascending => ARROW_UP,
        ColumnSort::Descending => ARROW_DOWN,
        ColumnSort::Default => CHEVRONS_UP_DOWN,
    };

    Some(
        svg(svg::Handle::from_memory(handle))
            .width(Length::Fixed(12.0))
            .height(Length::Fixed(12.0))
            .style(move |theme: &Theme, _status| svg::Style {
                color: Some(if active {
                    Tokens::of(theme).foreground
                } else {
                    theme::alpha(Tokens::of(theme).muted_foreground, 0.7)
                }),
            })
            .into(),
    )
}

/// One header cell: the title, the indicator, and — when the column is sortable
/// and the caller gave an `on_sort` — the whole cell as the button that asks for
/// the next direction in the cycle.
fn head_cell<'a, Message: Clone + 'a>(
    column: &Column,
    index: usize,
    sort: Option<(usize, ColumnSort)>,
    size: ControlSize,
    radius: f32,
    message: Option<Message>,
) -> Element<'a, Message> {
    let mut content = row![]
        .spacing(4.0)
        .align_y(Alignment::Center)
        .push(
            text(column.title.clone())
                .size(theme::FONT_SIZE_SM)
                .font(Font {
                    weight: iced::font::Weight::Medium,
                    ..Font::DEFAULT
                })
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        );

    if let Some(indicator) = indicator(column.sortable, sort, index) {
        content = content.push(indicator);
    }

    let body = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::from(column.align))
        .align_y(Alignment::Center)
        .padding([0.0, pad_x(size)]);

    if message.is_some() {
        let mut control = button(body)
            .width(column.length())
            .height(Length::Fixed(row_height(size)))
            .padding(0)
            .style(move |theme: &Theme, status: button::Status| {
                // Resolved here, inside the closure, and never at build time: a
                // surface read from the light tokens before the cell is drawn
                // would paint a white header under the dark theme.
                let tokens = Tokens::of(theme);
                let background = match status {
                    button::Status::Pressed => Some(tokens.muted),
                    button::Status::Hovered => Some(theme::alpha(tokens.muted, HEAD_HOVER_TINT)),
                    _ => None,
                };
                button::Style {
                    background: background.map(Into::into),
                    text_color: tokens.foreground,
                    border: Border::default(),
                    shadow: iced::Shadow::default(),
                    snap: false,
                }
            });
        if let Some(message) = message {
            control = control.on_press(message);
        }
        return control.into();
    }

    container(body)
        .width(column.length())
        .height(Length::Fixed(row_height(size)))
        .style(move |_theme: &Theme| container::Style {
            border: Border {
                radius: iced::border::top(radius),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .into()
}

/// The header band: the row of header cells and its bottom rule, on the muted
/// surface.
fn header<'a, Message: Clone + 'a>(
    columns: &[Column],
    sort: Option<(usize, ColumnSort)>,
    size: ControlSize,
    radius: f32,
    on_sort: Option<&dyn Fn(usize, ColumnSort) -> Message>,
) -> Element<'a, Message> {
    let mut cells = row![].spacing(0.0).width(Length::Fill);
    for (index, column) in columns.iter().enumerate() {
        // A click reports where the cycle lands, not where it was; without a
        // handler the cell is not a button and reports nothing.
        let message = if column.sortable {
            on_sort.map(|on_sort| on_sort(index, next_sort(sort, index)))
        } else {
            None
        };
        cells = cells.push(head_cell(column, index, sort, size, radius, message));
    }

    container(column![cells, rule()].spacing(0.0).width(Length::Fill))
        .width(Length::Fill)
        .style(move |theme: &Theme| container::Style {
            background: Some(theme::alpha(Tokens::of(theme).muted, BAND_TINT).into()),
            border: Border {
                radius: iced::border::top(radius),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .into()
}

/// What every body row of one table draws the same way: the caller's selection,
/// the stripe and size switches, the inner corner, and whether a row can be
/// pressed at all.
#[derive(Debug, Clone, Copy)]
struct BodyStyle {
    selected: Option<usize>,
    stripe: bool,
    size: ControlSize,
    radius: f32,
    /// Whether the caller registered an `on_select`, which is the Slint
    /// `row-selectable` gate: without it no row lights and none reports.
    selectable: bool,
}

/// One body row: the cells, the separator under them, and the surface the
/// selection, the hover and the stripe are drawn on. `selected` is the caller's
/// own answer; `message` is the press the caller registered.
fn body_row<'a, Message: Clone + 'a>(
    columns: &[Column],
    row_value: &Row,
    index: usize,
    last: bool,
    style: BodyStyle,
    message: Option<Message>,
) -> Element<'a, Message> {
    let BodyStyle {
        selected,
        stripe,
        size,
        radius,
        selectable,
    } = style;
    let selected = selected == Some(index);

    let mut cells = row![].spacing(0.0).width(Length::Fill);
    for (column_index, column) in columns.iter().enumerate() {
        let cell = row_value.cells.get(column_index);
        // Owned, because the row is borrowed only for this pass: iced's `text`
        // would otherwise tie the element's lifetime to the caller's rows.
        let label = cell.map(Cell::text).unwrap_or_default().to_owned();
        let status = cell.map(|cell| cell.status).unwrap_or_default();
        cells = cells.push(
            container(
                text(label)
                    .size(theme::FONT_SIZE_SM)
                    .style(move |theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).status_color(status)),
                    }),
            )
            .width(column.length())
            .height(Length::Fixed(row_height(size) - 1.0))
            .align_x(Alignment::from(column.align))
            .align_y(Alignment::Center)
            .padding([0.0, pad_x(size)]),
        );
    }

    let stripes = stripe && index % 2 == 1;
    let stack = column![
        cells.height(Length::Fixed(row_height(size) - 1.0)),
        rule()
    ]
    .spacing(0.0)
    .width(Length::Fill);

    let surface = move |theme: &Theme, status: button::Status| {
        // Resolved here, inside the closure, and never at build time.
        let tokens = Tokens::of(theme);
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
        let background = if selected {
            theme::alpha(
                tokens.selection,
                if tokens.dark {
                    SELECT_TINT_DARK
                } else {
                    SELECT_TINT_LIGHT
                },
            )
        } else if hovered && selectable {
            theme::alpha(tokens.muted, ROW_HOVER_TINT)
        } else if stripes {
            theme::alpha(tokens.muted, STRIPE_TINT)
        } else {
            Color::TRANSPARENT
        };
        let corners = if last {
            iced::border::bottom(radius)
        } else {
            iced::border::Radius::default()
        };
        button::Style {
            background: (background.a > 0.0).then_some(background.into()),
            text_color: tokens.foreground,
            border: Border {
                radius: corners,
                ..Border::default()
            },
            shadow: iced::Shadow::default(),
            snap: false,
        }
    };

    let mut control = button(stack)
        .width(Length::Fill)
        .height(Length::Fixed(row_height(size)))
        .padding(0)
        .style(surface);
    if let Some(message) = message {
        control = control.on_press(message);
    }
    control.into()
}

/// The body: the rows in a [`Scrollable`], so a bounded height scrolls them under
/// the fixed header. With no rows it is the empty text on the surface.
fn body<'a, Message: Clone + 'a>(
    columns: &[Column],
    rows: &[Row],
    style: BodyStyle,
    height: Length,
    empty_text: &str,
    on_select: Option<&dyn Fn(usize) -> Message>,
) -> Element<'a, Message> {
    let last = rows.len().saturating_sub(1);
    let mut stack = column![].spacing(0.0).width(Length::Fill);

    if rows.is_empty() {
        stack = stack.push(
            container(
                text(empty_text.to_string())
                    .size(theme::FONT_SIZE_SM)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
            )
            .width(Length::Fill)
            .padding([8.0, pad_x(style.size)]),
        );
    } else {
        for (index, row_value) in rows.iter().enumerate() {
            let message = on_select.map(|on_select| on_select(index));
            stack = stack.push(body_row(columns, row_value, index, index == last, style, message));
        }
    }

    Scrollable::new(stack)
        .axis(Axis::Vertical)
        .height(height)
        .into()
}

/// A data table, built with the methods below.
///
/// The application owns the sort and the selection; see the module comment.
pub struct DataTable<'a, Message> {
    columns: Vec<Column>,
    rows: Vec<Row>,
    sort: Option<(usize, ColumnSort)>,
    selected: Option<usize>,
    stripe: bool,
    bordered: bool,
    size: ControlSize,
    empty_text: String,
    height: Length,
    width: Length,
    on_sort: Option<Box<dyn Fn(usize, ColumnSort) -> Message + 'a>>,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message> DataTable<'a, Message> {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            sort: None,
            selected: None,
            stripe: false,
            bordered: true,
            size: ControlSize::default(),
            empty_text: "No data".to_string(),
            height: Length::Shrink,
            width: Length::Fill,
            on_sort: None,
            on_select: None,
        }
    }

    pub fn column(mut self, column: impl Into<Column>) -> Self {
        self.columns.push(column.into());
        self
    }

    pub fn columns(mut self, columns: impl IntoIterator<Item = Column>) -> Self {
        self.columns.extend(columns);
        self
    }

    pub fn row(mut self, row: Row) -> Self {
        self.rows.push(row);
        self
    }

    pub fn rows(mut self, rows: impl IntoIterator<Item = Row>) -> Self {
        self.rows.extend(rows);
        self
    }

    /// Which column is sorted and which way, so the header draws the arrow. The
    /// component only draws it — the rows it is given are the rows it shows.
    pub fn sort(mut self, column: usize, direction: ColumnSort) -> Self {
        self.sort = Some((column, direction));
        self
    }

    /// No column is sorted: every sortable header draws the two-headed chevron.
    pub fn unsorted(mut self) -> Self {
        self.sort = None;
        self
    }

    /// The row drawn as the selection, a display index. `None`, the default, is
    /// the Slint `selected-row: -1`.
    pub fn selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    /// Tint the odd rows, the Slint `stripe`.
    pub fn stripe(mut self, stripe: bool) -> Self {
        self.stripe = stripe;
        self
    }

    /// Draws the outer border and the rounded corners. `false` keeps the surface
    /// and its bands and removes the edge and the corners, the Slint `bordered`.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The text a body with no rows draws, the Slint `empty-text`.
    pub fn empty_text(mut self, empty_text: impl Into<String>) -> Self {
        self.empty_text = empty_text.into();
        self
    }

    /// Bounds the body, so it scrolls its rows and draws a bar. Without it the
    /// body sizes to its rows, which is the Slint component with no height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// A sortable header was pressed. The component reports the column and the
    /// direction its cycle lands on, and leaves the rows to the caller.
    pub fn on_sort(mut self, on_sort: impl Fn(usize, ColumnSort) -> Message + 'a) -> Self {
        self.on_sort = Some(Box::new(on_sort));
        self
    }

    /// A row was pressed. The component reports the display index and leaves the
    /// selection to the caller, so the row shown as selected is the caller's own.
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }
}

impl<'a, Message> Default for DataTable<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<DataTable<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: DataTable<'a, Message>) -> Self {
        let DataTable {
            columns,
            rows,
            sort,
            selected,
            stripe,
            bordered,
            size,
            empty_text,
            height,
            width,
            on_sort,
            on_select,
        } = control;

        // The border and its radius are the one switch: without the edge the
        // surface is square and the bands have no inner corner to round.
        let border_width = if bordered { 1.0 } else { 0.0 };
        let radius = if bordered { theme::RADIUS_MD } else { 0.0 };
        let inner = (radius - border_width).max(0.0);

        let head = header(&columns, sort, size, inner, on_sort.as_deref());
        let body = body(
            &columns,
            &rows,
            BodyStyle {
                selected,
                stripe,
                size,
                radius: inner,
                selectable: on_select.is_some(),
            },
            height,
            &empty_text,
            on_select.as_deref(),
        );

        container(column![head, body].spacing(0.0).width(Length::Fill))
            .width(width)
            .padding(border_width)
            .clip(true)
            .style(move |theme: &Theme| {
                // Resolved here, inside the closure, and never at build time: a
                // surface read from the light tokens before the table is drawn
                // would paint a white table under the dark theme.
                let tokens = Tokens::of(theme);
                container::Style {
                    background: Some(tokens.background.into()),
                    border: Border {
                        color: if bordered {
                            tokens.border
                        } else {
                            Color::TRANSPARENT
                        },
                        width: border_width,
                        radius: radius.into(),
                    },
                    ..container::Style::default()
                }
            })
            .into()
    }
}
