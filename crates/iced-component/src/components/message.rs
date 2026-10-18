//! `message` — one chat turn: the identity marker, the sender line, the bubble
//! surface, the actions under it, and the delivery footer, all placed on the
//! edge the turn's role puts them.
//!
//! The Slint authority is `crates/slint-component/ui/message.slint`, with the
//! example beside it in `ui/examples/message.slint`; the GPUI original is
//! `crates/component/src/message.rs`. This port follows the Slint rendering,
//! because that is the design the two galleries share:
//!
//! - **[`Role`]** (the Slint `MessageRole`) decides the side, the marker's fill
//!   and ink, and the initials an empty string falls back to. [`Message::alignment`]
//!   overrides the side the role would pick.
//! - **[`MessageRole`]** is the marker itself — the finished `super::message_role`
//!   disc, reused rather than redrawn — unless the caller passes its own portrait
//!   through [`Message::avatar`], which is [`super::avatar::Avatar`]'s job.
//! - **[`Bubble`]** is the surface. The body is its text and
//!   [`Message::child`] adds rich content inside it, so an attachment, a code
//!   block or another control is composed the way the GPUI page describes.
//! - **[`Button`]** is the footer's `Retry`, and any action the caller adds
//!   through [`Message::action`].
//!
//! Four deliberate differences from the Slint source, all because iced 0.14 has
//! no equivalent:
//!
//! - **The action row is the caller's.** Slint's `show-actions` draws three fixed
//!   icon buttons (copy, heart, star) from its own SVG folder; iced ships no icon
//!   set, so [`Message::action`] takes the controls and their messages, exactly as
//!   the page says actions belong to the application.
//! - **`Retry` is drawn only when it can fire.** Slint always draws the footer
//!   control and calls back on press; iced spells "cannot be pressed" as a control
//!   with no message, so [`Message::on_retry`] draws it and no message leaves it
//!   out.
//! - **The metadata inset is explicit.** GPUI insets the header and footer to the
//!   bubble's own text (`px_3`), and removes the inset for a ghost surface; Slint
//!   leaves the metadata flush with the bubble's outer edge. This follows GPUI,
//!   and [`Message::content_inset`] overrides the inherited value.
//! - **No minimum width.** Slint holds `min-width: 0` on the row and its stack;
//!   iced has no `min-width`, so a long body is capped by the bubble's own
//!   [`super::bubble::MAX_WIDTH`] rather than by the column.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{column, container, row, text};
use iced::{Alignment as IcedAlignment, Element, Font, Length, Padding, Theme};

use super::bubble::{Bubble, Content as BubbleContent, Variant as BubbleVariant};
use super::button::{Button, Variant as ButtonVariant};
use super::message_role::MARKER_EDGE;
use super::theme::{self, ControlSize, Tokens};

// The role and its marker are re-exported so a caller imports the turn's parts
// from `message` alone, as the Slint component re-exports `MessageRole`. The
// edge type is the bubble's own, reused rather than redeclared.
pub use super::bubble::Alignment;
pub use super::message_role::{MessageRole, Role};

/// The 10px gap between the sender line, the bubble, the actions and the footer:
/// GPUI's `gap(0.625rem)` and the Slint `VerticalLayout`'s `spacing: 10px`.
const STACK_GAP: f32 = 10.0;

/// The 6px gap inside the sender and footer rows, Slint's `spacing: 6px`.
const META_GAP: f32 = 6.0;

/// The metadata text size, 11px in the Slint source.
const META_SIZE: f32 = 11.0;

/// How far the footer clears the avatar column: the marker's edge plus the row's
/// gap, which is the Slint footer's `padding-left: 40px`.
const AVATAR_OFFSET: f32 = MARKER_EDGE + theme::SPACING_SM;

/// The header, actions and footer inset, GPUI's `px_3`, which lines them up with
/// the text inside a regular bubble surface.
const CONTENT_INSET: f32 = theme::SPACING_MD;

/// One chat turn, built with the methods below.
pub struct Message<'a, Msg> {
    role: Role,
    alignment: Option<Alignment>,
    author: String,
    time: String,
    body: String,
    initials: Option<String>,
    footer: String,
    failed: bool,
    show_avatar: bool,
    avatar: Option<Element<'a, Msg>>,
    bubble_variant: Option<BubbleVariant>,
    bubble_content: Option<BubbleContent<'a, Msg>>,
    content_inset: Option<bool>,
    children: Vec<Element<'a, Msg>>,
    actions: Vec<Element<'a, Msg>>,
    on_retry: Option<Msg>,
}

impl<'a, Msg> Message<'a, Msg> {
    /// A received turn at the leading edge, with no sender line, body or footer.
    pub fn new() -> Self {
        Self {
            role: Role::default(),
            alignment: None,
            author: String::new(),
            time: String::new(),
            body: String::new(),
            initials: None,
            footer: String::new(),
            failed: false,
            show_avatar: true,
            avatar: None,
            bubble_variant: None,
            bubble_content: None,
            content_inset: None,
            children: Vec::new(),
            actions: Vec::new(),
            on_retry: None,
        }
    }

