//! A labeled chip that can be dismissed. Mirrors GPUI Component's `Tag` and
//! the Slint version beside it: four variants, a status, a leading icon, sizes
//! from 16 to 32 pixels tall, and an optional close control.
//!
//! The height is the control height less eight pixels, so the chip sits inside
//! a control-sized row: 16, 24 and 32 pixels for the small, medium and large
//! steps. The fill follows the variant — the theme's muted surface by default,
//! the secondary surface, nothing with a border, or the status colour at the
//! theme's tint — and the ink is the status colour when the tag is a status
//! one and the foreground otherwise.
//!
//! iced has no icon set of its own and the application owns its assets, so the
//! leading icon is an element the caller passes. The close control is drawn as
//! a ghost button in the same shape as [`super::button`], with the embedded
//! Lucide `x` as its mark: the `×` character carries its own side bearings in
//! its em box, so it sat against the control's leading edge rather than at its
//! centre.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{button, container, row, svg, text};
use iced::{Border, Color, Element, Length, Theme};

use super::theme::{self, ControlSize, Status, Tokens};

/// How the chip is filled. The names are the Slint `TagVariant`'s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// The muted surface.
    #[default]
    Default,
    /// The secondary surface.
    Secondary,
    /// No fill, with a one-pixel border.
    Outline,
    /// The status colour at the theme's tint, with the status colour for ink.
    Status,
}

/// A tag, built with the methods below.
pub struct Tag<'a, Message> {
    label: String,
    variant: Variant,
    status: Status,
    size: ControlSize,
    icon: Option<Element<'a, Message>>,
    on_close: Option<Message>,
}

impl<'a, Message> Tag<'a, Message> {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::default(),
            status: Status::default(),
            size: ControlSize::default(),
            icon: None,
            on_close: None,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// The status a status tag reports. The other variants ignore it.
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self.variant = Variant::Status;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// A leading element at the size's icon box, two pixels smaller than the
    /// box the theme gives icons.
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Draws the close control and sends this message when it is pressed.
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// The chip's height: the control height less the eight pixels a tag keeps
    /// for itself — 16, 20 or 32.
    fn height(&self) -> f32 {
        match self.size {
            ControlSize::XSmall => 12.0,
            ControlSize::Small => 16.0,
            ControlSize::Large => 32.0,
            ControlSize::Medium => 24.0,
        }
    }

    fn pad_x(&self) -> f32 {
        self.size.control_padding() - 2.0
    }

    fn ink(&self, tokens: Tokens) -> Color {
        match self.variant {
            Variant::Status => status_ink(tokens, self.status),
            _ => tokens.foreground,
        }
    }

    fn fill(&self, tokens: Tokens) -> Option<Color> {
        match self.variant {
            Variant::Default => Some(tokens.muted),
            Variant::Secondary => Some(tokens.secondary),
            Variant::Outline => None,
            Variant::Status => {
                Some(theme::alpha(tokens.status_color(self.status), if tokens.dark { 0.18 } else { 0.12 }))
            }
        }
    }
}

/// The ink a tinted status chip draws its label in.
///
/// The dark theme's tint is deep enough for the status hue itself, but the light
/// theme tints the page's own data colours at 12%, and several of those are
/// already pale — `#93c5fd` as ink on its own tint is a label that has to be
/// hunted for. Darkening the same hue keeps the meaning and the contrast. This
/// is the same reading [`super::button::ink`] makes when it refuses hue-on-hue.
fn status_ink(tokens: Tokens, status: Status) -> Color {
    let tone = tokens.status_color(status);
    if tokens.dark {
        tone
    } else {
        theme::darken(tone, 0.45)
    }
}

/// The colours a tag draws its chip and its ink with.
pub struct Style {
    pub variant: Variant,
    pub status: Status,
}

impl Style {
    pub fn new(variant: Variant) -> Self {
        Self {
            variant,
            status: Status::default(),
        }
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn resolve(&self, tokens: Tokens) -> container::Style {
        let ink = match self.variant {
            Variant::Status => status_ink(tokens, self.status),
            _ => tokens.foreground,
        };
        container::Style {
            background: match self.variant {
                Variant::Default => Some(tokens.muted.into()),
                Variant::Secondary => Some(tokens.secondary.into()),
                Variant::Outline => None,
                Variant::Status => Some(
                    theme::alpha(
                        tokens.status_color(self.status),
                        if tokens.dark { 0.18 } else { 0.12 },
                    )
                    .into(),
                ),
            },
            text_color: Some(ink),
            border: Border {
                color: tokens.border,
                width: if self.variant == Variant::Outline { 1.0 } else { 0.0 },
                radius: theme::RADIUS_MD.into(),
            },
            ..container::Style::default()
        }
    }
}

impl<'a, Message> From<Tag<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Tag<'a, Message>) -> Self {
        let variant = control.variant;
        let status = control.status;
        let size = control.size;
        let height = control.height();
        let pad_x = control.pad_x();
        let closable = control.on_close.is_some();
        let label = control.label.clone();

        let mut content = row![]
            .spacing(4.0)
            .align_y(iced::Alignment::Center)
            .height(Length::Fill);

        if let Some(icon) = control.icon {
            content = content.push(icon);
        }

        content = content.push(
            text(label)
                .size(size.control_font_size())
                .style(move |theme: &Theme| text::Style {
                    color: Some(match variant {
                        Variant::Status => status_ink(Tokens::of(theme), status),
                        _ => Tokens::of(theme).foreground,
                    }),
                }),
        );

        if let Some(message) = control.on_close {
            // The mark is the embedded `x`, not the `×` character: a text child
            // is laid out at its box's leading edge, so the glyph sat a pixel
            // from the left of a 14x14 box and a third of the way down when
            // measured. The container centres the mark on both axes. The ink is
            // resolved inside the closure because an SVG tinted once at build
            // time keeps the light theme's colour in the dark.
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
            .width(Length::Fixed(CLOSE_SIZE))
            .height(Length::Fixed(CLOSE_SIZE))
            .style(|theme: &Theme, status| {
                let tokens = Tokens::of(theme);
                // The close control is never a status-coloured surface, so its
                // glyph is the muted foreground rather than the chip's ink.
                let ink = tokens.muted_foreground;
                button::Style {
                    background: match status {
                        button::Status::Hovered | button::Status::Pressed => {
                            Some(theme::alpha(tokens.foreground, 0.1).into())
                        }
                        _ => None,
                    },
                    text_color: ink,
                    border: Border {
                        radius: theme::RADIUS_SM.into(),
                        ..Border::default()
                    },
                    shadow: iced::Shadow::default(),
                    snap: false,
                }
            })
            .on_press(message);

            content = content.push(close);
        }

        container(content)
            .height(Length::Fixed(height))
            .padding(iced::Padding {
                top: 0.0,
                right: if closable { 4.0 } else { pad_x },
                bottom: 0.0,
                left: pad_x,
            })
            .align_y(iced::Alignment::Center)
            .style(move |theme: &Theme| {
                Style::new(variant)
                    .status(status)
                    .resolve(Tokens::of(theme))
            })
            .into()
    }
}

/// The close control's box and the size its mark is drawn at: the 14px square
/// the unfixed control already kept, which is the tag's own inline size.
const CLOSE_SIZE: f32 = 14.0;

/// The Lucide `x` from `crates/slint-component/ui/icons/x.svg` (Lucide, ISC):
/// the mark the Slint close control names, in the same one-line form the
/// crate's other embedded icons use.
const CLOSE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>"#;
