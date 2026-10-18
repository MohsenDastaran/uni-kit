//! `marker` — plain or highlighted text at text-sm.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example marker`.

fn main() -> iced::Result {
    iced_component::demos::marker::run(iced::Theme::Light)
}
