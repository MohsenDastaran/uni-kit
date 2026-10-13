//! `time-field` — a segmented time of day: the 24-hour clock, the seconds
//! precision, the 12-hour clock with its AM/PM label, the size ladder and the
//! disabled and invalid states, with the caller owning the value and taking
//! every edit back through `on_change`.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example time_field`.

fn main() -> iced::Result {
    iced_component::demos::time_field::run(iced::Theme::Light)
}
