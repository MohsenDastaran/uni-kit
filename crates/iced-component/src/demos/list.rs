//! `list` — a selectable list: rows of a title, a description and an optional
//! leading icon, on the styled scroll area.
//!
//! Run it with `cargo run --example list`, or in the browser as `?story=list`
//! in the gallery the website embeds.
//!
//! Each section carries the heading the website page gives it — **Basic List**,
//! **List Items with Icons and Actions**, **List with Search** — and the code
//! icon that asks the embedding page to scroll to that sample. Clicking either
//! one posts the section's index and title through [`crate::bridge`], the same
//! bridge the Slint and GPUI galleries use.
//!
//! The `// section: <title>` comments below are the contract the website reads to
//! cut this file into the usage panels a component page shows, so the code on the
//! page is the code that runs here. Keep them, one per section, in order, and keep
//! the closing `// section: end` after the last one.
//!
//! The headings are the three the Slint example fills. GPUI's page goes on to
//! sections, events, loading, infinite scroll, drag reordering and a file browser;
//! none of those are the Slint component's shape, so this port leaves them without
//! an iced sample rather than standing in for them — the module says the same
//! thing at the top of `list.rs`.

use iced::widget::container::Style as ContainerStyle;
use iced::widget::{column, container, mouse_area, row, svg, text, Space};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::icon::Icon;
use crate::components::list::{List, ListEntry};
use crate::components::scrollable::Scrollable;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The height of the first list's box, so its rows scroll under their own bar
/// while the section stays on screen. The other two lists size to their rows.
const BASIC_HEIGHT: f32 = 220.0;

/// A folder, the Lucide `folder` glyph, from `crates/slint-component/ui/icons/`.
const FOLDER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/></svg>"#;

/// A clock, the Lucide `clock` glyph.
const CLOCK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/></svg>"#;

/// Two people, the Lucide `users` glyph.
const USERS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><path d="M16 3.128a4 4 0 0 1 0 7.744"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><circle cx="9" cy="7" r="4"/></svg>"#;

/// A bin, the Lucide `trash-2` glyph.
const TRASH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 11v6"/><path d="M14 11v6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M3 6h18"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>"#;

/// A magnifier, the Lucide `search` glyph.
const SEARCH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21 21-4.34-4.34"/><circle cx="11" cy="11" r="8"/></svg>"#;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — list")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// The selected row of each list, `None` before anything is clicked. The
    /// starting values are the Slint example's own `selected:` properties.
    basic_selected: Option<usize>,
    icons_selected: Option<usize>,
    search_selected: Option<usize>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            basic_selected: Some(1),
            icons_selected: Some(0),
            search_selected: Some(0),
            hovered: None,
            pressed_source: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SelectBasic(usize),
    SelectIcons(usize),
    SelectSearch(usize),
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
        Message::SelectBasic(index) => app.basic_selected = Some(index),
        Message::SelectIcons(index) => app.icons_selected = Some(index),
        Message::SelectSearch(index) => app.search_selected = Some(index),
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

/// A leading icon at the row's 16px box, tinted the foreground the Slint list
/// asks for. [`Icon`] resolves that inside its own style closure, so the icon
/// follows the theme like every other part.
fn entry_icon<'a>(handle: &'static [u8]) -> Element<'a, Message> {
    Icon::new(svg::Handle::from_memory(handle))
        .size(ControlSize::Medium)
        .into()
}

/// The search field's glyph, in the muted foreground the Slint example draws it
/// in. It is a raw `svg` rather than an [`Icon`] because the colour has to be
/// resolved from the theme's tokens while the field renders.
fn search_glyph<'a>() -> Element<'a, Message> {
    svg(svg::Handle::from_memory(SEARCH))
        .width(Length::Fixed(14.0))
        .height(Length::Fixed(14.0))
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// The search field's rule, `Rectangle { height: 1px; background: Theme.border }`.
fn rule<'a>() -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(|theme: &Theme| ContainerStyle {
            background: Some(Tokens::of(theme).border.into()),
            ..ContainerStyle::default()
        })
        .into()
}

/// The field's surface: the page's fill inside a one-pixel border at
/// `radius-md`, resolved inside the closure so the dark theme is dark.
fn field_surface(theme: &Theme) -> ContainerStyle {
    let tokens = Tokens::of(theme);
    ContainerStyle {
        background: Some(tokens.background.into()),
        border: iced::border::rounded(theme::RADIUS_MD)
            .color(tokens.border)
            .width(1.0),
        ..ContainerStyle::default()
    }
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
/// example wraps its sections in, untitled because the heading above it carries
/// the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic List
    // Eight destinations, longer than the box so the rows move under their own
    // bar. `on_select` reports the row and the app keeps the selection, which is
    // the Slint `selected` / `changed` pair.
    let basic_entries = [
        ListEntry::new("Inbox").description("12 unread"),
        ListEntry::new("Sent").description("On this device"),
        ListEntry::new("Archive").description("Last 30 days"),
        ListEntry::new("Spam").description("Nothing reported"),
        ListEntry::new("Drafts").description("3 unsent"),
        ListEntry::new("Trash").description("Empties after 30 days"),
        ListEntry::new("Starred").description("8 conversations"),
        ListEntry::new("Snoozed").description("Until tomorrow"),
    ];
    let basic = List::new(basic_entries)
        .selected(app.basic_selected)
        .height(Length::Fixed(BASIC_HEIGHT))
        .on_select(Message::SelectBasic);

    // section: List Items with Icons and Actions
    // A leading icon turns a row into a destination. The artwork is the
    // application's, taken here from the same Lucide set the Slint icons folder
    // carries.
    let icon_entries = [
        ListEntry::new("Project files")
            .description("src/ and Cargo.toml")
            .icon(entry_icon(FOLDER)),
        ListEntry::new("Recent")
            .description("dock.slint, menu.slint")
            .icon(entry_icon(CLOCK)),
        ListEntry::new("Shared with me")
            .description("3 people")
            .icon(entry_icon(USERS)),
        ListEntry::new("Trash")
            .description("Empties after 30 days")
            .icon(entry_icon(TRASH)),
    ];
    let icons = List::new(icon_entries)
        .selected(app.icons_selected)
        .on_select(Message::SelectIcons);

    // section: List with Search
    // A field above the list is the usual shape. The field itself is an `Input`
    // in GPUI; iced has one too, but the Slint example mocks it so the focus stays
    // on the list, and this port keeps the mock rather than wiring an input whose
    // query filters nothing.
    let search_entries = [
        ListEntry::new("Button").description("Push button"),
        ListEntry::new("Badge").description("Status label"),
        ListEntry::new("Banner").description("Page notice"),
    ];
    let search_field = row![
        search_glyph(),
        text("Search components…")
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    ]
    .spacing(theme::SPACING_SM)
    .padding(theme::SPACING_SM)
    .align_y(Alignment::Center);

    let search = container(
        column![
            search_field,
            rule(),
            List::new(search_entries)
                .selected(app.search_selected)
                .on_select(Message::SelectSearch),
        ]
        .spacing(theme::SPACING_SM),
    )
    .style(field_surface);

    // section: end
    let status = text("Click a row to select it — press a heading or its code icon for that sample.")
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        });

    let gallery = column![
        heading(app, 0, "Basic List"),
        body_box(basic),
        heading(app, 1, "List Items with Icons and Actions"),
        body_box(icons),
        heading(app, 2, "List with Search"),
        body_box(search),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The gallery's own scroll area is the component the page documents, so the
    // frame is drawn by the thing under test.
    Scrollable::new(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
