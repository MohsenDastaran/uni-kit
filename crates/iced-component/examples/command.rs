//! `command` — a command palette with groups, shortcuts and a search field: the
//! inline panel, the same palette without search, inside a dialog, inside a
//! scroll area, with variable-height rows, with the rich stock rows, and the
//! line that records the last choice.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example command`.

fn main() -> iced::Result {
    iced_component::demos::command::run(iced::Theme::Light)
}
