//! A modal surface: a dimmed backdrop with a centered panel carrying a title, a
//! description, the caller's content, a footer and a close button. Mirrors GPUI
//! Component's `Dialog` and the Slint `dialog` beside it.
//!
//! # What it composes
//!
//! The Slint file is the authority, and its panel is four things stacked with a
//! 16px gap inside a padded surface: a header row of `title`/`description` and
//! the close button, the caller's `@children`, and — placed by the caller as one
//! of those children — a [`DialogFooter`]. This port builds the same column:
//! [`Dialog`] draws the header from [`Dialog::title`] and [`Dialog::description`]
//! and takes the caller's content through [`Dialog::content`] and its
//! [`Dialog::footer`].
//!
//! # Why a layer and not the popover's overlay
//!
//! [`Popover`](super::popover) answers a click outside itself and can be one of
//! several overlays at once. iced delivers an event to *every* overlay in the
//! group that owns them (`overlay::Group` updates them all and ignores a
//! neighbour's capture), so two open overlays would each see a press meant for
//! the other. A dialog is exactly the case where the topmost layer must be the
//! only one answered, so this is not an overlay: it is an ordinary full-size
//! layer placed in a [`Stack`](iced::widget::Stack), and a stack routes an event
//! to its children **last first** and stops at the first capture
//! (`stack.rs:231`). Two dialogs therefore nest: the top one answers, the one
//! below never hears the press.
//!
//! The caller puts each dialog in the same stack as the page:
//!
//! ```ignore
//! stack![page, Dialog::new().open(open).title("Welcome").on_open_change(Message::Open)]
//! ```
//!
//! The layer fills whatever container holds it, which is the Slint component's
//! own contract (`width: 100%; height: 100%`): over a window it is a window
//! modal, over a card it is that card's. A closed dialog is zero-sized, so it
//! takes no room and captures nothing.
//!
//! # The surface and the dismissal
//!
//! The backdrop is the `overlay` token — the same dim GPUI and Slint draw — and
//! the panel is the popover fill, a one-pixel `border` edge, `radius-lg` and the
//! shadow token, the surface the rest of this crate's overlays use. Exactly one
//! container owns each: the backdrop container owns the dim, the panel container
//! owns the panel.
//!
//! A press on the backdrop asks to close when [`Dialog::overlay_closable`] allows
//! it. The panel itself is wrapped in a `mouse_area` whose only job is to capture
//! that press, so a click on the panel's own body reaches the scrim behind
//! nothing. iced captures an event only for a `mouse_area` that has a message, so
//! both wrappers publish through [`Dialog::on_open_change`]: the backdrop asks
//! for `false` when closing is allowed and for `true` — the state it is already
//! in — when it is not, and the panel always asks for `true`. A dialog with no
//! `on_open_change` has nothing to publish, draws neither capture, and is a
//! static panel.
//!
//! # What iced 0.14 has no way to express
//!
//! - **No focus trap.** There is no focus API a widget can take: iced offers a
//!   `text_input` its own focus and an `operation` that moves focus, but nothing
//!   a layer can hold. Tab therefore reaches whatever is behind the dialog, and
//!   focus is not moved into it or restored to the trigger when it closes. GPUI's
//!   dialog owns both; this port cannot, and says so rather than pretending.
//! - **No Escape at the layer.** An overlay is handed every key; an in-flow
//!   widget is handed a key only by a focused control. The component can listen
//!   for nothing, so the application binds Escape, exactly as the `date-picker`
//!   demo does for the panel iced's popover cannot take the key for. GPUI's
//!   `.keyboard(true)` is that binding written inside the component; here it is
//!   the caller's subscription.
//! - **No scroll lock.** A modal swallows presses but not the wheel, which
//!   continues to the page behind it. Slint's `TouchArea` does not consume the
//!   wheel either.
//! - **No transition.** The Slint panel has no open animation, so neither does
//!   this; a layer appears and disappears in one frame.
//! - **`content-padding` is a value, not a breakpoint.** Slint narrows its inset
//!   to 16px when the frame is under 420px wide; iced's container has no
//!   width-conditional padding, so the inset is the value given and the panel is
//!   capped by [`Dialog::max_width`] with a fixed 24px side margin.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::widget::{Space, column, container, mouse_area, row, svg, text};
use iced::{Alignment, Border, Color, Element, Length, Shadow, Theme, Vector, mouse};

