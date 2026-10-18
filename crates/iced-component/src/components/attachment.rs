//! A file or media chip. Mirrors GPUI Component's `Attachment`, through the
//! Slint version beside it: a media slot, a metadata stack, an action and a
//! remove control, the five lifecycle states, four densities and a vertical
//! preview. `name` is the title and `description` is the second line — the
//! Slint property called `size`. A `progress` below 100 draws the metadata's
//! bar, and a preview that is not at full strength fades toward the media fill.
//!
//! The two exports are [`Attachment`] for one chip and [`Group`] for the
//! horizontally scrolling row of them, which is the Slint `AttachmentGroup`.
//!
//! An iced control carries its messages rather than calling back, so what Slint
//! expresses as `closable` and `clickable` flags with callbacks is here the
//! presence of a message: `.on_remove(..)` draws the remove control, and
//! `.on_press(..)` makes the whole card activate and draw the muted hover
//! surface. An action without a message draws the disabled treatment, which is
//! how iced reports a control that cannot be pressed, and a card without one
//! simply does not activate.
//!
//! **What this file draws itself, and why.** iced has no icon set and an
//! application owns its assets, so the media glyph and the preview are the
//! caller's elements: `.icon(..)` is drawn centred when there is no preview and
//! `.preview(..)` is drawn in the media box. `remove-icon` likewise takes an
//! element and falls back to the Slint default's mark, the Lucide `x` embedded
//! below — not the `×` text character, which carries its own side bearings in
//! its em box and so cannot be centred, and which a browser without the right
//! face cannot draw at all. The overlay spinner and the metadata's progress bar
//! are drawn on a `canvas` here, because a file installed on its own may not
//! have `spinner.rs` or `progress.rs` beside it to reach for.
//!
//! **Where this differs from the Slint component, deliberately:**
//!
//! - `icon-color` cannot repaint an element the caller built, so it is the
//!   media box's inherited text colour: a caller's text glyph takes it, while
//!   an image or an SVG keeps the colours the caller drew it in.
//! - A failed piece's media glyph is the foreground, not the destructive hue
//!   the Slint `icon-color` default names. The failed media fill *is* the
//!   destructive hue at a tenth over the page, so destructive ink on it is a
//!   hue on its own pale tint: 3.29:1 in the built-in light table. The failure
//!   still reads through the fill, the border and the description; the glyph
//!   only has to be legible.
//! - The remove control's `remove-variant` is not offered. The Slint default is
//!   the ghost variant, which is what the action control uses too, and that is
//!   the one treatment drawn here; a caller that wants another passes its own
//!   control through the action slot.
//! - `shimmer` and `shimmer-styled` have no equivalent: iced cannot paint a
//!   gradient through part of a text run and this crate has no animation driver.
//!   A loading title is drawn in the same colour and position the Slint
//!   `ShimmerText` keeps (its base colour is the title colour), and the loading
//!   state reads through the overlay ring and the progress bar.
//! - `truncate` and elision: iced 0.14 text has no ellipsis. A title never wraps
//!   — that is the Slint `wrap: no-wrap` — and one longer than the card is
//!   clipped by the card instead of elided.
//! - iced has no `min-width`, so a horizontal card fills the width it is given
//!   (`.width(..)` when the caller wants a fixed chip) where Slint holds 160px,
//!   and a vertical card is the column width unless the caller overrides it.
//! - The overlay ring's angle is `.phase(..)`, in `0.0..=1.0`, which the caller
//!   advances from a subscription; Slint takes it from `animation-tick()`.
//! - `progress` is 0–100, as in Slint. The pending state's border is the faded
//!   solid border the Slint source draws; iced has no dashed border.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::mouse;
use iced::widget::canvas::path::arc::Arc;
use iced::widget::canvas::{self, Canvas, Frame, Path};
use iced::widget::text::{LineHeight, Style as TextStyle, Wrapping};
use iced::widget::{button, column, container, row, scrollable, space, stack, svg, text};
use iced::{
    Background, Border, Color, Degrees, Element, Font, Length, Pixels, Radians, Rectangle,
    Renderer, Shadow, Theme,
};

