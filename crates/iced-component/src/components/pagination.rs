//! Page controls. Mirrors GPUI Component's `Pagination` and the Slint version
//! beside it: a previous button, a run of page numbers five wide that always
//! contains the current page, ellipses when pages outside the run are hidden, the
//! first and last pages outside it, and a next button.
//!
//! The application owns the page: the component is given the page and the total
//! and reports the page a click asks for through [`Pagination::on_change`]. It
//! never moves its own selection, and it clamps whatever it is given, so an
//! out-of-range page draws the nearest real one.
//!
//! **The row sits in a [`Scrollable`](super::scrollable::Scrollable).** A run at
//! the large size is wide, and the Slint component's row simply clips when its
//! parent is narrower; iced gives the same row a horizontal scroll area, so a
//! narrow container scrolls the controls instead of losing them.
//!
//! The two navigation buttons are the shared [`Button`](super::button::Button) in
//! its ghost variant, and the current page is that button's primary variant —
//! exactly the `ButtonVariant.primary : ghost` ternary the Slint file writes.
//! iced ships no icon set, so the two chevrons are drawn from the same Lucide
//! artwork the Slint icons folder carries; a caller that wants its own arrows can
//! still compose the buttons itself.
//!
//! # What iced cannot draw, named rather than faked
//!
//! - **The ellipsis is text, not a menu.** GPUI opens a dropdown listing the
//!   hidden pages and caps that list at a hundred; the Slint component draws a
//!   `…` and reaches the hidden pages through the first and last buttons, which
//!   is what this port draws.
//! - **The run is fixed at five.** GPUI's `visible_pages` is a builder; the Slint
//!   `window-size` is a constant, and this port follows it.
//! - **There is no disabled property.** GPUI has one flag for the whole control;
//!   Slint disables only the arrow with nowhere to go, and so does this.
//! - **There is no tooltip or accessible label.** iced has no tooltip on a bare
//!   button, and the Slint `accessible-label` names a property iced does not
//!   expose here; a caller wraps the pagination where those matter.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::svg;
use iced::widget::{row, text};
use iced::{Alignment, Element, Length, Theme};

use super::button::{Button, Variant};
use super::scrollable::{Axis, Scrollable};
use super::theme::{ControlSize, Tokens};

/// The run of page numbers is five wide and always contains the current page, so
/// the current page can never scroll out of the control. The Slint component's
/// `window-size`.
const WINDOW_SIZE: usize = 5;

/// Half of it, two. Slint keeps this as its own integer because `window-size / 2`
/// is a float division there and would shift the run by one on odd pages.
const WINDOW_HALF: usize = 2;

/// The gap between two controls, `spacing: 4px` in the Slint component.
const SPACING: f32 = 4.0;

/// `chevron-left`, from `crates/slint-component/ui/icons/` (Lucide, ISC).
const CHEVRON_LEFT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>"#;

/// `chevron-right`, from the same set.
const CHEVRON_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#;

/// Page controls, built with the methods below.
pub struct Pagination<'a, Message> {
    page: usize,
    pages: usize,
    size: ControlSize,
    compact: bool,
    on_change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

impl<'a, Message> Pagination<'a, Message> {
    /// Page one of ten, at the small size and compact, which is the Slint
    /// component's own default.
    pub fn new() -> Self {
        Self {
            page: 1,
            pages: 10,
            size: ControlSize::Small,
            compact: true,
            on_change: None,
        }
    }

    /// The current page, one-based. Clamped to the total when it is drawn, so the
    /// call order of `page` and `pages` never matters.
    pub fn page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    /// The number of pages. At least one, which is the Slint `pages`.
    pub fn pages(mut self, pages: usize) -> Self {
        self.pages = pages.max(1);
        self
    }

    /// The size the page buttons and the two arrows take.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The Slint `compact`: `true` — the default — draws the two step buttons
    /// icon-only, and `false` labels them "Previous" and "Next" beside their
    /// arrows.
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// A page was asked for. The component reports the requested number and
    /// leaves the current page to the caller, which is what makes the state on
    /// screen the caller's own. Without a handler the controls report nothing and
    /// draw unpressed, which is iced's "cannot be pressed".
    pub fn on_change(mut self, on_change: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }
}

