//! `shimmer` — a moving highlight over a muted band, at a value the caller
//! advances.
//!
//! iced cannot paint a gradient through part of a text run, so the component
//! draws the band and returns the label for the caller to place over it. The
//! buttons stand in for the subscription an application would run.
//!
//! Run it with `cargo run --example shimmer`, or in the browser as
//! `?story=shimmer` in the gallery the website embeds.

use iced::widget::{column, row, stack, text};
use iced::{Element, Length, Task};

use crate::components::button::{Button, Variant};
use crate::components::shimmer::{self, Shimmer};
use crate::components::theme::Tokens;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — shimmer")
        .theme(theme)
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
            app.progress += 0.05;
            if app.progress > 1.0 {
                app.progress = 0.0;
            }
        }
        Message::Reset => app.progress = 0.0,
    }
    Task::none()
}

/// A shimmer band with its label laid over it, which is how a caller gets the
/// text and the highlight in the same place.
fn line<'a>(label: &str, progress: f32, reverse: bool, spread: f32) -> Element<'a, Message> {
    stack![
        Shimmer::new(progress).spread(spread).reverse(reverse),
        iced::widget::container(shimmer::label(label))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y(Length::Fill),
    ]
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    let progress = app.progress;

    column![
        row![
            Button::new("Advance")
                .variant(Variant::Primary)
                .on_press(Message::Step),
            Button::new("Reset").on_press(Message::Reset),
            text(format!("{:.0}%", progress * 100.0))
                .size(14.0)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
        line("Loading your workspace…", progress, false, 0.3),
        line("A narrower highlight", progress, false, 0.1),
        line("Moving the other way", progress, true, 0.3),
        text("The band waits at the edges while the highlight is outside it.")
            .size(14.0),
    ]
    .spacing(12)
    .padding(16)
    .into()
}