use super::theme::{self, ControlSize, Tokens};

/// The lifecycle a piece reports. The names are the Slint `AttachmentStatus`'s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
    /// Ready to be sent: the border fades, and a preview is at full strength.
    Pending,
    /// On its way: the title is loading and the metadata's bar is drawn.
    Uploading,
    /// Being handled elsewhere: the title is loading.
    Processing,
    /// Rejected: the description, the border and a previewless media slot turn
    /// destructive.
    Failed,
    /// Ready. The default, and the resting look.
    #[default]
    Complete,
}

/// Which way the piece is laid out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis {
    /// Media, metadata and actions in one row. The default.
    #[default]
    Horizontal,
    /// The preview above the metadata, a square, with the actions over its
    /// upper trailing corner.
    Vertical,
}

/// The colours a piece resolves to, given the tokens in force.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colors {
    pub fill: Color,
    pub title: Color,
    pub icon: Color,
    pub media: Color,
    pub description: Color,
    pub border: Color,
}

/// The flags a piece's colours are resolved from, and an optional override for
/// each of them — the Slint properties of the same names. Sizes and overrides
/// are values captured when the element is built; the colours themselves are
/// resolved from the theme at render time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub status: Status,
    pub clickable: bool,
    pub radius: f32,
    pub fill: Option<Color>,
    pub title: Option<Color>,
    pub icon: Option<Color>,
    pub media: Option<Color>,
    pub description: Option<Color>,
    pub border: Option<Color>,
}

impl Style {
    pub fn new(status: Status) -> Self {
        Self {
            status,
            clickable: false,
            radius: theme::RADIUS_XL,
            fill: None,
            title: None,
            icon: None,
            media: None,
            description: None,
            border: None,
        }
    }

    /// Whether the card activates, which is what draws the hover surface.
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    /// The card's corner: the large radius at xsmall, the extra large one
    /// otherwise, as the Slint source computes it.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn fill(mut self, fill: Color) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn title(mut self, title: Color) -> Self {
        self.title = Some(title);
        self
    }

    pub fn icon(mut self, icon: Color) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn media(mut self, media: Color) -> Self {
        self.media = Some(media);
        self
    }

    pub fn description(mut self, description: Color) -> Self {
        self.description = Some(description);
        self
    }

    pub fn border(mut self, border: Color) -> Self {
        self.border = Some(border);
        self
    }

    /// The colours in force: the overrides where they are set, and the status
    /// treatment otherwise — a failed piece is destructive in its description,
    /// its border and, without a preview, its media slot; a pending one fades
    /// its border; everything else is the card surface. The media *glyph* is
    /// the foreground in every state: the failed media fill is the destructive
    /// hue at a tenth, and destructive ink on it is a hue on its own pale tint
    /// (3.29:1 in the built-in light table), so the failure reads through the
    /// fill, the border and the description instead.
    pub fn resolve(&self, tokens: Tokens) -> Colors {
        let failed = self.status == Status::Failed;
        Colors {
            fill: self.fill.unwrap_or(tokens.background),
            title: self.title.unwrap_or(tokens.foreground),
            icon: self.icon.unwrap_or(tokens.foreground),
            media: self.media.unwrap_or(if failed {
                theme::alpha(tokens.destructive, 0.1)
            } else {
                tokens.muted
            }),
            description: self.description.unwrap_or(if failed {
                theme::alpha(tokens.destructive, 0.8)
            } else {
                tokens.muted_foreground
            }),
            border: self.border.unwrap_or(if failed {
                theme::alpha(tokens.destructive, 0.3)
            } else if self.status == Status::Pending {
                theme::alpha(tokens.muted_foreground, 0.45)
            } else {
                tokens.border
            }),
        }
    }

    /// The card, in the shape iced's button style takes. A clickable card under
    /// the pointer is the muted surface at half strength; otherwise the card is
    /// its own fill, which is also what a card with no message draws while iced
    /// reports it disabled.
    pub fn card(&self, tokens: Tokens, status: button::Status) -> button::Style {
        let colors = self.resolve(tokens);
        let fill = if self.clickable
            && matches!(status, button::Status::Hovered | button::Status::Pressed)
        {
            theme::alpha(tokens.muted, 0.5)
        } else {
            colors.fill
        };

        button::Style {
            background: Some(Background::Color(fill)),
            text_color: colors.title,
            border: Border {
                color: colors.border,
                width: 1.0,
                radius: self.radius.into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    }
}

/// A file or media chip, built with the methods below.
pub struct Attachment<'a, Message> {
    name: String,
    description: String,
    progress: f32,
    status: Status,
    density: ControlSize,
    axis: Axis,
    icon: Option<Element<'a, Message>>,
    preview: Option<Element<'a, Message>>,
    show_media: bool,
    show_meta: bool,
    overlay: bool,
    phase: f32,
    media_box: f32,
    action: String,
    action_size: ControlSize,
    remove_icon: Option<Element<'a, Message>>,
    on_remove: Option<Message>,
    on_press: Option<Message>,
    on_action: Option<Message>,
    width: Option<Length>,
    style: Style,
}

