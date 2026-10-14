//! `clipboard` — a copy control, in the shape the Slint example shows: an input
//! with the clipboard as its suffix, the check coming back when the press
//! reports the field's value.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example clipboard`.

fn main() -> iced::Result {
    iced_component::demos::clipboard::run(iced::Theme::Light)
}
