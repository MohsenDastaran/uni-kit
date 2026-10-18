//! `virtual-list` — a windowed list that builds only the rows on screen.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example virtual_list`.

fn main() -> iced::Result {
    iced_component::demos::virtual_list::run(iced::Theme::Light)
}
