//! `message-role` — the two sides of a conversation turn, and the marker each
//! one shows.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example message_role`.

fn main() -> iced::Result {
    iced_component::demos::message_role::run(iced::Theme::Light)
}
