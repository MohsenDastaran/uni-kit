//! `pagination` — page controls: the labelled and compact step buttons, the run
//! of page numbers with its ellipses, the four sizes and the two boundaries where
//! an arrow has nowhere to go.
//!
//! Run it with `cargo run --example pagination`, or in the browser as
//! `?story=pagination` in the gallery the website embeds.
//!
//! Each section carries the heading the website page gives it — **Basic
//! Pagination**, **Pages**, **Different Sizes**, **Disabled State** — and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use.
//!
//! The `// section: <title>` comments below are the contract the website reads to
//! cut this file into the usage panels a component page shows, so the code on the
//! page is the code that runs here. Keep them, one per section, in order, and keep
//! the closing `// section: end` after the last one.
//!
//! The page is the app's, not the component's, so every pagination here reports a
//! request and the app holds the number that comes back — the Slint `changed`
//! callback and the GPUI `on_click` handler in iced's message form.

use iced::widget::{column, container, mouse_area, row, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::pagination::Pagination;
use crate::components::scrollable::Scrollable;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — pagination")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// One current page per pagination on the page, in the order they appear.
    /// They are the Slint example's `page:` properties, kept as state so a click
    /// has somewhere to land.
    basic_page: usize,
    pages_page: usize,
    xsmall_page: usize,
    medium_page: usize,
    large_page: usize,
    first_page: usize,
    last_page: usize,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            basic_page: 5,
            pages_page: 1,
            xsmall_page: 2,
            medium_page: 3,
            large_page: 4,
            first_page: 1,
            last_page: 5,
            hovered: None,
            pressed_source: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    BasicPage(usize),
    PagesPage(usize),
    XSmallPage(usize),
    MediumPage(usize),
    LargePage(usize),
    FirstPage(usize),
    LastPage(usize),
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
        Message::BasicPage(page) => app.basic_page = page,
        Message::PagesPage(page) => app.pages_page = page,
        Message::XSmallPage(page) => app.xsmall_page = page,
        Message::MediumPage(page) => app.medium_page = page,
        Message::LargePage(page) => app.large_page = page,
        Message::FirstPage(page) => app.first_page = page,
        Message::LastPage(page) => app.last_page = page,
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
    // section: Basic Pagination
    // GPUI's plain `Pagination` labels its step buttons and pairs each label with
    // its arrow: "‹ Previous" and "Next ›". The run stays five wide and carries
    // the current page at its centre.
    let basic = Pagination::new()
        .page(app.basic_page)
        .pages(10)
        .compact(false)
        .on_change(Message::BasicPage);

    // section: Pages
    // The compact default: icon-only step buttons, the run at page one so the
    // trailing ellipsis and the last page are both in view.
    let pages = Pagination::new()
        .page(app.pages_page)
        .pages(12)
        .on_change(Message::PagesPage);

    // section: Different Sizes
    // One control at each of the four tiers. `xsmall` keeps the arrows at the
    // control height the size gives it, so the whole row shrinks with it.
    let sizes = column![
        Pagination::new()
            .page(app.xsmall_page)
            .pages(8)
            .size(ControlSize::XSmall)
            .on_change(Message::XSmallPage),
        Pagination::new()
            .page(app.medium_page)
            .pages(8)
            .size(ControlSize::Medium)
            .on_change(Message::MediumPage),
        Pagination::new()
            .page(app.large_page)
            .pages(8)
            .size(ControlSize::Large)
            .on_change(Message::LargePage),
    ]
    .spacing(theme::SPACING_MD)
    .align_x(Alignment::Start);

    // section: Disabled State
    // The arrow that has nowhere to go is disabled, so a first page cannot step
    // back and a last page cannot step on. The page numbers stay live, which is
    // the Slint component: only the arrow at the boundary is `enabled: false`.
    let disabled = column![
        Pagination::new()
            .page(app.first_page)
            .pages(5)
            .on_change(Message::FirstPage),
        Pagination::new()
            .page(app.last_page)
            .pages(5)
            .on_change(Message::LastPage),
    ]
    .spacing(theme::SPACING_MD)
    .align_x(Alignment::Start);

    // section: end
    let status = text("Click a page number or an arrow — the app holds the page it asks for.")
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        });

    let gallery = column![
        heading(app, 0, "Basic Pagination"),
        body_box(basic),
        heading(app, 1, "Pages"),
        body_box(pages),
        heading(app, 2, "Different Sizes"),
        body_box(sizes),
        heading(app, 3, "Disabled State"),
        body_box(disabled),
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
