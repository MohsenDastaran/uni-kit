//! `message_scroller` — a conversation that stays pinned to its newest turn,
//! in the two bubble variants the scroller's rows draw, with the unread rule,
//! the bottom fade and the jump-to-newest control it shows when the reader has
//! left the tail.
//!
//! The flag the scroller reports through is shared with the view rather than
//! copied into it, because iced hands the scroll event to an `Fn` closure and
//! the view is rebuilt after every message: a `Cell` owned by both sides is what
//! iced's own scroll documentation suggests, and `Rc<Cell<bool>>` is how this
//! example keeps one.
//!
//! Run it with `cargo run --example message_scroller`, or in the browser as
//! `?story=message_scroller` in the gallery the website embeds. Scroll the transcript up
//! with the wheel to bring out the fade and the round control; press the control
//! to come back to the tail.

use std::cell::Cell;
use std::rc::Rc;

use iced::widget::{Id, button, column, container, row, text};
use iced::{Element, Length, Task, Theme};

use crate::components::message_scroller::{MessageScroller, Turn, jump_to_end};
use crate::components::theme;

/// The scroller's name. It is what the jump control's message passes to
/// `jump_to_end`, and it has to be unique on screen.
const CONVERSATION: &str = "conversation";

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — message scroller")
        .theme(|app: &App| Some(if app.dark { Theme::Dark } else { Theme::Light }))
        .theme(theme)
        .run()
}

/// The flag the scroller and the view share.
fn signal(value: bool) -> Rc<Cell<bool>> {
    Rc::new(Cell::new(value))
}

/// The conversation the example opens with. The turns are long enough to wrap
/// at the bubble's cap, so a sent row and a received row both show the shape of
/// their variant.
fn transcript() -> Vec<Turn> {
    vec![
        Turn::received(
            "The transcript is a scroller, so the newest turn is the one you see: \
             the list keeps the viewport against its end while rows arrive.",
        ),
        Turn::sent(
            "And a row is a bubble, not a whole message: no avatar, no author \
             line, no footer.",
        ),
        Turn::received(
            "Sent turns sit on the muted surface at the trailing edge; received \
             turns are plain ghost text at the leading edge.",
        ),
        Turn::sent(
            "Scrolling up brings out the bottom fade and the round control that \
             comes back to the tail.",
        ),
    ]
}

#[derive(Debug, Clone)]
struct App {
    /// The turns themselves. The application owns the collection; the scroller
    /// is told the index of the unread boundary and nothing else.
    turns: Vec<Turn>,
    /// The flag the view's scroll callback writes. `true` while the newest turn
    /// is out of view, which is what the fade and the jump control are drawn
    /// from — the same thing Slint's `scrolled-up` property carries.
    scrolled_up: Rc<Cell<bool>>,
    /// Whether the rule is drawn above the first unseen turn, and where.
    unread: bool,
    /// Whether the scroller draws its jump control at all.
    jump_button: bool,
    dark: bool,
}

#[derive(Debug, Clone)]
enum Message {
    /// The wheel moved the transcript. `true` means it left the tail.
    Scrolled(bool),
    /// The jump control was pressed. The button itself reports; the viewport
    /// moves because the update runs `jump_to_end`.
    Jumped,
    /// A new turn arrived at the tail.
    Received,
    /// The reader sent a turn.
    Sent,
    /// Show or hide the unread rule.
    MarkUnread,
    /// Take the jump control out of the layout entirely, which is GPUI's
    /// `jump_button(false)`.
    ToggleJumpButton,
    ToggleTheme,
}

fn boot() -> App {
    App {
        turns: transcript(),
        scrolled_up: signal(false),
        unread: true,
        jump_button: true,
        dark: false,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Scrolled(scrolled_up) => {
            app.scrolled_up.set(scrolled_up);
            Task::none()
        }
        // iced owns the scroll offset, so the component reports that the jump
        // was asked for and the application moves the viewport.
        Message::Jumped => {
            app.scrolled_up.set(false);
            jump_to_end(Id::new(CONVERSATION))
        }
        Message::Received => {
            app.turns.push(Turn::received(
                "A turn that arrived while you were away. The list follows it \
                 only while you are at the tail.",
            ));
            Task::none()
        }
        Message::Sent => {
            app.turns.push(Turn::sent(
                "A turn you sent. The application appends to the collection and \
                 the scroller is told where the unread boundary moved.",
            ));
            Task::none()
        }
        Message::MarkUnread => {
            app.unread = !app.unread;
            Task::none()
        }
        Message::ToggleJumpButton => {
            app.jump_button = !app.jump_button;
            Task::none()
        }
        Message::ToggleTheme => {
            app.dark = !app.dark;
            Task::none()
        }
    }
}

