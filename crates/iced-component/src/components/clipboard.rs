//! A copy control. Mirrors GPUI Component's `Clipboard` and the Slint version
//! beside it: a ghost icon-only button whose copy mark becomes a check once the
//! caller says the value was copied, and whose press reports the value it holds.
//!
//! iced already ships the clipboard itself — `iced::clipboard::write` is a
//! `Task` an application returns from `update` — but no control to press. What
//! this adds over that call is the button, the two Lucide marks from
//! `crates/slint-component/ui/icons`, and the value a press carries.
//!
//! **The check is the caller's state.** The Slint and GPUI versions own `copied`
//! and clear it from a two-second timer. iced's timer needs a runtime feature
//! this crate does not enable — the same gap [`Icon`](super::icon) names for its
//! spin — so this takes the flag back and draws what it is told instead. The
//! application sets it on the press and clears it when the value it describes
//! stops being the one that was copied.
//!
//! **The write is the application's.** An element cannot perform an effect;
//! iced's clipboard API is a `Task` the application returns from `update`. The
//! press reports the value and the application writes it, which is the split
//! Slint draws between `copy-requested` and its host. On wasm that `Task` is a
//! no-op in iced 0.14 — `window_clipboard`'s wasm provider is a dummy that
//! refuses every write — so in the browser gallery the check is the whole of
//! what a press shows.
//!
//! **Two GPUI properties have no iced 0.14 counterpart.** There is no
//! accessibility tree to carry `accessibility_label`, and no tooltip: a caller
//! that wants one wraps this in [`Tooltip`](super::tooltip). Neither is faked.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::svg;
use iced::{Element, Theme};

use super::button::{self, Button, Variant};
use super::theme::{ControlSize, Tokens};

/// A copy control, built with the methods below.
pub struct Clipboard<'a, Message> {
    value: String,
    size: ControlSize,
    copied: bool,
    on_copy: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

impl<'a, Message> Clipboard<'a, Message> {
    /// A copy control for `value`: a ghost icon-only button, extra-small by
    /// default, which is the size both references default to.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            size: ControlSize::XSmall,
            copied: false,
            on_copy: None,
        }
    }

    /// The control's size. The icon box follows it: 12 / 14 / 16 / 24 pixels,
    /// the same ladder [`Button`] gives an icon.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// Whether the caller has just copied `value`: draws the check instead of
    /// the copy mark. The caller clears it — see the module comment for why the
    /// two-second reset is not the component's here.
    pub fn copied(mut self, copied: bool) -> Self {
        self.copied = copied;
        self
    }

    /// What a press reports — the value, so the application can hand it to
    /// `iced::clipboard::write`. Without it the control is drawn but disabled,
    /// the way every control in this crate spells "cannot be pressed".
    pub fn on_copy(mut self, on_copy: impl Fn(String) -> Message + 'a) -> Self {
        self.on_copy = Some(Box::new(on_copy));
        self
    }
}

impl<'a, Message> From<Clipboard<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Clipboard<'a, Message>) -> Self {
        let Clipboard {
            value,
            size,
            copied,
            on_copy,
        } = control;

        let artwork: &'static [u8] = if copied { CHECK } else { COPY };
        // The ink is resolved inside the draw closure, because the appearance is
        // only known when iced asks. An SVG tinted once at build time keeps its
        // own black — the colour it was born with — through the dark theme.
        let mark = svg(svg::Handle::from_memory(artwork))
            .width(size.icon_size())
            .height(size.icon_size())
            .style(|theme: &Theme, _status| svg::Style {
                color: Some(button::ink(Tokens::of(theme), Variant::Ghost, false)),
            });

        let mut button = Button::new("")
            .variant(Variant::Ghost)
            .size(size)
            .icon(mark);

        if let Some(on_copy) = on_copy {
            button = button.on_press(on_copy(value));
        }

        button.into()
    }
}

// The two Lucide marks the Slint icon set names (`copy` and `check`, Lucide,
// ISC), one line each, so the embedded artwork is the same the Slint and GPUI
// galleries draw.

const COPY: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>"#;

const CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 6 9 17l-5-5"/></svg>"#;
