//! `avatar` — a user portrait, in the fallbacks, sizes and chrome the catalog
//! shares with GPUI and Slint.
//!
//! iced ships no image and no icon set, and this example embeds neither: the
//! portrait is a handle built from RGBA bytes the file generates at startup, and
//! the icon fallback is an inline SVG string. Both arrive through the same
//! public API an application uses for its own assets.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example avatar`.

fn main() -> iced::Result {
    iced_component::demos::avatar::run(iced::Theme::Light)
}
