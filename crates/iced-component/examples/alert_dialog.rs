//! `alert-dialog` — a confirmation dialog: the crate's `Dialog` with a centered
//! footer of a cancel and a confirm action, and the two channels the application
//! answers.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example alert-dialog`.

fn main() -> iced::Result {
    iced_component::demos::alert_dialog::run(iced::Theme::Light)
}
