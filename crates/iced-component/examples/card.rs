//! `card` — a bordered surface with a badge, a title, a description and a body.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example card`.

fn main() -> iced::Result {
    iced_component::demos::card::run(iced::Theme::Light)
}
