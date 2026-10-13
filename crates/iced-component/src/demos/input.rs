//! `input` — the field's whole gallery the way the Slint example shows it: the
//! basic value and placeholder, the prefix, suffix and clear slots, a masked
//! password with a reveal toggle, the four sizes, the disabled and invalid
//! states, validation, Enter, and a borderless field inside a frame of its own.
//!
//! Run it with `cargo run --example input`, or in the browser as `?story=input`
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
//! Four Slint sections cannot mean the same thing here, and the cards say so
//! rather than approximating:
//!
//! - **Read-only** has no iced counterpart. iced's field either accepts edits or
//!   is disabled, so the "Disabled Input" card shows the disabled state and a
//!   comment names what is missing.
//! - **Clean on Escape** is the application's. iced's field handles no keys of
//!   its own outside editing, so the clear control is an ordinary suffix — the
//!   "Prefix, suffix and clear button" card builds one — and Escape belongs to
//!   the application's key subscriptions.
//! - **The reveal toggle** is a caller's suffix: iced masks a value with
//!   `secure`, where Slint's `masked` and `mask-toggle` are one property.
//! - **Atomic inline tokens** have no iced equivalent at all: the value is one
//!   plain `String`, so a reference cannot live inside it as a token that
//!   selection and deletion move as a unit. See the last card.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Border, Element, Length, Task, Theme};

use crate::components::button::{self as button, Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::input::Input;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — input")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// One field per example, because iced has no input state to own them: the
    /// caller holds each value and takes every edit back through `on_input`.
    email: String,
    name: String,
    greeting: String,
    search: String,
    username: String,
    amount: String,
    password: String,
    large: String,
    medium: String,
    small: String,
    xsmall: String,
    state_invalid: String,
    valid_email: String,
    invalid_email: String,
    escape: String,
    query: String,
    borderless: String,
    mention: String,
    /// Whether the password field is showing its value.
    revealed: bool,
    /// The text the last Enter submitted.
    submitted: Option<String>,
    /// The last suffix control that was pressed, so a click is visible.
    notice: Option<&'static str>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Email(String),
    Name(String),
    Greeting(String),
    Search(String),
    Username(String),
    Amount(String),
    Password(String),
    Large(String),
    Medium(String),
    Small(String),
    XSmall(String),
    StateInvalid(String),
    ValidEmail(String),
    InvalidEmail(String),
    Escape(String),
    Query(String),
    Borderless(String),
    Mention(String),
    ClearSearch,
    ClearEscape,
    Reveal,
    Submit(String),
    Notice(&'static str),
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
        greeting: "John Doe".to_string(),
        amount: "1,280.00".to_string(),
        password: "password123".to_string(),
        state_invalid: "not-an-email".to_string(),
        valid_email: "you@example.com".to_string(),
        invalid_email: "not-an-address".to_string(),
        escape: "press escape to clear".to_string(),
        borderless: "Borderless field".to_string(),
        ..App::default()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Email(value) => app.email = value,
        Message::Name(value) => app.name = value,
        Message::Greeting(value) => app.greeting = value,
        Message::Search(value) => app.search = value,
        Message::Username(value) => app.username = value,
        Message::Amount(value) => app.amount = value,
        Message::Password(value) => app.password = value,
        Message::Large(value) => app.large = value,
        Message::Medium(value) => app.medium = value,
        Message::Small(value) => app.small = value,
        Message::XSmall(value) => app.xsmall = value,
        Message::StateInvalid(value) => app.state_invalid = value,
        Message::ValidEmail(value) => app.valid_email = value,
        Message::InvalidEmail(value) => app.invalid_email = value,
        Message::Escape(value) => app.escape = value,
        Message::Query(value) => app.query = value,
        Message::Borderless(value) => app.borderless = value,
        Message::Mention(value) => app.mention = value,
        Message::ClearSearch => app.search.clear(),
        Message::ClearEscape => app.escape.clear(),
        Message::Reveal => app.revealed = !app.revealed,
        Message::Submit(value) => app.submitted = Some(value),
        Message::Notice(label) => app.notice = Some(label),
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

/// A glyph for a field's prefix or suffix slot, in the muted colour both the
/// Slint and GPUI examples draw a slot in. iced ships no icon set, so the artwork
/// is the caller's; this is the same Lucide set the Slint example names.
fn glyph<'a>(markup: &'static [u8]) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(16.0)
        .height(16.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// The same glyph inside a ghost extra-small control, tinted with the ink that
/// control draws its label in rather than the muted slot colour. It is 12px,
/// which is Slint's `icon-size(xsmall)` for the button that carries it.
fn glyph_ink<'a>(markup: &'static [u8]) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(12.0)
        .height(12.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(button::ink(Tokens::of(theme), Variant::Ghost, false)),
        })
        .into()
}

