//! `toggle` — the gallery the Slint example shows: an icon-only row, a
//! controlled toggle, the two variants, the sizes, the individual events, a
//! toolbar strip of outlined toggles, and the disabled states.
//!
//! Run it with `cargo run --example toggle`, or in the browser as `?story=toggle`
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
//! Two Slint sections need a `ToggleGroup`, the row that keeps one selection
//! across several toggles. The form family asked for a group around `radio` and
//! not around `toggle`, so **Group** and **Toggle Group Variants and Sizes**
//! have no iced counterpart here and the page keeps them for GPUI.

use iced::widget::{Space, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Border, Element, Length, Task, Theme};

use crate::components::button::Button;
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, ControlSize, Tokens};
use crate::components::toggle::{self, Toggle, Variant};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — toggle")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    usage: bool,
    bold: bool,
    italic: bool,
    underline: bool,
    controlled: bool,
    variant_default: bool,
    variant_default_on: bool,
    variant_outline: bool,
    variant_outline_on: bool,
    size_small: bool,
    size_medium: bool,
    size_large: bool,
    event_bold: bool,
    event_italic: bool,
    event_last: Option<&'static str>,
    toolbar_bold: bool,
    toolbar_italic: bool,
    toolbar_underline: bool,
    toolbar_comment: bool,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Usage(bool),
    Bold(bool),
    Italic(bool),
    Underline(bool),
    Controlled(bool),
    FlipControlled,
    VariantDefault(bool),
    VariantDefaultOn(bool),
    VariantOutline(bool),
    VariantOutlineOn(bool),
    SizeSmall(bool),
    SizeMedium(bool),
    SizeLarge(bool),
    EventBold(bool),
    EventItalic(bool),
    ToolbarBold(bool),
    ToolbarItalic(bool),
    ToolbarUnderline(bool),
    ToolbarComment(bool),
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
        variant_default_on: true,
        variant_outline_on: true,
        size_small: true,
        size_medium: true,
        size_large: true,
        italic: true,
        event_italic: true,
        ..App::default()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Usage(selected) => app.usage = selected,
        Message::Bold(selected) => app.bold = selected,
        Message::Italic(selected) => app.italic = selected,
        Message::Underline(selected) => app.underline = selected,
        Message::Controlled(selected) => app.controlled = selected,
        Message::FlipControlled => app.controlled = !app.controlled,
        Message::VariantDefault(selected) => app.variant_default = selected,
        Message::VariantDefaultOn(selected) => app.variant_default_on = selected,
        Message::VariantOutline(selected) => app.variant_outline = selected,
        Message::VariantOutlineOn(selected) => app.variant_outline_on = selected,
        Message::SizeSmall(selected) => app.size_small = selected,
        Message::SizeMedium(selected) => app.size_medium = selected,
        Message::SizeLarge(selected) => app.size_large = selected,
        Message::EventBold(selected) => {
            app.event_bold = selected;
            app.event_last = Some(if selected { "Bold on" } else { "Bold off" });
        }
        Message::EventItalic(selected) => {
            app.event_italic = selected;
            app.event_last = Some(if selected { "Italic on" } else { "Italic off" });
        }
        Message::ToolbarBold(selected) => app.toolbar_bold = selected,
        Message::ToolbarItalic(selected) => app.toolbar_italic = selected,
        Message::ToolbarUnderline(selected) => app.toolbar_underline = selected,
        Message::ToolbarComment(selected) => app.toolbar_comment = selected,
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

/// A Lucide glyph from the set this repository already ships, at the size the
/// control's step gives an icon and in the ink the toggle draws its label in.
/// iced ships no icon set, so the artwork is the caller's; Slint names an SVG.
fn glyph<'a>(markup: &'static [u8], size: ControlSize, selected: bool) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(size.icon_size())
        .height(size.icon_size())
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(toggle::ink(Tokens::of(theme), selected, false)),
        })
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

