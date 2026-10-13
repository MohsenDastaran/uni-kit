//! `form` — composition and validation over the controls beside it: the layout,
//! the labels, the error placement and the state that ties a control to its
//! message.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example form`.

fn main() -> iced::Result {
    iced_component::demos::form::run(iced::Theme::Light)
}