    /// Which side the turn came from. The role picks the edge, the marker's fill
    /// and ink, and the fallback initials; a sent turn is the trailing one.
    pub fn role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }

    /// Place the turn at a specific edge, overriding the role's own side.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    /// The sender name, drawn above the bubble at `text-xs` and semibold.
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = author.into();
        self
    }

    /// The timestamp beside the sender, drawn at 11px in the muted ink.
    pub fn time(mut self, time: impl Into<String>) -> Self {
        self.time = time.into();
        self
    }

    /// The message body, drawn in the bubble's own ink.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    /// The marker's initials. Empty is the default and reads as the role's own
    /// fallback, exactly as the Slint component treats an empty string.
    pub fn initials(mut self, initials: impl Into<String>) -> Self {
        self.initials = Some(initials.into());
        self
    }

    /// The footer line, drawn at 11px in the muted ink.
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = footer.into();
        self
    }

    /// A turn that failed to send: the bubble turns destructive, the footer says
    /// "Failed to send", and a `Retry` control appears when [`Message::on_retry`]
    /// can fire.
    pub fn failed(mut self, failed: bool) -> Self {
        self.failed = failed;
        self
    }

    /// Whether the identity marker is drawn. Slint's `show-avatar`.
    pub fn show_avatar(mut self, show_avatar: bool) -> Self {
        self.show_avatar = show_avatar;
        self
    }

    /// The caller's own portrait or marker, replacing the role's initials disc.
    /// Normally a `super::avatar::Avatar`; the slot accepts any element.
    pub fn avatar(mut self, avatar: impl Into<Element<'a, Msg>>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    /// Select the bubble's surface treatment. The default follows the role —
    /// `filled` for a sent turn, `secondary` for a received one — and a failed
    /// turn is always destructive.
    pub fn bubble_variant(mut self, variant: BubbleVariant) -> Self {
        self.bubble_variant = Some(variant);
        self
    }

    /// Replace the bubble's visible surface, for a caller-owned colour. The body
    /// text follows the surface's ink, as `super::bubble::Content` describes.
    pub fn bubble_content(mut self, content: BubbleContent<'a, Msg>) -> Self {
        self.bubble_content = Some(content);
        self
    }

    /// Keep or remove the metadata inset. The default is the regular `px_3` inset,
    /// removed for a ghost surface because there is no surface to align with.
    pub fn content_inset(mut self, content_inset: bool) -> Self {
        self.content_inset = Some(content_inset);
        self
    }

    /// Rich content inside the bubble, after its body text: an attachment, a code
    /// block, an image or another control.
    pub fn child(mut self, child: impl Into<Element<'a, Msg>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// A control in the action row under the bubble. The message carries the
    /// control's own message, as every iced component does.
    pub fn action(mut self, action: impl Into<Element<'a, Msg>>) -> Self {
        self.actions.push(action.into());
        self
    }

    /// The message the footer's `Retry` reports. Without one no `Retry` is drawn.
    pub fn on_retry(mut self, message: Msg) -> Self {
        self.on_retry = Some(message);
        self
    }
}

impl<'a, Msg> Default for Message<'a, Msg> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Msg> From<Message<'a, Msg>> for Element<'a, Msg>
where
    Msg: Clone + 'a,
{
    fn from(control: Message<'a, Msg>) -> Self {
        let role = control.role;
        let at_end = control
            .alignment
            .unwrap_or_else(|| {
                if role.is_sent() {
                    Alignment::End
                } else {
                    Alignment::Start
                }
            })
            .is_end();
        let axis = if at_end {
            IcedAlignment::End
        } else {
            IcedAlignment::Start
        };
        let alignment = if at_end {
            Alignment::End
        } else {
            Alignment::Start
        };
        let failed = control.failed;
        let variant = if failed {
            BubbleVariant::Destructive
        } else {
            control.bubble_variant.unwrap_or(if role.is_sent() {
                BubbleVariant::Filled
            } else {
                BubbleVariant::Secondary
            })
        };
        // A ghost has no surface for the metadata to line up with, which is the
        // inset GPUI removes and `content_inset` overrides.
        let inset = control
            .content_inset
            .unwrap_or(variant != BubbleVariant::Ghost);
        let padding = if inset { CONTENT_INSET } else { 0.0 };

        // The marker: the caller's portrait when there is one, the role's own
        // disc otherwise. `MessageRole` resolves every token inside its style
        // closure, so the disc follows the theme.
        let marker: Option<Element<'a, Msg>> = if control.show_avatar {
            Some(match control.avatar {
                Some(avatar) => avatar,
                None => MessageRole::new(role)
                    .initials(control.initials.clone().unwrap_or_default())
                    .failed(failed)
                    .into(),
            })
        } else {
            None
        };

        // The content column: the sender line, the bubble, the actions. Its width
        // is `Fill` inside the row below, which is what lets the bubble's own
        // `Fill` surface resolve instead of laying out at zero.
        let mut stack = column![]
            .spacing(STACK_GAP)
            .width(Length::Fill)
            .align_x(axis);

        // The sender line. `author` is the foreground at extra-small semibold,
        // `time` the muted 11px beside it, as Slint draws the pair.
        if !control.author.is_empty() || !control.time.is_empty() {
            let mut header = row![].spacing(META_GAP).align_y(IcedAlignment::Center);
            if !control.author.is_empty() {
                let author = control.author.clone();
                header = header.push(
                    text(author)
                        .size(theme::FONT_SIZE_XS)
                        .font(Font {
                            weight: iced::font::Weight::Semibold,
                            ..Font::DEFAULT
                        })
                        .style(|theme: &Theme| text::Style {
                            color: Some(Tokens::of(theme).foreground),
                        }),
                );
            }
            if !control.time.is_empty() {
                let time = control.time.clone();
                header =
                    header.push(
                        text(time)
                            .size(META_SIZE)
                            .style(|theme: &Theme| text::Style {
                                color: Some(Tokens::of(theme).muted_foreground),
                            }),
                    );
            }
            stack = stack.push(container(header).padding([0.0, padding]));
        }

        // The bubble. `content` is the caller's surface overrides; the body text
        // and every child are moved into whatever surface it names.
        let mut bubble = Bubble::new()
            .alignment(alignment)
            .variant(variant)
            .text(control.body);
        if let Some(content) = control.bubble_content {
            bubble = bubble.content(content);
        }
        for child in control.children {
            bubble = bubble.child(child);
        }
        stack = stack.push(bubble);

        // The action row: the caller's controls, spaced as Slint spaces the
        // buttons `show-actions` would draw.
        if !control.actions.is_empty() {
            let actions = row(control.actions)
                .spacing(theme::SPACING_XXS)
                .align_y(IcedAlignment::Center);
            stack = stack.push(container(actions).padding([0.0, padding]));
        }

        // The identity row. The marker is bottom-aligned with the content, so it
        // stays flush however tall the bubble grows, and it swaps sides with the
        // alignment.
        let mut frame = row![]
            .spacing(theme::SPACING_SM)
            .align_y(IcedAlignment::End)
            .width(Length::Fill);
        if at_end {
            frame = frame.push(stack);
            if let Some(marker) = marker {
                frame = frame.push(marker);
            }
        } else {
            if let Some(marker) = marker {
                frame = frame.push(marker);
            }
            frame = frame.push(stack);
        }

        // The footer is outside the identity row, so the bottom-anchored marker
        // stays flush with the content whatever the footer contains, and it
        // carries the marker's width plus the row gap on the marker's side.
        let has_footer = !control.footer.is_empty() || failed;
        let mut root = column![]
            .spacing(STACK_GAP)
            .width(Length::Fill)
            .align_x(axis);
        root = root.push(frame);

        if has_footer {
            let mut footer = row![].spacing(META_GAP).align_y(IcedAlignment::Center);
            if failed {
                footer = footer.push(text("Failed to send").size(META_SIZE).style(
                    |theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).destructive),
                    },
                ));
            }
            if !control.footer.is_empty() {
                let value = control.footer.clone();
                footer =
                    footer.push(
                        text(value)
                            .size(META_SIZE)
                            .style(|theme: &Theme| text::Style {
                                color: Some(Tokens::of(theme).muted_foreground),
                            }),
                    );
            }
            if let Some(message) = control.on_retry.clone() {
                if failed {
                    footer = footer.push(
                        Button::new("Retry")
                            .variant(ButtonVariant::Text)
                            .size(ControlSize::XSmall)
                            .on_press(message),
                    );
                }
            }

            let clear = if control.show_avatar {
                AVATAR_OFFSET
            } else {
                0.0
            };
            root = root.push(container(footer).padding(Padding {
                top: 0.0,
                right: if at_end { clear } else { 0.0 },
                bottom: 0.0,
                left: if at_end { 0.0 } else { clear },
            }));
        }

        root.into()
    }
}

