//! `shimmer` — a moving highlight through the label's own ink, swept by the
//! component's own clock.
//!
//! The label is the component: `Shimmer::new(text)` draws the text and
//! recolours spans of it as the band passes, so there is no track behind it.
//! The demo replays a `once` sweep and switches the in-context sample between
//! loading and finished; the looping labels need no controls.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example shimmer`.

fn main() -> iced::Result {
    iced_component::demos::shimmer::run(iced::Theme::Light)
}
