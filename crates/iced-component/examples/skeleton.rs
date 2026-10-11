//! `skeleton` — a static placeholder block at a pulse the caller advances.
//!
//! iced's timer needs a runtime feature this crate does not enable, so the
//! pulse is a value instead: zero is the dimmest the Slint animation reaches,
//! one is the resting block. The buttons stand in for the subscription an
//! application would run.
//!
//! Run it with `cargo run --example skeleton`.

use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};

use iced_component::components::button::{Button, Variant};
use iced_component::components::skeleton::Skeleton;
use iced_component::components::theme::Tokens;

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — skeleton")
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    pulse: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Dim,
    Brighten,
}

fn boot() -> App {
    App { pulse: 1.0 }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Dim => app.pulse = (app.pulse - 0.25).max(0.0),
        Message::Brighten => app.pulse = (app.pulse + 0.25).min(1.0),
    }
    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    let pulse = app.pulse;

    // The skeleton blocks carry no message, so they are pushed into a column
    // whose type the controls have already fixed.
    let mut preview = column![].spacing(12);
    preview = preview.push(
        row![
            Skeleton::new()
                .width(Length::Fixed(48.0))
                .height(Length::Fixed(48.0))
                .radius(24.0),
            column![
                Skeleton::new()
                    .width(Length::Fixed(120.0))
                    .height(Length::Fixed(16.0)),
                Skeleton::new()
                    .width(Length::Fixed(100.0))
                    .height(Length::Fixed(12.0)),
            ]
            .spacing(8),
        ]
        .spacing(12),
    );
    preview = preview.push(Skeleton::new().height(Length::Fixed(16.0)));
    preview = preview.push(
        Skeleton::new()
            .width(Length::Fixed(200.0))
            .height(Length::Fixed(16.0)),
    );

    column![
        row![
            Button::new("Dimmer").on_press(Message::Dim),
            Button::new("Brighter")
                .variant(Variant::Primary)
                .on_press(Message::Brighten),
            text(format!("pulse {:.2}", pulse))
                .size(14.0)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
        // A card preview: an avatar, a name and an email line.
        container(preview).padding(16).style(|theme: &iced::Theme| container::Style {
            border: iced::Border {
                color: Tokens::of(theme).border,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..container::Style::default()
        }),
    ]
    .spacing(16)
    .padding(16)
    .into()
}
