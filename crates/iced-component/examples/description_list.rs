//! `description-list` (module `description_list`) — term and detail pairs, in
//! columns, with borders and separators.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example description_list`.

fn main() -> iced::Result {
    iced_component::demos::description_list::run(iced::Theme::Light)
}
