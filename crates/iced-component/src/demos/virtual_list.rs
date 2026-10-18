//! `virtual-list` — a windowed list: only the rows on screen are built, on the
//! styled scroll area, with the range the window covers shown above it.
//!
//! Run it with `cargo run --example virtual_list`, or in the browser as
//! `?story=virtual-list` in the gallery the website embeds.
//!
//! Each section carries the heading the website page gives it — **Basic Vertical
//! Virtual List**, **Basic Scroll Control**, **Variable Item Sizes** — and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use.
//!
//! The `// section: <title>` comments below are the contract the website reads to
//! cut this file into the usage panels a component page shows, so the code on the
//! page is the code that runs here. Keep them, one per section, in order, and
//! keep the closing `// section: end` after the last one.
//!
//! The three headings are the ones the Slint example's `GroupBox`es carry, and
//! the page documents each at `###`. The first proves the window: 5000 rows, of
//! which about a dozen are built, and the status line prints the range
//! `VirtualList::range` handed the render function rather than asserting the
//! count. The second is the Slint example's control strip — Top, Row 50,
//! Center 25, Bottom — driving the list with iced's `scroll_to` and reading the
//! range back out. The third is the Slint example's `Variable Item Sizes`, which
//! is one taller fixed row rather than a height per item; this port has the same
//! single-row-height component the Slint file has, and the component's module
//! docs name that limit rather than faking per-item sizes.

use iced::widget::{button, column, container, mouse_area, row, svg, text};
use iced::{Alignment, Border, Color, Element, Length, Shadow, Task, Theme};

use crate::components::button::Button;
use crate::components::group_box::{self, GroupBox};
use crate::components::icon::Icon;
use crate::components::scrollable::Scrollable;
use crate::components::theme::{self, ControlSize, Tokens};
use crate::components::virtual_list::{self, VirtualList};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The basic list: the page's 5000 rows at the Slint `row-height: 32px`, in a
/// 240 px box. Large enough that laying all of them out would be the point.
const BASIC_ROWS: usize = 5000;
const BASIC_HEIGHT: f32 = 240.0;
const ROW_HEIGHT: f32 = 32.0;

/// The control strip's list: the Slint example's 60 rows in a 260 px box.
const SCROLL_ROWS: usize = 60;
const SCROLL_HEIGHT: f32 = 260.0;

/// The taller list: the Slint example's six milestones at `row-height: 44px` in
/// a 200 px box.
const TALL_HEIGHT: f32 = 200.0;
const TALL_ROW_HEIGHT: f32 = 44.0;
const MILESTONES: [&str; 6] = [
    "Milestone: first commit",
    "Milestone: gallery online",
    "Milestone: dock ships",
    "Milestone: menu alignment",
    "Milestone: editor chrome",
    "Milestone: 76 pages",
];

/// The scroll region the control strip drives, so iced's `scroll_to` can name it.
const SCROLL_ID: &str = "virtual-list-control";

/// An up arrow, the Lucide `arrow-up` glyph the Slint buttons carry.
const ARROW_UP: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m5 12 7-7 7 7"/><path d="M12 19V5"/></svg>"#;

/// A down arrow, the Lucide `arrow-down` glyph.
const ARROW_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14"/><path d="m19 12-7 7-7-7"/></svg>"#;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — virtual-list")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// The scroll offset of each list, read from its `Viewport`. The window is
    /// computed from it, so this is the state that makes the list window at all;
    /// the starting selections are the Slint example's own `selected:` values.
    basic_offset: f32,
    basic_selected: Option<usize>,
    scroll_offset: f32,
    scroll_selected: Option<usize>,
    tall_offset: f32,
    tall_selected: Option<usize>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            basic_offset: 0.0,
            basic_selected: Some(2),
            scroll_offset: 0.0,
            scroll_selected: Some(4),
            tall_offset: 0.0,
            tall_selected: Some(0),
            hovered: None,
            pressed_source: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    BasicViewport(f32),
    SelectBasic(usize),
    ScrollViewport(f32),
    SelectScroll(usize),
    /// A control-strip button was pressed: move the region to this offset. The
    /// app writes the offset into its own state as well as issuing iced's
    /// `scroll_to`, so the window follows the press immediately instead of on the
    /// next viewport report.
    ScrollTo(f32),
    TallViewport(f32),
    SelectTall(usize),
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
        Message::BasicViewport(y) => app.basic_offset = y,
        Message::SelectBasic(index) => app.basic_selected = Some(index),
        Message::ScrollViewport(y) => app.scroll_offset = y,
        Message::SelectScroll(index) => app.scroll_selected = Some(index),
        Message::ScrollTo(offset) => {
            app.scroll_offset = offset;
            // The Slint component's `scroll-to`: iced scrolls a named region by
            // operation, and the region answers through `on_scroll` afterwards.
            return iced::widget::operation::scroll_to(
                SCROLL_ID,
                iced::widget::operation::AbsoluteOffset {
                    x: None,
                    y: Some(offset),
                },
            );
        }
        Message::TallViewport(y) => app.tall_offset = y,
        Message::SelectTall(index) => app.tall_selected = Some(index),
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

/// One row: the Slint component's `Rectangle`, its label inset 12px, the even
/// rows tinted `muted` a quarter of the way, and the selected row filled with
/// `accent`. It is a button because a row is the pointer target, and the hover
/// tint is the `list` component's own half-accent.
fn striped_row<'a>(
    index: usize,
    label: String,
    selected: bool,
    select: Message,
) -> Element<'a, Message> {
    let striped = index % 2 == 0;

    button(
        container(text(label).size(theme::FONT_SIZE_SM))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([0.0, 12.0])
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(0)
    .on_press(select)
    .style(move |theme: &Theme, status| row_style(theme, status, striped, selected))
    .into()
}

