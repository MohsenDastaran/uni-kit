//! `input-group` — one frame around a field and its addons, the way the Slint
//! example shows it: a clear control, the alignment of addons before and after
//! the field, buttons and an icon menu trigger, the textarea with its counter
//! and submit row beside the track, the disabled, invalid and read-only states,
//! the four sizes, and the inline reference iced cannot hold inside the value.
//!
//! Run it with `cargo run --example input_group`, or in the browser as
//! `?story=input-group` in the gallery the website embeds.
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
//! The page's headings are the target, and one of them — **JavaScript** — has
//! no Rust sample to cut and is therefore sample-less rather than faked: the
//! port is Rust, and the page's JavaScript block documents the GPUI
//! `gpui-component` binding, which has no iced counterpart.
//!
//! **Three Slint behaviours cannot mean the same thing here, and the cards say
//! so rather than approximating:**
//!
//! - **Read-only** has no single-line iced field. `Textarea` has `readonly`;
//!   `Input` either accepts edits or is disabled, so the read-only card draws a
//!   live field and names what is missing.
//! - **Clicking an addon does not focus the field.** iced reports a press to the
//!   element that was pressed, and the field is not that element, so an addon
//!   has no focus path to it. The field's own `prefix` slot still sits inside
//!   the frame and takes the click.
//! - **A menu trigger opens nothing here.** The Slint example's ellipsis addon
//!   is a plain ghost button too, and the group carries no menu state; a real
//!   dropdown is the `menu` component's, composed by the caller.
//!
//! One more is worth stating because it is a shape, not a gap: the group's track
//! is one fixed-height row, so the taller `Textarea` stays outside it, exactly as
//! the Slint example leaves it, with its counter and submit row beside the track.

use iced::widget::text_editor::{Action, Content};
use iced::widget::{Space, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Border, Color, Element, Length, Task, Theme};

use crate::components::button::{self as button, Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::input::Input;
use crate::components::input_group::{InputGroup, InputGroupAddon};
use crate::components::textarea::Textarea;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — input-group")
        .theme(theme)
        .run()
}

#[derive(Default)]
struct App {
    /// One field per editable example: iced holds no input state of its own, so
    /// the caller owns each value and takes every edit back through `on_input`.
    search: String,
    site: String,
    find: String,
    query: String,
    menu: String,
    small: String,
    medium: String,
    large: String,
    email: String,
    mention: String,
    /// The textarea's content, which iced keeps whole — text, caret and
    /// selection — and reports back as an `Action`.
    reply: Content,
    /// The last addon control that was pressed, so a click is visible.
    notice: Option<&'static str>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    SearchChanged(String),
    SiteChanged(String),
    FindChanged(String),
    QueryChanged(String),
    MenuChanged(String),
    SmallChanged(String),
    MediumChanged(String),
    LargeChanged(String),
    EmailChanged(String),
    MentionChanged(String),
    Reply(Action),
    ClearSearch,
    Notice(&'static str),
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
        // A value, so the clear control and its seam are drawn in the gallery as
        // well as in an application that has typed something.
        search: "dock".to_string(),
        site: "gpui-kit.com".to_string(),
        email: "not-an-address".to_string(),
        reply: Content::with_text("Dock now keeps every toolbar chip on one baseline."),
        ..App::default()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::SearchChanged(value) => app.search = value,
        Message::SiteChanged(value) => app.site = value,
        Message::FindChanged(value) => app.find = value,
        Message::QueryChanged(value) => app.query = value,
        Message::MenuChanged(value) => app.menu = value,
        Message::SmallChanged(value) => app.small = value,
        Message::MediumChanged(value) => app.medium = value,
        Message::LargeChanged(value) => app.large = value,
        Message::EmailChanged(value) => app.email = value,
        Message::MentionChanged(value) => app.mention = value,
        Message::Reply(action) => app.reply.perform(action),
        Message::ClearSearch => app.search.clear(),
        Message::Notice(label) => app.notice = Some(label),
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

/// A glyph for a field's `prefix` slot or an addon, in the muted colour both the
/// Slint and GPUI examples draw a slot in. iced ships no icon set, so the artwork
/// is the caller's; this is the same Lucide set the Slint example names, and the
/// style closure colours it — an SVG keeps its own black otherwise, which
/// disappears in the dark theme.
fn glyph<'a>(markup: &'static [u8]) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(14.0)
        .height(14.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// The same glyph in the danger colour, for the invalid card's trailing mark.
fn danger_glyph<'a>(markup: &'static [u8]) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(14.0)
        .height(14.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).danger),
        })
        .into()
}

