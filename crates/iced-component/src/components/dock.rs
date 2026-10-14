//! A dock workspace.
//!
//! The registry key is `dock`; the file and the module share the name.
//!
//! # What this is, and what the reference is
//!
//! Slint's `dock.slint` and GPUI's `crates/base/src/dock` are **a workspace
//! layout system**: a tree of split and tab nodes, drag-and-drop between them,
//! zoom, a lock, and a serialized layout. iced has no layout engine to hang that
//! on, and pretending it does would be a tiling window manager written badly. So
//! this port is the part a component can honestly express, and the rest is named
//! below rather than faked:
//!
//! - **four regions** — left, centre, right, bottom — with **one tab group
//!   each**, the shape the Slint example draws;
//! - a **tab strip** per group: click to select, a close control on the active
//!   closable tab, and **drag a tab along its own strip** to reorder it — the
//!   drop lands before the tab under the pointer, or at the strip's tail;
//! - a **live splitter** on each edge dock: dragging it reports every pixel, the
//!   same widget-tree drag [`ResizeHandle`](super::resizable::ResizeHandle) makes,
//!   with the right and bottom docks inverted so the dragged edge stays under the
//!   pointer;
//! - **collapsing** an edge dock to a rail or a bar, and restoring it.
//!
//! # Named, not faked
//!
//! - **A region is one tab group, and the pointer drag reorders inside it.** The
//!   Slint model keeps up to four groups per region, drops a tab on another
//!   group's strip to join it, and splits the centre when a tab is dropped on a
//!   group's left or right edge. Here a region is one group, so the drag replaces
//!   a tab's position within its own strip; the two Slint moves that cross a
//!   strip — join another group, dock beside a group — are left out.
//!   [`Dock::on_move`] is region-aware, so an application can move a panel into
//!   another region from its own command or menu; the pointer cannot carry it
//!   there.
//! - **No zoom and no layout lock.** Both are Slint toolbar state; neither is
//!   part of the four-region shape this port keeps.
//! - **No persistence.** Slint's `DockState` is a value a view can serialize;
//!   iced 0.14 keeps the caller's own Rust values and has no layout schema, so
//!   "restore a layout" is the application re-creating its panel list. The demo
//!   does exactly that, and says so.
//! - **No viewport breakpoints.** Slint collapses the edge docks below 660px and
//!   refuses to split the centre below 880px; iced's component is handed no
//!   viewport width, so `collapsed` is the caller's decision.
//! - **No transition.** A region snaps between its pane and its rail, as the
//!   Slint one does.
//! - **The close control is on the active closable tab**, where Slint reveals it
//!   on hover *or* when active. Hover is kept in the strip's own widget-tree
//!   state and is not visible to the `Dock` that builds the tab children.
//!
//! # Three things costs that were paid on purpose
//!
//! - **The divider is this file's own, not `resizable`'s.** `ResizeHandle`
//!   reports `start + delta` on one axis with no way to invert it, and the right
//!   dock has to report `start - delta`; extending a landed component's file to
//!   add an `inverted` builder is a change to somebody else's work, so the dock
//!   carries a five-line-shape divider of its own, in the same colour and with
//!   the same widget-tree drag. The alternative was a wrong-signed right edge.
//! - **The tab strip is a custom [`Widget`] with children.** Measuring a tab's
//!   width is what makes a drop land between the right pair of tabs, and a
//!   stateless `mouse_area` cannot know its own children's bounds. The strip
//!   keeps the pointer's press, the tab under it and the insertion index in
//!   [`tree::State`], so the drop indicator is live rather than a value that
//!   appears on release.
//! - **Every colour is read while iced renders.** A theme switch repaints the
//!   dock because each surface resolves `Tokens::of(theme)` inside its style
//!   closure or its `draw`, not while the element is built.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one part gets
// a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::advanced::Renderer as _;
use iced::advanced::layout::{self, Limits};
use iced::advanced::renderer;
use iced::advanced::widget::Tree;
use iced::advanced::widget::tree::{self, Tag};
use iced::advanced::{Clipboard, Layout, Shell, Widget, mouse, overlay};
use iced::widget::{button, column, container, row, svg, text};
use iced::widget::container::Style as ContainerStyle;
use iced::{Alignment, Border, Element, Event, Font, Length, Padding, Point, Rectangle, Size, Theme,
           Vector, font, touch};

use super::theme::{self, Tokens};

/// A panel's identity. It is stable across a move, which is what a reorder and a
/// close address rather than a position.
pub type PanelId = u64;

