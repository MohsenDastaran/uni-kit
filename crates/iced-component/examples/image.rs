//! `image` — a framed picture: a source, the three fits, a corner radius, a
//! placeholder and a selecting gallery, in the states the catalog shares with
//! GPUI and Slint.
//!
//! iced ships no image codecs and this example fetches nothing: the pictures are
//! RGBA bytes the demo generates at startup, which is the same `Handle` an
//! application gets from a decoded file. The placeholder icon is an inline SVG,
//! tinted from the style closure so it stays visible in the dark theme.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example image`.

fn main() -> iced::Result {
    iced_component::demos::image::run(iced::Theme::Light)
}
