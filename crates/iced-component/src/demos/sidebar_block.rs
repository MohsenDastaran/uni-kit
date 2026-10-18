//! `sidebar-block` — the Blocks page's workspace screen: a grouped navigation
//! rail beside the workspace it controls.
//!
//! A block is not a component sample. This is the finished application screen
//! the site's Blocks page shows for iced, composed from the same files
//! `uni-kit add iced` writes: the [`Sidebar`] with its groups, its disclosed
//! project group and its footer avatar, the [`Card`] figures, the [`Button`]
//! that carries the screen's one commitment, and the [`DataTable`] the figures
//! describe.
//!
//! The rail and the pane are one state. Selecting an entry moves the heading
//! beside it; the Projects row also discloses the projects underneath it, on the
//! [`collapsible::reveal`](crate::components::collapsible::reveal) the sidebar
//! composes. The rail's own collapse control folds it to the 56px icon rail, and
//! the pane puts the trigger back beside the heading, which is the arrangement
//! the Slint block uses.
//!
//! Run it with `cargo run --bin iced-gallery -- sidebar-block`, or in the
//! browser as `?story=sidebar-block` in the gallery the website embeds. The
//! Blocks page asks for that story and reads this file for its usage prompt.

use iced::font::Weight;
use iced::widget::{button, column, container, responsive, row, svg, text};
use iced::{Alignment, Color, Element, Font, Length, Shadow, Task, Theme};

use crate::components::button::{Button, Variant as ButtonVariant};
use crate::components::card::Card;
use crate::components::data_table::{Align, Cell, Column, ColumnSort, DataTable, Row};
use crate::components::sidebar::{Sidebar, SidebarEntry, SidebarGroup};
use crate::components::theme::{self, ControlSize, Status, Tokens};

/// The pane's inset, the Slint block's `padding: 24px`.
const PANE_PADDING: f32 = 24.0;
/// The gap between the heading, the figures and the table.
const PANE_GAP: f32 = 20.0;
/// The `panel-left` trigger the pane draws while the rail is collapsed,
/// `width: 28px; height: 28px` in the Slint block.
const TRIGGER: f32 = 28.0;
/// The data table's header row at `ControlSize::Small`, plus its two borders.
/// The table's body is the space the pane has left once this is taken out.
const TABLE_CHROME: f32 = 30.0 + 2.0;
/// A body shorter than this is not a table; the screen scrolls its rows in the
/// smallest frame rather than collapsing the header onto them.
const MIN_TABLE_BODY: f32 = 90.0;

// The Lucide marks the screen uses, from the same ISC set
// `crates/slint-component/ui/icons/` carries.

/// `box`, the rail's brand mark.
const BOX: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"/><path d="m3.3 7 8.7 5 8.7-5"/><path d="M12 22V12"/></svg>"#;
/// `panel-left`, the trigger the pane owns while the rail is folded.
const PANEL_LEFT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/></svg>"#;
/// `layout-dashboard`, the Overview row.
const LAYOUT_DASHBOARD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/></svg>"#;
/// `folder`, the Projects row and its disclosed children.
const FOLDER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/></svg>"#;
/// `chart-column`, the Reports row.
const CHART_COLUMN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M18 17V9"/><path d="M13 17V5"/><path d="M8 17v-3"/></svg>"#;
/// `users`, the Team row.
const USERS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><path d="M16 3.128a4 4 0 0 1 0 7.744"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><circle cx="9" cy="7" r="4"/></svg>"#;
/// `database`, the Billing row.
const DATABASE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5V19A9 3 0 0 0 21 19V5"/><path d="M3 12A9 3 0 0 0 21 12"/></svg>"#;
/// `settings`, the Settings row.
const SETTINGS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>"#;
/// `plus`, the New project button's mark.
const PLUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>"#;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — sidebar")
        .theme(theme)
        .run()
}

/// The rail's destination. The sidebar addresses a row by group and index, so
/// this is where the two agree rather than a second list the rail could drift
/// from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Overview,
    Projects,
    Reports,
    Team,
    Billing,
    Settings,
}

