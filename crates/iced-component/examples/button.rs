//! `button` — a push button, in the variants, sizes and states the catalog
//! shares with GPUI and Slint.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example button`.

fn main() -> iced::Result {
    iced_component::demos::button::run(iced::Theme::Light)
}
