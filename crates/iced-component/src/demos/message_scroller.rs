//! `message_scroller` — the gallery the page documents: the scroller created and
//! started at its tail, its rows and the application's own empty state,
//! appending and follow-tail, prepending earlier history, the unread rule, the
//! built-in jump control, and the style slots a host can take over.
//!
//! Run it with `cargo run --example message_scroller`, or in the browser as
//! `?story=message-scroller` in the gallery the website embeds.
//!
//! The section titles below are `website/component/message-scroller.md`'s own
//! sample headings, which is the depth the page documents each one at. The Slint
//! example's six `GroupBox`es fold into them through the page's `SHARED` table
//! (`website/src/lib/remark-slint-source.js`): Conversation → create state,
//! Empty conversation → render rows and an empty state, Streaming responses →
//! append and follow-tail, Earlier history → prepend, Jump controls →
//! jump-to-latest, and Custom jump button → the style slots. The unread rule is
//! the page's own seventh sample and has no Slint box.
//!
//! Each section carries a heading the page has — the title, and the code icon
//! that asks the embedding page to scroll to that sample. Clicking either one
//! posts the section's index and title through [`crate::bridge`], the same bridge
//! the Slint and GPUI galleries use. The `// section: <title>` comments below are
//! the contract the website reads to cut this file into the usage panels a
//! component page shows, so the code on the page is the code that runs here. Keep
//! them, one per section, in order, and keep the closing `// section: end` after
//! the last one.
//!
//! Two things the page documents have no iced counterpart here and are named
//! rather than faked: iced 0.14 exposes no arbitrary scroll-to-index, so the
//! unread section shows the rule alone where the GPUI sample navigates to a row;
//! and the jump control has no renderer callback or style refinement, so a
//! host-drawn control is `jump_button(false)` plus the application's own button,
//! which is what the style-slots section shows.
//!
//! The flag the scroller reports through is shared with the view rather than
//! copied into it, because iced hands the scroll event to an `Fn` closure and the
//! view is rebuilt after every message: a `Cell` owned by both sides is what
//! iced's own scroll documentation suggests, and `Rc<Cell<bool>>` is how this
//! example keeps one.

use crate::components::button::{Button, Variant as ButtonVariant};
use crate::components::group_box::{self, GroupBox};
use crate::components::message_scroller::{MessageScroller, Turn, jump_to_end};
use crate::components::theme::{self, ControlSize, Tokens};
use iced::widget::{Id, button, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment as Axis, Color, Element, Length, Task, Theme};
use std::cell::Cell;
use std::rc::Rc;

/// The gap between sections, the page's own `xl`.
const PAGE: f32 = theme::SPACING_XL;

/// The height of a scroller's panel, so every section is a frame of the same
/// size rather than one that grows with its copy.
const PANEL_HEIGHT: f32 = 220.0;

/// The interactive scroller's name. It is what this demo's own jump control
/// passes to `jump_to_end`, and it has to be unique on screen.
const CONVERSATION: &str = "tail";

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — message scroller")
        .theme(theme)
        .run()
}

/// The flag the scroller and the view share.
fn signal(value: bool) -> Rc<Cell<bool>> {
    Rc::new(Cell::new(value))
}

/// The conversation the example opens with. The turns are long enough to wrap at
/// the bubble's cap, so a sent row and a received row both show the shape of
/// their variant, and there are enough of them to overflow the panel.
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
        Turn::received(
            "Appending a newer turn keeps the viewport at the tail only while the \
             reader is already there.",
        ),
        Turn::sent(
            "Earlier history arrives above the anchor, so the turn being read \
             does not move.",
        ),
    ]
}

/// The older turns a "load earlier" prepends above the current transcript.
fn earlier_messages(count: u32) -> Vec<Turn> {
    (0..2)
        .map(|index| {
            Turn::received(format!(
                "Earlier message {} of batch {}: inserted above the transcript and \
                 kept out of the reader's way.",
                index + 1,
                count + 1
            ))
        })
        .collect()
}

