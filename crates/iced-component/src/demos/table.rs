//! `table` — a static table: header, body, footer and caption, built from the
//! Slint example's invoice data with the striped, bordered and size switches it
//! offers.
//!
//! Run it with `cargo run --example table`, or in the browser as `?story=table`
//! in the gallery the website embeds.
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
//! The headings this demo and the page share are the ones iced can honestly
//! fill: **Basic table**, **Striped, bordered and sizes**, **Column widths**,
//! **Text Alignment**, and **Without Border (via Styled)** — where iced has no
//! `Styled` trait, so the section shows `bordered(false)`, the component's own
//! switch for the same result. The page's **Custom Styling** example is left
//! without an iced sample rather than shown as a stand-in: iced has no fluent
//! style trait to apply, and the table's own surface already reads the tokens.

use iced::widget::{column, container, mouse_area, row, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::Button;
use crate::components::group_box::{self, GroupBox};
use crate::components::table::{
    Align, Table, TableBody, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};
use crate::components::tag::Tag;
use crate::components::theme::{self, ControlSize, Status, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The Slint example's invoices, long enough that the first table's bounded body
/// scrolls under its header.
const INVOICES: [(&str, &str, &str, &str); 12] = [
    ("INV001", "Paid", "Credit Card", "$250.00"),
    ("INV002", "Pending", "PayPal", "$150.00"),
    ("INV003", "Unpaid", "Bank Transfer", "$350.00"),
    ("INV004", "Paid", "Credit Card", "$450.00"),
    ("INV005", "Paid", "PayPal", "$550.00"),
    ("INV006", "Pending", "Bank Transfer", "$650.00"),
    ("INV007", "Paid", "Credit Card", "$750.00"),
    ("INV008", "Unpaid", "PayPal", "$850.00"),
    ("INV009", "Paid", "Bank Transfer", "$950.00"),
    ("INV010", "Paid", "Credit Card", "$1,050.00"),
    ("INV011", "Pending", "PayPal", "$1,150.00"),
    ("INV012", "Paid", "Bank Transfer", "$1,250.00"),
];

/// The height of the first table's body, so its rows scroll while the header and
/// caption stay put.
const BODY_HEIGHT: f32 = 220.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — table")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// The switches the Slint example offers: the stripe, the border, and the
    /// cell size.
    striped: bool,
    bordered: bool,
    size: ControlSize,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            striped: false,
            bordered: true,
            size: ControlSize::Medium,
            hovered: None,
            pressed_source: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    ToggleStriped,
    ToggleBordered,
    SetSize(ControlSize),
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
        Message::ToggleStriped => app.striped = !app.striped,
        Message::ToggleBordered => app.bordered = !app.bordered,
        Message::SetSize(size) => app.size = size,
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

/// A status drawn the way the Slint example draws it: a tinted badge in the
/// status colour rather than plain text in the row.
fn status_tag<'a, Message: Clone + 'a>(label: &str) -> Element<'a, Message> {
    let status = match label {
        "Paid" => Status::Success,
        "Pending" => Status::Warning,
        _ => Status::Danger,
    };
    Tag::new(label).status(status).into()
}

/// The four columns of the invoice rows at one size, so the size switch rebuilds
/// the same row rather than a second copy of it.
fn invoice_row<'a, Message: Clone + 'a>(
    index: usize,
    invoice: &str,
    status: &str,
    method: &str,
    amount: &str,
    size: ControlSize,
    badge: bool,
) -> TableRow<'a, Message> {
    let status_cell = if badge {
        TableCell::new(status).content(status_tag(status))
    } else {
        TableCell::new(status)
    };

    TableRow::new()
        .index(index)
        .cell(TableCell::new(invoice).mono(true).size(size))
        .cell(status_cell.size(size))
        .cell(TableCell::new(method).size(size))
        .cell(
            TableCell::new(amount)
                .align(Align::End)
                .mono(true)
                .size(size),
        )
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

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic table
    // The header band, twelve rows in a bounded scroll area, and the caption.
    // Slint stacks the rows and lets the page scroll; this port puts the body on
    // `scrollable`, so the header and the caption stay while the rows move. The
    // `index` on each row is what draws the rule above it.
    let mut basic_rows = Vec::new();
    for (index, (invoice, status, method, amount)) in INVOICES.iter().enumerate() {
        basic_rows.push(invoice_row(
            index,
            invoice,
            status,
            method,
            amount,
            ControlSize::Medium,
            false,
        ));
    }

    let basic = Table::new()
        .header(TableHeader::new().row(
            TableRow::new()
                .head("Invoice")
                .head("Status")
                .head("Method")
                .head(TableHead::new("Amount").align(Align::End)),
        ))
        .body(TableBody::new(basic_rows).height(Length::Fixed(BODY_HEIGHT)))
        .caption("A list of your recent invoices.");

    // section: Striped, bordered and sizes
    // The three switches the Slint example puts above its table. Only what the
    // caller changes is state: the table itself reads the switch back.
    let controls = row![
        Button::new("Striped")
            .selected(app.striped)
            .on_press(Message::ToggleStriped),
        Button::new("Bordered")
            .selected(app.bordered)
            .on_press(Message::ToggleBordered),
        Button::new("Small")
            .selected(app.size == ControlSize::Small)
            .on_press(Message::SetSize(ControlSize::Small)),
        Button::new("Medium")
            .selected(app.size == ControlSize::Medium)
            .on_press(Message::SetSize(ControlSize::Medium)),
        Button::new("Large")
            .selected(app.size == ControlSize::Large)
            .on_press(Message::SetSize(ControlSize::Large)),
    ]
    .spacing(theme::SPACING_SM);

    let mut striped_rows = Vec::new();
    for (index, (invoice, status, method, amount)) in INVOICES.iter().take(5).enumerate() {
        striped_rows.push(
            invoice_row(
                index,
                invoice,
                status,
                method,
                amount,
                app.size,
                true,
            )
            .striped(app.striped),
        );
    }

    let striped = Table::new()
        .bordered(app.bordered)
        .header(TableHeader::new().row(
            TableRow::new()
                .head(TableHead::new("Invoice").size(app.size))
                .head(TableHead::new("Status").size(app.size).align(Align::Center))
                .head(TableHead::new("Method").size(app.size))
                .head(TableHead::new("Amount").size(app.size).align(Align::End)),
        ))
        .body(TableBody::new(striped_rows))
        .footer(TableFooter::new().row(
            TableRow::new()
                .cell(TableCell::new("Total").span(3).size(app.size))
                .cell(
                    TableCell::new("$1,750.00")
                        .align(Align::End)
                        .size(app.size),
                ),
        ));

    // section: Column widths
    // A width on the head and the cell of a column pins it, and the columns
    // without one share what is left. Slint expresses the same thing with
    // `width` and leaves the rest at its preferred width.
    let widths = Table::new()
        .header(TableHeader::new().row(
            TableRow::new()
                .head(TableHead::new("ID").width(80.0))
                .head("Name")
                .head(TableHead::new("Date").width(120.0)),
        ))
        .body(TableBody::new([
            TableRow::new()
                .index(0)
                .cell(TableCell::new("INV-0042").mono(true).width(80.0))
                .cell("Northwind Trading")
                .cell(TableCell::new("2026-11-10").width(120.0)),
            TableRow::new()
                .index(1)
                .cell(TableCell::new("INV-0041").mono(true).width(80.0))
                .cell("Contoso Ltd")
                .cell(TableCell::new("2026-11-09").width(120.0)),
        ]));

    // section: Text Alignment
    // A center-aligned header and an end-aligned number, the two alignments a
    // table needs beyond the default start.
    let alignment = Table::new()
        .header(TableHeader::new().row(
            TableRow::new()
                .head("Item")
                .head(TableHead::new("Status").align(Align::Center))
                .head(TableHead::new("Amount").align(Align::End)),
        ))
        .body(TableBody::new([
            TableRow::new()
                .index(0)
                .cell("Team plan")
                .cell(TableCell::new("Active").align(Align::Center))
                .cell(TableCell::new("$250.00").align(Align::End).mono(true)),
            TableRow::new()
                .index(1)
                .cell("Seats")
                .cell(TableCell::new("12").align(Align::Center))
                .cell(TableCell::new("$60.00").align(Align::End).mono(true)),
        ]));

    // section: Without Border (via Styled)
    // GPUI drops the edge with `.border_0().rounded_none()`. iced has no `Styled`
    // trait, so the component carries the switch: `bordered(false)` keeps the
    // fill and the bands and removes the edge and the corners.
    let plain = Table::new()
        .bordered(false)
        .header(TableHeader::new().row(
            TableRow::new()
                .head("Method")
                .head(TableHead::new("Amount").align(Align::End)),
        ))
        .body(TableBody::new([
            TableRow::new()
                .index(0)
                .cell("Credit Card")
                .cell(TableCell::new("$250.00").align(Align::End).mono(true)),
            TableRow::new()
                .index(1)
                .cell("PayPal")
                .cell(TableCell::new("$150.00").align(Align::End).mono(true)),
        ]));

    // section: end
    let status = text("Press a heading or its code icon — the page scrolls to that sample.")
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        });

    let gallery = column![
        heading(app, 0, "Basic table"),
        body_box(basic),
        heading(app, 1, "Striped, bordered and sizes"),
        body_box(column![controls, striped].spacing(theme::SPACING_MD)),
        heading(app, 2, "Column widths"),
        body_box(widths),
        heading(app, 3, "Text Alignment"),
        body_box(alignment),
        heading(app, 4, "Without Border (via Styled)"),
        body_box(plain),
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
