//! `tabs` — a tab bar over the active tab's content, in the three variants and
//! the sizes the catalog shares with GPUI and Slint.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example tabs`.

fn main() -> iced::Result {
    iced_component::demos::tabs::run(iced::Theme::Light)
}