fn view(app: &App) -> Element<'_, Message> {
    // section: Usage
    // The pressed value is the caller's: `new(selected)` reads it and
    // `on_toggle` reports the requested new one, which the owner writes back.
    let usage = row![
        Toggle::new(app.usage)
            .label("Bold")
            .icon(glyph(BOLD, ControlSize::Medium, app.usage))
            .on_toggle(Message::Usage),
        Toggle::new(true).label("Pressed"),
        Toggle::new(false).label("Unpressed"),
    ]
    .spacing(8.0);

    // section: Toggle
    // An icon on its own is a square control; the icon set belongs to the
    // application, so the element is the caller's where Slint names a file.
    let icons = row![
        Toggle::new(app.bold)
            .icon(glyph(BOLD, ControlSize::Medium, app.bold))
            .on_toggle(Message::Bold),
        Toggle::new(app.italic)
            .icon(glyph(ITALIC, ControlSize::Medium, app.italic))
            .on_toggle(Message::Italic),
        Toggle::new(app.underline)
            .icon(glyph(UNDERLINE, ControlSize::Medium, app.underline))
            .on_toggle(Message::Underline),
    ]
    .spacing(8.0);

    // section: Controlled Toggle
    // The host owns the value, so anything else on the page can move it too.
    let controlled = column![
        row![
            Toggle::new(app.controlled)
                .label("Bold")
                .icon(glyph(BOLD, ControlSize::Medium, app.controlled))
                .on_toggle(Message::Controlled),
            Button::new(if app.controlled {
                "Turn off"
            } else {
                "Turn on"
            })
            .on_press(Message::FlipControlled),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
        text(format!(
            "Bold is {}",
            if app.controlled { "on" } else { "off" }
        )),
    ]
    .spacing(10.0);

    // section: Toggle Variants
    // `default` stays flat until it is hovered or pressed; `outline` keeps a
    // frame at rest, which is what a toolbar wants.
    let variants = row![
        Toggle::new(app.variant_default)
            .label("Default")
            .on_toggle(Message::VariantDefault),
        Toggle::new(app.variant_default_on)
            .label("Default on")
            .on_toggle(Message::VariantDefaultOn),
        Toggle::new(app.variant_outline)
            .variant(Variant::Outline)
            .label("Outline")
            .on_toggle(Message::VariantOutline),
        Toggle::new(app.variant_outline_on)
            .variant(Variant::Outline)
            .label("Outline on")
            .on_toggle(Message::VariantOutlineOn),
    ]
    .spacing(8.0);

    // section: Different Sizes
    let sizes = row![
        Toggle::new(app.size_small)
            .size(ControlSize::Small)
            .label("Small")
            .on_toggle(Message::SizeSmall),
        Toggle::new(app.size_medium)
            .label("Medium")
            .on_toggle(Message::SizeMedium),
        Toggle::new(app.size_large)
            .size(ControlSize::Large)
            .label("Large")
            .on_toggle(Message::SizeLarge),
    ]
    .spacing(8.0);

    // section: Individual Toggle Events
    // `on_toggle` reports the state it moved to, so a host can record or
    // forward it without reading the property back.
    let events = column![
        row![
            Toggle::new(app.event_bold)
                .label("Bold")
                .icon(glyph(BOLD, ControlSize::Medium, app.event_bold))
                .on_toggle(Message::EventBold),
            Toggle::new(app.event_italic)
                .label("Italic")
                .icon(glyph(ITALIC, ControlSize::Medium, app.event_italic))
                .on_toggle(Message::EventItalic),
        ]
        .spacing(8.0),
        text(format!(
            "Last change: {}",
            app.event_last.unwrap_or("nothing yet")
        )),
    ]
    .spacing(10.0);

    // section: Toolbar with Toggle Buttons
    // A row of icon toggles on a strip: the outlined variant is what the
    // toolbar wants, and the divider is a one-pixel rule between groups.
    let toolbar = container(
        row![
            Toggle::new(app.toolbar_bold)
                .variant(Variant::Outline)
                .icon(glyph(BOLD, ControlSize::Medium, app.toolbar_bold))
                .on_toggle(Message::ToolbarBold),
            Toggle::new(app.toolbar_italic)
                .variant(Variant::Outline)
                .icon(glyph(ITALIC, ControlSize::Medium, app.toolbar_italic))
                .on_toggle(Message::ToolbarItalic),
            Toggle::new(app.toolbar_underline)
                .variant(Variant::Outline)
                .icon(glyph(UNDERLINE, ControlSize::Medium, app.toolbar_underline))
                .on_toggle(Message::ToolbarUnderline),
            container(Space::new().width(1.0).height(20.0)).style(|theme: &Theme| {
                container::Style {
                    background: Some(Tokens::of(theme).border.into()),
                    ..container::Style::default()
                }
            }),
            Toggle::new(app.toolbar_comment)
                .variant(Variant::Outline)
                .label("Comment")
                .icon(glyph(MESSAGE, ControlSize::Medium, app.toolbar_comment))
                .on_toggle(Message::ToolbarComment),
        ]
        .spacing(4.0)
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .height(44.0)
    .padding(6.0)
    .align_y(Alignment::Center)
    .style(|theme: &Theme| container::Style {
        border: Border {
            color: Tokens::of(theme).border,
            width: 1.0,
            radius: theme::RADIUS_MD.into(),
        },
        ..container::Style::default()
    });

    // section: Disabled State
    let disabled = column![
        Toggle::new(false).label("Disabled"),
        Toggle::new(true).label("Disabled on"),
    ]
    .spacing(10.0);

    // section: end
    let status = text("Press a toggle — the owner writes the reported value back.");

    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Toggle"),
        body_box(icons),
        heading(app, 2, "Controlled Toggle"),
        body_box(controlled),
        heading(app, 3, "Toggle Variants"),
        body_box(variants),
        heading(app, 4, "Different Sizes"),
        body_box(sizes),
        heading(app, 5, "Individual Toggle Events"),
        body_box(events),
        heading(app, 6, "Toolbar with Toggle Buttons"),
        body_box(toolbar),
        heading(app, 7, "Disabled State"),
        body_box(disabled),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The Lucide marks the Slint example names, from the icon set this repository
// already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so both
// galleries draw the same glyphs.

const BOLD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8"/></svg>"#;

const ITALIC: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="19" x2="10" y1="4" y2="4"/><line x1="14" x2="5" y1="20" y2="20"/><line x1="15" x2="9" y1="4" y2="20"/></svg>"#;

const UNDERLINE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 4v6a6 6 0 0 0 12 0V4"/><line x1="4" x2="20" y1="20" y2="20"/></svg>"#;

const MESSAGE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
