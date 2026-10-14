//! `message` — the whole chat turn: the identity marker, the sender line, the
//! bubble, its actions and the delivery footer, at both edges and in the ghost
//! and caller-owned surfaces.
//!
//! Run it with `cargo run --example message`, or in the browser as
//! `?story=message` in the gallery the website embeds.
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
//! The titles are `website/component/message.md`'s own sample headings, at the
//! depth the page documents each one: the anatomy, the alignment, the rich
//! content, the grouping, the ghost surfaces and the custom styling are its `##`
//! sections, while avatar, header, content and footer are `###` under "Avatar,
//! header, content, and footer". The Slint example's six `GroupBox`es are folded
//! into them, and the two sections it has no box for — grouping and the ghost
//! inset — are drawn from the page they document.
//!
//! The update type is `Event` rather than the usual `Message` because this
//! component is itself a `Message`: the two names cannot share a scope.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment as Axis, Color, Element, Length, Task, Theme};

use crate::components::avatar::Avatar;
use crate::components::bubble::{Alignment, Content, Variant as BubbleVariant};
use crate::components::button::{Button, Variant as ButtonVariant};
use crate::components::group_box::{self, GroupBox};
use crate::components::message::{Group, Message, Role};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = theme::SPACING_XL;

/// The gap between two turns inside one section.
const TURN_GAP: f32 = theme::SPACING_LG;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — message")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// The one interaction the reference has besides the delivery footer: the
    /// failed turn's `Retry` reports through the message, and the copy action is
    /// an ordinary button the caller owns.
    retries: u32,
    copies: u32,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Event {
    Retry,
    Copy,
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

fn update(app: &mut App, event: Event) -> Task<Event> {
    match event {
        Event::Retry => app.retries += 1,
        Event::Copy => app.copies += 1,
        Event::HoverSource(title) => app.hovered = title,
        Event::PressSource(title) => app.pressed_source = title,
        Event::ShowSource { index, title } => {
            app.pressed_source = None;
            // The page scrolls to the sample; natively this does nothing.
            crate::bridge::show(index, title);
        }
    }
    Task::none()
}

/// A line of text whose colour is resolved from the active tokens when iced
/// renders it, so a caller-owned child inside a bubble reads correctly in either
/// theme. An arbitrary child does not inherit the surface's ink.
fn line<'a>(value: &'a str, size: f32, tone: fn(&Tokens) -> Color) -> Element<'a, Event> {
    text(value)
        .size(size)
        .style(move |theme: &Theme| iced::widget::text::Style {
            color: Some(tone(&Tokens::of(theme))),
        })
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Event> {
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
    .on_press(Event::PressSource(Some(title)))
    .on_release(Event::ShowSource { index, title });

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
    .on_enter(Event::HoverSource(Some(title)))
    .on_exit(Event::HoverSource(None))
    .on_press(Event::PressSource(Some(title)))
    .on_release(Event::ShowSource { index, title });

    row![label, icon]
        .spacing(theme::SPACING_SM)
        .align_y(Axis::Center)
        .into()
}

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Event>>) -> Element<'a, Event> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

