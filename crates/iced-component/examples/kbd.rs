//! `kbd` — a keyboard-shortcut chip, and a row of them.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example kbd`.

fn main() -> iced::Result {
    iced_component::demos::kbd::run(iced::Theme::Light)
}
