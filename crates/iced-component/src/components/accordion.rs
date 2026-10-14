//! A vertical stack of collapsible sections. Mirrors GPUI Component's
//! `Accordion` and the Slint version beside it: a bordered card by default,
//! sizes, a disabled state, several sections open at once with `multiple`, and
//! a toggle event carrying the section and its new state.
//!
//! The registry key is `accordion`; the file and the module share the name.
//!
//! A section is a header row over a content panel, with a 1px separator between
//! sections, and a chevron marking whether it is open. The insets are the Slint
//! ones at each size: 6×4 at xsmall, 8×6 at small, 12×8 at medium and 16×12 at
//! large, with text-xs, text-sm, text-sm and text-md titles. The chevron box is
//! 12px, 14px at the large size.
//!
//! **The header row's content is the caller's element**, built with the
//! interactive part the caller wants — `button("Details").style(button::text)
//! .on_press(..)`, a `mouse_area`, or a row of controls. Slint's
//! `AccordionItem` owns its clickable title and reports `toggled(open)`; an
//! iced control carries its own message instead, so the click that toggles is
//! the caller's, and this component supplies the chrome around it. That is the
//! same split the two libraries already have, and it is what
//! [`accordion::text_header`] and [`accordion::labeled_header`] build for the
//! common case.
//!
//! **The whole header is one target, the chevron lane included.** The trailing
//! lane is component chrome the caller never wired to anything, so a hit area
//! that stopped at the caller's own element would leave the arrow inert — the
//! one mark on the row that says "this opens" would be the one place a click did
//! nothing. [`header_target`] therefore covers the entire row, chevron lane and
//! all, exactly as the Slint `TouchArea` overlays its `Rectangle`, and reports
//! the row's own message; [`Section::on_toggle`] is how a caller that did not
//! build the row's press itself says what that message is. A press is still
//! routed to the caller's element first, so a control *inside* the header keeps
//! its own behaviour and only a click the content did not take reaches the row.
//!
//! This is the one place that knows a header row has a trailing lane; a caller
//! that mounts a header through [`header_target`] gets it, and a caller that
//! does not keeps the plain row it asked for.
//!
//! Which sections are open is the caller's state too. iced has no `in-out`
//! property to bind, so the caller keeps the open set and passes each section's
//! `open` in.
//!
//! **The panel is [`collapsible::reveal`].** A section's content is wrapped in
//! the same animated reveal the collapsible panel uses, so the height moves from
//! nothing to the content's measured size over [`collapsible::REVEAL_DURATION`]
//! — the 150ms Slint gives `animate height` — and a toggle mid-flight reverses
//! from the height on screen. The content stays mounted while the section is
//! closed, because that is what lets the reveal measure it and turn around; this
//! is the composition rather than a second animation, so the accordion inherits
//! the mechanism and its timing instead of re-deriving them.
//!
//! Two things the composition does not carry. The chevron is drawn in its end
//! state: iced 0.14 has no rotation transition for an arbitrary widget, so
//! Slint's `animate transform-rotation { duration: 150ms; easing: ease-out; }`
//! has no counterpart here and the reveal alone carries the motion. The
//! chevron itself is Lucide's `chevron-down` while closed and `chevron-up`
//! while open — the two ends of the 180° turn Slint animates — embedded here
//! because iced ships no icon set, and swapped rather than turned. And the
//! Slint accordion writes `easing: ease-out` on its height while the collapsible
//! authority names no easing — linear — so the shared reveal is a straight line;
//! the duration matches, the curve is the collapsible's.
//!
//! A caller putting a `Canvas` in a section should know the section moves every
//! frame while it reveals: under the web fallback a `Canvas` in a region a height
//! animation moves smears, so prefer a widget that draws quads. This file itself
//! draws no `Canvas`.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{button, column, container, row, svg, text};
use iced::widget::container::Style as ContainerStyle;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{tree, Tree, Widget};
use iced::advanced::Renderer as _;
use iced::advanced::{mouse, overlay, renderer, Clipboard, Shell};
use iced::{Alignment, Background, Border, Color, Element, Event, Length, Rectangle, Size, Theme};

use super::collapsible;
use super::theme::{self, ControlSize, Tokens};

/// One section: the header row, the panel below it and whether it is open.
pub struct Section<'a, Message> {
    header: Element<'a, Message>,
    content: Vec<Element<'a, Message>>,
    open: bool,
    chevron: bool,
    disclosure: Option<Element<'a, Message>>,
    on_toggle: Option<Message>,
}

