//! `slider` — a numeric range control in the sizes, bounds and steps the catalog
//! shares with GPUI and Slint, with click and drag on the track and the arrow
//! keys over it.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example slider`.

fn main() -> iced::Result {
    iced_component::demos::slider::run(iced::Theme::Light)
}
