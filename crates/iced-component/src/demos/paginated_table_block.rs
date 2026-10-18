//! `paginated-table-block` — the Blocks page's result screen: a command bar over
//! one page of invoices, with the count and the control that replaces the rows
//! underneath them.
//!
//! A block is not a component sample. This is the finished application screen the
//! site's Blocks page shows for iced, composed from the same files
//! `uni-kit add iced` writes: the [`Toolbar`] with its command groups, the
//! [`DataTable`] showing one page of the invoice result, and the [`Pagination`]
//! that moves to the next page beside the count that says which part of the
//! result is on screen.
//!
//! The page is the screen's one state. The table draws the rows of the page the
//! caller holds and the footer's count reads the same number, so a press on the
//! control moves the rows and the count together. The rows are generated from
//! their index with the reference's own rule rather than written out — twenty
//! pages is a hundred invoices nobody reads — and the rule is the one GPUI's
//! invoice story uses, so page one is the Slint page's own invoice list.
//!
//! Sorting is left out on purpose. The reference block draws the same five rows
//! whatever a header is pressed with, because the result is not in this view and
//! sorting the visible page would be a statement about the page rather than the
//! result; the bar's three utility commands are the reference's chrome and the
//! block has no destination for them, which is what GPUI's block does too.
//!
//! Run it with `cargo run --bin iced-gallery -- paginated-table-block`, or in the
//! browser as `?story=paginated-table-block` in the gallery the website embeds.
//! The Blocks page asks for that story and reads this file for its usage prompt.

use iced::widget::{Space, column, container, row, svg, text};
use iced::{Alignment, Color, Element, Length, Task, Theme};

use crate::components::button::{Button, Variant as ButtonVariant};
use crate::components::data_table::{Align, Cell, Column, DataTable, Row};
use crate::components::pagination::Pagination;
use crate::components::theme::{self, ControlSize, Status, Tokens};
use crate::components::toolbar::{Toolbar, ToolbarGroup, ToolbarSeparator, ToolbarSpacer};

/// Rows on one page, and how many pages the result has. Twenty pages is what
/// gives the control somewhere to go.
const PAGE_SIZE: usize = 5;
const PAGES: usize = 20;
const TOTAL: usize = PAGE_SIZE * PAGES;

/// The table's row height at [`ControlSize::Small`], the Slint row ladder's
/// 30px. A page is five of them, so the body is sized to its rows rather than
/// stretched: a frame of empty rows under the data reads as missing data rather
/// than as room to spare, which is the same rule the GPUI story states.
const ROW_HEIGHT: f32 = 30.0;

/// The frame's inset and the gap between the bar, the table and the footer.
const INSET: f32 = 16.0;
const GAP: f32 = 12.0;

/// The customers the result is drawn from, cycled so no page repeats one.
const CUSTOMERS: [&str; 24] = [
    "Northwind Trading",
    "Contoso Ltd",
    "Fabrikam",
    "Adventure Works",
    "Litware Inc",
    "Tailspin Toys",
    "Proseware",
    "Wide World Importers",
    "Fourth Coffee",
    "Wingtip Toys",
    "Lucerne Publishing",
    "Trey Research",
    "Blue Yonder Airlines",
    "Coho Vineyard",
    "Alpine Ski House",
    "Humongous Insurance",
    "Margie's Travel",
    "Relecloud",
    "Southridge Video",
    "VanArsdel Ltd",
    "Woodgrove Bank",
    "Graphic Design Institute",
    "School of Fine Art",
    "VanArsdel Research",
];

/// The statuses, weighted towards the ones that settle.
const STATUSES: [&str; 8] = [
    "Paid", "Paid", "Pending", "Paid", "Overdue", "Paid", "Pending", "Draft",
];

// The Lucide marks the bar uses, from the same ISC set the Slint block draws:
// `funnel`, `columns-3`, `download` and `plus`.

/// `funnel`, the Filter command.
const FUNNEL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 20a1 1 0 0 0 .553.895l2 1A1 1 0 0 0 14 21v-7a2 2 0 0 1 .517-1.341L21.74 4.67A1 1 0 0 0 21 3H3a1 1 0 0 0-.742 1.67l7.225 7.989A2 2 0 0 1 10 14z"/></svg>"#;
/// `columns-3`, the Columns command.
const COLUMNS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/><path d="M15 3v18"/></svg>"#;
/// `download`, the Export command.
const DOWNLOAD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/><path d="M12 15V3"/></svg>"#;
/// `plus`, the New invoice command's mark.
const PLUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>"#;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — paginated table")
        .theme(theme)
        .run()
}

/// The screen's whole state: which page of the result is on screen, one-based
/// like the control's own numbering.
#[derive(Debug, Clone)]
struct App {
    page: usize,
}

impl Default for App {
    fn default() -> Self {
        Self { page: 1 }
    }
}

#[derive(Debug, Clone)]
enum Message {
    /// A page was asked for: a step or a page number. The control clamps the
    /// value before it reports, and the screen still keeps it as given.
    Page(usize),
    /// One of the bar's three utility commands. The reference screen's bar is
    /// chrome around the table, so a press reports and the block has no
    /// destination for it — GPUI's block binds an empty `on_click` for exactly
    /// this reason.
    Command,
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Page(page) => app.page = page,
        Message::Command => {}
    }
    Task::none()
}