/// The tab strip's height, Slint's `DockTab` `height: 30px`.
const HEADER_HEIGHT: f32 = 30.0;
/// The strip's border, Slint's one-pixel `Theme.border` edge.
const STRIP_BORDER: f32 = 1.0;
/// The collapse control's width, Slint's 26px header button.
const TOGGLE_WIDTH: f32 = 26.0;
/// A collapsed left or right dock: Slint's 34px rail.
const RAIL_WIDTH: f32 = 34.0;
/// The divider's thickness across the axis it separates, Slint's 5px splitter.
const DIVIDER_THICKNESS: f32 = 5.0;
/// How far the divider's hit box reaches into each neighbouring pane, as Slint's
/// `TouchArea` does.
const DIVIDER_OVERHANG: f32 = 4.0;
/// The distance a press must travel before it is a drag and not a click, Slint's
/// four-pixel threshold.
const DRAG_THRESHOLD: f32 = 4.0;
/// The no-close-tab right padding, Slint's `padding-right: 12px`.
const TAB_PADDING: f32 = 12.0;
/// The close-tab right padding, Slint's `padding-right: 8px`.
const TAB_CLOSE_PADDING: f32 = 8.0;
/// A close control's box, Slint's 16x16 glyph.
const CLOSE_SIZE: f32 = 16.0;
/// The Lucide `x` from `crates/slint-component/ui/icons/x.svg` (Lucide, ISC):
/// the tab close mark, in the same one-line form the crate's other embedded
/// icons use.
const CLOSE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>"#;
/// The active tab's underline, Slint's two-pixel primary rule inset ten pixels.
const UNDERLINE_HEIGHT: f32 = 2.0;
const UNDERLINE_INSET: f32 = 10.0;

/// Which dock a panel sits in. The names and the four-way shape are Slint's
/// `DockRegion`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Region {
    #[default]
    Left,
    Center,
    Right,
    Bottom,
}

impl Region {
    /// Every region, in the order the layout reads: left, centre, right, bottom.
    pub const ALL: [Region; 4] = [Region::Left, Region::Center, Region::Right, Region::Bottom];

    /// The region's slot in the four fixed tables below.
    pub const fn index(self) -> usize {
        match self {
            Region::Left => 0,
            Region::Center => 1,
            Region::Right => 2,
            Region::Bottom => 3,
        }
    }

    /// The region's starting size, Slint's `region-size: [208px, 480px, 208px,
    /// 176px]`. The centre's is the width its pane opens at; it is not resizable.
    pub const fn default_size(self) -> f32 {
        match self {
            Region::Left => 208.0,
            Region::Center => 480.0,
            Region::Right => 208.0,
            Region::Bottom => 176.0,
        }
    }

    /// The floor a drag clamps to, Slint's `set-region-size` table: 140px for a
    /// side dock, 280px for the centre, 96px for the bottom.
    pub const fn minimum(self) -> f32 {
        match self {
            Region::Left | Region::Right => 140.0,
            Region::Center => 280.0,
            Region::Bottom => 96.0,
        }
    }

    /// The ceiling a drag clamps to, Slint's table: 420px for a side dock,
    /// 960px for the centre, 420px for the bottom.
    pub const fn maximum(self) -> f32 {
        match self {
            Region::Left | Region::Right => 420.0,
            Region::Center => 960.0,
            Region::Bottom => 420.0,
        }
    }

    /// `size` held inside the region's own floor and ceiling, which is what Slint
    /// applies before it writes a dragged length back.
    pub fn clamp_size(self, size: f32) -> f32 {
        size.clamp(self.minimum(), self.maximum())
    }

    /// Whether the region is a side dock, which is everything but the centre.
    pub fn is_edge(self) -> bool {
        self != Region::Center
    }
}

/// One panel: its identity, its tab's title, the dock it belongs to, whether its
/// tab can be closed, and its body.
///
/// The order panels are given to [`Dock`] in is the order their tabs are drawn
/// and the order [`Dock::on_move`] reports against.
pub struct Panel<'a, Message> {
    id: PanelId,
    title: String,
    region: Region,
    closable: bool,
    content: Element<'a, Message>,
}

impl<'a, Message> Panel<'a, Message> {
    /// A closable panel in the left dock, titled `title`, with `content` as its
    /// body.
    pub fn new(
        id: PanelId,
        title: impl Into<String>,
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            id,
            title: title.into(),
            region: Region::default(),
            closable: true,
            content: content.into(),
        }
    }

    /// The dock the panel starts in. Slint's `DockPanelDef.region`.
    pub fn region(mut self, region: Region) -> Self {
        self.region = region;
        self
    }

    /// Whether the tab carries a close control. Slint's `DockPanelDef.closable`.
    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
}

/// A dock workspace: the panels, which one is active in each region, which edge
/// docks are collapsed, and the region sizes.
///
/// [`Dock`] is a value built fresh each frame from the caller's state, exactly as
/// [`Slider`](super::slider) and [`ResizeHandle`](super::resizable::ResizeHandle)
/// are: `active`, `collapsed` and the sizes are read in, and every pointer
/// operation is reported back as a message for the application to write.
pub struct Dock<'a, Message> {
    panels: Vec<Panel<'a, Message>>,
    active: [Option<PanelId>; 4],
    collapsed: [bool; 4],
    sizes: [f32; 4],
    on_select: Option<Rc<dyn Fn(PanelId) -> Message + 'a>>,
    on_close: Option<Rc<dyn Fn(PanelId) -> Message + 'a>>,
    on_move: Option<Rc<dyn Fn(PanelId, Region, Option<PanelId>) -> Message + 'a>>,
    on_resize: Option<Rc<dyn Fn(Region, f32) -> Message + 'a>>,
    on_toggle: Option<Rc<dyn Fn(Region) -> Message + 'a>>,
}

