//! `icon` — an SVG the application supplies, tinted from the theme and drawn at
//! the catalog's four sizes, still or turning.
//!
//! The artwork below is the example's own: this component never carries an icon
//! set, because the set belongs to the application. A real one embeds Lucide or
//! its own brand marks the same way, through `svg::Handle::from_memory`.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example icon`.

fn main() -> iced::Result {
    iced_component::demos::icon::run(iced::Theme::Light)
}
