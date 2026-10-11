//! `empty` — an empty-state panel with a media slot, a title, a description and
//! up to two actions.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example empty`.

fn main() -> iced::Result {
    iced_component::demos::empty::run(iced::Theme::Light)
}
