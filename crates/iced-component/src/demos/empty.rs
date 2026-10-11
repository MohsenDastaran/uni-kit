//! `empty` — an empty-state panel with a media slot, a title, a description and
//! up to two actions.
//!
//! Run it with `cargo run --example empty`, or in the browser as
//! `?story=empty` in the gallery the website embeds.

use iced::widget::{column, container, text};
use iced::{Element, Task};

use crate::components::button::{Button, Variant};
use crate::components::empty::{Align, Empty, Media};
use crate::components::theme::Tokens;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — empty")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    archived: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Create,
    Import,
    Archive,
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Archive => app.archived = true,
        Message::Create | Message::Import => {}
    }
    Task::none()
}

/// The icon set belongs to the application, so the media slot takes an element.
fn glyph(glyph: &str) -> Element<'static, Message> {
    container(text(glyph.to_string()).size(16.0).style(|theme: &iced::Theme| text::Style {
        color: Some(Tokens::of(theme).foreground),
    }))
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    let action = if app.archived {
        "Unarchive"
    } else {
        "Archive"
    };

    column![
        Empty::new()
            .media_slot(glyph("📥"))
            .title("No projects yet")
            .description(
                "Projects hold the work a team shares. Create one to get started, or import an existing repository.",
            )
            .action(
                Button::new("New project")
                    .variant(Variant::Primary)
                    .on_press(Message::Create),
            )
            .secondary_action(Button::new("Import").outline().on_press(Message::Import))
            .bordered(true),
        Empty::new()
            .media(Media::None)
            .align(Align::Start)
            .title("Nothing archived")
            .description("Items you archive stay here until you delete them.")
            .action(Button::new(action).on_press(Message::Archive)),
    ]
    .spacing(16)
    .padding(16)
    .into()
}
