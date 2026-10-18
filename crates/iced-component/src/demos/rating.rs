//! `rating` — the whole gallery the Slint example shows: a live row with its
//! value, the three sizes, a read-only row and a custom maximum.
//!
//! Run it with `cargo run --example rating`, or in the browser as
//! `?story=rating` in the gallery the website embeds.
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
//! The four headings are the Slint example's GroupBox titles. The page's other
//! examples — a controlled rating in a view, the click-behaviour rules and a
//! custom colour — are compositions or GPUI-only properties; the click rule is
//! documented on the component instead of faked here.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::rating::Rating;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — rating")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// The row is controlled: the owner holds the value and every press comes
    /// back through `on_change`, which is how iced spells an in-out property.
    rating: usize,
    /// The star the pointer is over, as `on_hover` reported it; `None` between
    /// the stars and after the pointer leaves the row.
    hovered: Option<usize>,
    /// The last value a press asked for, so a click is visible.
    last: Option<usize>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered_section: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A star was pressed and asked for this value.
    Rate(usize),
    /// The pointer entered a star, or left the row.
    Hover(Option<usize>),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot() -> App {
    App {
        rating: 4,
        hovered: None,
        last: None,
        hovered_section: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Rate(value) => {
            app.rating = value;
            app.last = Some(value);
        }
        Message::Hover(value) => app.hovered = value,
        Message::HoverSource(title) => app.hovered_section = title,
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
    let hovered = app.hovered_section == Some(title);
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

fn view(app: &App) -> Element<'_, Message> {
    // section: Rate this
    // The value is the owner's; a press comes back as the value it asked for.
    // The hover is the same round trip: `on_hover` reports the star, `hovered`
    // takes it back, and the stars up to it preview in the active colour.
    let rate_this = column![
        Rating::new(app.rating)
            .hovered(app.hovered)
            .on_change(Message::Rate)
            .on_hover(Message::Hover),
        text(format!("{} of 5", app.rating))
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    ]
    .spacing(8.0);

    // section: Different Sizes
    // A row with no message is read-only: the stars draw, nothing is reported.
    let sizes = column![
        Rating::new(3).size(ControlSize::Small),
        Rating::new(3),
        Rating::new(4).size(ControlSize::Large),
    ]
    .spacing(8.0);

    // section: Disabled State
    let disabled = Rating::new(2);

    // section: Custom Maximum
    let custom_max = Rating::new(7).max(10);

    // section: end
    let status = text(match app.last {
        Some(value) => format!("Last press: {value} of 5 — the owner wrote the value back."),
        None => "Pick a star — clicking a filled star clears it.".to_string(),
    });

    let gallery = column![
        heading(app, 0, "Rate this"),
        body_box(rate_this),
        heading(app, 1, "Different Sizes"),
        body_box(sizes),
        heading(app, 2, "Disabled State"),
        body_box(disabled),
        heading(app, 3, "Custom Maximum"),
        body_box(custom_max),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
