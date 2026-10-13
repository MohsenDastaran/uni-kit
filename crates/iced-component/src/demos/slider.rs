//! `slider` — the whole gallery the Slint example shows: a volume, a row of
//! basic sliders, a custom step, a range of the caller's own, the two sliders
//! the custom-styling card draws, and a disabled one.
//!
//! Run it with `cargo run --example slider`, or in the browser as
//! `?story=slider` in the gallery the website embeds.
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
//! **The slider handles its own arrow keys; the rest are the application's.**
//! iced reports no focus to a widget, so the control steps when the pointer is
//! over it — the same stand-in iced's own slider uses — and
//! `number_input::step_value` is the bounded step it takes. `Home`, `End`,
//! `Page Up` and `Page Down` are not the control's, so the keyboard section wires
//! them here through a subscription, exactly as the `number_input` demo wires
//! the keys its field does not report. `Tab` and `Shift+Tab` are iced's own.
//!
//! **Custom styling is not a property on either side.** Slint's
//! `Custom Styling` card draws two ordinary sliders because its slider exposes no
//! colour of its own, and this port does the same rather than inventing one: the
//! track, the fill and the thumb come from the theme tokens, and a per-instance
//! colour is not part of the API.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Font, Length, Subscription, Task, Theme, font};

use crate::components::group_box::{self, GroupBox};
use crate::components::number_input::{self, StepAction};
use crate::components::slider::Slider;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — slider")
        .theme(theme)
        .subscription(subscription)
        .run()
}

/// Which slider a message is about. The messages carry the slot rather than the
/// value, so one arm serves every slider on the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Volume,
    BasicTop,
    BasicMiddle,
    BasicBottom,
    Stepped,
    Ranged,
    StyledTop,
    StyledBottom,
    Keyboard,
}

#[derive(Debug)]
struct App {
    volume: f32,
    basic_top: f32,
    basic_middle: f32,
    basic_bottom: f32,
    stepped: f32,
    ranged: f32,
    styled_top: f32,
    styled_bottom: f32,
    /// The slider the keyboard section's `Home`/`End`/`Page Up`/`Page Down`
    /// keys act on: iced reports no focus, so the subscription names one control
    /// rather than querying which is focused.
    keyboard: f32,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A slider moved: already clamped to its range and snapped to its step.
    Changed(Slot, f32),
    /// The key subscription saw `Home` or `End`, which jump to a bound.
    Bound(f32),
    /// The key subscription saw `Page Up` or `Page Down`, the larger step the
    /// website's key table names.
    Page(StepAction),
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
        volume: 42.0,
        basic_top: 25.0,
        basic_middle: 50.0,
        basic_bottom: 75.0,
        stepped: 20.0,
        ranged: 21.0,
        styled_top: 40.0,
        styled_bottom: 65.0,
        keyboard: 50.0,
        hovered: None,
        pressed_source: None,
    }
}

/// The value behind a slot, so one arm moves any slider on the page.
fn value_mut(app: &mut App, slot: Slot) -> &mut f32 {
    match slot {
        Slot::Volume => &mut app.volume,
        Slot::BasicTop => &mut app.basic_top,
        Slot::BasicMiddle => &mut app.basic_middle,
        Slot::BasicBottom => &mut app.basic_bottom,
        Slot::Stepped => &mut app.stepped,
        Slot::Ranged => &mut app.ranged,
        Slot::StyledTop => &mut app.styled_top,
        Slot::StyledBottom => &mut app.styled_bottom,
        Slot::Keyboard => &mut app.keyboard,
    }
}

