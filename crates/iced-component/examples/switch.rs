//! `switch` — an on/off track with a sliding thumb, a label, the four sizes and
//! a disabled state, the shape the catalog shares with GPUI and Slint.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example switch`.

fn main() -> iced::Result {
    iced_component::demos::switch::run(iced::Theme::Light)
}
