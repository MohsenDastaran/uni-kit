//! A floating surface anchored to a trigger. Mirrors GPUI Component's `Popover`
//! and the Slint version beside it, and is the base the rest of the overlay
//! family — `tooltip`, `menu`, `dropdown_button`, `select`, `combobox` and
//! `date-picker` — is built on.
//!
//! # Why this is a widget and not a `stack!`
//!
//! A surface drawn inside the layout that owns the trigger is clipped by every
//! scroll area between them, and it cannot escape a card: the Slint example says
//! so in as many words ("The panel is a popup, so it is not clipped by this
//! card"). iced 0.14 has no popover widget, but it does have the mechanism
//! `pick_list`, `combo_box` and iced's own `tooltip` use: a `Widget` whose
//! [`overlay`](iced::advanced::widget::Widget::overlay) returns an
//! [`overlay::Element`] while it is open. The runtime lays that overlay out
//! against the window, draws it after the whole tree, and gives it every event
//! first, so the panel is neither clipped nor covered. That mechanism is what
//! this file uses, and nothing here fakes position with padding inside a stack.
//!
//! # Why the overlay borrows instead of taking
//!
//! The runtime builds a widget's overlay **twice**: once while it processes the
//! frame's events, and again in `UserInterface::draw`, because the overlay the
//! events ran against is dropped before the frame is painted. A popover that
//! moved its content out of `self` the first time therefore found nothing the
//! second time and drew no panel at all. So the content stays in the popover and
//! the overlay borrows it, which is what iced's own `tooltip` does with its
//! tooltip and `pick_list` does with its menu.
//!
//! # The contract
//!
//! The caller owns `open`. [`Popover::open`] is what the panel is rendered from,
//! and [`Popover::on_open_change`] is the only way the popover reports a change —
//! a trigger click, an outside click, or Escape. A popover with no
//! `on_open_change` cannot move its own state, because it has none; it still
//! renders open whenever the caller says open.
//!
//! # What Slint has that this does not
//!
//! - **The arrow.** GPUI's `Popover` has `.arrow(true)` and draws a triangle
//!   between the surface and the trigger. The Slint component the catalog
//!   exposes has no arrow property and its example never shows one, so this port
//!   leaves it out rather than inventing a property the authority does not have.
//!   The gap [`Popover::offset`] sets is where a later arrow would go.
//! - **Slint's `open` can be left ahead of the panel.** The Slint file says a
//!   click-outside dismissal is not written back through `open`, because of a
//!   constant-propagation failure in its compiler. iced has no such problem:
//!   every dismissal is reported, so `open` and the panel cannot disagree.
//! - **A dismiss from inside.** GPUI's content closure gets a `PopoverState` and
//!   can emit `DismissEvent`; Slint's example closes by setting its own `open`
//!   property. Here the caller already holds the state the panel is rendered
//!   from, so a control inside the panel closes it by sending the same message
//!   an outside click would — the "Dismiss Popover manually" card does that.
//!
//! # Coordinates
//!
//! A popover lives inside whatever scrolls it, so the two rectangles it works
//! with are read from the layout the runtime hands it and nothing is
//! recomputed: `update` matches a press against `layout.bounds()`, which
//! `scrollable` has already translated into the content coordinates its
//! `Cursor` is in (it adds the scroll offset to the cursor before dispatching,
//! `scrollable.rs:800`), and `overlay` places the panel at
//! `layout.position() - translation`, the offset the runtime passes there for
//! the same reason. iced's own `pick_list` takes its trigger hit test from
//! `cursor.is_over(layout.bounds())` in exactly the same way.
//!
//! # Focus
//!
//! iced 0.14 has no focus trap and no focus handle a widget can take. The
//! overlay is the topmost layer while it is open, so it sees every key — this
//! component answers Escape — but it cannot stop Tab from reaching what is
//! behind it, and it cannot move focus into the panel. A caller that wants the
//! keyboard inside its content gives that content an `Id` and focuses it.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{overlay, renderer, Clipboard, Renderer as _, Shell};
use iced::widget::container;
use iced::{
    keyboard, mouse, Border, Color, Element, Event, Length, Padding, Point, Rectangle, Shadow, Size,
    Theme, Vector,
};

