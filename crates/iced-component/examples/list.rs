//! `list` — a selectable list of rows on the styled scroll area.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example list`.

fn main() -> iced::Result {
    iced_component::demos::list::run(iced::Theme::Light)
}
