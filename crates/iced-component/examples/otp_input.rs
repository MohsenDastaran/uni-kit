//! `otp-input` — a one-time code as a row of single-character cells: the
//! digits-only value, the auto-advance, the backspace step back, the sizes, the
//! masked cells and the disabled field, with the caller owning the code.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example otp_input`.

fn main() -> iced::Result {
    iced_component::demos::otp_input::run(iced::Theme::Light)
}
