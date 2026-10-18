//! `calendar` — a month grid.
//!
//! and the selection; the component reports picks and shift requests.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example calendar`.

fn main() -> iced::Result {
    iced_component::demos::calendar::run(iced::Theme::Light)
}