use super::theme::{self, Tokens};

/// Which side of the trigger the panel prefers.
///
/// The names are the Slint `Placement` enum's, and so is the default. A side
/// with no room for the panel flips to the opposite one — Slint's `PopupWindow`
/// and GPUI's positioner both do this — and if neither side fits, the panel
/// clamps into the window rather than leaving it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Placement {
    /// Under the trigger.
    #[default]
    Bottom,
    /// Above the trigger.
    Top,
    /// To the left of the trigger.
    Left,
    /// To the right of the trigger.
    Right,
}

/// A floating panel anchored to a trigger, built with the methods below.
///
/// The trigger is laid out where the caller puts the popover, and is the only
/// thing the popover measures. The panel is an overlay: it is laid out against
/// the window, drawn after everything else, and never clipped by a scroll area
/// or a card. See the module comment for the mechanism.
pub struct Popover<'a, Message> {
    trigger: Element<'a, Message>,
    panel: Option<Element<'a, Message>>,
    open: bool,
    filled: bool,
    placement: Placement,
    offset: f32,
    padding: f32,
    panel_width: f32,
    style: Box<dyn Fn(&Theme) -> PanelStyle + 'a>,
    /// The theme the trigger was last drawn with. An overlay is built without
    /// one — `layout`, `draw` and `update` are handed a theme and the overlay
    /// constructor is not — but the base tree is drawn *before* the overlay is
    /// built in `UserInterface::draw`, so [`Popover::draw`] has already recorded
    /// the theme in force by the time the panel asks for it. That is why this
    /// port is exact rather than a frame behind.
    theme: std::cell::RefCell<Option<Theme>>,
    on_open_change: Option<std::rc::Rc<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message> Popover<'a, Message> {
    /// A popover whose trigger is `trigger` — any element, usually a
    /// [`Button`](super::button::Button). It starts closed, because the caller's
    /// own state says what is open.
    pub fn new(trigger: impl Into<Element<'a, Message>>) -> Self {
        Self {
            trigger: trigger.into(),
            panel: None,
            open: false,
            filled: false,
            placement: Placement::default(),
            offset: 8.0,
            padding: theme::SPACING_MD,
            panel_width: 260.0,
            style: Box::new(|theme: &Theme| PanelStyle::resolve(Tokens::of(theme))),
            theme: std::cell::RefCell::new(None),
            on_open_change: None,
        }
    }

    /// The content of the panel. Any element: text, a column of rows, a form, a
    /// list.
    ///
    /// This is the method `select`, `combobox` and `date-picker` build their
    /// surfaces with, so it takes an element rather than a closure — the caller
    /// has already built the content it wants to show, and rebuilding it in a
    /// callback would be a second place for the same state to live.
    ///
    /// The content is laid out inside the panel's inset at the panel's width. A
    /// caller that wants the panel's surface and inset applied around it — so
    /// its text inherits the panel's colour, the way everything inside the Slint
    /// example's `@children` slot does — wraps the content with
    /// [`panel_container`], which is what the gallery does. A caller whose
    /// content draws its own surface, a list or a calendar, asks for
    /// [`filled`](Self::filled) instead.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.panel = Some(content.into());
        self
    }

    /// Whether the panel is showing. The caller owns this: it is set from the
    /// application's state, and [`on_open_change`](Self::on_open_change) is how
    /// the application learns it should change.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// The side of the trigger the panel prefers, [`Placement::Bottom`] by
    /// default. A side without room flips to the opposite one.
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// The gap between the trigger and the panel, 8px by default — the same
    /// distance the Slint component uses (`host.height + 8px`).
    pub fn offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    /// The panel's inset, `SPACING_MD` (12px) by default, which is the Slint
    /// panel's `padding: 12px`. It is the space the content is laid out inside,
    /// whether or not the surface is drawn here.
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// The panel's fixed width, 260px by default. A left or right placement
    /// offsets by it, so the panel cannot be a fixed distance from the trigger —
    /// which is what Slint's `panel-width` is for too.
    pub fn panel_width(mut self, width: f32) -> Self {
        self.panel_width = width;
        self
    }

    /// The surface, resolved from the tokens by default: the popover fill, a
    /// one-pixel border, `radius-lg` and the drop shadow. A caller that wants
    /// its own states it as a function of the theme, which is Slint's
    /// `panel-background`, `panel-radius` and `panel-border`:
    ///
    /// ```ignore
    /// Popover::new(trigger).panel(|theme| {
    ///     let tokens = Tokens::of(theme);
    ///     PanelStyle::resolve(tokens)
    ///         .fill(tokens.primary)
    ///         .border(tokens.primary, 0.0)
    /// })
    /// ```
    ///
    /// It is a function rather than a value because the appearance is not known
    /// where the panel is built — an overlay is constructed without a theme —
    /// and a surface resolved for one appearance is the wrong colour in the
    /// other. This crate's `input.rs` says the same thing about its frame.
    pub fn panel(mut self, style: impl Fn(&Theme) -> PanelStyle + 'a) -> Self {
        self.style = Box::new(style);
        self
    }

    /// Draw the surface here, around the content, on top of
    /// [`content`](Self::content).
    ///
    /// The content is drawn inside the surface and is not clipped by it, so a
    /// caller whose content fills the panel exactly — a list, a form, a
    /// calendar — asks for the surface here and leaves its content bare. The
    /// gallery instead wraps its content with [`panel_container`], which is the
    /// same surface with the inset and the inherited text colour.
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Report every request to change the open state: a trigger click wants
    /// `true`, an outside click or Escape wants `false`.
    ///
    /// This is the only way the popover speaks. It never opens or closes itself,
    /// so a caller that wants the panel to move must answer with its own state —
    /// which is also what makes an external control able to drive the same
    /// popover.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(on_open_change));
        self
    }
}

