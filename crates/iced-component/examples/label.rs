//! `label` — a form label with a required mark and a suffix slot.
//!
//! Run it with `cargo run --example label`.

use iced::widget::{column, row, text};
use iced::{Element, Task};

use iced_component::components::label::Label;
use iced_component::components::theme::{ControlSize, Tokens};

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — label")
        .run()
}

fn boot() {}

fn update(_: &mut (), _: ()) -> Task<()> {
    Task::none()
}

fn hint<'a>(hint: &str) -> Element<'a, ()> {
    text(hint.to_string())
        .size(12.0)
        .style(|theme: &iced::Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

fn view(_: &()) -> Element<'_, ()> {
    column![
        Label::new("Workspace name"),
        Label::new("Email address").required(true),
        Label::new("Seats").suffix(hint("up to 20")),
        Label::new("Billing email")
            .required(true)
            .suffix(hint("we only use it for invoices")),
        row![
            Label::new("Small").size(ControlSize::Small),
            Label::new("Medium").size(ControlSize::Medium),
            Label::new("Large").size(ControlSize::Large),
        ]
        .spacing(24),
    ]
    .spacing(16)
    .padding(16)
    .into()
}
