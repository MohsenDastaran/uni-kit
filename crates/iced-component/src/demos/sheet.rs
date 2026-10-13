//! `sheet` — a panel anchored to an edge of the surface it covers, with a
//! header, a scrolling body and a footer action.
//!
//! Run it with `cargo run --example sheet`, or in the browser as
//! `?story=sheet` in the gallery the website embeds.
//!
//! Each section carries a heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! The headings this demo and the page share are the ones the iced component can
//! actually fill: **Basic Sheet**, **Sheet with Placement** and **Sheet with
//! List**. The rest of `sheet.md` describes GPUI's window-level host —
//! `open_sheet`, `margin_top`, a resizable edge, programmatic close — which iced
//! 0.14 has no equivalent for; the component names those gaps in its own module
//! rather than faking them here.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Border, Element, Length, Padding, Task, Theme};

use crate::components::button::Button;
use crate::components::group_box::{self, GroupBox};
use crate::components::sheet::{Placement, Sheet};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — sheet")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// One flag per sheet, because each demo sheet stands in for a window of its
    /// own; a real application owns one.
    basic_open: bool,
    placement_open: bool,
    /// The edge the placement sheet comes from.
    side: Placement,
    /// The list sheet, open from the start the way the Slint example's second
    /// group is.
    list_open: bool,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            basic_open: false,
            placement_open: false,
            side: Placement::Right,
            list_open: true,
            hovered: None,
            pressed_source: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SetBasic(bool),
    SetPlacement(bool),
    SetSide(Placement),
    SetList(bool),
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
        Message::SetBasic(open) => app.basic_open = open,
        Message::SetPlacement(open) => app.placement_open = open,
        Message::SetSide(side) => {
            app.side = side;
            app.placement_open = true;
        }
        Message::SetList(open) => app.list_open = open,
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

/// The name of the edge a placement comes from, which is the placement sheet's
/// title.
fn edge_name(placement: Placement) -> &'static str {
    match placement {
        Placement::Left => "From the left",
        Placement::Top => "From the top",
        Placement::Bottom => "From the bottom",
        Placement::Right => "From the right",
    }
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon is
/// the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and 0.55
/// on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(text(title).size(theme::FONT_SIZE_SM).font(iced::Font {
            weight: iced::font::Weight::Medium,
            ..iced::Font::DEFAULT
        }))
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
/// example wraps its sections in, untitled because the heading above it carries the
/// title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// The framed surface a demo sheet stands in for, the way the Slint example
/// gives each `Sheet` a clipped frame the size of a window. The sheet's panel is
/// anchored to this rectangle, not to the gallery window.
fn frame<'a>(height: f32, hint: &'a str) -> Element<'a, Message> {
    container(text(hint).size(theme::FONT_SIZE_XS).style(|theme: &Theme| {
        iced::widget::text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        }
    }))
    .padding(theme::SPACING_LG)
    .width(Length::Fill)
    .height(Length::Fixed(height))
    .style(|theme: &Theme| {
        let tokens = Tokens::of(theme);
        container::Style {
            background: Some(theme::alpha(tokens.muted, 0.3).into()),
            border: Border {
                color: tokens.border,
                width: 1.0,
                radius: theme::RADIUS_LG.into(),
            },
            ..container::Style::default()
        }
    })
    .into()
}

/// One row of the file list: Slint's 30px muted band.
fn file_row(index: usize) -> Element<'static, Message> {
    container(
        text(format!("report-{}.pdf", index + 1))
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| iced::widget::text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
    )
    .padding(Padding {
        top: 0.0,
        right: 10.0,
        bottom: 0.0,
        left: 10.0,
    })
    .width(Length::Fill)
    .height(Length::Fixed(30.0))
    .align_y(Alignment::Center)
    .style(|theme: &Theme| {
        let tokens = Tokens::of(theme);
        container::Style {
            background: Some(theme::alpha(tokens.muted, 0.4).into()),
            border: Border {
                radius: theme::RADIUS_SM.into(),
                ..Border::default()
            },
            ..container::Style::default()
        }
    })
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic Sheet
    // The whole shape of the component: a title, a description, a body and a
    // footer, closed by the header's control, the overlay or Escape.
    let basic = column![
        Button::new("Open sheet")
            .outline()
            .on_press(Message::SetBasic(true)),
        Sheet::new(frame(
            320.0,
            "The panel covers this frame; the page around it is out of reach.",
        ))
        .open(app.basic_open)
        .title("Notifications")
        .description("You have three unread messages.")
        .body(
            column![
                text("The body scrolls if it needs to, and the header and footer stay put.")
                    .size(theme::FONT_SIZE_SM)
            ]
            .spacing(theme::SPACING_SM),
        )
        .footer("Mark all as read")
        .on_footer_press(Message::SetBasic(false))
        .on_open_change(Message::SetBasic),
    ]
    .spacing(theme::SPACING_MD);

    // section: Sheet with Placement
    // One button per edge, sharing the frame: the panel is anchored to
    // whichever side the button names.
    let edges = row![
        Button::new("Left sheet")
            .outline()
            .on_press(Message::SetSide(Placement::Left)),
        Button::new("Top sheet")
            .outline()
            .on_press(Message::SetSide(Placement::Top)),
        Button::new("Right sheet")
            .outline()
            .on_press(Message::SetSide(Placement::Right)),
        Button::new("Bottom sheet")
            .outline()
            .on_press(Message::SetSide(Placement::Bottom)),
    ]
    .spacing(theme::SPACING_SM);

    let placement = column![
        edges,
        Sheet::new(frame(
            320.0,
            "The panel comes from the edge a button names."
        ))
        .open(app.placement_open)
        .placement(app.side)
        .title(edge_name(app.side))
        .description("Opened from the edge the button names.")
        .body(
            column![
                text("Press a button above to bring the sheet in from another edge.")
                    .size(theme::FONT_SIZE_XS)
            ]
            .spacing(theme::SPACING_SM),
        )
        .on_open_change(Message::SetPlacement),
    ]
    .spacing(theme::SPACING_MD);

    // section: Sheet with List
    // The body scrolls inside the panel, so the header and the footer stay put
    // however long the list is.
    let list = Sheet::new(frame(
        360.0,
        "40 files. The list scrolls; the header does not.",
    ))
    .open(app.list_open)
    .size(280.0)
    .title("File Explorer")
    .description("40 files. The list scrolls; the header does not.")
    .body(column((0..40).map(file_row)).spacing(theme::SPACING_SM))
    .footer("Download all")
    .on_footer_press(Message::SetList(false))
    .on_open_change(Message::SetList);

    // section: end
    let status = text("Press a heading or its code icon — the page scrolls to that sample.");

    let gallery = column![
        heading(app, 0, "Basic Sheet"),
        body_box(basic),
        heading(app, 1, "Sheet with Placement"),
        body_box(placement),
        heading(app, 2, "Sheet with List"),
        body_box(list),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
