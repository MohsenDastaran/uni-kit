//! `collapsible` — a single expandable panel, shown the way the Slint example
//! shows it: a trigger row with a chevron, a content panel underneath, and the
//! caller's own `open` state deciding which is drawn.
//!
//! Run it with `cargo run --example collapsible`, or in the browser as
//! `?story=collapsible` in the gallery the website embeds.
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
//! The sections are the Slint example's `GroupBox` headings, in its order: the
//! titled trigger, the row that is itself the trigger, the bottom-edge trigger,
//! and the panels a trigger beside or above a card opens. **Animated reveal**
//! comes first and is the page's own section rather than a Slint GroupBox: it is
//! the one place the panel is built with `animated(true)`, so the expansion the
//! component documents can be watched rather than read about. The others stay
//! immediate, which is what a caller gets by default.

use iced::widget::{
    Space, button as raw_button, column, container, mouse_area, row, scrollable, svg, text,
};
use iced::{Alignment, Background, Border, Color, Element, Length, Padding, Task, Theme};

use crate::components::avatar::{Avatar, AvatarSize};
use crate::components::button::{self, Button, Variant};
use crate::components::checkbox::Checkbox;
use crate::components::collapsible::{self, Collapsible};
use crate::components::group_box::{self, GroupBox};
use crate::components::progress::Progress;
use crate::components::tag::{Tag, Variant as TagVariant};
use crate::components::theme::{self, ControlSize, Status, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — collapsible")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// One flag per panel, because an iced control carries no state: the caller
    /// holds `open` and takes every toggle back as a message.
    basic_open: bool,
    /// The one panel built with `animated(true)`, so the reveal it shows is the
    /// component's animation rather than a still.
    animated_open: bool,
    row_open: bool,
    usage_open: bool,
    settings_open: bool,
    keys_open: bool,
    components_open: bool,
    ui_open: bool,
    profile_open: bool,
    advanced_open: bool,
    push: bool,
    email: bool,
    sms: bool,
    /// The last control pressed that only reports, so a click is visible.
    notice: Option<&'static str>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleBasic,
    ToggleAnimated,
    ToggleRow,
    ToggleUsage,
    ToggleSettings,
    ToggleKeys,
    ToggleComponents,
    ToggleUi,
    ToggleProfile,
    ToggleAdvanced,
    Push(bool),
    Email(bool),
    Sms(bool),
    Notice(&'static str),
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
        settings_open: true,
        keys_open: true,
        components_open: true,
        profile_open: true,
        advanced_open: true,
        push: true,
        ..App::default()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::ToggleBasic => app.basic_open = !app.basic_open,
        Message::ToggleAnimated => app.animated_open = !app.animated_open,
        Message::ToggleRow => app.row_open = !app.row_open,
        Message::ToggleUsage => app.usage_open = !app.usage_open,
        Message::ToggleSettings => app.settings_open = !app.settings_open,
        Message::ToggleKeys => app.keys_open = !app.keys_open,
        Message::ToggleComponents => app.components_open = !app.components_open,
        Message::ToggleUi => app.ui_open = !app.ui_open,
        Message::ToggleProfile => app.profile_open = !app.profile_open,
        Message::ToggleAdvanced => app.advanced_open = !app.advanced_open,
        Message::Push(value) => app.push = value,
        Message::Email(value) => app.email = value,
        Message::Sms(value) => app.sms = value,
        Message::Notice(label) => app.notice = Some(label),
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

// ------------------------------------------------------------------- pieces

/// A mark from the icon set this repository ships (`crates/slint-component/ui/icons`,
/// Lucide, ISC), so the two galleries draw the same glyphs. The ink is resolved
/// inside the style closure, because the theme is only known when iced asks —
/// an unresolved colour is the trap that once drew a dark icon as black.
fn mark<'a>(markup: &'static [u8], size: f32, color: fn(&Theme) -> Color) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(size)
        .height(size)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(color(theme)),
        })
        .into()
}

/// The muted ink most leading icons draw in.
fn muted(theme: &Theme) -> Color {
    Tokens::of(theme).muted_foreground
}

/// The ink a ghost icon control draws its label in.
fn ghost_ink(theme: &Theme) -> Color {
    button::ink(Tokens::of(theme), Variant::Ghost, false)
}

