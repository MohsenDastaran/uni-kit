//! `dock-block` — the Blocks page's application shell: a toolbar over a
//! dockable workspace, with a status bar underneath.
//!
//! A block is not a component sample. This is the finished application screen the
//! site's Blocks page shows for iced, composed from the same files
//! `uni-kit add iced` writes: the [`Toolbar`] running the commands, the [`Dock`]
//! holding the panels of a document workspace, and the [`StatusBar`] under it
//! saying what is open and where the caret is.
//!
//! The workspace is one state held here: which panel is active in each region,
//! which edge docks are folded, and how wide each one is. A tab press selects,
//! a close control removes the panel, a tab dragged along its own strip asks for
//! a new position, a divider drag reports the width it wants, and a collapse
//! control folds or restores an edge dock — every one of them a message this
//! screen writes into its own state. The reference's two pointer moves that cross
//! a strip, joining another group and dropping beside one, are what iced's dock
//! does not do, so the panel copy says what it can.
//!
//! The bar's three commands are the reference's chrome: this screen's job is the
//! workspace, and GPUI's block binds an empty callback for the same reason.
//!
//! Run it with `cargo run --bin iced-gallery -- dock-block`, or in the browser as
//! `?story=dock-block` in the gallery the website embeds. The Blocks page asks
//! for that story and reads this file for its usage prompt.

use iced::widget::{column, container, svg, text};
use iced::{Border, Color, Element, Length, Task, Theme};

use crate::components::button::{Button, Variant as ButtonVariant};
use crate::components::dock::{Dock, Panel, Region};
use crate::components::status_bar::{StatusBar, StatusItem};
use crate::components::theme::{self, ControlSize, Tokens};
use crate::components::toolbar::{Toolbar, ToolbarGroup, ToolbarSeparator, ToolbarSpacer};

/// The toolbar's inset above and below its commands and beside them, the Slint
/// block's own bar padding.
const BAR_PADDING_VERTICAL: f32 = 8.0;
const BAR_PADDING_HORIZONTAL: f32 = 12.0;

// The Lucide marks the screen uses, from the same ISC set the Slint block draws.

/// `folder-open`, the Open command.
const FOLDER_OPEN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2"/></svg>"#;
/// `download`, the Save command.
const DOWNLOAD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/><path d="M12 15V3"/></svg>"#;
/// `play`, the Run command.
const PLAY: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="6 3 20 12 6 21 6 3"/></svg>"#;
/// `check`, the status bar's Ready mark.
const CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"/></svg>"#;
/// `git-branch`, the status bar's branch mark.
const GIT_BRANCH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="6" x2="6" y1="3" y2="15"/><circle cx="18" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><path d="M18 9a9 9 0 0 1-9 9"/></svg>"#;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — dock")
        .theme(theme)
        .run()
}

/// One panel of the workspace: its identity, its tab's title, the dock it starts
/// in, whether its tab closes, and its body.
struct PanelDef {
    id: u64,
    title: &'static str,
    body: &'static str,
    region: Region,
    closable: bool,
}

/// The workspace's first layout: a left dock of two, a centre of two, and a
/// bottom dock of two, in the order their tabs are drawn.
fn default_layout() -> Vec<PanelDef> {
    vec![
        PanelDef {
            id: 1,
            title: "Explorer",
            body: "Reorder this tab along its strip.",
            region: Region::Left,
            closable: true,
        },
        PanelDef {
            id: 2,
            title: "Search",
            body: "Two panels can share one tab group.",
            region: Region::Left,
            closable: true,
        },
        PanelDef {
            id: 3,
            title: "main.rs",
            body: "Drag a divider to resize the centre.",
            region: Region::Center,
            closable: false,
        },
        PanelDef {
            id: 4,
            title: "lib.rs",
            body: "The centre pane takes the surplus width.",
            region: Region::Center,
            closable: true,
        },
        PanelDef {
            id: 5,
            title: "Terminal",
            body: "The bottom dock shares the workspace column.",
            region: Region::Bottom,
            closable: true,
        },
        PanelDef {
            id: 6,
            title: "Problems",
            body: "No problems detected.",
            region: Region::Bottom,
            closable: true,
        },
    ]
}

/// Which panel each region opens on, in [`Region::ALL`] order. The right dock
/// opens with no panels, as both reference blocks do.
const DEFAULT_ACTIVE: [Option<u64>; 4] = [Some(1), Some(3), None, Some(5)];

/// The screen's whole state: the panels in tab order, which one is active in
/// each region, which edge docks are collapsed, and each region's size.
struct App {
    panels: Vec<PanelDef>,
    active: [Option<u64>; 4],
    collapsed: [bool; 4],
    sizes: [f32; 4],
}