fn subscription(_app: &App) -> Subscription<Message> {
    // The slider's own keys are the four arrows, which the control reads while
    // the pointer is over it. `Home`, `End` and the two page keys are not the
    // control's, and iced gives the application no focus report, so the
    // subscription acts on the one slider the keyboard section names.
    iced::keyboard::listen().filter_map(|event| match event {
        iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Home),
            ..
        } => Some(Message::Bound(0.0)),
        iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::End),
            ..
        } => Some(Message::Bound(100.0)),
        iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::PageUp),
            ..
        } => Some(Message::Page(StepAction::Increment)),
        iced::keyboard::Event::KeyPressed {
            key: iced::keyboard::Key::Named(iced::keyboard::key::Named::PageDown),
            ..
        } => Some(Message::Page(StepAction::Decrement)),
        _ => None,
    })
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Changed(slot, value) => *value_mut(app, slot) = value,
        Message::Bound(value) => app.keyboard = value,
        Message::Page(action) => {
            // The larger page step is still the bounded step the component uses,
            // so a page key at a bound stops where an arrow would.
            if let Some(value) =
                number_input::step_value(app.keyboard as f64, action, 10.0, 0.0, 100.0)
            {
                app.keyboard = value as f32;
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

/// Muted text under an example, carrying what the demo has to say about it.
fn muted_line<'a>(line: &str) -> Element<'a, Message> {
    text(line.to_string())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// The value beside a slider, in the muted colour, the way the Slint example
/// prints the volume.
fn readout<'a>(value: f32, suffix: &str) -> Element<'a, Message> {
    text(format!("{}{suffix}", value.round() as i64))
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A slider on a slot, reporting its value back to the same slot.
fn slider<'a>(slot: Slot, value: f32) -> Slider<'a, Message> {
    Slider::new(value).on_change(move |value| Message::Changed(slot, value))
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

fn view(app: &App) -> Element<'_, Message> {
    // section: Volume
    // The Slint example's volume: one slider bound to a number, with the number
    // printed beside it. The value is the caller's — the slider reports what a
    // drag asked for and the application writes it back.
    let volume = column![
        row![
            slider(Slot::Volume, app.volume).width(Length::Fill),
            readout(app.volume, "%"),
        ]
        .spacing(12.0)
        .align_y(Alignment::Center),
    ]
    .spacing(8.0);

    // section: Basic Slider
    // The value is a plain number, so a slider backs any bounded quantity. Three
    // sliders, each with the default 0..=100 range and step of one.
    let basic = column![
        slider(Slot::BasicTop, app.basic_top),
        slider(Slot::BasicMiddle, app.basic_middle),
        slider(Slot::BasicBottom, app.basic_bottom),
    ]
    .spacing(10.0);

    // section: Custom Step Intervals
    // A step of ten, so a drag snaps to 0, 10, 20 … and the arrow keys move by
    // the same interval.
    let stepped = column![
        slider(Slot::Stepped, app.stepped).step(10.0),
        muted_line(&format!(
            "Step 10 — the value is {}.",
            number_input::shown(app.stepped as f64)
        )),
    ]
    .spacing(8.0);

    // section: Min/Max Configuration
    // The bounds are the caller's: this one runs from -10 to 40, the temperature
    // range the Slint group names.
    let ranged = column![
        slider(Slot::Ranged, app.ranged)
            .minimum(-10.0)
            .maximum(40.0),
        muted_line(&format!(
            "From -10 to 40 at step 1 — the value is {}.",
            number_input::shown(app.ranged as f64)
        )),
    ]
    .spacing(8.0);

    // section: Custom Styling
    // Two ordinary sliders. Slint's slider exposes no colour of its own and its
    // example draws the same two, so the track, the fill and the thumb are the
    // theme's and this port does not invent a per-instance colour.
    let styled = column![
        slider(Slot::StyledTop, app.styled_top),
        slider(Slot::StyledBottom, app.styled_bottom),
        muted_line("The track, fill and thumb are theme tokens; a slider has no colour property on either side."),
    ]
    .spacing(10.0);

    // section: Disabled State
    // A control with no message cannot move, which is how iced spells disabled;
    // `disabled(true)` also draws the muted track and thumb, which iced cannot
    // fade as one subtree.
    let disabled = column![
        Slider::<Message>::new(30.0).disabled(true),
        muted_line("Disabled: the fill and the thumb's edge take a lower weight, and the pointer and the keys do nothing."),
    ]
    .spacing(8.0);

    // section: Keyboard Shortcuts
    // The four arrows are the control's own, read while the pointer is over it
    // because iced reports no focus to a widget: `←`/`↓` and `→`/`↑` take the
    // bounded step. `Home`, `End`, `Page Up` and `Page Down` are the
    // application's, wired through the subscription above, and `Tab`/`Shift+Tab`
    // are iced's focus traversal.
    let keyboard = column![
        slider(Slot::Keyboard, app.keyboard),
        muted_line(&format!(
            "← ↓ → ↑ step, Home and End jump, Page Up and Page Down move by ten — the value is {}.",
            number_input::shown(app.keyboard as f64)
        )),
        muted_line("iced reports no focus to the application, so the subscription acts on this slider whatever is focused."),
    ]
    .spacing(8.0);

    // section: end
    let status = text("Drag a track or press an arrow — the gallery is live.")
        .size(theme::FONT_SIZE_SM)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        });

    let gallery = column![
        heading(app, 0, "Volume"),
        body_box(volume),
        heading(app, 1, "Basic Slider"),
        body_box(basic),
        heading(app, 2, "Custom Step Intervals"),
        body_box(stepped),
        heading(app, 3, "Min/Max Configuration"),
        body_box(ranged),
        heading(app, 4, "Custom Styling"),
        body_box(styled),
        heading(app, 5, "Disabled State"),
        body_box(disabled),
        heading(app, 6, "Keyboard Shortcuts"),
        body_box(keyboard),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
