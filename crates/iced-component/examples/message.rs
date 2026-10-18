//! `message` — a whole chat turn: the identity marker, the sender line, the
//! bubble, its actions and the delivery footer, at both edges and in the ghost
//! and caller-owned surfaces.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example message`.

fn main() -> iced::Result {
    iced_component::demos::message::run(iced::Theme::Light)
}
