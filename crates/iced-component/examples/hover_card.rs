//! `hover-card` — a delayed card of rich content over whatever the pointer rests
//! on.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example hover_card`.

fn main() -> iced::Result {
    iced_component::demos::hover_card::run(iced::Theme::Light)
}