/// An SVG in a square box, coloured from the tokens in force while it renders.
fn glyph<'a, Message: 'a>(
    bytes: &'static [u8],
    size: f32,
    ink: fn(Tokens) -> Color,
) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(ink(Tokens::of(theme))),
        })
        .into()
}

/// One invoice of the result.
struct Invoice {
    id: String,
    customer: &'static str,
    status: &'static str,
    method: &'static str,
    amount: String,
    tone: Status,
}

/// The invoice at `index` in the result, newest first. Generated from the index
/// rather than listed: twenty pages of data is one rule here, where the same
/// list written out would be a hundred literals nobody reads. The mixer is the
/// GPUI story's, so the first page is the Slint block's rows.
fn invoice(index: usize) -> Invoice {
    let status = STATUSES[(index * 3) % STATUSES.len()];
    // A mixer rather than a step, so consecutive rows do not read as an
    // arithmetic sequence.
    let dollars = ((index * 2654435) % 100003) % 4800 + 120;
    let cents = (index * 37) % 100;

    Invoice {
        id: format!("INV-{:04}", TOTAL - index),
        customer: CUSTOMERS[(index * 7) % CUSTOMERS.len()],
        status,
        method: match status {
            "Paid" => "Card",
            "Pending" | "Overdue" => "Bank transfer",
            _ => "—",
        },
        amount: if dollars >= 1000 {
            format!("${},{:03}.{:02}", dollars / 1000, dollars % 1000, cents)
        } else {
            format!("${}.{:02}", dollars, cents)
        },
        tone: match status {
            "Paid" => Status::Success,
            "Pending" => Status::Warning,
            "Overdue" => Status::Danger,
            _ => Status::Default,
        },
    }
}

/// The bar above the table: the three utility commands, a rule, and the one
/// commitment. The spacer keeps the commands at the leading edge, which is the
/// Slint block's `ToolbarSpacer`.
fn command_bar<'a>() -> Element<'a, Message> {
    let mut commands = ToolbarGroup::new().size(ControlSize::Small);
    for (label, mark) in [
        ("Filter", FUNNEL),
        ("Columns", COLUMNS),
        ("Export", DOWNLOAD),
    ] {
        commands = commands.child(
            Button::new(label)
                .size(ControlSize::Small)
                .icon(glyph(mark, 14.0, |tokens: Tokens| tokens.foreground))
                .on_press(Message::Command),
        );
    }

    Toolbar::new()
        .size(ControlSize::Small)
        .spacing(theme::SPACING_SM)
        .child(commands)
        .child(ToolbarSeparator::new())
        .child(
            Button::new("New invoice")
                .variant(ButtonVariant::Primary)
                .size(ControlSize::Small)
                .icon(glyph(PLUS, 14.0, |tokens: Tokens| {
                    tokens.primary_foreground
                }))
                .on_press(Message::Command),
        )
        .content(ToolbarSpacer::new())
        .into()
}

/// The table's columns. Customer takes what the fixed columns leave, which is
/// the Slint block's `width: 0`.
fn columns() -> [Column; 5] {
    [
        Column::new("invoice", "Invoice").width(120.0),
        Column::new("customer", "Customer"),
        Column::new("status", "Status").width(120.0),
        Column::new("method", "Method").width(140.0),
        Column::new("amount", "Amount")
            .width(120.0)
            .align(Align::End),
    ]
}

/// One page of the result. The caller has already decided which page that is;
/// the table draws the rows it is handed.
fn invoice_table<'a>(page: usize) -> Element<'a, Message> {
    let first = (page - 1) * PAGE_SIZE;
    let rows = (0..PAGE_SIZE).map(|row| {
        let invoice = invoice(first + row);
        Row::new(invoice.id.clone()).cells([
            Cell::new(invoice.id.clone()),
            Cell::new(invoice.customer),
            Cell::new(invoice.status).status(invoice.tone),
            Cell::new(invoice.method),
            Cell::new(invoice.amount),
        ])
    });

    DataTable::new()
        .columns(columns())
        .rows(rows)
        .stripe(true)
        .size(ControlSize::Small)
        .height(Length::Fixed(ROW_HEIGHT * PAGE_SIZE as f32))
        .into()
}

/// The row under the table: where this page sits in the result on the left, and
/// the control that replaces it on the right. It is pinned under the table
/// rather than scrolling with it, so it stays reachable.
fn footer<'a>(page: usize) -> Element<'a, Message> {
    let first = (page - 1) * PAGE_SIZE + 1;
    let last = page * PAGE_SIZE;

    row![
        text(format!("Showing {first}–{last} of {TOTAL} invoices"))
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
        Space::new().width(Length::Fill),
        Pagination::new()
            .page(page)
            .pages(PAGES)
            .compact(false)
            .on_change(Message::Page),
    ]
    .spacing(GAP)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // The page is sized to its rows and the spacer takes the rest, so the frame
    // keeps the table at the top and the footer against the bottom edge.
    let frame = column![
        command_bar(),
        invoice_table(app.page),
        Space::new().height(Length::Fill),
        footer(app.page),
    ]
    .spacing(GAP)
    .width(Length::Fill)
    .height(Length::Fill);

    container(frame)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(INSET)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            border: iced::Border {
                color: Tokens::of(theme).border,
                width: 1.0,
                radius: theme::RADIUS_LG.into(),
            },
            ..container::Style::default()
        })
        .into()
}