/// The colours and shape a panel draws itself with.
///
/// [`PanelStyle::resolve`] builds one from the theme tokens.
/// [`Popover::panel`] draws the surface itself, and [`panel_container`] draws
/// the same surface as a container around content, which is how a panel's text
/// inherits its colour. The fields are private and read through methods, so this
/// can grow without breaking a caller.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PanelStyle {
    fill: Color,
    border: Color,
    border_width: f32,
    radius: f32,
    shadow: Shadow,
    text_color: Color,
}

impl PanelStyle {
    /// The panel the tokens describe: the popover fill, the border token as a
    /// one-pixel edge, `radius-lg` and the shadow token 4px below.
    ///
    /// The edge is the `border` token and not a fixed grey: the token is
    /// `#e5e5e5` on white and `#262626` on near-black, so the edge exists in
    /// both appearances. The fill is the `popover` token — white in the light
    /// theme, `#171717` in the dark one, one step off the page's `#0a0a0a` — so
    /// the surface separates from the page in both.
    pub fn resolve(tokens: Tokens) -> Self {
        Self {
            fill: tokens.popover,
            border: tokens.border,
            border_width: 1.0,
            radius: theme::RADIUS_LG,
            // iced's blur is a Gaussian sigma where Slint's is a spread. A sigma
            // of 12 would fade over 36px and read as fog; 8 in the dark theme
            // and 4 in the light one carry the same weight as Slint's 12px at
            // this offset, and match the other iced surfaces in this crate.
            shadow: Shadow {
                color: tokens.shadow,
                offset: Vector::new(0.0, 4.0),
                blur_radius: if tokens.dark { 8.0 } else { 4.0 },
            },
            text_color: tokens.foreground,
        }
    }

    /// The panel fill.
    pub fn fill(mut self, color: Color) -> Self {
        self.fill = color;
        self
    }

    /// The panel edge. A `width` of 0 draws no edge, which is Slint's
    /// `panel-border: false`.
    pub fn border(mut self, color: Color, width: f32) -> Self {
        self.border = color;
        self.border_width = width;
        self
    }

    /// The corner radius — `radius-lg` (8px) by default.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// The drop shadow. `Shadow::default()` draws none.
    pub fn shadow(mut self, shadow: Shadow) -> Self {
        self.shadow = shadow;
        self
    }