impl<'a, Message> Dock<'a, Message> {
    /// A workspace holding `panels`, with no panel active, no dock collapsed and
    /// every region at [`Region::default_size`].
    pub fn new(panels: impl IntoIterator<Item = Panel<'a, Message>>) -> Self {
        Self {
            panels: panels.into_iter().collect(),
            active: [None; 4],
            collapsed: [false; 4],
            sizes: [
                Region::Left.default_size(),
                Region::Center.default_size(),
                Region::Right.default_size(),
                Region::Bottom.default_size(),
            ],
            on_select: None,
            on_close: None,
            on_move: None,
            on_resize: None,
            on_toggle: None,
        }
    }

    /// Which panel is drawn as the region's active tab. A region left unset shows
    /// its first panel, which is what Slint's model selects on reset.
    pub fn active(mut self, region: Region, panel: Option<PanelId>) -> Self {
        self.active[region.index()] = panel;
        self
    }

    /// Whether an edge dock is drawn as its rail instead of its pane. Slint's
    /// `collapsed[region]`.
    pub fn collapsed(mut self, region: Region, collapsed: bool) -> Self {
        self.collapsed[region.index()] = collapsed;
        self
    }

    /// An edge dock's size, in pixels: a width for left and right, a height for
    /// the bottom. The value is not clamped here — the caller keeps a size that
    /// has already been through [`Region::clamp_size`], or the first paint is
    /// outside the floor the first drag would apply.
    pub fn size(mut self, region: Region, size: f32) -> Self {
        self.sizes[region.index()] = size;
        self
    }

    /// A tab was chosen. Selecting a panel in a collapsed dock is reported here
    /// too, and the application decides whether to expand it.
    pub fn on_select(mut self, on_select: impl Fn(PanelId) -> Message + 'a) -> Self {
        self.on_select = Some(Rc::new(on_select));
        self
    }

    /// A tab's close control was pressed. A panel with no `on_close` draws no
    /// close control, because iced spells a disabled control as the absence of a
    /// message.
    pub fn on_close(mut self, on_close: impl Fn(PanelId) -> Message + 'a) -> Self {
        self.on_close = Some(Rc::new(on_close));
        self
    }

    /// A tab was dragged to a new position. `panel` belongs in `region`,
    /// immediately before `before` — or at the end of the region when `before` is
    /// `None`. The panel is still in the caller's own list; the move is the
    /// caller's to make, exactly as a slider's value is.
    ///
    /// The pointer reports a reorder inside `panel`'s own region, so `region` is
    /// that region and `before` is one of its panels. The message carries the
    /// region so an application can also move a panel between regions through a
    /// command of its own; the tab drag does not cross strips.
    pub fn on_move(
        mut self,
        on_move: impl Fn(PanelId, Region, Option<PanelId>) -> Message + 'a,
    ) -> Self {
        self.on_move = Some(Rc::new(on_move));
        self
    }

    /// An edge dock's divider was dragged: the size a drag asks for, already
    /// clamped by the region's own bounds.
    pub fn on_resize(mut self, on_resize: impl Fn(Region, f32) -> Message + 'a) -> Self {
        self.on_resize = Some(Rc::new(on_resize));
        self
    }

    /// An edge dock's collapse control was pressed.
    pub fn on_toggle(mut self, on_toggle: impl Fn(Region) -> Message + 'a) -> Self {
        self.on_toggle = Some(Rc::new(on_toggle));
        self
    }
}

/// Which way a divider separates its neighbours: a vertical strip between
/// columns, or a horizontal strip between rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    /// The coordinate a drag travels along.
    fn position(self, point: Point) -> f32 {
        match self {
            Self::Horizontal => point.x,
            Self::Vertical => point.y,
        }
    }

    /// The strip's size: fixed across, filling along.
    fn size(self) -> Size<Length> {
        match self {
            Self::Horizontal => Size::new(Length::Fixed(DIVIDER_THICKNESS), Length::Fill),
            Self::Vertical => Size::new(Length::Fill, Length::Fixed(DIVIDER_THICKNESS)),
        }
    }
}

/// The divider between a dock and the region beside it.
///
/// It is [`ResizeHandle`](super::resizable::ResizeHandle)'s sibling with two
/// differences, and both are why it is written here rather than imported: the
/// axis is derived from the `Region`, and `inverted` flips the sign so the right
/// and bottom docks grow when their edge is dragged *towards* the centre.
struct Divider<'a, Message> {
    region: Region,
    inverted: bool,
    split: f32,
    on_resize: Option<Rc<dyn Fn(Region, f32) -> Message + 'a>>,
}

/// The pointer's hold on one divider, kept in the widget tree across the frames
/// a drag spans.
#[derive(Debug, Default)]
struct DividerDrag {
    dragging: bool,
    /// iced draws only when asked to, so the strip's hover colour needs this flag
    /// and a redraw when it changes.
    hovered: bool,
    press: f32,
    start_split: f32,
}

impl<'a, Message> Divider<'a, Message> {
    fn new(region: Region, inverted: bool, split: f32) -> Self {
        Self {
            region,
            inverted,
            split,
            on_resize: None,
        }
    }

    fn on_resize(
        mut self,
        on_resize: impl Fn(Region, f32) -> Message + 'a,
    ) -> Self {
        self.on_resize = Some(Rc::new(on_resize));
        self
    }

