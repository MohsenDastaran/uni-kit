//! `empty` — an empty-state panel with a media slot, a title, a description and
//! up to two actions.
//!
//! Run it with `cargo run --example empty`, or in the browser as
//! `?story=empty` in the gallery the website embeds.
//!
//! The Slint example's seven outlined sections — Basic usage, Outline,
//! Background, Avatar, Avatar group, Inputs and custom content and Constrained
//! layouts — are the seven here, in the same order, each under the `##` heading
//! `website/component/empty.md` documents it at. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! Each heading carries the title and the code icon that asks the embedding page
//! to scroll to that sample. Both are the button, as in Slint's `SourceHeading`,
//! and a click posts the section's index and title through [`crate::bridge`],
//! the same bridge the Slint and GPUI galleries use.
//!
//! Where iced differs from Slint the difference is stated rather than faked.
//! Slint's `Empty` has a free `@children` slot under its action row; this one
//! ends at that row, so the Basic usage sample's "Learn more" line sits as a
//! sibling beneath the panel and the Inputs sample puts its field and hint in
//! the row. Slint's `AvatarGroup` overlaps its avatars with negative spacing;
//! iced has none, so the group is a spaced row. The media is the caller's own
//! element — Slint names an icon and builds avatars from initials — so the demo
//! embeds the same Lucide marks the Slint example names and the same two-letter
//! avatars.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::avatar::{Avatar, AvatarSize};
use crate::components::button::{Button, Variant};
use crate::components::empty::{Align, Empty, Media};
use crate::components::group_box::{self, GroupBox};
use crate::components::icon::Icon;
use crate::components::input::Input;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = theme::SPACING_XL;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's `##` headings — have them. The index is what the page is
/// asked to scroll to.
const SECTIONS: [&str; 7] = [
    "Basic usage",
    "Outline",
    "Background",
    "Avatar",
    "Avatar group",
    "Inputs and custom content",
    "Constrained layouts",
];

pub fn run(theme: Theme) -> iced::Result {
    // The palette in force at boot is the one the Background sample draws its
    // explicit fill from. Embedded, the gallery has already parked the page's
    // own palette, so this is the page's set; native, and standalone in a
    // browser, it is the built-in light or dark table.
    let tokens = Tokens::of(&theme);
    iced::application(move || boot(tokens), update, view)
        .title("uni-kit — empty")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone)]
