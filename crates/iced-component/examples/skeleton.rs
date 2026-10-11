//! `skeleton` — a static placeholder block at a pulse the caller advances.
//!
//! iced's timer needs a runtime feature this crate does not enable, so the
//! pulse is a value instead: zero is the dimmest the Slint animation reaches,
//! one is the resting block. The buttons stand in for the subscription an
//! application would run.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example skeleton`.

fn main() -> iced::Result {
    iced_component::demos::skeleton::run(iced::Theme::Light)
}