    /// The left and right docks are columns, so their divider is a vertical
    /// strip; the bottom dock is a row, so its divider is horizontal.
    fn axis(&self) -> Axis {
        match self.region {
            Region::Bottom => Axis::Vertical,
            _ => Axis::Horizontal,
        }
    }

    fn is_enabled(&self) -> bool {
        self.on_resize.is_some()
    }

    fn clamp(&self, next: f32) -> f32 {
        self.region.clamp_size(next)
    }

    fn grip(&self, bounds: Rectangle) -> Rectangle {
        bounds.expand(DIVIDER_OVERHANG)
    }

    fn report(&self, next: f32, shell: &mut Shell<'_, Message>) {
        if (next - self.split).abs() > f32::EPSILON
            && let Some(on_resize) = &self.on_resize
        {
            shell.publish(on_resize(self.region, next));
        }
    }
}

impl<'a, Message> Widget<Message, Theme, iced::Renderer> for Divider<'a, Message>
where
    Message: 'a,
{
    fn tag(&self) -> Tag {
        Tag::of::<DividerDrag>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(DividerDrag::default())
    }

    fn size(&self) -> Size<Length> {
        self.axis().size()
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &Limits,
    ) -> layout::Node {
        let size = self.axis().size();
        layout::atomic(limits, size.width, size.height)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let drag = tree.state.downcast_mut::<DividerDrag>();
        let grip = self.grip(layout.bounds());
        let axis = self.axis();
        let enabled = self.is_enabled();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if enabled && let Some(position) = cursor.position_over(grip) {
                    drag.dragging = true;
                    drag.press = axis.position(position);
                    drag.start_split = self.split;
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                drag.dragging = false;
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let over = cursor.is_over(grip);
                if over != drag.hovered {
                    drag.hovered = over;
                    shell.request_redraw();
                }
                if drag.dragging && enabled {
                    if let Some(position) = cursor.land().position() {
                        let delta = axis.position(position) - drag.press;
                        let delta = if self.inverted { -delta } else { delta };
                        self.report(self.clamp(drag.start_split + delta), shell);
                    }
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorLeft) => {
                if drag.hovered {
                    drag.hovered = false;
                    shell.request_redraw();
                }
            }
            Event::Touch(touch::Event::FingerPressed { position, .. }) => {
                if enabled && grip.contains(*position) {
                    drag.dragging = true;
                    drag.press = axis.position(*position);
                    drag.start_split = self.split;
                    shell.capture_event();
                }
            }
            Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                drag.dragging = false;
            }
            Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if drag.dragging && enabled {
                    let delta = axis.position(*position) - drag.press;
                    let delta = if self.inverted { -delta } else { delta };
                    self.report(self.clamp(drag.start_split + delta), shell);
                    shell.capture_event();
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let tokens = Tokens::of(theme);
        let drag = tree.state.downcast_ref::<DividerDrag>();
        let active = drag.dragging || drag.hovered;
        let color = if active {
            theme::alpha(tokens.primary, 0.5)
        } else {
            tokens.border
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                ..renderer::Quad::default()
            },
            color,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let dragging = tree.state.downcast_ref::<DividerDrag>().dragging;
        if self.is_enabled() && (dragging || cursor.is_over(self.grip(layout.bounds()))) {
            match self.axis() {
                Axis::Horizontal => mouse::Interaction::ResizingHorizontally,
                Axis::Vertical => mouse::Interaction::ResizingVertically,
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message> From<Divider<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(divider: Divider<'a, Message>) -> Self {
        Element::new(divider)
    }
}

/// The pointer's hold on one tab strip, kept in the widget tree across the frames
/// a drag spans. The strip's layout is measured once per frame and read back from
/// the [`Layout`] in `update`, so the bounds are never stored here.
#[derive(Debug, Default)]
struct StripDrag {
    /// The tab the pointer is over, for its hover fill.
    hovered: Option<usize>,
    /// The tab a press landed on, until it is released or becomes a drag.
    pressed: Option<usize>,
    press_x: f32,
    dragging: bool,
    /// Where the dragged tab would be inserted: before this tab, or at the end.
    drop: usize,
}

/// A group's tab strip: Slint's `DockHeader` without the collapse control, which
/// the pane draws beside it.
///
/// It lays its tab children out itself, which is what lets a drag land between
/// the right pair of tabs; the children are the caller's labels and close
/// controls, and every hover, press and drop is this widget's own state.
struct TabStrip<'a, Message> {
    region: Region,
    ids: Vec<PanelId>,
    children: Vec<Element<'a, Message>>,
    active: Option<PanelId>,
    on_select: Option<Rc<dyn Fn(PanelId) -> Message + 'a>>,
    on_move: Option<Rc<dyn Fn(PanelId, Region, Option<PanelId>) -> Message + 'a>>,
}

impl<'a, Message> TabStrip<'a, Message> {
    /// The index of the tab whose bounds hold `point`, if any.
    fn index_at(&self, layout: Layout<'_>, point: Point) -> Option<usize> {
        layout
            .children()
            .position(|child| child.bounds().contains(point))
    }

    /// Where an insertion at `point` goes: before the first tab whose centre is
    /// to the right of the pointer, or at the end.
    fn drop_at(&self, layout: Layout<'_>, point: Point) -> usize {
        layout
            .children()
            .position(|child| point.x < child.bounds().center_x())
            .unwrap_or(self.children.len())
    }

    /// The x an insertion indicator is drawn at.
    fn insertion_x(&self, layout: Layout<'_>, drop: usize) -> f32 {
        let mut children = layout.children();
        if drop == 0 {
            return children.next().map_or_else(
                || layout.bounds().x,
                |child| child.bounds().x,
            );
        }
        let mut x = layout.bounds().x + layout.bounds().width;
        for (i, child) in children.enumerate() {
            if i + 1 == drop {
                x = child.bounds().x + child.bounds().width;
                break;
            }
            if i == drop {
                x = child.bounds().x;
                break;
            }
        }
        x
    }
}

impl<'a, Message> Widget<Message, Theme, iced::Renderer> for TabStrip<'a, Message>
where
    Message: 'a,
{
    fn tag(&self) -> Tag {
        Tag::of::<StripDrag>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(StripDrag::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(HEADER_HEIGHT))
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &Limits,
    ) -> layout::Node {
        let size = limits.resolve(
            Length::Fill,
            Length::Fixed(HEADER_HEIGHT),
            Size::new(0.0, HEADER_HEIGHT),
        );
        let mut x = 0.0;
        let mut nodes = Vec::with_capacity(self.children.len());
        for (child, state) in self.children.iter_mut().zip(tree.children.iter_mut()) {
            let child_limits = Limits::new(
                Size::ZERO,
                Size::new((size.width - x).max(0.0), HEADER_HEIGHT),
            );
            let node = child
                .as_widget_mut()
                .layout(state, renderer, &child_limits)
                .move_to(Point::new(x, 0.0));
            x += node.bounds().width;
            nodes.push(node);
        }
        layout::Node::with_children(size, nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        self.children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .for_each(|((child, state), layout)| {
                child
                    .as_widget_mut()
                    .operate(state, layout, renderer, operation);
            });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        // A close control is a child, and it owns its own press; forwarding first
        // is what lets it capture the event and keeps the press from starting a
        // drag on the tab behind it.
        for ((child, state), child_layout) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child.as_widget_mut().update(
                state,
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        let drag = tree.state.downcast_mut::<StripDrag>();
        let bounds = layout.bounds();

        // Move the hover fill with the pointer even while a button is down.
        if let Event::Mouse(mouse::Event::CursorMoved { .. }) = event {
            let over = cursor
                .position_over(bounds)
                .and_then(|point| self.index_at(layout, point));
            if drag.hovered != over {
                drag.hovered = over;
                shell.request_redraw();
            }
        }

        if shell.is_event_captured() {
            drag.pressed = None;
            drag.dragging = false;
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(point) = cursor.position_over(bounds)
                    && let Some(index) = self.index_at(layout, point)
                {
                    drag.pressed = Some(index);
                    drag.press_x = point.x;
                    drag.dragging = false;
                    drag.drop = index;
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let Some(index) = drag.pressed
                    && let Some(point) = cursor.position()
                {
                    if !drag.dragging && (point.x - drag.press_x).abs() > DRAG_THRESHOLD {
                        drag.dragging = true;
                    }
                    if drag.dragging {
                        drag.drop = self.drop_at(layout, point);
                        shell.request_redraw();
                        shell.capture_event();
                    }
                    let _ = index;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(index) = drag.pressed.take() {
                    if drag.dragging {
                        if let Some(on_move) = &self.on_move {
                            let before = self.ids.get(drag.drop).copied();
                            shell.publish(on_move(self.ids[index], self.region, before));
                        }
                        drag.dragging = false;
                        shell.request_redraw();
                    } else if let Some(on_select) = &self.on_select {
                        shell.publish(on_select(self.ids[index]));
                    }
                    shell.capture_event();
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                drag.pressed = None;
                drag.dragging = false;
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let tokens = Tokens::of(theme);
        let drag = tree.state.downcast_ref::<StripDrag>();
        let bounds = layout.bounds();

        // The strip's own surface: the muted band and its one-pixel edge, then
        // the tabs over it.
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: tokens.border,
                    width: STRIP_BORDER,
                    radius: 0.0.into(),
                },
                ..renderer::Quad::default()
            },
            tokens.muted,
        );

        renderer.with_layer(bounds, |renderer| {
            for (index, ((child, state), child_layout)) in self
                .children
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
                .enumerate()
            {
                let tab = child_layout.bounds();
                let active = self.ids.get(index) == self.active.as_ref();
                let hovered =
                    drag.hovered == Some(index) && drag.pressed.is_none() && !active;
                let fill = if active {
                    Some(tokens.background)
                } else if hovered {
                    // Slint hovers with `muted.with-alpha(0.6)` over a muted
                    // strip, which is the same colour twice; this steps the
                    // strip halfway to the page instead, so the hover reads in
                    // both appearances.
                    Some(theme::mix(tokens.muted, tokens.background, 0.6))
                } else {
                    None
                };
                if let Some(fill) = fill {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: tab,
                            ..renderer::Quad::default()
                        },
                        fill,
                    );
                }
                if active {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle::new(
                                Point::new(
                                    tab.x + UNDERLINE_INSET,
                                    tab.y + tab.height - UNDERLINE_HEIGHT,
                                ),
                                Size::new(
                                    (tab.width - UNDERLINE_INSET * 2.0).max(0.0),
                                    UNDERLINE_HEIGHT,
                                ),
                            ),
                            ..renderer::Quad::default()
                        },
                        tokens.primary,
                    );
                }

                let _ = (child, state);
                child.as_widget().draw(
                    state,
                    renderer,
                    theme,
                    style,
                    child_layout,
                    cursor,
                    viewport,
                );
            }

            if drag.dragging {
                let x = self.insertion_x(layout, drag.drop);
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle::new(
                            Point::new(x - 1.5, bounds.y),
                            Size::new(3.0, bounds.height),
                        ),
                        ..renderer::Quad::default()
                    },
                    tokens.primary,
                );
            }
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if tree.state.downcast_ref::<StripDrag>().dragging {
            mouse::Interaction::Grabbing
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message> From<TabStrip<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(strip: TabStrip<'a, Message>) -> Self {
        Element::new(strip)
    }
}

/// A tab's label and its close control, in the padding Slint's `DockTab` sets.
fn tab<'a, Message: Clone + 'a>(
    id: PanelId,
    title: &str,
    active: bool,
    closable: bool,
    on_close: Option<&Rc<dyn Fn(PanelId) -> Message + 'a>>,
) -> Element<'a, Message> {
    let label = text(title.to_string())
        .size(theme::FONT_SIZE_XS)
        .font(Font {
            weight: if active {
                font::Weight::Semibold
            } else {
                font::Weight::Medium
            },
            ..Font::DEFAULT
        })
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            text::Style {
                color: Some(if active {
                    tokens.foreground
                } else {
                    tokens.muted_foreground
                }),
            }
        });

    let mut content = row![label]
        .spacing(6.0)
        .align_y(Alignment::Center)
        .height(Length::Fill)
        .padding(Padding {
            top: 0.0,
            right: if closable {
                TAB_CLOSE_PADDING
            } else {
                TAB_PADDING
            },
            bottom: 0.0,
            left: TAB_PADDING,
        });

    if closable && active && let Some(on_close) = on_close {
        // The mark is the embedded `x`, not the `×` character: a text child is
        // laid out at its box's leading edge, so the glyph sat a pixel from the
        // left of a 16x16 box when measured. The container centres it on both
        // axes; the ink is resolved inside the closure so an SVG does not keep
        // the light theme's colour in the dark.
        let mark = svg(svg::Handle::from_memory(CLOSE_XML))
            .width(CLOSE_SIZE)
            .height(CLOSE_SIZE)
            .style(|theme: &Theme, _status| svg::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            });

        let close = button(
            container(mark)
                .center_x(Length::Fill)
                .center_y(Length::Fill),
        )
        .padding(0.0)
        .width(CLOSE_SIZE)
        .height(CLOSE_SIZE)
        .style(|theme: &Theme, status| {
            let tokens = Tokens::of(theme);
            let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
            button::Style {
                background: hovered.then(|| tokens.accent.into()),
                text_color: tokens.muted_foreground,
                border: Border::default(),
                ..button::Style::default()
            }
        })
        .on_press(on_close(id));
        content = content.push(close);
    }

    content.into()
}