impl Section {
    fn group_entry(self) -> (usize, usize) {
        match self {
            Self::Overview => (0, 0),
            Self::Projects => (0, 1),
            Self::Reports => (0, 2),
            Self::Team => (1, 0),
            Self::Billing => (1, 1),
            Self::Settings => (1, 2),
        }
    }

    fn from_group_entry(group: usize, entry: usize) -> Option<Self> {
        match (group, entry) {
            (0, 0) => Some(Self::Overview),
            (0, 1) => Some(Self::Projects),
            (0, 2) => Some(Self::Reports),
            (1, 0) => Some(Self::Team),
            (1, 1) => Some(Self::Billing),
            (1, 2) => Some(Self::Settings),
            _ => None,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Projects => "Projects",
            Self::Reports => "Reports",
            Self::Team => "Team",
            Self::Billing => "Billing",
            Self::Settings => "Settings",
        }
    }

    /// What the pane says underneath the heading. The caption belongs to the
    /// destination, not to the figures, so it moves with the selection.
    fn caption(self) -> &'static str {
        match self {
            Self::Overview => "A quick view of your workspace activity.",
            Self::Projects => "Every project you can reach, and where it stands.",
            Self::Reports => "The numbers behind the last quarter.",
            Self::Team => "Who is in the workspace and what they can do.",
            Self::Billing => "Your plan, your invoices and your limits.",
            Self::Settings => "How the workspace behaves for everyone in it.",
        }
    }

    fn icon(self) -> &'static [u8] {
        match self {
            Self::Overview => LAYOUT_DASHBOARD,
            Self::Projects => FOLDER,
            Self::Reports => CHART_COLUMN,
            Self::Team => USERS,
            Self::Billing => DATABASE,
            Self::Settings => SETTINGS,
        }
    }
}

/// One row of the table. The rows are written here rather than loaded: a block
/// is a screen, not a data layer. A row the New project button adds is the one
/// owned value, because its name counts up.
struct ProjectRow {
    name: String,
    owner: String,
    state: String,
    updated: String,
    status: Status,
}

/// The workspace's projects, as written. The first three also sit under the
/// Projects row, so the rail and the table name the same objects.
const PROJECTS: [(&str, &str, &str, &str, Status); 8] = [
    (
        "Atlas",
        "Dana Whitfield",
        "Active",
        "2 hours ago",
        Status::Success,
    ),
    (
        "Beacon",
        "Ravi Menon",
        "Active",
        "Yesterday",
        Status::Success,
    ),
    (
        "Compass",
        "Lena Ortiz",
        "Paused",
        "3 days ago",
        Status::Warning,
    ),
    (
        "Delta Sync",
        "Tomas Berg",
        "Active",
        "4 days ago",
        Status::Success,
    ),
    (
        "Ember",
        "Aiko Tanaka",
        "Done",
        "1 week ago",
        Status::Default,
    ),
    (
        "Foundry",
        "Noah Clarke",
        "Active",
        "1 week ago",
        Status::Success,
    ),
    (
        "Granite",
        "Priya Raman",
        "Paused",
        "2 weeks ago",
        Status::Warning,
    ),
    (
        "Harbour",
        "Sofia Almeida",
        "Done",
        "3 weeks ago",
        Status::Default,
    ),
];

/// The block's whole state: where the rail points, whether its Projects group
/// is open, whether the rail is folded, and what the table is doing. Every part
/// is the caller's, which is the controlled shape the components keep.
struct App {
    section: Section,
    projects_open: bool,
    rail_collapsed: bool,
    sort: Option<(usize, ColumnSort)>,
    selected_row: Option<usize>,
    /// The projects the New project button has added, in the order they were
    /// added. Their names count up so two presses are two rows.
    drafts: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            section: Section::Projects,
            // Open on the first frame: the workspace is what the screen is
            // about, and the group's three projects are its own rows.
            projects_open: true,
            rail_collapsed: false,
            sort: None,
            selected_row: None,
            drafts: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    /// A top-level rail row was pressed. The Projects row also asks its group
    /// to open or close, which is the one state the sidebar leaves to us.
    Select { group: usize, entry: usize },
    /// The rail's collapse control, with the value it wants.
    ToggleRail(bool),
    /// A sortable header was pressed, with the direction its cycle lands on.
    Sort {
        column: usize,
        direction: ColumnSort,
    },
    /// A body row was pressed.
    SelectRow(usize),
    /// The screen's one commitment: add a draft project to the table.
    NewProject,
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Select { group, entry } => {
            if let Some(section) = Section::from_group_entry(group, entry) {
                if section == Section::Projects {
                    app.projects_open = !app.projects_open;
                }
                app.section = section;
            }
        }
        Message::ToggleRail(collapsed) => app.rail_collapsed = collapsed,
        Message::Sort { column, direction } => {
            app.sort = direction.is_sorted().then_some((column, direction));
            // The body reorders, so the row that was drawn as selected is no
            // longer the one the index names.
            app.selected_row = None;
        }
        Message::SelectRow(index) => app.selected_row = Some(index),
        Message::NewProject => {
            app.drafts
                .push(format!("Untitled project {}", app.drafts.len() + 1));
            app.selected_row = None;
        }
    }
    Task::none()
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

