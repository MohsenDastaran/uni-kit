//! `data-table` — a typed, interactive table: sortable headers, row selection,
//! status-coloured cells, stripes and the size ladder, built from the Slint
//! example's invoices, quotes, trades and people.
//!
//! Run it with `cargo run --example data_table`, or in the browser as
//! `?story=data-table` in the gallery the website embeds.
//!
//! The application owns the sort and the selection, exactly as the Slint file
//! leaves them: a header press reports the direction its cycle lands on, a row
//! press reports the row, and everything on screen is the app's own state. There
//! is no `TableSort` global — the demo's own `rows_for` is that order function,
//! so the rows shown are the rows the caller computed.
//!
//! Each section carries a heading the component page gives it — the title, and
//! the code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! The headings are the ones the Slint example's `GroupBox`es carry, and the page
//! has each one: Invoices, Column Configuration, Virtual Scrolling for Large
//! Datasets, Sorting Implementation, Cell Rendering, Selection Modes, Reading and
//! Writing the Selection, Column Resizing and Moving, Table Styling, Financial
//! Data Table, User Management Table.
//!
//! Three things iced cannot draw are named in their section rather than faked:
//! the large-dataset table scrolls every row it is given instead of virtualising,
//! the resizing section sets widths instead of dragging a header handle, and the
//! component has no keyboard because iced 0.14 reports no widget focus to the
//! application.

use iced::widget::{column, container, mouse_area, row, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::Button;
use crate::components::data_table::{Align, Cell, Column, ColumnSort, DataTable, Row};
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, ControlSize, Status, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// One row's cells: the text and the status that colours it.
type Cells = &'static [(&'static str, Status)];

/// The table slots, one per section that holds a table. Each one owns its sort
/// and its selection, so a click in one table never moves another.
mod section {
    pub const INVOICES: usize = 0;
    pub const CONFIGURATION: usize = 1;
    pub const VIRTUAL: usize = 2;
    pub const SORTING: usize = 3;
    pub const CELLS: usize = 4;
    pub const SELECTION: usize = 5;
    pub const SELECTION_API: usize = 6;
    pub const RESIZING: usize = 7;
    pub const STYLING: usize = 8;
    pub const FINANCIAL: usize = 9;
    pub const USERS: usize = 10;
    pub const COUNT: usize = 11;
}

// ---------------------------------------------------------------- the data
//
// The Slint example's rows, kept as text so a section reads like the table it
// draws. The first cell is the row's id, which is what `Row::new` takes.

const INVOICES: &[Cells] = &[
    &[
        ("INV001", Status::Default),
        ("Paid", Status::Success),
        ("$250", Status::Default),
    ],
    &[
        ("INV002", Status::Default),
        ("Pending", Status::Warning),
        ("$150", Status::Default),
    ],
    &[
        ("INV003", Status::Default),
        ("Unpaid", Status::Danger),
        ("$350", Status::Default),
    ],
    &[
        ("INV004", Status::Default),
        ("Paid", Status::Success),
        ("$90", Status::Default),
    ],
    &[
        ("INV005", Status::Default),
        ("Pending", Status::Warning),
        ("$1,200", Status::Default),
    ],
];

