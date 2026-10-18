//! `sidebar` — an application sidebar: a header, scrolling groups of entries
//! and a footer, with the collapsed rail and the application's own expand
//! trigger.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example sidebar`.

fn main() -> iced::Result {
    iced_component::demos::sidebar::run(iced::Theme::Light)
}