#[derive(Debug, Clone)]
struct App {
    /// The turns themselves. The application owns the collection; the component
    /// is told the unread boundary and nothing else.
    turns: Vec<Turn>,
    /// The flag the view's scroll callback writes. `true` while the newest turn
    /// is out of view, which is what the fade and the jump control are drawn
    /// from — the same thing Slint's `scrolled-up` property carries.
    scrolled_up: Rc<Cell<bool>>,
    /// Whether the rule is drawn above the first unseen turn.
    unread: bool,
    /// Whether the scroller draws its built-in jump control.
    jump_button: bool,
    /// How many turns have been appended and prepended, so the buttons say so.
    appended: u32,
    prepended: u32,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// The wheel moved the transcript. `true` means it left the tail.
    Scrolled(bool),
    /// A jump control was pressed. The region named runs `jump_to_end`.
    JumpTo(&'static str),
    /// A new turn arrived at the tail.
    Received,
    /// The reader sent a turn.
    Sent,
    /// Older history was loaded above the transcript.
    Prepend,
    /// Show or hide the unread rule.
    ToggleUnread,
    /// Show or hide the built-in jump control.
    ToggleJumpButton,
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot() -> App {
    App {
        turns: transcript(),
        scrolled_up: signal(false),
        unread: true,
        jump_button: true,
        appended: 0,
        prepended: 0,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Scrolled(scrolled_up) => app.scrolled_up.set(scrolled_up),
        // iced owns the scroll offset, so the component reports that the jump
        // was asked for and the application moves the named viewport. The flag
        // clears here as well as through the next scroll report, so the control
        // leaves the frame at once rather than at the next wheel event.
        Message::JumpTo(id) => {
            app.scrolled_up.set(false);
            return jump_to_end(id);
        }
        Message::Received => {
            app.turns.push(Turn::received(
                "A turn that arrived while you were away. The list follows it \
                 only while you are at the tail.",
            ));
            app.appended += 1;
        }
        Message::Sent => {
            app.turns.push(Turn::sent(
                "A turn you sent. The application appends to the collection and \
                 the scroller keeps the viewport against its end.",
            ));
            app.appended += 1;
        }
        Message::Prepend => {
            let count = app.prepended;
            let mut earlier = earlier_messages(count);
            earlier.extend(app.turns.clone());
            app.turns = earlier;
            app.prepended += 1;
        }
        Message::ToggleUnread => app.unread = !app.unread,
        Message::ToggleJumpButton => app.jump_button = !app.jump_button,
        Message::HoverSource(title) => app.hovered = title,
        Message::PressSource(title) => app.pressed_source = title,
        Message::ShowSource { index, title } => {
            app.pressed_source = None;
            // The page scrolls to the sample; natively this does nothing.
            crate::bridge::show(index, title);
        }
    }
    Task::none()
}

/// A line of text whose colour is resolved from the active tokens when iced
/// renders it, so a caller-owned child reads correctly in either theme.
fn line<'a>(
    value: &'a str,
    size: f32,
    tone: fn(&Tokens) -> Color,
) -> iced::widget::Text<'a, Theme, iced::Renderer> {
    text(value)
        .size(size)
        .style(move |theme: &Theme| iced::widget::text::Style {
            color: Some(tone(&Tokens::of(theme))),
        })
}

/// A chip: one small control the sections use to reach the states the component
/// has, the same shape the page's controls take.
fn chip<'a>(label: impl Into<String>, message: Message) -> Element<'a, Message> {
    button(text(label.into()).size(theme::FONT_SIZE_XS))
        .on_press(message)
        .into()
}

