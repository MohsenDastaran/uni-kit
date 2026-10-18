//! `tree` — a disclosure tree, mirroring the Slint `Tree` and GPUI Component's
//! `Tree`.
//!
//! The Slint version keeps a *flattened* model: every row carries its own
//! `depth`, `folder`, `expanded` and `hidden`, because Slint has no recursion and
//! cannot walk a parent chain. iced has recursion, so this version takes the
//! tree itself — [`Node`]s that own their children — and *the caller owns which
//! nodes are open*, as the brief asks: [`Tree::is_open`] answers whether a
//! node's children are shown and [`Tree::on_toggle`] reports the request to
//! change it. An application stores a set of open ids (or a single "current"
//! node) and rebuilds the tree each frame; nothing is hidden inside the
//! component.
//!
//! **A folder discloses through [`collapsible::reveal`].** The component builds
//! the tree recursively, and every folder's children are wrapped in the reveal
//! the collapsible panel gained: the column is measured at its full height and
//! clipped to the fraction of it `open` has reached, so a folder opens and closes
//! over [`collapsible::REVEAL_DURATION`] (150ms, linear, the Slint duration)
//! rather than jumping. The reveal is **mounted even while the folder is
//! closed**, which is what lets it measure and reverse mid-flight; a collapsed
//! subtree is therefore still built and laid out each frame, clipped to nothing.
//! [`Tree::animated`] turns the motion off and returns to mounting the children
//! only while they are open.
//!
//! Reproduced from the Slint file:
//!
//! - the row metrics: `padding-left: 8 + depth * 16`, `padding-right: 8`, `4`
//!   above and below, `6` between the pieces;
//! - the disclosure arrow owns a hit area of its own *over* the row's, so
//!   clicking the arrow opens the folder instead of selecting the row. iced
//!   buttons capture the press and a button nested in a button gets it first,
//!   which is the same outcome;
//! - the selected row takes `accent`, a hovered one `accent` at 40%, rest is
//!   transparent;
//! - folder and leaf glyphs sit at 14 px in the foreground, the chevron at 12 px
//!   in the muted foreground.
//!
//! Two differences are forced. Slint's glyphs are names into a shipped SVG
//! folder, and iced's icons are the caller's (README): [`Node::icon`] takes the
//! caller's element and [`Tree::disclosure`] takes a closure that builds the
//! arrow for a given open state. The default arrow is the Lucide
//! `chevron-right`/`chevron-down` the Slint file names, embedded here because a
//! text triangle is a glyph the wasm face does not have and draws as nothing.
//! Slint paints hover on the whole row; iced's row is a `button`, so it reports
//! hover, press and disabled the same way every other control in this crate
//! does, and the arrow reports no hover of its own — the row under it already
//! does.
//!
//! Slint swaps the two chevrons rather than rotating one, and this follows it:
//! the chevron is drawn in its end state and the panel carries the motion. A
//! caller that wants the collapsible's turning chevron passes its own element to
//! [`Tree::disclosure`] and turns it over [`collapsible::REVEAL_DURATION`].
//!
//! One gap is named rather than faked: **there is no keyboard navigation.**
//! GPUI's `TreeState` binds the arrow keys to selection and disclosure; iced
//! 0.14 gives this crate no tab-order or focus-trap API to hang a tree key
//! context on, so the pointer is the only path through the rows here. The rows
//! and the arrows are real buttons and the component holds no hidden state, so a
//! caller that grows a key context can drive the same `on_select` and
//! `on_toggle` messages.

#![allow(dead_code)]

use iced::widget::{button, column, container, row, space, svg, text};
use iced::{Background, Border, Color, Element, Length, Padding, Shadow, Theme};

use super::collapsible;
use super::theme::{self, Tokens};

/// The chevron lane: 16px wide and 20px tall, the box the Slint `TouchArea`
/// overlays, with the 12px glyph centred in it so a leaf's icon never moves when
/// a sibling's arrow appears.
const DISCLOSURE_LANE: f32 = 16.0;
const DISCLOSURE_LANE_HEIGHT: f32 = 20.0;
const DISCLOSURE_GLYPH: f32 = 12.0;

/// One row of the tree, with the children it discloses.
pub struct Node<'a, Id, Message> {
    id: Id,
    label: String,
    icon: Option<Element<'a, Message>>,
    children: Vec<Node<'a, Id, Message>>,
}

impl<'a, Id, Message> Node<'a, Id, Message> {
    /// A leaf.
    pub fn new(id: Id, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            icon: None,
            children: Vec::new(),
        }
    }

    /// A leading element drawn before the label, at the 14 px the Slint icon
    /// box uses. The icon set belongs to the application.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Adds a child, which makes this node a folder that can be disclosed.
    pub fn push(mut self, child: Node<'a, Id, Message>) -> Self {
        self.children.push(child);
        self
    }

    /// Adds the children in one call.
    pub fn extend(mut self, children: impl IntoIterator<Item = Node<'a, Id, Message>>) -> Self {
        self.children.extend(children);
        self
    }

    fn is_folder(&self) -> bool {
        !self.children.is_empty()
    }
}

