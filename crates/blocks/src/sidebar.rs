//! A workspace shell: a navigation rail beside the data it controls.
//!
//! The rail is a [`Sidebar`], the panel is a heading, three figures and a
//! [`DataTable`]. Selecting a rail entry moves the panel with it, so the two
//! halves read as one screen rather than as a sidebar on its own.
//!
//! ```text
//! npx @dastaran/uni-kit@latest add gpui sidebar
//! ```
//!
//! The command writes `src/components/sidebar.rs` and adds `pub mod sidebar;`
//! to `src/components/mod.rs`. Add `mod components;` to the crate root, then:
//!
//! ```rust,ignore
//! use components::sidebar::SidebarBlock;
//!
//! gpui_kit::open_window(WindowOptions::default(), cx, |window, cx| {
//!     SidebarBlock::view(window, cx)
//! });
//! ```

use gpui_kit::component::{
    ActiveTheme as _, Icon, IconName, Sizable as _, StyledExt as _,
    button::{Button, ButtonVariants as _},
    card::{Card, CardContent, CardHeader},
    h_flex,
    sidebar::{
        Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
        SidebarToggleButton,
    },
    table::{Column, ColumnSort, DataTable, TableDelegate, TableState},
    tag::Tag,
    v_flex,
};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::{
    App, AppContext as _, Context, Div, Entity, IntoElement, ParentElement, Render, Styled,
    TextAlign, Window, div, px,
};

/// One row of the table. `&'static str` because the rows are written here
/// rather than loaded: a block is a screen, not a data layer.
struct Project {
    name: &'static str,
    owner: &'static str,
    status: &'static str,
    updated: &'static str,
}

/// The rows, as they are written: project, owner, status, last update.
const ROWS: [(&str, &str, &str, &str); 8] = [
    ("Atlas", "Dana Whitfield", "Active", "2 hours ago"),
    ("Beacon", "Ravi Menon", "Active", "Yesterday"),
    ("Compass", "Lena Ortiz", "Paused", "3 days ago"),
    ("Delta Sync", "Tomas Berg", "Active", "4 days ago"),
    ("Ember", "Aiko Tanaka", "Done", "1 week ago"),
    ("Foundry", "Noah Clarke", "Active", "1 week ago"),
    ("Granite", "Priya Raman", "Paused", "2 weeks ago"),
    ("Harbour", "Sofia Almeida", "Done", "3 weeks ago"),
];

/// The rail entry that is selected. The rail sets it and the panel reads it, so
/// the two halves are one state rather than two.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    Dashboard,
    Projects,
    Reports,
    Team,
    Billing,
    Settings,
}

impl Section {
    fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Projects => "Projects",
            Self::Reports => "Reports",
            Self::Team => "Team",
            Self::Billing => "Billing",
            Self::Settings => "Settings",
        }
    }

    fn caption(self) -> &'static str {
        match self {
            Self::Dashboard => "A quick view of your workspace activity.",
            Self::Projects => "Every project you can reach, and where it stands.",
            Self::Reports => "The numbers behind the last quarter.",
            Self::Team => "Who is in the workspace and what they can do.",
            Self::Billing => "Your plan, your invoices and your limits.",
            Self::Settings => "How the workspace behaves for everyone in it.",
        }
    }

    fn icon(self) -> IconName {
        match self {
            Self::Dashboard => IconName::LayoutDashboard,
            Self::Projects => IconName::Folder,
            Self::Reports => IconName::ChartPie,
            Self::Team => IconName::User,
            Self::Billing => IconName::FileText,
            Self::Settings => IconName::Settings2,
        }
    }
}

/// The table's rows and columns. A block writes its own delegate, because the
/// table renders what the delegate hands it: the screen decides the columns,
/// and the component decides how a table behaves.
struct ProjectTable {
    columns: Vec<Column>,
    rows: Vec<Project>,
}

