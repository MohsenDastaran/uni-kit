//! `marker` — plain or highlighted text at text-sm.
//!
//! Run it with `cargo run --example marker`, or in the browser as
//! `?story=marker` in the gallery the website embeds.

use iced::widget::{column, row, text};
use iced::{Element, Task};

use crate::components::marker::{Marker, Variant};

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — marker")
        .theme(theme)
        .run()
}

fn boot() {}

fn update(_: &mut (), _: ()) -> Task<()> {
    Task::none()
}

fn view(_: &()) -> Element<'_, ()> {
    column![
        row![
            text("The plan is ").size(14.0),
            Marker::new("almost full").highlight(),
            text(" and the next tier is cheaper.").size(14.0),
        ]
        .align_y(iced::Alignment::Center),
        // A whole line is a marker too, when the band runs the width of it.
        Marker::new("Everything on this line is highlighted").highlight(),
        Marker::new("Plain text, the same size and colour without the band"),
        Marker::new("The band is the warning colour at 35%.")
            .variant(Variant::Highlight),
    ]
    .spacing(12)
    .padding(16)
    .into()
}
