//! `bubble` — a chat surface: the variants, the two alignments, rich and long
//! content, groups, reactions, and a caller-owned surface colour.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example bubble`.

fn main() -> iced::Result {
    iced_component::demos::bubble::run(iced::Theme::Light)
}
