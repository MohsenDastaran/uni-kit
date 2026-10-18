//! `tag` — a chip in four variants, with a leading icon and a close control.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example tag`.

fn main() -> iced::Result {
    iced_component::demos::tag::run(iced::Theme::Light)
}
