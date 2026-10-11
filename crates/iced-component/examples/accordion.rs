//! `accordion` — a vertical stack of collapsible sections.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example accordion`.

fn main() -> iced::Result {
    iced_component::demos::accordion::run(iced::Theme::Light)
}
