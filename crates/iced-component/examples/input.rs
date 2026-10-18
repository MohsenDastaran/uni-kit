//! `input` — a single-line text field, in the states and sizes the catalog
//! shares with GPUI and Slint, with a prefix and a suffix slot of any element.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example input`.

fn main() -> iced::Result {
    iced_component::demos::input::run(iced::Theme::Light)
}