/// A line of text in one of the semantic inks. It is the `Text` itself rather
/// than an element, so a caller can give it `Length::Fill` in a row.
fn line<'a>(
    value: &'a str,
    size: f32,
    color: fn(&Theme) -> Color,
) -> iced::widget::Text<'a, Theme, iced::Renderer> {
    text(value)
        .size(size)
        .style(move |theme: &Theme| text::Style {
            color: Some(color(theme)),
        })
}

/// A hairline the width of its column, for the rows a panel separates.
fn rule<'a>() -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).border.into()),
            ..container::Style::default()
        })
        .into()
}

/// A bordered surface, the shape both examples put their panel content in.
fn card<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(body)
        .width(Length::Fill)
        .padding(8.0)
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: Tokens::of(theme).border,
                width: 1.0,
                radius: theme::RADIUS_LG.into(),
            },
            ..container::Style::default()
        })
        .into()
}

/// [`card`] on the muted surface, which is how the example shades a summary.
fn soft_card<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(body)
        .width(Length::Fill)
        .padding(iced::Padding {
            top: 8.0,
            right: 12.0,
            bottom: 8.0,
            left: 12.0,
        })
        .style(|theme: &Theme| {
            let tokens = Tokens::of(theme);
            container::Style {
                background: Some(tokens.muted.scale_alpha(0.4).into()),
                border: Border {
                    color: tokens.border,
                    width: 1.0,
                    radius: theme::RADIUS_LG.into(),
                },
                ..container::Style::default()
            }
        })
        .into()
}

/// The trigger is an element, so this is what a caller writes when the row is
/// its own control rather than the component's built-in ghost trigger: an accent
/// fill under the pointer, and the caller's message on release.
fn pressable<'a>(
    content: impl Into<Element<'a, Message>>,
    message: Message,
) -> Element<'a, Message> {
    raw_button(content)
        .width(Length::Fill)
        .padding(0)
        .style(|theme: &Theme, status| {
            let tokens = Tokens::of(theme);
            let hovered = matches!(
                status,
                raw_button::Status::Hovered | raw_button::Status::Pressed
            );
            raw_button::Style {
                background: Some(Background::Color(if hovered {
                    tokens.accent
                } else {
                    Color::TRANSPARENT
                })),
                text_color: tokens.foreground,
                border: Border {
                    radius: theme::RADIUS_MD.into(),
                    ..Border::default()
                },
                ..raw_button::Style::default()
            }
        })
        .on_press(message)
        .into()
}

/// A ghost extra-small icon control: 20px square with no fill of its own, the
/// shape the Slint example uses for `plus`, `ellipsis` and the order chevrons.
fn icon_button<'a>(markup: &'static [u8], message: Message) -> Button<'a, Message> {
    Button::new("")
        .variant(Variant::Ghost)
        .size(ControlSize::XSmall)
        .icon(mark(markup, 12.0, ghost_ink))
        .on_press(message)
}

/// The chevron an open panel points down and a closed one points right, as the
/// sliding panels in the example draw it.
fn chevron<'a>(open: bool) -> Element<'a, Message> {
    mark(if open { CHEVRON_DOWN } else { CHEVRON_RIGHT }, 14.0, muted)
}

/// A row that is the whole trigger, with its chevron leading the label.
fn trigger_row<'a>(open: bool, label: &'a str, message: Message) -> Element<'a, Message> {
    pressable(
        container(
            row![
                chevron(open),
                line(label, theme::FONT_SIZE_SM, |theme| {
                    Tokens::of(theme).foreground
                })
            ]
            .spacing(8.0)
            .align_y(Alignment::Center),
        )
        .padding([6.0, 10.0])
        .width(Length::Fill),
        message,
    )
}

/// A folder row in the nested tree: the chevron, the folder, and the name, with
/// the depth carried by `left`.
fn folder_row<'a>(open: bool, label: &'a str, left: f32, message: Message) -> Element<'a, Message> {
    pressable(
        container(
            row![
                chevron(open),
                mark(if open { FOLDER_OPEN } else { FOLDER }, 14.0, muted),
                line(label, theme::FONT_SIZE_SM, |theme| Tokens::of(theme)
                    .foreground),
            ]
            .spacing(8.0)
            .align_y(Alignment::Center),
        )
        .padding(Padding {
            top: 4.0,
            right: 8.0,
            bottom: 4.0,
            left,
        })
        .width(Length::Fill),
        message,
    )
}

