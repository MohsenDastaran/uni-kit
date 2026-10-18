//! `tree` — a disclosure tree.
//!
//! the component only reports the requests.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example tree`.

fn main() -> iced::Result {
    iced_component::demos::tree::run(iced::Theme::Light)
}
