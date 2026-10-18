//! `text-view` — a rendered Markdown document, in the blocks, states and
//! spacings the catalog shares with GPUI and Slint.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example text_view`.

fn main() -> iced::Result {
    iced_component::demos::text_view::run(iced::Theme::Light)
}
