//! `notification` — cards that report a status, stacked in a corner of the frame
//! they are given.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example notification`.

fn main() -> iced::Result {
    iced_component::demos::notification::run(iced::Theme::Light)
}
