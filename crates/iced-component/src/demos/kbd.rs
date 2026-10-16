//! `kbd` — a keyboard-shortcut chip, and a row of them.
//!
//! Run it with `cargo run --example kbd`, or in the browser as
//! `?story=kbd` in the gallery the website embeds.
//!
//! The Slint example's three outlined sections — Shortcuts, Multiple Modifiers,
//! and Arrow Keys and Function Keys — are the three here, in the same order,
//! under the heading the component page documents each at. The `// section:
//! <title>` comments below are the contract the website reads to cut this file
//! into the usage panels a component page shows, so the code on the page is the
//! code that runs here. Keep them, one per section, in order, and keep the
//! closing `// section: end` after the last one.
//!
//! Each heading carries the title and the code icon that asks the embedding page
//! to scroll to that sample. Both are the button, as in Slint's `SourceHeading`,
//! and a click posts the section's index and title through [`crate::bridge`],
//! the same bridge the Slint and GPUI galleries use.
//!
//! The marks a chip can paint are the face's, not the shortcut's. On wasm the
//! gallery's monospace family resolves to the embedded Fira Sans, whose cmap has
//! `⇧` (U+21E7), `⌫` (U+232B), `⌦`, `⇪`, `⏎` and the arrows, and **not** `⌘`
//! (U+2318), `⌥` (U+2325) or `⌃` (U+2303). A mark the face lacks is not a
//! missing chip: the glyph is simply absent, so `⌘N` reads as a bare `N` and the
//! shortcut reads wrong. The gallery therefore spells those marks in words —
//! `Cmd` for command, `Alt` for option — as `demos/menu.rs` does; the rule is
//! documented on `Menu::shortcut`. The marks the face does have are kept, so
//! `⇧` is a real shift glyph and `Cmd ⇧ P` is a chord of one word and two marks.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::kbd::{Kbd, sequence};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's `###` headings — have them. The index is what the page is
/// asked to scroll to.
const SECTIONS: [&str; 3] = [
    "Shortcuts",
    "Multiple Modifiers",
    "Arrow Keys and Function Keys",
];

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — kbd")
        .theme(theme)
        .run()
}

/// Which section heading the pointer is over, and which was pressed, so the code
/// icon can draw the two states the Slint heading animates to.
#[derive(Debug, Clone, Default)]
struct App {
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::HoverSource(title) => app.hovered = title,
        Message::PressSource(title) => app.pressed_source = title,
        Message::ShowSource { index, title } => {
            app.pressed_source = None;
            // The page scrolls to the sample; natively this does nothing.
            crate::bridge::show(index, title);
        }
    }
    Task::none()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box the other galleries draw.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(text(title).size(theme::FONT_SIZE_SM).font(iced::Font {
            weight: iced::font::Weight::Medium,
            ..iced::Font::DEFAULT
        }))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .style(|theme: &Theme| container::Style {
            text_color: Some(Tokens::of(theme).muted_foreground),
            ..container::Style::default()
        }),
    )
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    let icon = mouse_area(
        container(
            svg(svg::Handle::from_memory(CODE_XML))
                .width(16.0)
                .height(16.0)
                .style(|theme: &Theme, _status| svg::Style {
                    color: Some(Tokens::of(theme).primary),
                }),
        )
        .width(28.0)
        .height(28.0)
        .center_x(28.0)
        .center_y(28.0)
        .style(move |theme: &Theme| {
            let primary = Tokens::of(theme).primary;
            let (fill, line) = if pressed {
                (0.34, 0.7)
            } else if hovered {
                (0.22, 0.55)
            } else {
                (0.12, 0.35)
            };
            container::Style {
                background: Some(primary.scale_alpha(fill).into()),
                border: iced::border::rounded(theme::RADIUS_LG)
                    .color(primary.scale_alpha(line))
                    .width(1.0),
                ..container::Style::default()
            }
        }),
    )
    .on_enter(Message::HoverSource(Some(title)))
    .on_exit(Message::HoverSource(None))
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    row![label, icon]
        .spacing(8.0)
        .align_y(Alignment::Center)
        .into()
}

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn panel<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A titled section: the heading row, then the sample it names.
fn section<'a>(
    app: &App,
    index: usize,
    title: &'static str,
    body: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![heading(app, index, title), body.into()]
        .spacing(8)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Shortcuts
    // The Slint example's first box: a chord on the default cap, and a longer
    // one whose words are spelled because the wasm face has no `⌘`.
    let shortcuts = panel(
        column![sequence(["Cmd", "K"]), sequence(["Ctrl", "Shift", "P"]),]
            .spacing(8)
            .align_x(Alignment::Start),
    );

    // section: Multiple Modifiers
    // Three chords: marks the face carries beside words it does not, and the
    // same chord again at the small step, which is what an inline hint uses.
    let modifiers = panel(
        column![
            sequence(["Cmd", "⇧", "P"]),
            sequence(["Ctrl", "Alt", "Delete"]),
            row![
                Kbd::<Message>::new("Cmd").size(ControlSize::Small),
                Kbd::<Message>::new("K").size(ControlSize::Small),
            ]
            .spacing(4.0)
            .align_y(Alignment::Center),
        ]
        .spacing(8)
        .align_x(Alignment::Start),
    );

    // section: Arrow Keys and Function Keys
    // The arrows are real glyphs in the wasm face; the named keys are spelled
    // because `⎋` and `⏎` are not in it.
    let arrows = panel(sequence(["↑", "↓", "←", "→", "Enter", "Esc"]));

    // section: end
    let gallery = column![
        section(app, 0, SECTIONS[0], shortcuts),
        section(app, 1, SECTIONS[1], modifiers),
        section(app, 2, SECTIONS[2], arrows),
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The page itself is a control, so a press outside a panel lands on a widget
    // rather than on the bare window.
    container(scrollable(gallery).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            ..container::Style::default()
        })
        .into()
}

// The Lucide `</>` mark the code icon uses, from the icon set this repository
// already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three
// galleries draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
