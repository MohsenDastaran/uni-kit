//! `message-role` — which side of a conversation a turn came from.
//!
//! The Slint authority, `crates/slint-component/ui/message-role.slint`, declares
//! one thing: `export enum MessageRole { user, assistant }`. It is shared by
//! `message.slint` and `bubble.slint` rather than drawn, so it has no sizes,
//! paddings, radii or colours of its own; every value the role decides is
//! decided in the two components that consume it. This port draws what those two
//! draw from the role alone, and nothing else:
//!
//! - **the marker a turn shows for it.** `message.slint` gives both sides an
//!   avatar at `AvatarSize.medium`; a sent turn fills it with
//!   `primary`, takes `primary-foreground` initials and borders it with the page
//!   background, and a received turn fills it with `muted` and takes
//!   `muted-foreground`. A sent turn that failed swaps both to `destructive`.
//!   [`MessageRole`] is that marker.
//! - **the side it sits on.** `user` is the sent turn and takes the trailing
//!   edge, `assistant` the received turn and the leading edge. A marker cannot
//!   place itself, so [`Role::alignment`] returns the alignment the caller puts
//!   it at; the example shows both edges.
//! - **the initials it falls back to.** `message.slint` reads an empty caller
//!   string as `"YO"` for `user` and `"AL"` for `assistant`, and so does this.
//!
//! [`Role`] is the Slint enum and carries every derivation above, so a caller
//! composing its own `Message` row can place a turn and colour its own avatar
//! exactly as `message.slint` does.
//!
//! Two things about the authority are reproduced rather than corrected, because
//! it is the authority: a failed sent turn's initials are drawn in
//! `destructive` — the fill itself, not `destructive-foreground` — and a
//! received turn is `muted` whatever `failed` says, since `message.slint` uses
//! its `tint` only on the sent side. The marker is a fixed 32-pixel circle, the
//! `AvatarSize.medium` the two Slint components both pick, with the avatar's
//! one-pixel border and no shadow.
//!
//! `message-role.slint` is an enum and draws nothing, so iced has no second
//! rendering of it to match: a role is a value here too. The reason this file
//! draws at all is that the registry key has to install as a component.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::{container, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow, Theme};

use super::theme::Tokens;

/// The marker's edge in pixels: `AvatarSize.medium` in `avatar.slint`, which is
/// the size `message.slint` gives the avatar of every turn.
pub const MARKER_EDGE: f32 = 32.0;

/// The avatar's own border width, one pixel.
pub const MARKER_BORDER_WIDTH: f32 = 1.0;

/// The Slint `MessageRole`, with the values its two consumers derive from it.
///
/// Slint defaults the property to `assistant` on both `Message` and `Bubble`,
/// so that is the default here too, even though `user` is the first Slint
/// variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Role {
    /// The sent turn: the trailing edge, the primary fill, `"YO"`.
    User,
    /// The received turn: the leading edge, the muted fill, `"AL"`.
    #[default]
    Assistant,
}

impl Role {
    /// Whether this is the side that sent the turn.
    pub fn is_sent(self) -> bool {
        matches!(self, Self::User)
    }

    /// Where the turn sits: a sent turn at the trailing edge, a received one at
    /// the leading edge. This is `message.slint`'s `alignment` property.
    pub fn alignment(self) -> Alignment {
        if self.is_sent() {
            Alignment::End
        } else {
            Alignment::Start
        }
    }

    /// The initials drawn when the caller names none, which is the fallback
    /// `message.slint` applies to an empty string.
    pub fn default_initials(self) -> &'static str {
        match self {
            Self::User => "YO",
            Self::Assistant => "AL",
        }
    }

    /// The marker's fill. Only the sent side answers to `failed`; the received
    /// side is muted either way.
    pub fn fill(self, tokens: &Tokens, failed: bool) -> Color {
        match self {
            Self::User if failed => tokens.destructive,
            Self::User => tokens.primary,
            Self::Assistant => tokens.muted,
        }
    }

    /// The initials' colour, which `message.slint` takes from the same pair of
    /// tokens as the fill — `primary-foreground` on the primary fill, and
    /// `destructive` itself on the failed one.
    pub fn ink(self, tokens: &Tokens, failed: bool) -> Color {
        match self {
            Self::User if failed => tokens.destructive,
            Self::User => tokens.primary_foreground,
            Self::Assistant => tokens.muted_foreground,
        }
    }

    /// The marker's border, which `message.slint` draws in the page's own
    /// background so the disc reads as cut out of it.
    pub fn marker_border(self, tokens: &Tokens) -> Color {
        tokens.background
    }
}

/// A role's marker, built with the methods below: the disc a chat turn shows for
/// the side it came from.
pub struct MessageRole<Message> {
    role: Role,
    initials: Option<String>,
    failed: bool,
    _message: std::marker::PhantomData<Message>,
}

impl<Message> MessageRole<Message> {
    pub fn new(role: Role) -> Self {
        Self {
            role,
            initials: None,
            failed: false,
            _message: std::marker::PhantomData,
        }
    }

    /// The initials to draw. A sent turn defaults to `"YO"`, a received one to
    /// `"AL"`; the empty string is the default too, as Slint reads it.
    pub fn initials(mut self, initials: impl Into<String>) -> Self {
        self.initials = Some(initials.into());
        self
    }

    /// A sent turn that failed to send, which `message.slint` tints
    /// destructively. It has no effect on a received turn.
    pub fn failed(mut self, failed: bool) -> Self {
        self.failed = failed;
        self
    }

    /// The initials actually drawn: the caller's when they are not empty, and
    /// the role's fallback otherwise.
    pub fn shown_initials(&self) -> &str {
        match self.initials.as_deref() {
            Some("") | None => self.role.default_initials(),
            Some(initials) => initials,
        }
    }
}

/// The marker's resolved look: its fill, border, corner and the colour its
/// initials are drawn in.
pub struct Style {
    pub role: Role,
    pub failed: bool,
}

impl Style {
    pub fn new(role: Role) -> Self {
        Self {
            role,
            failed: false,
        }
    }

    pub fn failed(mut self, failed: bool) -> Self {
        self.failed = failed;
        self
    }

    /// The disc: a fill, the avatar's one-pixel border in the page background,
    /// and the circle `avatar.slint` derives as `edge / 2`.
    pub fn resolve(&self, tokens: Tokens) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.role.fill(&tokens, self.failed))),
            border: Border {
                color: self.role.marker_border(&tokens),
                width: MARKER_BORDER_WIDTH,
                radius: (MARKER_EDGE / 2.0).into(),
            },
            shadow: Shadow::default(),
            ..container::Style::default()
        }
    }
}

impl<'a, Message> From<MessageRole<Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: MessageRole<Message>) -> Self {
        let role = control.role;
        let failed = control.failed;
        let style = Style::new(role).failed(failed);
        let initials = control.shown_initials().to_string();

        // The initials take their colour from the closure rather than from a
        // palette captured here, so a theme switch moves them with the disc.
        let content = container(
            text(initials)
                .size(MARKER_EDGE * 0.38)
                .font(iced::Font {
                    weight: iced::font::Weight::Semibold,
                    ..iced::Font::DEFAULT
                })
                .style(move |theme: &Theme| text::Style {
                    color: Some(role.ink(&Tokens::of(theme), failed)),
                }),
        )
        .width(Length::Fixed(MARKER_EDGE))
        .height(Length::Fixed(MARKER_EDGE))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .clip(true)
        .style(move |theme: &Theme| style.resolve(Tokens::of(theme)));

        content.into()
    }
}
