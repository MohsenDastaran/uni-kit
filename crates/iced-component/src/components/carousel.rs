//! `carousel` — a paginated viewer for a set of related slides. Mirrors GPUI
//! Component's `Carousel` and the Slint version beside it: a viewport that shows
//! one or more slides, previous and next controls, optional pagination, an
//! optional looping wrap, and a horizontal or vertical layout.
//!
//! The application owns the index. The carousel is given the slide it is on and
//! reports the one a control asks for through [`Carousel::on_select`]; it never
//! moves itself, which is the same controlled shape as [`List`](super::list::List)
//! and [`Pagination`](super::pagination::Pagination) beside it.
//!
//! # What iced cannot draw, named rather than faked
//!
//! - **There is no autoplay.** `iced::time::every` exists only with the
//!   `tokio`/`smol` runtime backends, and this crate resolves `thread-pool`, so a
//!   component cannot own a timer. GPUI's `CarouselState` advances on an interval
//!   and the Slint file can be driven from outside; here the interval has to
//!   belong to the application, which is the one place a runtime always exists.
//! - **There is no scrolling, snapping track.** iced's scroll area moves its
//!   content but has no snap point, and the Slint component does not scroll at
//!   all: it draws the window of `visible` slides from the current origin. This
//!   port is that window, which makes the honest description **a paginated
//!   viewer** rather than a track you drag. No pointer or trackpad gesture, no
//!   momentum, and no partial slide at the edge.
//! - **There is no accessibility tree.** iced 0.14 exposes no region role or
//!   position-in-set, and no label on the controls; the Slint `FocusScope`'s
//!   keyboard navigation is an application subscription, as in `stepper`'s demo.
//! - **A slide is the caller's element.** Slint names an icon and draws a
//!   numbered card; GPUI's `CarouselItem` wraps children. Here [`Carousel::item`]
//!   takes an [`Element`], so a slide can be a label, a picture
//!   ([`ImageView`](super::image::ImageView)) or anything else, and there is no
//!   `CarouselContent`/`CarouselItem` pair to keep in step with a count.
//! - **There is no focus ring or Tab stop.** iced 0.14 gives a bare button no
//!   focus ring and no Space or Enter activation, so the keyboard path and any
//!   ring belong to the application.
//!
//! The arrows and the pagination dots are [`button::handle`](super::button::handle)
//! — iced's own button with no padding of its own — so pressing, hovering and
//! "cannot be pressed" are the shared component's, not a second copy. Every
//! colour is read from [`Tokens`] inside a style closure, so a theme switch
//! repaints the carousel: resolving a surface before the frame draws is what once
//! made a dark panel white.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::button;
use iced::widget::svg;
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, Shadow, Theme};

use super::button::handle;
use super::theme::{self, ControlSize, Tokens};

/// The gap between two slides, `spacing: 12px` in the Slint component.
const SLIDE_GAP: f32 = 12.0;
/// The gap around the viewport and the pagination, `spacing: 16px` there.
const CONTROL_GAP: f32 = 16.0;
/// The gap between two pagination dots, `spacing: 8px` there.
const DOT_GAP: f32 = 8.0;
/// A labelled step's horizontal inset, which is what makes it a pill.
const LABEL_PADDING: f32 = 14.0;

/// A slide's card: the page's own fill inside a one-pixel border at the extra
/// large radius, the shape the Slint `CarouselSlide` and GPUI's slide draw. The
/// content is centred, so a caller's label reads as the slide.
fn slide<'a, Message: 'a>(
    content: Element<'a, Message>,
    width: f32,
    height: f32,
) -> Element<'a, Message> {
    // The tokens are resolved in the closure, never at build time.
    container(content)
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(|theme: &Theme| {
            let tokens = Tokens::of(theme);
            container::Style {
                background: Some(tokens.background.into()),
                text_color: Some(tokens.foreground),
                border: Border {
                    color: tokens.border,
                    width: 1.0,
                    radius: theme::RADIUS_XL.into(),
                },
                ..container::Style::default()
            }
        })
        .into()
}

