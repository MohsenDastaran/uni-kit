//! `message_scroller` — a conversation that stays pinned to its newest turn,
//! in the two bubble variants the scroller's rows draw, with the unread rule,
//! the bottom fade and the jump-to-newest control it shows when the reader has
//! left the tail.
//!
//! The flag the scroller reports through is shared with the view rather than
//! copied into it, because iced hands the scroll event to an `Fn` closure and
//! the view is rebuilt after every message: a `Cell` owned by both sides is what
//! iced's own scroll documentation suggests, and `Rc<Cell<bool>>` is how this
//! example keeps one.
//!
//! with the wheel to bring out the fade and the round control; press the control
//! to come back to the tail.
//!
//! The demo itself lives in the library, so the gallery the website embeds
//! shows the same one. Run it with `cargo run --example message_scroller`.

fn main() -> iced::Result {
    iced_component::demos::message_scroller::run(iced::Theme::Light)
}
