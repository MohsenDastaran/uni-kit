//! `pagination` — page controls with the run, the ellipses and the two step
//! buttons.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example pagination`.

fn main() -> iced::Result {
    iced_component::demos::pagination::run(iced::Theme::Light)
}
