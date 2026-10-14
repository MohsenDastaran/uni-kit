//! `carousel` — a paginated viewer for a set of related slides: the viewport,
//! the previous and next controls, the pagination dots, the looping wrap and the
//! vertical layout, shown the way the website's Carousel page documents them.
//!
//! Run it with `cargo run --example carousel`, or in the browser as
//! `?story=carousel` in the gallery the website embeds.
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
//! The titles are the page's own `##` sections rather than the Slint example's
//! six GroupBoxes: `carousel.md` documents this component at that depth, and
//! `icedSections` reads the same titles back. Two of the page's sections carry a
//! sample iced can only answer honestly — **Events**, where a request is a
//! `Message` rather than an event stream, and **Accessibility**, where iced 0.14
//! has no accessibility tree and the caption is the visible substitute.
//!
//! # What this is, said plainly
//!
//! **A carousel without a timer is a paginated viewer.** This crate's runtime is
//! `thread-pool`, so `iced::time::every` has no backend and the component cannot
//! own an autoplay timer — the application would have to, and nothing here does.
//! The track does not scroll or snap either, so this is the Slint component's
//! shape: a window of slides moved by the arrows, the dots and the keyboard.

use iced::font::Weight;
use iced::widget::{column, container, mouse_area, row, svg, text};
use iced::{Alignment, Element, Font, Length, Task, Theme};

use crate::components::button::Button;
use crate::components::carousel::{Carousel, Orientation};
use crate::components::group_box::{self, GroupBox};
use crate::components::scrollable::{Axis, Scrollable};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — carousel")
        .theme(theme)
        .subscription(subscription)
        .run()
}

/// Which carousel a message is about. Each section owns its own index, because
/// the application owns every selection — the component never moves itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Usage,
    Composition,
    Multiple,
    Orientation,
    Looping,
    Controlled,
    Events,
    Pagination,
    SizeSmall,
    SizeLarge,
    Custom,
    Accessibility,
}

/// A keyboard step, the page's Orientation section: Left and Right on a
/// horizontal carousel, Up and Down on a vertical one, Home and End at the ends.
#[derive(Debug, Clone, Copy)]
enum Step {
    Next,
    Previous,
    First,
    Last,
}

#[derive(Debug)]
struct App {
    usage: usize,
    composition: usize,
    multiple: usize,
    orientation: usize,
    looping: usize,
    controlled: usize,
    events: usize,
    pagination: usize,
    size_small: usize,
    size_large: usize,
    custom: usize,
    accessibility: usize,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            usage: 0,
            composition: 0,
            multiple: 0,
            orientation: 0,
            looping: 0,
            // The Slint `Controlled` section starts on its second slide.
            controlled: 1,
            events: 0,
            pagination: 0,
            size_small: 0,
            size_large: 0,
            custom: 0,
            accessibility: 0,
            hovered: None,
            pressed_source: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Select(Slot, usize),
    Step(Step),
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
        Message::Select(slot, index) => match slot {
            Slot::Usage => app.usage = index,
            Slot::Composition => app.composition = index,
            Slot::Multiple => app.multiple = index,
            Slot::Orientation => app.orientation = index,
            Slot::Looping => app.looping = index,
            Slot::Controlled => app.controlled = index,
            Slot::Events => app.events = index,
            Slot::Pagination => app.pagination = index,
            Slot::SizeSmall => app.size_small = index,
            Slot::SizeLarge => app.size_large = index,
            Slot::Custom => app.custom = index,
            Slot::Accessibility => app.accessibility = index,
        },
        Message::Step(step) => {
            // iced reports no focus to the application, so the subscription
            // cannot know which carousel is focused and drives the first one —
            // the same limitation `stepper`'s demo names.
            let count = 3usize;
            app.usage = match step {
                Step::Next => (app.usage + 1).min(count - 1),
                Step::Previous => app.usage.saturating_sub(1),
                Step::First => 0,
                Step::Last => count - 1,
            };
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

fn subscription(_app: &App) -> iced::Subscription<Message> {
    // Slint's `FocusScope` moves the carousel with the arrow keys and jumps with
    // Home and End. iced reports no focus here, so the application owns the key
    // path and points it at the first carousel.
    iced::keyboard::listen().filter_map(|event| match event {
        iced::keyboard::Event::KeyPressed { key, .. } => match key.as_ref() {
            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight) => {
                Some(Message::Step(Step::Next))
            }
            iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft) => {
                Some(Message::Step(Step::Previous))
            }
            iced::keyboard::Key::Named(iced::keyboard::key::Named::Home) => {
                Some(Message::Step(Step::First))
            }
            iced::keyboard::Key::Named(iced::keyboard::key::Named::End) => {
                Some(Message::Step(Step::Last))
            }
            _ => None,
        },
        _ => None,
    })
}