impl ProjectTable {
    fn new() -> Self {
        Self {
            columns: vec![
                Column::new("project", "Project").sortable().width(200.),
                Column::new("owner", "Owner").sortable().width(180.),
                Column::new("status", "Status").width(110.),
                Column::new("updated", "Updated").text_right().width(130.),
            ],
            rows: ROWS
                .into_iter()
                .map(|(name, owner, status, updated)| Project {
                    name,
                    owner,
                    status,
                    updated,
                })
                .collect(),
        }
    }

    /// The cell frame. Every cell fills its row so the columns share one
    /// baseline, and a right-aligned column lines up with its heading.
    fn cell(&self, col: &Column) -> Div {
        div()
            .h_full()
            .h_flex()
            .items_center()
            .when(col.align == TextAlign::Right, |this| this.justify_end())
    }
}

/// A status as the tags the rest of the screen wears.
fn status_tag(status: &str) -> Tag {
    match status {
        "Active" => Tag::success().outline().child(status.to_string()),
        "Paused" => Tag::warning().outline().child(status.to_string()),
        _ => Tag::new().outline().child(status.to_string()),
    }
    .xsmall()
}

impl TableDelegate for ProjectTable {
    fn columns_count(&self, _: &App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _: &App) -> usize {
        self.rows.len()
    }

    fn column(&self, col_ix: usize, _: &App) -> Column {
        self.columns[col_ix].clone()
    }

    fn perform_sort(
        &mut self,
        col_ix: usize,
        sort: ColumnSort,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) {
        // The key is cloned so the sort does not hold a borrow of `columns`
        // while it sorts `rows`.
        let Some(key) = self.columns.get(col_ix).map(|col| col.key.clone()) else {
            return;
        };

        // `Default` is the third click: the arrow leaves the heading and the
        // rows return to the order they were written in.
        let ordered = |a: &Project, b: &Project| match key.as_ref() {
            "owner" => a.owner.cmp(b.owner),
            _ => a.name.cmp(b.name),
        };
        self.rows.sort_by(|a, b| match sort {
            ColumnSort::Descending => ordered(b, a),
            _ => ordered(a, b),
        });
    }

    fn render_td(
        &mut self,
        row_ix: usize,
        col_ix: usize,
        _: &mut Window,
        _: &mut Context<TableState<Self>>,
    ) -> impl IntoElement {
        let (Some(row), Some(col)) = (self.rows.get(row_ix), self.columns.get(col_ix)) else {
            return div().into_any_element();
        };

        match col.key.as_ref() {
            "project" => self.cell(col).child(row.name).into_any_element(),
            "owner" => self.cell(col).child(row.owner).into_any_element(),
            "status" => self
                .cell(col)
                .child(status_tag(row.status))
                .into_any_element(),
            "updated" => self.cell(col).child(row.updated).into_any_element(),
            _ => div().into_any_element(),
        }
    }
}

/// The block. Hold it in a window and render it; the rail and the table keep
/// their own state.
pub struct SidebarBlock {
    section: Section,
    collapsed: bool,
    table: Entity<TableState<ProjectTable>>,
}