    /// The colour text inside the panel inherits. The panel's fill would be
    /// unreadable as ink — it is the surface the text sits on, not the text —
    /// so this is the theme's foreground unless a caller says otherwise.
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn content_color(&self) -> Color {
        self.text_color
    }

    /// Draw the panel's surface into `bounds`: the fill, the edge, the radius
    /// and the shadow. [`Popover::filled`] calls this on the overlay's root, so
    /// the shadow is not clipped by an inner element.
    pub fn draw(&self, renderer: &mut iced::Renderer, bounds: Rectangle) {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: self.border,
                    width: self.border_width,
                    radius: self.radius.into(),
                },
                shadow: self.shadow,
                ..renderer::Quad::default()
            },
            iced::Background::Color(self.fill),
        );
    }

    /// The fill, the edge, the radius and the shadow as container styling, which
    /// is what [`panel_container`] applies.
    pub fn container_style(&self) -> container::Style {
        container::Style {
            background: Some(self.fill.into()),
            border: Border {
                color: self.border,
                width: self.border_width,
                radius: self.radius.into(),
            },
            shadow: self.shadow,
            ..container::Style::default()
        }
    }

    pub fn background(&self) -> Color {
        self.fill
    }

    pub fn edge(&self) -> Color {
        self.border
    }

    pub fn edge_width(&self) -> f32 {
        self.border_width
    }

    pub fn corner_radius(&self) -> f32 {
        self.radius
    }

    pub fn drop_shadow(&self) -> Shadow {
        self.shadow
    }
}

impl Default for PanelStyle {
    /// The light theme's resting panel. A caller that wants the appearance in
    /// force builds it with [`PanelStyle::resolve`] and
    /// [`Tokens::of`](super::theme::Tokens::of).
    fn default() -> Self {
        Self::resolve(Tokens::of(&Theme::Light))
    }
}

/// The panel's surface around `content`: the fill, the edge, the radius, the
/// shadow and the panel's text colour, inherited by everything inside.
///
/// A caller wraps its content with this when it wants the panel to look after
/// the surface, which is what the Slint example's `@children` slot gets. Like
/// [`Popover::panel`] it takes a function of the theme, so the surface follows
/// the appearance in force. The popover's own inset still applies — the content
/// is laid out inside the panel either way — so this adds no padding of its own.
///
/// [`PanelStyle::container_style`] is the same styling, for a caller that builds
/// the container itself; [`Popover::filled`] is the same surface drawn by the
/// panel, for content that is already a surface.
pub fn panel_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    style: impl Fn(&Theme) -> PanelStyle + 'a,
) -> Element<'a, Message>
where
    Message: 'a,
{
    container(content)
        .width(Length::Fill)
        .style(move |theme: &Theme| style(theme).container_style())
        .into()
}

/// The panel's own sub-widget state: the tree the caller's content lives in,
/// kept across frames so a field or a scroll position inside a panel does not
/// reset every time the panel opens.
struct State {
    panel: Tree,
}

impl State {
    fn new() -> Self {
        Self {
            panel: Tree::empty(),
        }
    }
}

/// Where the panel ends up, once the trigger and the window are known.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Placed {
    /// The panel's top-left corner, in window coordinates.
    origin: Point,
}

impl Placed {
    /// The panel's corner for a side, with no regard for the window.
    fn corner(trigger: Rectangle, panel: Size, gap: f32, placement: Placement) -> Point {
        match placement {
            Placement::Bottom => Point::new(
                trigger.x + (trigger.width - panel.width) / 2.0,
                trigger.y + trigger.height + gap,
            ),
            Placement::Top => Point::new(
                trigger.x + (trigger.width - panel.width) / 2.0,
                trigger.y - panel.height - gap,
            ),
            Placement::Left => Point::new(
                trigger.x - panel.width - gap,
                trigger.y + (trigger.height - panel.height) / 2.0,
            ),
            Placement::Right => Point::new(
                trigger.x + trigger.width + gap,
                trigger.y + (trigger.height - panel.height) / 2.0,
            ),
        }
    }

    /// The opposite side.
    fn flip(placement: Placement) -> Placement {
        match placement {
            Placement::Bottom => Placement::Top,
            Placement::Top => Placement::Bottom,
            Placement::Left => Placement::Right,
            Placement::Right => Placement::Left,
        }
    }

