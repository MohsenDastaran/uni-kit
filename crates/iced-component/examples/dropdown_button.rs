//! `dropdown_button` — a split button: an action half, and a caret half that
//! opens a `menu`.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example dropdown_button`.

fn main() -> iced::Result {
    iced_component::demos::dropdown_button::run(iced::Theme::Light)
}