impl SidebarBlock {
    /// The view to hand to a window:
    /// `cx.new(|cx| SidebarBlock::new(window, cx))`.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            section: Section::Projects,
            collapsed: false,
            table: cx.new(|cx| TableState::new(ProjectTable::new(), window, cx)),
        }
    }

    /// The same view, already built.
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    /// One rail entry. Clicking it selects the section the panel shows.
    fn nav_item(&self, value: Section, cx: &mut Context<Self>) -> SidebarMenuItem {
        SidebarMenuItem::new(value.label())
            .icon(value.icon())
            .active(self.section == value)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.section = value;
                cx.notify();
            }))
    }

    /// The rail: a brand header, two groups of navigation and the person the
    /// workspace belongs to.
    fn rail(&self, cx: &mut Context<Self>) -> Sidebar<SidebarGroup<SidebarMenu>> {
        let collapsed = self.collapsed;

        Sidebar::new("sidebar-block")
            .collapsible(true)
            .collapsed(collapsed)
            .w(px(240.))
            .header(
                SidebarHeader::new()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .size_8()
                            .flex_shrink_0()
                            .rounded(cx.theme().radius)
                            .bg(cx.theme().sidebar_primary)
                            .text_color(cx.theme().sidebar_primary_foreground)
                            .child(Icon::new(IconName::GalleryVerticalEnd)),
                    )
                    .when(!collapsed, |this| {
                        this.child(
                            v_flex().flex_1().overflow_hidden().child("Acme Inc").child(
                                div()
                                    .text_xs()
                                    .text_color(cx.theme().sidebar_foreground.opacity(0.7))
                                    .child("Enterprise"),
                            ),
                        )
                    }),
            )
            .child(
                SidebarGroup::new("Workspace").child(SidebarMenu::new().children([
                    self.nav_item(Section::Dashboard, cx),
                    self.nav_item(Section::Projects, cx),
                    self.nav_item(Section::Reports, cx),
                ])),
            )
            .child(
                SidebarGroup::new("Manage").child(SidebarMenu::new().children([
                    self.nav_item(Section::Team, cx),
                    self.nav_item(Section::Billing, cx),
                    self.nav_item(Section::Settings, cx),
                ])),
            )
            .footer(
                SidebarFooter::new().child(
                    h_flex()
                        .gap_2()
                        .child(Icon::new(IconName::CircleUser))
                        .when(!collapsed, |this| this.child("Dana Whitfield")),
                ),
            )
    }

    /// The three figures above the table. They describe the workspace rather
    /// than the selected section, so they stay put as the selection moves.
    fn stats(&self, cx: &mut Context<Self>) -> Div {
        let figures: [(&'static str, &'static str, IconName); 3] = [
            ("12", "Active projects", IconName::Folder),
            ("84%", "Tasks completed", IconName::CircleCheck),
            ("28", "Team members", IconName::User),
        ];

        h_flex()
            .w_full()
            .gap_3()
            .children(figures.map(|(figure, label, icon)| {
                Card::new()
                    .flex_1()
                    .child(
                        CardHeader::new().child(
                            h_flex()
                                .w_full()
                                .items_center()
                                .justify_between()
                                .gap_2()
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(cx.theme().muted_foreground)
                                        .child(label),
                                )
                                .child(
                                    Icon::new(icon)
                                        .small()
                                        .text_color(cx.theme().muted_foreground),
                                ),
                        ),
                    )
                    .child(CardContent::new().child(div().text_2xl().font_semibold().child(figure)))
            }))
    }
}

impl Render for SidebarBlock {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rail = self.rail(cx);
        let stats = self.stats(cx);
        let section = self.section;
        let collapsed = self.collapsed;

        h_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(rail)
            .child(
                v_flex()
                    .h_full()
                    .min_w_0()
                    .flex_1()
                    .gap_4()
                    .p_4()
                    .child(
                        h_flex()
                            .w_full()
                            .items_center()
                            .gap_3()
                            .child(SidebarToggleButton::new().collapsed(collapsed).on_click(
                                cx.listener(|this, _, _, cx| {
                                    this.collapsed = !this.collapsed;
                                    cx.notify();
                                }),
                            ))
                            .child(
                                v_flex()
                                    .min_w_0()
                                    .flex_1()
                                    .gap_1()
                                    .child(div().text_xl().font_semibold().child(section.label()))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(section.caption()),
                                    ),
                            )
                            .child(
                                Button::new("new-project")
                                    .icon(IconName::Plus)
                                    .label("New project")
                                    .primary(),
                            ),
                    )
                    .child(stats)
                    .child(
                        div()
                            .min_h_0()
                            .flex_1()
                            .child(DataTable::new(&self.table).stripe(true)),
                    ),
            )
    }
}
