//! `date-picker` — a date field that opens a calendar, with the date and time,
//! the size ladder, blocked weekends, a range and one to three months.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example date_picker`.

fn main() -> iced::Result {
    iced_component::demos::date_picker::run(iced::Theme::Light)
}
