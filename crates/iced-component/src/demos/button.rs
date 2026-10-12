//! `button` — the whole gallery the Slint version shows: the variants, the
//! outline set, the four sizes, icons and states, button groups, progress,
//! dropdowns, a custom size, a custom colour and icon-only buttons, each in its
//! own outlined section.
//!
//! Run it with `cargo run --example button`, or in the browser as
//! `?story=button` in the gallery the website embeds.
//!
//! Two things Slint's `Button` does that this one cannot, both visible below:
//! `suffix-icon` draws a glyph at the trailing edge, and `ButtonGroup` squares
//! the corners where two buttons meet. iced's button carries one icon and draws
//! it before the label, and every button rounds its own corners, so the dropdown
//! chevrons lead and a group is buttons set against one another rather than
//! joined into one control. Both are Slint behaviour this port does not have;
//! neither is faked here.

use iced::widget::{column, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::{self, Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets
/// them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — button")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// What was pressed last, so a click is visible. Every button in the gallery
    /// carries a message, because a button without one is how iced draws a
    /// disabled control and these are the enabled ones; the single disabled
    /// button below is the one that has none.
    pressed: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Pressed(&'static str),
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Pressed(label) => app.pressed = Some(label),
    }
    Task::none()
}

/// A live control, labelled and ready to report a press.
fn control(label: &'static str) -> Button<'static, Message> {
    Button::new(label).on_press(Message::Pressed(label))
}

/// A control with no label: the icon is the whole control, which is what Slint's
/// `icon`-without-`text` draws.
fn icon_control(
    name: &'static str,
    markup: &'static [u8],
    variant: Variant,
    outline: bool,
) -> Button<'static, Message> {
    let control = Button::new("")
        .icon(glyph(markup, variant, outline))
        .variant(variant)
        .on_press(Message::Pressed(name));
    if outline {
        control.outline()
    } else {
        control
    }
}

/// A glyph from the set the Slint example names — `icon: "search"` there, the
/// same Lucide mark here. iced ships no icon set, so the artwork is the caller's,
/// and it is tinted like the label beside it: the ink of the button's own
/// variant, resolved from the theme when the glyph is drawn rather than captured
/// when the element was built.
fn glyph<'a>(markup: &'static [u8], variant: Variant, outline: bool) -> Element<'a, Message> {
    let tone = move |theme: &Theme| button::ink(Tokens::of(theme), variant, outline);
    svg(svg::Handle::from_memory(markup))
        .width(16.0)
        .height(16.0)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(tone(theme)),
        })
        .into()
}

/// One section, titled and outlined, the way the Slint example wraps each of its
/// groups in a `GroupBox`.
fn section<'a>(title: &'static str, body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .title(title)
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    let variants = column![
        row![
            control("Default"),
            control("Primary").variant(Variant::Primary),
            control("Secondary").variant(Variant::Secondary),
            control("Danger").variant(Variant::Danger),
            control("Warning").variant(Variant::Warning),
            control("Success").variant(Variant::Success),
            control("Info").variant(Variant::Info),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
        row![
            control("Ghost").variant(Variant::Ghost),
            control("Link").variant(Variant::Link),
            control("Text").variant(Variant::Text),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    ]
    .spacing(8.0);

    let outline = row![
        control("Primary").variant(Variant::Primary).outline(),
        control("Secondary").variant(Variant::Secondary).outline(),
        control("Danger").variant(Variant::Danger).outline(),
        control("Warning").variant(Variant::Warning).outline(),
        control("Success").variant(Variant::Success).outline(),
        control("Info").variant(Variant::Info).outline(),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    // 20, 24, 32 and 40 pixels tall, centred on one another so the row reads as
    // a ladder rather than as four controls on a baseline.
    let sizes = row![
        control("Extra small").size(ControlSize::XSmall),
        control("Small").size(ControlSize::Small),
        control("Medium").size(ControlSize::Medium),
        control("Large").size(ControlSize::Large),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    let icons_and_states = row![
        control("Search").icon(glyph(SEARCH, Variant::Default, false)),
        // Slint puts this one on the trailing edge; see the module note.
        control("Options").icon(glyph(CHEVRON_DOWN, Variant::Default, false)),
        icon_control("Settings", SETTINGS, Variant::Ghost, false),
        icon_control("Add", PLUS, Variant::Primary, false),
        control("Saving").loading(true),
        control("Selected").selected(true),
        // No message, so iced draws it disabled. This is the only control in the
        // gallery that is inert on purpose.
        Button::new("Disabled"),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    // Touching, not joined: each button keeps its own radius, where Slint's
    // `group-start`/`group-end` square the shared corners.
    let groups = row![
        row![
            control("One").outline(),
            control("Two").outline(),
            control("Three").outline(),
        ]
        .spacing(0.0)
        .align_y(Alignment::Center),
        column![
            control("One").outline(),
            control("Two").outline(),
            control("Three").outline(),
        ]
        .spacing(0.0),
    ]
    .spacing(16.0)
    .align_y(Alignment::Center);

    let progress = row![
        control("Installing").variant(Variant::Primary).loading(true),
        control("Installing").loading(true),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    let dropdown = row![
        control("Primary")
            .variant(Variant::Primary)
            .icon(glyph(CHEVRON_DOWN, Variant::Primary, false)),
        control("Default").icon(glyph(CHEVRON_DOWN, Variant::Default, false)),
        control("Ghost")
            .variant(Variant::Ghost)
            .icon(glyph(CHEVRON_DOWN, Variant::Ghost, false)),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    let custom_size = row![icon_control("Favourite", HEART, Variant::Ghost, false)
        .size(ControlSize::XSmall)]
    .align_y(Alignment::Center);

    let custom_color = row![
        control("Custom Button").variant(Variant::Success),
        control("Outline Button").variant(Variant::Success).outline(),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    let icon_only = row![
        icon_control("Search", SEARCH, Variant::Primary, false),
        icon_control("Info", INFO, Variant::Default, false),
        icon_control("Close", X, Variant::Danger, false),
        icon_control("Search outlined", SEARCH, Variant::Default, true),
        icon_control("Back", ARROW_LEFT, Variant::Ghost, false),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    let status = text(match app.pressed {
        Some(label) => format!("Last pressed: {label}"),
        None => "Press any control but the disabled one — the gallery is live.".to_string(),
    });

    let gallery = column![
        section("Variants", variants),
        section("Outline", outline),
        section("Sizes", sizes),
        section("Icons and states", icons_and_states),
        section("Button group", groups),
        section("Progress", progress),
        section("Dropdown", dropdown),
        section("Custom size", custom_size),
        section("Custom color", custom_color),
        section("Icon only", icon_only),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The marks the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so
// the two galleries draw the same glyphs rather than two lookalikes.

const SEARCH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21 21-4.34-4.34"/><circle cx="11" cy="11" r="8"/></svg>"#;

const SETTINGS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"/><circle cx="12" cy="12" r="3"/></svg>"#;

const PLUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>"#;

const X: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>"#;

const INFO: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#;

const CHEVRON_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#;

const ARROW_LEFT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>"#;

const HEART: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5"/></svg>"#;
