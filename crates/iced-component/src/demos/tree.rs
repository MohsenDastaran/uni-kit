//! `tree` — a disclosure tree, shown the way the Slint example shows it: rows
//! indented by depth, a folder whose chevron flips while its children reveal,
//! and the caller's own open set and selection deciding what is drawn.
//!
//! Run it with `cargo run --example tree`, or in the browser as `?story=tree`
//! in the gallery the website embeds.
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
//! The sections are the Slint example's three `GroupBox` headings, in its order,
//! and their titles are the page's own `###` headings: **Basic Tree**, **File
//! Tree with Icons**, and **Tree with Selection Handling**. The page's other
//! usage headings describe GPUI's entity-backed `TreeState` — dynamic, lazy and
//! filtered loading, programmatic control, multi-select, disabled items — which
//! iced's caller-owned tree has no method for; they are left without an iced
//! sample rather than shown as a stand-in.

use std::collections::HashSet;

use iced::widget::{Space, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Border, Color, Element, Length, Padding, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, Tokens};
use crate::components::tree::{Node, Tree};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The settings tree's rows, in order: the id a row selects and its label. The
/// detail pane needs the label and the position, and the tree only reports the
/// id, so the demo keeps the list the way an application would.
const SETTINGS: [(&str, &str); 6] = [
    ("workspace", "Workspace"),
    ("general", "General"),
    ("appearance", "Appearance"),
    ("editor", "Editor"),
    ("fonts", "Fonts"),
    ("keymap", "Keymap"),
];

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — tree")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// Which folders are open. This is the state the Slint model keeps in
    /// `expanded`, and the one a caller must own here.
    open: HashSet<&'static str>,
    /// The selected row of each tree, by id. Slint keeps a row *index* per Tree
    /// (`selected: 1`, `selected: 2`, `picked`); an application keys selection
    /// by stable identity instead, and still keeps one per tree.
    basic_selected: Option<&'static str>,
    files_selected: Option<&'static str>,
    picked: Option<&'static str>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(&'static str),
    SelectBasic(&'static str),
    SelectFiles(&'static str),
    SelectSetting(&'static str),
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
        open: HashSet::from([
            "src",
            "slint-component",
            "slint-component/ui",
            "workspace",
            "editor",
        ]),
        basic_selected: Some("src/lib.rs"),
        files_selected: Some("slint-component/ui"),
        picked: Some("general"),
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Toggle(id) => {
            if !app.open.remove(id) {
                app.open.insert(id);
            }
        }
        Message::SelectBasic(id) => app.basic_selected = Some(id),
        Message::SelectFiles(id) => app.files_selected = Some(id),
        Message::SelectSetting(id) => app.picked = Some(id),
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

// ------------------------------------------------------------------- pieces

/// A mark from the icon set the Slint gallery ships (`crates/slint-component/ui/icons`,
/// Lucide, ISC), so the two galleries draw the same glyphs. The ink is resolved
/// inside the style closure, because the theme is only known when iced asks — an
/// unresolved colour is the trap that once drew a dark icon as black.
fn mark<'a>(markup: &'static [u8], size: f32, color: fn(&Theme) -> Color) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(size)
        .height(size)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(color(theme)),
        })
        .into()
}

/// The ink the tree's folder and file glyphs draw in, `Theme.foreground` in the
/// Slint file.
fn ink(theme: &Theme) -> Color {
    Tokens::of(theme).foreground
}

/// A folder: its glyph follows the open state, exactly as Slint swaps
/// `folder-open` for `folder`.
fn folder<'a>(
    id: &'static str,
    label: &'static str,
    open: bool,
    children: Vec<Node<'a, &'static str, Message>>,
) -> Node<'a, &'static str, Message> {
    Node::new(id, label)
        .icon(mark(if open { FOLDER_OPEN } else { FOLDER }, 14.0, ink))
        .extend(children)
}

/// A leaf.
fn file<'a>(id: &'static str, label: &'static str) -> Node<'a, &'static str, Message> {
    Node::new(id, label).icon(mark(FILE, 14.0, ink))
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

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A line of text in one of the semantic inks.
fn line<'a>(value: &'a str, size: f32, color: fn(&Theme) -> Color) -> Element<'a, Message> {
    text(value)
        .size(size)
        .style(move |theme: &Theme| text::Style {
            color: Some(color(theme)),
        })
        .into()
}

