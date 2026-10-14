//! `card` — a bordered content surface with a badge above a title, a
//! description, and a body the caller fills, in the six samples the Slint
//! example and the component page document: Default, Stacked, Login, Size,
//! Spacing and Image.
//!
//! Run it with `cargo run --example card`, or in the browser as `?story=card`
//! in the gallery the website embeds.
//!
//! The Slint example's six outlined sections are the six here, in the same
//! order, each under the `###` heading `website/component/card.md` documents it
//! at. The `// section: <title>` comments below are the contract the website
//! reads to cut this file into the usage panels a component page shows, so the
//! code on the page is the code that runs here. Keep them, one per section, in
//! order, and keep the closing `// section: end` after the last one.
//!
//! Each heading carries the title and the code icon that asks the embedding
//! page to scroll to that sample. Both are the button, as in Slint's
//! `SourceHeading`, and a click posts the section's index and title through
//! [`crate::bridge`], the same bridge the Slint and GPUI galleries use.
//!
//! Where iced differs from Slint the difference is stated rather than faked.
//! The Slint card has a `cover` image slot above the header and a free `inset`;
//! this one draws the header first and derives the inset from `size` — 16px at
//! the medium size, 12px at the small size — so the Image sample passes the
//! caller's media element as the body and says so, and Spacing shows the two
//! insets the component has rather than four it cannot.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::{Button, Variant};
use crate::components::card::{self, Card};
use crate::components::group_box::{self, GroupBox};
use crate::components::input::Input;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets
/// them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's `###` headings — have them. The index is what the page is
/// asked to scroll to.
const SECTIONS: [&str; 6] = ["Default", "Stacked", "Login", "Size", "Spacing", "Image"];

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — card")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// The login field's value: iced has no input state to own it, so the caller
    /// holds it and takes every edit back through `on_input`.
    email: String,
    /// What the sample actions last did, reported after `// section: end`.
    saved: u32,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Email(String),
    Saved,
    Nothing,
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
        Message::Email(value) => app.email = value,
        Message::Saved => app.saved += 1,
        Message::Nothing => {}
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

/// A muted line: the supporting copy the Slint card draws in the muted
/// foreground.
fn muted<'a>(value: impl Into<String>) -> Element<'a, Message> {
    text(value.into())
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A stand-in for the Slint card's `cover` image: the caller's own media
/// element, which an application fills with `image(handle)` or `svg(handle)`.
/// The Slint card draws it above the header; this one has no slot there, so the
/// sample passes it as the body instead.
fn cover<'a>(caption: &'static str) -> Element<'a, Message> {
    container(
        text(caption)
            .size(11.0)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    )
    .width(Length::Fill)
    .height(160.0)
    .align_x(Alignment::Center)
    .align_y(Alignment::Center)
    .style(|theme: &Theme| container::Style {
        background: Some(Tokens::of(theme).muted.into()),
        border: iced::border::rounded(theme::RADIUS_MD),
        ..container::Style::default()
    })
    .into()
}

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box the other galleries draw.
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
        .spacing(theme::SPACING_SM)
        .align_y(Alignment::Center)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Default
    // The page's first documented sample and the Slint example's first
    // `GroupBox`: a header, a body, and an end-aligned footer of actions.
    let default = body_box(
        Card::new()
            .title("Team")
            .description("Invite members and set their roles.")
            .body(
                column![
                    muted("Twelve seats remaining on this workspace."),
                    card::footer(
                        row![
                            Button::new("Cancel")
                                .variant(Variant::Ghost)
                                .on_press(Message::Nothing),
                            Button::new("Invite")
                                .variant(Variant::Primary)
                                .on_press(Message::Saved),
                        ]
                        .spacing(theme::SPACING_SM),
                    ),
                ]
                .spacing(theme::SPACING_LG),
            ),
    );

    // section: Stacked
    // Two cards side by side, each filling its half: the Slint example's
    // `horizontal-stretch: 1` pair.
    let stacked = body_box(
        row![
            Card::new()
                .title("Usage")
                .body(muted("4.2 GB of 10 GB used.")),
            Card::new()
                .title("Plan")
                .body(muted("Team · billed monthly.")),
        ]
        .spacing(theme::SPACING_LG),
    );

    // section: Login
    // A field and two actions in the card body: the Slint example's login form,
    // with the input's placeholder and a link variant for the secondary action.
    let login = body_box(
        Card::new()
            .title("Login to your account")
            .description("Enter your email below to login to your account")
            .body(
                column![
                    Input::new("card-login-email")
                        .placeholder("m@example.com")
                        .value(app.email.clone())
                        .width(Length::Fill)
                        .on_input(Message::Email),
                    Button::new("Forgot your password?")
                        .variant(Variant::Link)
                        .on_press(Message::Nothing),
                    Button::new("Login")
                        .variant(Variant::Primary)
                        .width(Length::Fill)
                        .on_press(Message::Saved),
                ]
                .spacing(theme::SPACING_MD),
            ),
    );

    // section: Size
    // The two sizes: medium keeps the 16px inset and the text-md title, small
    // tightens both.
    let size = body_box(
        column![
            Card::new()
                .title("Default")
                .description("The standard inset and title size."),
            Card::new()
                .title("Small")
                .description("A compact card for dense layouts.")
                .size(ControlSize::Small),
        ]
        .spacing(theme::SPACING_LG),
    );

    // section: Spacing
    // The inset is the card's padding and the gap between the header, the body
    // and the footer at once. Slint lets the sample set it freely; this
    // component derives it from `size`, so the sample shows the two it has —
    // 16px at the medium size and 12px at the small size — rather than four it
    // cannot.
    let spacing = body_box(
        column![
            Card::new()
                .title("16 px")
                .description("The medium inset and the gap between sections.")
                .body(muted("A body at the same inset.")),
            Card::new()
                .title("12 px")
                .description("The small inset and the gap between sections.")
                .size(ControlSize::Small)
                .body(muted("A body at the same inset.")),
        ]
        .spacing(theme::SPACING_LG),
    );

    // section: Image
    // Slint's card has a `cover` image slot above the header. This one draws
    // the header first and has no slot above it, so the caller's media element
    // is the body — an application passes `image(handle)` or `svg(handle)`
    // here — and the badge rides above the title as it does in Slint.
    let image = body_box(
        Card::new()
            .badge("Featured")
            .title("Design systems meetup")
            .description("A practical talk on component APIs and shipping faster.")
            .body(cover("attachment-preview.svg")),
    );

    // section: end
    // The status line the sample actions report to, out of the samples.
    let status = text(format!("saved {} times", app.saved)).size(theme::FONT_SIZE_SM);

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        default,
        heading(app, 1, SECTIONS[1]),
        stacked,
        heading(app, 2, SECTIONS[2]),
        login,
        heading(app, 3, SECTIONS[3]),
        size,
        heading(app, 4, SECTIONS[4]),
        spacing,
        heading(app, 5, SECTIONS[5]),
        image,
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The page itself is a control, so a press outside a panel lands on a widget
    // rather than on the bare window.
    container(scrollable(gallery).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            ..container::Style::default()
        })
        .into()
}

// The Lucide mark the code icon uses, from the icon set this repository already
// ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three galleries
// draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