/// A vertical stack of consecutive turns from one sender, the Slint
/// `BubbleGroup`/GPUI `MessageGroup`. It does not decide which sender owns a
/// message; the application says which rows group.
pub struct Group<'a, Msg> {
    alignment: Alignment,
    spacing: f32,
    children: Vec<Element<'a, Msg>>,
}

impl<'a, Msg> Group<'a, Msg> {
    /// An empty group, aligned to the leading edge and spaced 8px apart, GPUI's
    /// `gap_2`.
    pub fn new() -> Self {
        Self {
            alignment: Alignment::default(),
            spacing: theme::SPACING_SM,
            children: Vec::new(),
        }
    }

    /// Align the whole stack to an edge, as the Slint `BubbleGroup`'s `align`.
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// The gap between the rows.
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Add a complete message.
    pub fn child(mut self, child: impl Into<Element<'a, Msg>>) -> Self {
        self.children.push(child.into());
        self
    }
}

impl<'a, Msg> Default for Group<'a, Msg> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Msg> From<Group<'a, Msg>> for Element<'a, Msg>
where
    Msg: 'a,
{
    fn from(control: Group<'a, Msg>) -> Self {
        column(control.children)
            .spacing(control.spacing)
            .width(Length::Fill)
            .align_x(control.alignment.axis())
            .into()
    }
}
