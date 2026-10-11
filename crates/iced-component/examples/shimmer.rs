//! `shimmer` — a moving highlight over a muted band, at a value the caller
//! advances.
//!
//! iced cannot paint a gradient through part of a text run, so the component
//! draws the band and returns the label for the caller to place over it. The
//! buttons stand in for the subscription an application would run.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example shimmer`.

fn main() -> iced::Result {
    iced_component::demos::shimmer::run(iced::Theme::Light)
}