impl<'a, Message> Section<'a, Message> {
    /// A section whose header is the caller's row. Put the click that toggles
    /// it inside that row, or name it with [`Self::on_toggle`].
    pub fn new(header: impl Into<Element<'a, Message>>) -> Self {
        Self {
            header: header.into(),
            content: Vec::new(),
            open: false,
            chevron: true,
            disclosure: None,
            on_toggle: None,
        }
    }

    /// Whether the panel is shown. The caller's state decides, and the caller
    /// flips it from the header row's message.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// A panel child. Call it more than once to stack the panel's content.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content.push(content.into());
        self
    }

    /// Draws the chevron in its end state, down when closed and up when open.
    pub fn chevron(mut self, chevron: bool) -> Self {
        self.chevron = chevron;
        self
    }

    /// The caller's own disclosure, drawn in the chevron lane instead of the
    /// default arrow. The icon set belongs to the caller — the component only
    /// carries Lucide's two chevrons as a fallback — so pass the mark the open
    /// state wants: this element is drawn as it is handed over.
    pub fn disclosure(mut self, disclosure: impl Into<Element<'a, Message>>) -> Self {
        self.disclosure = Some(disclosure.into());
        self
    }

    /// The row itself toggles, reporting this message, for a header whose
    /// element does not carry the press on its own — which is what makes the
    /// chevron lane live rather than inert. The header element keeps its own
    /// clicks: a press the content takes is answered by the content, and only a
    /// click the content left unhandled reaches the row.
    pub fn on_toggle(mut self, on_toggle: Message) -> Self {
        self.on_toggle = Some(on_toggle);
        self
    }
}

/// An accordion, built with the methods below.
pub struct Accordion<'a, Message> {
    sections: Vec<Section<'a, Message>>,
    size: ControlSize,
    bordered: bool,
}

impl<'a, Message> Accordion<'a, Message> {
    pub fn new(sections: impl IntoIterator<Item = Section<'a, Message>>) -> Self {
        Self {
            sections: sections.into_iter().collect(),
            size: ControlSize::default(),
            bordered: true,
        }
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Removes the outer border and the radius, `bordered: false` in Slint.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// The horizontal inset at each size: 6 / 8 / 16 / 12, the Slint values.
    pub fn padding_x(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 6.0,
            ControlSize::Small => 8.0,
            ControlSize::Large => 16.0,
            ControlSize::Medium => 12.0,
        }
    }

    /// The vertical inset at each size: 4 / 6 / 12 / 8.
    pub fn padding_y(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 4.0,
            ControlSize::Small => 6.0,
            ControlSize::Large => 12.0,
            ControlSize::Medium => 8.0,
        }
    }

    /// The title size: text-xs at xsmall, text-md at large, text-sm between.
    pub fn label_size(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => theme::FONT_SIZE_XS,
            ControlSize::Large => theme::FONT_SIZE_MD,
            _ => theme::FONT_SIZE_SM,
        }
    }

    /// The chevron box: 12px, and 14px at the large size.
    pub fn chevron_size(&self) -> f32 {
        match self.size {
            ControlSize::Large => 14.0,
            _ => 12.0,
        }
    }
}

/// The colour a section's title is drawn in.
pub fn ink(tokens: Tokens, enabled: bool) -> Color {
    if enabled {
        tokens.foreground
    } else {
        tokens.muted_foreground
    }
}

/// The header's and the panel's insets: the Trigger's own padding is the
/// authority, and it is the same `padding-x`/`padding-y` pair the Slint
/// `AccordionItem` puts on its header row and repeats on its content column —
/// `padding-left`/`padding-right` at `padding-x`, `padding-top`/`padding-bottom`
/// at `padding-y`.
///
/// A row-wide target hands the trailing lane its right inset on its own side —
/// `lane_inset` — because a chevron sits flush against the row's own edge only
/// while the target is the caller's element; the row inside a target keeps the
/// lane's pixel off the card instead.
fn header_padding(pad_x: f32, pad_y: f32, lane_inset: f32) -> iced::Padding {
    iced::Padding {
        top: pad_y,
        right: pad_x + lane_inset,
        bottom: pad_y,
        left: pad_x,
    }
}

/// A section's header background at rest, hovered and pressed. Pressed is the
/// accent and hovered is the accent at half strength, which is what the Slint
/// theme does; a disabled header draws nothing.
pub fn header_fill(tokens: Tokens, enabled: bool, hovered: bool, pressed: bool) -> Color {
    if !enabled {
        Color::TRANSPARENT
    } else if pressed {
        tokens.accent
    } else if hovered {
        theme::alpha(tokens.accent, 0.5)
    } else {
        Color::TRANSPARENT
    }
}

