//! `scrollable` — a styled scroll area, in the axes, bar geometry and show modes
//! the catalog shares with GPUI and Slint.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example scrollable`.

fn main() -> iced::Result {
    iced_component::demos::scrollable::run(iced::Theme::Light)
}