/// A file row in the nested tree, which is a label rather than a control.
fn file_row<'a>(label: &'a str, left: f32) -> Element<'a, Message> {
    container(
        row![
            mark(FILE, 14.0, muted),
            line(label, theme::FONT_SIZE_SM, |theme| Tokens::of(theme)
                .foreground),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    )
    .padding(Padding {
        top: 4.0,
        right: 8.0,
        bottom: 4.0,
        left,
    })
    .width(Length::Fill)
    .into()
}

/// A label/value line in the profile panel, which keeps its two columns.
fn detail_line<'a>(markup: &'static [u8], label: &'a str, value: &'a str) -> Element<'a, Message> {
    row![
        mark(markup, 14.0, muted),
        line(label, theme::FONT_SIZE_XS, muted),
        line(value, theme::FONT_SIZE_XS, |theme| Tokens::of(theme)
            .foreground),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center)
    .into()
}

/// One row of an API key: the chip, the environment name, the key itself, and
/// the row's actions.
fn key_row<'a>(environment: &'a str, key: &'a str) -> Element<'a, Message> {
    row![
        container(mark(HASH, 12.0, |theme| Tokens::of(theme).chart_2))
            .width(20.0)
            .height(20.0)
            .center_x(20.0)
            .center_y(20.0)
            .style(|theme: &Theme| container::Style {
                background: Some(Tokens::of(theme).muted.into()),
                border: Border {
                    radius: theme::RADIUS_SM.into(),
                    ..Border::default()
                },
                ..container::Style::default()
            }),
        line(environment, theme::FONT_SIZE_XS, |theme| Tokens::of(theme)
            .foreground),
        container(line(key, theme::FONT_SIZE_XS, |theme| Tokens::of(theme)
            .foreground))
        .width(Length::Fill)
        .padding([2.0, 6.0])
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).muted.into()),
            border: Border {
                radius: theme::RADIUS_SM.into(),
                ..Border::default()
            },
            ..container::Style::default()
        }),
        icon_button(ELLIPSIS, Message::Notice("Key actions")),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center)
    .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon is
/// the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and 0.55
/// on hover, 0.34 and 0.7 while pressed.
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

// --------------------------------------------------------------------- view

