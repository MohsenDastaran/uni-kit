//! `combobox` — a select-like trigger with a search field in its panel, one or
//! many choices, disabled rows, a footer that creates the typed item, the sizes,
//! the maximum the multiple choice takes, the disabled state and the keyboard
//! the component leaves to its host.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example combobox`.

fn main() -> iced::Result {
    iced_component::demos::combobox::run(iced::Theme::Light)
}
