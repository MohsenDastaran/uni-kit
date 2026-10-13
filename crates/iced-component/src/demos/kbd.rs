//! `kbd` — a keyboard-shortcut chip, and a row of them.
//!
//! Run it with `cargo run --example kbd`, or in the browser as
//! `?story=kbd` in the gallery the website embeds.
//!
//! The marks a chip can paint are the face's, not the shortcut's. On wasm the
//! gallery's monospace family resolves to the embedded Fira Sans, whose cmap has
//! `⇧` (U+21E7), `⌫` (U+232B) and the arrows, and **not** `⌘` (U+2318),
//! `⌥` (U+2325) or `⌃` (U+2303). A mark the face lacks is not a missing chip:
//! the glyph is simply absent, so `⌘N` reads as a bare `N` and the shortcut
//! reads wrong. The gallery therefore spells those marks in words — `Cmd` for
//! command, `Alt` for option — as `demos/menu.rs` does; the rule is documented
//! on `Menu::shortcut`. The marks the face does have are kept, so `⇧` is a real
//! shift glyph and `Cmd ⇧ P` is a chord of one word and two marks.

use iced::widget::{column, row, text};
use iced::{Element, Task};

use crate::components::kbd::{sequence, Kbd};
use crate::components::theme::ControlSize;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — kbd")
        .theme(theme)
        .run()
}

fn boot() {}

fn update(_: &mut (), _: ()) -> Task<()> {
    Task::none()
}

fn hint<'a>(keys: Element<'static, ()>, label: &str) -> Element<'a, ()> {
    row![keys, text(label.to_string()).size(14.0)]
        .spacing(8)
        .align_y(iced::Alignment::Center)
        .into()
}

fn view(_: &()) -> Element<'_, ()> {
    // `Cmd` is spelled out because the wasm face has no `⌘`; `⇧` is a real glyph
    // in that face, so it stays a mark rather than becoming the word `Shift`.
    column![
        hint(Kbd::new("Cmd").into(), "the command key on its own"),
        hint(Kbd::new("K").size(ControlSize::Small).into(), "small: 18px tall"),
        hint(
            Kbd::new("Enter").size(ControlSize::Large).into(),
            "large: 24px tall",
        ),
        hint(
            Kbd::new("Esc").size(ControlSize::XSmall).into(),
            "xsmall: 16px tall",
        ),
        hint(sequence(["Cmd", "⇧", "P"]), "a sequence, 4px apart"),
    ]
    .spacing(12)
    .padding(16)
    .into()
}
