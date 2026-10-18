//! `textarea` — a multi-line text field, in the states and sizes the catalog
//! shares with GPUI and Slint, with the rows that decide its height and a frame
//! the caller can drop.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example textarea`.

fn main() -> iced::Result {
    iced_component::demos::textarea::run(iced::Theme::Light)
}
