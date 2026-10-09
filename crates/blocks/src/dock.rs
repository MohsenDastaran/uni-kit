//! An application shell: a toolbar over a dockable workspace, with a status bar
//! underneath.
//!
//! The workspace is a [`DockArea`] wearing the styled [`DockSkin`]: drag a tab
//! onto another group to join it, or onto an edge to split the centre. The
//! toolbar and the status bar are ordinary components around it, which is what
//! makes the shell a screen rather than a dock on its own.
//!
//! ```text
//! npx @dastaran/uni-kit@latest add gpui dock
//! ```
//!
//! The command writes `src/components/dock.rs` and adds `pub mod dock;` to
//! `src/components/mod.rs`. Add `mod components;` to the crate root, then:
//!
//! ```rust,ignore
//! use components::dock::DockBlock;
//!
//! gpui_kit::open_window(WindowOptions::default(), cx, |window, cx| {
//!     DockBlock::view(window, cx)
//! });
//! ```

use std::rc::Rc;

use gpui_kit::component::{
    ActiveTheme as _, Icon, IconName, Selectable as _, Sizable as _,
    button::{Button, ButtonVariants as _},
    dock::{
        BasePanel, DockArea, DockLayout, DockPlacement, DockSkin, Panel, PanelEvent, panel_handle,
    },
    h_flex,
    separator::Separator,
    status_bar::StatusBar,
    toolbar::Toolbar,
    v_flex,
};
use gpui_kit::{
    App, AppContext as _, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ParentElement, Render, SharedString, Styled, Window, div, px,
};

/// A panel body. A real application has one per view; here every panel is the
/// same type holding its own text, which is enough to show a workspace.
struct BlockPanel {
    name: &'static str,
    title: SharedString,
    body: SharedString,
    focus_handle: FocusHandle,
    closable: bool,
}

impl BlockPanel {
    fn new(
        name: &'static str,
        title: &'static str,
        body: &'static str,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| Self {
            name,
            title: title.into(),
            body: body.into(),
            focus_handle: cx.focus_handle(),
            closable: true,
        })
    }
}

impl EventEmitter<PanelEvent> for BlockPanel {}

impl Focusable for BlockPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl BasePanel for BlockPanel {
    fn panel_name(&self) -> &'static str {
        self.name
    }

    fn closable(&self, _: &App) -> bool {
        self.closable
    }
}

impl Panel for BlockPanel {
    fn title(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.title.clone()
    }
}

impl Render for BlockPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_4()
            .text_color(cx.theme().foreground)
            .child(self.body.clone())
    }
}

/// The block. Hold it in a window and render it; the dock owns the layout, the
/// toolbar drives it.
pub struct DockBlock {
    dock_area: Entity<DockArea>,
    skin: Rc<DockSkin>,
    close_buttons: bool,
}

impl DockBlock {
    /// The view to hand to a window: `cx.new(|cx| DockBlock::new(window, cx))`.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (dock_area, skin) = DockSkin::dock_area("dock-block", Some(1), window, cx);

        let explorer = BlockPanel::new(
            "DockBlockExplorer",
            "Explorer",
            "Drag this tab into another group.",
            cx,
        );
        let search = BlockPanel::new(
            "DockBlockSearch",
            "Search",
            "Two panels can share one tab group.",
            cx,
        );
        let editor = BlockPanel::new(
            "DockBlockEditor",
            "main.rs",
            "Drop a tab near an edge to split this group.",
            cx,
        );
        let terminal = BlockPanel::new(
            "DockBlockTerminal",
            "Terminal",
            "The bottom dock shares the workspace column.",
            cx,
        );
        let problems =
            BlockPanel::new("DockBlockProblems", "Problems", "No problems detected.", cx);

        // The centre is the document area: a narrow explorer, a centre that
        // takes the surplus, and a terminal band. The centre's own tab cannot be
        // closed, because closing it would leave the workspace empty.
        dock_area.update(cx, |area, cx| {
            editor.update(cx, |editor, _| editor.closable = false);

            area.set_center(
                DockLayout::h_split()
                    .child(
                        DockLayout::tabs()
                            .panel_view(panel_handle(explorer), cx)
                            .panel_view(panel_handle(search), cx),
                        Some(px(240.)),
                    )
                    .child(
                        DockLayout::tabs().panel_view(panel_handle(editor), cx),
                        None,
                    ),
                window,
                cx,
            );
            area.set_dock(
                DockPlacement::Bottom,
                DockLayout::tabs()
                    .panel_view(panel_handle(terminal), cx)
                    .panel_view(panel_handle(problems), cx),
                window,
                cx,
            );
            area.set_dock_size(DockPlacement::Bottom, px(160.), window, cx);
            area.set_dock_collapsible(DockPlacement::Bottom, true, window, cx);
        });
        skin.set_toggle_button_visible(true, cx);

        Self {
            dock_area,
            skin,
            close_buttons: false,
        }
    }

    /// The same view, already built.
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    /// The commands above the workspace.
    fn toolbar(&self, cx: &mut Context<Self>) -> Toolbar {
        let close_buttons = self.close_buttons;

        Toolbar::new("dock-block-toolbar")
            .child(
                Button::new("open")
                    .icon(IconName::FolderOpen)
                    .label("Open")
                    .on_click(|_, _, _| {}),
            )
            .child(
                Button::new("find")
                    .icon(IconName::Search)
                    .tooltip("Find in files"),
            )
            .content(Separator::vertical().h_5())
            .child(
                Button::new("run")
                    .icon(IconName::Play)
                    .label("Run")
                    .primary(),
            )
            // The spacer keeps the commands at the leading edge; the last button
            // sits at the trailing one.
            .content(div().flex_1())
            .child(
                Button::new("tab-close")
                    .icon(IconName::Close)
                    .tooltip("Tab close buttons")
                    .selected(close_buttons)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.close_buttons = !this.close_buttons;
                        this.skin.set_close_button_visible(this.close_buttons, cx);
                        cx.notify();
                    })),
            )
    }

    /// The row under the workspace: what is open, and where the caret is.
    fn status_bar(&self) -> StatusBar {
        StatusBar::new()
            .left(
                Button::new("branch")
                    .ghost()
                    .xsmall()
                    .icon(IconName::Github)
                    .label("main"),
            )
            .left(Separator::vertical())
            .left(
                h_flex()
                    .items_center()
                    .gap_1()
                    .child(Icon::new(IconName::CircleCheck).xsmall())
                    .child("0 problems"),
            )
            .right("Rust")
            .right(Separator::vertical())
            .right("Ln 1, Col 1")
    }
}

impl Render for DockBlock {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let toolbar = self.toolbar(cx);
        let status_bar = self.status_bar();

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(
                div()
                    .flex_shrink_0()
                    .px_2()
                    .py_1()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(toolbar),
            )
            .child(div().min_h_0().flex_1().child(self.dock_area.clone()))
            .child(
                div()
                    .flex_shrink_0()
                    .border_t_1()
                    .border_color(cx.theme().border)
                    .child(status_bar),
            )
    }
}
