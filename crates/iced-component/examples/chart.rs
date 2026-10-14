//! `chart` — the plot surface: a line, a bar chart, the axes and the hover
//! tooltip, in the shape the Slint example shows.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example chart`.

fn main() -> iced::Result {
    iced_component::demos::chart::run(iced::Theme::Light)
}
