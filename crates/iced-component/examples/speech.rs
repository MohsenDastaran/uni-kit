//! `speech` — dictation: a microphone control and a live level waveform, in the
//! shape the Slint example shows, with the session the application owns.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example speech`.

fn main() -> iced::Result {
    iced_component::demos::speech::run(iced::Theme::Light)
}
