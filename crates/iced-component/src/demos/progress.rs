//! `progress` — a linear bar and a circle, at a value the caller advances.
//!
//! iced's timer needs a runtime feature this crate does not enable, so the
//! component takes the value instead. The buttons below stand in for the
//! subscription an application would run: each press moves the value, and the
//! loading forms use the same value as their phase.
//!
//! Run it with `cargo run --example progress`, or in the browser as
//! `?story=progress` in the gallery the website embeds.

use iced::widget::{column, row, text};
use iced::{Element, Task};

use crate::components::button::{Button, Variant};
use crate::components::progress::{Progress, ProgressCircle};
use crate::components::theme::{ControlSize, Status, Tokens};

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — progress")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    value: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Step,
    Reset,
}

fn boot() -> App {
    App { value: 0.35 }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Step => {
            app.value += 0.05;
            if app.value > 1.0 {
                app.value = 0.0;
            }
        }
        Message::Reset => app.value = 0.0,
    }
    Task::none()
}

fn caption(value: f32) -> iced::widget::Text<'static, iced::Theme, iced::Renderer> {
    text(format!("{:.0}%", value * 100.0))
        .size(14.0)
        .style(|theme: &iced::Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
}

fn view(app: &App) -> Element<'_, Message> {
    let value = app.value;

    // section: Progress
    // The buttons stand in for the subscription an application would run: each
    // press moves the value the bar and the loading form read.
    let progress = column![
        row![
            Button::new("Advance")
                .variant(Variant::Primary)
                .on_press(Message::Step),
            Button::new("Reset").on_press(Message::Reset),
            caption(value),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
        Progress::new(value),
        text("The loading form: the value is the phase of the band.").size(14.0),
        Progress::new(value).loading(true),
    ]
    .spacing(12);

    // section: Values
    let values = column![
        Progress::new(0.0),
        Progress::new(0.25),
        Progress::new(0.5),
        Progress::new(0.75),
        Progress::new(1.0),
    ]
    .spacing(12);

    // section: Sizes and status colors
    let sizes = column![
        text("The four heights: 4, 6, 8 and 10 pixels.").size(14.0),
        Progress::new(value).size(ControlSize::XSmall),
        Progress::new(value).size(ControlSize::Small),
        Progress::new(value),
        Progress::new(value).size(ControlSize::Large),
        text("Two of the status colours.").size(14.0),
        Progress::new(value).status(Status::Success),
        Progress::new(value).status(Status::Danger),
    ]
    .spacing(12);

    // section: Progress circle
    let circles = column![
        text("The circle, at 8, 12, 16 and 20 across.").size(14.0),
        row![
            ProgressCircle::new(value).size(ControlSize::XSmall),
            ProgressCircle::new(value).size(ControlSize::Small),
            ProgressCircle::new(value),
            ProgressCircle::new(value).size(ControlSize::Large),
            ProgressCircle::new(value).diameter(48.0),
            ProgressCircle::new(value).loading(true),
            ProgressCircle::new(value).status(Status::Warning),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(12);
    // section: end

    column![progress, values, sizes, circles]
        .spacing(24)
        .padding(16)
        .into()
}
