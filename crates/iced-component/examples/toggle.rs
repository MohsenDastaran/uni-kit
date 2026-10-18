//! `toggle` — a pressed-in button for a binary state, with an optional icon and
//! label, the two variants, the four sizes and a disabled state.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example toggle`.

fn main() -> iced::Result {
    iced_component::demos::toggle::run(iced::Theme::Light)
}