/// The muted "no panel here" line a body draws, Slint's `Drop a panel here`.
fn empty_body<'a, Message: 'a>() -> Element<'a, Message> {
    text("Drop a panel here")
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(theme::alpha(Tokens::of(theme).muted_foreground, 0.65)),
        })
        .into()
}

/// A dock pane: its header — the collapse control and the tab strip — over the
/// active panel's body.
#[allow(clippy::too_many_arguments)]
fn pane<'a, Message: Clone + 'a>(
    region: Region,
    panels: Vec<Panel<'a, Message>>,
    active: Option<PanelId>,
    collapsed: bool,
    width: Length,
    height: Length,
    on_select: Option<&Rc<dyn Fn(PanelId) -> Message + 'a>>,
    on_close: Option<&Rc<dyn Fn(PanelId) -> Message + 'a>>,
    on_move: Option<&Rc<dyn Fn(PanelId, Region, Option<PanelId>) -> Message + 'a>>,
    on_toggle: Option<&Rc<dyn Fn(Region) -> Message + 'a>>,
) -> Element<'a, Message> {
    let mut ids = Vec::with_capacity(panels.len());
    let mut children = Vec::with_capacity(panels.len());
    let mut body: Option<Element<'a, Message>> = None;

    // A region with no active panel shows its first, which is what Slint's
    // `active-slot` fallback does.
    let active = active.or_else(|| panels.first().map(|panel| panel.id));

    for panel in panels {
        let is_active = active == Some(panel.id);
        if is_active {
            body = Some(panel.content);
        }
        ids.push(panel.id);
        children.push(tab(
            panel.id,
            &panel.title,
            is_active,
            panel.closable,
            on_close,
        ));
    }

    let strip: Element<'a, Message> = TabStrip {
        region,
        ids,
        children,
        active,
        on_select: on_select.cloned(),
        on_move: on_move.cloned(),
    }
    .into();

    let mut header = row![].spacing(0.0).width(Length::Fill).height(Length::Fixed(HEADER_HEIGHT));
    if let Some(on_toggle) = on_toggle
        && region.is_edge()
        && !collapsed
    {
        header = header.push(collapse_control(region, on_toggle.clone()));
    }
    header = header.push(strip);

    let body: Element<'a, Message> = match body {
        Some(content) => container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(theme::SPACING_LG)
            .style(|theme: &Theme| ContainerStyle {
                background: Some(Tokens::of(theme).background.into()),
                ..ContainerStyle::default()
            })
            .into(),
        None => container(empty_body())
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::Start)
            .padding(iced::Padding {
                top: theme::SPACING_MD,
                right: theme::SPACING_LG,
                bottom: theme::SPACING_LG,
                left: theme::SPACING_LG,
            })
            .style(|theme: &Theme| ContainerStyle {
                background: Some(Tokens::of(theme).background.into()),
                ..ContainerStyle::default()
            })
            .into(),
    };

    container(
        column![header, body]
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(0.0),
    )
    .width(width)
    .height(height)
    .style(|theme: &Theme| ContainerStyle {
        background: Some(Tokens::of(theme).background.into()),
        ..ContainerStyle::default()
    })
    .into()
}

