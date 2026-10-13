//! `toolbar` — a transparent row of commands: a size ladder, labels and a field
//! beside the commands, a custom surface, and the groups that hold a run
//! together, with an overflow menu at the trailing edge.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example toolbar`.

fn main() -> iced::Result {
    iced_component::demos::toolbar::run(iced::Theme::Light)
}
