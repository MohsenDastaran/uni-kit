//! `hover-card` — a delayed card of rich content over whatever the pointer rests
//! on.
//!
//! Run it with `cargo run --example hover_card`, or in the browser as
//! `?story=hover-card` in the gallery the website embeds.
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
//! The four headings are the Slint example's GroupBox titles, all of which the
//! component page already has: **Hover**, **User Profile Preview**, **Custom
//! Timing** and **Positioning**. The rest of `hover-card.md` describes GPUI's
//! `content` closure, its separate open and close delays and its `anchor`
//! property, none of which the Slint component exposes; the component names
//! those gaps in its own module rather than faking them here.

use iced::time::Duration;
use iced::widget::{column, container, row, scrollable, svg, text, Space};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::{Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::hover_card::HoverCard;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — hover-card")
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

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = iced::widget::mouse_area(
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

    let icon = iced::widget::mouse_area(
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

/// A link-styled trigger, the shape the Slint example's host buttons have.
fn trigger<'a>(label: &'static str) -> Button<'a, Message> {
    Button::new(label).variant(Variant::Link)
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Hover
    // The whole component: a trigger, a title and a description. The card opens
    // once the pointer has rested on the trigger for the delay.
    let hover = HoverCard::new(trigger("@ada"))
        .title("Ada Lovelace")
        .description("Mathematician and the first computer programmer.");

    // section: User Profile Preview
    // A card can open over any hoverable host, and its content is the caller's
    // title and description, so the same component stands in for a profile and
    // for a channel.
    let profiles = row![
        HoverCard::new(trigger("Open profile"))
            .title("grace@example.com")
            .description("Maintains the Slint port. Reviewed 42 changes this month."),
        HoverCard::new(trigger("Join channel"))
            .title("#design")
            .description("12 members. Last message 4 minutes ago."),
    ]
    .spacing(theme::SPACING_LG)
    .align_y(Alignment::Center);

    // section: Custom Timing
    // `delay` is the only timing control Slint has: zero opens as soon as the
    // pointer arrives, and a longer delay lets a passing pointer go by.
    let timing = column![
        HoverCard::new(trigger("No delay"))
            .title("Immediate")
            .description("Opens as soon as the pointer rests.")
            .delay(Duration::ZERO),
        HoverCard::new(trigger("600 ms"))
            .title("Deliberate")
            .description("Waits before opening, so a passing pointer does not flash it.")
            .delay(Duration::from_millis(600)),
    ]
    .spacing(theme::SPACING_LG)
    .align_x(Alignment::Start);

    // section: Positioning
    // The card follows its host. The shared placement rule puts it under the
    // trigger and flips it above when the frame has no room below.
    let positioning = row![
        HoverCard::new(trigger("Left"))
            .title("Start")
            .description("Anchored to this trigger."),
        Space::new().width(Length::Fill),
        HoverCard::new(trigger("Right"))
            .title("End")
            .description("Anchored to the trailing trigger."),
    ]
    .width(Length::Fill)
    .align_y(Alignment::Center);

    // section: end
    let status = text("Press a heading or its code icon — the page scrolls to that sample.");

    let gallery = column![
        heading(app, 0, "Hover"),
        body_box(hover),
        heading(app, 1, "User Profile Preview"),
        body_box(profiles),
        heading(app, 2, "Custom Timing"),
        body_box(timing),
        heading(app, 3, "Positioning"),
        body_box(positioning),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
