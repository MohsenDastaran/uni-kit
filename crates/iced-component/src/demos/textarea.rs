//! `textarea` — the multi-line field's gallery the way the Slint example shows
//! it: the basic value and placeholder, a field that grows with its content, the
//! value and the events that carry it, the size ladder, the normal, read-only,
//! invalid and disabled states, and the token composer that frames a borderless
//! field of its own.
//!
//! Run it with `cargo run --example textarea`, or in the browser as
//! `?story=textarea` in the gallery the website embeds.
//!
//! Each section carries a heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! **Two sections cannot mean the same thing here, and the cards say so rather
//! than approximating:**
//!
//! - **Auto-grow** is real in iced, where the Slint component says it cannot
//!   measure itself. `Length::Shrink` sizes the field to its content, so
//!   `auto_grow` genuinely grows between two row counts and then scrolls.
//! - **Atomic inline tokens** have no iced equivalent at all: the content is one
//!   plain string, so a reference cannot live inside it as a token that the
//!   caret and deletion move as a unit. The card builds the Slint composer —
//!   chips on their own line above a borderless field — and reports the text and
//!   the references apart, which is the closest real thing.

use iced::widget::text_editor::{Action, Content};
use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Border, Color, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::textarea::Textarea;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The references the composer's message carries. Slint cannot join an array,
/// so its example lists these as a fixed string; Rust can, and the readout below
/// does.
const REFERENCES: &[&str] = &["src/main.rs", "tests/dock.rs", "Cargo.toml"];

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — textarea")
        .theme(theme)
        .run()
}

#[derive(Default)]
struct App {
    /// One `Content` per example, because iced keeps the text, the caret and the
    /// selection in it and the caller owns it: every edit comes back through
    /// `on_action` and is applied with `perform`.
    bio: Content,
    notes: Content,
    draft: Content,
    normal: Content,
    readonly: Content,
    invalid: Content,
    disabled: Content,
    small: Content,
    medium: Content,
    large: Content,
    message: Content,
    /// The kind of the last action the events field reported, so a keystroke is
    /// visible as well as its result.
    last_event: Option<&'static str>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Bio(Action),
    Notes(Action),
    Draft(Action),
    Normal(Action),
    Readonly(Action),
    Invalid(Action),
    Small(Action),
    Medium(Action),
    Large(Action),
    Message(Action),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot() -> App {
    App {
        bio: Content::with_text("I build desktop tools in Rust and iced."),
        notes: Content::with_text("Dock now keeps every toolbar chip on one baseline."),
        invalid: Content::with_text("missing closing brace"),
        readonly: Content::with_text("Generated from the manifest."),
        message: Content::with_text("Please review the dock changes before the release."),
        ..App::default()
    }
}

/// The name of an action, for the events readout: an insert, a paste, a caret
/// move, a selection. `Action`'s own `Debug` prints the text a paste carried,
/// which is more than a line under the field can hold.
fn event_name(action: &Action) -> &'static str {
    match action {
        Action::Edit(_) => "edit",
        Action::Move(_) => "move",
        Action::Select(_) => "select",
        Action::SelectWord => "select word",
        Action::SelectLine => "select line",
        Action::SelectAll => "select all",
        Action::Click(_) => "click",
        Action::Drag(_) => "drag",
        Action::Scroll { .. } => "scroll",
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Bio(action) => app.bio.perform(action),
        Message::Notes(action) => app.notes.perform(action),
        Message::Draft(action) => {
            app.last_event = Some(event_name(&action));
            app.draft.perform(action);
        }
        Message::Normal(action) => app.normal.perform(action),
        // Read-only: the field never publishes an edit, so whatever arrives is a
        // motion or a selection and applying it is safe.
        Message::Readonly(action) => app.readonly.perform(action),
        Message::Invalid(action) => app.invalid.perform(action),
        Message::Small(action) => app.small.perform(action),
        Message::Medium(action) => app.medium.perform(action),
        Message::Large(action) => app.large.perform(action),
        Message::Message(action) => app.message.perform(action),
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

/// A muted line under an example, carrying what the demo has to say about it.
fn muted_line<'a>(line: &str) -> Element<'a, Message> {
    text(line.to_string())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// One reference in the composer's row: the file glyph and the name on a
/// `secondary` chip, the shape the Slint example draws. It is an element beside
/// the value, not part of it.
fn token_chip<'a>(name: &'a str) -> Element<'a, Message> {
    container(
        row![
            svg(svg::Handle::from_memory(FILE))
                .width(12.0)
                .height(12.0)
                .style(|theme: &Theme, _status| svg::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
            text(name)
                .size(theme::FONT_SIZE_XS)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
        ]
        .spacing(4.0)
        .align_y(Alignment::Center),
    )
    .padding([3.0, 6.0])
    .style(move |theme: &Theme| {
        let tokens = Tokens::of(theme);
        container::Style {
            background: Some(tokens.secondary.into()),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: theme::RADIUS_SM.into(),
            },
            ..container::Style::default()
        }
    })
    .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(
            text(title)
                .size(theme::FONT_SIZE_SM)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..iced::Font::DEFAULT
                }),
        )
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