struct App {
    /// The search field's value: iced has no input state to own it, so the demo
    /// holds it and takes every edit back through `on_input`.
    search: String,
    /// The tokens the view names a colour from explicitly — the Background
    /// sample's muted fill.
    tokens: Tokens,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// The search field was edited.
    Search(String),
    /// A sample action with nothing behind it, as in the component page's own
    /// snippets.
    Nothing,
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot(tokens: Tokens) -> App {
    App {
        search: String::new(),
        tokens,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Search(value) => app.search = value,
        Message::Nothing => {}
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

/// An icon the application supplies, drawn in the theme's foreground, which is
/// what the Slint `Icon` defaults its colour to.
fn mark<'a>(handle: &'static [u8], pixels: f32) -> Element<'a, Message> {
    Icon::new(svg::Handle::from_memory(handle))
        .size(pixels)
        .into()
}

/// An avatar stand-in, at the Slint example's medium edge. iced has no
/// `AvatarGroup`, so a group is a row of these.
fn initials<'a>(value: &'static str) -> Element<'a, Message> {
    Avatar::new()
        .initials(value)
        .size(AvatarSize::Medium)
        .into()
}

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A muted line: the supporting copy the Slint panels draw in the muted
/// foreground.
fn muted<'a>(value: &'static str) -> Element<'a, Message> {
    text(value)
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
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
        .spacing(theme::SPACING_SM)
        .align_y(Alignment::Center)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic usage
    // The page's first documented sample and the Slint example's first
    // `GroupBox`: a framed icon, a title, a description and two actions. Slint
    // keeps its "Learn more" line in the panel's free children slot; this
    // `Empty` ends at the action row, so the line is a sibling beneath it.
    let basic = body_box(
        column![
            Empty::new()
                .media_slot(mark(FOLDER, 16.0))
                .title("No projects yet")
                .description("You haven't created any projects yet. Get started by creating your first project.")
                .action(
                    Button::new("Create project")
                        .variant(Variant::Primary)
                        .on_press(Message::Nothing),
                )
                .secondary_action(
                    Button::new("Import sample")
                        .outline()
                        .on_press(Message::Nothing),
                ),
            container(
                text("Learn more")
                    .size(theme::FONT_SIZE_SM)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).primary),
                    }),
            )
            .width(Length::Fill)
            .center_x(Length::Fill),
        ]
        .spacing(theme::SPACING_SM),
    );

    // section: Outline
    // The panel's one-pixel border in the theme's border colour. Slint cannot
    // draw a dashed border and uses the solid one; iced keeps Slint's answer,
    // which is also what GPUI draws.
    let outline = body_box(
        Empty::new()
            .bordered(true)
            .media_slot(mark(BOX, 16.0))
            .title("Cloud storage is empty")
            .description("Upload files to your cloud storage to access them anywhere.")
            .secondary_action(
                Button::new("Upload files…")
                    .outline()
                    .on_press(Message::Nothing),
            ),
    );

    // section: Background
    // A semantic surface on the panel itself: the Slint example's
    // `Theme.muted.with-alpha(0.35)`, named from the theme the application
    // booted with rather than written by hand.
    let background = body_box(
        Empty::new()
            .background(app.tokens.muted.scale_alpha(0.35))
            .media_slot(mark(BELL, 16.0))
            .title("No notifications")
            .description("You're all caught up. New notifications will appear here.")
            .secondary_action(
                Button::new("Refresh")
                    .outline()
                    .on_press(Message::Nothing),
            ),
    );

    // section: Avatar
    // One avatar in the bare media slot: Slint's `avatars: ["AM"]`, which
    // replaces the framed icon with an avatar at its own size.
    let avatar = body_box(
        Empty::new()
            .media(Media::Plain)
            .media_slot(initials("AM"))
            .title("Alex is offline")
            .description("You can leave a message for Alex to read when they're back.")
            .action(
                Button::new("Leave a message…")
                    .variant(Variant::Primary)
                    .on_press(Message::Nothing),
            ),
    );

    // section: Avatar group
    // Several avatars in the same slot, which the group owns the overlap and
    // size of. Slint's `AvatarGroup` overlaps with negative spacing; iced has
    // none, so the group is a spaced row.
    let group = body_box(
        Empty::new()
            .media(Media::Plain)
            .media_slot(
                row![initials("AM"), initials("TL"), initials("SC")]
                    .spacing(theme::SPACING_XS)
                    .align_y(Alignment::Center),
            )
            .title("No team members")
            .description("Invite your team to collaborate on this project.")
            .action(
                Button::new("Invite members…")
                    .variant(Variant::Primary)
                    .on_press(Message::Nothing),
            ),
    );

    // section: Inputs and custom content
    // The field and its hint in the row this `Empty` draws under the
    // description, which is where Slint's free children go. The application
    // owns the value and takes every edit back through `on_input`.
    let inputs = body_box(
        Empty::new()
            .media(Media::None)
            .title("Looking for a project?")
            .description("The page you're looking for doesn't exist. Try searching for another page.")
            .action(
                column![
                    Input::new("empty-search")
                        .placeholder("Search projects")
                        .prefix(mark(SEARCH, 16.0))
                        .value(app.search.clone())
                        .width(Length::Fill)
                        .on_input(Message::Search),
                    container(muted("Search by name or try a different keyword."))
                        .width(Length::Fill)
                        .center_x(Length::Fill),
                ]
                .spacing(theme::SPACING_SM)
                .width(Length::Fixed(320.0)),
            ),
    );

    // section: Constrained layouts
    // A 340px panel at the tighter 16px inset, leading-aligned with a 300px
    // text column: the Slint example's `preferred-width`, `align: start` and
    // `text-max-width` refined on the root and the header together.
    let constrained = body_box(
        container(
            Empty::new()
                .bordered(true)
                .padding(16.0)
                .text_max_width(300.0)
                .align(Align::Start)
                .media_slot(mark(INBOX, 16.0))
                .title("No shared files")
                .description("Add files so your team can review and edit them together.")
                .secondary_action(
                    Button::new("Add files…")
                        .outline()
                        .on_press(Message::Nothing),
                ),
        )
        .width(Length::Fixed(340.0)),
    );

    // section: end
    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        basic,
        heading(app, 1, SECTIONS[1]),
        outline,
        heading(app, 2, SECTIONS[2]),
        background,
        heading(app, 3, SECTIONS[3]),
        avatar,
        heading(app, 4, SECTIONS[4]),
        group,
        heading(app, 5, SECTIONS[5]),
        inputs,
        heading(app, 6, SECTIONS[6]),
        constrained,
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

// The Lucide marks the Slint example names, from the icon set this repository
// already ships (`crates/slint-component/ui/icons`, Lucide, ISC).
const FOLDER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/></svg>"#;
const BOX: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"/><path d="m3.3 7 8.7 5 8.7-5"/><path d="M12 22V12"/></svg>"#;
const BELL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"/></svg>"#;
const INBOX: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/></svg>"#;
const SEARCH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21 21-4.34-4.34"/><circle cx="11" cy="11" r="8"/></svg>"#;

// The Lucide mark the code icon uses, from the icon set this repository already
// ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three galleries
// draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
