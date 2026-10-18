//! `scrollable` — the styled scroll area: the three axes, a custom bar, the
//! viewport it reports, and the tokens its track and thumb are drawn from.
//!
//! Run it with `cargo run --example scrollable`, or in the browser as
//! `?story=scrollable` in the gallery the website embeds.
//!
//! Each section carries the heading the website page gives it — Usage, Custom
//! Scrollbars, Virtualization, Theme Customization — and the code icon that asks
//! the embedding page to scroll to that sample. Clicking either one posts the
//! section's index and title through [`crate::bridge`], the same bridge the Slint
//! and GPUI galleries use.
//!
//! The `// section: <title>` comments below are not decoration: the website reads
//! them to cut this file into the usage panels a component page shows, so the code
//! on the page is the code that runs here rather than a second copy of it. Keep
//! them, one per section, in the order the sections appear, and keep the closing
//! `// section: end` after the last one — without it the lines that assemble the
//! gallery, the status line among them, would be shown as part of the last
//! section.
//!
//! **Virtualization is the one heading this port cannot honour as written.** iced
//! 0.14 has no windowed list and iced's own `table` does not have one either;
//! `scrollable` lays out and draws every child it is given. The section below
//! therefore shows the honest thing — a large list whose rows are all real, and
//! the viewport that `on_scroll` reports, which is the primitive a windowed list
//! is built on — rather than an API that does not exist.

use iced::widget::{column, container, mouse_area, row, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::scrollable::{Axis, Ink, Scrollable, Visibility};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The height of one row in the numbered lists.
const ROW_HEIGHT: f32 = 28.0;

/// The gap between two rows, which is part of a row's pitch for the offset maths
/// in the `Virtualization` section.
const ROW_SPACING: f32 = 4.0;

/// How far the list advances per row: the row and the gap under it.
const ROW_PITCH: f32 = ROW_HEIGHT + ROW_SPACING;

/// The fixed widths the example boxes use, in the Slint example's 120 px cards.
const CARD_WIDTH: f32 = 120.0;
const CARD_HEIGHT: f32 = 88.0;
const CELL_WIDTH: f32 = 88.0;
const CELL_HEIGHT: f32 = 56.0;

/// The list the `Virtualization` section draws: large enough that building all of
/// it is the point, small enough to stay responsive in the browser gallery.
const WINDOW_ROWS: usize = 500;

/// The visible height of that list, so the reported range is a real answer.
const WINDOW_HEIGHT: f32 = 160.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — scrollable")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// The offset of the `Usage` section's vertical area, reported by
    /// `on_scroll`. The Slint component exposes the same numbers as its
    /// `viewport-x` and `viewport-y`.
    usage_x: f32,
    usage_y: f32,
    /// The vertical offset of the `Virtualization` list, read back as a range of
    /// visible rows.
    window_offset: f32,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// The `Usage` area's viewport moved.
    UsageViewport(f32, f32),
    /// The `Virtualization` list's viewport moved.
    WindowViewport(f32),
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
        Message::UsageViewport(x, y) => {
            app.usage_x = x;
            app.usage_y = y;
        }
        Message::WindowViewport(y) => app.window_offset = y,
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

/// The fill and the hairline every row, card and cell in the examples draws.
fn cell_style(theme: &Theme) -> container::Style {
    let tokens = Tokens::of(theme);
    container::Style {
        background: Some(theme::alpha(tokens.muted, 0.35).into()),
        border: iced::border::rounded(theme::RADIUS_MD)
            .color(tokens.border)
            .width(1.0),
        text_color: Some(tokens.foreground),
        ..container::Style::default()
    }
}

/// A column of numbered rows, taller than the box that will hold it.
fn rows(from: usize, to: usize) -> Element<'static, Message> {
    let mut list = column![].spacing(ROW_SPACING);

    for index in from..=to {
        list = list.push(
            container(text(format!("Row {index}")).size(theme::FONT_SIZE_XS))
                .width(Length::Fill)
                .height(Length::Fixed(ROW_HEIGHT))
                .padding([0.0, 8.0])
                .align_y(Alignment::Center)
                .style(cell_style),
        );
    }

    list.into()
}

/// A row of fixed cards, wider than the box that will hold it.
fn cards(count: usize) -> Element<'static, Message> {
    let mut list = row![].spacing(theme::SPACING_SM);

    for index in 1..=count {
        list = list.push(
            container(text(format!("Card {index}")).size(theme::FONT_SIZE_XS))
                .width(Length::Fixed(CARD_WIDTH))
                .height(Length::Fixed(CARD_HEIGHT))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .style(cell_style),
        );
    }

    list.into()
}

/// A grid that overflows both ways, for the `Both` axis.
fn grid(columns: usize, rows: usize) -> Element<'static, Message> {
    let mut list = column![].spacing(theme::SPACING_SM);

    for row_index in 0..rows {
        let mut line = row![].spacing(theme::SPACING_SM);

        for column_index in 0..columns {
            line = line.push(
                container(
                    text(format!("{},{}", row_index + 1, column_index + 1))
                        .size(theme::FONT_SIZE_XS),
                )
                .width(Length::Fixed(CELL_WIDTH))
                .height(Length::Fixed(CELL_HEIGHT))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .style(cell_style),
            );
        }

        list = list.push(line);
    }

    list.into()
}

