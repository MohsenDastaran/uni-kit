//! `status-bar` — the whole gallery the Slint example shows: the two read-only
//! ends with a centre, the pressable segments that report a click, a separator
//! with a custom centre element, and the two surface overrides.
//!
//! Run it with `cargo run --example status-bar`, or in the browser as
//! `?story=status-bar` in the gallery the website embeds.
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
//! The four headings are the Slint example's GroupBox titles. Each bar sits at
//! the bottom of a fixed-height pane, which is where a status bar lives; the
//! pane draws no surface of its own, so the bar's own fill and edge are what
//! separate it from the page. The icons are the caller's, because iced ships no
//! icon set, and the caller colours them: an SVG with no colour of its own keeps
//! the black in its bytes and vanishes under the dark theme.

use iced::widget::{Space, column, container, mouse_area, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::progress::Progress;
use crate::components::status_bar::{StatusBar, StatusItem};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The width of the centre progress in the third section, Slint's `width: 96px`.
const PROGRESS_WIDTH: f32 = 96.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — status-bar")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// What the last press reported, so a click has a visible result. The Slint
    /// example starts at "nothing yet".
    pressed: String,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered_section: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A segment was pressed. The line is what it stands for.
    Pressed(&'static str),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

/// The section headings, in the order the Slint example and the component page
/// have them. The index is what the page is asked to scroll to.
const SECTIONS: [&str; 4] = [
    "Labels",
    "Buttons",
    "Separators and custom elements",
    "Custom styling",
];

fn boot() -> App {
    App {
        pressed: "nothing yet".to_string(),
        hovered_section: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Pressed(what) => app.pressed = what.to_string(),
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

    iced::widget::row![label, icon]
        .spacing(8.0)
        .align_y(Alignment::Center)
        .into()
}

/// A pane the bar sits at the bottom of, which is where a status bar lives. The
/// pane draws no surface and no edge of its own, so the bar's own fill and its
/// top rule are what separate it from the page above it — one edge, on the bar
/// that owns the boundary.
fn pane<'a>(height: f32, bar: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    column![Space::new().height(Length::Fill), bar.into(),]
        .width(Length::Fill)
        .height(Length::Fixed(height))
        .into()
}

/// A muted line, for the readout under the pressable section.
fn muted<'a>(line: impl Into<String>) -> Element<'a, Message> {
    text(line.into())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Labels
    // A label segment is read-only: muted ink, no hover and no surface. The
    // centre's alignment follows the two ends, so with both present it is
    // centered between them.
    let labels = pane(
        60.0,
        StatusBar::new()
            .left(StatusItem::label("Ready"))
            .child(StatusItem::label("README.md"))
            .right(StatusItem::label("UTF-8")),
    );

    // section: Buttons
    // A button segment is the shared ghost button at the extra-small size, so it
    // takes the hover accent and reports the message its caller gave it.
    let buttons = column![
        pane(
            48.0,
            StatusBar::new()
                .left(
                    StatusItem::button("main")
                        .icon(github())
                        .on_press(Message::Pressed("the branch")),
                )
                .right(
                    StatusItem::button("Ln 1, Col 1")
                        .on_press(Message::Pressed("the cursor position")),
                ),
        ),
        muted(format!("Last click: {}", app.pressed)),
    ]
    .spacing(10.0);

    // section: Separators and custom elements
    // A separator divides runs of items, and the centre holds any element — here
    // the same progress bar the Slint example puts there.
    let mixed = pane(
        60.0,
        StatusBar::new()
            .left(StatusItem::button("main").icon(github()))
            .left(StatusItem::separator())
            .left(StatusItem::label("0 problems").icon(circle_check()))
            .right(StatusItem::label("Ln 12, Col 4"))
            .child(
                container(Progress::new(0.6).size(ControlSize::XSmall))
                    .width(Length::Fixed(PROGRESS_WIDTH)),
            ),
    );

    // section: Custom styling
    // The fill and the edge are the bar's own properties; both resolve the theme
    // in force when the bar is drawn, not when it is built.
    let custom = pane(
        60.0,
        StatusBar::new()
            .background(|theme| Tokens::of(theme).secondary)
            .border_color(|theme| Tokens::of(theme).border)
            .left(StatusItem::label("Ready"))
            .right(StatusItem::label("Rust")),
    );

    // section: end
    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        labels,
        heading(app, 1, SECTIONS[1]),
        buttons,
        heading(app, 2, SECTIONS[2]),
        mixed,
        heading(app, 3, SECTIONS[3]),
        custom,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

/// The branch glyph, from the Slint icon set (`crates/slint-component/ui/icons`,
/// the github mark). The bytes carry no colour of their own, so the style
/// closure is what keeps it legible: an uncoloured SVG keeps its black and
/// disappears on the dark surface.
fn github<'a>() -> Element<'a, Message> {
    svg(svg::Handle::from_memory(GITHUB))
        .width(12.0)
        .height(12.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).foreground),
        })
        .into()
}

/// The problem-count glyph, the same set's `circle-check`, in the muted ink a
/// read-only segment uses. The caller colours it because a label segment cannot
/// recolour an element it did not build.
fn circle_check<'a>() -> Element<'a, Message> {
    svg(svg::Handle::from_memory(CIRCLE_CHECK))
        .width(12.0)
        .height(12.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

const GITHUB: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/></svg>"#;

const CIRCLE_CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m16 9-5.5 5.5L8 12"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
