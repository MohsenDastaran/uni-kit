//! `stepper` — a step-by-step progress indicator in the layouts, sizes and
//! disabled states the catalog shares with GPUI and Slint, with optional icons
//! and descriptions.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example stepper`.

fn main() -> iced::Result {
    iced_component::demos::stepper::run(iced::Theme::Light)
}
