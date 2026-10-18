//! `checkbox` — a binary choice with a label, a description under it, the
//! indeterminate dash, sizes and a disabled state, shown the way the Slint
//! example shows it: the basic choices, the default pair, the four sizes, a
//! controlled box, the disabled looks, a box without a label, the three
//! labelled shapes, a short list and a form.
//!
//! Run it with `cargo run --example checkbox`, or in the browser as
//! `?story=checkbox` in the gallery the website embeds.
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
//! The page's **Custom Tab Order** heading has no iced counterpart: iced 0.14
//! owns no tab-order or focus-trap API, so a checkbox cannot be given a position
//! in a tab sequence. The page keeps that example for GPUI rather than showing a
//! stand-in here. Everything else the Slint example draws is one of the nine
//! sections below.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::checkbox::Checkbox;
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — checkbox")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// One value per box, because iced's checkbox holds no state of its own: the
    /// caller passes the value in and takes the requested one back.
    terms: bool,
    product_updates: bool,
    /// The checked value of the always-indeterminate box: unchecked it draws the
    /// minus, checked it draws the tick.
    some: bool,
    default_updates: bool,
    default_remember: bool,
    size_xs: bool,
    size_sm: bool,
    size_md: bool,
    size_lg: bool,
    controlled: bool,
    without_label: bool,
    label_updates: bool,
    label_notify: bool,
    label_terms: bool,
    option_1: bool,
    option_2: bool,
    option_3: bool,
    form_terms: bool,
    form_newsletter: bool,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Terms(bool),
    ProductUpdates(bool),
    Indeterminate(bool),
    DefaultUpdates(bool),
    DefaultRemember(bool),
    SizeXs(bool),
    SizeSm(bool),
    SizeMd(bool),
    SizeLg(bool),
    Controlled(bool),
    WithoutLabel(bool),
    LabelUpdates(bool),
    LabelNotify(bool),
    LabelTerms(bool),
    Option1(bool),
    Option2(bool),
    Option3(bool),
    FormTerms(bool),
    FormNewsletter(bool),
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
        terms: true,
        default_remember: true,
        size_xs: true,
        size_sm: true,
        size_md: true,
        size_lg: true,
        label_updates: true,
        option_1: true,
        ..App::default()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Terms(value) => app.terms = value,
        Message::ProductUpdates(value) => app.product_updates = value,
        Message::Indeterminate(value) => app.some = value,
        Message::DefaultUpdates(value) => app.default_updates = value,
        Message::DefaultRemember(value) => app.default_remember = value,
        Message::SizeXs(value) => app.size_xs = value,
        Message::SizeSm(value) => app.size_sm = value,
        Message::SizeMd(value) => app.size_md = value,
        Message::SizeLg(value) => app.size_lg = value,
        Message::Controlled(value) => app.controlled = value,
        Message::WithoutLabel(value) => app.without_label = value,
        Message::LabelUpdates(value) => app.label_updates = value,
        Message::LabelNotify(value) => app.label_notify = value,
        Message::LabelTerms(value) => app.label_terms = value,
        Message::Option1(value) => app.option_1 = value,
        Message::Option2(value) => app.option_2 = value,
        Message::Option3(value) => app.option_3 = value,
        Message::FormTerms(value) => app.form_terms = value,
        Message::FormNewsletter(value) => app.form_newsletter = value,
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
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
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
        container(
            text(title)
                .size(theme::FONT_SIZE_SM)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..iced::Font::DEFAULT
                }),
        )
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
    // section: Basic
    // The four looks the Slint example opens with: the plain choice, the one
    // with a supporting line, the indeterminate dash, and a disabled control.
    let basic = column![
        Checkbox::new(app.terms)
            .label("Accept terms and conditions")
            .on_toggle(Message::Terms),
        Checkbox::new(app.product_updates)
            .label("Receive product updates")
            .description("Occasional emails about new releases.")
            .on_toggle(Message::ProductUpdates),
        // `indeterminate(true)` draws the minus while the box is unchecked;
        // toggling it on draws the tick, as Slint's does.
        Checkbox::new(app.some)
            .indeterminate(true)
            .label("Indeterminate")
            .on_toggle(Message::Indeterminate),
        // No message is how iced draws a control that cannot be pressed.
        Checkbox::new(false).label("Disabled"),
    ]
    .spacing(12.0);

    // section: Default
    // Checked and unchecked options mixed freely in one column.
    let default = column![
        Checkbox::new(app.default_updates)
            .label("Product updates")
            .on_toggle(Message::DefaultUpdates),
        Checkbox::new(app.default_remember)
            .label("Remember this device")
            .on_toggle(Message::DefaultRemember),
    ]
    .spacing(12.0);

    // section: Sizes
    // The box's side is 12 / 14 / 16 / 18 across the size ladder.
    let sizes = row![
        Checkbox::new(app.size_xs)
            .size(ControlSize::XSmall)
            .label("XS")
            .on_toggle(Message::SizeXs),
        Checkbox::new(app.size_sm)
            .size(ControlSize::Small)
            .label("SM")
            .on_toggle(Message::SizeSm),
        Checkbox::new(app.size_md)
            .size(ControlSize::Medium)
            .label("MD")
            .on_toggle(Message::SizeMd),
        Checkbox::new(app.size_lg)
            .size(ControlSize::Large)
            .label("LG")
            .on_toggle(Message::SizeLg),
    ]
    .spacing(16.0);

    // section: Controlled Checkbox
    // The value lives on the application: `on_toggle` reports the requested
    // value, `update` stores it, and the next frame draws it — what Slint's
    // `checked <=> root.updates` binding does.
    let controlled = column![
        Checkbox::new(app.controlled)
            .label("Receive product updates")
            .on_toggle(Message::Controlled),
        muted_line(if app.controlled {
            "Subscribed"
        } else {
            "Not subscribed"
        }),
    ]
    .spacing(8.0);

    // section: Disabled State
    // The two disabled looks, one on and one off.
    let disabled = column![
        Checkbox::new(true).label("Checked"),
        Checkbox::new(false).label("Unchecked"),
    ]
    .spacing(12.0);

    // section: Without label
    // The box alone, with no line beside it.
    let without_label = Checkbox::new(app.without_label).on_toggle(Message::WithoutLabel);

    // section: Labels
    // iced has one supporting slot under the label, `description`, where Slint
    // nests a second `Text`. The page's wrapping and markdown children have no
    // iced counterpart, so the supporting sentence is the description here.
    let labels = column![
        Checkbox::new(app.label_updates)
            .label("Automatic updates")
            .description("Download updates when the application is idle.")
            .on_toggle(Message::LabelUpdates),
        Checkbox::new(app.label_notify)
            .label("Notify me when a new device signs in to my account")
            .on_toggle(Message::LabelNotify),
        Checkbox::new(app.label_terms)
            .label("Accept the terms")
            .description("Read the terms of service before continuing.")
            .on_toggle(Message::LabelTerms),
    ]
    .spacing(12.0);

    // section: Checkbox List
    // Independent choices: each box carries its own value.
    let list = column![
        Checkbox::new(app.option_1)
            .label("Option 1")
            .on_toggle(Message::Option1),
        Checkbox::new(app.option_2)
            .label("Option 2")
            .on_toggle(Message::Option2),
        Checkbox::new(app.option_3)
            .label("Option 3")
            .on_toggle(Message::Option3),
    ]
    .spacing(12.0);

    // section: Form Integration
    // Two boxes of one form, each with its own value on the owner.
    let form = column![
        Checkbox::new(app.form_terms)
            .label("I agree to the terms and conditions")
            .on_toggle(Message::FormTerms),
        Checkbox::new(app.form_newsletter)
            .label("Subscribe to newsletter")
            .on_toggle(Message::FormNewsletter),
    ]
    .spacing(12.0);

    // section: end
    let status =
        text("Click a box, or a heading and its code icon — the page scrolls to that sample.");

    let gallery = column![
        heading(app, 0, "Basic"),
        body_box(basic),
        heading(app, 1, "Default"),
        body_box(default),
        heading(app, 2, "Sizes"),
        body_box(sizes),
        heading(app, 3, "Controlled Checkbox"),
        body_box(controlled),
        heading(app, 4, "Disabled State"),
        body_box(disabled),
        heading(app, 5, "Without label"),
        body_box(without_label),
        heading(app, 6, "Labels"),
        body_box(labels),
        heading(app, 7, "Checkbox List"),
        body_box(list),
        heading(app, 8, "Form Integration"),
        body_box(form),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