const VIRTUAL: &[Cells] = &[
    &[
        ("INV001", Status::Default),
        ("Paid", Status::Success),
        ("$250", Status::Default),
    ],
    &[
        ("INV002", Status::Default),
        ("Pending", Status::Warning),
        ("$150", Status::Default),
    ],
    &[
        ("INV003", Status::Default),
        ("Unpaid", Status::Danger),
        ("$350", Status::Default),
    ],
    &[
        ("INV004", Status::Default),
        ("Paid", Status::Success),
        ("$90", Status::Default),
    ],
    &[
        ("INV005", Status::Default),
        ("Pending", Status::Warning),
        ("$1,200", Status::Default),
    ],
    &[
        ("INV006", Status::Default),
        ("Paid", Status::Success),
        ("$480", Status::Default),
    ],
    &[
        ("INV007", Status::Default),
        ("Unpaid", Status::Danger),
        ("$75", Status::Default),
    ],
    &[
        ("INV008", Status::Default),
        ("Paid", Status::Success),
        ("$2,040", Status::Default),
    ],
    &[
        ("INV009", Status::Default),
        ("Pending", Status::Warning),
        ("$610", Status::Default),
    ],
    &[
        ("INV010", Status::Default),
        ("Paid", Status::Success),
        ("$128", Status::Default),
    ],
    &[
        ("INV011", Status::Default),
        ("Unpaid", Status::Danger),
        ("$990", Status::Default),
    ],
    &[
        ("INV012", Status::Default),
        ("Pending", Status::Warning),
        ("$340", Status::Default),
    ],
    &[
        ("INV013", Status::Default),
        ("Paid", Status::Success),
        ("$1,760", Status::Default),
    ],
    &[
        ("INV014", Status::Default),
        ("Unpaid", Status::Danger),
        ("$55", Status::Default),
    ],
    &[
        ("INV015", Status::Default),
        ("Paid", Status::Success),
        ("$870", Status::Default),
    ],
    &[
        ("INV016", Status::Default),
        ("Pending", Status::Warning),
        ("$430", Status::Default),
    ],
];

const PEOPLE: &[Cells] = &[
    &[
        ("1", Status::Default),
        ("Ada Lovelace", Status::Default),
        ("36", Status::Default),
        ("ada@uni.dev", Status::Default),
    ],
    &[
        ("2", Status::Default),
        ("Alan Turing", Status::Default),
        ("41", Status::Default),
        ("alan@uni.dev", Status::Default),
    ],
    &[
        ("3", Status::Default),
        ("Grace Hopper", Status::Default),
        ("85", Status::Default),
        ("grace@uni.dev", Status::Default),
    ],
];

const NAMES: &[Cells] = &[
    &[
        ("Cara", Status::Default),
        ("41", Status::Default),
        ("cara@uni.dev", Status::Default),
    ],
    &[
        ("Bob", Status::Default),
        ("22", Status::Default),
        ("bob@uni.dev", Status::Default),
    ],
    &[
        ("Ada", Status::Default),
        ("36", Status::Default),
        ("ada@uni.dev", Status::Default),
    ],
    &[
        ("Dan", Status::Default),
        ("27", Status::Default),
        ("dan@uni.dev", Status::Default),
    ],
];

const TRADES: &[Cells] = &[
    &[
        ("AAPL", Status::Default),
        ("Long", Status::Info),
        ("+2.40%", Status::Success),
    ],
    &[
        ("NVDA", Status::Default),
        ("Short", Status::Warning),
        ("-1.10%", Status::Danger),
    ],
    &[
        ("TSLA", Status::Default),
        ("Long", Status::Info),
        ("+3.18%", Status::Success),
    ],
    &[
        ("AMZN", Status::Default),
        ("Short", Status::Warning),
        ("-0.42%", Status::Danger),
    ],
];

const QUOTES: &[Cells] = &[
    &[
        ("AAPL", Status::Default),
        ("Apple Inc.", Status::Default),
        ("228.52", Status::Default),
        ("+1.24", Status::Success),
        ("52,430,100", Status::Default),
        ("3,480,000", Status::Default),
    ],
    &[
        ("NVDA", Status::Default),
        ("NVIDIA Corporation", Status::Default),
        ("134.81", Status::Default),
        ("-0.62", Status::Danger),
        ("180,220,400", Status::Default),
        ("3,310,000", Status::Default),
    ],
    &[
        ("TSLA", Status::Default),
        ("Tesla, Inc.", Status::Default),
        ("251.44", Status::Default),
        ("+3.18", Status::Success),
        ("98,110,250", Status::Default),
        ("802,000", Status::Default),
    ],
    &[
        ("MSFT", Status::Default),
        ("Microsoft Corporation", Status::Default),
        ("428.02", Status::Default),
        ("+0.41", Status::Success),
        ("22,540,800", Status::Default),
        ("3,180,000", Status::Default),
    ],
    &[
        ("AMZN", Status::Default),
        ("Amazon.com, Inc.", Status::Default),
        ("186.33", Status::Default),
        ("-1.07", Status::Danger),
        ("41,220,600", Status::Default),
        ("1,950,000", Status::Default),
    ],
    &[
        ("700.HK", Status::Default),
        ("Tencent Holdings", Status::Default),
        ("412.60", Status::Default),
        ("+0.87", Status::Success),
        ("18,440,200", Status::Default),
        ("3,820,000", Status::Default),
    ],
];

