//! `button` — a push button, in the variants, sizes and states the catalog
//! shares with GPUI and Slint.
//!
//! Run it with `cargo run --example button`.

use iced::widget::{column, row, text};
use iced::{Element, Task};

use iced_component::components::button::{Button, Variant};
use iced_component::components::theme::ControlSize;

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — button")
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    saved: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Save,
    /// A button with a message is enabled; one without is disabled, which is
    /// how iced draws a control that cannot be pressed. The state demo below
    /// uses this to show the resting look without meaning anything by it.
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
    column![
        // The variants: a solid primary, a secondary, the status colours, and
        // the three that draw no fill of their own.
        row![
            Button::new("Primary")
                .variant(Variant::Primary)
                .on_press(Message::Save),
            Button::new("Secondary")
                .variant(Variant::Secondary)
                .on_press(Message::Nothing),
            Button::new("Danger")
                .variant(Variant::Danger)
                .on_press(Message::Nothing),
            Button::new("Success")
                .variant(Variant::Success)
                .on_press(Message::Nothing),
            Button::new("Warning")
                .variant(Variant::Warning)
                .on_press(Message::Nothing),
            Button::new("Info")
                .variant(Variant::Info)
                .on_press(Message::Nothing),
        ]
        .spacing(8),
        row![
            Button::new("Default").on_press(Message::Nothing),
            Button::new("Outlined").outline().on_press(Message::Nothing),
            Button::new("Ghost")
                .variant(Variant::Ghost)
                .on_press(Message::Nothing),
            Button::new("Link")
                .variant(Variant::Link)
                .on_press(Message::Nothing),
            Button::new("Text")
                .variant(Variant::Text)
                .on_press(Message::Nothing),
        ]
        .spacing(8),
        // The sizes: 20, 24, 32 and 40 pixels tall.
        row![
            Button::new("Extra small")
                .size(ControlSize::XSmall)
                .on_press(Message::Nothing),
            Button::new("Small")
                .size(ControlSize::Small)
                .on_press(Message::Nothing),
            Button::new("Medium")
                .size(ControlSize::Medium)
                .on_press(Message::Nothing),
            Button::new("Large")
                .size(ControlSize::Large)
                .on_press(Message::Nothing),
        ]
        .spacing(8),
        // The states a control reports about itself.
        row![
            Button::new("Selected")
                .variant(Variant::Primary)
                .selected(true)
                .on_press(Message::Nothing),
            Button::new("Working")
                .loading(true)
                .on_press(Message::Nothing),
            // No message, so iced draws it disabled.
            Button::new("Disabled"),
        ]
        .spacing(8),
        text(format!("saved {} times", app.saved)),
    ]
    .spacing(12)
    .padding(16)
    .into()
}
