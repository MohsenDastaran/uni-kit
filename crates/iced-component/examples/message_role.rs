//! `message-role` — the two sides of a conversation turn, and the marker each
//! one shows.
//!
//! Run it with `cargo run --example message_role`.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length, Task};

use iced_component::components::message_role::{MessageRole, Role};
use iced_component::components::theme;

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — message-role")
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    /// Whether the sent turn failed, which is the one state the marker itself
    /// reports: message.slint tints a failed sent turn destructively.
    failed: bool,
}

#[derive(Debug, Clone)]
enum Message {
    /// Flips the sent turn between delivered and failed.
    ToggleFailed,
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::ToggleFailed => app.failed = !app.failed,
    }
    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    let sent = Role::User;
    let received = Role::Assistant;

    column![
        // The two roles, each with the initials message.slint falls back to
        // when the caller names none: "YO" for the sent side, "AL" for the
        // received one.
        row![
            MessageRole::new(sent),
            MessageRole::new(received),
        ]
        .spacing(theme::SPACING_SM),
        // The caller's own initials replace the fallback; the role still picks
        // the fill and the colour the initials are drawn in.
        row![
            MessageRole::new(sent).initials("MS"),
            MessageRole::new(received).initials("ST"),
        ]
        .spacing(theme::SPACING_SM),
        // The failed state. It tints the sent turn destructive and leaves the
        // received one muted, which is what message.slint does with its
        // `failed` flag. The sent marker follows the app state below.
        row![
            MessageRole::new(sent).failed(app.failed),
            MessageRole::new(received).failed(true),
        ]
        .spacing(theme::SPACING_SM),
        button(text(if app.failed {
            "Sent turn: failed"
        } else {
            "Sent turn: delivered"
        }))
        .on_press(Message::ToggleFailed),
        // The side each role sits on, which is what Role::alignment carries:
        // the received turn at the leading edge, the sent one at the trailing
        // edge.
        row![
            text("received").size(theme::FONT_SIZE_XS),
            container(MessageRole::new(received))
                .width(Length::Fill)
                .align_x(received.alignment()),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(Alignment::Center),
        row![
            container(MessageRole::new(sent))
                .width(Length::Fill)
                .align_x(sent.alignment()),
            text("sent").size(theme::FONT_SIZE_XS),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(Alignment::Center),
    ]
    .spacing(theme::SPACING_MD)
    .padding(theme::SPACING_LG)
    .into()
}