impl<'a, Message> Attachment<'a, Message> {
    /// A piece whose title is `name`.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            progress: 100.0,
            status: Status::default(),
            density: ControlSize::default(),
            axis: Axis::default(),
            icon: None,
            preview: None,
            show_media: true,
            show_meta: true,
            overlay: false,
            phase: 0.0,
            media_box: 0.0,
            action: String::new(),
            action_size: ControlSize::XSmall,
            remove_icon: None,
            on_remove: None,
            on_press: None,
            on_action: None,
            width: None,
            style: Style::new(Status::default()),
        }
    }

    /// The second line — a type, a size, a reason. The Slint `size` property.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// How far along the piece is, in `0.0..=100.0`. Below 100 the metadata
    /// stack draws its bar.
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress;
        self
    }

    /// The lifecycle the piece reports, which draws its colours.
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self.style.status = status;
        self
    }

    /// The step the whole chip is scaled at.
    pub fn density(mut self, density: ControlSize) -> Self {
        self.density = density;
        self
    }

    /// The axis the chip is laid out along.
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// The caller's glyph, drawn centred in the media box whenever there is no
    /// preview. Slint's `icon`, whose name it cannot resolve here.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// The caller's preview, drawn in the media box in place of the glyph. An
    /// image element wants `content_fit(Cover)` and the box's size to fill it.
    pub fn preview(mut self, preview: impl Into<Element<'a, Message>>) -> Self {
        self.preview = Some(preview.into());
        self
    }

    /// Whether the media box is drawn at all. On by default.
    pub fn show_media(mut self, show_media: bool) -> Self {
        self.show_media = show_media;
        self
    }

    /// Whether the metadata stack is drawn at all. On by default.
    pub fn show_meta(mut self, show_meta: bool) -> Self {
        self.show_meta = show_meta;
        self
    }

    /// Draws the loading ring over the media box.
    pub fn overlay(mut self, overlay: bool) -> Self {
        self.overlay = overlay;
        self
    }

    /// Where the loading ring's arc starts, in `0.0..=1.0`. One full turn is
    /// one unit, and the caller advances it; Slint takes it from
    /// `animation-tick()`.
    pub fn phase(mut self, phase: f32) -> Self {
        self.phase = phase;
        self
    }

    /// The media box's side, in pixels. Zero, the default, takes the density's.
    pub fn media_box(mut self, media_box: f32) -> Self {
        self.media_box = media_box;
        self
    }

    /// A text action beside the remove control, drawn as the ghost button.
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = action.into();
        self
    }

    /// The size the action control is drawn at. Extra small by default.
    pub fn action_size(mut self, action_size: ControlSize) -> Self {
        self.action_size = action_size;
        self
    }

    /// The remove control's glyph. Without one the Slint default's shape, the
    /// `×` character, is drawn.
    pub fn remove_icon(mut self, remove_icon: impl Into<Element<'a, Message>>) -> Self {
        self.remove_icon = Some(remove_icon.into());
        self
    }

    /// Draws the remove control and sends this message when it is pressed.
    pub fn on_remove(mut self, message: Message) -> Self {
        self.on_remove = Some(message);
        self
    }

    /// Makes the whole card activate, and draw the muted surface under the
    /// pointer. Without a message the card is not clickable.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sends this message when the action control is pressed. Without one the
    /// action is drawn disabled.
    pub fn on_action(mut self, message: Message) -> Self {
        self.on_action = Some(message);
        self
    }

    /// The card's width. A horizontal chip fills the width it is given and a
    /// vertical one is the column width, unless this overrides it.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Replaces the resolved colours outright.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn fill(mut self, fill: Color) -> Self {
        self.style = self.style.fill(fill);
        self
    }

    pub fn title_color(mut self, title: Color) -> Self {
        self.style = self.style.title(title);
        self
    }

    pub fn icon_color(mut self, icon: Color) -> Self {
        self.style = self.style.icon(icon);
        self
    }

    pub fn media_background(mut self, media: Color) -> Self {
        self.style = self.style.media(media);
        self
    }

    pub fn description_color(mut self, description: Color) -> Self {
        self.style = self.style.description(description);
        self
    }

    pub fn border_color(mut self, border: Color) -> Self {
        self.style = self.style.border(border);
        self
    }
}