/// The same glyph inside a ghost control, tinted with the ink that control draws
/// its label in rather than the muted slot colour. It is 14px at the compact
/// size, which is Slint's `icon-size` for the addon buttons the group carries.
fn glyph_ink<'a>(markup: &'static [u8]) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(14.0)
        .height(14.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(button::ink(Tokens::of(theme), Variant::Ghost, false)),
        })
        .into()
}

/// A ghost compact icon control: the addon button the group carries. Every one
/// has a message, because a control without one is how iced draws a disabled
/// button.
fn icon_button<'a>(markup: &'static [u8], message: Message) -> Button<'a, Message> {
    Button::new("")
        .variant(Variant::Ghost)
        .size(ControlSize::Small)
        .icon(glyph_ink(markup))
        .on_press(message)
}

/// An addon of plain muted text, the shape Slint's `InputGroupAddon` holds.
fn text_addon<'a>(label: &'a str) -> InputGroupAddon<'a, Message> {
    InputGroupAddon::new().child(text(label.to_string()).size(theme::FONT_SIZE_SM))
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

/// A reference chip: the file glyph and the name on a `secondary` surface, the
/// shape the Slint example draws. It is a prefix element beside the value, not a
/// token inside it.
fn token_chip<'a>(name: &'a str) -> Element<'a, Message> {
    container(
        row![
            svg(svg::Handle::from_memory(FILE))
                .width(12.0)
                .height(12.0)
                .style(|theme: &Theme, _status| svg::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
            text(name.to_string())
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
    // section: Input with a clear button
    // The field keeps its own `prefix` slot, inside the frame; the clear control
    // is an addon after it, and appears only while there is something to clear.
    let mut search = InputGroup::new().input(
        Input::new("group-search")
            .value(app.search.as_str())
            .placeholder("Search the docs…")
            .prefix(glyph(SEARCH))
            .on_input(Message::SearchChanged),
    );
    if !app.search.is_empty() {
        search = search.addon_end(
            InputGroupAddon::new()
                .divider(true)
                .child(icon_button(X, Message::ClearSearch)),
        );
    }
    let clear = column![
        search,
        muted_line(&if app.search.is_empty() {
            "The clear control appears once there is something to clear.".to_string()
        } else {
            format!("Value: {}", app.search)
        }),
    ]
    .spacing(8.0);

    // section: Parts and alignment
    // Addons before and after the field sit in one track. Slint's group is a
    // single horizontal row, so the block sides GPUI's part has are the caller's
    // own column rather than a side of this component.
    let parts = column![
        InputGroup::new()
            .input(
                Input::new("group-site")
                    .value(app.site.as_str())
                    .placeholder("gpui-kit.com")
                    .on_input(Message::SiteChanged),
            )
            .addon_start(text_addon("https://"))
            .addon_end(text_addon(".com")),
        InputGroup::new()
            .input(
                Input::new("group-find")
                    .value(app.find.as_str())
                    .placeholder("Search")
                    .on_input(Message::FindChanged),
            )
            .addon_start(InputGroupAddon::new().child(glyph(SEARCH)))
            .addon_end(text_addon("Ctrl K")),
    ]
    .spacing(12.0);

    // section: Buttons, icons, and menus
    // A primary action fills the trailing edge; the ellipsis is the compact ghost
    // control the Slint example draws. It opens nothing here — the group carries
    // no menu state, and a real dropdown belongs to the `menu` component.
    let buttons = column![
        InputGroup::new()
            .input(
                Input::new("group-query")
                    .value(app.query.as_str())
                    .placeholder("Search the docs…")
                    .on_input(Message::QueryChanged),
            )
            .addon_end(
                InputGroupAddon::new().child(
                    Button::new("Search")
                        .variant(Variant::Primary)
                        .on_press(Message::Notice("Search")),
                )
            ),
        InputGroup::new()
            .input(
                Input::new("group-menu")
                    .value(app.menu.as_str())
                    .placeholder("More actions…")
                    .on_input(Message::MenuChanged),
            )
            .addon_end(
                InputGroupAddon::new()
                    .divider(true)
                    .child(icon_button(ELLIPSIS, Message::Notice("More actions"))),
            ),
    ]
    .spacing(12.0);

    // section: Textarea with a counter and submit action
    // The group's track is one fixed-height row, so the taller textarea stays
    // outside it — which is what the Slint example does too. The Slint comment
    // says the frame is generic and a taller child works the same way; iced's
    // group has a definite height, so its field is the textarea's own frame.
    let count = app.reply.text().chars().count();
    let composer = column![
        Textarea::new(&app.reply)
            .placeholder("Write a reply…")
            .rows(3)
            .on_action(Message::Reply),
        row![
            text(format!("{count} / 280"))
                .size(theme::FONT_SIZE_XS)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
            Space::new().width(Length::Fill),
            Button::new("Reply")
                .variant(Variant::Primary)
                .on_press(Message::Notice("Reply")),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    ]
    .spacing(8.0);

    // section: Disabled, read-only, and validation
    // Validation stays visible while editing is disabled, the way the field's
    // own edge does. iced's single-line field has no read-only mode, so the
    // first card stays live and the note names what is missing.
    let states = column![
        InputGroup::new()
            .input(
                Input::new("group-readonly")
                    .value("copied from the registry")
                    .on_input(Message::EmailChanged),
            )
            .addon_end(
                InputGroupAddon::new()
                    .divider(true)
                    .child(icon_button(COPY, Message::Notice("Copy"))),
            ),
        InputGroup::new()
            .invalid(true)
            .input(
                Input::new("group-invalid")
                    .value(app.email.as_str())
                    .placeholder("you@example.com")
                    .on_input(Message::EmailChanged),
            )
            .addon_end(InputGroupAddon::new().child(danger_glyph(CIRCLE_X))),
        InputGroup::new()
            .disabled(true)
            .input(Input::new("group-disabled").placeholder("Disabled"))
            .addon_end(
                InputGroupAddon::new()
                    .divider(true)
                    .child(Button::new("Go")),
            ),
        muted_line("Read-only has no single-line iced field; the textarea is the one that has it."),
    ]
    .spacing(12.0);

    // section: Sizes and custom styles
    // The size sets the frame's height, the field's text size and the addons'
    // inset together. The frame follows the field's height, so a large group is
    // 44px rather than the 40px a button uses.
    let sizes = column![
        InputGroup::new()
            .size(ControlSize::Small)
            .input(
                Input::new("group-small")
                    .value(app.small.as_str())
                    .placeholder("Small")
                    .on_input(Message::SmallChanged),
            )
            .addon_end(
                InputGroupAddon::new().child(
                    Button::new("Go")
                        .size(ControlSize::Small)
                        .on_press(Message::Notice("Go")),
                )
            ),
        InputGroup::new()
            .size(ControlSize::Medium)
            .input(
                Input::new("group-medium")
                    .value(app.medium.as_str())
                    .placeholder("Medium")
                    .on_input(Message::MediumChanged),
            )
            .addon_end(
                InputGroupAddon::new().child(
                    Button::new("Go")
                        .variant(Variant::Default)
                        .on_press(Message::Notice("Go")),
                )
            ),
        InputGroup::new()
            .size(ControlSize::Large)
            .input(
                Input::new("group-large")
                    .value(app.large.as_str())
                    .placeholder("Large")
                    .on_input(Message::LargeChanged),
            )
            .addon_end(
                InputGroupAddon::new().child(
                    Button::new("Go")
                        .size(ControlSize::Large)
                        .on_press(Message::Notice("Go")),
                )
            ),
    ]
    .spacing(12.0);

    // section: Inline references
    // iced's field holds one plain `String`, so a reference cannot live inside
    // the value as a token the caret and deletion move as a unit. The chip is a
    // prefix element beside the value, not part of it.
    let references = column![
        InputGroup::new()
            .input(
                Input::new("group-mention")
                    .value(app.mention.as_str())
                    .placeholder("Ask @alice")
                    .prefix(token_chip("src/main.rs"))
                    .on_input(Message::MentionChanged),
            )
            .addon_end(
                InputGroupAddon::new().child(
                    Button::new("Send")
                        .variant(Variant::Primary)
                        .on_press(Message::Notice("Send")),
                )
            ),
        muted_line("The chip is a prefix element; `@alice` in the value is ordinary text."),
    ]
    .spacing(8.0);

    // section: end
    let status = text(match app.notice {
        Some(label) => format!("Last addon control: {label}"),
        None => "Type in a field, or press an addon control — the gallery is live.".to_string(),
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, "Input with a clear button"),
        body_box(clear),
        heading(app, 1, "Parts and alignment"),
        body_box(parts),
        heading(app, 2, "Buttons, icons, and menus"),
        body_box(buttons),
        heading(app, 3, "Textarea with a counter and submit action"),
        body_box(composer),
        heading(app, 4, "Disabled, read-only, and validation"),
        body_box(states),
        heading(app, 5, "Sizes and custom styles"),
        body_box(sizes),
        heading(app, 6, "Inline references"),
        body_box(references),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The marks the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the
// two galleries draw the same glyphs rather than two lookalikes.

const SEARCH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21 21-4.34-4.34"/><circle cx="11" cy="11" r="8"/></svg>"#;

const X: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>"#;

const ELLIPSIS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>"#;

const COPY: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>"#;

const CIRCLE_X: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/></svg>"#;

const FILE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
