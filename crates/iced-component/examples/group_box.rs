//! `group-box` (module `group_box`) — a heading over content, in the plain,
//! filled and outlined shapes.
//!
//! Run it with `cargo run --example group_box`.

use iced::widget::{column, text};
use iced::{Element, Task};

use iced_component::components::group_box::{GroupBox, Variant};
use iced_component::components::label::Label;
use iced_component::components::theme::Tokens;

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — group-box")
        .run()
}

fn boot() {}

fn update(_: &mut (), _: ()) -> Task<()> {
    Task::none()
}

/// A field: a label and its value, stacked at the 4px gap a label uses.
fn field<'a>(label: &str, value: &str) -> Element<'a, ()> {
    column![
        Label::new(label.to_string()),
        text(value.to_string())
            .size(14.0)
            .style(|theme: &iced::Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    ]
    .spacing(4)
    .into()
}

fn view(_: &()) -> Element<'_, ()> {
    // Each group owns its body, so the surface's 16px gap is what stacks them.
    let details = || {
        column![
            field("Workspace", "uni-kit"),
            field("Owner", "ada@example.com"),
        ]
        .spacing(16)
    };

    column![
        GroupBox::new()
            .title("Profile")
            .body(details())
            .footer("The name and the address the team sees."),
        GroupBox::new()
            .title("Profile")
            .variant(Variant::Fill)
            .body(details())
            .footer("A filled surface, in the theme's group-box colour."),
        GroupBox::new()
            .title("Profile")
            .variant(Variant::Outline)
            .body(details())
            .footer("An outlined surface, with the same inset and gap."),
    ]
    .spacing(24)
    .padding(16)
    .into()
}