/// The Slint source's per-density metrics.
impl<'a, Message> Attachment<'a, Message> {
    fn failed(&self) -> bool {
        self.status == Status::Failed
    }

    /// A preview is dimmed in every state but pending and complete.
    fn dim_preview(&self) -> bool {
        self.preview.is_some() && !matches!(self.status, Status::Pending | Status::Complete)
    }

    fn pad(&self) -> f32 {
        match self.density {
            ControlSize::XSmall => 4.0,
            ControlSize::Small => 6.0,
            ControlSize::Large => 12.0,
            ControlSize::Medium => 8.0,
        }
    }

    fn gap(&self) -> f32 {
        match self.density {
            ControlSize::XSmall => 6.0,
            ControlSize::Small => 10.0,
            ControlSize::Large => 12.0,
            ControlSize::Medium => 8.0,
        }
    }

    fn media_side(&self) -> f32 {
        if self.media_box > 0.0 {
            self.media_box
        } else {
            match self.density {
                ControlSize::XSmall => 28.0,
                ControlSize::Small => 32.0,
                ControlSize::Large => 48.0,
                ControlSize::Medium => 40.0,
            }
        }
    }

    fn title_size(&self) -> f32 {
        match self.density {
            ControlSize::XSmall | ControlSize::Small => theme::FONT_SIZE_XS,
            ControlSize::Large => theme::FONT_SIZE_MD,
            ControlSize::Medium => theme::FONT_SIZE_SM,
        }
    }

    fn radius(&self) -> f32 {
        if self.density == ControlSize::XSmall {
            theme::RADIUS_LG
        } else {
            theme::RADIUS_XL
        }
    }

    fn has_actions(&self) -> bool {
        self.on_remove.is_some() || !self.action.is_empty()
    }

    /// The vertical card's width: 7.5rem with metadata and 6rem without.
    fn column_width(&self) -> f32 {
        if self.show_meta { 120.0 } else { 96.0 }
    }

    /// The height the metadata stack needs, in the terms the Slint source
    /// counts it: a title line, a description line and the bar's gap.
    fn label_height(&self) -> f32 {
        if !self.show_meta {
            return 0.0;
        }
        let title_line = self.title_size() * 1.25;
        let description_line = theme::FONT_SIZE_XS * 1.25;

        let title = if self.name.is_empty() {
            0.0
        } else {
            title_line
        };
        let description = if self.description.is_empty() {
            0.0
        } else {
            description_line + if self.name.is_empty() { 0.0 } else { 2.0 }
        };
        let bar = if self.progress < 100.0 { 6.0 } else { 0.0 };

        title + description + bar
    }

