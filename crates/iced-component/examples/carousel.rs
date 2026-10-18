//! `carousel` — a paginated viewer for a set of related slides, with previous
//! and next controls, pagination dots and a looping wrap.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example carousel`.

fn main() -> iced::Result {
    iced_component::demos::carousel::run(iced::Theme::Light)
}
