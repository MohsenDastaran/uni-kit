//! `status-bar` — a window footer split into leading, centre and trailing
//! regions, with read-only labels, pressable segments and separators.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example status-bar`.

fn main() -> iced::Result {
    iced_component::demos::status_bar::run(iced::Theme::Light)
}