    fn actions_height(&self) -> f32 {
        if !self.has_actions() {
            0.0
        } else if self.action.is_empty() {
            20.0
        } else {
            24.0
        }
    }

    fn card_height(&self) -> f32 {
        if self.axis == Axis::Vertical {
            let media = if self.show_media {
                self.column_width()
            } else {
                0.0
            };
            let meta = if self.show_meta {
                self.label_height() + 16.0
            } else {
                0.0
            };
            media + meta
        } else {
            self.pad() * 2.0
                + (if self.show_media {
                    self.media_side()
                } else {
                    0.0
                })
                .max(if self.show_meta {
                    self.label_height()
                } else {
                    0.0
                })
                .max(self.actions_height())
        }
    }

    fn card_width(&self) -> Length {
        self.width.unwrap_or(if self.axis == Axis::Vertical {
            Length::Fixed(self.column_width())
        } else {
            Length::Fill
        })
    }
}

impl<'a, Message: Clone + 'a> Attachment<'a, Message> {
    fn render(mut self) -> Element<'a, Message> {
        let vertical = self.axis == Axis::Vertical;
        let style = self.style.radius(self.radius());
        let card_height = self.card_height();
        let width = self.card_width();
        let pad = self.pad();
        let gap = self.gap();
        let media_side = self.media_side();
        let title_size = self.title_size();
        let show_media = self.show_media;
        let show_meta = self.show_meta;
        let dimmed = self.dim_preview();
        let overlay = self.overlay;
        let phase = self.phase;
        let on_press = self.on_press.take();

        let name = self.name.clone();
        let description = self.description.clone();
        let progress = self.progress;

        let preview = self.preview.take();
        let icon = self.icon.take();
        let remove_icon = self.remove_icon.take();
        let action = self.action.clone();
        let action_size = self.action_size;
        let on_action = self.on_action.take();
        let on_remove = self.on_remove.take();

        // The action row, which the horizontal layout puts in the row and the
        // vertical one over the media box.
        let actions: Option<Element<'a, Message>> = if self.has_actions() {
            let remove = if on_remove.is_some() {
                Some(remove_icon.unwrap_or_else(|| {
                    // The mark is the embedded `x`, not the `×` character: a
                    // text glyph is laid out at its box's leading edge, so it
                    // sits a pixel from the left and a third of the way down
                    // (measured on the unfixed control: ink 4x5 in a 20x21 box,
                    // 1/6/15/10 from its edges). `ghost_icon` centres this on
                    // both axes. The ink is resolved inside the closure because
                    // an SVG tinted once at build time keeps the light theme's
                    // colour in the dark.
                    svg(svg::Handle::from_memory(CLOSE_XML))
                        .width(CLOSE_SIZE)
                        .height(CLOSE_SIZE)
                        .style(|theme: &Theme, _status| svg::Style {
                            color: Some(
                                ghost_style(Tokens::of(theme), button::Status::Active).text_color,
                            ),
                        })
                        .into()
                }))
            } else {
                None
            };
            Some(action_row(
                action,
                action_size,
                on_action,
                remove,
                on_remove,
            ))
        } else {
            None
        };

        let meta: Option<Element<'a, Message>> = if show_meta {
            let inset = if vertical { 8.0 } else { 0.0 };
            Some(meta_stack(
                style,
                name,
                description,
                progress,
                title_size,
                inset,
            ))
        } else {
            None
        };

        let content: Element<'a, Message> = if vertical {
            let media = if show_media {
                Some(media_box(
                    style,
                    Length::Fill,
                    Length::Fixed(self.column_width()),
                    preview,
                    icon,
                    dimmed,
                    overlay,
                    phase,
                    actions,
                ))
            } else {
                None
            };
            let mut column = column![].width(Length::Fill).height(Length::Fill);
            if let Some(media) = media {
                column = column.push(media);
            }
            if let Some(meta) = meta {
                column = column.push(meta);
            }
            column.into()
        } else {
            let media = if show_media {
                Some(media_box(
                    style,
                    Length::Fixed(media_side),
                    Length::Fixed(media_side),
                    preview,
                    icon,
                    dimmed,
                    overlay,
                    phase,
                    None,
                ))
            } else {
                None
            };
            let mut content = row![]
                .spacing(gap)
                .align_y(iced::Center)
                .width(Length::Fill)
                .height(Length::Fill);
            if let Some(media) = media {
                content = content.push(media);
            }
            if let Some(meta) = meta {
                content = content.push(meta);
            }
            if let Some(actions) = actions {
                content = content.push(actions);
            }
            container(content)
                .padding(pad)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        };

        let mut card = button(content)
            .padding(0.0)
            .width(width)
            .height(Length::Fixed(card_height))
            .clip(true)
            .style(move |theme: &Theme, status| style.card(Tokens::of(theme), status));

        if let Some(message) = on_press {
            card = card.on_press(message);
        }

        card.into()
    }
}