/// A titled example: a small label over the area it names.
fn sample<'a>(
    title: &'static str,
    area: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![
        text(title)
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
        area.into(),
    ]
    .spacing(6.0)
    .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28 px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
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
    // section: Usage
    // The three axes in one box: a column of rows taller than a fixed box, a row
    // of cards wider than one, and a grid that overflows both ways. The vertical
    // area reports its viewport, which is the offset line below the box.
    let usage = column![
        row![
            sample(
                "Vertical",
                Scrollable::new(rows(1, 20))
                    .width(Length::Fixed(220.0))
                    .height(Length::Fixed(180.0))
                    .on_scroll(|viewport| {
                        let offset = viewport.absolute_offset();
                        Message::UsageViewport(offset.x, offset.y)
                    }),
            ),
            sample(
                "Horizontal",
                Scrollable::new(cards(10))
                    .axis(Axis::Horizontal)
                    .width(Length::Fixed(220.0))
                    .height(Length::Fixed(120.0)),
            ),
            sample(
                "Both",
                Scrollable::new(grid(8, 8))
                    .axis(Axis::Both)
                    .width(Length::Fixed(260.0))
                    .height(Length::Fixed(180.0)),
            ),
        ]
        .spacing(theme::SPACING_LG),
        text(format!(
            "vertical area at x {:.0}, y {:.0}",
            app.usage_x, app.usage_y
        ))
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        }),
    ]
    .spacing(theme::SPACING_MD);

    // section: Custom Scrollbars
    // The bar itself, where iced allows it to be arranged: a thicker bar, a rail
    // embedded with a gap between it and the content, and a thumb drawn from a
    // different token. There is no manual scrollbar widget to attach the way GPUI
    // has one — iced's region owns its bar — so this is the whole custom surface.
    let custom = row![
        sample(
            "Thicker bar",
            Scrollable::new(rows(1, 20))
                .scrollbar_width(18.0)
                .scroller_width(12.0)
                .width(Length::Fixed(200.0))
                .height(Length::Fixed(160.0)),
        ),
        sample(
            "Embedded track",
            Scrollable::new(rows(1, 20))
                .scrollbar_spacing(6.0)
                .track(Ink::Border)
                .thumb(Ink::Primary)
                .width(Length::Fixed(200.0))
                .height(Length::Fixed(160.0)),
        ),
        sample(
            "Foreground thumb",
            Scrollable::new(rows(1, 20))
                .thumb(Ink::Foreground)
                .width(Length::Fixed(200.0))
                .height(Length::Fixed(160.0)),
        ),
    ]
    .spacing(theme::SPACING_LG);

    // section: Virtualization
    // iced 0.14 has no windowed list, so every one of these rows is a real
    // element that is built and drawn; nothing here is virtual. What the
    // component does expose is the viewport, and `on_scroll` reads the visible
    // range out of it. A caller that needs windowing builds it on that number —
    // this port does not fake an API for it.
    let first = (app.window_offset / ROW_PITCH).floor().max(0.0) as usize + 1;
    let visible = (WINDOW_HEIGHT / ROW_PITCH).ceil() as usize;
    let last = (first + visible - 1).min(WINDOW_ROWS);
    let virtualization = column![
        text(format!(
            "rows {first}–{last} of {WINDOW_ROWS} are on screen; all {WINDOW_ROWS} are built"
        ))
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        }),
        Scrollable::new(rows(1, WINDOW_ROWS))
            .height(Length::Fixed(WINDOW_HEIGHT))
            .on_scroll(|viewport| Message::WindowViewport(viewport.absolute_offset().y)),
    ]
    .spacing(theme::SPACING_SM);

    // section: Theme Customization
    // The bar reads the theme's tokens when it is drawn, so these are choices of
    // *which* token, not of colour: a track from the border, a thumb from the
    // foreground, a thumb from the primary, and the two show modes iced can
    // express. iced has no per-component theme JSON to write, which is what the
    // GPUI page configures instead.
    let themed = column![
        row![
            sample(
                "Muted thumb",
                Scrollable::new(rows(1, 20))
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(160.0)),
            ),
            sample(
                "Track and foreground",
                Scrollable::new(rows(1, 20))
                    .track(Ink::Border)
                    .thumb(Ink::Foreground)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(160.0)),
            ),
            sample(
                "Primary thumb",
                Scrollable::new(rows(1, 20))
                    .thumb(Ink::Primary)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(160.0)),
            ),
        ]
        .spacing(theme::SPACING_LG),
        row![
            sample(
                "On hover",
                Scrollable::new(rows(1, 20))
                    .visibility(Visibility::Hover)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(160.0)),
            ),
            sample(
                "Hidden",
                Scrollable::new(rows(1, 20))
                    .visibility(Visibility::Hidden)
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(160.0)),
            ),
        ]
        .spacing(theme::SPACING_LG),
    ]
    .spacing(theme::SPACING_MD);

    // section: end
    let status = text(
        "Scroll any area; the offset line under Usage follows the vertical one.",
    )
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Custom Scrollbars"),
        body_box(custom),
        heading(app, 2, "Virtualization"),
        body_box(virtualization),
        heading(app, 3, "Theme Customization"),
        body_box(themed),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The gallery's own scroll area is this component, so the page's frame is
    // drawn by the thing the page documents.
    Scrollable::new(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
