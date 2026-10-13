//! A rating: a row of stars that reports the value it wants to become. Mirrors
//! GPUI Component's `Rating` and the Slint version beside it — a value, a
//! maximum, the four sizes, a read-only row and GPUI's click rule.
//!
//! **The value is the owner's.** iced has no in-out property, so [`Rating::new`]
//! takes the value and [`Rating::on_change`] reports the one a press asked for;
//! the owner writes it back, which is what makes the row controllable. A press
//! on a star that is already part of the value clears it to one less — GPUI's
//! rule, and the one the component page documents — so a five-star row can be
//! returned to nothing.
//!
//! **Hover is reported, not held.** The stars up to the one under the pointer
//! are drawn in the active colour without becoming filled, which is GPUI's
//! preview. A star's own style closure sees only its own interaction status, so
//! the row cannot know which star the pointer is over without being told:
//! [`Rating::on_hover`] reports the star that was entered and `None` when the
//! pointer leaves, and [`Rating::hovered`] takes the value back to draw with.
//!
//! **Read-only is the absence of a message**, as everywhere in this crate:
//! `on_change(msg)` produces an interactive row, and one built without it draws
//! the same stars and reports nothing. Slint fades a read-only row to half
//! opacity, which iced 0.14 cannot do to a subtree, and neither reference fades
//! it either: the filled stars keep the active colour, and only the pointer
//! response is gone.
//!
//! The star is the component's own chrome, so the two Lucide marks are embedded
//! here the way `pagination` embeds its chevrons. Their colour is resolved from
//! the tokens inside the draw closure, never when the element is built: a star
//! tinted once at build time would keep the light theme's warning yellow after
//! the window switched to dark.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::button as iced_button;
use iced::widget::svg;
use iced::widget::{mouse_area, row};
use iced::{Border, Color, Element, Shadow, Theme};

use super::button::handle;
use super::theme::{self, ControlSize, Tokens};

/// A row of stars, built with the methods below.
pub struct Rating<'a, Message> {
    value: usize,
    max: usize,
    size: ControlSize,
    hovered: Option<usize>,
    on_change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    on_hover: Option<Box<dyn Fn(Option<usize>) -> Message + 'a>>,
}

impl<'a, Message> Rating<'a, Message> {
    /// A rating of `value`, out of five stars at the medium size. Without
    /// [`on_change`](Self::on_change) the row is read-only.
    pub fn new(value: usize) -> Self {
        Self {
            value,
            max: 5,
            size: ControlSize::default(),
            hovered: None,
            on_change: None,
            on_hover: None,
        }
    }

    /// The number of stars, five by default. A value above it is clamped, as
    /// GPUI's `max` clamps.
    pub fn max(mut self, max: usize) -> Self {
        self.max = max;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The star the pointer is over, as [`on_hover`](Self::on_hover) reported
    /// it. Stars up to it are drawn in the active colour without becoming
    /// filled; a read-only row shows no preview.
    pub fn hovered(mut self, hovered: Option<usize>) -> Self {
        self.hovered = hovered;
        self
    }

    /// The value to report when a star is pressed. Without this the row is
    /// read-only: no star can be pressed and no hover is reported.
    pub fn on_change(mut self, on_change: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// The star the pointer entered, and `None` when it leaves the row — the
    /// half of the preview that the row cannot observe for itself. The owner
    /// writes the value back through [`hovered`](Self::hovered).
    pub fn on_hover(mut self, on_hover: impl Fn(Option<usize>) -> Message + 'a) -> Self {
        self.on_hover = Some(Box::new(on_hover));
        self
    }
}

/// The colour a star is drawn in: the active colour while it is filled or the
/// pointer is over it, and the muted foreground at 40% otherwise, which is the
/// Slint row's `muted-foreground.with-alpha(0.4)`.
fn star_ink(tokens: Tokens, filled: bool, marked: bool) -> Color {
    if filled || marked {
        tokens.warning
    } else {
        theme::alpha(tokens.muted_foreground, 0.4)
    }
}

/// One star. The outline and the solid mark are the same Lucide path, so a
/// filled star keeps the silhouette of the empty one it replaces.
fn star<'a, Message: 'a>(filled: bool, marked: bool, pixels: f32) -> Element<'a, Message> {
    let artwork: &'static [u8] = if filled { STAR_FILL } else { STAR };

    svg(svg::Handle::from_memory(artwork))
        .width(pixels)
        .height(pixels)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(star_ink(Tokens::of(theme), filled, marked)),
        })
        .into()
}

/// The handle's chrome: none. The star is the whole control, so the pressable
/// shape draws nothing of its own and the row reads as stars rather than as five
/// buttons.
fn chrome(_theme: &Theme, _status: iced_button::Status) -> iced_button::Style {
    iced_button::Style {
        background: None,
        text_color: Color::TRANSPARENT,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

impl<'a, Message> From<Rating<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Rating<'a, Message>) -> Self {
        let Rating {
            value,
            max,
            size,
            hovered,
            on_change,
            on_hover,
        } = control;
        let value = value.min(max);
        let hovered = hovered.filter(|hovered| *hovered <= max);
        let enabled = on_change.is_some();
        // A read-only row reports nothing at all, hover included, so the hover
        // callback is only wired up when there is a value to change.
        let on_hover = if enabled { on_hover.as_ref() } else { None };
        // Slint's star is the icon box plus two pixels: 14 / 16 / 18 / 26.
        let pixels = size.icon_size() + 2.0;

        let stars: Vec<Element<'a, Message>> = (1..=max)
            .map(|ix| {
                let filled = ix <= value;
                let marked = enabled && hovered.is_some_and(|hovered| hovered >= ix);
                // The value a press wants to become: a star that is already part
                // of the value clears itself rather than setting itself again.
                let on_press = on_change
                    .as_ref()
                    .map(|on_change| on_change(if value >= ix { ix - 1 } else { ix }));

                let mut area = mouse_area(handle(star(filled, marked, pixels), on_press, chrome));
                if let Some(on_hover) = on_hover {
                    area = area.on_enter(on_hover(Some(ix))).on_exit(on_hover(None));
                }
                area.into()
            })
            .collect();

        // Slint's row spacing, 2px, with the stars at the start of the row.
        row(stars).spacing(2.0).into()
    }
}

/// The Lucide `star` mark, the one the GPUI and Slint icon sets draw.
const STAR: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/></svg>"#;

/// The same mark filled, which is what a star that is part of the value draws.
const STAR_FILL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/></svg>"#;