    /// Prefer `placement`, flip when it does not fit, and clamp the result into
    /// the window. The trigger is in window coordinates.
    fn resolve(
        trigger: Rectangle,
        panel: Size,
        window: Size,
        gap: f32,
        placement: Placement,
    ) -> Self {
        let fits = match placement {
            Placement::Bottom => {
                trigger.y + trigger.height + gap + panel.height <= window.height
            }
            Placement::Top => trigger.y - gap - panel.height >= 0.0,
            Placement::Left => trigger.x - gap - panel.width >= 0.0,
            Placement::Right => {
                trigger.x + trigger.width + gap + panel.width <= window.width
            }
        };
        let side = if fits { placement } else { Self::flip(placement) };
        let mut origin = Self::corner(trigger, panel, gap, side);
        // A panel with no room on either side clamps to the edge rather than
        // hanging out of the window.
        origin.x = origin.x.clamp(0.0, (window.width - panel.width).max(0.0));
        origin.y = origin.y.clamp(0.0, (window.height - panel.height).max(0.0));
        Self { origin }
    }
}

/// The overlay: it positions the caller's content against the trigger, draws the
/// panel's surface around it, and answers the two dismissals.
///
/// It borrows the content rather than owning it. The runtime builds a widget's
/// overlay twice per frame — once for the events, once for the paint — so an
/// overlay that took the content would leave the second call with nothing to
/// draw. See the module comment.
struct Panel<'a, 'b, Message> {
    content: &'b mut Element<'a, Message>,
    tree: &'b mut Tree,
    style: PanelStyle,
    /// Whether the surface is drawn here, around the content.
    filled: bool,
    /// The trigger, in window coordinates.
    trigger: Rectangle,
    gap: f32,
    padding: f32,
    width: f32,
    placement: Placement,
    /// Shared, not taken. The runtime builds a widget's overlay twice per frame,
    /// so a callback that moved out of the popover on the first build would be
    /// gone by the second — and the trigger's click would never reach the
    /// application.
    on_open_change: Option<std::rc::Rc<dyn Fn(bool) -> Message + 'a>>,
}

impl<Message> Panel<'_, '_, Message> {
    /// Ask the caller to close, once per request.
    fn dismiss(&self, shell: &mut Shell<'_, Message>) {
        if let Some(on_open_change) = &self.on_open_change {
            shell.publish(on_open_change(false));
        }
    }

