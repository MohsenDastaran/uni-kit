//! `group-box` (module `group_box`) — a heading over content, in the plain,
//! filled and outlined shapes.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example group_box`.

fn main() -> iced::Result {
    iced_component::demos::group_box::run(iced::Theme::Light)
}