/// The collapse control a pane's header carries: Slint's 26px `Panel` icon
/// button, as a flat arrow in the muted ink.
fn collapse_control<'a, Message: Clone + 'a>(
    region: Region,
    on_toggle: Rc<dyn Fn(Region) -> Message + 'a>,
) -> Element<'a, Message> {
    let arrow = match region {
        Region::Left => "←",
        Region::Right => "→",
        Region::Bottom => "↓",
        Region::Center => "×",
    };

    button(
        text(arrow)
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    )
    .width(TOGGLE_WIDTH)
    .height(Length::Fixed(HEADER_HEIGHT))
    .padding(0.0)
    .style(|theme: &Theme, status| {
        let tokens = Tokens::of(theme);
        let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
        button::Style {
            background: hovered.then(|| tokens.accent.into()),
            text_color: tokens.muted_foreground,
            border: Border::default(),
            ..button::Style::default()
        }
    })
    .on_press(on_toggle(region))
    .into()
}

/// A collapsed left or right dock: the 34px rail of Slint's `DockRail`, one
/// button per panel. A press selects that panel; the application expands the dock
/// from [`Dock::on_select`] if it wants to.
fn rail<'a, Message: Clone + 'a>(
    region: Region,
    panels: Vec<Panel<'a, Message>>,
    active: Option<PanelId>,
    on_select: Option<&Rc<dyn Fn(PanelId) -> Message + 'a>>,
) -> Element<'a, Message> {
    let arrow = if region == Region::Right { "→" } else { "←" };
    let mut items = column![].spacing(0.0).width(Length::Fill);

    for panel in panels {
        let id = panel.id;
        let selected = active == Some(id);
        let content = container(
            text(arrow.to_string())
                .size(theme::FONT_SIZE_SM)
                .style(move |theme: &Theme| {
                    let tokens = Tokens::of(theme);
                    text::Style {
                        color: Some(if selected {
                            tokens.foreground
                        } else {
                            tokens.muted_foreground
                        }),
                    }
                }),
        )
        .width(Length::Fill)
        .height(Length::Fixed(RAIL_WIDTH))
        .center_x(Length::Fill)
        .center_y(Length::Fixed(RAIL_WIDTH));

        let item: Element<'a, Message> = match on_select {
            Some(on_select) => button(content)
                .width(Length::Fill)
                .height(Length::Fixed(RAIL_WIDTH))
                .padding(0.0)
                .style(move |theme: &Theme, status| {
                    let tokens = Tokens::of(theme);
                    let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
                    button::Style {
                        background: if selected {
                            Some(tokens.accent.into())
                        } else if hovered {
                            Some(tokens.muted.into())
                        } else {
                            None
                        },
                        text_color: tokens.muted_foreground,
                        border: Border::default(),
                        ..button::Style::default()
                    }
                })
                .on_press(on_select(id))
                .into(),
            None => content.into(),
        };
        items = items.push(item);
    }

    container(items)
        .width(Length::Fixed(RAIL_WIDTH))
        .height(Length::Fill)
        .style(|theme: &Theme| ContainerStyle {
            background: Some(Tokens::of(theme).sidebar.into()),
            border: Border {
                color: Tokens::of(theme).border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..ContainerStyle::default()
        })
        .into()
}

