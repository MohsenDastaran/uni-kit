//! `dock` — a four-region workspace: a centre with left, right and bottom docks,
//! each a tab strip over its active panel, resizable by dragging a divider,
//! collapsible, and rearrangeable by dragging a tab onto another tab.
//!
//! Run it with `cargo run --example dock`, or in the browser as `?story=dock`
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
//! **The page documents seven sections and this demo fills all seven.** The
//! first is the workspace the gallery runs; the others are the pieces it is made
//! of — a panel, the initial layout, a restore, the move, the sizes, and how to
//! run it — each cut from this file rather than written beside it. Sections that
//! are not themselves a picture (the restore, the move, the sizes) carry a short
//! note in their box, exactly as the Slint example's own "Move a panel to
//! another group" box does.
//!
//! **The workspace is the application's.** `active`, `collapsed` and `sizes`
//! are this demo's own values, read into the [`Dock`] every frame and written
//! back from its messages; a drag reports every pixel it travels.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Font, Length, Task, Theme, font};

use crate::components::button::Button;
use crate::components::dock::{Dock, Panel, Region};
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// One panel of the workspace: the shape Slint's `DockPanelDef` declares and the
/// fields [`panel`] turns into a [`Panel`].
#[derive(Debug, Clone)]
struct PanelDef {
    id: u64,
    title: &'static str,
    body: &'static str,
    region: Region,
    closable: bool,
}

/// Which panel each region opens on, in `Region::ALL` order.
const DEFAULT_ACTIVE: [Option<u64>; 4] = [Some(1), Some(3), Some(7), Some(5)];

