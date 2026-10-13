//! `table` — a static table: header, body, footer and caption.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example table`.

fn main() -> iced::Result {
    iced_component::demos::table::run(iced::Theme::Light)
}