/// The metadata stack: the title, the description and the bar.
fn meta_stack<'a, Message: 'a>(
    style: Style,
    name: String,
    description: String,
    progress: f32,
    title_size: f32,
    inset: f32,
) -> Element<'a, Message> {
    let title_line = title_size * 1.25;
    let description_line = theme::FONT_SIZE_XS * 1.25;
    let mut meta = column![].spacing(2.0).width(Length::Fill);

    if !name.is_empty() {
        meta = meta.push(
            text(name)
                .size(title_size)
                .line_height(LineHeight::Absolute(Pixels(title_line)))
                .font(Font {
                    weight: iced::font::Weight::Medium,
                    ..Font::DEFAULT
                })
                .wrapping(Wrapping::None)
                .width(Length::Fill)
                .style(move |theme: &Theme| TextStyle {
                    color: Some(style.resolve(Tokens::of(theme)).title),
                }),
        );
    }

    if !description.is_empty() {
        meta = meta.push(
            text(description)
                .size(theme::FONT_SIZE_XS)
                .line_height(LineHeight::Absolute(Pixels(description_line)))
                .wrapping(Wrapping::None)
                .width(Length::Fill)
                .style(move |theme: &Theme| TextStyle {
                    color: Some(style.resolve(Tokens::of(theme)).description),
                }),
        );
    }

    if progress < 100.0 {
        meta = meta.push(
            Canvas::new(Bar {
                value: (progress / 100.0).clamp(0.0, 1.0),
            })
            .width(Length::Fill)
            .height(Length::Fixed(PROGRESS_GIRTH)),
        );
    }

    container(meta).padding(inset).width(Length::Fill).into()
}

/// The media slot: the caller's preview or glyph, the fade over a preview that
/// is not at full strength, the loading ring, and — vertically — the actions
/// over the upper trailing corner.
fn media_box<'a, Message: 'a>(
    style: Style,
    width: Length,
    height: Length,
    preview: Option<Element<'a, Message>>,
    icon: Option<Element<'a, Message>>,
    dimmed: bool,
    overlay: bool,
    phase: f32,
    actions: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut layers: Vec<Element<'a, Message>> = Vec::new();
    let previewed = preview.is_some();

    // A failed piece's destructive media surface and glyph are for the slot
    // that has no preview of its own to carry the failure; a preview keeps the
    // muted box behind it.
    let media_style = if previewed {
        Style {
            status: Status::Complete,
            ..style
        }
    } else {
        style
    };

    if let Some(media) = preview.or(icon) {
        layers.push(
            container(media)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::Center)
                .align_y(iced::Center)
                .into(),
        );
    }

    // Slint draws a non-resting preview at 60%, which is the preview at 60%
    // over the media fill; a wash of that fill at 40% composites to the same
    // thing without owning the caller's element.
    if dimmed {
        layers.push(
            container(space())
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |theme: &Theme| container::Style {
                    background: Some(
                        theme::alpha(media_style.resolve(Tokens::of(theme)).media, 0.4).into(),
                    ),
                    ..container::Style::default()
                })
                .into(),
        );
    }

    if overlay {
        layers.push(
            container(
                Canvas::new(Ring { progress: phase })
                    .width(Length::Fixed(SPINNER))
                    .height(Length::Fixed(SPINNER)),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::Center)
            .align_y(iced::Center)
            .into(),
        );
    }

    if let Some(actions) = actions {
        layers.push(
            container(actions)
                .padding(8.0)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::Right)
                .align_y(iced::Top)
                .into(),
        );
    }

    let content: Element<'a, Message> = match layers.len() {
        0 => space().into(),
        1 => layers.remove(0),
        _ => stack(layers)
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
    };

    container(content)
        .width(width)
        .height(height)
        .clip(true)
        .style(move |theme: &Theme| {
            let colors = media_style.resolve(Tokens::of(theme));
            container::Style {
                background: Some(colors.media.into()),
                // The glyph's colour, which a caller's text element inherits.
                text_color: Some(colors.icon),
                border: Border {
                    radius: theme::RADIUS_MD.into(),
                    ..Border::default()
                },
                ..container::Style::default()
            }
        })
        .into()
}