/// A disclosure tree, built with the methods below.
///
/// ```ignore
/// Tree::new(nodes)
///     .selected(app.selected.clone())
///     .is_open(|id| app.open.contains(id))
///     .on_toggle(Message::Toggle)
///     .on_select(Message::Select)
/// ```
pub struct Tree<'a, Id, Message> {
    nodes: Vec<Node<'a, Id, Message>>,
    selected: Option<Id>,
    is_open: Box<dyn Fn(&Id) -> bool + 'a>,
    on_select: Option<Box<dyn Fn(Id) -> Message + 'a>>,
    on_toggle: Option<Box<dyn Fn(Id) -> Message + 'a>>,
    disclosure: Option<Box<dyn Fn(bool) -> Element<'a, Message> + 'a>>,
    indent: f32,
    animated: bool,
}

impl<'a, Id, Message> Tree<'a, Id, Message> {
    /// A tree of `nodes`. Every folder starts closed until [`Self::is_open`]
    /// says otherwise.
    pub fn new(nodes: Vec<Node<'a, Id, Message>>) -> Self
    where
        Id: 'a,
        Message: 'a,
    {
        Self {
            nodes,
            selected: None,
            is_open: Box::new(|_| false),
            on_select: None,
            on_toggle: None,
            disclosure: None,
            indent: 16.0,
            animated: true,
        }
    }

    /// The row drawn as selected. The caller owns the selection.
    pub fn selected(mut self, selected: Option<Id>) -> Self {
        self.selected = selected;
        self
    }

    /// Whether a node's children are shown. The caller owns this state.
    pub fn is_open(mut self, is_open: impl Fn(&Id) -> bool + 'a) -> Self {
        self.is_open = Box::new(is_open);
        self
    }

    /// A row was clicked. Without it every row is drawn disabled.
    pub fn on_select(mut self, on_select: impl Fn(Id) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// The disclosure arrow was clicked, for the caller to flip its open state.
    pub fn on_toggle(mut self, on_toggle: impl Fn(Id) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// The arrow for a given open state, built by the caller because the icon
    /// set belongs to the application. The default is the Lucide
    /// `chevron-right`/`chevron-down` pair the Slint file names.
    pub fn disclosure(mut self, disclosure: impl Fn(bool) -> Element<'a, Message> + 'a) -> Self {
        self.disclosure = Some(Box::new(disclosure));
        self
    }

    /// Pixels added per level of depth. The Slint version uses 16.
    pub fn indent(mut self, indent: f32) -> Self {
        self.indent = indent;
        self
    }

    /// Whether a folder's children reveal over [`collapsible::REVEAL_DURATION`].
    /// On by default, which is the animation the tree gained; `false` shows and
    /// hides them outright and mounts a closed folder's children not at all.
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

/// The parts of the builder the recursion reads while it consumes the nodes.
struct Walk<'a, Id, Message> {
    selected: Option<Id>,
    is_open: Box<dyn Fn(&Id) -> bool + 'a>,
    on_select: Option<Box<dyn Fn(Id) -> Message + 'a>>,
    on_toggle: Option<Box<dyn Fn(Id) -> Message + 'a>>,
    disclosure: Option<Box<dyn Fn(bool) -> Element<'a, Message> + 'a>>,
    indent: f32,
    animated: bool,
}

/// The fill a row draws, given the interaction status and the selection.
fn row_background(tokens: Tokens, status: button::Status, selected: bool) -> Color {
    match status {
        button::Status::Disabled => Color::TRANSPARENT,
        button::Status::Pressed => tokens.accent,
        button::Status::Hovered => {
            if selected {
                tokens.accent
            } else {
                theme::alpha(tokens.accent, 0.4)
            }
        }
        button::Status::Active => {
            if selected {
                tokens.accent
            } else {
                Color::TRANSPARENT
            }
        }
    }
}

fn row_style(tokens: Tokens, status: button::Status, selected: bool) -> button::Style {
    let fill = row_background(tokens, status, selected);
    button::Style {
        background: if fill.a <= 0.0 {
            None
        } else {
            Some(Background::Color(fill))
        },
        text_color: tokens.foreground,
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    }
}

/// The arrow's own hit area draws no fill: the row underneath already reports
/// the hover, and a second, lighter accent inside the first reads as two
/// highlights. Slint's overlay is transparent for the same reason.
fn arrow_style(tokens: Tokens, _status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: tokens.muted_foreground,
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    }
}

/// The default disclosure: Lucide's `chevron-right` while closed and
/// `chevron-down` while open, in the muted foreground. Swapped, not turned —
/// that is what the Slint tree does.
fn default_disclosure<'a, Message: 'a>(open: bool) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(if open {
        CHEVRON_DOWN
    } else {
        CHEVRON_RIGHT
    }))
    .width(Length::Fixed(DISCLOSURE_GLYPH))
    .height(Length::Fixed(DISCLOSURE_GLYPH))
    .style(|theme: &Theme, _status| svg::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    })
    .into()
}