fn view(app: &App) -> Element<'_, Event> {
    // section: Anatomy and basic usage
    // The whole row in its shortest complete form: a sender line, the bubble,
    // and a footer under both. A received turn is the default; the sent one
    // names its role.
    let anatomy = column![
        Message::new()
            .author("Alice")
            .time("10:24 AM")
            .initials("AL")
            .body("Can you review this?")
            .footer("Read"),
        Message::new()
            .role(Role::User)
            .author("You")
            .time("10:25 AM")
            .initials("YO")
            .body("Sure — I will send notes shortly.")
            .footer("Delivered"),
    ]
    .spacing(TURN_GAP)
    .width(Length::Fill);

    // section: Alignment
    // `user` turns sit at the trailing edge and take the primary surface;
    // everything else starts at the leading edge on the secondary one. The role
    // picks the side, and `alignment` can override it.
    let alignment = column![
        Message::new()
            .author("Support")
            .time("10:42 AM")
            .initials("ST")
            .body("Your issue has been assigned to the team."),
        Message::new()
            .role(Role::User)
            .author("You")
            .time("Just now")
            .initials("YO")
            .body("Thank you for the quick update.")
            .footer("Delivered"),
    ]
    .spacing(TURN_GAP)
    .width(Length::Fill);

    // section: Avatar
    // Initials stand in whenever there is no image. A caller passes its own
    // `Avatar` through the slot, and a turn with no identity column pulls its
    // text to the edge instead.
    let avatar = column![
        Message::new()
            .author("Alice Chen")
            .initials("AC")
            .body("Initials stand in whenever there is no image."),
        Message::new()
            .author("Jordan Park")
            .initials("JP")
            .body("A turn with no avatar slot pulls its text to the edge instead.")
            .show_avatar(false),
        Message::new()
            .author("Casey Lee")
            .avatar(Avatar::new().initials("CL"))
            .body("The caller's own Avatar can fill the slot."),
    ]
    .spacing(TURN_GAP)
    .width(Length::Fill);

    // section: Header
    // The header carries the sender and the timestamp, and lines up with the
    // bubble rather than with the avatar.
    let header = column![
        Message::new()
            .author("Alice Chen")
            .time("10:24 AM")
            .initials("AC")
            .body("The header carries the sender and the timestamp."),
        Message::new()
            .role(Role::User)
            .author("You")
            .time("10:25 AM")
            .initials("YO")
            .body("And it lines up with the bubble, not with the avatar."),
    ]
    .spacing(TURN_GAP)
    .width(Length::Fill);

    // section: Content
    // The body is the bubble's text; `child` composes arbitrary rich content
    // into the same surface, and the application colours what it adds.
    let content = column![
        Message::new()
            .author("Alice")
            .initials("AL")
            .body("The export is ready.")
            .child(
                row![
                    line("export.zip", theme::FONT_SIZE_SM, |tokens| tokens
                        .foreground),
                    line("2.4 MB · ZIP", theme::FONT_SIZE_XS, |tokens| {
                        tokens.muted_foreground
                    }),
                ]
                .spacing(theme::SPACING_SM)
                .align_y(Axis::Center),
            )
            .footer("Read"),
        Message::new()
            .role(Role::User)
            .author("You")
            .initials("YO")
            .body("Send it over.")
            .footer("Delivered"),
    ]
    .spacing(TURN_GAP)
    .width(Length::Fill);

    // section: Footer
    // A footer states delivery; a failed turn says so and draws its `Retry` once
    // the caller supplies the message it reports.
    let footer = column![
        Message::new()
            .role(Role::User)
            .initials("YO")
            .body("This one was delivered.")
            .footer("Delivered"),
        Message::new()
            .role(Role::User)
            .initials("YO")
            .body("The response could not be sent.")
            .failed(true)
            .on_retry(Event::Retry),
    ]
    .spacing(TURN_GAP)
    .width(Length::Fill);

    // section: Rich content and actions
    // The actions are the caller's: the message reserves the row, and each
    // control carries its own message. A rich child sits in the bubble above it.
    let rich = Message::new()
        .author("Support")
        .time("10:42 AM")
        .initials("ST")
        .body("The install failure is coming from the workspace package.")
        .child(line(
            "workspace-package@0.14.2",
            theme::FONT_SIZE_XS,
            |tokens| tokens.muted_foreground,
        ))
        .action(
            Button::new("Copy")
                .variant(ButtonVariant::Ghost)
                .size(ControlSize::XSmall)
                .on_press(Event::Copy),
        )
        .action(
            Button::new("Retry")
                .variant(ButtonVariant::Ghost)
                .size(ControlSize::XSmall)
                .on_press(Event::Retry),
        )
        .footer("Read");

    // section: Grouping
    // `Group` is a plain vertical stack: the application decides which turns
    // belong to one sender and which edge the stack sits on.
    let grouping = column![
        Group::new()
            .child(
                Message::new()
                    .author("Alice")
                    .initials("AL")
                    .body("Can you tell me what changed?"),
            )
            .child(
                Message::new()
                    .author("Alice")
                    .initials("AL")
                    .body("The registry route was stale."),
            ),
        Group::new()
            .alignment(Alignment::End)
            .child(
                Message::new()
                    .role(Role::User)
                    .author("You")
                    .initials("YO")
                    .body("I removed the stale route."),
            )
            .child(
                Message::new()
                    .role(Role::User)
                    .author("You")
                    .initials("YO")
                    .body("The updated registry is ready to review."),
            ),
    ]
    .spacing(theme::SPACING_XL)
    .width(Length::Fill);

    // section: Ghost surfaces and content insets
    // A ghost turn has no surface to align with, so its metadata drops the
    // regular inset; the caller can keep it or remove it explicitly.
    let ghost = column![
        Message::new()
            .author("System")
            .time("Just now")
            .initials("SY")
            .body("The conversation has been archived.")
            .bubble_variant(BubbleVariant::Ghost)
            .footer("No further action required"),
        Message::new()
            .author("System")
            .initials("SY")
            .body("This ghost turn keeps the inset with the regular surface.")
            .bubble_variant(BubbleVariant::Ghost)
            .content_inset(true),
        Message::new()
            .author("Alice")
            .initials("AL")
            .body("This regular turn removes the inset the caller owns.")
            .content_inset(false),
    ]
    .spacing(TURN_GAP)
    .width(Length::Fill);

    // section: Custom styling and theme tokens
    // The surface is the bubble's own `Content`, so the fill, ink and edge come
    // from semantic tokens resolved inside the style closure — the same route
    // the page documents for a caller-owned message surface.
    let custom = Message::new()
        .author("System")
        .time("Just now")
        .initials("SY")
        .body("A message on a caller-owned surface.")
        .bubble_content(
            Content::new()
                .fill(|tokens| theme::alpha(tokens.success, 0.15))
                .text_color(|tokens| tokens.success)
                .border_color(|tokens| theme::alpha(tokens.success, 0.35))
                .border_width(1.0)
                .radius(theme::RADIUS_LG),
        )
        .footer("Delivered");

    // section: end
    let status = text(format!(
        "Retries: {} · Copy requested: {}",
        app.retries, app.copies
    ));

    let gallery = column![
        heading(app, 0, "Anatomy and basic usage"),
        body_box(anatomy),
        heading(app, 1, "Alignment"),
        body_box(alignment),
        heading(app, 2, "Avatar"),
        body_box(avatar),
        heading(app, 3, "Header"),
        body_box(header),
        heading(app, 4, "Content"),
        body_box(content),
        heading(app, 5, "Footer"),
        body_box(footer),
        heading(app, 6, "Rich content and actions"),
        body_box(rich),
        heading(app, 7, "Grouping"),
        body_box(grouping),
        heading(app, 8, "Ghost surfaces and content insets"),
        body_box(ghost),
        heading(app, 9, "Custom styling and theme tokens"),
        body_box(custom),
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
