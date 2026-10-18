//! `switch` — the gallery the Slint example shows: the on/off track with its
//! sliding thumb, the four sizes, labels, and the disabled states.
//!
//! Run it with `cargo run --example switch`, or in the browser as `?story=switch`
//! in the gallery the website embeds.
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
//! The page's other examples — a custom colour, a tooltip, a focus ring, a
//! settings panel with a description beside each switch — hang properties the
//! iced port does not have on the same control, and the port leaves them to GPUI
//! rather than showing a stand-in.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::switch::Switch;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — switch")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    usage: bool,
    airplane: bool,
    wifi: bool,
    size_xs: bool,
    size_sm: bool,
    size_md: bool,
    size_lg: bool,
    anonymous: bool,
    on_top: bool,
    reopen: bool,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Usage(bool),
    Airplane(bool),
    Wifi(bool),
    SizeXs(bool),
    SizeSm(bool),
    SizeMd(bool),
    SizeLg(bool),
    Anonymous(bool),
    OnTop(bool),
    Reopen(bool),
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
        usage: true,
        wifi: true,
        size_xs: true,
        size_sm: true,
        size_md: true,
        size_lg: true,
        anonymous: true,
        reopen: true,
        ..App::default()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Usage(checked) => app.usage = checked,
        Message::Airplane(checked) => app.airplane = checked,
        Message::Wifi(checked) => app.wifi = checked,
        Message::SizeXs(checked) => app.size_xs = checked,
        Message::SizeSm(checked) => app.size_sm = checked,
        Message::SizeMd(checked) => app.size_md = checked,
        Message::SizeLg(checked) => app.size_lg = checked,
        Message::Anonymous(checked) => app.anonymous = checked,
        Message::OnTop(checked) => app.on_top = checked,
        Message::Reopen(checked) => app.reopen = checked,
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

fn view(app: &App) -> Element<'_, Message> {
    // section: Usage
    // The value is the caller's: `new(checked)` reads it and `on_toggle` reports
    // the requested new one, which the owner writes back.
    let usage = column![
        Switch::new(app.usage)
            .label("Enable notifications")
            .on_toggle(Message::Usage),
        text(format!("Notifications are {}.", on_off(app.usage))),
    ]
    .spacing(8.0);

    // section: Basic
    let basic = column![
        Switch::new(app.airplane)
            .label("Airplane mode")
            .on_toggle(Message::Airplane),
        Switch::new(app.wifi)
            .label("Wi-Fi")
            .on_toggle(Message::Wifi),
        // No message: the switch cannot be pressed, and iced draws it disabled.
        Switch::new(false).label("Disabled"),
    ]
    .spacing(12.0);

    // section: Sizes
    // The track is 28×16 for the two small steps and 36×20 for the two large
    // ones; the thumb follows at 12 and 16.
    let sizes = row![
        Switch::new(app.size_xs)
            .size(ControlSize::XSmall)
            .on_toggle(Message::SizeXs),
        Switch::new(app.size_sm)
            .size(ControlSize::Small)
            .on_toggle(Message::SizeSm),
        Switch::new(app.size_md)
            .size(ControlSize::Medium)
            .on_toggle(Message::SizeMd),
        Switch::new(app.size_lg)
            .size(ControlSize::Large)
            .on_toggle(Message::SizeLg),
    ]
    .spacing(16.0);

    // section: With Label
    let labels = column![
        Switch::new(app.anonymous)
            .label("Send anonymous usage data")
            .on_toggle(Message::Anonymous),
        Switch::new(app.on_top)
            .label("Keep the window on top of others")
            .on_toggle(Message::OnTop),
        Switch::new(app.reopen)
            .label("Reopen the last workspace on launch")
            .on_toggle(Message::Reopen),
    ]
    .spacing(10.0);

    // section: Disabled State
    let disabled = column![
        Switch::new(false).label("Off and disabled"),
        Switch::new(true).label("On and disabled"),
    ]
    .spacing(10.0);

    // section: end
    let status = text("Press a switch — the owner writes the reported value back.");

    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Basic"),
        body_box(basic),
        heading(app, 2, "Sizes"),
        body_box(sizes),
        heading(app, 3, "With Label"),
        body_box(labels),
        heading(app, 4, "Disabled State"),
        body_box(disabled),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

/// The word a state reads as, for the demo's own status lines.
fn on_off(checked: bool) -> &'static str {
    if checked { "on" } else { "off" }
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
