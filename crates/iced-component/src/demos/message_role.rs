//! `message-role` — the two sides of a conversation turn, and the marker each
//! one shows.
//!
//! Run it with `cargo run --example message_role`, or in the browser as
//! `?story=message_role` in the gallery the website embeds.
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
//! iced's message-role is the marker a turn shows for its side, and nothing
//! else: the disc, the initials it falls back to, and the one state
//! `message.slint` tints. The headings this demo and the page share are the ones
//! it can actually fill: **Usage**, **Custom Initials**, **Failed State** and
//! **Alignment**.

use iced::widget::{button, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::message_role::{MessageRole, Role};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — message-role")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// Whether the sent turn failed, which is the one state the marker itself
    /// reports: message.slint tints a failed sent turn destructively.
    failed: bool,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// Flips the sent turn between delivered and failed.
    ToggleFailed,
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
        Message::ToggleFailed => app.failed = !app.failed,
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
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon is
/// the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and 0.55
/// on hover, 0.34 and 0.7 while pressed.
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
/// example wraps its sections in, untitled because the heading above it carries the
/// title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Usage
    // The two roles, each with the initials message.slint falls back to when the
    // caller names none: "YO" for the sent side, "AL" for the received one.
    let usage = row![
        MessageRole::new(Role::User),
        MessageRole::new(Role::Assistant),
    ]
    .spacing(theme::SPACING_SM);

    // section: Custom Initials
    // The caller's own initials replace the fallback; the role still picks the
    // fill and the colour the initials are drawn in.
    let custom = row![
        MessageRole::new(Role::User).initials("MS"),
        MessageRole::new(Role::Assistant).initials("ST"),
    ]
    .spacing(theme::SPACING_SM);

    // section: Failed State
    // The failed state. It tints the sent turn destructive and leaves the
    // received one muted, which is what message.slint does with its `failed`
    // flag. The button flips the sent marker.
    let failed = column![
        row![
            MessageRole::new(Role::User).failed(app.failed),
            MessageRole::new(Role::Assistant).failed(true),
        ]
        .spacing(theme::SPACING_SM),
        button(text(if app.failed {
            "Sent turn: failed"
        } else {
            "Sent turn: delivered"
        }))
        .on_press(Message::ToggleFailed),
    ]
    .spacing(theme::SPACING_SM);

    // section: Alignment
    // The side each role sits on, which is what Role::alignment carries: the
    // received turn at the leading edge, the sent one at the trailing edge.
    let alignment = column![
        row![
            text("received").size(theme::FONT_SIZE_XS),
            container(MessageRole::new(Role::Assistant))
                .width(Length::Fill)
                .align_x(Role::Assistant.alignment()),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(Alignment::Center),
        row![
            container(MessageRole::new(Role::User))
                .width(Length::Fill)
                .align_x(Role::User.alignment()),
            text("sent").size(theme::FONT_SIZE_XS),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(Alignment::Center),
    ]
    .spacing(theme::SPACING_MD);

    // section: end
    let status = text("Press a heading or its code icon — the page scrolls to that sample.");

    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Custom Initials"),
        body_box(custom),
        heading(app, 2, "Failed State"),
        body_box(failed),
        heading(app, 3, "Alignment"),
        body_box(alignment),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
