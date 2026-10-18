//! `number-input` — a numeric field with steppers, in the states, sizes and
//! bounds the catalog shares with GPUI and Slint, with a prefix and a suffix slot
//! of any element.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example number-input`.

fn main() -> iced::Result {
    iced_component::demos::number_input::run(iced::Theme::Light)
}
