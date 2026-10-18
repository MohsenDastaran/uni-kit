//! The gallery's own page: what to show when `?story=` names nothing.
//!
//! The gallery is also how somebody finds out what exists, so a missing or
//! mistyped story is answered with the list rather than a blank window or a
//! panic. It is the one demo the dispatcher in `scripts/mod-rs.mjs` does not
//! enumerate, because it is not a story: nothing asks for it by name.

use crate::demos::STORIES;
use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};

/// Run the list, saying which story was asked for and did not exist.
pub fn run(theme: iced::Theme, requested: String) -> iced::Result {
    iced::application(move || requested.clone(), update, view)
        .title("uni-kit — iced gallery")
        .theme(theme)
        .run()
}

fn update(_: &mut String, _: ()) -> Task<()> {
    Task::none()
}

fn view(requested: &String) -> Element<'_, ()> {
    let note = if requested.is_empty() {
        "No story was asked for. These components have a demo:".to_string()
    } else {
        format!("There is no demo called \"{requested}\". These components have one:")
    };

    // Four names to a line keeps the list readable in the frame the component
    // pages embed, which is wider than it is tall.
    let mut list = column![].spacing(6.0);
    for chunk in STORIES.chunks(4) {
        let mut line = row![].spacing(18.0);
        for name in chunk {
            line = line.push(text(*name).size(13.0));
        }
        list = list.push(line);
    }

    container(
        column![text("uni-kit — iced gallery").size(20.0), text(note), list].spacing(14.0),
    )
    .padding(24.0)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
