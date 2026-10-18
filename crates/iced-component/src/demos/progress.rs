//! `progress` — a linear bar and a circle, at a value the caller advances.
//!
//! iced's timer needs a runtime feature this crate does not enable, so the
//! component takes the value instead. The buttons below stand in for the
//! subscription an application would run: `Advance` and `Reset` move the value,
//! and `Loading` moves the bar and its circle between the determinate and
//! indeterminate forms, which take the same value as their phase.
//!
//! Each section carries the heading the page gives its sample — the title, and
//! the code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use.
//!
//! The `// section: <title>` comments below are not decoration: the website reads
//! them to cut this file into the usage panels a component page shows, so the code
//! on the page is the code that runs here rather than a second copy of it. Keep
//! them, one per section, in the order the sections appear, and keep the closing
//! `// section: end` after the last one — without it the lines that assemble the
//! gallery would be shown as part of the last section.
//!
//! Run it with `cargo run --example progress`, or in the browser as
//! `?story=progress` in the gallery the website embeds.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::{Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::progress::{Progress, ProgressCircle};
use crate::components::theme::{self, ControlSize, Status, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    // `cx.theme()` is not available while a view is built, so the palette the
    // custom-style section names is read once, here, and carried on the state.
    let tokens = Tokens::of(&theme);
    iced::application(move || boot(tokens), update, view)
        .title("uni-kit — progress")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone)]
struct App {
    value: f32,
    /// Whether the bar and the circle are in their indeterminate form. The
    /// `Loading` button is the only thing that moves it, so the state can be
    /// entered and left without restarting the demo.
    loading: bool,
    /// The palette the custom-style section hands to the bar's fill and edge.
    tokens: Tokens,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Step,
    Reset,
    ToggleLoading,
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot(tokens: Tokens) -> App {
    App {
        value: 0.35,
        loading: false,
        tokens,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Step => {
            app.value += 0.05;
            if app.value > 1.0 {
                app.value = 0.0;
            }
        }
        Message::Reset => app.value = 0.0,
        Message::ToggleLoading => app.loading = !app.loading,
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

fn caption(
    value: f32,
    loading: bool,
) -> iced::widget::Text<'static, iced::Theme, iced::Renderer> {
    // The value is ignored while loading — it is the band's phase instead — so
    // the percentage is not shown then, which is the Slint demo's reading.
    let label = if loading {
        "…".to_string()
    } else {
        format!("{:.0}%", value * 100.0)
    };
    text(label)
        .size(14.0)
        .style(|theme: &iced::Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
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
    let value = app.value;

    // section: Progress
    // The buttons stand in for the subscription an application would run: each
    // press moves the value the bar and the loading form read, and `Loading` is
    // the one that changes the form rather than the value.
    let progress = column![
        row![
            Button::new("Advance")
                .variant(Variant::Primary)
                .on_press(Message::Step),
            Button::new("Reset").on_press(Message::Reset),
            Button::new("Loading")
                .selected(app.loading)
                .on_press(Message::ToggleLoading),
            caption(value, app.loading),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
        Progress::new(value),
        text(if app.loading {
            "Loading: the value is the phase of the band."
        } else {
            "Determinate: the value is the filled width."
        })
        .size(14.0),
        Progress::new(value).loading(app.loading),
    ]
    .spacing(12);

    // section: Values
    let values = column![
        Progress::new(0.0),
        Progress::new(0.25),
        Progress::new(0.5),
        Progress::new(0.75),
        Progress::new(1.0),
    ]
    .spacing(12);

    // section: Sizes and status colors
    let sizes = column![
        text("The four heights: 4, 6, 8 and 10 pixels.").size(14.0),
        Progress::new(value).size(ControlSize::XSmall),
        Progress::new(value).size(ControlSize::Small),
        Progress::new(value),
        Progress::new(value).size(ControlSize::Large),
        text("Two of the status colours.").size(14.0),
        Progress::new(value).status(Status::Success),
        Progress::new(value).status(Status::Danger),
    ]
    .spacing(12);

    // section: Progress circle
    let circles = column![
        text("The circle, at 8, 12, 16 and 20 across.").size(14.0),
        row![
            ProgressCircle::new(value).size(ControlSize::XSmall),
            ProgressCircle::new(value).size(ControlSize::Small),
            ProgressCircle::new(value),
            ProgressCircle::new(value).size(ControlSize::Large),
            ProgressCircle::new(value).diameter(48.0),
            ProgressCircle::new(value).loading(app.loading),
            ProgressCircle::new(value).status(Status::Warning),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
    ]
    .spacing(12);

    // section: Custom Style
    // The page's own sample — a 16-pixel bar with a 2-pixel corner and a 2-pixel
    // edge, none of which the size ladder above can express. Its `green_light`
    // fill and `green` edge are the success token at half strength and whole.
    let custom_style = column![
        Progress::new(value)
            .height(16.0)
            .rounded(2.0)
            .color(theme::alpha(app.tokens.success, 0.5))
            .border(2.0)
            .border_color(app.tokens.success),
    ]
    .spacing(12);
    // section: end

    let gallery = column![
        heading(app, 0, "Progress"),
        body_box(progress),
        heading(app, 1, "Values"),
        body_box(values),
        heading(app, 2, "Sizes and status colors"),
        body_box(sizes),
        heading(app, 3, "Progress circle"),
        body_box(circles),
        heading(app, 4, "Custom Style"),
        body_box(custom_style),
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
