//! A confirmation dialog: the modal that interrupts with one question and
//! offers a cancel and a confirm action. Mirrors the Slint `AlertDialog`, which
//! composes `Dialog` with an opinionated footer, and GPUI Component's
//! `AlertDialog`.
//!
//! # What it composes
//!
//! The Slint file is the authority, and it is a [`Dialog`] whose child is a
//! centered `DialogFooter` of two buttons — the cancel one first, the confirm
//! one last, switched to the danger variant when `destructive` is set. It adds
//! four properties (`title`, `description`, `cancel-text`, `confirm-text`) and
//! two callbacks (`cancelled`, `confirmed`) around that, and nothing else: there
//! is no icon slot, no caller body and no width of its own, because the example
//! draws those with a plain `Dialog`. This port is the same shape: [`AlertDialog`]
//! builds the [`Dialog`] and its [`DialogFooter`], [`AlertDialog::title`] and
//! [`AlertDialog::description`] are passed through to the panel, and
//! [`AlertDialog::cancel_text`] and [`AlertDialog::confirm_text`] name the
//! buttons.
//!
//! # Why a layer and not the popover's overlay
//!
//! An alert dialog *is* a dialog, so it inherits the answer [`dialog`]
//! documents: a modal is not a `Widget::overlay`, because iced's `overlay::Group`
//! hands an event to every overlay in the group and ignores a neighbour's
//! capture. It is a full-size in-flow layer placed in a
//! [`Stack`](iced::widget::Stack), and a stack routes an event to its children
//! last-first and stops at the first capture. This component does not add a
//! third mechanism; it builds on [`Dialog`], so the caller places it in the same
//! stack as the page and every routing rule applies unchanged.
//!
//! # The two channels, and who owns the open flag
//!
//! The caller owns `open`, exactly as with [`Dialog`]: [`AlertDialog::open`] is
//! what the layer renders from, and the component never changes it.
//!
//! [`AlertDialog::on_open_change`] is the composed dialog's own report, and it
//! carries the Slint `closed` callback's meaning: a press on the backdrop, the
//! close button and the Cancel button all ask for `false`, which the Slint
//! component sends to `cancelled`. The same closure is also what makes the
//! layer capture — iced captures an event only for a `mouse_area` that has a
//! message — so an alert dialog without `on_open_change` draws no Cancel button
//! and captures nothing; it is a static panel.
//!
//! [`AlertDialog::on_confirm`] is the confirm button's message, the Slint
//! `confirmed` callback. The application answers it by setting `open(false)`
//! itself and doing the work, which is what keeps the component controlled and
//! lets a confirmation that must not close simply not close.
//!
//! # What iced 0.14 has no way to express
//!
//! Everything [`dialog`] names is inherited, because this is that component with
//! a footer built in: no focus trap (Tab reaches the page behind, and focus is
//! not moved in or restored on close), no Escape at the layer (the application
//! binds it), no scroll lock, no transition, and `content-padding` is a value
//! rather than Slint's sub-420px breakpoint. One more belongs to this component:
//! Slint fires `cancelled` whenever `closed` fires, including when the host
//! itself writes `open`; iced can only report the dismissals it sees, so a
//! caller that sets `open(false)` directly learns nothing.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one method
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::Element;

use super::button::{Button, Variant};
use super::dialog::{Dialog, DialogFooter};

/// The text the confirm button carries until
/// [`confirm_text`](AlertDialog::confirm_text) names another, the Slint
/// `confirm-text` default.
const CONFIRM_TEXT: &str = "Continue";

/// The text the cancel button carries until
/// [`cancel_text`](AlertDialog::cancel_text) names another, the Slint
/// `cancel-text` default.
const CANCEL_TEXT: &str = "Cancel";