/// Builds one node: its row, and — for a folder — its children under a reveal.
///
/// The recursion is the whole model. A folder returns a column of the row and
/// the revealed children, so a nested level inherits its parent's indentation
/// through its own `depth` rather than through accumulated padding; the design
/// guide's "when a nested level ends, return exactly to the parent spine" falls
/// out of that. The reveal is mounted even while the folder is closed, so a
/// collapsed subtree is still measured and a toggle can reverse mid-flight.
fn build<'a, Id, Message>(
    node: Node<'a, Id, Message>,
    depth: f32,
    tree: &Walk<'a, Id, Message>,
) -> Element<'a, Message>
where
    Id: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    let Node {
        id,
        label,
        icon,
        children,
    } = node;
    let is_folder = !children.is_empty();
    let open = (tree.is_open)(&id);
    let indent = 8.0 + depth * tree.indent;
    let selected = tree.selected.as_ref().is_some_and(|sel| *sel == id);

    let mut contents = row![]
        .spacing(6.0)
        .align_y(iced::Alignment::Center)
        .padding(Padding {
            top: 4.0,
            right: 8.0,
            bottom: 4.0,
            left: 0.0,
        });

    // The indent is a spacer rather than padding so the arrow's hit area starts
    // where the Slint `TouchArea` starts — 2px inside the row's left padding,
    // with the 12px glyph centred on the indent itself.
    contents = contents.push(space().width(Length::Fixed((indent - 2.0).max(0.0))));

    if is_folder {
        let arrow: Element<'a, Message> = match &tree.disclosure {
            Some(disclosure) => disclosure(open),
            None => default_disclosure(open),
        };
        let mut target = button(
            container(arrow)
                .width(Length::Fixed(DISCLOSURE_LANE))
                .height(Length::Fixed(DISCLOSURE_LANE_HEIGHT))
                .center_x(Length::Fixed(DISCLOSURE_LANE))
                .center_y(Length::Fixed(DISCLOSURE_LANE_HEIGHT)),
        )
        .padding(0)
        .width(Length::Fixed(DISCLOSURE_LANE))
        .style(|theme: &Theme, status| arrow_style(Tokens::of(theme), status));

        if let Some(on_toggle) = &tree.on_toggle {
            target = target.on_press(on_toggle(id.clone()));
        }
        contents = contents.push(target);
    }

    if let Some(icon) = icon {
        contents = contents.push(icon);
    }

    contents = contents.push(
        text(label)
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
    );

    let mut control = button(contents)
        .padding(0)
        .width(Length::Fill)
        .style(move |theme: &Theme, status| row_style(Tokens::of(theme), status, selected));
    if let Some(on_select) = &tree.on_select {
        control = control.on_press(on_select(id.clone()));
    }

    if !is_folder {
        return control.into();
    }

    let rows: Vec<Element<'a, Message>> = children
        .into_iter()
        .map(|child| build(child, depth + 1.0, tree))
        .collect();
    // A `Fill` child of a `Shrink` column lays out at zero, and a reveal is
    // `Fill` wide; every column on the way down therefore says `Fill` so the
    // rows resolve against the tree's own width rather than the shrink-to-fit
    // pass.
    let body = column(rows).width(Length::Fill);

    let mut section = column![control].width(Length::Fill);
    if tree.animated {
        section = section.push(collapsible::reveal(body, open));
    } else if open {
        section = section.push(body);
    }
    section.into()
}

impl<'a, Id, Message> From<Tree<'a, Id, Message>> for Element<'a, Message>
where
    Id: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    fn from(tree: Tree<'a, Id, Message>) -> Self {
        let Tree {
            nodes,
            selected,
            is_open,
            on_select,
            on_toggle,
            disclosure,
            indent,
            animated,
        } = tree;
        let walker = Walk {
            selected,
            is_open,
            on_select,
            on_toggle,
            disclosure,
            indent,
            animated,
        };
        let rows: Vec<Element<'a, Message>> = nodes
            .into_iter()
            .map(|node| build(node, 0.0, &walker))
            .collect();
        column(rows).width(Length::Fill).into()
    }
}

/// `chevron-right`, from the Lucide set the Slint icons folder carries (ISC).
const CHEVRON_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#;

/// `chevron-down`, from the same set.
const CHEVRON_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#;
