//! `resizable` — the whole gallery the Slint example shows: two panes around a
//! divider, a floor, a ceiling, three panes with two handles, nested splits,
//! a hidden pane, a file explorer, an IDE workspace, a dashboard of widgets,
//! a settings panel, and the divider's own appearance.
//!
//! Run it with `cargo run --example resizable`, or in the browser as
//! `?story=resizable` in the gallery the website embeds.
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
//! **The split is the application's, and it is written back on every pixel.**
//! A [`ResizeHandle`] reports what a drag asks for through `on_resize`, already
//! clamped to the floor and the optional ceiling it was given, and this demo
//! stores it. That is the controlled shape [`Slider`](crate::components::slider)
//! uses for its value, and it is why a drag is a message per pixel rather than a
//! value that appears on release.
//!
//! **The divider is horizontal by default.** [`Axis::Vertical`] turns one
//! ninety degrees, which the IDE Layout section uses for its terminal; the
//! other sections keep the horizontal handles the Slint example draws.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Font, Length, Task, Theme, font};

use crate::components::button::Button;
use crate::components::group_box::{self, GroupBox};
use crate::components::resizable::{Axis, ResizablePane, ResizeHandle};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — resizable")
        .theme(theme)
        .run()
}

/// Which split a message is about, so one arm moves any handle on the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Usage,
    Appearance,
    Constraints,
    Limits,
    MultiA,
    MultiB,
    NestedOuter,
    NestedInner,
    GroupTop,
    GroupBottom,
    Aside,
    Explorer,
    IdeSidebar,
    IdeTerminal,
    Dashboard,
    Settings,
}

#[derive(Debug)]
struct App {
    usage: f32,
    appearance: f32,
    constraints: f32,
    limits: f32,
    multi_a: f32,
    multi_b: f32,
    nested_outer: f32,
    nested_inner: f32,
    group_top: f32,
    group_bottom: f32,
    aside: f32,
    show_aside: bool,
    explorer: f32,
    ide_sidebar: f32,
    ide_terminal: f32,
    dashboard: f32,
    settings: f32,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A divider moved: already clamped to its floor and ceiling.
    Resized(Slot, f32),
    /// The Conditional Panel Visibility button.
    ToggleAside,
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
        usage: 220.0,
        appearance: 200.0,
        constraints: 240.0,
        limits: 200.0,
        multi_a: 180.0,
        multi_b: 180.0,
        nested_outer: 160.0,
        nested_inner: 160.0,
        group_top: 200.0,
        group_bottom: 200.0,
        aside: 180.0,
        show_aside: true,
        explorer: 200.0,
        ide_sidebar: 300.0,
        ide_terminal: 150.0,
        dashboard: 260.0,
        settings: 180.0,
        hovered: None,
        pressed_source: None,
    }
}

/// The split behind a slot, so one arm moves any handle on the page.
fn value_mut(app: &mut App, slot: Slot) -> &mut f32 {
    match slot {
        Slot::Usage => &mut app.usage,
        Slot::Appearance => &mut app.appearance,
        Slot::Constraints => &mut app.constraints,
        Slot::Limits => &mut app.limits,
        Slot::MultiA => &mut app.multi_a,
        Slot::MultiB => &mut app.multi_b,
        Slot::NestedOuter => &mut app.nested_outer,
        Slot::NestedInner => &mut app.nested_inner,
        Slot::GroupTop => &mut app.group_top,
        Slot::GroupBottom => &mut app.group_bottom,
        Slot::Aside => &mut app.aside,
        Slot::Explorer => &mut app.explorer,
        Slot::IdeSidebar => &mut app.ide_sidebar,
        Slot::IdeTerminal => &mut app.ide_terminal,
        Slot::Dashboard => &mut app.dashboard,
        Slot::Settings => &mut app.settings,
    }
}

/// A divider on a slot: it reports the split a drag asks for, and the arm above
/// writes it back.
fn handle<'a>(slot: Slot, split: f32) -> ResizeHandle<'a, Message> {
    ResizeHandle::new(split).on_resize(move |split| Message::Resized(slot, split))
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Resized(slot, split) => *value_mut(app, slot) = split,
        Message::ToggleAside => app.show_aside = !app.show_aside,
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

/// Muted text under an example, carrying what the demo has to say about it.
fn muted_line<'a>(line: &str) -> Element<'a, Message> {
    text(line.to_string())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A pane's content: a heading, and an optional muted line under it. The pane
/// itself draws no padding, so the column carries the 12px inset the Slint
/// example's own `VerticalLayout` sets.
fn pane<'a>(title: &'static str, note: &'static str) -> Element<'a, Message> {
    let mut body = column![text(title)
        .size(theme::FONT_SIZE_SM)
        .font(Font {
            weight: font::Weight::Medium,
            ..Font::DEFAULT
        })
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).foreground),
        })]
    .padding(12.0)
    .spacing(4.0);

    if !note.is_empty() {
        body = body.push(
            text(note)
                .size(theme::FONT_SIZE_XS)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        );
    }

    body.into()
}

