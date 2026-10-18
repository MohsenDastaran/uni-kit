//! `editor` — a source-code editor: a header, a line-number gutter, a monospace
//! field and a status bar.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example editor`.

fn main() -> iced::Result {
    iced_component::demos::editor::run(iced::Theme::Light)
}
