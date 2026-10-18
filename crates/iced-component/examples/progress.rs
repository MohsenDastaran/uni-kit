//! `progress` — a linear bar and a circle, at a value the caller advances.
//!
//! iced's timer needs a runtime feature this crate does not enable, so the
//! component takes the value instead. The buttons below stand in for the
//! subscription an application would run: each press moves the value, and the
//! loading forms use the same value as their phase.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example progress`.

fn main() -> iced::Result {
    iced_component::demos::progress::run(iced::Theme::Light)
}