    /// The child layout the content is drawn at, inside the panel's bounds.
    fn content_layout<'c>(layout: Layout<'c>) -> Layout<'c> {
        layout
            .children()
            .next()
            .expect("the panel lays out exactly one child")
    }
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer> for Panel<'_, '_, Message> {
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        // The panel is a fixed width, and the content is laid out inside it at
        // that width; `layout::padded` puts the panel's inset around whatever the
        // content turns out to be, so the node it returns — the panel — is the
        // surface the shadow and the fill are drawn into, and the content is not
        // clipped by it.
        let limits = layout::Limits::new(Size::ZERO, bounds)
            .width(Length::Fixed(self.width))
            .height(Length::Shrink);
        let panel_limits = limits.shrink(Padding::new(self.padding));
        let content = self.content
            .as_widget_mut()
            .layout(self.tree, renderer, &panel_limits);
        let size = content.size().expand(Padding::new(self.padding));
        let placed = Placed::resolve(self.trigger, size, bounds, self.gap, self.placement);
        let node = if self.padding == 0.0 {
            layout::Node::with_children(size, vec![content])
        } else {
            layout::Node::with_children(
                size,
                vec![content.move_to(Point::new(self.padding, self.padding))],
            )
        };
        node.translate(Vector::new(placed.origin.x, placed.origin.y))
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let bounds = layout.bounds();
        let inside = cursor.is_over(bounds);

        // The runtime gives an overlay the event before the widgets under it, so
        // this sees an outside press first and can stop it there. That is what
        // makes a press on the trigger while the panel is open close the panel
        // instead of also reaching the trigger and reopening it in the same
        // frame.
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        | Event::Touch(iced::touch::Event::FingerPressed { .. }) = event
            && !inside
            && !cursor.is_over(self.trigger)
        {
            self.dismiss(shell);
            shell.capture_event();
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(keyboard::key::Named::Escape),
            ..
        }) = event
        {
            self.dismiss(shell);
            shell.capture_event();
        }

        if inside {
            self.content.as_widget_mut().update(
                self.tree,
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                &bounds,
            );
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let content = Self::content_layout(layout);
        self.content
            .as_widget()
            .mouse_interaction(self.tree, content, cursor, &layout.bounds(), renderer)
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        inherited: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();
        if self.filled {
            self.style.draw(renderer, bounds);
        }

        let mut style = *inherited;
        style.text_color = self.style.text_color;
        // The overlay's node *is* the content, inside the panel's inset, so it
        // draws against its own layout.
        self.content
            .as_widget()
            .draw(self.tree, renderer, theme, &style, layout, cursor, &bounds);
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        operation.container(None, layout.bounds());
        let content = Self::content_layout(layout);
        operation.traverse(&mut |operation| {
            self.content
                .as_widget_mut()
                .operate(self.tree, content, renderer, operation);
        });
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Popover<'_, Message>
where
    Message: Clone + 'static,
{
    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<State>()
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(State::new())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.trigger.as_widget())]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[self.trigger.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.trigger.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.trigger.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.trigger
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
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
        // The base tree is drawn before the overlay is built for the same frame,
        // so this is the theme the panel about to be constructed belongs to.
        *self.theme.borrow_mut() = Some(theme.clone());
        self.trigger.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
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
        // The trigger opens the panel. The overlay leaves a press on the trigger
        // to this method for exactly this reason: the press that opens the panel
        // must reach the trigger, and the press that closes it must not.
        //
        // The hit test is against the bounds the runtime laid this widget out
        // with, which are in window coordinates — a widget's `position` is
        // relative to the scroll area's content, and in this gallery the trigger
        // reports y = 1399 there while the window shows it at y ~ 146.
        // The hit test is against the bounds the runtime laid this widget out
        // with. Inside a scroll area those are content coordinates, not window
        // ones — this gallery's trigger reports y = 1399 while the window shows
        // it at y ~ 146 — so a press only matches once the trigger is near the
        // top of the scrollable, where the two agree. The overlay below is
        // corrected with `translation`, which the runtime passes to `overlay`
        // for exactly that; `update` is handed no such offset, so this is the
        // one place the port is approximate. See the module comment.
        let trigger = layout.bounds();
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        | Event::Touch(iced::touch::Event::FingerPressed { .. }) = event
            && cursor.is_over(trigger)
            && !self.open
            && let Some(on_open_change) = &self.on_open_change
        {
            shell.publish(on_open_change(true));
        }
        self.trigger.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.trigger.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.trigger
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        _renderer: &iced::Renderer,
        _viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        if !self.open {
            return None;
        }
        let content = self.panel.as_mut()?;
        let state = tree.state.downcast_mut::<State>();
        let theme = self.theme.borrow().clone().unwrap_or(Theme::Light);
        let style = (self.style)(&theme);
        // The content's sub-tree is reconciled here, before the overlay is built
        // from it: iced's own `overlay` builds its menu the same way. Without it
        // the content's tree holds no widget state, and a text widget has no
        // paragraph to measure with — it lays out at zero height and draws
        // nothing.
        state.panel.diff(content.as_widget());
        // The overlay is laid out against the window, so the trigger has to be
        // in window coordinates: its layout position is in the scroll area's
        // content coordinates and `translation` is the offset between the two.
        let trigger = Rectangle::new(
            layout.position() - translation,
            layout.bounds().size(),
        );
        Some(overlay::Element::new(Box::new(Panel {
            content,
            tree: &mut state.panel,
            style,
            filled: self.filled,
            trigger,
            gap: self.offset,
            padding: self.padding,
            width: self.panel_width,
            placement: self.placement,
            on_open_change: self.on_open_change.clone(),
        })))
    }
}

impl<'a, Message> From<Popover<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'static,
{
    fn from(popover: Popover<'a, Message>) -> Self {
        Element::new(popover)
    }
}
