//! `radio` — the whole gallery the Slint example shows, plus both group
//! layouts: a group that keeps one selection, a standalone radio, the sizes, a
//! disabled radio and a disabled group.
//!
//! Run it with `cargo run --example radio`, or in the browser as `?story=radio`
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
//! The page's other examples — a controlled radio, a multi-line label with
//! custom content, the settings, survey and payment panels — are compositions
//! of a radio or a group with an application's own content, and the port leaves
//! them to GPUI rather than showing a stand-in.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::radio::{Radio, RadioGroup};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — radio")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// One choice per example, because a group is the owner's state: iced has
    /// no selection to own, so the caller holds each index and takes every
    /// change back through `on_change`.
    usage: Option<usize>,
    group: Option<usize>,
    standalone: bool,
    small: Option<usize>,
    medium: Option<usize>,
    large: Option<usize>,
    horizontal: Option<usize>,
    vertical: Option<usize>,
    /// The last choice a group reported, so a click is visible.
    last: Option<usize>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Usage(usize),
    Group(usize),
    Standalone(bool),
    Small(usize),
    Medium(usize),
    Large(usize),
    Horizontal(usize),
    Vertical(usize),
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
        usage: Some(0),
        group: Some(1),
        small: Some(0),
        medium: Some(0),
        large: Some(1),
        horizontal: Some(1),
        vertical: Some(0),
        ..App::default()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Usage(index) => {
            app.usage = Some(index);
            app.last = Some(index);
        }
        Message::Group(index) => {
            app.group = Some(index);
            app.last = Some(index);
        }
        Message::Standalone(checked) => app.standalone = checked,
        Message::Small(index) => {
            app.small = Some(index);
            app.last = Some(index);
        }
        Message::Medium(index) => {
            app.medium = Some(index);
            app.last = Some(index);
        }
        Message::Large(index) => {
            app.large = Some(index);
            app.last = Some(index);
        }
        Message::Horizontal(index) => {
            app.horizontal = Some(index);
            app.last = Some(index);
        }
        Message::Vertical(index) => {
            app.vertical = Some(index);
            app.last = Some(index);
        }
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
    // A group is the caller's index written back; a standalone radio is the
    // caller's boolean. Both are controlled values, the way every iced control is.
    let usage = column![
        RadioGroup::new()
            .options(["Email", "SMS", "Push"])
            .selected(app.usage)
            .on_change(Message::Usage),
        row![
            Radio::new(app.standalone)
                .label("Selected")
                .on_change(Message::Standalone),
            Radio::new(false)
                .label("Unselected")
                .on_change(Message::Standalone),
        ]
        .spacing(16.0),
    ]
    .spacing(12.0);

    // section: Group
    let group = RadioGroup::new()
        .options(["Option 1", "Option 2", "Option 3"])
        .selected(app.group)
        .on_change(Message::Group);

    // section: Standalone
    // A radio reports only `true`: it can never clear its own set, so the group
    // cannot be emptied. Without a message the control is disabled.
    let standalone = column![
        Radio::new(true)
            .label("Checked and enabled")
            .on_change(Message::Standalone),
        Radio::new(false)
            .label("Unchecked and enabled")
            .on_change(Message::Standalone),
    ]
    .spacing(12.0);

    // section: Different Sizes
    // The disc is 12 / 14 / 16 / 18, and a large label is text-lg.
    let sizes = column![
        RadioGroup::new()
            .size(ControlSize::XSmall)
            .options(["Extra small", "Compact"])
            .selected(app.small)
            .on_change(Message::Small),
        RadioGroup::new()
            .size(ControlSize::Medium)
            .options(["Medium", "Default"])
            .selected(app.medium)
            .on_change(Message::Medium),
        RadioGroup::new()
            .size(ControlSize::Large)
            .options(["Large", "Roomy"])
            .selected(app.large)
            .on_change(Message::Large),
    ]
    .spacing(12.0);

    // section: Horizontal Layout
    let horizontal = RadioGroup::horizontal()
        .options(["First", "Second", "Third"])
        .selected(app.horizontal)
        .on_change(Message::Horizontal);

    // section: Vertical Layout
    // The group with one option: a radio never toggles off, so the only choice
    // is the selected one and the group cannot be emptied.
    let vertical = RadioGroup::new()
        .options(["Standard delivery — the only method for this address"])
        .selected(app.vertical)
        .on_change(Message::Vertical);

    // section: Disabled Radio Group
    // `disabled(true)` drops every message in the set, which is what makes the
    // whole group inert without touching its selection.
    let disabled_group = RadioGroup::new()
        .options(["Option A", "Option B", "Option C"])
        .selected(Some(1))
        .disabled(true);

    // section: Disabled State
    let disabled = column![
        Radio::new(false).label("Disabled option"),
        Radio::new(true).label("Disabled and checked"),
    ]
    .spacing(12.0);

    // section: end
    let status = text(match app.last {
        Some(index) => format!("Last choice: option {index} — the owner wrote the index back."),
        None => "Pick an option — the gallery is live.".to_string(),
    });

    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Group"),
        body_box(group),
        heading(app, 2, "Standalone"),
        body_box(standalone),
        heading(app, 3, "Different Sizes"),
        body_box(sizes),
        heading(app, 4, "Horizontal Layout"),
        body_box(horizontal),
        heading(app, 5, "Vertical Layout"),
        body_box(vertical),
        heading(app, 6, "Disabled Radio Group"),
        body_box(disabled_group),
        heading(app, 7, "Disabled State"),
        body_box(disabled),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
