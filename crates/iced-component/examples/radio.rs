//! `radio` — one option in a mutually exclusive set, alone or in a group that
//! keeps a single selection, in the sizes and disabled states the catalog shares
//! with GPUI and Slint.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example radio`.

fn main() -> iced::Result {
    iced_component::demos::radio::run(iced::Theme::Light)
}
