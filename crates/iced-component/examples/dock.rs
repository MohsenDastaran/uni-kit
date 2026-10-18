//! `dock` — a four-region workspace of tab groups, in the shape the catalog
//! shares with GPUI and Slint: a panel is its identity, title, region and body,
//! a tab strip selects and closes, a divider drags every pixel and a tab drags
//! onto another to move its panel.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example dock`.

fn main() -> iced::Result {
    iced_component::demos::dock::run(iced::Theme::Light)
}