/// The chevron for a direction, from the Lucide artwork the Slint icons folder
/// carries (`chevron-left`, `chevron-right`, `chevron-up`, `chevron-down`). The
/// colour is resolved in the closure, so the mark follows the theme.
fn chevron<'a, Message: 'a>(
    orientation: Orientation,
    previous: bool,
    disabled: bool,
    pixels: f32,
) -> Element<'a, Message> {
    let markup = match (orientation, previous) {
        (Orientation::Horizontal, true) => CHEVRON_LEFT,
        (Orientation::Horizontal, false) => CHEVRON_RIGHT,
        (Orientation::Vertical, true) => CHEVRON_UP,
        (Orientation::Vertical, false) => CHEVRON_DOWN,
    };
    svg(svg::Handle::from_memory(markup))
        .width(Length::Fixed(pixels))
        .height(Length::Fixed(pixels))
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(if disabled {
                Tokens::of(theme).muted_foreground
            } else {
                Tokens::of(theme).foreground
            }),
        })
        .into()
}

/// One previous or next control: a circular outlined button, or a pill when the
/// caller gives it a label. Slint's `CarouselStep` in both shapes.
fn step<'a, Message: Clone + 'a>(
    orientation: Orientation,
    previous: bool,
    size: ControlSize,
    label: &str,
    message: Option<Message>,
) -> Element<'a, Message> {
    let side = size.control_height();
    let disabled = message.is_none();
    let icon_only = label.is_empty();

    let content: Element<'a, Message> =
        if icon_only {
            container(chevron(orientation, previous, disabled, size.icon_size()))
                .width(Length::Fixed(side))
                .height(Length::Fixed(side))
                .align_x(Alignment::Center)
                .align_y(Alignment::Center)
                .into()
        } else {
            container(text(label.to_string()).size(theme::FONT_SIZE_SM).style(
                move |theme: &Theme| text::Style {
                    color: Some(if disabled {
                        Tokens::of(theme).muted_foreground
                    } else {
                        Tokens::of(theme).foreground
                    }),
                },
            ))
            .height(Length::Fixed(side))
            .padding([0.0, LABEL_PADDING])
            .align_y(Alignment::Center)
            .into()
        };

    handle(content, message, move |theme: &Theme, status| {
        let tokens = Tokens::of(theme);
        let disabled = matches!(status, button::Status::Disabled);
        let background =
            if !disabled && matches!(status, button::Status::Hovered | button::Status::Pressed) {
                tokens.accent
            } else {
                Color::TRANSPARENT
            };
        button::Style {
            background: (background.a > 0.0).then_some(background.into()),
            text_color: if disabled {
                tokens.muted_foreground
            } else {
                tokens.foreground
            },
            border: Border {
                color: if disabled {
                    theme::alpha(tokens.border, 0.4)
                } else {
                    tokens.border
                },
                width: 1.0,
                radius: (side / 2.0).into(),
            },
            shadow: Shadow::default(),
            snap: false,
        }
    })
}

/// One pagination dot. The selected page is the primary fill and the rest sit a
/// fifth of the way from the page to the foreground, which is an indicator in
/// either appearance. It reports the page it stands for through the same
/// `on_select` the arrows use.
fn dot<'a, Message: Clone + 'a>(
    selected: bool,
    size: ControlSize,
    message: Option<Message>,
) -> Element<'a, Message> {
    let diameter = match size {
        ControlSize::XSmall => 6.0,
        ControlSize::Small => 8.0,
        ControlSize::Large => 12.0,
        ControlSize::Medium => 10.0,
    };

    let content: Element<'a, Message> = container(Space::new())
        .width(Length::Fixed(diameter))
        .height(Length::Fixed(diameter))
        .into();

    handle(content, message, move |theme: &Theme, status| {
        let tokens = Tokens::of(theme);
        let background = if selected {
            tokens.primary
        } else if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            theme::alpha(tokens.foreground, 0.4)
        } else {
            theme::alpha(tokens.foreground, 0.2)
        };
        button::Style {
            background: Some(background.into()),
            text_color: tokens.foreground,
            border: Border {
                radius: (diameter / 2.0).into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
            snap: false,
        }
    })
}

