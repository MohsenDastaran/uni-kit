//! `notification` — cards that report a status, stacked in a corner of the
//! frame they are given.
//!
//! Run it with `cargo run --example notification`, or in the browser as
//! `?story=notification` in the gallery the website embeds.
//!
//! Each section carries a heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! The seven headings are the Slint example's GroupBox titles, all of which the
//! component page already has: **Basic Notification**, **Notification Types**,
//! **Auto-hide Control**, **With Action Button**, **Notification with Title**,
//! **Placement** and **Clickable Notifications**. The rest of `notification.md`
//! describes GPUI's window-level host — `push_notification`, auto-hide timers,
//! unique ids, system delivery, a markdown content builder — which iced 0.14 has
//! no equivalent for; the component names those gaps in its own module rather
//! than faking them here.

use iced::widget::{column, container, row, scrollable, stack, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::{Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::notification::{Notification, NotificationLayer, Placement, Status};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The height of the framed surface a layer is anchored in, as the Slint example
/// gives each one.
const FRAME_HEIGHT: f32 = 300.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — notification")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// The anchor the placement section's stack is drawn at.
    placement: Placement,
    /// Whether the placement section's stack has a notice in it.
    placed: bool,
    /// The status of the notice the clickable section raised, if any.
    raised: Option<Status>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            placement: Placement::default(),
            placed: false,
            raised: None,
            hovered: None,
            pressed_source: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    SetPlacement(Placement),
    ShowPlaced,
    ClearPlaced,
    /// Raise the clickable section's notice, replacing any that is showing.
    Raise(Status),
    /// The raised notice's dismiss control was pressed.
    DismissRaised,
    /// A static card's dismiss control. The Slint example draws the control on
    /// these cards and leaves `closed` unwired, so pressing it is answered with
    /// a message that changes nothing; only the clickable section's card owns a
    /// list to remove itself from.
    DismissStatic,
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::SetPlacement(placement) => app.placement = placement,
        Message::ShowPlaced => app.placed = true,
        Message::ClearPlaced => app.placed = false,
        Message::Raise(status) => app.raised = Some(status),
        Message::DismissRaised => app.raised = None,
        Message::DismissStatic => {}
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

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = iced::widget::mouse_area(
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

    let icon = iced::widget::mouse_area(
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
            let (fill, line) = if pressed {
                (0.34, 0.7)
            } else if hovered {
                (0.22, 0.55)
            } else {
                (0.12, 0.35)
            };
            container::Style {
                background: Some(primary.scale_alpha(fill).into()),
                border: iced::border::rounded(theme::RADIUS_LG)
                    .color(primary.scale_alpha(line))
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
        .spacing(8.0)
        .align_y(Alignment::Center)
        .into()
}

/// The box under a heading: the same `GroupBox` with the outline variant the Slint
/// example wraps its sections in, untitled because the heading above it carries the
/// title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// The bounded surface a layer anchors inside, the way the Slint example gives
/// each stack a clipped rectangle. It is the caller's frame: the layer states its
/// own size and packs the stack into a corner of whatever holds it.
///
/// The frame is a fixed-height container, so a `Fill`-height layer inside it
/// resolves against the frame rather than against the scroll area's content.
fn framed<'a>(body: impl Into<Element<'a, Message>>, height: f32) -> Element<'a, Message> {
    container(body)
        .width(Length::Fill)
        .height(Length::Fixed(height))
        .style(|theme: &Theme| {
            let tokens = Tokens::of(theme);
            container::Style {
                background: Some(theme::alpha(tokens.muted, 0.3).into()),
                border: iced::Border {
                    color: tokens.border,
                    width: 1.0,
                    radius: theme::RADIUS_LG.into(),
                },
                ..container::Style::default()
            }
        })
        .into()
}

/// A hint centered in a frame, so an empty framed surface still says what it is.
fn empty_hint<'a>(hint: &'static str) -> Element<'a, Message> {
    container(
        text(hint)
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic Notification
    // The whole card: the status mark, a title and a message. The dismiss
    // control is drawn because the card is closable, which is the Slint default.
    let basic = Notification::new()
        .title("Saved")
        .message("Your changes are on the server.")
        .on_close(Message::DismissStatic);

    // section: Notification Types
    // `status` picks the glyph and the colour; the layout is the same for all
    // four.
    let types = column![
        Notification::new()
            .status(Status::Info)
            .title("Update available")
            .message("Version 0.6.5 is ready to install.")
            .on_close(Message::DismissStatic),
        Notification::new()
            .status(Status::Success)
            .title("Deployed")
            .message("The build is live.")
            .on_close(Message::DismissStatic),
        Notification::new()
            .status(Status::Warning)
            .title("Slow request")
            .message("The last query took 4.2s.")
            .on_close(Message::DismissStatic),
        Notification::new()
            .status(Status::Danger)
            .title("Upload failed")
            .message("The connection dropped.")
            .on_close(Message::DismissStatic),
    ]
    .spacing(theme::SPACING_MD)
    .align_x(Alignment::Start);

    // section: Auto-hide Control
    // A closable card carries a dismiss control; a sticky one does not, and the
    // application decides when it goes.
    let timing = column![
        Notification::new()
            .title("Closable")
            .message("This one has a dismiss button.")
            .on_close(Message::DismissStatic),
        Notification::new()
            .title("Sticky")
            .message("No dismiss button; the application decides.")
            .closable(false),
    ]
    .spacing(theme::SPACING_MD)
    .align_x(Alignment::Start);

    // section: With Action Button
    // A notice and the controls for it. Slint places the action row under the
    // card, so the caller owns it here too.
    let action = column![
        Notification::new()
            .status(Status::Warning)
            .title("Unsaved changes")
            .message("Leaving now discards the draft.")
            .on_close(Message::DismissStatic),
        row![
            Button::new("Discard")
                .outline()
                .on_press(Message::DismissStatic),
            Button::new("Keep editing")
                .variant(Variant::Primary)
                .on_press(Message::DismissStatic),
        ]
        .spacing(theme::SPACING_SM),
    ]
    .spacing(theme::SPACING_MD)
    .align_x(Alignment::Start);

    // section: Notification with Title
    // A title with no message: the body column simply omits the line.
    let titled = Notification::new()
        .title("Copied to clipboard")
        .on_close(Message::DismissStatic);

    // section: Placement
    // The six anchors. Pick one and press Show: the stack is packed against that
    // corner of the frame.
    let anchors = [
        (Placement::TopLeft, "Top left"),
        (Placement::TopCenter, "Top centre"),
        (Placement::TopRight, "Top right"),
        (Placement::BottomLeft, "Bottom left"),
        (Placement::BottomCenter, "Bottom centre"),
        (Placement::BottomRight, "Bottom right"),
    ];
    let picker = row(anchors.map(|(placement, label)| -> Element<'_, Message> {
        Button::new(label)
            .size(theme::ControlSize::XSmall)
            .selected(app.placement == placement)
            .on_press(Message::SetPlacement(placement))
            .into()
    }))
    .spacing(theme::SPACING_XS);

    let placed_layer: Element<'_, Message> = if app.placed {
        NotificationLayer::new()
            .placement(app.placement)
            .item(
                Notification::new()
                    .title("Placed")
                    .message("The stack sits against the chosen corner.")
                    .on_close(Message::DismissStatic),
            )
            .into()
    } else {
        NotificationLayer::new().placement(app.placement).into()
    };

    let placement = column![
        picker,
        row![
            Button::new("Show")
                .variant(Variant::Primary)
                .on_press(Message::ShowPlaced),
            Button::new("Clear").on_press(Message::ClearPlaced),
        ]
        .spacing(theme::SPACING_SM),
        // The layer is always mounted, so the frame keeps its shape and the
        // stack appears at the corner the buttons name.
        framed(placed_layer, FRAME_HEIGHT),
        text("The frame is the caller's; the layer fills it and packs the stack into one corner.")
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    ]
    .spacing(theme::SPACING_MD)
    .align_x(Alignment::Start);

    // section: Clickable Notifications
    // A button raises a notice, and the notice's dismiss control removes it from
    // the caller's list.
    let raisers = [
        (Status::Success, "Success"),
        (Status::Info, "Info"),
        (Status::Warning, "Warning"),
        (Status::Danger, "Error"),
    ];
    let raiser_buttons = row(
        raisers.map(|(status, label)| -> Element<'_, Message> {
            Button::new(label).on_press(Message::Raise(status)).into()
        }),
    )
    .spacing(theme::SPACING_SM);

    let raised_layer: Element<'_, Message> = match app.raised {
        Some(status) => NotificationLayer::new()
            .item(
                Notification::new()
                    .status(status)
                    .title(status_name(status))
                    .message(status_message(status))
                    .on_close(Message::DismissRaised),
            )
            .into(),
        None => iced::widget::Space::new().into(),
    };

    let clickable = column![
        raiser_buttons,
        framed(
            stack![empty_hint("Press a button to raise a notification."), raised_layer],
            FRAME_HEIGHT
        ),
        text("Press a button to raise a notice; press its dismiss control to take it down.")
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    ]
    .spacing(theme::SPACING_MD)
    .align_x(Alignment::Start);

    // section: end
    let status = text("Press a heading or its code icon — the page scrolls to that sample.");

    let gallery = column![
        heading(app, 0, "Basic Notification"),
        body_box(basic),
        heading(app, 1, "Notification Types"),
        body_box(types),
        heading(app, 2, "Auto-hide Control"),
        body_box(timing),
        heading(app, 3, "With Action Button"),
        body_box(action),
        heading(app, 4, "Notification with Title"),
        body_box(titled),
        heading(app, 5, "Placement"),
        body_box(placement),
        heading(app, 6, "Clickable Notifications"),
        body_box(clickable),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

/// The title a raised notice carries, matching the Slint example's button labels.
fn status_name(status: Status) -> &'static str {
    match status {
        Status::Default | Status::Info => "Update available",
        Status::Success => "Deployed",
        Status::Warning => "Slow request",
        Status::Danger => "Upload failed",
    }
}

/// The message a raised notice carries, matching the Slint example's.
fn status_message(status: Status) -> &'static str {
    match status {
        Status::Default | Status::Info => "Version 0.6.5 is ready.",
        Status::Success => "The build is live.",
        Status::Warning => "The last query took 4.2s.",
        Status::Danger => "The connection dropped.",
    }
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
