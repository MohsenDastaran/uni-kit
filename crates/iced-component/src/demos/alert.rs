//! `alert` — a callout in the status colours, with an optional title and a
//! dismiss control.
//!
//! Run it with `cargo run --example alert`, or in the browser as
//! `?story=alert` in the gallery the website embeds.

use iced::widget::{column, text};
use iced::{Element, Task};

use crate::components::alert::{Alert, Status};
use crate::components::button::{Button, Variant};
use crate::components::theme::ControlSize;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — alert")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone)]
struct App {
    /// The caller owns the flag: an alert has no `open` property in iced, so
    /// the view stops constructing it.
    shown: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { shown: true }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Dismiss,
    Show,
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Dismiss => app.shown = false,
        Message::Show => app.shown = true,
    }
    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    let mut content = column![].spacing(12).padding(16);

    content = content.push(Alert::new("A new version is available."));
    content = content.push(
        Alert::new("Your changes could not be saved.")
            .title("Something went wrong")
            .status(Status::Danger),
    );
    content = content.push(
        Alert::new("Checks passed.")
            .title("All good")
            .status(Status::Success),
    );
    content = content.push(
        Alert::new("The build is running. You can leave this page; the result will arrive in your inbox.")
            .title("Working in the background")
            .status(Status::Info)
            .banner(true),
    );

    if app.shown {
        content = content.push(
            Alert::new("This is the last reminder.")
                .status(Status::Warning)
                .size(ControlSize::Small)
                .on_close(Message::Dismiss),
        );
    } else {
        content = content.push(
            Button::new("Show the dismissible alert")
                .variant(Variant::Primary)
                .on_press(Message::Show),
        );
    }

    content = content.push(text("The alert's status decides its fill, edge and ink."));

    content.into()
}