/// The common header: a title, a chevron and nothing else, as a ghost button
/// that fills the row. Pass the message to toggle this section.
pub fn text_header<'a, Message: Clone + 'a>(
    title: impl Into<String>,
    label_size: f32,
    on_toggle: Message,
) -> Element<'a, Message> {
    let label = title.into();
    let content: Element<'a, Message> = row![
        text(label)
            .size(label_size)
            .width(Length::Fill)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..iced::Font::DEFAULT
            })
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
    ]
    .width(Length::Fill)
    .into();

    button(content)
        .width(Length::Fill)
        .padding(0.0)
        .style(|theme: &Theme, status| {
            let tokens = Tokens::of(theme);
            let hovered = matches!(
                status,
                button::Status::Hovered | button::Status::Pressed
            );
            button::Style {
                background: Some(Background::Color(if hovered {
                    theme::alpha(tokens.accent, 0.5)
                } else {
                    Color::TRANSPARENT
                })),
                text_color: tokens.foreground,
                border: Border::default(),
                ..button::Style::default()
            }
        })
        .on_press(on_toggle)
        .into()
}

/// A header with a leading icon element beside the title, as a ghost button
/// that fills the row. The caller owns the icon, because the icon set does.
pub fn labeled_header<'a, Message: Clone + 'a>(
    icon: impl Into<Element<'a, Message>>,
    title: impl Into<String>,
    label_size: f32,
    gap: f32,
    on_toggle: Message,
) -> Element<'a, Message> {
    let label = title.into();
    let content: Element<'a, Message> = row![
        icon.into(),
        text(label)
            .size(label_size)
            .width(Length::Fill)
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..iced::Font::DEFAULT
            })
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
    ]
    .spacing(gap)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into();

    button(content)
        .width(Length::Fill)
        .padding(0.0)
        .style(|theme: &Theme, status| {
            let tokens = Tokens::of(theme);
            let hovered = matches!(
                status,
                button::Status::Hovered | button::Status::Pressed
            );
            button::Style {
                background: Some(Background::Color(if hovered {
                    theme::alpha(tokens.accent, 0.5)
                } else {
                    Color::TRANSPARENT
                })),
                text_color: tokens.foreground,
                border: Border::default(),
                ..button::Style::default()
            }
        })
        .on_press(on_toggle)
        .into()
}

/// The default disclosure: Lucide's `chevron-down` while closed and
/// `chevron-up` while open, in the muted foreground — the two ends of the 180°
/// turn Slint's accordion animates on one `chevron-down`. Swapped, not turned,
/// because iced 0.14 has no rotation transition for an arbitrary widget. A
/// caller with its own icon set passes the element through
/// [`Section::disclosure`] instead.
pub fn chevron<'a, Message: 'a>(open: bool, size: f32) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(if open {
        CHEVRON_UP
    } else {
        CHEVRON_DOWN
    }))
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .style(|theme: &Theme, _status| svg::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    })
    .into()
}

/// A header row that toggles as one target, chevron lane included.
///
/// The interactive element and the trailing line are the two halves a section's
/// header has, and iced lays a `Row` out itself with no seam to widen one
/// child's hit box into its sibling's. This widget draws the two halves exactly
/// as that row would — the same spacing, the same centring, no gap between them
/// — and answers a press anywhere in the result with `on_toggle`. So the lane
/// the component's chevron sits in is inside the target rather than beside it,
/// which is what the Slint authority does with a `TouchArea` over the row.
///
/// The press goes to the content first. A control inside the header — a link, a
/// second button — therefore keeps its own behaviour, and the row's message is
/// published only when the content left the click unhandled. Hover and press for
/// the row's own fill are reported through `background`: it is called with the
/// hovered and pressed states and returns one layer drawn under the row, so the
/// caller's fill and the component's stay one implementation.
pub fn header_target<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    disclosure: impl Into<Element<'a, Message>>,
    spacing: f32,
    on_toggle: Message,
    background: impl Fn(&Theme, bool, bool) -> Option<BackgroundStyle> + 'a,
) -> Element<'a, Message> {
    Element::new(HeaderTarget {
        content: content.into(),
        disclosure: disclosure.into(),
        spacing,
        on_toggle,
        background: Box::new(background),
    })
}

/// The fill a header row draws under itself, resolved inside the closure that
/// asked for it — a colour read when the element is built is the light theme's
/// colour in both themes.
#[derive(Debug, Clone, Copy)]
pub struct BackgroundStyle {
    pub color: Color,
    pub radius: f32,
}

