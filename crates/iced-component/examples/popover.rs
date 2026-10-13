//! `popover` — a floating panel anchored to a trigger, opened by a click,
//! dismissed by a click outside it or by Escape, and placed on the side of the
//! trigger the caller prefers.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example popover`.

fn main() -> iced::Result {
    iced_component::demos::popover::run(iced::Theme::Light)
}