impl<'a, Message: Clone + 'a> From<Dock<'a, Message>> for Element<'a, Message> {
    fn from(dock: Dock<'a, Message>) -> Self {
        let Dock {
            panels,
            active,
            collapsed,
            sizes,
            on_select,
            on_close,
            on_move,
            on_resize,
            on_toggle,
        } = dock;

        let mut by_region: [Vec<Panel<'a, Message>>; 4] = Default::default();
        for panel in panels {
            by_region[panel.region.index()].push(panel);
        }

        let mut main = row![]
            .spacing(0.0)
            .width(Length::Fill)
            .height(Length::Fill);

        // Left dock, its divider, then the centre column.
        if !by_region[Region::Left.index()].is_empty() {
            if collapsed[Region::Left.index()] {
                main = main.push(rail(
                    Region::Left,
                    std::mem::take(&mut by_region[Region::Left.index()]),
                    active[Region::Left.index()],
                    on_select.as_ref(),
                ));
            } else {
                main = main.push(pane(
                    Region::Left,
                    std::mem::take(&mut by_region[Region::Left.index()]),
                    active[Region::Left.index()],
                    false,
                    Length::Fixed(sizes[Region::Left.index()]),
                    Length::Fill,
                    on_select.as_ref(),
                    on_close.as_ref(),
                    on_move.as_ref(),
                    on_toggle.as_ref(),
                ));
                main = main.push(divider(
                    Region::Left,
                    false,
                    sizes[Region::Left.index()],
                    on_resize.as_ref(),
                ));
            }
        }

        // The centre column: the centre pane over the bottom dock.
        let mut centre = column![pane(
            Region::Center,
            std::mem::take(&mut by_region[Region::Center.index()]),
            active[Region::Center.index()],
            false,
            Length::Fill,
            Length::Fill,
            on_select.as_ref(),
            on_close.as_ref(),
            on_move.as_ref(),
            on_toggle.as_ref(),
        )]
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(0.0);

        if !by_region[Region::Bottom.index()].is_empty() {
            if collapsed[Region::Bottom.index()] {
                centre = centre.push(expand_bar(
                    Region::Bottom,
                    by_region[Region::Bottom.index()]
                        .first()
                        .map(|panel| panel.title.clone())
                        .unwrap_or_default(),
                    on_toggle.as_ref(),
                ));
            } else {
                centre = centre.push(divider(
                    Region::Bottom,
                    true,
                    sizes[Region::Bottom.index()],
                    on_resize.as_ref(),
                ));
                centre = centre.push(pane(
                    Region::Bottom,
                    std::mem::take(&mut by_region[Region::Bottom.index()]),
                    active[Region::Bottom.index()],
                    false,
                    Length::Fill,
                    Length::Fixed(sizes[Region::Bottom.index()]),
                    on_select.as_ref(),
                    on_close.as_ref(),
                    on_move.as_ref(),
                    on_toggle.as_ref(),
                ));
            }
        }

        main = main.push(centre);

        // Right dock, with its divider before it.
        if !by_region[Region::Right.index()].is_empty() {
            if collapsed[Region::Right.index()] {
                main = main.push(rail(
                    Region::Right,
                    std::mem::take(&mut by_region[Region::Right.index()]),
                    active[Region::Right.index()],
                    on_select.as_ref(),
                ));
            } else {
                main = main.push(divider(
                    Region::Right,
                    true,
                    sizes[Region::Right.index()],
                    on_resize.as_ref(),
                ));
                main = main.push(pane(
                    Region::Right,
                    std::mem::take(&mut by_region[Region::Right.index()]),
                    active[Region::Right.index()],
                    false,
                    Length::Fixed(sizes[Region::Right.index()]),
                    Length::Fill,
                    on_select.as_ref(),
                    on_close.as_ref(),
                    on_move.as_ref(),
                    on_toggle.as_ref(),
                ));
            }
        }

        container(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .clip(true)
            .style(|theme: &Theme| ContainerStyle {
                background: Some(Tokens::of(theme).background.into()),
                border: Border {
                    color: Tokens::of(theme).border,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..ContainerStyle::default()
            })
            .into()
    }
}

/// A divider element for a region, wired to the dock's `on_resize`.
fn divider<'a, Message: 'a>(
    region: Region,
    inverted: bool,
    split: f32,
    on_resize: Option<&Rc<dyn Fn(Region, f32) -> Message + 'a>>,
) -> Element<'a, Message> {
    let mut divider = Divider::new(region, inverted, split);
    if let Some(on_resize) = on_resize {
        divider = divider.on_resize({
            let on_resize = on_resize.clone();
            move |region, size| on_resize(region, size)
        });
    }
    divider.into()
}

/// A collapsed bottom dock: the 30px bar Slint shows in its place, with the
/// active panel's title and the way back.
fn expand_bar<'a, Message: Clone + 'a>(
    region: Region,
    title: String,
    on_toggle: Option<&Rc<dyn Fn(Region) -> Message + 'a>>,
) -> Element<'a, Message> {
    let label = row![
        text("↑")
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
        text(title)
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    ]
    .spacing(8.0)
    .padding(iced::Padding {
        top: 0.0,
        right: 10.0,
        bottom: 0.0,
        left: 10.0,
    })
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .height(Length::Fixed(HEADER_HEIGHT));

    let bar: Element<'a, Message> = match on_toggle {
        Some(on_toggle) => button(label)
            .width(Length::Fill)
            .height(Length::Fixed(HEADER_HEIGHT))
            .padding(0.0)
            .style(|theme: &Theme, status| {
                let tokens = Tokens::of(theme);
                let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
                button::Style {
                    background: hovered.then(|| tokens.accent.into()),
                    text_color: tokens.muted_foreground,
                    border: Border::default(),
                    ..button::Style::default()
                }
            })
            .on_press(on_toggle(region))
            .into(),
        None => label.into(),
    };

    container(bar)
        .width(Length::Fill)
        .height(Length::Fixed(HEADER_HEIGHT))
        .style(|theme: &Theme| ContainerStyle {
            background: Some(Tokens::of(theme).sidebar.into()),
            border: Border {
                color: Tokens::of(theme).border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..ContainerStyle::default()
        })
        .into()
}
