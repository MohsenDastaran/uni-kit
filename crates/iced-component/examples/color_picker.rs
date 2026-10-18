//! `color-picker` — a colour field that opens a palette and an HSLA panel, with
//! the compact square and the framed field trigger, the size ladder, a caller's
//! featured row and the hex field.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example color_picker`.

fn main() -> iced::Result {
    iced_component::demos::color_picker::run(iced::Theme::Light)
}
