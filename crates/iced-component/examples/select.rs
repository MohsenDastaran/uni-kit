//! `select` — a closed list of options behind a field-shaped trigger, with the
//! placeholder, the sizes, the report a choice makes, the trigger's own surface,
//! the disabled state and the keyboard traversal the component leaves to its
//! host.
//!
//! The demo itself lives in the library, so the gallery the website embeds shows
//! the same one. Run it with `cargo run --example select`.

fn main() -> iced::Result {
    iced_component::demos::select::run(iced::Theme::Light)
}
