//! A data table with one page of rows and the control that moves between pages.
//!
//! The block is the table and its footer together: a page of rows, the count
//! that says which part of the result is on screen, and a `Pagination` that
//! replaces the rows. The footer is pinned below the table rather than scrolling
//! with it, so the control stays reachable however many rows a page holds.

use gpui_kit::component::{
    ActiveTheme as _, Sizable as _, Size, StyledExt, h_flex,
    pagination::Pagination,
    table::{Column, ColumnSort, DataTable, TableDelegate, TableState},
    tag::Tag,
    v_flex,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    App, AppContext as _, Context, Div, Entity, FocusHandle, Focusable, InteractiveElement,
    IntoElement, ParentElement, Render, Styled, TextAlign, Window, div, px,
};

use crate::{ChangeStorySize, story_toolbar};

/// Rows on one page, and how many pages the result has. Twenty pages is what
/// gives the control somewhere to go.
const PAGE_SIZE: usize = 5;
const PAGES: usize = 20;
const TOTAL: usize = PAGE_SIZE * PAGES;

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

/// One invoice.
struct Invoice {
    id: String,
    customer: &'static str,
    status: &'static str,
    method: &'static str,
    amount: String,
}

/// The invoice at `index` in the result, newest first. Generated from the index
/// rather than listed: twenty pages of data is one rule here, where the same
/// list written out would be a hundred literals nobody reads.
fn invoice(index: usize) -> Invoice {
    let status = STATUSES[(index * 3) % STATUSES.len()];
    // A mixer rather than a step, so consecutive rows do not read as an
    // arithmetic sequence: the same rule the Slint page uses.
    let dollars = ((index * 2654435) % 100003) % 4800 + 120;
    let cents = (index * 37) % 100;

    Invoice {
        id: format!("INV-{:04}", TOTAL - index),
        customer: CUSTOMERS[(index * 7) % CUSTOMERS.len()],
        status,
        method: match status {
            "Paid" => "Card",
            "Pending" => "Bank transfer",
            "Overdue" => "Bank transfer",
            _ => "—",
        },
        amount: if dollars >= 1000 {
            format!("${},{:03}.{:02}", dollars / 1000, dollars % 1000, cents)
        } else {
            format!("${}.{:02}", dollars, cents)
        },
    }
}

fn status_tag(status: &str) -> Tag {
    match status {
        "Paid" => Tag::success().outline().child(status.to_string()),
        "Pending" => Tag::warning().outline().child(status.to_string()),
        "Overdue" => Tag::danger().outline().child(status.to_string()),
        _ => Tag::new().child(status.to_string()),
    }
    .xsmall()
}

pub struct PaginatedTableStory {
    focus_handle: FocusHandle,
    size: Size,
    /// One-based, matching `Pagination`'s own numbering.
    page: usize,
    table: Entity<TableState<InvoiceTable>>,
}

/// The table's rows for the page it is showing.
struct InvoiceTable {
    columns: Vec<Column>,
    /// Zero-based, because it indexes `INVOICES` rather than a page control.
    page: usize,
}

impl InvoiceTable {
    fn new(page: usize) -> Self {
        Self {
            columns: vec![
                Column::new("invoice", "Invoice").width(px(120.)),
                Column::new("customer", "Customer").width(px(220.)),
                Column::new("status", "Status").width(px(120.)),
                Column::new("method", "Method").width(px(170.)),
                Column::new("amount", "Amount").width(px(140.)).text_right(),
            ],
            page,
        }
    }

    /// The cell frame, so a right-aligned column lines up with its heading.
    fn cell(&self, col: &Column) -> Div {
        div()
            .h_full()
            .h_flex()
            .items_center()
            .when(col.align == TextAlign::Right, |this| this.justify_end())
    }
}

impl TableDelegate for InvoiceTable {
    fn columns_count(&self, _: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _: &App) -> usize {
        PAGE_SIZE
    }

    fn column(&self, col_ix: usize, _: &App) -> Column {
        self.columns[col_ix].clone()
    }

    fn perform_sort(
        &mut self,
        _: usize,
        _: ColumnSort,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) {
        // One page is on screen at a time, so sorting the visible five would be
        // a statement about the page rather than the result. A real screen sorts
        // the whole set and returns to page one.
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let row = invoice(self.page * PAGE_SIZE + row_ix);
        let Some(col) = self.columns.get(col_ix) else {
            return div().into_any_element();
        };

        match col_ix {
            0 => self.cell(col).child(row.id).into_any_element(),
            1 => self.cell(col).child(row.customer).into_any_element(),
            2 => self
                .cell(col)
                .child(status_tag(row.status))
                .into_any_element(),
            3 => self.cell(col).child(row.method).into_any_element(),
            4 => self.cell(col).child(row.amount).into_any_element(),
            _ => div().into_any_element(),
        }
    }
}

impl super::Story for PaginatedTableStory {
    fn title() -> &'static str {
        "Paginated Table"
    }

    fn description() -> &'static str {
        "A data table showing one page of a result, with the control that moves between pages."
    }

    fn new_view(window: &mut Window, cx: &mut App) -> Entity<impl Render> {
        Self::view(window, cx)
    }
}

impl PaginatedTableStory {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let table = cx.new(|cx| TableState::new(InvoiceTable::new(0), window, cx));

        Self {
            focus_handle: cx.focus_handle(),
            size: Size::default(),
            page: 1,
            table,
        }
    }

    fn go_to_page(&mut self, page: usize, cx: &mut Context<Self>) {
        self.page = page;
        let table = self.table.clone();
        table.update(cx, |table, cx| {
            table.delegate_mut().page = page.saturating_sub(1);
            table.refresh(cx);
        });
        cx.notify();
    }
}

impl Focusable for PaginatedTableStory {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PaginatedTableStory {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        let first = (self.page - 1) * PAGE_SIZE + 1;
        let last = self.page * PAGE_SIZE;
        // A heading row, a row per invoice, and the table's two border pixels.
        let page_height = self.size.table_row_height() * (PAGE_SIZE as f32 + 1.) + px(2.);

        v_flex()
            .size_full()
            .gap_4()
            .on_action(cx.listener(|this, action: &ChangeStorySize, _, cx| {
                this.size = action.0;
                let table = this.table.clone();
                table.update(cx, |table, cx| table.refresh(cx));
                cx.notify();
            }))
            .child(story_toolbar(self.size))
            .child(
                v_flex()
                    .min_h_0()
                    .flex_1()
                    // A page is a heading row and five rows, so the table is sized
                    // rather than stretched: filling the frame would leave a field
                    // of empty rows under the data, which reads as missing rather
                    // than as room to spare. The same rule the Slint page uses.
                    .child(
                        div().w_full().h(page_height).child(
                            DataTable::new(&self.table)
                                .with_size(self.size)
                                .stripe(true),
                        ),
                    )
                    // The footer is part of the block: the count says where the
                    // page sits in the result, the control replaces the rows.
                    .child(
                        h_flex()
                            .w_full()
                            .min_h_9()
                            .items_center()
                            .justify_between()
                            .gap_3()
                            .px_3()
                            .bg(cx.theme().muted.opacity(0.35))
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("Showing {}–{} of {} invoices", first, last, TOTAL))
                            .child(
                                Pagination::new("paginated-table")
                                    .current_page(self.page)
                                    .total_pages(PAGES)
                                    .with_size(self.size)
                                    .on_click({
                                        let entity = entity.clone();
                                        move |page, _, cx| {
                                            entity.update(cx, |this, cx| {
                                                this.go_to_page(*page, cx);
                                            });
                                        }
                                    }),
                            ),
                    ),
            )
    }
}
