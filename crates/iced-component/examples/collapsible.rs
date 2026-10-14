//! `collapsible` — a single expandable panel, in the shapes the catalog shares
//! with GPUI and Slint: a titled trigger, a trigger row, a bottom-edge trigger,
//! and the panels a trigger beside or above a card opens.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example collapsible`.

fn main() -> iced::Result {
    iced_component::demos::collapsible::run(iced::Theme::Light)
}