/// The box under a heading: the same `GroupBox` with the outline variant the Slint
/// example wraps its sections in, untitled because the heading above it carries the
/// title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic usage
    let basic = column![
        Textarea::new(&app.bio)
            .id("bio")
            .placeholder("Tell us about yourself…")
            .on_action(Message::Bio),
    ]
    .spacing(12.0);

    // section: Auto-grow
    // `Length::Shrink` measures the content, so this field really grows; the
    // Slint component cannot measure itself and draws a fixed taller box. Past
    // the maximum it scrolls, which is the textarea's counterpart of max_rows.
    let auto_grow = column![
        Textarea::new(&app.notes)
            .placeholder("Release notes…")
            .auto_grow(2, 8)
            .on_action(Message::Notes),
    ]
    .spacing(12.0);

    // section: Value and events
    // The caller owns the `Content`: `on_action` reports every interaction, the
    // caller applies it with `perform`, and the text is read back from the same
    // value. Reading is `content.text()`, setting is a fresh `Content`.
    let events = column![
        Textarea::new(&app.draft)
            .placeholder("Type a note…")
            .rows(4)
            .on_action(Message::Draft),
        muted_line(&format!(
            "Draft: {}",
            if app.draft.is_empty() {
                "(empty)".to_string()
            } else {
                app.draft.text()
            }
        )),
        muted_line(&format!(
            "Last event: {}",
            app.last_event.unwrap_or("nothing yet")
        )),
    ]
    .spacing(8.0);

    // section: Sizes
    // The size steps the text size and the padding together; each one stands
    // three of that size's control heights tall unless `rows` or `auto_grow`
    // says otherwise.
    let sizes = column![
        Textarea::new(&app.small)
            .size(ControlSize::Small)
            .placeholder("Small")
            .on_action(Message::Small),
        Textarea::new(&app.medium)
            .size(ControlSize::Medium)
            .placeholder("Medium")
            .on_action(Message::Medium),
        Textarea::new(&app.large)
            .size(ControlSize::Large)
            .placeholder("Large")
            .on_action(Message::Large),
    ]
    .spacing(12.0);

    // section: Appearance
    // The four states the Slint component draws. Read-only swallows the key
    // bindings that edit and keeps the frame, the caret and the selection;
    // disabled is the absence of a message, which is how this crate spells a
    // control that cannot be used.
    let appearance = column![
        Textarea::new(&app.normal)
            .placeholder("Normal")
            .on_action(Message::Normal),
        Textarea::new(&app.readonly)
            .readonly(true)
            .on_action(Message::Readonly),
        Textarea::new(&app.invalid)
            .invalid(true)
            .on_action(Message::Invalid),
        Textarea::new(&app.disabled).placeholder("Disabled"),
    ]
    .spacing(12.0);

    // section: Atomic inline tokens
    // iced's `Content` is one plain string, so a reference cannot live inside it
    // as a token that the caret and deletion move as one. This is the Slint
    // composer instead: chips on their own line above a borderless field, and
    // the text and the references reported apart.
    let composer = container(
        column![
            row(REFERENCES.iter().copied().map(token_chip))
                .spacing(6.0)
                .align_y(Alignment::Center),
            Textarea::new(&app.message)
                .appearance(false)
                .placeholder("Write a message…")
                .height(Length::Fill)
                .on_action(Message::Message),
        ]
        .spacing(8.0),
    )
    .padding(8.0)
    .width(Length::Fill)
    .height(Length::Fixed(150.0))
    .style(|theme: &Theme| {
        let tokens = Tokens::of(theme);
        container::Style {
            background: Some(tokens.background.into()),
            border: Border {
                color: tokens.input,
                width: 1.0,
                radius: theme::RADIUS_MD.into(),
            },
            ..container::Style::default()
        }
    });

    let tokens = column![
        composer,
        muted_line(&format!(
            "Text: {}",
            if app.message.is_empty() {
                "(empty)".to_string()
            } else {
                app.message.text()
            }
        )),
        muted_line(&format!("References: {}", REFERENCES.join(" · "))),
    ]
    .spacing(10.0);

    // section: end
    let gallery = column![
        heading(app, 0, "Basic usage"),
        body_box(basic),
        heading(app, 1, "Auto-grow"),
        body_box(auto_grow),
        heading(app, 2, "Value and events"),
        body_box(events),
        heading(app, 3, "Sizes"),
        body_box(sizes),
        heading(app, 4, "Appearance"),
        body_box(appearance),
        heading(app, 5, "Atomic inline tokens"),
        body_box(tokens),
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The marks the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the
// two galleries draw the same glyphs rather than two lookalikes.

const FILE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
