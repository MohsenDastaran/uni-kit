//! `badge` — a count, a dot or an icon anchored to the corner of its child.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example badge`.

fn main() -> iced::Result {
    iced_component::demos::badge::run(iced::Theme::Light)
}
