//! `description-list` (module `description_list`) — term and detail pairs, in
//! columns, with borders and separators.
//!
//! Run it with `cargo run --example description_list`, or in the browser as
//! `?story=description_list` in the gallery the website embeds.

use iced::widget::{column, text};
use iced::{Element, Task};

use crate::components::description_list::{DescriptionList, Item, Layout};
use crate::components::theme::ControlSize;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — description-list")
        .theme(theme)
        .run()
}

fn boot() {}

fn update(_: &mut (), _: ()) -> Task<()> {
    Task::none()
}

fn view(_: &()) -> Element<'_, ()> {
    let items = vec![
        Item::new("Name", "Ada Lovelace"),
        Item::new("Email", "ada@example.com"),
        Item::new("Role", "Owner"),
        Item::separator(),
        Item::new("Plan", "Team").span(2),
        Item::new("Seats", "12"),
        Item::new("Region", "eu-west-1"),
        Item::new(
            "Description",
            "A single workspace for the whole team, billed monthly, with a seat added for every member who joins.",
        )
        .span(3),
    ];

    column![
        text("One column, bordered, the medium insets:").size(14.0),
        DescriptionList::new(items.clone()),
        text("Three columns, with a span of two and a span of three:").size(14.0),
        DescriptionList::new(items.clone())
            .columns(3)
            .label_width(80.0),
        text("The stacked form, at the large insets and font size:").size(14.0),
        DescriptionList::new(items)
            .layout(Layout::Vertical)
            .size(ControlSize::Large),
    ]
    .spacing(16)
    .padding(16)
    .into()
}