#[derive(Debug, Clone)]
enum Message {
    /// A tab was chosen. Selecting inside a collapsed dock expands it.
    Select(u64),
    /// A tab's close control was pressed.
    Close(u64),
    /// A tab was dragged onto another: panel, destination region, and the panel
    /// to place it before — or `None` for the end of the region.
    Move(u64, Region, Option<u64>),
    /// A divider moved: already clamped to the region's floor and ceiling.
    Resize(Region, f32),
    /// An edge dock's collapse control was pressed.
    Toggle(Region),
    /// The Restore section's button.
    ResetLayout,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

/// The workspace: the panels in tab order, which one is active in each region,
/// which edge docks are collapsed, and each region's size.
#[derive(Debug)]
struct App {
    panels: Vec<PanelDef>,
    active: [Option<u64>; 4],
    collapsed: [bool; 4],
    sizes: [f32; 4],
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

// section: Runnable example
/// Run the demo: the same application the library's gallery serves as
/// `?story=dock`, in a native window.
pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — dock")
        .theme(theme)
        .run()
}

// section: Define a panel
/// One panel: its identity, its tab's title, the dock it starts in, whether its
/// tab closes, and its body. The order the panels are given in is the order
/// their tabs are drawn.
fn panel<'a>(def: &PanelDef) -> Panel<'a, Message> {
    Panel::new(def.id, def.title, body_text(def.body))
        .region(def.region)
        .closable(def.closable)
}

// section: Describe the initial layout
/// The workspace's first layout: seven panels the regions group into a left
/// dock of two, a centre of two, a right dock of one and a bottom dock of two.
fn default_layout() -> Vec<PanelDef> {
    vec![
        PanelDef {
            id: 1,
            title: "Explorer",
            body: "src/ · Cargo.toml · README.md",
            region: Region::Left,
            closable: true,
        },
        PanelDef {
            id: 2,
            title: "Search",
            body: "12 results in 4 files",
            region: Region::Left,
            closable: true,
        },
        PanelDef {
            id: 3,
            title: "main.rs",
            body: "fn main() { app::run(); }",
            region: Region::Center,
            closable: false,
        },
        PanelDef {
            id: 4,
            title: "lib.rs",
            body: "pub mod dock; pub mod theme;",
            region: Region::Center,
            closable: true,
        },
        PanelDef {
            id: 5,
            title: "Terminal",
            body: "$ cargo run --example dock",
            region: Region::Bottom,
            closable: true,
        },
        PanelDef {
            id: 6,
            title: "Problems",
            body: "No problems detected",
            region: Region::Bottom,
            closable: true,
        },
        PanelDef {
            id: 7,
            title: "Outline",
            body: "Dock · Panel · Region",
            region: Region::Right,
            closable: true,
        },
    ]
}

// section: Restore and persist layouts
impl App {
    /// Put the workspace back to the layout it started with.
    ///
    /// iced 0.14 has no Dock state to serialize or deserialize: the layout is
    /// this application's own values, so restoring is building them again. An
    /// application that wants to persist a workspace stores its own panel list
    /// and the three arrays beside it, with whatever it already uses for state.
    fn reset_layout(&mut self) {
        self.panels = default_layout();
        self.active = DEFAULT_ACTIVE;
        self.collapsed = [false; 4];
        self.sizes = DEFAULT_SIZES;
    }
}

// section: Move a panel to another group
impl App {
    /// Move `panel` into `region`, immediately before `before`, or to the end of
    /// the region when `before` is `None`. The panel keeps its body — only its
    /// region and position change.
    ///
    /// The tab drag reports a reorder inside a panel's own strip, so `region` is
    /// that strip's region there. The message carries the region anyway, which is
    /// what lets an application raise the same move into a *different* region
    /// from a menu or shortcut; the pointer does not cross strips.
    fn move_panel(&mut self, panel: u64, region: Region, before: Option<u64>) {
        let Some(index) = self.panels.iter().position(|def| def.id == panel) else {
            return;
        };
        let mut moved = self.panels.remove(index);
        moved.region = region;
        // `before` is resolved after the removal, so a panel moved onto its own
        // former neighbour lands where the pointer asked rather than one off.
        let at = before
            .and_then(|id| self.panels.iter().position(|def| def.id == id))
            .unwrap_or(self.panels.len());
        self.panels.insert(at, moved);
        self.active[region.index()] = Some(panel);
    }
}

// section: Style the workspace
/// The appearance the Dock does not take.
///
/// There is no `DockSkin`: a tab strip, a pane, a rail and a divider all read
/// their fills and edges from `theme.rs`, so the workspace is already wearing
/// the gallery's tokens and a theme switch repaints it. The one input the
/// component takes is each edge dock's size, and [`Region::clamp_size`] — the
/// region's own floor and ceiling — is what bounds a drag.
const DEFAULT_SIZES: [f32; 4] = [
    Region::Left.default_size(),
    Region::Center.default_size(),
    Region::Right.default_size(),
    Region::Bottom.default_size(),
];

// section: Create a dock area
fn view(app: &App) -> Element<'_, Message> {
    // The panels, which one is active in each region, which edge docks are
    // collapsed, and each region's size. Every change comes back as a message
    // and this demo writes it into its own state.
    let dock = Dock::new(app.panels.iter().map(panel))
        .active(Region::Left, app.active[Region::Left.index()])
        .active(Region::Center, app.active[Region::Center.index()])
        .active(Region::Right, app.active[Region::Right.index()])
        .active(Region::Bottom, app.active[Region::Bottom.index()])
        .collapsed(Region::Left, app.collapsed[Region::Left.index()])
        .collapsed(Region::Right, app.collapsed[Region::Right.index()])
        .collapsed(Region::Bottom, app.collapsed[Region::Bottom.index()])
        .size(Region::Left, app.sizes[Region::Left.index()])
        .size(Region::Right, app.sizes[Region::Right.index()])
        .size(Region::Bottom, app.sizes[Region::Bottom.index()])
        .on_select(Message::Select)
        .on_close(Message::Close)
        .on_move(Message::Move)
        .on_resize(Message::Resize)
        .on_toggle(Message::Toggle);

