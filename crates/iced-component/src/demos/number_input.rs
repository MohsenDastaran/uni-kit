//! `number-input` — the numeric field's whole gallery the way the Slint example
//! shows it: the quantity, the bounds and the step, the prefix and suffix slots,
//! the four sizes, the `− value +` stepper shape, the disabled state and a text
//! that is not a number.
//!
//! Run it with `cargo run --example number-input`, or in the browser as
//! `?story=number-input` in the gallery the website embeds.
//!
//! Each section carries a heading the component page has — the title, and the code
//! icon that asks the embedding page to scroll to that sample. Clicking either one
//! posts the section's index and title through [`crate::bridge`], the same bridge
//! the Slint and GPUI galleries use. The `// section: <title>` comments below are
//! the contract the website reads to cut this file into the usage panels a
//! component page shows, so the code on the page is the code that runs here. Keep
//! them, one per section, in order, and keep the closing `// section: end` after
//! the last one.
//!
//! The four sections are the page's four second-level headings. The three beyond
//! `Usage` do not describe a widget — one is a key table, one is prose, one is an
//! API listing — so each builds the closest real thing instead, and says in the
//! card what it could not express:
//!
//! - **Keyboard Navigation** is the application's here. iced's field reports text
//!   and submit only, so `↑`, `↓` and `Escape` are a `keyboard::listen`
//!   subscription in this demo, not a property of the control; `Enter` is
//!   `on_submit`; `Tab` and `Shift+Tab` are iced's own focus traversal. iced
//!   reports no focus to the application either, so the subscription cannot know
//!   which field is focused and steps the one the card names.
//! - **Best Practices** is prose, and a widget cannot be prose: the card builds
//!   the rules the control does enforce — a step, a range and the two bounds — and
//!   leaves validation, formatting and error copy, which are the application's,
//!   to the danger edge and the readout.
//! - **API Reference** is a table, so the card is the table in use: one control
//!   built with every method the reference lists, and the two helpers a
//!   programmatic step needs, [`step_value`](crate::components::number_input::step_value)
//!   and [`shown`](crate::components::number_input::shown).

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{font, Alignment, Element, Font, Length, Subscription, Task, Theme};

use crate::components::button::Button;
use crate::components::group_box::{self, GroupBox};
use crate::components::number_input::{self, NumberInput, StepAction};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — number-input")
        .theme(theme)
        .subscription(subscription)
        .run()
}

/// One numeric control's state: the number the steppers step and the text the
/// field shows. iced has no input state to own either, so the caller holds both —
/// `value` is what a step produces, `text` is what an edit produces.
#[derive(Debug, Default, Clone)]
struct FieldState {
    value: f64,
    text: String,
}

impl FieldState {
    /// A field showing its value, the way Slint's `shown` writes it.
    fn of(value: f64) -> Self {
        Self {
            value,
            text: number_input::shown(value),
        }
    }

    /// A field whose text is not a number, so the danger edge has something to
    /// report.
    fn typed(text: impl Into<String>) -> Self {
        Self {
            value: 0.0,
            text: text.into(),
        }
    }
}

/// Which control a message is about. The messages carry the slot rather than the
/// state, so one edit arm serves every field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Quantity,
    Price,
    Seats,
    AtMinimum,
    AtMaximum,
    Salary,
    Rejected,
    SizeSmall,
    SizeMedium,
    SizeLarge,
    Centered,
    Keyboard,
    PracticeSeats,
    PracticePrice,
    Api,
}

#[derive(Debug)]
struct App {
    quantity: FieldState,
    price: FieldState,
    seats: FieldState,
    at_minimum: FieldState,
    at_maximum: FieldState,
    salary: FieldState,
    rejected: FieldState,
    size_small: FieldState,
    size_medium: FieldState,
    size_large: FieldState,
    centered: FieldState,
    keyboard: FieldState,
    practice_seats: FieldState,
    practice_price: FieldState,
    api: FieldState,
    /// The text the last Enter committed, so the submit is visible.
    committed: Option<String>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A field's text changed. The caller keeps it raw — a partial number like
    /// `1.` is a state the field has to be able to hold.
    Edited(Slot, String),
    /// A stepper moved the value: already clamped to the range.
    Changed(Slot, f64),
    /// Enter, with the text that was in the field. The component never sees it, so
    /// the application clamps.
    Commit(Slot, String),
    /// The key subscription saw `↑` or `↓`.
    KeyStep(StepAction),
    /// The API card's own buttons: the programmatic step a caller performs on the
    /// value it holds.
    ApiStep(StepAction),
    /// The key subscription saw `Escape`.
    Escape,
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
        quantity: FieldState::of(3.0),
        price: FieldState::of(19.99),
        seats: FieldState::of(12.0),
        at_minimum: FieldState::of(0.0),
        at_maximum: FieldState::of(100.0),
        salary: FieldState::of(1250.0),
        rejected: FieldState::typed("abc"),
        size_small: FieldState::of(4.0),
        size_medium: FieldState::of(4.0),
        size_large: FieldState::of(4.0),
        centered: FieldState::of(42.0),
        keyboard: FieldState::of(3.0),
        practice_seats: FieldState::of(12.0),
        practice_price: FieldState::of(19.99),
        api: FieldState::of(50.0),
        committed: None,
        hovered: None,
        pressed_source: None,
    }
}

