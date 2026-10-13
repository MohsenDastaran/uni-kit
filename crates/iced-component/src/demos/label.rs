//! `label` — a form label with a required mark and a suffix slot, shown the way
//! the Slint example shows it: the required and optional form labels with a
//! hint, a plain label, and a label whose secondary text follows it.
//!
//! Run it with `cargo run --example label`, or in the browser as `?story=label`
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
//! iced's label is the text, the required mark and the suffix slot, so the
//! headings this demo and the page share are the ones it can actually fill:
//! **Form labels**, **Basic Label** and **Label with Secondary Text**. The
//! page's other examples — highlighting, masking, text alignment — have no iced
//! counterpart and are left without a sample; the page keeps them for GPUI
//! rather than showing a stand-in here.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — label")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
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

/// The muted line a label's suffix slot carries: a hint, a unit, or the shape of
/// what the field expects. Slint puts a second `Text` in the slot; this is the
/// same note in the muted colour, at the extra-small size.
fn hint<'a>(value: &'a str) -> Element<'a, Message> {
    text(value.to_string())
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
    // section: Usage
    // The whole component: a line of text, an asterisk when the field is
    // required, and a slot after both for a hint or the control itself.
    let usage = column![
        Label::new("Workspace name"),
        Label::new("Email address").required(true),
        Label::new("Seats").suffix(hint("up to 20")),
        Label::new("Billing email")
            .required(true)
            .suffix(hint("we only use it for invoices")),
    ]
    .spacing(12.0);

    // section: Form labels
    // The required mark and the secondary text are the two things a form needs
    // from a label: `required(true)` draws the asterisk in the danger colour, and
    // `suffix` carries what the field wants beside it.
    let form_labels = column![
        Label::new("Email").required(true),
        Input::new("email").placeholder("you@example.com"),
        Label::new("Phone").suffix(hint("(optional)")),
        Input::new("phone").placeholder("+1 555 0100"),
        Label::new("Release date")
            .required(true)
            .suffix(hint("(YYYY-MM-DD)")),
        Input::new("date").placeholder("2026-01-31"),
    ]
    .spacing(8.0);

    // section: Basic Label
    let basic = Label::new("This is a label");

    // section: Label with Secondary Text
    // A unit or a parenthetical note follows the label in the muted colour,
    // where Slint nests a smaller `Label` in the slot.
    let secondary = column![
        Label::new("Storage").suffix(hint("(GB)")),
        Label::new("Workspace name").suffix(hint("visible to everyone in the organisation")),
    ]
    .spacing(8.0);

    // section: end
    let status = text("Press a heading or its code icon — the page scrolls to that sample.");

    let gallery = column![
        heading(app, 0, "Usage"),
        body_box(usage),
        heading(app, 1, "Form labels"),
        body_box(form_labels),
        heading(app, 2, "Basic Label"),
        body_box(basic),
        heading(app, 3, "Label with Secondary Text"),
        body_box(secondary),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
