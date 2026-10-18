//! `sheet` — a panel anchored to an edge of the surface it covers, with a
//! header, a scrolling body and a footer action.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example sheet`.

fn main() -> iced::Result {
    iced_component::demos::sheet::run(iced::Theme::Light)
}
