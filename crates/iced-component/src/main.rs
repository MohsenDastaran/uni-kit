//! The gallery the website embeds, and the fastest way to look at a component
//! while working on it. It is one iced application that shows the component
//! named on the command line — the same shape as the GPUI gallery, which serves
//! every component from one wasm module rather than one build per page.

use iced::widget::text;
use iced::{Element, Task};

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — iced")
        .run()
}

fn boot() -> () {}

fn update(_: &mut (), _: ()) -> Task<()> {
    Task::none()
}

fn view(_: &()) -> Element<'_, ()> {
    text("uni-kit iced gallery").into()
}