fn view(app: &App) -> Element<'_, Message> {
    let scrolled_up = app.scrolled_up.get();
    // The rule belongs above the first turn the reader has not seen. Here that
    // is the second half of the transcript, and the toggle moves it, because the
    // meaning of "unread" belongs to the application.
    let unread_index = if app.unread {
        Some(app.turns.len() / 2)
    } else {
        None
    };

    let header = column![
        text("message scroller")
            .size(theme::FONT_SIZE_LG)
            .style(|theme: &Theme| iced::widget::text::Style {
                color: Some(theme::Tokens::of(theme).foreground),
            }),
        text(
            "A conversation pinned to its newest turn: bubbles, an unread rule, \
             a bottom fade and a jump-to-newest control."
        )
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| iced::widget::text::Style {
            color: Some(theme::Tokens::of(theme).muted_foreground),
        }),
        controls(app, scrolled_up),
    ]
    .spacing(theme::SPACING_MD);

    let conversation = conversation(app, unread_index, scrolled_up);

    let notes = column![
        text(if scrolled_up {
            "scrolled up: the fade and the jump control are drawn, and the list's \
             bottom padding grew to clear the control."
        } else {
            "at the tail: no fade, no control, and `anchor_bottom` keeps new rows \
             in view."
        })
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| iced::widget::text::Style {
            color: Some(theme::Tokens::of(theme).muted_foreground),
        }),
        text(format!(
            "{} turns — unread rule {} — jump control {}",
            app.turns.len(),
            if app.unread { "on" } else { "off" },
            if app.jump_button { "on" } else { "off" }
        ))
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| iced::widget::text::Style {
            color: Some(theme::Tokens::of(theme).muted_foreground),
        }),
    ]
    .spacing(theme::SPACING_XS);

    column![
        header,
        container(conversation)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme: &Theme| iced::widget::container::Style {
                border: iced::Border {
                    color: theme::Tokens::of(theme).border,
                    width: 1.0,
                    radius: theme::RADIUS_LG.into(),
                },
                ..iced::widget::container::Style::default()
            }),
        notes,
    ]
    .spacing(theme::SPACING_LG)
    .padding(theme::SPACING_XL)
    .into()
}

/// The controls the example uses to reach the states the component has: a rule
/// that is on or off, a jump control that is drawn or taken out, the two ends of
/// the transcript, and the light and dark token sets.
fn controls(app: &App, scrolled_up: bool) -> Element<'_, Message> {
    let chip = |label: &'static str, message: Message| {
        button(text(label).size(theme::FONT_SIZE_XS)).on_press(message)
    };

    row![
        chip("received turn", Message::Received),
        chip("sent turn", Message::Sent),
        chip(
            if app.unread {
                "unread rule on"
            } else {
                "unread rule off"
            },
            Message::MarkUnread
        ),
        chip(
            if app.jump_button {
                "jump control on"
            } else {
                "jump control off"
            },
            Message::ToggleJumpButton
        ),
        chip(
            if scrolled_up {
                "at the tail"
            } else {
                "away from the tail"
            },
            Message::Jumped
        ),
        chip("light / dark", Message::ToggleTheme),
    ]
    .spacing(theme::SPACING_SM)
    .wrap()
    .into()
}

/// The component itself, in one place so the builder reads as a whole.
fn conversation(app: &App, unread_index: Option<usize>, scrolled_up: bool) -> Element<'_, Message> {
    let flag = app.scrolled_up.clone();

    MessageScroller::new(Id::new(CONVERSATION), app.turns.clone())
        // Every state the component draws is decided here: `scrolled_up` is the
        // value the callback wrote, so the fade and the control appear as soon
        // as the reader leaves the tail.
        .scrolled_up(scrolled_up)
        .unread_index(unread_index)
        .jump_button(app.jump_button)
        // The list's own inset, GPUI's `p_5`.
        .padding(20.0)
        // The scrollbar is hidden in GPUI's story and shown here, so the
        // `scrollbar(false)` slot has something to differ from.
        .scrollbar(true)
        .on_scroll(move |scrolled_up| {
            flag.set(scrolled_up);
            Message::Scrolled(scrolled_up)
        })
        .on_jump(Message::Jumped)
        .into()
}