const USERS: &[Cells] = &[
    &[
        ("Ada Lovelace", Status::Default),
        ("ada@uni.dev", Status::Default),
        ("Admin", Status::Default),
        ("Active", Status::Success),
        ("2026-03-02", Status::Default),
    ],
    &[
        ("Grace Hopper", Status::Default),
        ("grace@uni.dev", Status::Default),
        ("Editor", Status::Default),
        ("Invited", Status::Info),
        ("2026-01-18", Status::Default),
    ],
    &[
        ("Alan Turing", Status::Default),
        ("alan@uni.dev", Status::Default),
        ("Viewer", Status::Default),
        ("Suspended", Status::Danger),
        ("2025-11-04", Status::Default),
    ],
    &[
        ("Katherine Johnson", Status::Default),
        ("katherine@uni.dev", Status::Default),
        ("Editor", Status::Default),
        ("Active", Status::Success),
        ("2026-02-11", Status::Default),
    ],
    &[
        ("Edsger Dijkstra", Status::Default),
        ("edsger@uni.dev", Status::Default),
        ("Admin", Status::Default),
        ("Active", Status::Success),
        ("2025-12-20", Status::Default),
    ],
];

/// The invoice columns, the Slint example's three: two fixed and the amount
/// taking what is left, end-aligned, every header sortable.
fn invoice_columns() -> Vec<Column> {
    vec![
        Column::new("invoice", "Invoice").width(100.0).sortable(),
        Column::new("status", "Status").width(100.0).sortable(),
        Column::new("amount", "Amount").align(Align::End).sortable(),
    ]
}

/// A cell's sort key: a number when the text reads as one — so `$1,200` sorts
/// above `$90` and `+3.18%` above `-1.10%` — and a lowercased string otherwise.
/// The Slint `TableSort` reads its rows the same way.
fn value(cells: Cells, column: usize) -> (bool, f64, String) {
    let text = cells.get(column).map(|cell| cell.0).unwrap_or_default();
    let cleaned: String = text
        .chars()
        .filter(|c| !matches!(c, '$' | ',' | '%' | '+'))
        .collect();
    match cleaned.parse::<f64>() {
        Ok(number) => (true, number, String::new()),
        Err(_) => (false, 0.0, text.to_lowercase()),
    }
}

/// The rows in display order: the caller's sort, applied to the caller's data.
/// This is the order function the Slint `TableSort` global is, written where the
/// application can see the rows it reorders.
fn rows_for(records: &[Cells], sort: Option<(usize, ColumnSort)>) -> Vec<Row> {
    let mut order: Vec<Cells> = records.to_vec();

    // A default sort is the natural order: the rows are left exactly as given.
    let sorted = sort.filter(|(_, direction)| direction.is_sorted());
    if let Some((column, direction)) = sorted {
        order.sort_by(|a, b| {
            value(a, column)
                .partial_cmp(&value(b, column))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if direction == ColumnSort::Descending {
            order.reverse();
        }
    }

    order
        .into_iter()
        .map(|cells| {
            let mut row = Row::new(cells.first().map(|cell| cell.0).unwrap_or_default());
            for (label, status) in cells.iter() {
                row = row.cell(Cell::new(*label).status(*status));
            }
            row
        })
        .collect()
}

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — data table")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// One sort per table slot, or none while it is unsorted.
    sorts: Vec<Option<(usize, ColumnSort)>>,
    /// One selected row per table slot, as a display index.
    selected: Vec<Option<usize>>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            sorts: vec![None; section::COUNT],
            selected: vec![None; section::COUNT],
            hovered: None,
            pressed_source: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    /// A sortable header was pressed: the direction the cycle landed on.
    Sort {
        table: usize,
        column: usize,
        direction: ColumnSort,
    },
    /// A row was pressed, by display index.
    Select {
        table: usize,
        index: usize,
    },
    ClearSelection(usize),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Sort {
            table,
            column,
            direction,
        } => {
            app.sorts[table] = if direction.is_sorted() {
                Some((column, direction))
            } else {
                None
            };
            // Slint clears the selection and returns to the top when the sort
            // changes, because the row indices moved under the selection.
            app.selected[table] = None;
        }
        Message::Select { table, index } => app.selected[table] = Some(index),
        Message::ClearSelection(table) => app.selected[table] = None,
        Message::HoverSource(title) => app.hovered = title,
        Message::PressSource(title) => app.pressed_source = title,
        Message::ShowSource { index, title } => {
            app.pressed_source = None;
            // The page scrolls to the sample; natively this does nothing.
            crate::bridge::show(index, title);
        }
    }
    Task::none()
}

