//! `attachment` — a file or media chip, in the lifecycle states, densities,
//! axes and slots the catalog shares with GPUI and Slint.
//!
//! The previews are elements the caller builds: here a text glyph stands in for
//! a media glyph and a tinted panel for an image preview, because iced ships no
//! icon set and an application owns its assets. A real application passes
//! `svg(handle)` or `image(handle)` in the same place.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example attachment`.

fn main() -> iced::Result {
    iced_component::demos::attachment::run(iced::Theme::Light)
}
