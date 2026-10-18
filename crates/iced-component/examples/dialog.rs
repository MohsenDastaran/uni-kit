//! `dialog` — a modal panel: title, description, a body, a footer and a close
//! button over a dimmed backdrop, with the close button, the backdrop and the
//! Escape binding the application owns.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example dialog`.

fn main() -> iced::Result {
    iced_component::demos::dialog::run(iced::Theme::Light)
}