    // The dock fills what it is given; the page fixes the height so the
    // workspace reads as one screen.
    let workspace = container(dock)
        .width(Length::Fill)
        .height(Length::Fixed(460.0));
    // section: end

    let gallery = column![
        heading(app, 0, "Create a dock area"),
        body_box(workspace),
        heading(app, 1, "Define a panel"),
        body_box(note(
            "A Panel is its identity, its tab's title, the dock it starts in, \
             whether its tab closes, and its body. The dock groups the panels by \
             region, in the order they are given."
        )),
        heading(app, 2, "Describe the initial layout"),
        body_box(note(
            "The panels declare their own regions, so there is no separate layout \
             tree to build: the dock draws the left, centre, right and bottom \
             docks from the list itself."
        )),
        heading(app, 3, "Restore and persist layouts"),
        body_box(
            column![
                Button::new("Reset layout")
                    .outline()
                    .on_press(Message::ResetLayout),
                note(
                    "iced keeps no serializable layout schema, so a restore is the \
                     application re-creating its own panel list, active tabs, \
                     collapse flags and sizes. The button runs exactly that."
                ),
            ]
            .spacing(10.0)
            .align_x(Alignment::Start),
        ),
        heading(app, 4, "Move a panel to another group"),
        body_box(note(
            "Drag a tab along its own strip to reorder it: the drop lands before \
             the tab under the pointer, or at the tail. The move is reported with \
             the panel to place it before, and the application writes its own \
             list — the same message an application can raise to move a panel \
             between regions. Dragging a tab onto another strip, and the \
             edge-drop split Slint docks beside a group with, are left out; the \
             component's module says so."
        )),
        heading(app, 5, "Style the workspace"),
        body_box(note(
            "There is no DockSkin to configure: every surface is read from the \
             component's own theme tokens, so the workspace already matches the \
             page and follows a theme switch. Each edge dock's size is the one \
             input the Dock takes, and the region clamps its own drag."
        )),
        heading(app, 6, "Runnable example"),
        body_box(note(
            "cargo run --example dock, or ?story=dock in the browser gallery the \
             website embeds. The workspace above is that application."
        )),
        text("Drag a tab, or a divider between a dock and the centre.")
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

/// The workspace's opening state: the first layout, the panels each region
/// opens on, nothing collapsed and every region at its own default size.
fn boot() -> App {
    App {
        panels: default_layout(),
        active: DEFAULT_ACTIVE,
        collapsed: [false; 4],
        sizes: DEFAULT_SIZES,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Select(panel) => {
            if let Some(def) = app.panels.iter().find(|def| def.id == panel) {
                let region = def.region;
                app.active[region.index()] = Some(panel);
                app.collapsed[region.index()] = false;
            }
        }
        Message::Close(panel) => {
            let region = app
                .panels
                .iter()
                .find(|def| def.id == panel)
                .map(|def| def.region);
            app.panels.retain(|def| def.id != panel);
            if let Some(region) = region {
                app.active[region.index()] = app
                    .panels
                    .iter()
                    .find(|def| def.region == region)
                    .map(|def| def.id);
            }
        }
        Message::Move(panel, region, before) => app.move_panel(panel, region, before),
        Message::Resize(region, size) => {
            app.sizes[region.index()] = region.clamp_size(size);
        }
        Message::Toggle(region) => {
            app.collapsed[region.index()] = !app.collapsed[region.index()];
        }
        Message::ResetLayout => app.reset_layout(),
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

/// A panel body: the muted content the Slint example writes into its
/// `DockPanelDef`. The dock insets it by 16px, so it draws none of its own.
fn body_text<'a>(body: &'static str) -> Element<'a, Message> {
    text(body)
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).foreground),
        })
        .into()
}

/// A short line under a section that is not itself a picture.
fn note<'a>(line: &'static str) -> Element<'a, Message> {
    text(line)
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
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

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
