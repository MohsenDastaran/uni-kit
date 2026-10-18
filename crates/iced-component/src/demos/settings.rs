//! `settings` — a settings page: labelled rows with a control at the trailing
//! edge, on the two surfaces a group takes.
//!
//! Run it with `cargo run --example settings`, or in the browser as
//! `?story=settings` in the gallery the website embeds.
//!
//! Each section carries the heading the component page gives it — **Basic
//! Settings** and **Group Variants** — and the code icon that asks the embedding
//! page to scroll to that sample. Clicking either one posts the section's index
//! and title through [`crate::bridge`], the same bridge the Slint and GPUI
//! galleries use.
//!
//! The `// section: <title>` comments below are the contract the website reads to
//! cut this file into the usage panels a component page shows, so the code on the
//! page is the code that runs here. Keep them, one per section, in order, and keep
//! the closing `// section: end` after the last one.
//!
//! The two headings are the two the Slint example fills — its two `GroupBox`
//! examples. The third block in that file is the gallery's finished screen
//! (`GalleryView.example == "last"`), which is the page rather than the
//! component, so this port does not add a section for it. Every row in the Slint
//! example is a switch, which is why every row here is one; the component itself
//! takes any element as a row's field, so a select or an input is the same call.
//!
//! The rows are controlled: a switch reports the value it wants, this application
//! stores it, and the next frame draws what it stored.

use iced::widget::{column, container, mouse_area, row, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::scrollable::Scrollable;
use crate::components::settings::{SettingRow, Settings};
use crate::components::switch::Switch;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — settings")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone)]
struct App {
    /// The basic card's three switches, in the Slint example's own order and
    /// initial state.
    basic: [bool; 3],
    /// The two rows on the filled surface.
    filled: [bool; 2],
    /// The two rows on the outlined surface.
    outlined: [bool; 2],
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            basic: [false, true, false],
            filled: [false, true],
            outlined: [true, false],
            hovered: None,
            pressed_source: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    /// A switch on the basic card reported the value it wants.
    Basic {
        row: usize,
        value: bool,
    },
    /// A switch on the filled surface reported the value it wants.
    Filled {
        row: usize,
        value: bool,
    },
    /// A switch on the outlined surface reported the value it wants.
    Outlined {
        row: usize,
        value: bool,
    },
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
        Message::Basic { row, value } => app.basic[row] = value,
        Message::Filled { row, value } => app.filled[row] = value,
        Message::Outlined { row, value } => app.outlined[row] = value,
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

/// The caption over a variant's surface, `font-size-xs` in the muted colour —
/// the Slint example's "Filled" and "Outlined" labels.
fn caption<'a>(value: &str) -> Element<'a, Message> {
    text(value.to_owned())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box the other galleries draw.
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
    // section: Basic Settings
    // The Slint example's own card: three switches inside an outlined group, each
    // row a title and a description with the control at the trailing edge. Two of
    // the three start off and one starts on, so the switch's two states are both
    // on screen.
    let basic = Settings::new()
        .row(
            SettingRow::new("Dark mode")
                .description("Use the dark palette.")
                .field(
                    Switch::new(app.basic[0]).on_toggle(|value| Message::Basic { row: 0, value }),
                ),
        )
        .row(
            SettingRow::new("Notifications")
                .description("Show desktop alerts.")
                .field(
                    Switch::new(app.basic[1]).on_toggle(|value| Message::Basic { row: 1, value }),
                ),
        )
        .row(
            SettingRow::new("Sounds")
                .description("Play a sound for new messages.")
                .field(
                    Switch::new(app.basic[2]).on_toggle(|value| Message::Basic { row: 2, value }),
                ),
        );

    // section: Group Variants
    // The same rows on the two surfaces a group takes. `Fill` is the themed
    // group-box surface; `Outline` is the transparent one with a one-pixel edge.
    // Both are `GroupBox` variants, so neither surface is written by hand.
    let filled = GroupBox::new().variant(group_box::Variant::Fill).body(
        Settings::new()
            .row(
                SettingRow::new("Reduce motion")
                    .description("Skip non-essential animation.")
                    .field(
                        Switch::new(app.filled[0])
                            .on_toggle(|value| Message::Filled { row: 0, value }),
                    ),
            )
            .row(
                SettingRow::new("Compact density")
                    .description("Fit more rows on screen.")
                    .field(
                        Switch::new(app.filled[1])
                            .on_toggle(|value| Message::Filled { row: 1, value }),
                    ),
            ),
    );

    let outlined = GroupBox::new().variant(group_box::Variant::Outline).body(
        Settings::new()
            .row(
                SettingRow::new("Usage data")
                    .description("Send anonymous crash reports.")
                    .field(
                        Switch::new(app.outlined[0])
                            .on_toggle(|value| Message::Outlined { row: 0, value }),
                    ),
            )
            .row(
                SettingRow::new("Personalised ads")
                    .description("Use activity to pick ads.")
                    .field(
                        Switch::new(app.outlined[1])
                            .on_toggle(|value| Message::Outlined { row: 1, value }),
                    ),
            ),
    );

    let group_variants = column![caption("Filled"), filled, caption("Outlined"), outlined,]
        .spacing(theme::SPACING_MD);

    // section: end
    let status =
        text("Click a switch — the row reports the value it wants and the application stores it.")
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            });

    let gallery = column![
        heading(app, 0, "Basic Settings"),
        body_box(basic),
        heading(app, 1, "Group Variants"),
        body_box(group_variants),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The gallery's own scroll area is the component the page documents, so the
    // frame is drawn by the thing under test.
    Scrollable::new(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