/// How a carousel is laid out: the browsing axis, and the axis its previous and
/// next controls sit on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    /// Slides in a row, previous on the left and next on the right. The default,
    /// as in Slint.
    #[default]
    Horizontal,
    /// Slides in a column, previous above and next below.
    Vertical,
}

/// A carousel, built with the methods below.
pub struct Carousel<'a, Message> {
    items: Vec<Element<'a, Message>>,
    index: usize,
    visible: usize,
    orientation: Orientation,
    looping: bool,
    controls: bool,
    pagination: bool,
    size: ControlSize,
    previous_label: String,
    next_label: String,
    frame: f32,
    cross: f32,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message> Carousel<'a, Message> {
    /// A carousel over the slides given, showing one at a time, horizontal, not
    /// looping, with previous and next and no pagination — the Slint component's
    /// own defaults.
    pub fn new(items: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        Self {
            items: items.into_iter().collect(),
            index: 0,
            visible: 1,
            orientation: Orientation::default(),
            looping: false,
            controls: true,
            pagination: false,
            size: ControlSize::default(),
            previous_label: String::new(),
            next_label: String::new(),
            frame: 320.0,
            cross: 256.0,
            on_select: None,
        }
    }

    /// Appends one slide. A slide is any element: a label, a picture, a card.
    pub fn item(mut self, item: impl Into<Element<'a, Message>>) -> Self {
        self.items.push(item.into());
        self
    }

    /// The slide the carousel is on. Clamped to the last real origin when it is
    /// drawn, unless the carousel loops, so the call order of `index` and `item`
    /// never matters.
    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    /// How many slides the viewport shows at once. Slint's `visible-items`.
    pub fn visible(mut self, visible: usize) -> Self {
        self.visible = visible.max(1);
        self
    }

    /// The browsing axis.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Whether navigation wraps from the last slide to the first. Slint's
    /// `looping`.
    pub fn looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    /// Whether the previous and next controls are drawn. The Slint `controls`.
    pub fn controls(mut self, controls: bool) -> Self {
        self.controls = controls;
        self
    }

    /// Whether the pagination dots are drawn under the viewport. The Slint
    /// `pagination`, off by default.
    pub fn pagination(mut self, pagination: bool) -> Self {
        self.pagination = pagination;
        self
    }

    /// The size the arrows and dots take, `ControlSize::Medium` by default.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// A label that replaces the previous chevron with a pill, as Slint's
    /// `previous-text` does. An empty label, the default, keeps the chevron.
    pub fn previous_label(mut self, label: impl Into<String>) -> Self {
        self.previous_label = label.into();
        self
    }

    /// A label that replaces the next chevron with a pill. Slint's `next-text`.
    pub fn next_label(mut self, label: impl Into<String>) -> Self {
        self.next_label = label.into();
        self
    }

    /// The viewport's size along the browsing axis, `frame: 320px` in the Slint
    /// component. A single slide is that square; several share it through the
    /// gap.
    pub fn frame(mut self, frame: f32) -> Self {
        self.frame = frame;
        self
    }

    /// The viewport's size on the other axis. It is the slide width of a vertical
    /// carousel, `cross: 256px` in the Slint component; a horizontal carousel is
    /// square along its axis and does not read it.
    pub fn cross(mut self, cross: f32) -> Self {
        self.cross = cross;
        self
    }

    /// A slide was asked for. The carousel reports the requested index and
    /// leaves the current one to the caller, which is what makes the state on
    /// screen the caller's own. Every control — arrows and dots alike — reports
    /// through this one callback, so there is no second navigation path to keep
    /// in step. Without a handler nothing is pressable, which is iced's "cannot
    /// be pressed".
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }
}

impl<'a, Message> Default for Carousel<'a, Message> {
    fn default() -> Self {
        Self::new(std::iter::empty())
    }
}

impl<'a, Message> From<Carousel<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Carousel<'a, Message>) -> Self {
        let Carousel {
            items,
            index,
            visible,
            orientation,
            looping,
            controls,
            pagination,
            size,
            previous_label,
            next_label,
            frame,
            cross,
            on_select,
        } = control;