/// A confirmation dialog, built with the methods below.
///
/// It becomes an [`Element`] through `From`, and what it becomes is the
/// crate's [`Dialog`] — the dimmed backdrop, the centered panel and the footer
/// this component builds. It draws nothing at all while
/// [`open`](Self::open) is false.
pub struct AlertDialog<'a, Message> {
    open: bool,
    title: String,
    description: String,
    cancel_text: String,
    confirm_text: String,
    destructive: bool,
    on_confirm: Option<Message>,
    on_open_change: Option<Rc<dyn Fn(bool) -> Message + 'a>>,
}

impl<'a, Message> AlertDialog<'a, Message> {
    /// A confirmation dialog with no title and no description, closed, whose
    /// buttons read `Cancel` and `Continue` and whose confirm button has the
    /// primary variant.
    pub fn new() -> Self {
        Self {
            open: false,
            title: String::new(),
            description: String::new(),
            cancel_text: CANCEL_TEXT.to_string(),
            confirm_text: CONFIRM_TEXT.to_string(),
            destructive: false,
            on_confirm: None,
            on_open_change: None,
        }
    }

    /// Whether the layer is showing. The caller owns this, exactly as with
    /// [`Dialog::open`].
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// The question the dialog asks, drawn in the panel's title colour. Empty
    /// draws no title, which is the Slint component's `Without title` case.
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

    /// The cancel button's label, Slint's `cancel-text`. `Cancel` by default.
    pub fn cancel_text(mut self, cancel_text: impl Into<String>) -> Self {
        self.cancel_text = cancel_text.into();
        self
    }

    /// The confirm button's label, Slint's `confirm-text`. `Continue` by
    /// default, because the button's label should name the result and an alert
    /// that only says `OK` names nothing — a caller that means `OK` says so.
    pub fn confirm_text(mut self, confirm_text: impl Into<String>) -> Self {
        self.confirm_text = confirm_text.into();
        self
    }

    /// Whether the confirm button takes the danger variant. Slint's
    /// `destructive`; false draws the primary variant instead.
    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    /// Report every request to change the open state: the close button, a press
    /// on the backdrop, the panel asking to stay open, and the Cancel button.
    ///
    /// It is the composed [`Dialog::on_open_change`] and the Slint `cancelled`
    /// callback at once. This is also the message that makes the layer capture a
    /// press, so a component without it draws no Cancel button and blocks
    /// nothing.
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    /// The message the confirm button publishes, the Slint `confirmed` callback.
    /// The application closes the dialog by answering with its own state.
    pub fn on_confirm(mut self, message: Message) -> Self {
        self.on_confirm = Some(message);
        self
    }

    /// The composed dialog, or nothing at all while it is closed.
    fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let Self {
            open,
            title,
            description,
            cancel_text,
            confirm_text,
            destructive,
            on_confirm,
            on_open_change,
        } = self;

        // The Slint footer is centered and carries the cancel action first, the
        // confirm one last. The cancel button is the composed dialog's own
        // channel, so a caller that gave the dialog no channel gets no button to
        // press: there would be nothing for it to publish.
        let mut footer = DialogFooter::new().centered(true);
        if let Some(on_open_change) = &on_open_change {
            footer = footer.child(Button::new(cancel_text).on_press(on_open_change(false)));
        }
        if let Some(on_confirm) = on_confirm {
            footer = footer.child(
                Button::new(confirm_text)
                    .variant(if destructive {
                        Variant::Danger
                    } else {
                        Variant::Primary
                    })
                    .on_press(on_confirm),
            );
        }

        let dialog = Dialog::new()
            .open(open)
            .title(title)
            .description(description)
            .footer(footer);

        match on_open_change {
            Some(on_open_change) => dialog
                .on_open_change(move |open| on_open_change(open))
                .into(),
            None => dialog.into(),
        }
    }
}

impl<Message> Default for AlertDialog<'_, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<AlertDialog<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(alert: AlertDialog<'a, Message>) -> Self {
        alert.into_element()
    }
}