impl Default for BackgroundStyle {
    fn default() -> Self {
        Self {
            color: Color::TRANSPARENT,
            radius: 0.0,
        }
    }
}

/// The row's own fill: asked for on each draw with the state the row is in, so
/// the colour is resolved from the theme in hand rather than when the element
/// was built.
type RowFill<'a> = Box<dyn Fn(&Theme, bool, bool) -> Option<BackgroundStyle> + 'a>;

/// A row's hover/press state, kept in the widget tree: iced repaints on request,
/// so a press that changes the fill has to ask for one itself.
#[derive(Debug, Default)]
struct State {
    hovered: bool,
    pressed: bool,
}

struct HeaderTarget<'a, Message> {
    content: Element<'a, Message>,
    disclosure: Element<'a, Message>,
    spacing: f32,
    on_toggle: Message,
    background: RowFill<'a>,
}

impl<'a, Message> Widget<Message, Theme, iced::Renderer> for HeaderTarget<'a, Message>
where
    Message: Clone + 'a,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content), Tree::new(&self.disclosure)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.content, &self.disclosure]);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // The two children are laid out against the same limits rather than one
        // after the other — a `Row` would have to shrink the second for the
        // first — and then placed side by side. The content takes what is left
        // of the row, so the lane stays at the row's trailing edge.
        let [content_state, disclosure_state] = &mut tree.children[..] else {
            unreachable!("a header row has exactly the content and the lane")
        };

        let content = self.content.as_widget_mut().layout(
            content_state,
            renderer,
            &limits.loose(),
        );
        let lane = self.disclosure.as_widget_mut().layout(
            disclosure_state,
            renderer,
            &limits.loose(),
        );
        let lane_width = lane.size().width;
        let width = (content.size().width + self.spacing + lane_width).max(0.0);

        layout::Node::with_children(
            Size::new(width, content.size().height.max(lane.size().height)),
            vec![
                content.move_to(iced::Point::new(0.0, 0.0)),
                lane.move_to(iced::Point::new(width - lane_width, 0.0)),
            ],
        )
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
        // The children are updated at the cursor they already have, so a press
        // over the chevron lane never reaches the content: its own bounds say
        // the cursor is outside it, and only this widget's row-wide bounds say
        // otherwise.
        for (index, child) in [&mut self.content, &mut self.disclosure]
            .into_iter()
            .enumerate()
        {
            child.as_widget_mut().update(
                &mut tree.children[index],
                event,
                layout.child(index),
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        let bounds = layout.bounds();
        let over = cursor.is_over(bounds);
        let state = tree.state.downcast_mut::<State>();

        match event {
            Event::Mouse(iced::mouse::Event::ButtonPressed(iced::mouse::Button::Left))
            | Event::Touch(iced::touch::Event::FingerPressed { .. }) => {
                if over && !shell.is_event_captured() {
                    state.pressed = true;
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left))
            | Event::Touch(iced::touch::Event::FingerLifted { .. }) => {
                if state.pressed {
                    state.pressed = false;
                    if over {
                        shell.publish(self.on_toggle.clone());
                    }
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Touch(iced::touch::Event::FingerLost { .. }) => {
                state.pressed = false;
                shell.request_redraw();
            }
            Event::Window(iced::window::Event::RedrawRequested(_)) => {
                state.hovered = over;
            }
            _ => {
                if state.hovered != over {
                    state.hovered = over;
                    shell.request_redraw();
                }
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let children = [&self.content, &self.disclosure]
            .into_iter()
            .enumerate()
            .map(|(index, child)| {
                child.as_widget().mouse_interaction(
                    &tree.children[index],
                    layout.child(index),
                    cursor,
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default();

        if children == mouse::Interaction::default() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            children
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
        let state = tree.state.downcast_ref::<State>();

        if let Some(fill) = (self.background)(theme, state.hovered, state.pressed) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: Border {
                        radius: fill.radius.into(),
                        ..Border::default()
                    },
                    ..renderer::Quad::default()
                },
                fill.color,
            );
        }

        for (index, child) in [&self.content, &self.disclosure]
            .into_iter()
            .enumerate()
        {
            child.as_widget().draw(
                &tree.children[index],
                renderer,
                theme,
                style,
                layout.child(index),
                cursor,
                viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: iced::Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        // Both children's paths travel together, because a header's content —
        // a select, a menu button — may open one.
        let [content_state, disclosure_state] = &mut tree.children[..] else {
            unreachable!("a header row has exactly the content and the lane")
        };

        let overlays = [
            self.content.as_widget_mut().overlay(
                content_state,
                layout.child(0),
                renderer,
                viewport,
                translation,
            ),
            self.disclosure.as_widget_mut().overlay(
                disclosure_state,
                layout.child(1),
                renderer,
                viewport,
                translation,
            ),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        (!overlays.is_empty()).then(|| overlay::Group::with_children(overlays).overlay())
    }
}

impl<'a, Message> From<HeaderTarget<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(target: HeaderTarget<'a, Message>) -> Self {
        Element::new(target)
    }
}

impl<'a, Message: Clone + 'a> From<Accordion<'a, Message>> for Element<'a, Message> {
    fn from(control: Accordion<'a, Message>) -> Self {
        let bordered = control.bordered;
        let pad_x = control.padding_x();
        let pad_y = control.padding_y();
        let chevron_size = control.chevron_size();

        let count = control.sections.len();
        let mut sections = column![].width(Length::Fill);

        for (index, section) in control.sections.into_iter().enumerate() {
            let open = section.open;
            let last = index + 1 == count;
            let enabled = section.on_toggle.is_some();

            let lane: Element<'a, Message> = match section.disclosure {
                Some(disclosure) => container(disclosure).center_y(chevron_size).into(),
                None if section.chevron => {
                    container(chevron(open, chevron_size)).center_y(chevron_size).into()
                }
                None => {
                    container(iced::widget::Space::new())
                        .width(Length::Fixed(0.0))
                        .into()
                }
            };

            let title = container(section.header).width(Length::Fill);

            // Until a section says what its row reports, the chrome is exactly
            // what it has always been: a plain row with the caller's own
            // interactive element in it. `on_toggle` is what mounts the
            // row-wide target and takes the chevron lane into it, so a caller
            // that already wired its header keeps the tree it had.
            let header: Element<'a, Message> = match section.on_toggle {
                Some(on_toggle) => header_target(
                    title.padding(header_padding(pad_x, pad_y, 0.0)),
                    lane,
                    12.0,
                    on_toggle,
                    move |theme: &Theme, hovered, pressed| {
                        Some(BackgroundStyle {
                            color: header_fill(Tokens::of(theme), enabled, hovered, pressed),
                            radius: theme::RADIUS_SM,
                        })
                    },
                ),
                None => row![title.padding(header_padding(pad_x, pad_y, pad_x)), lane]
                    .spacing(12.0)
                    .align_y(Alignment::Center)
                    .into(),
            };
            sections = sections.push(header);

            if !section.content.is_empty() {
                // Wrapped rather than mounted-or-not: the reveal measures the
                // content at its full height and clips it to the fraction of
                // that height the section has reached, so a closed section keeps
                // its content in the tree instead of dropping it and re-mounting
                // it — which is what lets a toggle mid-flight reverse from where
                // the panel already is. This is the collapsible panel's own
                // element, so the accordion inherits that animation rather than
                // re-deriving it.
                let body = column(section.content)
                    .width(Length::Fill)
                    .padding(header_padding(pad_x, pad_y, 0.0));
                sections = sections.push(collapsible::reveal(body, open));
            }

            if !last {
                sections = sections.push(
                    container(iced::widget::Space::new())
                        .width(Length::Fill)
                        .height(Length::Fixed(1.0))
                        .style(|theme: &Theme| ContainerStyle {
                            background: Some(Tokens::of(theme).border.into()),
                            ..ContainerStyle::default()
                        }),
                );
            }
        }

        container(sections)
            .width(Length::Fill)
            .padding(if bordered { 1.0 } else { 0.0 })
            .style(move |theme: &Theme| {
                let tokens = Tokens::of(theme);
                ContainerStyle {
                    background: Some(tokens.background.into()),
                    border: Border {
                        color: tokens.border,
                        width: if bordered { 1.0 } else { 0.0 },
                        radius: if bordered {
                            theme::RADIUS_LG.into()
                        } else {
                            0.0.into()
                        },
                    },
                    ..ContainerStyle::default()
                }
            })
            .into()
    }
}

// The default disclosure's artwork, taken from the icon set this repository
// ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the two galleries
// draw the same marks rather than two lookalikes. Slint's accordion draws
// `chevron-down` and turns it 180° on open; these are its two ends.

/// Lucide's `chevron-down`, the closed disclosure.
const CHEVRON_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#;

/// Lucide's `chevron-up`, the open disclosure.
const CHEVRON_UP: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 15-6-6-6 6"/></svg>"#;