/// One table slot: the caller's columns and records, with the sort, the
/// selection and the two handlers the app keeps per slot. Every section builds
/// its table through this, so the wiring is written once.
fn data_table(
    app: &App,
    table: usize,
    columns: Vec<Column>,
    records: &[Cells],
) -> DataTable<'static, Message> {
    let sort = app.sorts[table];
    let mut control = DataTable::new()
        .columns(columns)
        .rows(rows_for(records, sort))
        .selected(app.selected[table])
        .on_sort(move |column, direction| Message::Sort {
            table,
            column,
            direction,
        })
        .on_select(move |index| Message::Select { table, index });

    if let Some((column, direction)) = sort {
        control = control.sort(column, direction);
    }
    control
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(
            text(title)
                .size(theme::FONT_SIZE_SM)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..iced::Font::DEFAULT
                }),
        )
        .width(Length::Fill)
        .center_x(Length::Fill)
        .style(|theme: &Theme| container::Style {
            text_color: Some(Tokens::of(theme).muted_foreground),
            ..container::Style::default()
        }),
    )
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    let icon = mouse_area(
        container(
            svg(svg::Handle::from_memory(CODE_XML))
                .width(16.0)
                .height(16.0)
                .style(|theme: &Theme, _status| svg::Style {
                    color: Some(Tokens::of(theme).primary),
                }),
        )
        .width(28.0)
        .height(28.0)
        .center_x(28.0)
        .center_y(28.0)
        .style(move |theme: &Theme| {
            let primary = Tokens::of(theme).primary;
            let (fill, line) = if pressed {
                (0.34, 0.7)
            } else if hovered {
                (0.22, 0.55)
            } else {
                (0.12, 0.35)
            };
            container::Style {
                background: Some(primary.scale_alpha(fill).into()),
                border: iced::border::rounded(theme::RADIUS_LG)
                    .color(primary.scale_alpha(line))
                    .width(1.0),
                ..container::Style::default()
            }
        }),
    )
    .on_enter(Message::HoverSource(Some(title)))
    .on_exit(Message::HoverSource(None))
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    row![label, icon]
        .spacing(8.0)
        .align_y(Alignment::Center)
        .into()
}