        let count = items.len();
        if count == 0 {
            return Space::new().into();
        }

        // A window of at most the slide count, so an over-wide `visible` neither
        // repeats a slide nor divides the frame by nothing.
        let span = visible.clamp(1, count);
        let last = count.saturating_sub(span);
        // Looping wraps the origin; without it the index is clamped to the last
        // window that still shows a whole slide, which is the Slint `origin`.
        let origin = if looping {
            index % count
        } else {
            index.min(last)
        };

        let emit = |next: usize| on_select.as_ref().map(|on_select| on_select(next));

        let card_width = if orientation == Orientation::Vertical {
            cross
        } else if span <= 1 {
            frame
        } else {
            (frame - SLIDE_GAP * (span as f32 - 1.0)) / span as f32
        };
        let card_height = if orientation == Orientation::Vertical {
            (frame - SLIDE_GAP * (span as f32 - 1.0)) / span as f32
        } else {
            card_width
        };

        // One element per slot of the window. A looping carousel wraps through
        // the caller's slides, which is what Slint's numbered labels read as.
        let mut slots: Vec<Option<Element<'a, Message>>> = items.into_iter().map(Some).collect();
        let mut cells: Vec<Element<'a, Message>> = Vec::with_capacity(span);
        for slot in 0..span {
            let at = (origin + slot) % count;
            if let Some(content) = slots[at].take() {
                cells.push(slide(content, card_width, card_height));
            }
        }
        let track: Element<'a, Message> = match orientation {
            Orientation::Horizontal => {
                let mut track = row![].spacing(SLIDE_GAP);
                for cell in cells {
                    track = track.push(cell);
                }
                track.into()
            }
            Orientation::Vertical => {
                let mut track = column![].spacing(SLIDE_GAP);
                for cell in cells {
                    track = track.push(cell);
                }
                track.into()
            }
        };

        let previous_enabled = if looping { count > 1 } else { origin > 0 };
        let next_enabled = if looping { count > 1 } else { origin < last };
        let previous_at = if looping {
            (origin + count - 1) % count
        } else {
            origin.saturating_sub(1)
        };
        let next_at = if looping {
            (origin + 1) % count
        } else {
            (origin + 1).min(last)
        };

        let previous_message = previous_enabled.then(|| emit(previous_at)).flatten();
        let next_message = next_enabled.then(|| emit(next_at)).flatten();

        let body: Element<'a, Message> = if controls {
            let previous = step(orientation, true, size, &previous_label, previous_message);
            let next = step(orientation, false, size, &next_label, next_message);
            match orientation {
                Orientation::Horizontal => row![previous, track, next]
                    .spacing(CONTROL_GAP)
                    .align_y(Alignment::Center)
                    .into(),
                Orientation::Vertical => column![previous, track, next]
                    .spacing(CONTROL_GAP)
                    .align_x(Alignment::Center)
                    .into(),
            }
        } else {
            track.into()
        };

        // The root fills its parent so the viewport is centred in the space the
        // caller gives the carousel: a shrinking column would sit at the leading
        // edge however the caller centred its own box.
        let mut column = column![body]
            .width(Length::Fill)
            .spacing(CONTROL_GAP)
            .align_x(Alignment::Center);

        if pagination && count > 1 {
            let mut dots = row![].spacing(DOT_GAP).align_y(Alignment::Center);
            for page in 0..count {
                let selected = page == origin;
                dots = dots.push(dot(selected, size, emit(page)));
            }
            column = column.push(dots);
        }

        column.into()
    }
}

/// `chevron-left`, from `crates/slint-component/ui/icons/` (Lucide, ISC).
const CHEVRON_LEFT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>"#;

/// `chevron-right`, from the same set.
const CHEVRON_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#;

/// `chevron-up`, from the same set.
const CHEVRON_UP: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 15-6-6-6 6"/></svg>"#;

/// `chevron-down`, from the same set.
const CHEVRON_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#;
