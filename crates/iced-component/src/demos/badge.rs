//! `badge` — a count, a dot or an icon anchored to the corner of its child.
//!
//! Run it with `cargo run --example badge`, or in the browser as
//! `?story=badge` in the gallery the website embeds.

use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};

use crate::components::badge::Badge;
use crate::components::button::{Button, Variant};
use crate::components::theme::{ControlSize, Tokens};

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — badge")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Clear,
}

fn boot() -> u32 {
    3
}

fn update(count: &mut u32, message: Message) -> Task<Message> {
    match message {
        Message::Increment => *count += 1,
        Message::Clear => *count = 0,
    }
    Task::none()
}

/// A stand-in for the icon or avatar the badge anchors to. The icon set belongs
/// to the application, so the child here is a plain box and the badge's own
/// icon is an element the caller passes.
fn tile<'a>(label: &str, size: f32) -> Element<'a, Message> {
    container(
        text(label.to_string())
            .size(12.0)
            .style(|theme: &iced::Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    )
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .center(size)
    .style(|theme: &iced::Theme| container::Style {
        background: Some(Tokens::of(theme).muted.into()),
        border: iced::Border {
            radius: crate::components::theme::RADIUS_MD.into(),
            ..iced::Border::default()
        },
        ..container::Style::default()
    })
    .into()
}

fn view(count: &u32) -> Element<'_, Message> {
    column![
        row![
            Badge::new(tile("inbox", 32.0)).count(*count),
            Badge::new(tile("bell", 32.0)).count(*count).max(9),
            Badge::new(tile("bell", 32.0)).count(*count).max(9).size(ControlSize::Large),
            Badge::new(tile("bell", 32.0)).count(*count).size(ControlSize::Small),
        ]
        .spacing(16),
        row![
            Badge::new(tile("dot", 32.0)).dot().count(1),
            Badge::new(tile("icon", 32.0))
                .icon(
                    container(text("✓").size(12.0).style(|_: &iced::Theme| text::Style {
                        color: Some(iced::Color::WHITE),
                    }))
                    .center(14.0)
                )
                .inner_offset(2.0),
        ]
        .spacing(16),
        row![
            Button::new("Unread +1")
                .variant(Variant::Primary)
                .on_press(Message::Increment),
            Button::new("Clear").on_press(Message::Clear),
        ]
        .spacing(8),
    ]
    .spacing(16)
    .padding(16)
    .into()
}
