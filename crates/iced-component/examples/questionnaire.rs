//! `questionnaire` — a sequence of questions, one at a time: progress, choices,
//! a write-your-own row, an error line and the Back/Next pair.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example questionnaire`.

fn main() -> iced::Result {
    iced_component::demos::questionnaire::run(iced::Theme::Light)
}
