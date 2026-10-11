//! `tag` — a chip in four variants, with a leading icon and a close control.
//!
//! Run it with `cargo run --example tag`, or in the browser as
//! `?story=tag` in the gallery the website embeds.

use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};

use crate::components::tag::{Tag, Variant};
use crate::components::theme::{ControlSize, Status, Tokens};

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — tag")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    removed: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Close,
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Close => app.removed += 1,
    }
    Task::none()
}

/// The icon set belongs to the application, so the leading element is the
/// caller's.
fn glyph<'a>(glyph: &str) -> Element<'a, Message> {
    container(text(glyph.to_string()).size(12.0).style(|theme: &iced::Theme| text::Style {
        color: Some(Tokens::of(theme).foreground),
    }))
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    column![
        row![
            Tag::new("Default"),
            Tag::new("Secondary").variant(Variant::Secondary),
            Tag::new("Outline").variant(Variant::Outline),
        ]
        .spacing(8),
        row![
            Tag::new("Info").status(Status::Info),
            Tag::new("Success").status(Status::Success),
            Tag::new("Warning").status(Status::Warning),
            Tag::new("Danger").status(Status::Danger),
        ]
        .spacing(8),
        row![
            Tag::new("With icon").icon(glyph("★")),
            Tag::new("Closable").on_close(Message::Close),
            Tag::new("Both").icon(glyph("◆")).on_close(Message::Close),
        ]
        .spacing(8),
        row![
            Tag::new("Small").size(ControlSize::Small),
            Tag::new("Medium"),
            Tag::new("Large").size(ControlSize::Large),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center),
        text(format!("closed {} times", app.removed)).size(14.0),
        // A tag is as wide as its content, so a caller that wants it beside
        // something else places it in a row rather than filling a column.
        row![Tag::new("Placed"), text("beside a label").size(14.0)]
            .spacing(8)
            .align_y(iced::Alignment::Center)
            .height(Length::Shrink),
    ]
    .spacing(12)
    .padding(16)
    .into()
}
