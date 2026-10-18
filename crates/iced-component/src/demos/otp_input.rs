//! `otp-input` — a one-time code as a row of cells: the digits-only edit, the
//! cell that takes the caret as it fills, the backspace that clears and steps
//! back, the size ladder, the masked cells, the disabled field, and the
//! completion the caller reads.
//!
//! Run it with `cargo run --example otp_input`, or in the browser as
//! `?story=otp-input` in the gallery the website embeds.
//!
//! Each section carries a heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! panels a component page shows, so the code on the page is the code that runs
//! here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! What the page asks for and iced cannot answer is named here and on the cards
//! rather than approximated:
//!
//! - **Focus and blur are not reported.** iced's field hands out no such event,
//!   so `InputEvent::Focus`/`Blur` and the page's "clear on focus" pattern
//!   cannot be written. Programmatic focus — `otp_input::focus` — is what the
//!   Programmatic Control card drives.
//! - **Arrow keys are not reported either**, so the page's arrow-key navigation
//!   between cells has no iced expression. Tab still walks the cells, because
//!   each cell is a focusable field.
//! - **A resend countdown is application state and a `Subscription`.** The field
//!   never sees it; the Common Patterns card shows the disabled button a caller
//!   would draw while one is running.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::{Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::otp_input::{self, Edit, OtpInput};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — otp-input")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// The verification code: iced has no input state to own it, so this is the
    /// caller's value, set from every edit the field reports.
    code: String,
    /// What the last edit to `code` was, spelled the way the Slint example
    /// spells it.
    last: String,
    /// A code that starts filled, and one whose cells hide their characters.
    default_code: String,
    masked_code: String,
    /// A four-cell PIN, and the three sizes of a filled code.
    pin: String,
    size_large: String,
    size_medium: String,
    size_small: String,
    /// The code the Behavior card drives, and what its last edit was.
    behaviour: String,
    behaviour_last: String,
    /// The code the Common Patterns card submits, and what it submitted.
    pattern: String,
    submitted: Option<String>,
    /// The code the API Reference card drives, and whether it is masked.
    api: String,
    api_masked: bool,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// One variant per field, each carrying the edit the component reported.
    Code(Edit),
    DefaultCode(Edit),
    Masked(Edit),
    Pin(Edit),
    SizeLarge(Edit),
    SizeMedium(Edit),
    SizeSmall(Edit),
    Behaviour(Edit),
    Pattern(Edit),
    Api(Edit),
    ClearPattern,
    ClearApi,
    FillApi,
    ToggleMask,
    FocusApi,
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
        default_code: "123456".to_string(),
        masked_code: "123456".to_string(),
        pin: "1234".to_string(),
        size_large: "123456".to_string(),
        size_medium: "123456".to_string(),
        size_small: "1234".to_string(),
        behaviour_last: "nothing yet".to_string(),
        last: "nothing yet".to_string(),
        ..App::default()
    }
}

/// The caret move an edit asks for, as the Task `update` returns it. The
/// component decides which cell; the caller only has to hand the Task back.
fn caret(id: &str, edit: &Edit) -> Task<Message> {
    match edit.focus() {
        Some(index) => otp_input::focus(id, index),
        None => Task::none(),
    }
}

