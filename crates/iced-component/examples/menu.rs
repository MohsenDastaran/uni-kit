//! `menu` — a list of actions with icons, shortcuts, a check state, disabled
//! and destructive rows, and the keyboard traversal the Slint `Menu` leaves to
//! its host.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example menu`.

fn main() -> iced::Result {
    iced_component::demos::menu::run(iced::Theme::Light)
}
