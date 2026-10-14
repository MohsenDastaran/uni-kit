//! `clipboard` — a copy control, shown the way the Slint example shows it: an
//! input with the clipboard as its suffix, so the control copies the field's
//! value.
//!
//! Run it with `cargo run --example clipboard`, or in the browser as
//! `?story=clipboard` in the gallery the website embeds.
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
//! The one heading is the Slint example's single `GroupBox`, "Copy", which the
//! component page already has. **The two-second check is the caller's here.**
//! Slint owns `copied` and clears it from a `Timer`; iced's timer needs a runtime
//! feature this crate does not enable, so this gallery sets the check when the
//! press reports the value, writes the value through iced's own
//! `iced::clipboard::write`, and drops the check when the field changes — the
//! moment the check stops describing what the field holds.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::clipboard::Clipboard;
use crate::components::group_box::{self, GroupBox};
use crate::components::input::Input;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — clipboard")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// The field's value. The clipboard is built with it, so a press reports what
    /// the field holds now rather than what it held on the first frame.
    url: String,
    /// Whether the caller has said the value was copied; the component draws the
    /// check from it.
    copied: bool,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered_section: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// The field reported an edit.
    UrlChanged(String),
    /// The clipboard was pressed, carrying the value it holds.
    Copy(String),
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
        url: "https://github.com".to_string(),
        copied: false,
        hovered_section: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::UrlChanged(url) => {
            app.url = url;
            // The check belongs to the value that was copied, so an edit retires
            // it; nothing else here can clear it, because this crate's iced timer
            // is off (see the module comment).
            app.copied = false;
        }
        Message::Copy(value) => {
            app.copied = true;
            // The element cannot write the system clipboard. iced's clipboard API
            // is a `Task`, and returning it from `update` is the application
            // performing the write the press asked for.
            return iced::clipboard::write(value);
        }
        Message::HoverSource(title) => app.hovered_section = title,
        Message::PressSource(title) => app.pressed_source = title,
        Message::ShowSource { index, title } => {
            app.pressed_source = None;
            // The page scrolls to the sample; natively this does nothing.
            crate::bridge::show(index, title);
        }
    }
    Task::none()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered_section == Some(title);
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
/// example wraps its section in, untitled because the heading above it carries the
/// title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Copy
    // The clipboard is the field's suffix and copies the field's value: the field
    // is the caller's `String`, and a press reports the URL it holds now. The
    // check is the caller's flag, drawn by the component; the write is iced's own
    // `Task`, returned from `update`.
    let copy = Input::new("clipboard-url")
        .value(&app.url)
        .width(Length::Fixed(280.0))
        .on_input(Message::UrlChanged)
        .suffix(
            Clipboard::new(&app.url)
                .copied(app.copied)
                .on_copy(Message::Copy),
        );
    // section: end
    let status = text(match app.copied {
        true => format!(
            "Copied {} — the write is the application's iced::clipboard::write.",
            app.url
        ),
        false => "Press the copy control: it reports the field's value and the check comes back."
            .to_string(),
    })
    .size(theme::FONT_SIZE_SM)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![heading(app, 0, "Copy"), body_box(copy), status]
        .spacing(PAGE)
        .padding(PAGE)
        .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