use super::button::{self, Button};
use super::theme::{self, ControlSize, Tokens};

/// The gap between the panel's inset and the frame it is centered in, so the
/// panel never touches the window edge. Slint uses 48px in a frame at least
/// 420px wide, which is every window the gallery has; its 16px narrow-window
/// step is named in the module comment.
const PANEL_MARGIN: f32 = 24.0;

/// The panel's inset, the Slint `content-padding` default of 20px at a normal
/// width.
const PANEL_PADDING: f32 = 20.0;

/// The close glyph, the same stroke the crate's other icons use.
const CLOSE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>"#;

/// A modal panel, built with the methods below.
///
/// It becomes an [`Element`] through `From`, and what it becomes is a full-size
/// layer for a [`Stack`](iced::widget::Stack): the dimmed backdrop and the
/// centered panel. It draws nothing at all while [`open`](Self::open) is false.
pub struct Dialog<'a, Message> {
    open: bool,
    title: String,
    description: String,
    content: Option<Element<'a, Message>>,
    footer: Option<Element<'a, Message>>,
    close_button: bool,
    overlay_closable: bool,
    content_padding: Option<f32>,
    max_width: f32,
    style: Box<dyn Fn(&Theme) -> Style + 'a>,
    on_open_change: Option<Rc<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message> Dialog<'a, Message> {
    /// A dialog with no title, no content and no footer. It starts closed,
    /// because the caller's own state says what is open.
    pub fn new() -> Self {
        Self {
            open: false,
            title: String::new(),
            description: String::new(),
            content: None,
            footer: None,
            close_button: true,
            overlay_closable: true,
            content_padding: None,
            max_width: 440.0,
            style: Box::new(|theme: &Theme| Style::resolve(Tokens::of(theme))),
            on_open_change: None,
        }
    }

    /// Whether the layer is showing. The caller owns this: it is set from the
    /// application's state, and [`on_open_change`](Self::on_open_change) is how
    /// the application learns it should change.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// The heading, drawn in the panel's title colour. Empty draws no title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// The line under the title, drawn in the panel's description colour and
    /// wrapped to the panel's width. Empty draws no line.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// The dialog's body, the Slint `@children` slot. Any element: a column of
    /// fields, an icon row, a scroll area. A caller that wants the panel's own
    /// inset around several pieces puts them in a `column!` with the spacing it
    /// wants.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// The action row under the body. [`DialogFooter`] is the shape the Slint
    /// component gives it — the buttons at the trailing edge with an 8px gap —
    /// and is what the gallery uses; any element is accepted.
    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Whether the header's close button is drawn. Slint's `show-close`,
    /// true by default.
    pub fn close_button(mut self, close_button: bool) -> Self {
        self.close_button = close_button;
        self
    }

    /// Whether a press on the dimmed backdrop asks to close. Slint's
    /// `overlay-closable`, true by default. When false the backdrop still
    /// captures the press, so it never reaches the page behind; it just asks to
    /// stay open instead.
    pub fn overlay_closable(mut self, overlay_closable: bool) -> Self {
        self.overlay_closable = overlay_closable;
        self
    }

    /// The panel's inset, Slint's `content-padding`, 20px by default.
    pub fn content_padding(mut self, padding: f32) -> Self {
        self.content_padding = Some(padding);
        self
    }

    /// The widest the panel may be, 440px by default — the Slint cap.
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    /// The panel's surface and its two text colours. `Style::resolve` is the
    /// default, so a caller overrides only what it means to.
    ///
    /// It is a function of the theme rather than a value because the appearance
    /// in force is not known where the panel is built; a surface resolved for one
    /// appearance is the wrong colour in the other. This crate's `popover.rs`
    /// says the same thing about its own panel.
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self {
        self.style = Box::new(style);
        self
    }

    /// Report every request to change the open state: the close button, a press
    /// on the backdrop, or the panel asking to stay open.
    ///
    /// This is the only way the dialog speaks. It never closes itself, so a
    /// caller that wants it to move must answer with its own state — which is
    /// also what lets a control inside the panel close it by sending the same
    /// message a backdrop press would.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    /// The layer, or nothing at all while the dialog is closed.
    fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let Self {
            open,
            title,
            description,
            content,
            footer,
            close_button,
            overlay_closable,
            content_padding,
            max_width,
            style,
            on_open_change,
        } = self;

        if !open {
            // A closed dialog is out of the layout entirely: a zero-sized space
            // takes no room and captures nothing.
            return Space::new().into();
        }

        let style: Rc<dyn Fn(&Theme) -> Style + 'a> = Rc::from(style);
        let panel = panel_content(
            &style,
            title,
            description,
            content,
            footer,
            close_button,
            on_open_change.as_ref(),
        );

        let panel_box = container(panel)
            .width(Length::Fill)
            .max_width(max_width)
            .padding(content_padding.unwrap_or(PANEL_PADDING))
            .style({
                let style = style.clone();
                move |theme: &Theme| style(theme).container_style()
            });

        // The panel captures the press so a click on its own body does not reach
        // the backdrop under it. iced captures only for a `mouse_area` that has a
        // message, so the capture is expressed as "ask to stay open".
        let blocker: Element<'a, Message> = match &on_open_change {
            Some(on_open_change) => mouse_area(panel_box)
                .on_press(on_open_change(true))
                .interaction(mouse::Interaction::Idle)
                .into(),
            None => panel_box.into(),
        };

        let centered = container(blocker)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([0.0, PANEL_MARGIN])
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        // The backdrop. When closing is not allowed it still asks for the state
        // it is in, which is what captures the press rather than the answer.
        let backdrop_message = on_open_change.as_ref().map(|f| f(!overlay_closable));
        let backdrop = container(Space::new().width(Length::Fill).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .style({
                let style = style.clone();
                move |theme: &Theme| style(theme).backdrop_style()
            });

        let backdrop: Element<'a, Message> = match backdrop_message {
            Some(message) => mouse_area(backdrop)
                .on_press(message)
                .interaction(mouse::Interaction::Idle)
                .into(),
            None => mouse_area(backdrop)
                .interaction(mouse::Interaction::Idle)
                .into(),
        };

        iced::widget::stack![backdrop, centered]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<Message> Default for Dialog<'_, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The panel's column: the header, the caller's body and the footer.
