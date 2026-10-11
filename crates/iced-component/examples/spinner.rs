//! `spinner` — a rotating arc at a progress the caller advances.
//!
//! iced's timer needs a runtime feature this crate does not enable, so the
//! spinner takes the progress instead: one unit is one full turn. The buttons
//! stand in for the subscription an application would run, and without one the
//! spinner still draws its whole shape.
//!
//! Run it with `cargo run --example spinner`.

use iced::widget::{column, row, text};
use iced::{Element, Task};

use iced_component::components::button::{Button, Variant};
use iced_component::components::spinner::Spinner;
use iced_component::components::theme::{ControlSize, Tokens};

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — spinner")
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    progress: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Step,
    Reset,
}

fn boot() -> App {
    App { progress: 0.0 }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Step => {
            app.progress += 0.125;
            if app.progress > 1.0 {
                app.progress = 0.0;
            }
        }
        Message::Reset => app.progress = 0.0,
    }
    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    let progress = app.progress;

    column![
        row![
            Button::new("Turn an eighth")
                .variant(Variant::Primary)
                .on_press(Message::Step),
            Button::new("Reset").on_press(Message::Reset),
            text(format!("{:.0}°", progress * 360.0))
                .size(14.0)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
        text("12, 14, 16 and 24 pixels across.").size(14.0),
        row![
            Spinner::new().size(ControlSize::XSmall).progress(progress),
            Spinner::new().size(ControlSize::Small).progress(progress),
            Spinner::new().progress(progress),
            Spinner::new().size(ControlSize::Large).progress(progress),
            Spinner::new().diameter(48.0).progress(progress),
        ]
        .spacing(16)
        .align_y(iced::Alignment::Center),
        text("The colour is the caller's, and the sweep is the arc's share of the ring.")
            .size(14.0),
        row![
            Spinner::new()
                .progress(progress)
                .color(iced::Color::from_rgb8(0xef, 0x44, 0x44)),
            Spinner::new()
                .progress(progress)
                .sweep(0.25)
                .color(iced::Color::from_rgb8(0x22, 0xc5, 0x5e)),
            Spinner::new().progress(progress).sweep(1.0),
        ]
        .spacing(16)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(12)
    .padding(16)
    .into()
}
