//! `tooltip` — a delayed hover label over a trigger, placed on the side the
//! caller prefers and drawn as an overlay so no card or scroll edge can clip it.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example tooltip`.

fn main() -> iced::Result {
    iced_component::demos::tooltip::run(iced::Theme::Light)
}