fn panel_content<'a, Message>(
    style: &Rc<dyn Fn(&Theme) -> Style + 'a>,
    title: String,
    description: String,
    content: Option<Element<'a, Message>>,
    footer: Option<Element<'a, Message>>,
    show_close: bool,
    on_open_change: Option<&Rc<dyn Fn(bool) -> Message + 'a>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let mut headings = column![].spacing(4.0);
    if !title.is_empty() {
        let style = style.clone();
        headings = headings.push(
            text(title)
                .size(theme::FONT_SIZE_LG)
                .font(iced::Font {
                    weight: iced::font::Weight::Semibold,
                    ..iced::Font::DEFAULT
                })
                .style(move |theme: &Theme| text::Style {
                    color: Some(style(theme).title()),
                }),
        );
    }
    if !description.is_empty() {
        let style = style.clone();
        headings = headings.push(text(description).size(theme::FONT_SIZE_SM).style(
            move |theme: &Theme| text::Style {
                color: Some(style(theme).description()),
            },
        ));
    }

    // The title column takes the room, so the close button sits at the trailing
    // edge; with no title it is the whole header and the column is empty.
    let mut header = row![container(headings).width(Length::Fill)]
        .spacing(12.0)
        .align_y(Alignment::Start);
    if show_close && let Some(on_open_change) = on_open_change {
        header = header.push(close_button(on_open_change(false)));
    }

    let mut body = column![header].spacing(16.0);
    if let Some(content) = content {
        body = body.push(content);
    }
    if let Some(footer) = footer {
        body = body.push(footer);
    }
    body.into()
}

/// The header's close control: a ghost, extra-small, icon-only button, the shape
/// Slint's `Button { icon: "x"; variant: ghost; size: xsmall }` has.
fn close_button<'a, Message>(message: Message) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let glyph = svg(svg::Handle::from_memory(CLOSE_XML))
        .width(16.0)
        .height(16.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(button::ink(
                Tokens::of(theme),
                button::Variant::Ghost,
                false,
            )),
        });

    Button::new("")
        .icon(glyph)
        .variant(button::Variant::Ghost)
        .size(ControlSize::XSmall)
        .on_press(message)
        .into()
}

/// The panel's action row: its children at the trailing edge with an 8px gap,
/// Slint's `DialogFooter`.
///
/// The order is the order the children are added, so a caller writes its
/// secondary action first and its primary last, exactly as the reference does.
pub struct DialogFooter<'a, Message> {
    children: Vec<Element<'a, Message>>,
    centered: bool,
}