/// The rail: a brand header, the two groups, the disclosed Projects group and
/// the person the workspace belongs to.
fn rail(app: &App) -> Element<'_, Message> {
    Sidebar::new()
        .header_icon(BOX)
        .header_title("Acme Inc")
        .header_subtitle("Enterprise")
        .group(
            SidebarGroup::new("Workspace").entries([
                SidebarEntry::new(Section::Overview.label()).icon(Section::Overview.icon()),
                SidebarEntry::new(Section::Projects.label())
                    .icon(Section::Projects.icon())
                    .children(
                        PROJECTS
                            .iter()
                            .take(3)
                            .map(|project| SidebarEntry::new(project.0).icon(FOLDER)),
                    )
                    .open(app.projects_open),
                SidebarEntry::new(Section::Reports.label()).icon(Section::Reports.icon()),
            ]),
        )
        .group(SidebarGroup::new("Manage").entries([
            SidebarEntry::new(Section::Team.label()).icon(Section::Team.icon()),
            SidebarEntry::new(Section::Billing.label()).icon(Section::Billing.icon()),
            SidebarEntry::new(Section::Settings.label()).icon(Section::Settings.icon()),
        ]))
        .selected(app.section.group_entry().0, app.section.group_entry().1)
        .collapsed(app.rail_collapsed)
        .footer_initials("JL")
        .footer_name("Jason Lee")
        .on_select(|group, entry| Message::Select { group, entry })
        .on_toggle(Message::ToggleRail)
        .into_element()
}

/// The pane's own expand trigger, drawn while the rail is folded. It is the
/// Slint block's 28px bordered box, and it lives in the pane because the folded
/// rail has no room for it beside the brand mark.
fn expand_trigger<'a>() -> Element<'a, Message> {
    button(
        container(glyph(PANEL_LEFT, 16.0, |tokens: Tokens| {
            tokens.muted_foreground
        }))
        .center_x(TRIGGER)
        .center_y(TRIGGER),
    )
    .width(Length::Fixed(TRIGGER))
    .height(Length::Fixed(TRIGGER))
    .padding(0)
    .style(|theme: &Theme, status| {
        let tokens = Tokens::of(theme);
        let fill = if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            tokens.muted
        } else {
            Color::TRANSPARENT
        };
        button::Style {
            background: (fill.a > 0.0).then_some(fill.into()),
            text_color: tokens.muted_foreground,
            border: iced::border::rounded(theme::RADIUS_SM)
                .color(tokens.border)
                .width(1.0),
            shadow: Shadow::default(),
            snap: false,
        }
    })
    .on_press(Message::ToggleRail(false))
    .into()
}

/// The three figures above the table. They describe the workspace rather than
/// the selected destination, so they stay put as the rail moves.
fn figures<'a>() -> Element<'a, Message> {
    let cards = [
        ("12", "Active projects"),
        ("84%", "Tasks completed"),
        ("28", "Team members"),
    ];

    let mut row = row![].spacing(theme::SPACING_MD).width(Length::Fill);
    for (figure, label) in cards {
        row = row.push(Card::new().title(figure).description(label));
    }
    row.into()
}

