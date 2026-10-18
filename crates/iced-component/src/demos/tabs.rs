//! `tabs` — a tab bar over the active tab's content, in the three variants and
//! the sizes the Slint example shows: the basic bar, the underline, pill and
//! outline variants, the size ladder, a leading icon per tab, a wide bar and a
//! disabled tab.
//!
//! Run it with `cargo run --example tabs`, or in the browser as `?story=tabs`
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
//! iced has no tabs widget, so the component is the bar, the underline rule and
//! the body slot; the caller owns `selected` and passes the body for that index.
//! Two Slint behaviours have no iced counterpart and are named rather than
//! faked: the bar does not flick sideways when its labels overflow (iced scrolls
//! only through `scrollable`), and an icon is the caller's element rather than a
//! name the bar resolves, so the demo colours it with the component's own `ink`
//! rule.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::tabs::{ink, Tab, Tabs, Variant};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — tabs")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// One selection per section, because the bar is a controlled value: iced
    /// owns no selection, so the caller holds each index and writes it back
    /// through `on_press`.
    basic: usize,
    underline: usize,
    pill: usize,
    outline: usize,
    small: usize,
    medium: usize,
    large: usize,
    icons: usize,
    scrollable: usize,
    disabled: usize,
    /// The last tab a bar reported, so a choice is visible.
    last: Option<(&'static str, usize)>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A bar reported a tab: the section that owns it, and the index.
    Choose(&'static str, usize),
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
        // The variant bars start on their middle tab, as the Slint example does.
        underline: 1,
        pill: 1,
        outline: 1,
        ..App::default()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Choose(section, index) => {
            match section {
                "basic" => app.basic = index,
                "underline" => app.underline = index,
                "pill" => app.pill = index,
                "outline" => app.outline = index,
                "small" => app.small = index,
                "medium" => app.medium = index,
                "large" => app.large = index,
                "icons" => app.icons = index,
                "scrollable" => app.scrollable = index,
                "disabled" => app.disabled = index,
                _ => {}
            }
            app.last = Some((section, index));
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

/// A muted line under an example, carrying what the demo has to say about it.
fn muted_line<'a>(line: &str) -> Element<'a, Message> {
    text(line.to_string())
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A tab's leading icon, coloured with the component's own `ink` rule so the
/// selected pill's glyph takes `primary-foreground` with its label. An SVG keeps
/// its own black unless the style closure colours it, which is the trap every
/// icon in this gallery names.
fn tab_glyph<'a>(markup: &'static [u8], selected: bool) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(ControlSize::Medium.icon_size())
        .height(ControlSize::Medium.icon_size())
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(ink(Tokens::of(theme), Variant::Pill, selected, false)),
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

/// The three bars of the "Tab Variants" section. One helper keeps the labels
/// identical so the only difference on show is the variant.
fn variant_bar<'a>(
    variant: Variant,
    selected: usize,
    body: &'static str,
) -> Element<'a, Message> {
    let key = match variant {
        Variant::Underline => "underline",
        Variant::Pill => "pill",
        Variant::Outline => "outline",
    };
    Tabs::new([
        Tab::new("Default").on_press(Message::Choose(key, 0)),
        Tab::new("Selected").on_press(Message::Choose(key, 1)),
        Tab::new("Third").on_press(Message::Choose(key, 2)),
    ])
    .variant(variant)
    .selected(selected)
    .body(muted_line(body))
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic Tabs
    // The body is the active tab's content: the caller owns `selected` and hands
    // the bar the body for that index, which is what the Slint component's
    // `@children` slot holds.
    const BASIC_BODY: [&str; 3] = [
        "Account settings live here.",
        "Change your password here.",
        "Manage your settings here.",
    ];
    let basic = Tabs::new([
        Tab::new("Account").on_press(Message::Choose("basic", 0)),
        Tab::new("Password").on_press(Message::Choose("basic", 1)),
        Tab::new("Settings").on_press(Message::Choose("basic", 2)),
    ])
    .variant(Variant::Underline)
    .selected(app.basic)
    .body(muted_line(BASIC_BODY[app.basic]));

    // section: Tab Variants
    // The variant sets the whole bar: a rule under the active tab, a filled
    // pill, or an outlined one.
    let variants = column![
        variant_bar(Variant::Underline, app.underline, "Underline."),
        variant_bar(Variant::Pill, app.pill, "Pill."),
        variant_bar(Variant::Outline, app.outline, "Outline."),
    ]
    .spacing(16.0);

    // section: Tab Sizes
    let sizes = column![
        Tabs::new([
            Tab::new("Small").on_press(Message::Choose("small", 0)),
            Tab::new("Tabs").on_press(Message::Choose("small", 1)),
        ])
        .variant(Variant::Pill)
        .size(ControlSize::Small)
        .selected(app.small),
        Tabs::new([
            Tab::new("Medium").on_press(Message::Choose("medium", 0)),
            Tab::new("Tabs").on_press(Message::Choose("medium", 1)),
        ])
        .variant(Variant::Pill)
        .size(ControlSize::Medium)
        .selected(app.medium),
        Tabs::new([
            Tab::new("Large").on_press(Message::Choose("large", 0)),
            Tab::new("Tabs").on_press(Message::Choose("large", 1)),
        ])
        .variant(Variant::Pill)
        .size(ControlSize::Large)
        .selected(app.large),
    ]
    .spacing(12.0);

    // section: Tabs with Icons
    let icons = Tabs::new([
        Tab::new("Account")
            .icon(tab_glyph(USER, app.icons == 0))
            .on_press(Message::Choose("icons", 0)),
        Tab::new("Password")
            .icon(tab_glyph(LOCK, app.icons == 1))
            .on_press(Message::Choose("icons", 1)),
        Tab::new("Billing")
            .icon(tab_glyph(CIRCLE, app.icons == 2))
            .on_press(Message::Choose("icons", 2)),
    ])
    .variant(Variant::Pill)
    .selected(app.icons)
    .body(muted_line("An icon leads each label."));

    // section: Scrollable Tabs
    // Slint's bar flicks sideways when the labels do not fit. iced has no
    // flickable, so the caller wraps the bar in `scrollable` for that; the bar
    // itself is a plain row, and the demo names the difference rather than
    // faking a drag.
    let scrollable_tabs = Tabs::new([
        Tab::new("Overview").on_press(Message::Choose("scrollable", 0)),
        Tab::new("Activity").on_press(Message::Choose("scrollable", 1)),
        Tab::new("Members").on_press(Message::Choose("scrollable", 2)),
        Tab::new("Billing").on_press(Message::Choose("scrollable", 3)),
        Tab::new("Integrations").on_press(Message::Choose("scrollable", 4)),
        Tab::new("Security").on_press(Message::Choose("scrollable", 5)),
        Tab::new("Audit log").on_press(Message::Choose("scrollable", 6)),
    ])
    .variant(Variant::Pill)
    .selected(app.scrollable)
    .body(muted_line(
        "iced's bar is a plain row; wrap it in `scrollable` to reach the rest.",
    ));

    // section: Disabled Tabs
    // `disabled(true)` drops the tab's message, which is what makes it inert
    // without touching the selection.
    let disabled = Tabs::new([
        Tab::new("Enabled").on_press(Message::Choose("disabled", 0)),
        Tab::new("Disabled")
            .disabled(true)
            .on_press(Message::Choose("disabled", 1)),
        Tab::new("Also enabled").on_press(Message::Choose("disabled", 2)),
    ])
    .variant(Variant::Pill)
    .selected(app.disabled)
    .body(muted_line("The middle tab cannot be selected."));

    // section: end
    let status = text(match app.last {
        Some((section, index)) => {
            format!("Last chosen: {section} tab {index} — the owner wrote the index back.")
        }
        None => "Choose a tab — the gallery is live.".to_string(),
    });

    let gallery = column![
        heading(app, 0, "Basic Tabs"),
        body_box(basic),
        heading(app, 1, "Tab Variants"),
        body_box(variants),
        heading(app, 2, "Tab Sizes"),
        body_box(sizes),
        heading(app, 3, "Tabs with Icons"),
        body_box(icons),
        heading(app, 4, "Scrollable Tabs"),
        body_box(scrollable_tabs),
        heading(app, 5, "Disabled Tabs"),
        body_box(disabled),
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

const USER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"#;

const LOCK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>"#;

const CIRCLE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
