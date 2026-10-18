//! `input-group` — one frame around a text field and the text, icons and buttons
//! that sit around it, in the states and sizes the catalog shares with GPUI and
//! Slint.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example input_group`.

fn main() -> iced::Result {
    iced_component::demos::input_group::run(iced::Theme::Light)
}
