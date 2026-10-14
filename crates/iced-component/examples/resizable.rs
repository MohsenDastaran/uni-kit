//! `resizable` — a split view of panes and draggable dividers, in the shape the
//! catalog shares with GPUI and Slint: a pane is a styled surface, a handle owns
//! the split, and the drag reports every pixel it travels.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example resizable`.

fn main() -> iced::Result {
    iced_component::demos::resizable::run(iced::Theme::Light)
}
