//! `checkbox` — a binary choice with a label, a description under it, the
//! indeterminate dash, sizes and a disabled state.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example checkbox`.

fn main() -> iced::Result {
    iced_component::demos::checkbox::run(iced::Theme::Light)
}