/// A ghost extra-small icon control: 20px square with no fill of its own, the
/// shape Slint's xsmall ghost `Button` draws. Every one carries a message,
/// because a control without one is how iced draws a disabled button.
fn icon_button<'a>(markup: &'static [u8], message: Message) -> Button<'a, Message> {
    Button::new("")
        .variant(Variant::Ghost)
        .size(ControlSize::XSmall)
        .icon(glyph_ink(markup))
        .on_press(message)
}

/// Muted text in a suffix slot, for a unit rather than a glyph.
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

/// A token-shaped chip, to show what an iced field can and cannot hold: this is a
/// prefix element beside the value, not a reference inside it.
fn token_chip<'a>() -> Element<'a, Message> {
    container(text("Alice").size(theme::FONT_SIZE_XS))
        .padding([2.0, 6.0])
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            container::Style {
                background: Some(tokens.accent.into()),
                border: Border {
                    color: tokens.border,
                    width: 1.0,
                    radius: theme::RADIUS_SM.into(),
                },
                text_color: Some(tokens.accent_foreground),
                ..container::Style::default()
            }
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
    // section: Usage
    let usage = column![
        Input::new("email")
            .value(app.email.as_str())
            .placeholder("you@example.com")
            .on_input(Message::Email),
        muted_line(&if app.email.is_empty() {
            "The caller owns the value and takes every edit back through `on_input`.".to_string()
        } else {
            format!("Value: {}", app.email)
        }),
    ]
    .spacing(8.0);

    // section: Basic
    let basic = column![
        Input::new("name")
            .value(app.name.as_str())
            .placeholder("Enter your name...")
            .on_input(Message::Name),
        Input::new("greeting")
            .value(app.greeting.as_str())
            .on_input(Message::Greeting),
        muted_line(&if app.name.is_empty() {
            "Type to see the value.".to_string()
        } else {
            format!("Hello, {}!", app.name)
        }),
    ]
    .spacing(12.0);

    // section: Prefix, suffix and clear button
    let mut search = Input::new("search")
        .value(app.search.as_str())
        .placeholder("Search...")
        .prefix(glyph(SEARCH))
        .on_input(Message::Search);
    // Slint draws the clear control itself behind `cleanable`; here it is the
    // suffix, shown only while there is something to clear.
    if !app.search.is_empty() {
        search = search.suffix(icon_button(X, Message::ClearSearch));
    }

    let prefix_suffix = column![
        row![
            search,
            Input::new("username")
                .value(app.username.as_str())
                .placeholder("Username")
                .prefix(glyph(USER))
                .suffix(icon_button(INFO, Message::Notice("About usernames")))
                .on_input(Message::Username),
        ]
        .spacing(12.0),
        Input::new("amount")
            .value(app.amount.as_str())
            .prefix(slot_text("$", ControlSize::Medium))
            .suffix(slot_text("USD", ControlSize::Medium))
            .align_x(Alignment::End)
            .on_input(Message::Amount),
    ]
    .spacing(12.0);

    // section: Password
    // iced masks with `secure`; a reveal toggle is the caller's suffix, where
    // Slint's `masked` and `mask-toggle` are two properties of one control.
    let password = column![
        Input::new("password")
            .value(app.password.as_str())
            .prefix(glyph(LOCK))
            .secure(!app.revealed)
            .suffix(icon_button(
                if app.revealed { EYE_OFF } else { EYE },
                Message::Reveal,
            ))
            .on_input(Message::Password),
        muted_line(if app.revealed {
            "Revealed: the toggle is a suffix button."
        } else {
            "Masked: `secure(true)`."
        }),
    ]
    .spacing(12.0);

    // section: Sizes
    let sizes = row![
        Input::new("size-large")
            .value(app.large.as_str())
            .placeholder("Large")
            .size(ControlSize::Large)
            .on_input(Message::Large),
        Input::new("size-medium")
            .value(app.medium.as_str())
            .placeholder("Medium")
            .size(ControlSize::Medium)
            .on_input(Message::Medium),
        Input::new("size-small")
            .value(app.small.as_str())
            .placeholder("Small")
            .size(ControlSize::Small)
            .on_input(Message::Small),
        Input::new("size-xsmall")
            .value(app.xsmall.as_str())
            .placeholder("Extra small")
            .size(ControlSize::XSmall)
            .on_input(Message::XSmall),
    ]
    .spacing(12.0);

    // section: States
    // Slint also shows a read-only field here. iced's field either accepts edits
    // or is disabled, so the disabled state is the closest, and it is not
    // selectable the way read-only is.
    let states = row![
        Input::new("state-disabled")
            .value("Disabled")
            .disabled(true),
        Input::new("state-invalid")
            .value(app.state_invalid.as_str())
            .invalid(true)
            .on_input(Message::StateInvalid),
    ]
    .spacing(12.0);

    // section: Disabled Input
    let disabled = column![
        Input::new("disabled-value")
            .value("locked@example.com")
            .disabled(true),
        Input::new("disabled-placeholder")
            .placeholder("Empty and disabled")
            .disabled(true),
    ]
    .spacing(12.0);

    // section: Input Validation
    let validation = column![
        Input::new("valid-email")
            .value(app.valid_email.as_str())
            .prefix(glyph(MAIL))
            .on_input(Message::ValidEmail),
        Input::new("invalid-email")
            .value(app.invalid_email.as_str())
            .prefix(glyph(MAIL))
            .invalid(true)
            .on_input(Message::InvalidEmail),
    ]
    .spacing(12.0);

    // section: Clean on ESC
    // Escape belongs to the application here: iced's field handles no keys of its
    // own outside editing, so the clear control in the suffix is the mechanism the
    // port has.
    let escape = column![
        Input::new("escape")
            .value(app.escape.as_str())
            .placeholder("Clear this with the button")
            .suffix(icon_button(X, Message::ClearEscape))
            .on_input(Message::Escape),
        muted_line("iced's field takes no Escape action; the suffix button clears it."),
    ]
    .spacing(8.0);

    // section: Handle Input Events
    let events = column![
        Input::new("query")
            .value(app.query.as_str())
            .placeholder("Type and press enter")
            .prefix(glyph(SEARCH))
            .on_input(Message::Query)
            .on_submit(Message::Submit(app.query.clone())),
        muted_line(&format!(
            "Last submitted: {}",
            app.submitted.as_deref().unwrap_or("nothing yet")
        )),
    ]
    .spacing(8.0);

    // section: Custom Appearance
    // `appearance(false)` drops the frame so the field can sit inside a container
    // that draws its own, which is how both the Slint and GPUI examples use it.
    let custom = container(
        Input::new("borderless")
            .value(app.borderless.as_str())
            .appearance(false)
            .on_input(Message::Borderless),
    )
    .padding(6.0)
    .width(Length::Fill)
    .style(move |theme: &Theme| {
        let tokens = Tokens::of(theme);
        container::Style {
            background: Some(tokens.secondary.into()),
            border: Border {
                color: tokens.border,
                width: 1.0,
                radius: theme::RADIUS_LG.into(),
            },
            ..container::Style::default()
        }
    });

    // section: Atomic inline tokens
    // iced's field holds one plain `String`: a reference cannot live inside the
    // value as a token that selection and deletion move as a unit. A prefix or
    // suffix element is an adornment beside the value, not part of it, and that is
    // as close as the port comes.
    let tokens = column![
        Input::new("tokens")
            .value(app.mention.as_str())
            .placeholder("Ask @alice")
            .prefix(token_chip())
            .on_input(Message::Mention),
        muted_line("The chip is a prefix element; `@alice` in the value is ordinary text."),
    ]
    .spacing(8.0);

    // section: end
    let status = text(match app.notice {
        Some(label) => format!("Last suffix control: {label}"),
        None => match app.submitted.as_ref() {
            Some(query) => format!("Last submitted: {query}"),
            None => "Type in a field, or press Enter in the events one — the gallery is live."
                .to_string(),
        },
    });

    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Basic"),
        body_box(basic),
        heading(app, 2, "Prefix, suffix and clear button"),
        body_box(prefix_suffix),
        heading(app, 3, "Password"),
        body_box(password),
        heading(app, 4, "Sizes"),
        body_box(sizes),
        heading(app, 5, "States"),
        body_box(states),
        heading(app, 6, "Disabled Input"),
        body_box(disabled),
        heading(app, 7, "Input Validation"),
        body_box(validation),
        heading(app, 8, "Clean on ESC"),
        body_box(escape),
        heading(app, 9, "Handle Input Events"),
        body_box(events),
        heading(app, 10, "Custom Appearance"),
        body_box(custom),
        heading(app, 11, "Atomic inline tokens"),
        body_box(tokens),
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

const SEARCH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21 21-4.34-4.34"/><circle cx="11" cy="11" r="8"/></svg>"#;

const USER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>"#;

const INFO: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#;

const LOCK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>"#;

const EYE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0"/><circle cx="12" cy="12" r="3"/></svg>"#;

const EYE_OFF: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.733 5.076a10.744 10.744 0 0 1 11.205 6.575 1 1 0 0 1 0 .696 10.747 10.747 0 0 1-1.444 2.49"/><path d="M14.084 14.158a3 3 0 0 1-4.242-4.242"/><path d="M17.479 17.499a10.75 10.75 0 0 1-15.417-5.151 1 1 0 0 1 0-.696 10.75 10.75 0 0 1 4.446-5.143"/><path d="m2 2 20 20"/></svg>"#;

const MAIL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m22 7-8.991 5.727a2 2 0 0 1-2.009 0L2 7"/><rect x="2" y="4" width="20" height="16" rx="2"/></svg>"#;

const X: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
