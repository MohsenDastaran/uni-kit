//! `stepper` — the whole gallery the Slint example shows: the four-step order
//! with icons and its Previous/Next row, the centred text stepper, the vertical
//! one, and the size ladder beside a disabled step and a disabled stepper.
//!
//! Run it with `cargo run --example stepper`, or in the browser as
//! `?story=stepper` in the gallery the website embeds.
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
//! **The Previous and Next step is shared, not re-derived.** Slint's example
//! moves `order-step` with `+= 1` guarded by `enabled: order-step < last`, and
//! its `FocusScope` moves it again with the arrow keys. Both are the bounded step
//! `number_input`'s own steppers perform, so this demo calls
//! [`stepper::step_index`], which is built on `number_input::step_value`. A step
//! at the last index is `None`, and a button with no message is exactly iced's
//! disabled button, which is how the guard and the step become one thing.
//!
//! **The arrow keys are the application's.** iced reports no focus to a widget,
//! so the stepper cannot read `←`/`→` the way Slint's `FocusScope` does; the
//! subscription below moves the order stepper rather than faking a focused one.
//!
//! **The icon is the caller's, so the caller tints it.** iced ships no icon set,
//! and the stepper's indicator draws whatever element the item was given, so the
//! tint Slint computes from `checked` — `primary-foreground` on a checked
//! indicator, `secondary-foreground` otherwise — is computed here from the same
//! `index <= selected` rule the control uses. The marks are the Lucide glyphs the
//! Slint example names (`calendar`, `inbox`, `layers`, `info`, `house`,
//! `sparkles`, `folder`, `circle-check`).

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Font, Length, Subscription, Task, Theme, font};

use crate::components::button::{self, Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::number_input::StepAction;
use crate::components::stepper::{self, Stepper, StepperItem};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — stepper")
        .theme(theme)
        .subscription(subscription)
        .run()
}

/// Which stepper a message is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    /// The icon stepper the Previous and Next buttons move.
    Order,
    Center,
    Vertical,
    /// The three size steppers, which Slint binds to one selection.
    Size,
    /// The stepper with a disabled middle step.
    Gap,
    /// The API reference card's own control.
    Api,
}

#[derive(Debug)]
struct App {
    order_step: usize,
    center_step: usize,
    vertical_step: usize,
    /// The three size steppers share this, the way Slint's `size-step` is
    /// two-way bound to all three.
    size_step: usize,
    gap_step: usize,
    api_step: usize,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A step was pressed, and the stepper reports the index it wants.
    Selected(Slot, usize),
    /// The Previous or Next button asked for the neighbour of the order stepper.
    OrderStep(StepAction),
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
        order_step: 1,
        center_step: 0,
        vertical_step: 2,
        size_step: 1,
        gap_step: 0,
        api_step: 1,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Selected(slot, index) => match slot {
            Slot::Order => app.order_step = index,
            Slot::Center => app.center_step = index,
            Slot::Vertical => app.vertical_step = index,
            Slot::Size => app.size_step = index,
            Slot::Gap => app.gap_step = index,
            Slot::Api => app.api_step = index,
        },
        Message::OrderStep(action) => {
            // The disabled flags of the order stepper, all enabled. The bounded
            // step is `step_index`'s, which is `number_input::step_value`'s.
            let disabled = [false; 4];
            if let Some(index) = stepper::step_index(&disabled, app.order_step, action) {
                app.order_step = index;
            }
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

fn subscription(_app: &App) -> Subscription<Message> {
    // Slint's `FocusScope` moves the selected step with the arrow keys. iced
    // reports no focus to the application, so the subscription moves the order
    // stepper itself rather than pretending to know which control is focused.
    iced::keyboard::listen().filter_map(|event| match event {
        iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowRight),
            ..
        } => Some(Message::OrderStep(StepAction::Increment)),
        iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowLeft),
            ..
        } => Some(Message::OrderStep(StepAction::Decrement)),
        _ => None,
    })
}

/// A step's mark from the set the Slint example names — `icon: "calendar"`
/// there, the same Lucide glyph here. iced ships no icon set, so the artwork is
/// the caller's, and the caller tints it the way the indicator would: the
/// foreground of a checked indicator, the secondary foreground otherwise, and
/// muted for a disabled step.
fn step_glyph(
    size: ControlSize,
    checked: bool,
    disabled: bool,
    markup: &'static [u8],
) -> Element<'static, Message> {
    let edge = match size {
        ControlSize::Small => 12.0,
        ControlSize::Large => 18.0,
        _ => 14.0,
    };

    svg(svg::Handle::from_memory(markup))
        .width(edge)
        .height(edge)
        .style(move |theme: &Theme, _status| {
            let tokens = Tokens::of(theme);
            let base = if checked {
                tokens.primary_foreground
            } else {
                tokens.secondary_foreground
            };
            svg::Style {
                color: Some(if disabled {
                    theme::alpha(base, 0.5)
                } else {
                    base
                }),
            }
        })
        .into()
}