/// The row's surface, resolved inside the style closure so the dark theme is
/// dark: selected first, then the pointer, then the Slint `mod(index, 2)` stripe.
fn row_style(theme: &Theme, status: button::Status, striped: bool, selected: bool) -> button::Style {
    let tokens = Tokens::of(theme);
    let background = if selected {
        tokens.accent
    } else if matches!(status, button::Status::Hovered | button::Status::Pressed) {
        theme::alpha(tokens.accent, 0.5)
    } else if striped {
        theme::alpha(tokens.muted, 0.25)
    } else {
        Color::TRANSPARENT
    };

    button::Style {
        background: (background.a > 0.0).then_some(background.into()),
        text_color: tokens.foreground,
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    }
}

/// A muted line of state under a list — the range the window covers, which is the
/// windowing made visible rather than asserted.
fn range_line<'a>(label: String) -> Element<'a, Message> {
    text(label)
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// An arrow at the icon box's size, tinted by the button it sits in.
fn arrow<'a>(handle: &'static [u8]) -> Element<'a, Message> {
    Icon::new(svg::Handle::from_memory(handle))
        .size(ControlSize::Small)
        .into()
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
    // section: Basic Vertical Virtual List
    // 5000 rows, every one a real index the render function can be asked for, and
    // about a dozen of them built. The list owns its 240px height so it knows how
    // many rows fill it, the app holds the offset it reports, and the window is
    // the rows that offset covers. The line above the list is `VirtualList::range`
    // read back, which is the same range the render function was called for.
    let basic = VirtualList::new(BASIC_ROWS, |index, selected| {
        striped_row(
            index,
            format!("Row {}", index + 1),
            selected,
            Message::SelectBasic(index),
        )
    })
    .row_height(ROW_HEIGHT)
    .height(BASIC_HEIGHT)
    .offset(app.basic_offset)
    .selected(app.basic_selected)
    .on_scroll(|viewport| Message::BasicViewport(viewport.absolute_offset().y));
    let basic_range = basic.range();
    let basic_status = range_line(format!(
        "rows {}–{} of {} built ({})",
        basic_range.start + 1,
        basic_range.end,
        BASIC_ROWS,
        basic_range.end - basic_range.start
    ));
    let basic_section = column![basic_status, basic]
        .spacing(theme::SPACING_SM)
        .width(Length::Fill);

    // section: Basic Scroll Control
    // The Slint example's control strip. Each button asks iced to scroll the named
    // region — `scroll-to(0)`, `scroll-to-row(49)`, `scroll-to-row(24, center)`,
    // `scroll-to-row(59)` — and the line under it reports the range the region
    // came back with, the Slint `first-visible`..`last-visible`.
    let scroll = VirtualList::new(SCROLL_ROWS, |index, selected| {
        striped_row(
            index,
            format!("Row {}", index + 1),
            selected,
            Message::SelectScroll(index),
        )
    })
    .row_height(ROW_HEIGHT)
    .height(SCROLL_HEIGHT)
    .offset(app.scroll_offset)
    .selected(app.scroll_selected)
    .id(SCROLL_ID)
    .on_scroll(|viewport| Message::ScrollViewport(viewport.absolute_offset().y));
    let scroll_range = scroll.range();
    let scroll_controls = row![
        Button::new("Top")
            .icon(arrow(ARROW_UP))
            .on_press(Message::ScrollTo(0.0)),
        Button::new("Row 50").on_press(Message::ScrollTo(virtual_list::row_offset(
            49,
            SCROLL_ROWS,
            ROW_HEIGHT,
            SCROLL_HEIGHT,
        ))),
        Button::new("Center 25").on_press(Message::ScrollTo(virtual_list::centered_row_offset(
            24,
            SCROLL_ROWS,
            ROW_HEIGHT,
            SCROLL_HEIGHT,
        ))),
        Button::new("Bottom")
            .icon(arrow(ARROW_DOWN))
            .on_press(Message::ScrollTo(virtual_list::row_offset(
                59,
                SCROLL_ROWS,
                ROW_HEIGHT,
                SCROLL_HEIGHT,
            ))),
    ]
    .spacing(theme::SPACING_SM);
    let scroll_status = range_line(format!(
        "visible: {}..{} of {}",
        scroll_range.start, scroll_range.end, SCROLL_ROWS
    ));
    let scroll_section = column![scroll_controls, scroll_status, scroll]
        .spacing(theme::SPACING_SM)
        .width(Length::Fill);

    // section: Variable Item Sizes
    // The Slint example's third list: one taller row, a 44px milestone band,
    // rather than a size per item. This port has the same single-row-height
    // component the Slint file has, so this is what that heading draws; the GPUI
    // original's per-item sizes are named in the component's module docs instead
    // of a uniform band that pretends to be one.
    let tall = VirtualList::new(MILESTONES.len(), |index, selected| {
        striped_row(
            index,
            MILESTONES[index].to_string(),
            selected,
            Message::SelectTall(index),
        )
    })
    .row_height(TALL_ROW_HEIGHT)
    .height(TALL_HEIGHT)
    .offset(app.tall_offset)
    .selected(app.tall_selected)
    .on_scroll(|viewport| Message::TallViewport(viewport.absolute_offset().y));

    // section: end
    let status =
        text("Click a row to select it — press a heading or its code icon for that sample.")
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            });

    let gallery = column![
        heading(app, 0, "Basic Vertical Virtual List"),
        body_box(basic_section),
        heading(app, 1, "Basic Scroll Control"),
        body_box(scroll_section),
        heading(app, 2, "Variable Item Sizes"),
        body_box(tall),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The gallery's own scroll area is the component the page documents, so the
    // frame is drawn by the thing under test.
    Scrollable::new(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