impl Default for App {
    fn default() -> Self {
        Self {
            panels: default_layout(),
            active: DEFAULT_ACTIVE,
            collapsed: [false; 4],
            sizes: [
                Region::Left.default_size(),
                Region::Center.default_size(),
                Region::Right.default_size(),
                Region::Bottom.default_size(),
            ],
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    /// A tab was chosen. Selecting inside a folded dock restores it.
    Select(u64),
    /// A tab's close control was pressed.
    Close(u64),
    /// A tab was dragged: the panel, the region it belongs in, and the panel to
    /// place it before — or `None` for the end of the region.
    Move(u64, Region, Option<u64>),
    /// A divider moved: the size a drag asks for, already clamped.
    Resize(Region, f32),
    /// An edge dock's collapse control was pressed.
    Toggle(Region),
    /// One of the bar's three commands. The workspace is this screen's job, so
    /// the bar is the reference's chrome and a press reports without a
    /// destination — GPUI's block binds an empty `on_click` for the same reason.
    Command,
}

fn boot() -> App {
    App::default()
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
            // The closed panel may have been the region's active one; the next
            // panel in that region takes its place, or the region stays empty.
            if let Some(region) = region {
                app.active[region.index()] = app
                    .panels
                    .iter()
                    .find(|def| def.region == region)
                    .map(|def| def.id);
            }
        }
        Message::Move(panel, region, before) => move_panel(app, panel, region, before),
        Message::Resize(region, size) => app.sizes[region.index()] = region.clamp_size(size),
        Message::Toggle(region) => app.collapsed[region.index()] = !app.collapsed[region.index()],
        Message::Command => {}
    }
    Task::none()
}

/// Move `panel` into `region`, immediately before `before`, or to the end of the
/// region when `before` is `None`. The panel keeps its body; only its region and
/// its position change.
fn move_panel(app: &mut App, panel: u64, region: Region, before: Option<u64>) {
    let Some(index) = app.panels.iter().position(|def| def.id == panel) else {
        return;
    };
    let mut moved = app.panels.remove(index);
    moved.region = region;
    // `before` is resolved after the removal, so a panel moved onto its own
    // former neighbour lands where the pointer asked rather than one off.
    let at = before
        .and_then(|id| app.panels.iter().position(|def| def.id == id))
        .unwrap_or(app.panels.len());
    app.panels.insert(at, moved);
    app.active[region.index()] = Some(panel);
}

/// An SVG in a square box, coloured from the tokens in force while it renders.
fn glyph<'a, Message: 'a>(
    bytes: &'static [u8],
    size: f32,
    ink: fn(Tokens) -> Color,
) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(ink(Tokens::of(theme))),
        })
        .into()
}

/// One panel: its identity, its tab's title, its dock and its body. The dock
/// inlays the body by its own padding, so this draws none.
fn panel<'a>(def: &PanelDef) -> Panel<'a, Message> {
    Panel::new(
        def.id,
        def.title,
        text(def.body)
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
    )
    .region(def.region)
    .closable(def.closable)
}

/// The commands above the workspace. The spacer keeps them at the leading edge,
/// which is the reference's `ToolbarSpacer`.
fn command_bar<'a>() -> Element<'a, Message> {
    let file = ToolbarGroup::new()
        .size(ControlSize::Small)
        .child(
            Button::new("Open")
                .size(ControlSize::Small)
                .icon(glyph(FOLDER_OPEN, 14.0, |tokens: Tokens| tokens.foreground))
                .on_press(Message::Command),
        )
        .child(
            Button::new("Save")
                .size(ControlSize::Small)
                .icon(glyph(DOWNLOAD, 14.0, |tokens: Tokens| tokens.foreground))
                .on_press(Message::Command),
        );

    let run = Button::new("Run")
        .variant(ButtonVariant::Primary)
        .size(ControlSize::Small)
        .icon(glyph(PLAY, 14.0, |tokens: Tokens| {
            tokens.primary_foreground
        }))
        .on_press(Message::Command);

    Toolbar::new()
        .size(ControlSize::Small)
        .spacing(theme::SPACING_SM)
        .child(file)
        .child(ToolbarSeparator::new())
        .child(ToolbarGroup::new().size(ControlSize::Small).child(run))
        .content(ToolbarSpacer::new())
        .into()
}

/// The row under the workspace: what is open, and where the caret is. The bar
/// draws its own top edge, so only its items are written here.
fn status_bar<'a>() -> Element<'a, Message> {
    StatusBar::new()
        .left(
            StatusItem::label("Ready")
                .icon(glyph(CHECK, 12.0, |tokens: Tokens| tokens.muted_foreground)),
        )
        .left(
            StatusItem::label("main").icon(glyph(GIT_BRANCH, 12.0, |tokens: Tokens| {
                tokens.muted_foreground
            })),
        )
        .right(StatusItem::label("Rust"))
        .right(StatusItem::separator())
        .right(StatusItem::label("Ln 1, Col 1"))
        .into()
}

/// The workspace: the panels read into a [`Dock`] every frame, with this
/// screen's own active tabs, folded docks and sizes, and every pointer operation
/// reported back as a message.
fn workspace<'a>(app: &App) -> Element<'a, Message> {
    Dock::new(app.panels.iter().map(panel))
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
        .on_toggle(Message::Toggle)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // The bar keeps its own inset; the workspace takes what is left between it
    // and the status bar, which is the footer's own edge against the workspace.
    let shell = column![
        container(command_bar()).padding([BAR_PADDING_VERTICAL, BAR_PADDING_HORIZONTAL]),
        workspace(app),
        status_bar(),
    ]
    .width(Length::Fill)
    .height(Length::Fill);

    container(shell)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            border: Border {
                color: Tokens::of(theme).border,
                width: 1.0,
                radius: theme::RADIUS_LG.into(),
            },
            ..container::Style::default()
        })
        .into()
}