/// The table's columns. Project takes what the fixed columns leave, which is
/// the same shape the Slint block writes as `width: 0`.
fn columns() -> [Column; 4] {
    [
        Column::new("project", "Project").sortable(),
        Column::new("owner", "Owner").width(160.0).sortable(),
        Column::new("state", "Status").width(120.0).sortable(),
        Column::new("updated", "Updated")
            .width(110.0)
            .align(Align::End)
            .sortable(),
    ]
}

/// The value a column sorts by. It is the display text, so the header's answer
/// and the row's answer are the same.
fn sort_value(project: &ProjectRow, column: usize) -> &str {
    match column {
        0 => &project.name,
        1 => &project.owner,
        2 => &project.state,
        _ => &project.updated,
    }
}

/// Every row, in the order the current sort draws them: the written projects
/// and the drafts the button added.
fn ordered_rows(app: &App) -> Vec<ProjectRow> {
    let mut rows: Vec<ProjectRow> = PROJECTS
        .into_iter()
        .map(|(name, owner, state, updated, status)| ProjectRow {
            name: name.to_string(),
            owner: owner.to_string(),
            state: state.to_string(),
            updated: updated.to_string(),
            status,
        })
        .collect();
    rows.extend(app.drafts.iter().map(|name| ProjectRow {
        name: name.clone(),
        owner: "You".to_string(),
        state: "Draft".to_string(),
        updated: "Just now".to_string(),
        status: Status::Info,
    }));
    if let Some((column, direction)) = app.sort {
        rows.sort_by(|a, b| {
            let order = sort_value(a, column).cmp(sort_value(b, column));
            match direction {
                ColumnSort::Descending => order.reverse(),
                _ => order,
            }
        });
    }
    rows
}

/// The table, bounded to the pane's remaining height so its header stays put
/// and its body scrolls. The rows are the caller's, already sorted and marked.
fn projects_table<'a>(app: &App, body_height: f32) -> Element<'a, Message> {
    let rows = ordered_rows(app).into_iter().map(|project| {
        Row::new(project.name.clone()).cells([
            Cell::new(project.name.clone()),
            Cell::new(project.owner.clone()),
            Cell::new(project.state.clone()).status(project.status),
            Cell::new(project.updated.clone()),
        ])
    });

    let mut table = DataTable::new()
        .columns(columns())
        .rows(rows)
        .stripe(true)
        .size(ControlSize::Small)
        .height(Length::Fixed(body_height))
        .on_sort(|column, direction| Message::Sort { column, direction })
        .on_select(Message::SelectRow);
    if let Some((column, direction)) = app.sort {
        table = table.sort(column, direction);
    }
    if let Some(selected) = app.selected_row {
        table = table.selected(Some(selected));
    }
    table.into()
}

/// The pane beside the rail: the destination, the figures, and the table.
fn pane(app: &App) -> Element<'_, Message> {
    let section = app.section;

    let mut heading = row![]
        .spacing(theme::SPACING_MD)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .height(Length::Shrink);
    if app.rail_collapsed {
        heading = heading.push(expand_trigger());
    }
    heading = heading
        .push(
            column![
                text(section.label())
                    .size(theme::FONT_SIZE_XL)
                    .font(Font {
                        weight: Weight::Semibold,
                        ..Font::DEFAULT
                    })
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).foreground),
                    }),
                text(section.caption())
                    .size(theme::FONT_SIZE_SM)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
            ]
            .spacing(4.0)
            .width(Length::Fill),
        )
        .push(
            Button::new("New project")
                .variant(ButtonVariant::Primary)
                .icon(glyph(PLUS, 16.0, |tokens: Tokens| {
                    tokens.primary_foreground
                }))
                .on_press(Message::NewProject),
        );

    let table = responsive(move |size| {
        let body_height = (size.height - TABLE_CHROME).max(MIN_TABLE_BODY);
        projects_table(app, body_height)
    });

    container(
        column![heading, figures(), table]
            .spacing(PANE_GAP)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(PANE_PADDING)
    .style(|theme: &Theme| container::Style {
        background: Some(Tokens::of(theme).background.into()),
        ..container::Style::default()
    })
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // The rail is a fixed 232px expanded (56px folded); the pane takes the
    // rest, so the navigation stays subordinate to the work it controls.
    row![rail(app), pane(app)]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
