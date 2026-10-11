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
//! - folder and leaf glyphs sit at 14 px, the arrow at 12 px.
//!
//! Two differences are forced. Slint's glyphs are names into a shipped SVG
//! folder, and iced's icons are the caller's (README): [`Node::icon`] takes the
//! caller's element and [`Tree::disclosure`] takes a closure that builds the
//! arrow for a given open state. When neither is given the tree still draws: the
//! arrow falls back to the text chevron iced can draw without an asset, and a
//! node without an icon is simply a label. Slint paints hover on the whole row;
//! iced's row is a `button`, so it reports hover, press and disabled the same
//! way every other control in this crate does.

#![allow(dead_code)]

use iced::widget::{button, column, container, row, space, text};
use iced::{Background, Border, Color, Element, Length, Padding, Shadow, Theme};

use super::theme::{self, Tokens};

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
    pub fn extend(
        mut self,
        children: impl IntoIterator<Item = Node<'a, Id, Message>>,
    ) -> Self {
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
    /// set belongs to the application. The default is the text chevron iced can
    /// draw without an asset.
    pub fn disclosure(
        mut self,
        disclosure: impl Fn(bool) -> Element<'a, Message> + 'a,
    ) -> Self {
        self.disclosure = Some(Box::new(disclosure));
        self
    }

    /// Pixels added per level of depth. The Slint version uses 16.
    pub fn indent(mut self, indent: f32) -> Self {
        self.indent = indent;
        self
    }
}

/// The parts of the builder the walk reads while it consumes the nodes.
struct Walk<'a, Id, Message> {
    selected: Option<Id>,
    is_open: Box<dyn Fn(&Id) -> bool + 'a>,
    on_select: Option<Box<dyn Fn(Id) -> Message + 'a>>,
    on_toggle: Option<Box<dyn Fn(Id) -> Message + 'a>>,
    disclosure: Option<Box<dyn Fn(bool) -> Element<'a, Message> + 'a>>,
    indent: f32,
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

fn arrow_style(tokens: Tokens, status: button::Status) -> button::Style {
    let fill = match status {
        button::Status::Hovered => theme::alpha(tokens.accent, 0.6),
        button::Status::Pressed => tokens.accent,
        _ => Color::TRANSPARENT,
    };
    button::Style {
        background: if fill.a <= 0.0 {
            None
        } else {
            Some(Background::Color(fill))
        },
        text_color: tokens.muted_foreground,
        border: Border::default(),
        shadow: Shadow::default(),
        snap: false,
    }
}

/// Walks the model, appending one element per visible row. A node's children
/// are only visited while it is open, which is why the flattening happens here
/// and not in a model the caller maintains.
fn walk<'a, Id, Message>(
    nodes: Vec<Node<'a, Id, Message>>,
    depth: f32,
    tree: &Walk<'a, Id, Message>,
    out: &mut Vec<Element<'a, Message>>,
) where
    Id: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    for node in nodes {
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

        // The indent is a spacer rather than padding so the arrow's hit area
        // starts where the Slint `TouchArea` starts.
        contents = contents.push(space().width(Length::Fixed(indent)));

        if is_folder {
            let arrow: Element<'a, Message> = match &tree.disclosure {
                Some(disclosure) => disclosure(open),
                None => text(if open { "\u{25be}" } else { "\u{25b8}" })
                    .size(12.0)
                    .into(),
            };
            let mut target = button(
                container(arrow)
                    .width(Length::Fixed(16.0))
                    .height(Length::Fixed(20.0))
                    .center_x(Length::Fixed(16.0))
                    .center_y(Length::Fixed(20.0)),
            )
            .padding(0)
            .width(Length::Fixed(16.0))
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
            .style(move |theme: &Theme, status| {
                row_style(Tokens::of(theme), status, selected)
            });
        if let Some(on_select) = &tree.on_select {
            control = control.on_press(on_select(id.clone()));
        }
        out.push(control.into());

        if is_folder && open {
            walk(children, depth + 1.0, tree, out);
        }
    }
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
        } = tree;
        let walker = Walk {
            selected,
            is_open,
            on_select,
            on_toggle,
            disclosure,
            indent,
        };
        let mut rows = Vec::new();
        walk(nodes, 0.0, &walker, &mut rows);
        column(rows).width(Length::Fill).into()
    }
}