impl<'a, Message> DialogFooter<'a, Message> {
    /// An empty footer. Buttons are added with [`child`](Self::child).
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            centered: false,
        }
    }

    /// Add one button, in reading order.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Center the row instead of trailing it. Slint's `centered`.
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }
}

impl<Message> Default for DialogFooter<'_, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<DialogFooter<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(footer: DialogFooter<'a, Message>) -> Self {
        let alignment = if footer.centered {
            Alignment::Center
        } else {
            Alignment::End
        };

        container(row(footer.children).spacing(8.0))
            .width(Length::Fill)
            .align_x(alignment)
            .into()
    }
}

/// The panel's surface and the colours its text is drawn in.
///
/// [`Style::resolve`] builds one from the theme tokens. The fields are private
/// and read through methods, so this can grow without breaking a caller.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    fill: Color,
    border: Color,
    border_width: f32,
    radius: f32,
    shadow: Shadow,
    backdrop: Color,
    title_color: Color,
    description_color: Color,
    text_color: Color,
}

impl Style {
    /// The panel the tokens describe: the popover fill, the `border` token as a
    /// one-pixel edge, `radius-lg` and the shadow token 4px below, over the
    /// `overlay` token's dim backdrop — the same surface every other overlay in
    /// this crate draws.
    ///
    /// The edge is the `border` token and not a fixed grey: the token is
    /// `#e5e5e5` on white and `#262626` on near-black, so the edge exists in
    /// both appearances. The fill is `popover` — white in the light theme,
    /// `#171717` in the dark one, one step off the page's `#0a0a0a` — so the
    /// panel separates from the backdrop in both.
    pub fn resolve(tokens: Tokens) -> Self {
        Self {
            fill: tokens.popover,
            border: tokens.border,
            border_width: 1.0,
            radius: theme::RADIUS_LG,
            // iced's blur is a Gaussian sigma where Slint's is a spread; 8 in the
            // dark theme and 4 in the light one carry the same weight as Slint's
            // 24px at this offset, and match the crate's other surfaces.
            shadow: Shadow {
                color: tokens.shadow,
                offset: Vector::new(0.0, 4.0),
                blur_radius: if tokens.dark { 8.0 } else { 4.0 },
            },
            backdrop: tokens.overlay,
            title_color: tokens.foreground,
            description_color: tokens.muted_foreground,
            text_color: tokens.foreground,
        }
    }

    /// The panel fill. Slint's `panel-background`.
    pub fn fill(mut self, color: Color) -> Self {
        self.fill = color;
        self
    }

    /// The panel edge. A `width` of 0 draws no edge, which is Slint's
    /// `panel-border-width: 0`.
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

    /// The dim drawn over everything behind the panel. Slint's `Theme.overlay`.
    pub fn backdrop(mut self, color: Color) -> Self {
        self.backdrop = color;
        self
    }

    /// The title's ink. Slint's `title-color`.
    pub fn title_color(mut self, color: Color) -> Self {
        self.title_color = color;
        self
    }

    /// The description's ink. Slint's `description-color`.
    pub fn description_color(mut self, color: Color) -> Self {
        self.description_color = color;
        self
    }

    /// The colour text inside the panel inherits, which is the foreground
    /// unless a caller drawing on a filled panel says otherwise.
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }

    pub fn title(&self) -> Color {
        self.title_color
    }

    pub fn description(&self) -> Color {
        self.description_color
    }

    pub fn content_color(&self) -> Color {
        self.text_color
    }

    /// The panel's surface as a container's styling.
    pub fn container_style(&self) -> container::Style {
        container::Style {
            text_color: Some(self.text_color),
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

    /// The backdrop's styling: the dim, with no edge and no shadow.
    pub fn backdrop_style(&self) -> container::Style {
        container::Style {
            background: Some(self.backdrop.into()),
            ..container::Style::default()
        }
    }
}

impl Default for Style {
    /// The light theme's resting panel. A caller that wants the appearance in
    /// force builds it with [`Style::resolve`] and [`Tokens::of`].
    fn default() -> Self {
        Self::resolve(Tokens::of(&Theme::Light))
    }
}

impl<'a, Message> From<Dialog<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(dialog: Dialog<'a, Message>) -> Self {
        dialog.into_element()
    }
}
