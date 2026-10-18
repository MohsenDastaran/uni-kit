//! `rating` — a row of stars that reports the value it wants to become, in the
//! sizes and read-only state the catalog shares with GPUI and Slint.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example rating`.

fn main() -> iced::Result {
    iced_component::demos::rating::run(iced::Theme::Light)
}