/// A pane's content as a small list: an optional heading and one muted line per
/// row, the file tree and the settings navigation the Slint example draws.
fn list_pane<'a>(title: &'static str, rows: &'static [&'static str]) -> Element<'a, Message> {
    let mut body = column![].padding(12.0).spacing(6.0);

    if !title.is_empty() {
        body = body.push(
            text(title)
                .size(theme::FONT_SIZE_SM)
                .font(Font {
                    weight: font::Weight::Medium,
                    ..Font::DEFAULT
                })
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
        );
    }

    for row in rows {
        body = body.push(
            text(*row)
                .size(theme::FONT_SIZE_XS)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        );
    }

    body.into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(text(title).size(theme::FONT_SIZE_SM).font(Font {
            weight: font::Weight::Medium,
            ..Font::DEFAULT
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

fn view(app: &App) -> Element<'_, Message> {
    // section: Usage
    // Two panes around one handle. The fixed pane reads the handle's split, so
    // dragging the divider moves exactly one edge.
    let usage = row![
        ResizablePane::new()
            .width(app.usage)
            .child(pane("Sidebar", "Drag the divider.")),
        handle(Slot::Usage, app.usage).minimum(80.0),
        ResizablePane::new().child(pane("Content", "Takes whatever is left.")),
    ]
    .width(Length::Fill)
    .height(160.0);

    // section: Resize Handle Appearance
    // The divider is a 6px strip in the border colour, and its target reaches
    // four pixels into each pane. Hovering or holding it turns the strip the
    // primary colour at 45%, the one state it draws.
    let appearance = column![
        row![
            ResizablePane::new()
                .width(app.appearance)
                .child(pane("Pane", "A hairline, easy to grab.")),
            handle(Slot::Appearance, app.appearance).minimum(120.0),
            ResizablePane::new().child(pane("Pane", "The strip never widens.")),
        ]
        .width(Length::Fill)
        .height(120.0),
        muted_line(
            "The strip is 6px of the border colour; its hit box reaches 4px into each pane.",
        ),
    ]
    .spacing(8.0);

    // section: Panel Size Constraints
    // `minimum` is where the divider stops, so the pane cannot be dragged away.
    let constraints = row![
        ResizablePane::new()
            .width(app.constraints)
            .child(pane("240px to start", "Stops at 120px.")),
        handle(Slot::Constraints, app.constraints).minimum(120.0),
        ResizablePane::new().child(pane("Content", "")),
    ]
    .width(Length::Fill)
    .height(160.0);

    // section: Multiple Panels
    // Three panes are two handles in a row, each owning its own edge.
    let multiple = row![
        ResizablePane::new()
            .width(app.multi_a)
            .child(pane("Explorer", "")),
        handle(Slot::MultiA, app.multi_a).minimum(120.0),
        ResizablePane::new()
            .width(app.multi_b)
            .child(pane("Outline", "")),
        handle(Slot::MultiB, app.multi_b).minimum(120.0),
        ResizablePane::new().child(pane("Editor", "")),
    ]
    .width(Length::Fill)
    .height(160.0);

    // section: Nested Layouts
    // A split inside a pane is a normal element, so a workspace nests freely.
    let nested = row![
        ResizablePane::new()
            .width(app.nested_outer)
            .child(pane("Explorer", "")),
        handle(Slot::NestedOuter, app.nested_outer).minimum(80.0),
        ResizablePane::new().child(
            row![
                ResizablePane::new()
                    .width(app.nested_inner)
                    .child(pane("Outline", "")),
                handle(Slot::NestedInner, app.nested_inner).minimum(80.0),
                ResizablePane::new().child(pane("Editor", "")),
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        ),
    ]
    .width(Length::Fill)
    .height(200.0);

    // section: Nested Panel Groups
    // A column of two rows, each of which is itself a horizontal split.
    let groups = column![
        ResizablePane::new().child(
            row![
                ResizablePane::new()
                    .width(app.group_top)
                    .child(pane("Left", "")),
                handle(Slot::GroupTop, app.group_top).minimum(120.0),
                ResizablePane::new().child(pane("Centre", "")),
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        ),
        ResizablePane::new().child(
            row![
                ResizablePane::new()
                    .width(app.group_bottom)
                    .child(pane("Bottom left", "")),
                handle(Slot::GroupBottom, app.group_bottom).minimum(120.0),
                ResizablePane::new().child(pane("Bottom right", "")),
            ]
            .width(Length::Fill)
            .height(Length::Fill),
        ),
    ]
    .width(Length::Fill)
    .height(240.0)
    .spacing(0.0);

    // section: Conditional Panel Visibility
    // A pane can be left out entirely; the handle stays with it and the
    // remaining pane keeps the width the hidden one gave back.
    let visibility = column![
        Button::new(if app.show_aside {
            "Hide the sidebar"
        } else {
            "Show the sidebar"
        })
        .outline()
        .on_press(Message::ToggleAside),
        row![]
            .extend(app.show_aside.then(|| {
                Element::from(ResizablePane::new().width(app.aside).child(pane("Sidebar", "")))
            }))
            .extend(app.show_aside.then(|| {
                Element::from(handle(Slot::Aside, app.aside).minimum(120.0))
            }))
            .push(ResizablePane::new().child(pane("Content", "")))
            .width(Length::Fill)
            .height(140.0)
            .spacing(0.0),
    ]
    .spacing(10.0)
    .align_x(Alignment::Start);

    // section: Panel with Size Limits
    // A ceiling as well as a floor: the pane cannot grow past 320px either.
    let limits = row![
        ResizablePane::new()
            .width(app.limits)
            .child(pane("Capped pane", "120px .. 320px.")),
        handle(Slot::Limits, app.limits)
            .minimum(120.0)
            .maximum(320.0),
        ResizablePane::new().child(pane("Content", "")),
    ]
    .width(Length::Fill)
    .height(160.0);

    // section: File Explorer Layout
    // A file tree beside the file it opens.
    let explorer = row![
        ResizablePane::new().width(app.explorer).child(list_pane(
            "src",
            &["components", "lib.rs", "main.rs", "theme.rs"],
        )),
        handle(Slot::Explorer, app.explorer).minimum(140.0),
        ResizablePane::new().child(list_pane(
            "main.rs",
            &["fn main() {", "    App::new().run();", "}"],
        )),
    ]
    .width(Length::Fill)
    .height(220.0);

    // section: IDE Layout
    // A workspace: a sidebar beside an editor, and the editor split again into
    // the code above and the terminal below, with a vertical handle between them.
    let ide = row![
        ResizablePane::new()
            .width(app.ide_sidebar)
            .child(pane("File Explorer", "")),
        handle(Slot::IdeSidebar, app.ide_sidebar).minimum(200.0),
        ResizablePane::new().child(
            column![
                ResizablePane::new().child(pane("Code Editor", "")),
                handle(Slot::IdeTerminal, app.ide_terminal)
                    .minimum(100.0)
                    .axis(Axis::Vertical),
                ResizablePane::new()
                    .height(app.ide_terminal)
                    .child(pane("Terminal / Output", "")),
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(0.0),
        ),
    ]
    .width(Length::Fill)
    .height(240.0);

    // section: Dashboard with Widgets
    // Four widgets: a row of two splits, each split again into two rows.
    let dashboard = row![
        ResizablePane::new().width(app.dashboard).child(
            column![
                ResizablePane::new().child(pane("Revenue", "")),
                ResizablePane::new().child(pane("Signups", "")),
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(0.0),
        ),
        handle(Slot::Dashboard, app.dashboard).minimum(160.0),
        ResizablePane::new().child(
            column![
                ResizablePane::new().child(pane("Traffic", "")),
                ResizablePane::new().child(pane("Errors", "")),
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(0.0),
        ),
    ]
    .width(Length::Fill)
    .height(240.0);

    // section: Settings Panel
    // A navigation column beside the panel it selects.
    let settings = row![
        ResizablePane::new().width(app.settings).child(list_pane(
            "",
            &["General", "Appearance", "Shortcuts", "Updates"],
        )),
        handle(Slot::Settings, app.settings).minimum(140.0),
        ResizablePane::new().child(pane(
            "Appearance",
            "The panel the navigation selects.",
        )),
    ]
    .width(Length::Fill)
    .height(200.0);

    // section: end
    let status = text("Drag a divider — the gallery is live.")
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        });

    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Resize Handle Appearance"),
        body_box(appearance),
        heading(app, 2, "Panel Size Constraints"),
        body_box(constraints),
        heading(app, 3, "Multiple Panels"),
        body_box(multiple),
        heading(app, 4, "Nested Layouts"),
        body_box(nested),
        heading(app, 5, "Nested Panel Groups"),
        body_box(groups),
        heading(app, 6, "Conditional Panel Visibility"),
        body_box(visibility),
        heading(app, 7, "Panel with Size Limits"),
        body_box(limits),
        heading(app, 8, "File Explorer Layout"),
        body_box(explorer),
        heading(app, 9, "IDE Layout"),
        body_box(ide),
        heading(app, 10, "Dashboard with Widgets"),
        body_box(dashboard),
        heading(app, 11, "Settings Panel"),
        body_box(settings),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