/// The action slot: the text action, then the remove control, four pixels
/// apart and centred.
fn action_row<'a, Message: Clone + 'a>(
    action: String,
    action_size: ControlSize,
    on_action: Option<Message>,
    remove: Option<Element<'a, Message>>,
    on_remove: Option<Message>,
) -> Element<'a, Message> {
    let mut actions = row![].spacing(4.0).align_y(iced::Center);

    if !action.is_empty() {
        actions = actions.push(ghost_label(action, action_size, on_action));
    }
    if let Some(remove) = remove {
        actions = actions.push(ghost_icon(remove, on_remove));
    }

    actions.into()
}

/// A labelled ghost control: the shape the Slint `Button` draws for an
/// attachment's action, which is the variant's default.
fn ghost_label<'a, Message: Clone + 'a>(
    label: String,
    size: ControlSize,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let content = text(label).size(size.control_font_size()).font(Font {
        weight: iced::font::Weight::Medium,
        ..Font::DEFAULT
    });

    let mut control = button(content)
        .height(Length::Fixed(control_height(size)))
        .padding([0.0, control_pad(size)])
        .style(|theme: &Theme, status| ghost_style(Tokens::of(theme), status));

    if let Some(message) = on_press {
        control = control.on_press(message);
    }

    control.into()
}

