//! `settings` — a settings page: labelled rows with a control at the trailing
//! edge, on the two surfaces a group takes.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example settings`.

fn main() -> iced::Result {
    iced_component::demos::settings::run(iced::Theme::Light)
}