/// The box under a heading: the same `GroupBox` with the outline variant the Slint
/// example wraps its sections in, untitled because the heading above it carries
/// the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// The muted line a section uses to report the selection, the Slint example's
/// `Text` under its selectable tables.
fn selection_line<'a>(
    app: &App,
    table: usize,
    selected: &'static str,
    empty: &'static str,
    zero_based: bool,
) -> Element<'a, Message> {
    let label = match app.selected[table] {
        Some(index) if zero_based => format!("{selected}{index}"),
        Some(index) => format!("{selected}{}", index + 1),
        None => empty.to_string(),
    };

    text(label)
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Invoices
    // The component's own API, written out once: typed columns, typed rows, and
    // the two handlers. The app keeps the sort and the selection, so the header
    // arrow and the selected row are the app's state rather than the table's.
    let basic_columns = vec![
        Column::new("invoice", "Invoice").width(100.0).sortable(),
        Column::new("status", "Status").width(100.0).sortable(),
        Column::new("amount", "Amount").align(Align::End).sortable(),
    ];
    let basic_rows = rows_for(INVOICES, app.sorts[section::INVOICES]);
    let mut invoiced = DataTable::new()
        .columns(basic_columns)
        .rows(basic_rows)
        .selected(app.selected[section::INVOICES])
        .height(Length::Fixed(208.0))
        .on_sort(|column, direction| Message::Sort {
            table: section::INVOICES,
            column,
            direction,
        })
        .on_select(|index| Message::Select {
            table: section::INVOICES,
            index,
        });
    if let Some((column, direction)) = app.sorts[section::INVOICES] {
        invoiced = invoiced.sort(column, direction);
    }
    let invoices = invoiced;

    // section: Column Configuration
    // A column's width, alignment and sortability are its own: a fixed ID, a
    // sortable name, an end-aligned age, and an email taking what is left. A
    // header without `sortable` draws no indicator and no hover.
    let configuration = body_box(
        data_table(
            app,
            section::CONFIGURATION,
            vec![
                Column::new("id", "ID").width(64.0),
                Column::new("name", "Name").width(140.0).sortable(),
                Column::new("age", "Age").width(72.0).align(Align::End).sortable(),
                Column::new("email", "Email"),
            ],
            PEOPLE,
        )
        .height(Length::Fixed(150.0)),
    );

    // section: Virtual Scrolling for Large Datasets
    // Slint's body instantiates only the visible rows, so thousands stay cheap.
    // iced's `scrollable` lays out and draws every row it is given, on every
    // pass; this bounded body scrolls all sixteen, and the module names the gap
    // rather than pretending a windowing layer exists.
    let virtual_scrolling = body_box(
        data_table(app, section::VIRTUAL, invoice_columns(), VIRTUAL).height(Length::Fixed(190.0)),
    );

    // section: Sorting Implementation
    // Sorting stays with the application, which is the Slint `sort-changed`
    // branch: the header reports the direction the cycle lands on and the caller
    // reorders. `rows_for` is that order function, reading numbers by value so
    // `$1,200` sorts above `$90`.
    let sorting = body_box(
        data_table(
            app,
            section::SORTING,
            vec![
                Column::new("name", "Name").width(160.0).sortable(),
                Column::new("age", "Age").width(72.0).align(Align::End).sortable(),
                Column::new("email", "Email").sortable(),
            ],
            NAMES,
        )
        .height(Length::Fixed(170.0)),
    );

    // section: Cell Rendering
    // A cell's status colours its text — `Theme.status_color` in the Slint
    // component — so a gain reads green and a loss red without a badge in the
    // row.
    let cells = body_box(
        data_table(
            app,
            section::CELLS,
            vec![
                Column::new("symbol", "Symbol").width(100.0).sortable(),
                Column::new("side", "Side").width(80.0),
                Column::new("pnl", "P&L").align(Align::End).sortable(),
            ],
            TRADES,
        )
        .height(Length::Fixed(150.0)),
    );

    // section: Selection Modes
    // The table is selectable whenever the caller hands it an `on_select`; the
    // selected index is drawn on the selection token, which is the Slint
    // `Theme.selection` at 0.15 in the light theme and 0.2 in the dark one.
    let selection_line_below = selection_line(
        app,
        section::SELECTION,
        "Selected row ",
        "No row selected",
        false,
    );
    let selection = body_box(column![
        data_table(app, section::SELECTION, invoice_columns(), INVOICES).height(Length::Fixed(150.0)),
        selection_line_below,
    ]
    .spacing(theme::SPACING_SM));

    // section: Reading and Writing the Selection
    // The selection is one read and one write on the caller's side: a button can
    // set it and clear it without the table knowing, which is the Slint
    // `selected-row` property.
    let selection_controls = row![
        Button::new("Select row 3")
            .outline()
            .on_press(Message::Select {
                table: section::SELECTION_API,
                index: 2,
            }),
        Button::new("Clear")
            .outline()
            .on_press(Message::ClearSelection(section::SELECTION_API)),
    ]
    .spacing(theme::SPACING_SM);
    let selection_api_line =
        selection_line(app, section::SELECTION_API, "selected_row: ", "selected_row: none", true);
    let selection_api = body_box(column![
        selection_controls,
        data_table(app, section::SELECTION_API, invoice_columns(), INVOICES)
            .height(Length::Fixed(150.0)),
        selection_api_line,
    ]
    .spacing(theme::SPACING_SM));

    // section: Column Resizing and Moving
    // Slint drags a handle on the header and writes the new width back. iced has
    // no drag delta to hand a controlled widget, so the width is what the caller
    // sets: 120 and 120 here, and the amount taking the rest.
    let resizing = body_box(
        data_table(
            app,
            section::RESIZING,
            vec![
                Column::new("invoice", "Invoice").width(120.0).sortable(),
                Column::new("status", "Status").width(120.0).sortable(),
                Column::new("amount", "Amount").align(Align::End).sortable(),
            ],
            INVOICES,
        )
        .height(Length::Fixed(150.0)),
    );

    // section: Table Styling
    // The three appearance switches the Slint example offers: a size for the row
    // height and the cell padding, `stripe` for the alternating tint, and
    // `bordered` for the edge and the rounded corners.
    let styling = body_box(
        data_table(app, section::STYLING, invoice_columns(), INVOICES)
            .size(ControlSize::Large)
            .stripe(true)
            .bordered(true)
            .height(Length::Fixed(190.0)),
    );

    // section: Financial Data Table
    // Six columns of quotes, the change in the success and danger colours, and
    // the stripe the Slint example asks for on a wide table.
    let financial = body_box(
        data_table(
            app,
            section::FINANCIAL,
            vec![
                Column::new("symbol", "Symbol").width(88.0).sortable(),
                Column::new("name", "Name").width(180.0).sortable(),
                Column::new("price", "Price").width(88.0).align(Align::End).sortable(),
                Column::new("change", "Chg").width(80.0).align(Align::End).sortable(),
                Column::new("volume", "Volume").width(120.0).align(Align::End).sortable(),
                Column::new("market_cap", "Market Cap")
                    .width(120.0)
                    .align(Align::End)
                    .sortable(),
            ],
            QUOTES,
        )
        .stripe(true)
        .height(Length::Fixed(250.0)),
    );

    // section: User Management Table
    // The five-column people table, with the role and the status drawn from the
    // same cell status the rest of the demo uses.
    let users = body_box(
        data_table(
            app,
            section::USERS,
            vec![
                Column::new("name", "Name").width(160.0).sortable(),
                Column::new("email", "Email").width(180.0).sortable(),
                Column::new("role", "Role").width(90.0).sortable(),
                Column::new("status", "Status").width(100.0).sortable(),
                Column::new("last_login", "Last Login").sortable(),
            ],
            USERS,
        )
        .height(Length::Fixed(210.0)),
    );

    // section: end
    let status = text("Press a heading or its code icon — the page scrolls to that sample.")
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        });

    let gallery = column![
        heading(app, 0, "Invoices"),
        body_box(invoices),
        heading(app, 1, "Column Configuration"),
        configuration,
        heading(app, 2, "Virtual Scrolling for Large Datasets"),
        virtual_scrolling,
        heading(app, 3, "Sorting Implementation"),
        sorting,
        heading(app, 4, "Cell Rendering"),
        cells,
        heading(app, 5, "Selection Modes"),
        selection,
        heading(app, 6, "Reading and Writing the Selection"),
        selection_api,
        heading(app, 7, "Column Resizing and Moving"),
        resizing,
        heading(app, 8, "Table Styling"),
        styling,
        heading(app, 9, "Financial Data Table"),
        financial,
        heading(app, 10, "User Management Table"),
        users,
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The gallery's own scroll area is the component the page documents, so the
    // frame is drawn by the thing under test.
    crate::components::scrollable::Scrollable::new(gallery)
        .height(Length::Fill)
        .into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
