//! `focus-trap` — the region a modal surface puts its controls in, with the
//! frame the reference draws and the focus half iced 0.14 cannot.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example focus_trap`.

fn main() -> iced::Result {
    iced_component::demos::focus_trap::run(iced::Theme::Light)
}