/// The Slint example's cell readout: which cell moved, and to what.
fn describe(edit: &Edit) -> String {
    if edit.is_refused() {
        format!("cell {} → refused, digits only", edit.index() + 1)
    } else if edit.text().is_empty() {
        format!("cell {} → (cleared)", edit.index() + 1)
    } else {
        format!("cell {} → {}", edit.index() + 1, edit.text())
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Code(edit) => {
            app.last = describe(&edit);
            app.code = edit.value().to_string();
            caret("code", &edit)
        }
        Message::DefaultCode(edit) => {
            app.default_code = edit.value().to_string();
            caret("default", &edit)
        }
        Message::Masked(edit) => {
            app.masked_code = edit.value().to_string();
            caret("masked", &edit)
        }
        Message::Pin(edit) => {
            app.pin = edit.value().to_string();
            caret("pin", &edit)
        }
        Message::SizeLarge(edit) => {
            app.size_large = edit.value().to_string();
            caret("size-large", &edit)
        }
        Message::SizeMedium(edit) => {
            app.size_medium = edit.value().to_string();
            caret("size-medium", &edit)
        }
        Message::SizeSmall(edit) => {
            app.size_small = edit.value().to_string();
            caret("size-small", &edit)
        }
        Message::Behaviour(edit) => {
            app.behaviour_last = describe(&edit);
            app.behaviour = edit.value().to_string();
            caret("behaviour", &edit)
        }
        // Auto-submit on complete: iced's field has no `Complete` event, so the
        // pattern is this read of the code it handed back.
        Message::Pattern(edit) => {
            app.pattern = edit.value().to_string();
            if edit.is_complete() {
                app.submitted = Some(app.pattern.clone());
            }
            caret("pattern", &edit)
        }
        Message::Api(edit) => {
            app.api = edit.value().to_string();
            caret("api", &edit)
        }
        Message::ClearPattern => {
            app.pattern.clear();
            app.submitted = None;
            otp_input::focus("pattern", 0)
        }
        Message::ClearApi => {
            app.api.clear();
            otp_input::focus("api", 0)
        }
        Message::FillApi => {
            app.api = "1234".to_string();
            otp_input::focus("api", 3)
        }
        Message::ToggleMask => {
            app.api_masked = !app.api_masked;
            Task::none()
        }
        Message::FocusApi => otp_input::focus("api", 0),
        Message::HoverSource(title) => {
            app.hovered = title;
            Task::none()
        }
        Message::PressSource(title) => {
            app.pressed_source = title;
            Task::none()
        }
        Message::ShowSource { index, title } => {
            app.pressed_source = None;
            // The page scrolls to the sample; natively this does nothing.
            crate::bridge::show(index, title);
            Task::none()
        }
    }
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
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
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
    // Slint's example is six boxes — a verification code, a filled code, a masked
    // one, the sizes, the disabled field and a four-cell code — and the page puts
    // all of its Usage examples under one heading, so they are one box here.
    let usage = column![
        OtpInput::new("code")
            .value(app.code.as_str())
            .length(6)
            .on_edit(Message::Code),
        muted_line(&format!("Enter the 6-digit code. Last edit: {}", app.last)),
        OtpInput::new("default")
            .value(app.default_code.as_str())
            .on_edit(Message::DefaultCode),
        muted_line("A code that starts filled: the caller's value is the cells."),
        OtpInput::new("masked")
            .value(app.masked_code.as_str())
            .masked(true)
            .on_edit(Message::Masked),
        muted_line("masked(true): every cell hides its character."),
        row![
            OtpInput::new("size-small")
                .value(app.size_small.as_str())
                .length(4)
                .size(ControlSize::Small)
                .on_edit(Message::SizeSmall),
            OtpInput::new("size-medium")
                .value(app.size_medium.as_str())
                .size(ControlSize::Medium)
                .on_edit(Message::SizeMedium),
            OtpInput::new("size-large")
                .value(app.size_large.as_str())
                .size(ControlSize::Large)
                .on_edit(Message::SizeLarge),
        ]
        .spacing(16.0)
        .align_y(Alignment::Center),
        muted_line("The size ladder: 24 / 32 / 40, the Slint theme's control-height."),
        OtpInput::new("disabled").value("123456").disabled(true),
        muted_line("disabled(true) refuses input and draws the muted field."),
        OtpInput::new("pin")
            .value(app.pin.as_str())
            .length(4)
            .on_edit(Message::Pin),
        muted_line("A four-cell PIN: `length` is the cell count."),
    ]
    .spacing(12.0);

    // section: Behavior
    // The component owns the edit, so these are behaviours rather than settings.
    // A character takes the cell it was typed in and hands the caret to the next
    // one; Backspace clears the cell and steps back; a paste spreads over the
    // cells after this one; anything that is not a digit is refused and leaves
    // the code alone; and the code stops at `length`. Type a letter into the
    // field below — nothing lands, and the readout says so.
    let behaviour = column![
        OtpInput::new("behaviour")
            .value(app.behaviour.as_str())
            .length(4)
            .on_edit(Message::Behaviour),
        muted_line(&format!(
            "{} / 4 cells · {} · last edit: {}",
            app.behaviour.chars().count(),
            if app.behaviour.chars().count() == 4 {
                "complete"
            } else {
                "filling"
            },
            app.behaviour_last,
        )),
    ]
    .spacing(8.0);

    // section: Common Patterns
    // Completion is a read rather than an event: iced's field publishes a
    // message per edit, and `Edit::is_complete` is the check that submits the
    // code below. Clearing is the caller's too — the code is its own value — and
    // `otp_input::focus` is how it asks for the caret back. Two of the page's
    // patterns have no iced expression: nothing reports focus, so "clear on
    // focus" cannot be written, and a resend countdown is application state and a
    // `Subscription` the field never sees.
    let patterns = column![
        OtpInput::new("pattern")
            .value(app.pattern.as_str())
            .length(4)
            .on_edit(Message::Pattern),
        muted_line(&match app.submitted.as_ref() {
            Some(code) => format!("Submitted {code} — the caller saw the fourth cell fill."),
            None => "Fill all four cells and `update` submits the code.".to_string(),
        }),
        row![
            Button::new("Clear")
                .variant(Variant::Secondary)
                .outline()
                .size(ControlSize::Small)
                .on_press(Message::ClearPattern),
            // A resend button while the countdown runs: no message, so iced draws
            // it disabled, which is the state the field has nothing to do with.
            Button::new("Resend in 28s").size(ControlSize::Small),
        ]
        .spacing(8.0),
    ]
    .spacing(8.0);

    // section: Programmatic Control
    // The programmatic calls the page documents: `value` is the caller's code,
    // `length` the cells, `masked` the hidden characters, `size` the ladder,
    // `disabled` the muted field and `on_edit` every cell change.
    // `otp_input::focus` is the caret, returned from `update` as a Task. The four
    // buttons are the page's four programmatic calls: set the value, clear it,
    // toggle masking, and focus. The last card is a usage example, not the API
    // listing — `## API Reference` ends the page's example region, so a card named
    // for it has no heading to scroll to.
    let api = column![
        OtpInput::new("api")
            .value(app.api.as_str())
            .length(4)
            .masked(app.api_masked)
            .on_edit(Message::Api),
        row![
            Button::new("Fill 1 2 3 4")
                .size(ControlSize::Small)
                .on_press(Message::FillApi),
            Button::new("Clear")
                .size(ControlSize::Small)
                .on_press(Message::ClearApi),
            Button::new(if app.api_masked { "Show" } else { "Mask" })
                .size(ControlSize::Small)
                .on_press(Message::ToggleMask),
            Button::new("Focus first cell")
                .size(ControlSize::Small)
                .on_press(Message::FocusApi),
        ]
        .spacing(8.0),
        muted_line(&format!(
            "value: {:?} · masked: {} · {} / 4 cells",
            app.api,
            app.api_masked,
            app.api.chars().count(),
        )),
    ]
    .spacing(8.0);

    // section: end
    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Behavior"),
        body_box(behaviour),
        heading(app, 2, "Common Patterns"),
        body_box(patterns),
        heading(app, 3, "Programmatic Control"),
        body_box(api),
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The mark the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the
// two galleries draw the same glyph rather than two lookalikes.

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
