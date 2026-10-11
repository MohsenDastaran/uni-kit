//! `alert` — a callout in the status colours, with an optional title and a
//! dismiss control.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example alert`.

fn main() -> iced::Result {
    iced_component::demos::alert::run(iced::Theme::Light)
}