/// A chevron for a Previous or Next button, in the ink the button's own variant
/// uses, muted when the button has no message to send.
fn chevron<'a>(markup: &'static [u8], muted: bool) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(16.0)
        .height(16.0)
        .style(move |theme: &Theme, _status| {
            let tokens = Tokens::of(theme);
            svg::Style {
                color: Some(if muted {
                    tokens.muted_foreground
                } else {
                    button::ink(tokens, Variant::Default, false)
                }),
            }
        })
        .into()
}

/// A Previous or Next control: the Slint example's button, disabled when the step
/// cannot move — no message is how iced draws that, so the guard and the step are
/// the same decision.
fn order_button(
    label: &'static str,
    markup: &'static [u8],
    message: Option<Message>,
    variant: Variant,
) -> Button<'static, Message> {
    let control = Button::new(label)
        .variant(variant)
        .icon(chevron(markup, message.is_none()));

    match message {
        Some(message) => control.on_press(message),
        None => control,
    }
}

/// Muted text under an example, carrying what the demo has to say about it.
fn muted_line<'a>(line: &str) -> Element<'a, Message> {
    text(line.to_string())
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
        container(text(title).size(theme::FONT_SIZE_SM).font(Font {
            weight: font::Weight::Medium,
            ..Font::DEFAULT
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

/// The four order steps, each with its own icon, tinted for the selection they
/// are given.
fn order_items(selected: usize) -> Vec<StepperItem<'static, Message>> {
    const LABELS: [&str; 4] = ["Order Details", "Shipping", "Preview", "Finish"];
    const MARKS: [&[u8]; 4] = [CALENDAR, INBOX, LAYERS, INFO];

    LABELS
        .iter()
        .zip(MARKS)
        .enumerate()
        .map(|(index, (label, markup))| {
            StepperItem::new().label(*label).icon(step_glyph(
                ControlSize::Medium,
                index <= selected,
                false,
                markup,
            ))
        })
        .collect()
}

/// The three vertical steps, each with an icon and a description.
fn vertical_items(selected: usize) -> Vec<StepperItem<'static, Message>> {
    const LABELS: [&str; 4] = ["Step 1", "Step 2", "Step 3", "Step 4"];
    const DESCRIPTIONS: [&str; 4] = [
        "Description for step 1.",
        "Description for step 2.",
        "Description for step 3.",
        "Description for step 4.",
    ];
    const MARKS: [&[u8]; 4] = [HOUSE, SPARKLES, FOLDER, CIRCLE_CHECK];

    (0..4)
        .map(|index| {
            StepperItem::new()
                .label(LABELS[index])
                .description(DESCRIPTIONS[index])
                .icon(step_glyph(
                    ControlSize::Medium,
                    index <= selected,
                    false,
                    MARKS[index],
                ))
        })
        .collect()
}

/// The three size steps, "Cart / Address / Payment" as the Slint example names
/// them.
fn size_items() -> Vec<StepperItem<'static, Message>> {
    ["Cart", "Address", "Payment"]
        .into_iter()
        .map(|label| StepperItem::new().label(label))
        .collect()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: With icons
    // The order stepper and the row beneath it: the count, and the two buttons
    // that move the selection. Each button asks `step_index` for its neighbour,
    // and a button with no neighbour has no message, which is iced's disabled
    // button.
    let order = Stepper::new()
        .items(order_items(app.order_step))
        .selected_index(app.order_step)
        .on_change(|index| Message::Selected(Slot::Order, index));

    let count = 4;
    let previous = stepper::step_index(&[false; 4], app.order_step, StepAction::Decrement)
        .map(|_| Message::OrderStep(StepAction::Decrement));
    let next = stepper::step_index(&[false; 4], app.order_step, StepAction::Increment)
        .map(|_| Message::OrderStep(StepAction::Increment));
    let last = app.order_step + 1 == count;

    let icons = column![
        order,
        row![
            text(format!("Step {} of {}", app.order_step + 1, count))
                .size(theme::FONT_SIZE_SM)
                .width(Length::Fill)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
            order_button("Previous", CHEVRON_LEFT, previous, Variant::Default),
            order_button(
                if last { "Done" } else { "Next" },
                CHEVRON_RIGHT,
                next,
                Variant::Primary,
            ),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    ]
    .spacing(16.0);

    // section: Text center
    // Each item's indicator is centred over its label and description, and the
    // connector crosses the item on both sides of it.
    let center = Stepper::new()
        .items([
            StepperItem::new()
                .label("Step 1")
                .description("Desc for step 1."),
            StepperItem::new()
                .label("Step 2")
                .description("Desc for step 2."),
            StepperItem::new()
                .label("Step 3")
                .description("Desc for step 3."),
        ])
        .text_center(true)
        .selected_index(app.center_step)
        .on_change(|index| Message::Selected(Slot::Center, index));

    // section: Vertical
    // The indicators run down the leading edge with the labels and descriptions
    // beside them, and the connector runs between the indicators.
    let vertical = Stepper::vertical()
        .items(vertical_items(app.vertical_step))
        .selected_index(app.vertical_step)
        .on_change(|index| Message::Selected(Slot::Vertical, index));

    // section: Sizes and disabled
    // The three size steppers share one selection, the way Slint binds all three
    // to `size-step`: clicking any of them moves the three together. Beside them
    // a single disabled step, and a whole stepper disabled with `enabled: false`
    // — which is the absence of a message here, not a faded subtree.
    let sizes = column![
        Stepper::new()
            .size(ControlSize::XSmall)
            .items(size_items())
            .selected_index(app.size_step)
            .on_change(|index| Message::Selected(Slot::Size, index)),
        Stepper::new()
            .size(ControlSize::Small)
            .items(size_items())
            .selected_index(app.size_step)
            .on_change(|index| Message::Selected(Slot::Size, index)),
        Stepper::new()
            .size(ControlSize::Large)
            .items(size_items())
            .selected_index(app.size_step)
            .on_change(|index| Message::Selected(Slot::Size, index)),
        Stepper::new()
            .items([
                StepperItem::new().label("Available"),
                StepperItem::new().label("Locked").disabled(true),
                StepperItem::new().label("Available"),
            ])
            .selected_index(app.gap_step)
            .on_change(|index| Message::Selected(Slot::Gap, index)),
        Stepper::new()
            .items(size_items())
            .selected_index(1)
            .disabled(true),
        muted_line("The xsmall, small and large steppers share one selection; the locked step refuses a press, and the last stepper refuses every one."),
    ]
    .spacing(theme::SPACING_LG);

    // section: API Reference
    // One control built with every method the reference lists, and the step a
    // programmatic Previous/Next takes: `stepper::step_index`, which is
    // `number_input::step_value` plus the rule that a disabled step is passed
    // over.
    let api = column![
        Stepper::new()
            .items([
                StepperItem::new().label("Cart"),
                StepperItem::new().label("Address"),
                StepperItem::new().label("Payment"),
            ])
            .selected_index(app.api_step)
            .size(ControlSize::Medium)
            .text_center(false)
            .disabled(false)
            .width(Length::Fill)
            .on_change(|index| Message::Selected(Slot::Api, index)),
        muted_line("step_index(&[false, true, false], 0, Increment) passes the locked step and lands on Payment."),
    ]
    .spacing(10.0);

    // section: end
    let status =
        text("Click a step, or step the order with Previous and Next — the gallery is live.")
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            });

    let gallery = column![
        heading(app, 0, "With icons"),
        body_box(icons),
        heading(app, 1, "Text center"),
        body_box(center),
        heading(app, 2, "Vertical"),
        body_box(vertical),
        heading(app, 3, "Sizes and disabled"),
        body_box(sizes),
        heading(app, 4, "API Reference"),
        body_box(api),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
const CALENDAR: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M8 2v4"/><path d="M16 2v4"/><rect width="18" height="18" x="3" y="4" rx="2"/><path d="M3 10h18"/></svg>"#;
const INBOX: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/></svg>"#;
const LAYERS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z"/><path d="m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65"/><path d="m22 12.65-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65"/></svg>"#;
const INFO: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#;
const HOUSE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"/><path d="M3 10a2 2 0 0 1 .709-1.528l7-5.999a2 2 0 0 1 2.582 0l7 5.999A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/></svg>"#;
const SPARKLES: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9.937 15.5A2 2 0 0 0 8.5 14.063l-6.135-1.582a.5.5 0 0 1 0-.962L8.5 9.936A2 2 0 0 0 9.937 8.5l1.582-6.135a.5.5 0 0 1 .963 0L14.063 8.5A2 2 0 0 0 15.5 9.937l6.135 1.581a.5.5 0 0 1 0 .964L15.5 14.063a2 2 0 0 0-1.437 1.437l-1.582 6.135a.5.5 0 0 1-.963 0z"/><path d="M20 3v4"/><path d="M22 5h-4"/><path d="M4 17v2"/><path d="M5 18H3"/></svg>"#;
const FOLDER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/></svg>"#;
const CIRCLE_CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/></svg>"#;
const CHEVRON_LEFT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>"#;
const CHEVRON_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#;