/// One slide's content: the number, semibold, at the size the section asks for.
/// The card around it — the border, the radius, the centring — is the
/// component's, so a caller's slide is only its content.
fn slide<'a>(label: impl Into<String>, size: f32) -> Element<'a, Message> {
    text(label.into())
        .size(size)
        .font(Font {
            weight: Weight::Semibold,
            ..Font::DEFAULT
        })
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).foreground),
        })
        .into()
}

/// A run of numbered slides, `1..=count`.
fn slides<'a>(count: usize, size: f32) -> Vec<Element<'a, Message>> {
    (0..count)
        .map(|index| slide((index + 1).to_string(), size))
        .collect()
}

/// A live caption. The page's Events and Accessibility sections both want one:
/// the first shows that a control reports the slide it wants, the second is the
/// visible substitute for the accessibility tree iced 0.14 does not carry.
fn caption<'a>(label: impl Into<String>) -> Element<'a, Message> {
    text(label.into())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
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

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Usage
    // One full-width slide at a time, the Slint `Basic`: previous, viewport,
    // next, and the pagination dots under them. The arrows at the ends are
    // disabled, because the index cannot leave the first or last slide.
    let usage = Carousel::new(slides(3, 36.0))
        .index(app.usage)
        .pagination(true)
        .on_select(|index| Message::Select(Slot::Usage, index));

    // section: Composition
    // `Carousel` is the whole composition: the viewport, the two controls and
    // the dots are its parts rather than five types to keep in step with a
    // count. A caller composes a slide by handing over any element, and every
    // part reports through the one `on_select`, so there is no second
    // navigation path to keep in step.
    let composed = Carousel::new(slides(4, 36.0))
        .index(app.composition)
        .controls(true)
        .pagination(true)
        .on_select(|index| Message::Select(Slot::Composition, index));

    // section: Multiple items
    // A viewport several slides wide. A fractional slide is not possible —
    // the window is whole slides, which is what makes this a paginated viewer
    // rather than a track that scrolls — so five slides show three at a time.
    let multiple = Carousel::new(slides(5, 24.0))
        .index(app.multiple)
        .visible(3)
        .on_select(|index| Message::Select(Slot::Multiple, index));

    // section: Orientation
    // Vertical turns the browsing axis and the arrows with it: previous above,
    // next below, and a 192-pixel viewport for two 90-pixel slides. Left and
    // Right become Up and Down.
    let orientation = Carousel::new(slides(3, 30.0))
        .index(app.orientation)
        .orientation(Orientation::Vertical)
        .visible(2)
        .frame(192.0)
        .cross(256.0)
        .on_select(|index| Message::Select(Slot::Orientation, index));

    // section: Looping
    // Looping wraps from the last slide to the first, so neither arrow is ever
    // disabled. The window wraps through the caller's slides with it.
    let looping = Carousel::new(slides(4, 36.0))
        .index(app.looping)
        .looping(true)
        .pagination(true)
        .on_select(|index| Message::Select(Slot::Looping, index));

    // section: Controlled selection
    // The index is the application's: the carousel is told where it is and
    // reports where a press wants to go. "Go to first" and "Go to last" move the
    // same state the arrows do, which is what makes the readout and the slide
    // agree.
    let controlled = Carousel::new(slides(3, 36.0))
        .index(app.controlled)
        .on_select(|index| Message::Select(Slot::Controlled, index));
    let controlled_readout = caption(format!("Selected slide: {} of 3", app.controlled + 1));
    let controlled_actions = row![
        Button::new("Go to first")
            .size(ControlSize::Small)
            .on_press(Message::Select(Slot::Controlled, 0)),
        Button::new("Go to last")
            .size(ControlSize::Small)
            .on_press(Message::Select(Slot::Controlled, 2)),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    // section: Events
    // iced has no event stream to subscribe to here: a control reports the slide
    // it wants as the very `Message` the application's `update` already handles,
    // and the application decides what the new index means. The caption is that
    // round trip made visible.
    let events = Carousel::new(slides(3, 36.0))
        .index(app.events)
        .on_select(|index| Message::Select(Slot::Events, index));
    let events_readout = caption(format!("Selected slide: {} of 3", app.events + 1));

    // section: Pagination indicators
    // The dots are optional and are controls of their own: each stands for a
    // page and reports it through the same callback the arrows use. With the
    // arrows off, the dots are the whole navigation.
    let pagination = Carousel::new(slides(5, 36.0))
        .index(app.pagination)
        .controls(false)
        .pagination(true)
        .on_select(|index| Message::Select(Slot::Pagination, index));

    // section: Control size
    // One semantic size for the arrows, the dots and the label, so a carousel
    // scales as a whole. Small is a 24-pixel control and large a 40-pixel one.
    let size_small = Carousel::new(slides(3, 28.0))
        .index(app.size_small)
        .size(ControlSize::Small)
        .frame(160.0)
        .pagination(true)
        .on_select(|index| Message::Select(Slot::SizeSmall, index));
    let size_large = Carousel::new(slides(3, 28.0))
        .index(app.size_large)
        .size(ControlSize::Large)
        .frame(160.0)
        .pagination(true)
        .on_select(|index| Message::Select(Slot::SizeLarge, index));

    // section: Custom controls
    // A label replaces the direction-appropriate chevron and grows the control
    // into a pill, which is Slint's `previous-text` and `next-text`. The
    // navigation and the disabled boundary are the same either way.
    let custom = Carousel::new(slides(3, 36.0))
        .index(app.custom)
        .previous_label("Back")
        .next_label("Forward")
        .on_select(|index| Message::Select(Slot::Custom, index));

    // section: Accessibility
    // iced 0.14 exposes no accessibility tree, so a carousel cannot announce a
    // region role or a position in the set the way GPUI's `aria_position_in_set`
    // does. The caption is the honest substitute: the position is on screen, and
    // the keyboard path is the application's subscription.
    let accessibility = Carousel::new(slides(3, 36.0))
        .index(app.accessibility)
        .pagination(true)
        .on_select(|index| Message::Select(Slot::Accessibility, index));
    let accessibility_readout = caption(format!(
        "Slide {} of 3 — the caption the accessibility tree would carry",
        app.accessibility + 1
    ));

    // section: end
    let status = caption(
        "Press an arrow or a dot to move a slide — press a heading or its code icon for that sample. \
         Without a timer this is a paginated viewer.",
    );

    // Each column fills its half: a `Length::Fill` child inside a shrinking
    // column is laid out against a zero cross size, which is the trap the
    // roadmap records for a `Fill` child in a `Shrink` box.
    let size_row = row![
        column![caption("Small"), size_small]
            .spacing(8.0)
            .width(Length::Fill)
            .align_x(Alignment::Center),
        column![caption("Large"), size_large]
            .spacing(8.0)
            .width(Length::Fill)
            .align_x(Alignment::Center),
    ]
    .spacing(32.0)
    .width(Length::Fill)
    .align_y(Alignment::Start);

    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Composition"),
        body_box(composed),
        heading(app, 2, "Multiple items"),
        body_box(multiple),
        heading(app, 3, "Orientation"),
        body_box(orientation),
        heading(app, 4, "Looping"),
        body_box(looping),
        heading(app, 5, "Controlled selection"),
        body_box(
            column![
                controlled,
                row![controlled_readout, controlled_actions]
                    .spacing(16.0)
                    .align_y(Alignment::Center),
            ]
            .spacing(12.0)
            .width(Length::Fill),
        ),
        heading(app, 6, "Events"),
        body_box(
            column![events, events_readout]
                .spacing(12.0)
                .width(Length::Fill),
        ),
        heading(app, 7, "Pagination indicators"),
        body_box(pagination),
        heading(app, 8, "Control size"),
        body_box(size_row),
        heading(app, 9, "Custom controls"),
        body_box(custom),
        heading(app, 10, "Accessibility"),
        body_box(
            column![accessibility, accessibility_readout]
                .spacing(12.0)
                .width(Length::Fill),
        ),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The gallery's own scroll area is a finished component the page documents,
    // so the frame around the samples is the thing beside the one under test.
    Scrollable::new(gallery)
        .axis(Axis::Vertical)
        .height(Length::Fill)
        .into()
}

/// The code icon beside a heading, `code` from the Lucide set.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
