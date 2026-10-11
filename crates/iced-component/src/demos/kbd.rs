//! `kbd` — a keyboard-shortcut chip, and a row of them.
//!
//! Run it with `cargo run --example kbd`, or in the browser as
//! `?story=kbd` in the gallery the website embeds.

use iced::widget::{column, row, text};
use iced::{Element, Task};

use crate::components::kbd::{sequence, Kbd};
use crate::components::theme::ControlSize;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — kbd")
        .theme(theme)
        .run()
}

fn boot() {}

fn update(_: &mut (), _: ()) -> Task<()> {
    Task::none()
}

fn hint<'a>(keys: Element<'static, ()>, label: &str) -> Element<'a, ()> {
    row![keys, text(label.to_string()).size(14.0)]
        .spacing(8)
        .align_y(iced::Alignment::Center)
        .into()
}

fn view(_: &()) -> Element<'_, ()> {
    column![
        hint(Kbd::new("⌘").into(), "the command key on its own"),
        hint(Kbd::new("K").size(ControlSize::Small).into(), "small: 18px tall"),
        hint(
            Kbd::new("Enter").size(ControlSize::Large).into(),
            "large: 24px tall",
        ),
        hint(
            Kbd::new("Esc").size(ControlSize::XSmall).into(),
            "xsmall: 16px tall",
        ),
        hint(sequence(["⌘", "⇧", "P"]), "a sequence, 4px apart"),
    ]
    .spacing(12)
    .padding(16)
    .into()
}