/// The component in a fixed-height box, so a section is a frame the same size as
/// every other one and its scroll region has bounds to resolve against.
fn panel<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(
            container(body)
                .width(Length::Fill)
                .height(Length::Fixed(PANEL_HEIGHT)),
        )
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(text(title).size(theme::FONT_SIZE_SM).font(iced::Font {
            weight: iced::font::Weight::Medium,
            ..iced::Font::DEFAULT
        }))
        .width(Length::Fill)
        .center_x(Length::Fill)
        .style(|theme: &Theme| container::Style {
            text_color: Some(Tokens::of(theme).muted_foreground),
            ..container::Style::default()
        }),
    )
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    let icon = mouse_area(
        container(
            svg(svg::Handle::from_memory(CODE_XML))
                .width(16.0)
                .height(16.0)
                .style(|theme: &Theme, _status| svg::Style {
                    color: Some(Tokens::of(theme).primary),
                }),
        )
        .width(28.0)
        .height(28.0)
        .center_x(28.0)
        .center_y(28.0)
        .style(move |theme: &Theme| {
            let primary = Tokens::of(theme).primary;
            let (fill, edge) = if pressed {
                (0.34, 0.7)
            } else if hovered {
                (0.22, 0.55)
            } else {
                (0.12, 0.35)
            };
            container::Style {
                background: Some(primary.scale_alpha(fill).into()),
                border: iced::border::rounded(theme::RADIUS_LG)
                    .color(primary.scale_alpha(edge))
                    .width(1.0),
                ..container::Style::default()
            }
        }),
    )
    .on_enter(Message::HoverSource(Some(title)))
    .on_exit(Message::HoverSource(None))
    .on_press(Message::PressSource(Some(title)))
    .on_release(Message::ShowSource { index, title });

    row![label, icon]
        .spacing(theme::SPACING_SM)
        .align_y(Axis::Center)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    let scrolled_up = app.scrolled_up.get();
    // The rule belongs above the first turn the reader has not seen. That is the
    // application's meaning, so it is the application that resolves it to an
    // index; the toggle moves it.
    let unread_index = app.unread.then_some(app.turns.len() / 2);

    // section: Create state and choose the starting position
    // iced has no separate state entity: the id names the region, so a caller's
    // own jump control can pass it to `jump_to_end`, and the turns are the
    // application's collection. The starting position is the live tail, because
    // the region anchors to its end while rows arrive; there is no persisted
    // offset and no arbitrary start index to choose.
    let starting = panel(MessageScroller::new(Id::new("starting"), app.turns.clone()));

    // section: Render rows and an empty state
    // A row is a bubble, chosen by role: a sent turn is the muted surface at the
    // trailing edge and a received turn is ghost text at the leading edge, so the
    // component draws the rows itself from the turns. An empty list draws
    // nothing, so the empty state stays with the application beside it.
    let rows = panel(MessageScroller::new(Id::new("rows"), app.turns.clone()));
    let empty = container(
        column![
            line("No messages yet", theme::FONT_SIZE_SM, |tokens| {
                tokens.foreground
            }),
            line(
                "Ask a question to start the thread.",
                theme::FONT_SIZE_XS,
                |tokens| tokens.muted_foreground,
            ),
        ]
        .spacing(theme::SPACING_XS),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill);
    let render = row![rows, empty].spacing(theme::SPACING_MD);

    // section: Append, streaming, and follow-tail behavior
    // Appending while the reader follows the tail keeps the newest row in view;
    // appending after they scroll up preserves their place and raises the fade
    // and the control. iced owns the offset, so the component reports the
    // crossing and the caller feeds it back through `scrolled_up`.
    let tail = column![
        panel(
            MessageScroller::new(Id::new(CONVERSATION), app.turns.clone())
                .scrolled_up(scrolled_up)
                .unread_index(unread_index)
                .on_scroll({
                    let flag = app.scrolled_up.clone();
                    move |scrolled_up| {
                        flag.set(scrolled_up);
                        Message::Scrolled(scrolled_up)
                    }
                })
                .on_jump(Message::JumpTo(CONVERSATION)),
        ),
        row![
            chip(
                format!("received turn ({})", app.appended),
                Message::Received
            ),
            chip("sent turn", Message::Sent),
            line(
                "the newest turn stays in view only while the reader is at the tail",
                theme::FONT_SIZE_XS,
                |tokens| tokens.muted_foreground,
            ),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(Axis::Center),
    ]
    .spacing(theme::SPACING_SM);

    // section: Prepend earlier history
    // Older records are the application's to load and splice in at the front.
    // The region keeps its anchor, so the turns the reader is looking at do not
    // move when earlier ones arrive above them.
    let history = panel(MessageScroller::new(Id::new("history"), app.turns.clone()));
    let prepend = column![
        history,
        chip(
            format!("load earlier ({})", app.prepended),
            Message::Prepend,
        ),
        line(
            "a prepend inserts above the anchor, so the visible turn stays put",
            theme::FONT_SIZE_XS,
            |tokens| tokens.muted_foreground,
        ),
    ]
    .spacing(theme::SPACING_SM);

    // section: Unread and arbitrary navigation
    // The unread rule is the index of the first turn the reader has not seen — a
    // value the application owns, because it is the one that knows an unread,
    // searched or bookmarked identity. iced 0.14 exposes only `jump_to_end`, so
    // arbitrary scroll-to-index has no counterpart here; the rule marks the
    // boundary and the jump returns to the tail.
    let navigation = panel(
        MessageScroller::new(Id::new("navigation"), app.turns.clone()).unread_index(unread_index),
    );
    let navigating = column![
        navigation,
        chip(
            if app.unread {
                "unread rule on"
            } else {
                "unread rule off"
            },
            Message::ToggleUnread,
        ),
    ]
    .spacing(theme::SPACING_SM);

    // section: Jump-to-latest controls
    // The control is drawn while `scrolled_up` is true and reports the press; the
    // caller runs `jump_to_end` on the region's id, because iced owns the offset.
    // Asking for `scrolled_up(true)` here is what draws the control without a
    // scroll, the way the Slint example sets `scrolled-up`.
    let jump = panel(
        MessageScroller::new(Id::new("jump"), app.turns.clone())
            .scrolled_up(true)
            .jump_button(app.jump_button)
            .on_jump(Message::JumpTo("jump")),
    );
    let jumping = column![
        jump,
        chip(
            if app.jump_button {
                "built-in control on"
            } else {
                "built-in control off"
            },
            Message::ToggleJumpButton,
        ),
    ]
    .spacing(theme::SPACING_SM);

    // section: Scrollbar and style slots
    // The builder carries the style slots the page documents: the list inset, the
    // muted bubble's width cap and the scrollbar. A host-drawn control is
    // `jump_button(false)` plus an application button that runs `jump_to_end` on
    // the same id, which is why this frame keeps `scrolled_up` true.
    let slots = panel(
        MessageScroller::new(Id::new("slots"), app.turns.clone())
            .scrolled_up(true)
            .jump_button(false)
            .scrollbar(false)
            .padding(theme::SPACING_LG)
            .max_bubble_width(300.0),
    );
    let latest = Button::new("Latest")
        .variant(ButtonVariant::Secondary)
        .size(ControlSize::Small)
        .on_press(Message::JumpTo("slots"));
    let styled = column![slots, latest].spacing(theme::SPACING_SM);

    // section: end
    let status = text(format!(
        "{} turns · {} appended · {} prepended · scrolled up: {}",
        app.turns.len(),
        app.appended,
        app.prepended,
        scrolled_up
    ))
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| iced::widget::text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, "Create state and choose the starting position"),
        starting,
        heading(app, 1, "Render rows and an empty state"),
        render,
        heading(app, 2, "Append, streaming, and follow-tail behavior"),
        tail,
        heading(app, 3, "Prepend earlier history"),
        prepend,
        heading(app, 4, "Unread and arbitrary navigation"),
        navigating,
        heading(app, 5, "Jump-to-latest controls"),
        jumping,
        heading(app, 6, "Scrollbar and style slots"),
        styled,
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The Lucide mark the code icon uses, from the icon set this repository already
// ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three galleries
// draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