/// The state behind a slot, so one arm edits any of them.
fn field_mut(app: &mut App, slot: Slot) -> &mut FieldState {
    match slot {
        Slot::Quantity => &mut app.quantity,
        Slot::Price => &mut app.price,
        Slot::Seats => &mut app.seats,
        Slot::AtMinimum => &mut app.at_minimum,
        Slot::AtMaximum => &mut app.at_maximum,
        Slot::Salary => &mut app.salary,
        Slot::Rejected => &mut app.rejected,
        Slot::SizeSmall => &mut app.size_small,
        Slot::SizeMedium => &mut app.size_medium,
        Slot::SizeLarge => &mut app.size_large,
        Slot::Centered => &mut app.centered,
        Slot::Keyboard => &mut app.keyboard,
        Slot::PracticeSeats => &mut app.practice_seats,
        Slot::PracticePrice => &mut app.practice_price,
        Slot::Api => &mut app.api,
    }
}

/// The range the controls on the keyboard and API cards clamp a commit to.
fn ceiling(slot: Slot) -> f64 {
    match slot {
        Slot::Keyboard => 10.0,
        _ => 100.0,
    }
}

fn subscription(_app: &App) -> Subscription<Message> {
    // Slint's field forwards the keys it does not handle through `key-pressed`,
    // and `clean-on-escape` clears on Escape. iced's field reports text and submit
    // only, so the same keys are an application subscription — one that cannot know
    // which field is focused, because iced does not report that either.
    iced::keyboard::listen().filter_map(|event| match event {
        iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowUp),
            ..
        } => Some(Message::KeyStep(StepAction::Increment)),
        iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::ArrowDown),
            ..
        } => Some(Message::KeyStep(StepAction::Decrement)),
        iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape),
            ..
        } => Some(Message::Escape),
        _ => None,
    })
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Edited(slot, text) => field_mut(app, slot).text = text,
        Message::Changed(slot, value) => {
            let field = field_mut(app, slot);
            field.value = value;
            field.text = number_input::shown(value);
        }
        Message::Commit(slot, text) => {
            // A typed out-of-range value is kept while typing and clamped here,
            // which is the application's half of the Slint behaviour: the control
            // clamps what a stepper produces, the caller clamps what a person
            // types.
            if let Ok(value) = text.parse::<f64>() {
                let value = value.clamp(0.0, ceiling(slot));
                let field = field_mut(app, slot);
                field.value = value;
                field.text = number_input::shown(value);
                app.committed = Some(number_input::shown(value));
            }
        }
        Message::KeyStep(action) => {
            let keyboard = &mut app.keyboard;
            if let Some(value) = number_input::step_value(keyboard.value, action, 1.0, 0.0, 10.0) {
                keyboard.value = value;
                keyboard.text = number_input::shown(value);
            }
        }
        Message::ApiStep(action) => {
            let api = &mut app.api;
            if let Some(value) = number_input::step_value(api.value, action, 5.0, 0.0, 100.0) {
                api.value = value;
                api.text = number_input::shown(value);
            }
        }
        Message::Escape => app.keyboard.text.clear(),
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

/// Muted text in a prefix or suffix slot, for a currency mark or a unit rather
/// than a glyph.
fn slot_text<'a>(value: &'a str, size: ControlSize) -> Element<'a, Message> {
    text(value.to_string())
        .size(size.input_font_size())
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
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