impl<'a, Message> Default for Pagination<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// One of the two arrows, in the icon box the size gives it. iced ships no icon
/// set, so the chevron is drawn from the Lucide artwork the Slint folder carries.
///
/// An arrow with nowhere to go is drawn in the muted ink, like the carousel's
/// step: a `Button` cannot carry its disabled `text_color` into an SVG the
/// caller supplies, so the disabled ink is chosen here. Without it the boundary
/// arrow is inert but painted in the enabled foreground, which reads as live.
fn chevron<'a, Message: 'a>(
    previous: bool,
    size: ControlSize,
    disabled: bool,
) -> Element<'a, Message> {
    let pixels = size.icon_size();
    let handle = if previous {
        CHEVRON_LEFT
    } else {
        CHEVRON_RIGHT
    };
    svg(svg::Handle::from_memory(handle))
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

/// A step button: the ghost variant, icon-only when compact and labelled
/// otherwise. A previous button leads with its arrow; a next button trails it,
/// which is the Slint `icon` / `suffix-icon` split.
fn nav_button<'a, Message: Clone + 'a>(
    size: ControlSize,
    compact: bool,
    previous: bool,
    message: Option<Message>,
) -> Element<'a, Message> {
    let glyph = chevron(previous, size, message.is_none());
    let label = if previous { "Previous" } else { "Next" };

    let mut control = Button::new(if compact { "" } else { label })
        .variant(Variant::Ghost)
        .size(size);
    control = if compact || previous {
        control.icon(glyph)
    } else {
        control.suffix_icon(glyph)
    };
    if let Some(message) = message {
        control = control.on_press(message);
    }
    control.into()
}

/// A page number: the primary variant for the current page, the ghost one for
/// every other, which is the Slint ternary.
fn page_button<'a, Message: Clone + 'a>(
    page: usize,
    selected: bool,
    size: ControlSize,
    message: Option<Message>,
) -> Element<'a, Message> {
    let mut control = Button::new(page.to_string())
        .variant(if selected {
            Variant::Primary
        } else {
            Variant::Ghost
        })
        .size(size);
    if let Some(message) = message {
        control = control.on_press(message);
    }
    control.into()
}

/// The ellipsis standing in for the hidden pages.
fn ellipsis<'a, Message: 'a>(size: ControlSize) -> Element<'a, Message> {
    text("…")
        .size(size.control_font_size())
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

impl<'a, Message> From<Pagination<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Pagination<'a, Message>) -> Self {
        let pages = control.pages.max(1);
        let page = control.page.clamp(1, pages);
        let size = control.size;
        let compact = control.compact;
        let on_change = control.on_change;

        // One request per control, clamped the way the Slint `go` function clamps
        // it. `None` when the caller registered nothing, which leaves the control
        // unpressed rather than reporting into the void.
        let emit = |next: usize| on_change.as_ref().map(|on_change| on_change(next.clamp(1, pages)));

        let last_start = pages.saturating_sub(WINDOW_SIZE - 1).max(1);
        let start = page.saturating_sub(WINDOW_HALF).clamp(1, last_start);
        let end = (start + WINDOW_SIZE - 1).min(pages);

        let mut controls = row![]
            .spacing(SPACING)
            .align_y(Alignment::Center)
            .width(Length::Shrink);

        controls = controls.push(nav_button(
            size,
            compact,
            true,
            (page > 1)
                .then(|| emit(page.saturating_sub(1)))
                .flatten(),
        ));

        // Page one sits outside the run once the run has moved past it.
        if start > 1 {
            controls = controls.push(page_button(1, page == 1, size, emit(1)));
        }
        if start > 2 {
            controls = controls.push(ellipsis(size));
        }

        for number in start..=end {
            controls = controls.push(page_button(number, page == number, size, emit(number)));
        }

        if end < pages.saturating_sub(1) {
            controls = controls.push(ellipsis(size));
        }
        if end < pages {
            controls = controls.push(page_button(pages, page == pages, size, emit(pages)));
        }

        controls = controls.push(nav_button(
            size,
            compact,
            false,
            (page < pages)
                .then(|| emit(page.saturating_add(1)))
                .flatten(),
        ));

        Scrollable::new(controls)
            .axis(Axis::Horizontal)
            .height(Length::Fixed(size.control_height()))
            .into()
    }
}