/// The tree, wired the one way a caller wires it: the open set and that tree's
/// selection in, the toggle and the select back out.
fn tree<'a>(
    app: &'a App,
    nodes: Vec<Node<'a, &'static str, Message>>,
    selected: Option<&'static str>,
    on_select: fn(&'static str) -> Message,
) -> Element<'a, Message> {
    Tree::new(nodes)
        .selected(selected)
        .is_open(move |id: &&'static str| app.open.contains(*id))
        .on_toggle(Message::Toggle)
        .on_select(on_select)
        .into()
}

// --------------------------------------------------------------------- view

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic Tree
    // `src` opens on boot and `components` does not: the chevron shows which, and
    // the children of a folder are wrapped in `collapsible::reveal`, so opening
    // one slides its rows out over 150ms instead of snapping them into place.
    // The reveal stays mounted while the folder is closed, which is what lets a
    // second click mid-flight turn it around from where it is.
    let basic = vec![
        folder(
            "src",
            "src",
            app.open.contains("src"),
            vec![file("src/lib.rs", "lib.rs"), file("src/main.rs", "main.rs")],
        ),
        folder(
            "components",
            "components",
            app.open.contains("components"),
            vec![
                file("src/components/tree.rs", "tree.rs"),
                file("src/components/calendar.rs", "calendar.rs"),
            ],
        ),
        file("Cargo.toml", "Cargo.toml"),
    ];

    // section: File Tree with Icons
    // The Slint example's file tree: the folder and file glyphs are the caller's
    // elements through `Node::icon`, and the arrow is the component's own Lucide
    // chevron unless the caller supplies one.
    let files = vec![folder(
        "slint-component",
        "slint-component",
        app.open.contains("slint-component"),
        vec![
            folder(
                "slint-component/ui",
                "ui",
                app.open.contains("slint-component/ui"),
                vec![
                    file("slint-component/ui/dock.slint", "dock.slint"),
                    file("slint-component/ui/menu.slint", "menu.slint"),
                    file("slint-component/ui/theme.slint", "theme.slint"),
                ],
            ),
            file("slint-component/Cargo.toml", "Cargo.toml"),
            file("slint-component/README.md", "README.md"),
        ],
    )];

    // section: Tree with Selection Handling
    // The tree beside the thing it selects, which is how a settings window or a
    // file browser puts one to work. The component reports the id; the demo maps
    // it back to the row it names.
    let settings = vec![
        folder(
            "workspace",
            "Workspace",
            app.open.contains("workspace"),
            vec![file("general", "General"), file("appearance", "Appearance")],
        ),
        folder(
            "editor",
            "Editor",
            app.open.contains("editor"),
            vec![file("fonts", "Fonts"), file("keymap", "Keymap")],
        ),
    ];

    let picked = app
        .picked
        .and_then(|id| SETTINGS.iter().position(|(row, _)| *row == id));

    let title = picked
        .map(|ix| SETTINGS[ix].1)
        .unwrap_or("Nothing selected");
    let subtitle = match picked {
        Some(ix) => format!("Section {} of {}", ix + 1, SETTINGS.len()),
        None => "Pick a row on the left.".to_string(),
    };

    let detail = container(
        column![
            line(title, theme::FONT_SIZE_SM, ink),
            // Owned, because the section count is formatted: `line` borrows and the
            // view returns this element past the call.
            text(subtitle)
                .size(theme::FONT_SIZE_XS)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        ]
        .spacing(6.0),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(16.0);

    let sidebar = container(tree(app, settings, app.picked, Message::SelectSetting))
        .width(Length::Fixed(220.0))
        .height(Length::Fill)
        .padding(Padding {
            top: 4.0,
            right: 4.0,
            bottom: 4.0,
            left: 4.0,
        })
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).sidebar.into()),
            ..container::Style::default()
        });

    let divider = container(Space::new())
        .width(Length::Fixed(1.0))
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).border.into()),
            ..container::Style::default()
        });

    let selection = container(
        row![sidebar, divider, detail]
            .spacing(0.0)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fixed(190.0))
    .style(|theme: &Theme| container::Style {
        border: Border {
            color: Tokens::of(theme).border,
            width: 1.0,
            radius: theme::RADIUS_MD.into(),
        },
        ..container::Style::default()
    });

    // section: end
    let status = text("Click a row to select it, or a chevron to open its folder.")
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        });

    let gallery = column![
        heading(app, 0, "Basic Tree"),
        body_box(tree(app, basic, app.basic_selected, Message::SelectBasic)),
        heading(app, 1, "File Tree with Icons"),
        body_box(tree(app, files, app.files_selected, Message::SelectFiles)),
        heading(app, 2, "Tree with Selection Handling"),
        body_box(selection),
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

const FOLDER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/></svg>"#;

const FOLDER_OPEN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2"/></svg>"#;

const FILE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