fn view(app: &App) -> Element<'_, Message> {
    // section: Animated reveal
    // `animated(true)` measures the panel and moves it from nothing to its full
    // height over 150ms — the duration the Slint component gives `animate
    // height`. The content stays mounted while closed, so pressing the trigger
    // again mid-flight reverses the reveal from the height it has already
    // reached instead of restarting it.
    let animated = Collapsible::new()
        .trigger(collapsible::trigger(
            "Advanced options",
            app.animated_open,
            Message::ToggleAnimated,
        ))
        .open(app.animated_open)
        .animated(true)
        .content(
            column![
                line(
                    "The panel is measured from its content, then clipped to \
                     however far the reveal has come.",
                    theme::FONT_SIZE_SM,
                    muted,
                ),
                line(
                    "Toggle it again while it moves and it turns around from \
                     where it is.",
                    theme::FONT_SIZE_SM,
                    muted,
                ),
            ]
            .spacing(8.0),
        );

    // section: Basic
    // The trigger sits beside the title and the summary stays visible: the ghost
    // chevron button is the panel's trigger, and the status row never moves.
    let basic_header = row![
        text("Order #4189")
            .size(theme::FONT_SIZE_SM)
            .font(iced::Font {
                weight: iced::font::Weight::Semibold,
                ..iced::Font::DEFAULT
            })
            .width(Length::Fill)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
        icon_button(CHEVRONS_UP_DOWN, Message::ToggleBasic),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    let basic = column![
        basic_header,
        soft_card(
            row![
                line("Status", theme::FONT_SIZE_SM, muted).width(Length::Fill),
                Tag::new("Shipped")
                    .variant(TagVariant::Status)
                    .status(Status::Success)
                    .size(ControlSize::Small),
            ]
            .spacing(8.0)
            .align_y(Alignment::Center),
        ),
        Collapsible::new().open(app.basic_open).content(
            column![
                card(
                    column![
                        line("Shipping address", theme::FONT_SIZE_SM, |theme| Tokens::of(
                            theme
                        )
                        .foreground),
                        line("100 Market St, San Francisco", theme::FONT_SIZE_SM, muted),
                    ]
                    .spacing(4.0)
                ),
                card(
                    column![
                        line("Items", theme::FONT_SIZE_SM, |theme| Tokens::of(theme)
                            .foreground),
                        line("2x Studio Headphones", theme::FONT_SIZE_SM, muted),
                    ]
                    .spacing(4.0)
                ),
            ]
            .spacing(8.0),
        ),
    ]
    .spacing(8.0);

    // section: Row trigger
    // The built-in trigger is the whole row: the component draws the title and
    // the ruled chevron and reports the caller's message.
    let row_trigger = Collapsible::new()
        .trigger(collapsible::trigger(
            "How do I reset my password?",
            app.row_open,
            Message::ToggleRow,
        ))
        .open(app.row_open)
        .content(line(
            "Click the Forgot Password link on the sign in page, and we will send \
             you an email with instructions to create a new one.",
            theme::FONT_SIZE_SM,
            muted,
        ));

    // section: Bottom trigger
    // The trigger sits on the bottom edge of the card it opens, so the panel's
    // content is the breakdown above it and the button below reopens it.
    let usage = Collapsible::new().open(app.usage_open).content(
        column![
            row![
                line("Requests", theme::FONT_SIZE_XS, muted).width(Length::Fill),
                line("$210.84", theme::FONT_SIZE_XS, |theme| Tokens::of(theme)
                    .foreground),
            ]
            .spacing(8.0),
            row![
                line("Active CPU", theme::FONT_SIZE_XS, muted).width(Length::Fill),
                line("$21.95", theme::FONT_SIZE_XS, |theme| Tokens::of(theme)
                    .foreground),
            ]
            .spacing(8.0),
            row![
                line("Events", theme::FONT_SIZE_XS, muted).width(Length::Fill),
                line("$21.20", theme::FONT_SIZE_XS, |theme| Tokens::of(theme)
                    .foreground),
            ]
            .spacing(8.0),
            row![
                line("Storage", theme::FONT_SIZE_XS, muted).width(Length::Fill),
                line("$20.45", theme::FONT_SIZE_XS, |theme| Tokens::of(theme)
                    .foreground),
            ]
            .spacing(8.0),
        ]
        .spacing(8.0),
    );

    let usage_card = card(
        column![
            row![
                line("3 days remaining in cycle", theme::FONT_SIZE_SM, |theme| {
                    Tokens::of(theme).foreground
                })
                .width(Length::Fill),
                Button::new("Billing")
                    .outline()
                    .size(ControlSize::XSmall)
                    .on_press(Message::Notice("Billing")),
            ]
            .spacing(8.0)
            .align_y(Alignment::Center),
            soft_card(
                column![
                    row![
                        line("$18.08 / $20", theme::FONT_SIZE_SM, |theme| Tokens::of(
                            theme
                        )
                        .foreground),
                        line("$200", theme::FONT_SIZE_SM, |theme| Tokens::of(theme)
                            .foreground),
                    ]
                    .spacing(8.0),
                    Progress::new(0.9),
                ]
                .spacing(8.0),
            ),
            usage,
        ]
        .spacing(12.0),
    );

    let bottom = column![
        usage_card,
        container(
            Button::new("")
                .outline()
                .size(ControlSize::XSmall)
                .icon(chevron(app.usage_open))
                .on_press(Message::ToggleUsage),
        )
        .width(Length::Fill)
        .center_x(Length::Fill),
    ]
    .spacing(0.0);

    // section: Settings
    // The trigger is the caller's own row, which is why the component reports
    // only the message: here a chevron and a title open the notification panel.
    let settings = column![
        trigger_row(
            app.settings_open,
            "Notification settings",
            Message::ToggleSettings
        ),
        Collapsible::new().open(app.settings_open).content(card(
            column![
                Checkbox::new(app.push)
                    .label("Push notifications")
                    .on_toggle(Message::Push),
                rule(),
                Checkbox::new(app.email)
                    .label("Email notifications")
                    .on_toggle(Message::Email),
                rule(),
                Checkbox::new(app.sms)
                    .label("SMS notifications")
                    .on_toggle(Message::Sms),
            ]
            .spacing(8.0),
        )),
    ]
    .spacing(8.0);

    // section: Row actions
    // Actions live beside the trigger and in every row: the plus adds a key, the
    // ellipsis holds each row's own commands.
    let keys = column![
        row![
            trigger_row(app.keys_open, "API Keys", Message::ToggleKeys),
            icon_button(PLUS, Message::Notice("Add key")),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
        Collapsible::new().open(app.keys_open).content(
            column![
                key_row("Production", "PRDK230454*242SDIFPPL"),
                key_row("Development", "DUILO30454*242SDIFUIP"),
                key_row("Staging", "IPPODAS230454*242SDI"),
            ]
            .spacing(8.0),
        ),
    ]
    .spacing(12.0);

    // section: Nested
    // Panels nest to any depth: the folder rows are triggers, and the indent is
    // the caller's, so a nested panel returns to its parent's spine when it ends.
    let nested = column![
        folder_row(
            app.components_open,
            "components",
            8.0,
            Message::ToggleComponents
        ),
        Collapsible::new().open(app.components_open).content(
            column![
                folder_row(app.ui_open, "ui", 20.0, Message::ToggleUi),
                Collapsible::new().open(app.ui_open).content(
                    column![
                        file_row("button.rs", 48.0),
                        file_row("card.rs", 48.0),
                        file_row("dialog.rs", 48.0),
                    ]
                    .spacing(2.0),
                ),
                file_row("login_form.rs", 32.0),
            ]
            .spacing(2.0),
        ),
        file_row("main.rs", 22.0),
    ]
    .spacing(2.0);

    // section: Profile
    // Shows who someone is, and their details only on request.
    let profile = column![
        pressable(
            container(
                row![
                    Avatar::new().initials("JL").size(AvatarSize::XSmall),
                    line("@huacnlee", theme::FONT_SIZE_SM, |theme| Tokens::of(theme)
                        .foreground)
                    .width(Length::Fill),
                    chevron(app.profile_open),
                ]
                .spacing(8.0)
                .align_y(Alignment::Center),
            )
            .padding([4.0, 8.0])
            .width(Length::Fill),
            Message::ToggleProfile,
        ),
        Collapsible::new().open(app.profile_open).content(
            column![
                detail_line(INBOX, "Last activity", "2 hours ago"),
                detail_line(CALENDAR, "Online since", "Today, 9:00 AM"),
                detail_line(GLOBE, "Location", "Hong Kong"),
            ]
            .spacing(8.0),
        ),
    ]
    .spacing(12.0);

    // section: Details
    // The titled trigger form, open on boot: the title and the chevron are the
    // component's own, and the caller still owns the state and the message.
    let details = Collapsible::new()
        .trigger(collapsible::trigger(
            "Advanced",
            app.advanced_open,
            Message::ToggleAdvanced,
        ))
        .open(app.advanced_open)
        .content(line(
            "These options are for power users.",
            theme::FONT_SIZE_SM,
            |theme| Tokens::of(theme).foreground,
        ));

    // section: end
    let status = text(match app.notice {
        Some(label) => format!("Last control: {label}"),
        None => "Click a trigger: each panel is the caller's own open state.".to_string(),
    })
    .size(theme::FONT_SIZE_SM)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, "Animated reveal"),
        body_box(animated),
        heading(app, 1, "Basic"),
        body_box(basic),
        heading(app, 2, "Row trigger"),
        body_box(row_trigger),
        heading(app, 3, "Bottom trigger"),
        body_box(bottom),
        heading(app, 4, "Settings"),
        body_box(settings),
        heading(app, 5, "Row actions"),
        body_box(keys),
        heading(app, 6, "Nested"),
        body_box(nested),
        heading(app, 7, "Profile"),
        body_box(profile),
        heading(app, 8, "Details"),
        body_box(details),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The marks the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the
// two galleries draw the same glyphs rather than two lookalikes.

const CHEVRON_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#;

const CHEVRON_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#;

const CHEVRONS_UP_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/></svg>"#;

const PLUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>"#;

const ELLIPSIS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>"#;

const HASH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="4" x2="20" y1="9" y2="9"/><line x1="4" x2="20" y1="15" y2="15"/><line x1="10" x2="8" y1="3" y2="21"/><line x1="16" x2="14" y1="3" y2="21"/></svg>"#;

const FOLDER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/></svg>"#;

const FOLDER_OPEN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2"/></svg>"#;

const FILE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/></svg>"#;

const INBOX: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/></svg>"#;

const CALENDAR: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 2v3"/><path d="M16 2v3"/><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18"/></svg>"#;

const GLOBE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