/// An icon-only ghost control: a square the control's own height, the minimum
/// width the Slint `Button` keeps for a control with no label.
fn ghost_icon<'a, Message: Clone + 'a>(
    icon: Element<'a, Message>,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let side = control_height(ControlSize::XSmall);

    // The mark fills the control on both axes, so it is centred by construction
    // rather than left at the top-left iced's button would otherwise place it.
    let mut control = button(
        container(icon)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .width(Length::Fixed(side))
    .height(Length::Fixed(side))
    .padding(0.0)
    .style(|theme: &Theme, status| ghost_style(Tokens::of(theme), status));

    if let Some(message) = on_press {
        control = control.on_press(message);
    }

    control.into()
}

/// The ghost variant's fills: nothing of its own, the secondary foreground for
/// ink, the accent under the pointer and the input surface mixed toward the
/// page while held. A control with no message is the disabled treatment.
fn ghost_style(tokens: Tokens, status: button::Status) -> button::Style {
    let (background, ink) = match status {
        button::Status::Hovered => (
            if tokens.dark {
                theme::alpha(tokens.accent, 0.5)
            } else {
                tokens.accent
            },
            tokens.secondary_foreground,
        ),
        button::Status::Pressed => (
            theme::mix(tokens.input, tokens.background, 0.3),
            tokens.secondary_foreground,
        ),
        button::Status::Disabled => (Color::TRANSPARENT, tokens.muted_foreground),
        button::Status::Active => (Color::TRANSPARENT, tokens.secondary_foreground),
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: ink,
        border: Border {
            radius: theme::RADIUS_MD.into(),
            ..Border::default()
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

/// The height a control takes: 20 / 24 / 32 / 40 pixels.
fn control_height(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 20.0,
        ControlSize::Small => 24.0,
        ControlSize::Large => 40.0,
        ControlSize::Medium => 32.0,
    }
}

/// The horizontal padding a labelled control takes.
fn control_pad(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 4.0,
        ControlSize::Small => 8.0,
        ControlSize::Large => 16.0,
        ControlSize::Medium => 10.0,
    }
}

/// The bar's height, the Slint `Progress` at `ControlSize.xsmall`.
const PROGRESS_GIRTH: f32 = 4.0;

/// The loading ring's diameter, the Slint `Spinner` at `ControlSize.small`.
const SPINNER: f32 = 14.0;

/// The remove mark's size, in the 20px extra-small ghost control.
///
/// The Slint authority is a ghost icon-only `Button` at `xsmall`, which draws
/// its 12px `x` centred in a 20px square; this crate's dialog, notification,
/// sheet and alert close controls all draw this same stroke at 16px, and the
/// alert's is a 16px box the 16px mark fills. 16px is the size here.
const CLOSE_SIZE: f32 = 16.0;

/// The Lucide `x` from `crates/slint-component/ui/icons/x.svg` (Lucide, ISC):
/// the default `remove-icon`'s mark, in the same one-line form the crate's
/// other embedded icons use.
const CLOSE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>"#;

/// The metadata's bar: the primary colour at a fifth over the track, and the
/// fill up to the value.
struct Bar {
    value: f32,
}

impl<Message> canvas::Program<Message> for Bar {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let tokens = Tokens::of(theme);
        let mut frame = Frame::new(renderer, bounds.size());
        let radius = frame.height() / 2.0;

        frame.fill_rectangle(
            iced::Point::ORIGIN,
            frame.size(),
            theme::alpha(tokens.primary, 0.2),
        );

        let width = frame.width() * self.value.clamp(0.0, 1.0);

        if width > 0.0 {
            let path = Path::rounded_rectangle(
                iced::Point::new(0.0, 0.0),
                iced::Size::new(width, frame.height()),
                radius.into(),
            );
            frame.fill(&path, tokens.primary);
        }

        vec![frame.into_geometry()]
    }
}

/// The loading ring: the open arc the Slint `Spinner` draws, its start taken
/// from the caller's phase.
struct Ring {
    progress: f32,
}

impl<Message> canvas::Program<Message> for Ring {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let tokens = Tokens::of(theme);
        let mut frame = Frame::new(renderer, bounds.size());
        let diameter = frame.width().min(frame.height());
        let stroke = (diameter * 0.15).clamp(1.0, 3.0);
        let center = frame.center();
        let radius = ((diameter - stroke) / 2.0).max(0.0);
        let start = self.progress.fract() * 360.0 - 90.0;
        let end = start + 270.0;

        let arc = Path::new(|builder| {
            builder.arc(Arc {
                center,
                radius,
                start_angle: Radians::from(Degrees::from(start)),
                end_angle: Radians::from(Degrees::from(end)),
            });
        });

        frame.stroke(
            &arc,
            canvas::Stroke::default()
                .with_color(tokens.foreground)
                .with_width(stroke)
                .with_line_cap(canvas::LineCap::Round),
        );

        vec![frame.into_geometry()]
    }
}

/// A row of attachments that scrolls when they do not fit. The Slint
/// `AttachmentGroup`: twelve pixels between the pieces, aligned to the start,
/// and the row's own height.
pub struct Group<'a, Message> {
    children: Vec<Element<'a, Message>>,
}

impl<'a, Message> Group<'a, Message> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    pub fn extend(mut self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        self.children.extend(children);
        self
    }
}

impl<'a, Message> Default for Group<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: 'a> From<Group<'a, Message>> for Element<'a, Message> {
    fn from(group: Group<'a, Message>) -> Self {
        scrollable(row(group.children).spacing(12.0).align_y(iced::Center))
            .horizontal()
            .width(Length::Fill)
            .into()
    }
}

impl<'a, Message: Clone + 'a> From<Attachment<'a, Message>> for Element<'a, Message> {
    fn from(control: Attachment<'a, Message>) -> Self {
        control.render()
    }
}
