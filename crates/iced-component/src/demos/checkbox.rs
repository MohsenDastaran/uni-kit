//! `checkbox` — a binary choice with a label, a description under it, the
//! indeterminate dash, sizes and a disabled state.
//!
//! Run it with `cargo run --example checkbox`, or in the browser as
//! `?story=checkbox` in the gallery the website embeds.

use iced::widget::{column, row, text};
use iced::{Element, Task};

use crate::components::checkbox::Checkbox;
use crate::components::theme::ControlSize;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — checkbox")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    terms: bool,
    updates: bool,
    some: bool,
}

#[derive(Debug, Clone)]
enum Message {
    /// iced reports the requested value, not the toggled one, so the message
    /// carries it and `update` owns the decision — as it does in Slint, where
    /// `changed(checked)` reports what the control wants to become.
    Terms(bool),
    Updates(bool),
    Some(bool),
}

fn boot() -> App {
    App {
        terms: true,
        updates: false,
        some: false,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Terms(checked) => app.terms = checked,
        Message::Updates(checked) => app.updates = checked,
        Message::Some(checked) => app.some = checked,
    }
    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    let summary = format!(
        "terms {} · updates {} · some {}",
        app.terms, app.updates, app.some
    );

    column![
        Checkbox::new(app.terms)
            .label("Accept terms and conditions")
            .on_toggle(Message::Terms),
        Checkbox::new(app.updates)
            .label("Product updates")
            .description("One email a month, no more")
            .on_toggle(Message::Updates),
        // The dash: a choice that stands for "some of them".
        Checkbox::new(app.some)
            .indeterminate(true)
            .label("Select all")
            .on_toggle(Message::Some),
        // Without a label the box stands alone, as GPUI's does.
        Checkbox::new(true).on_toggle(Message::Terms),
        // The two disabled looks: one on, one off. No message means disabled.
        row![
            Checkbox::new(true).label("Checked and disabled"),
            Checkbox::new(false).label("Unchecked and disabled"),
        ]
        .spacing(24),
        // The sizes.
        row![
            Checkbox::new(true).size(ControlSize::XSmall).label("Extra small"),
            Checkbox::new(true).size(ControlSize::Small).label("Small"),
            Checkbox::new(true).size(ControlSize::Medium).label("Medium"),
            Checkbox::new(true).size(ControlSize::Large).label("Large"),
        ]
        .spacing(24),
        text(summary),
    ]
    .spacing(16)
    .padding(16)
    .into()
}
