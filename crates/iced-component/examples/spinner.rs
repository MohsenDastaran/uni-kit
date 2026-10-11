//! `spinner` — a rotating arc at a progress the caller advances.
//!
//! iced's timer needs a runtime feature this crate does not enable, so the
//! spinner takes the progress instead: one unit is one full turn. The buttons
//! stand in for the subscription an application would run, and without one the
//! spinner still draws its whole shape.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example spinner`.

fn main() -> iced::Result {
    iced_component::demos::spinner::run(iced::Theme::Light)
}