/// The label to the left of a narrow counter, the shape the Slint example's
/// Integer Counter uses: it takes the row's spare width, so the control keeps the
/// width it was given.
fn row_label<'a>(label: &str, muted: bool) -> Element<'a, Message> {
    text(label.to_string())
        .size(theme::FONT_SIZE_SM)
        .width(Length::Fill)
        .style(move |theme: &Theme| text::Style {
            color: Some(if muted {
                Tokens::of(theme).muted_foreground
            } else {
                Tokens::of(theme).foreground
            }),
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
                .font(Font {
                    weight: font::Weight::Medium,
                    ..Font::DEFAULT
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
    // section: Usage
    // The Slint example's gallery in one panel: the bounds and the step, the
    // prefix and suffix slots, the four sizes, the `− value +` shape, the disabled
    // state and a value that is not a number. Every control is `value` — the number
    // a step works from — plus `text`, the caller's editing buffer.
    let usage = column![
        row![
            NumberInput::new("quantity")
                .value(app.quantity.value)
                .text(app.quantity.text.as_str())
                .minimum(0.0)
                .maximum(99.0)
                .on_input(|text| Message::Edited(Slot::Quantity, text))
                .on_change(|value| Message::Changed(Slot::Quantity, value)),
            NumberInput::new("price")
                .value(app.price.value)
                .text(app.price.text.as_str())
                .step(0.5)
                .prefix(slot_text("$", ControlSize::Medium))
                .on_input(|text| Message::Edited(Slot::Price, text))
                .on_change(|value| Message::Changed(Slot::Price, value)),
        ]
        .spacing(12.0),
        row![
            row_label("Seats, in twos", false),
            NumberInput::new("seats")
                .value(app.seats.value)
                .text(app.seats.text.as_str())
                .minimum(0.0)
                .maximum(20.0)
                .step(2.0)
                .width(150.0)
                .on_input(|text| Message::Edited(Slot::Seats, text))
                .on_change(|value| Message::Changed(Slot::Seats, value)),
        ]
        .spacing(10.0)
        .align_y(Alignment::Center),
        row![
            row_label("At the minimum", false),
            NumberInput::new("at-minimum")
                .value(app.at_minimum.value)
                .text(app.at_minimum.text.as_str())
                .minimum(0.0)
                .maximum(99.0)
                .width(150.0)
                .on_input(|text| Message::Edited(Slot::AtMinimum, text))
                .on_change(|value| Message::Changed(Slot::AtMinimum, value)),
        ]
        .spacing(10.0)
        .align_y(Alignment::Center),
        row![
            row_label("At the maximum", false),
            NumberInput::new("at-maximum")
                .value(app.at_maximum.value)
                .text(app.at_maximum.text.as_str())
                .minimum(0.0)
                .maximum(100.0)
                .step(5.0)
                .suffix(slot_text("%", ControlSize::Medium))
                .width(170.0)
                .on_input(|text| Message::Edited(Slot::AtMaximum, text))
                .on_change(|value| Message::Changed(Slot::AtMaximum, value)),
        ]
        .spacing(10.0)
        .align_y(Alignment::Center),
        NumberInput::new("salary")
            .value(app.salary.value)
            .text(app.salary.text.as_str())
            .step(50.0)
            .prefix(slot_text("$", ControlSize::Medium))
            .suffix(slot_text("/mo", ControlSize::Medium))
            .on_input(|text| Message::Edited(Slot::Salary, text))
            .on_change(|value| Message::Changed(Slot::Salary, value)),
        NumberInput::new("rejected")
            .value(app.rejected.value)
            .text(app.rejected.text.as_str())
            .minimum(0.0)
            .maximum(99.0)
            .invalid(true)
            .on_input(|text| Message::Edited(Slot::Rejected, text))
            .on_change(|value| Message::Changed(Slot::Rejected, value)),
        row![
            NumberInput::new("size-small")
                .value(app.size_small.value)
                .text(app.size_small.text.as_str())
                .size(ControlSize::Small)
                .on_input(|text| Message::Edited(Slot::SizeSmall, text))
                .on_change(|value| Message::Changed(Slot::SizeSmall, value)),
            NumberInput::new("size-medium")
                .value(app.size_medium.value)
                .text(app.size_medium.text.as_str())
                .size(ControlSize::Medium)
                .on_input(|text| Message::Edited(Slot::SizeMedium, text))
                .on_change(|value| Message::Changed(Slot::SizeMedium, value)),
            NumberInput::new("size-large")
                .value(app.size_large.value)
                .text(app.size_large.text.as_str())
                .size(ControlSize::Large)
                .on_input(|text| Message::Edited(Slot::SizeLarge, text))
                .on_change(|value| Message::Changed(Slot::SizeLarge, value)),
        ]
        .spacing(12.0),
        row![
            NumberInput::new("centered")
                .value(app.centered.value)
                .text(app.centered.text.as_str())
                .minimum(0.0)
                .maximum(99.0)
                .prefix(slot_text("$", ControlSize::Medium))
                .centered(true)
                .width(220.0)
                .on_input(|text| Message::Edited(Slot::Centered, text))
                .on_change(|value| Message::Changed(Slot::Centered, value)),
            NumberInput::new("disabled")
                .value(8.0)
                .minimum(0.0)
                .maximum(20.0)
                .disabled(true)
                .width(220.0),
        ]
        .spacing(12.0),
        muted_line(&format!(
            "Quantity {} — a step clamps to 0..=99, and the stepper that would cross a bound is disabled.",
            number_input::shown(app.quantity.value)
        )),
    ]
    .spacing(14.0);

    // section: Keyboard Navigation
    // The table's keys are the application's, because iced's field reports text
    // and submit only: `↑` and `↓` step through the subscription above, `Escape`
    // clears through it, `Enter` arrives as `on_submit`, and `Tab`/`Shift+Tab` are
    // iced's own focus traversal. iced gives the application no focus report, so
    // the subscription steps this one field whatever is focused.
    let keyboard = column![
        NumberInput::new("keyboard")
            .value(app.keyboard.value)
            .text(app.keyboard.text.as_str())
            .minimum(0.0)
            .maximum(10.0)
            .on_input(|text| Message::Edited(Slot::Keyboard, text))
            .on_change(|value| Message::Changed(Slot::Keyboard, value))
            .on_submit(Message::Commit(Slot::Keyboard, app.keyboard.text.clone())),
        muted_line(&format!(
            "↑ / ↓ step it, Escape clears it, Enter clamps what is typed{}. Tab and Shift+Tab are iced's.",
            match app.committed.as_deref() {
                Some(value) => format!(" — last committed {value}"),
                None => String::new(),
            }
        )),
    ]
    .spacing(8.0);

    // section: Best Practices
    // The seven rules are prose, and prose is not a widget. The card builds the
    // rules the control itself enforces — a step, a range, and the two bounds that
    // clamp and disable — and leaves validation, formatting and error copy to the
    // application, which the danger edge and the readout report.
    let practices = column![
        row![
            row_label("Seats, in twos", false),
            NumberInput::new("practice-seats")
                .value(app.practice_seats.value)
                .text(app.practice_seats.text.as_str())
                .minimum(0.0)
                .maximum(20.0)
                .step(2.0)
                .width(120.0)
                .on_input(|text| Message::Edited(Slot::PracticeSeats, text))
                .on_change(|value| Message::Changed(Slot::PracticeSeats, value)),
        ]
        .spacing(10.0)
        .align_y(Alignment::Center),
        row![
            row_label("Amount, two decimals", false),
            NumberInput::new("practice-price")
                .value(app.practice_price.value)
                .text(app.practice_price.text.as_str())
                .minimum(0.0)
                .maximum(1000.0)
                .step(0.5)
                .prefix(slot_text("$", ControlSize::Medium))
                .width(120.0)
                .on_input(|text| Message::Edited(Slot::PracticePrice, text))
                .on_change(|value| Message::Changed(Slot::PracticePrice, value)),
        ]
        .spacing(10.0)
        .align_y(Alignment::Center),
        muted_line("The step and the range are the control's: a step clamps, and a bound disables its stepper."),
        muted_line("Validation, formatting and the message beside a field are the application's — Slint has no property for them either."),
    ]
    .spacing(10.0);

    // section: API Reference
    // The reference is a table, so the card is the table in use: one control built
    // with every method it lists, and the two helpers a programmatic step needs.
    // `step_value` is the arithmetic the steppers perform; `shown` is the text they
    // write back.
    let api = column![
        NumberInput::new("api")
            .value(app.api.value)
            .text(app.api.text.as_str())
            .minimum(0.0)
            .maximum(100.0)
            .step(5.0)
            .placeholder("0 to 100")
            .size(ControlSize::Medium)
            .prefix(slot_text("$", ControlSize::Medium))
            .suffix(slot_text("%", ControlSize::Medium))
            .invalid(false)
            .centered(false)
            .width(Length::Fill)
            .on_input(|text| Message::Edited(Slot::Api, text))
            .on_change(|value| Message::Changed(Slot::Api, value))
            .on_submit(Message::Commit(Slot::Api, app.api.text.clone())),
        row![
            Button::new("Decrease")
                .size(ControlSize::Small)
                .on_press(Message::ApiStep(StepAction::Decrement)),
            Button::new("Increase")
                .size(ControlSize::Small)
                .on_press(Message::ApiStep(StepAction::Increment)),
            muted_line(&format!(
                "step_value({}, Decrement, 5.0, 0.0, 100.0) = {:?}",
                number_input::shown(app.api.value),
                number_input::step_value(app.api.value, StepAction::Decrement, 5.0, 0.0, 100.0)
                    .map(number_input::shown),
            )),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    ]
    .spacing(12.0);

    // section: end
    let status = text(match app.committed.as_deref() {
        Some(value) => format!("Last committed: {value}"),
        None => "Step a control with its − / + buttons — the gallery is live.".to_string(),
    })
    .size(theme::FONT_SIZE_SM)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Keyboard Navigation"),
        body_box(keyboard),
        heading(app, 2, "Best Practices"),
        body_box(practices),
        heading(app, 3, "API Reference"),
        body_box(api),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
