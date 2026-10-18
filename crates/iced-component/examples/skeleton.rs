//! `skeleton` — a loading placeholder that pulses from the frame's own clock.
//!
//! The component samples each frame's instant and asks for the next one, so the
//! blocks move as soon as the window opens; there is no subscription and no
//! button to press. The demo itself lives in the library, so the gallery the
//! website embeds shows the same one. Run it with `cargo run --example skeleton`.

fn main() -> iced::Result {
    iced_component::demos::skeleton::run(iced::Theme::Light)
}
