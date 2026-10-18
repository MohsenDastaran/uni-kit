//! `skeleton` — a loading placeholder that pulses from the frame's own clock.
//!
//! The three samples are the Slint example's `GroupBox`es in its order: a
//! loading card, a circular avatar placeholder, and the different shapes. Every
//! block animates on its own; nothing here advances a value, drives a
//! subscription or waits for a button, because the component samples each
//! frame's instant and asks for the next frame.
//!
//! Run it with `cargo run --example skeleton`, or in the browser as
//! `?story=skeleton` in the gallery the website embeds.
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
//! The sections are the Slint example's samples mapped onto the headings
//! `website/component/skeleton.md` already documents: **Loading**, **Circle
//! Skeleton** and **Different Shapes**. No heading is invented for the site.

use crate::components::group_box::{self, GroupBox};
use crate::components::skeleton::Skeleton;
use crate::components::theme::{self, Tokens};
use iced::font::Weight;
use iced::widget::{Space, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

/// The page padding and the gap between sections, as the Slint example sets
/// them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — skeleton")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
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

// ------------------------------------------------------------------- pieces

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon is
/// the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and 0.55
/// on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(text(title).size(theme::FONT_SIZE_SM).font(iced::Font {
            weight: Weight::Medium,
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

// --------------------------------------------------------------------- view

fn view(app: &App) -> Element<'_, Message> {
    // section: Loading
    // The Slint box: two text lines and a tall block. The second line is 70% of
    // the box, which the Slint example writes as `width: 70%`; a row with a
    // three-part spacer says the same thing in iced's lengths.
    let loading = column![
        Skeleton::new().height(Length::Fixed(16.0)),
        row![
            Skeleton::new()
                .width(Length::FillPortion(7))
                .height(Length::Fixed(16.0)),
            Space::new().width(Length::FillPortion(3)),
        ],
        Skeleton::new().height(Length::Fixed(80.0)),
    ]
    .spacing(16.0);

    // section: Circle Skeleton
    // A 48px avatar placeholder, round because its radius is half its height.
    let circle = Skeleton::new()
        .width(Length::Fixed(48.0))
        .height(Length::Fixed(48.0))
        .radius(24.0);

    // section: Different Shapes
    // Slint's three: a 12px text line with a 2px radius, a 160x40 block at 8px,
    // and the 48px circle again, which is where the radius rules meet.
    let shapes = column![
        Skeleton::new().height(Length::Fixed(12.0)).radius(2.0),
        row![
            Skeleton::new()
                .width(Length::Fixed(160.0))
                .height(Length::Fixed(40.0))
                .radius(8.0),
            Skeleton::new()
                .width(Length::Fixed(48.0))
                .height(Length::Fixed(48.0))
                .radius(24.0),
        ]
        .spacing(16.0),
    ]
    .spacing(16.0);

    // section: end
    let status = text(
        "The pulse runs from the frame's own instant: 1400ms per cycle, \
         opacity 0.55 to 0.90.",
    )
    .size(theme::FONT_SIZE_SM)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, "Loading"),
        body_box(loading),
        heading(app, 1, "Circle Skeleton"),
        body_box(circle),
        heading(app, 2, "Different Shapes"),
        body_box(shapes),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The `</>` mark, taken from the icon set this repository already ships
// (`crates/slint-component/ui/icons`, Lucide, ISC), so the two galleries draw
// the same glyph rather than two lookalikes.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
