//! `card` — a bordered surface with a badge, a title, a description and a body.
//!
//! Run it with `cargo run --example card`, or in the browser as
//! `?story=card` in the gallery the website embeds.

use iced::widget::{column, row, text};
use iced::{Element, Task};

use crate::components::button::{Button, Variant};
use crate::components::card::{self, Card};
use crate::components::label::Label;
use crate::components::theme::{ControlSize, Tokens};

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — card")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    saved: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Save,
    Nothing,
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Save => app.saved += 1,
        Message::Nothing => {}
    }
    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    let body = column![
        Label::new("Payment method").required(true),
        text("Visa ending in 4242")
            .size(14.0)
            .style(|theme: &iced::Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    ]
    .spacing(8);

    let footer = row![
        Button::new("Cancel")
            .variant(Variant::Ghost)
            .on_press(Message::Nothing),
        Button::new("Save")
            .variant(Variant::Primary)
            .on_press(Message::Save),
    ]
    .spacing(8);

    column![
        Card::new()
            .badge("New")
            .title("Confirm your plan")
            .description("You are about to move to the team plan. It starts today and renews monthly.")
            .body(body),
        Card::new()
            .title("Invoices")
            .description("Every invoice for this workspace.")
            .size(ControlSize::Small)
            .body(card::footer(footer)),
        text(format!("saved {} times", app.saved)),
    ]
    .spacing(16)
    .padding(16)
    .into()
}
