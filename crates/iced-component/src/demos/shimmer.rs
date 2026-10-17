//! `shimmer` — a moving highlight over a muted band, swept by the component's
//! own clock.
//!
//! This is the Slint example's four samples: the basic labels, the timing and
//! spread variants, a one-shot sweep with a replay, and the loading state in
//! context. Nothing here advances a value or drives a subscription — the band
//! samples each frame's own instant and asks for the next frame, so the motion
//! runs from the moment the page shows it. The only controls replay a `once`
//! sweep and switch the in-context sample between loading and finished.
//!
//! iced cannot paint a gradient through part of a text run, so the component
//! draws the band and returns the label for the caller to place over it. Each
//! sample below uses `stack!` for that pairing.
//!
//! Run it with `cargo run --example shimmer`, or in the browser as
//! `?story=shimmer` in the gallery the website embeds.
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
//! The sections are the Slint example's `GroupBox` samples mapped onto the
//! headings `website/component/shimmer.md` already documents the way the Slint
//! page maps them: **Basic usage**, **Duration**, **Direction and play-once**,
//! and **Compose with Marker**. No heading is invented for the site.

use crate::components::button::{Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::shimmer::Shimmer;
use crate::components::theme::{self, Tokens};
use iced::widget::{column, container, mouse_area, row, stack, svg, text};
use iced::{Alignment, Border, Color, Element, Length, Padding, Task, Theme};
use std::time::Duration;

/// The page padding and the gap between sections, as the Slint example sets
/// them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — shimmer")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// The in-context sample's state, the Slint example's `generating`.
    generating: bool,
    /// Starts at one so the first build is the default sweep.
    replay: u64,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Replay,
    ToggleGenerating,
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
        generating: true,
        replay: 1,
        ..App::default()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Replay => app.replay += 1,
        Message::ToggleGenerating => app.generating = !app.generating,
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

// ------------------------------------------------------------------- pieces

/// A mark from the icon set this repository ships (`crates/slint-component/ui/icons`,
/// Lucide, ISC), so the two galleries draw the same glyphs. The ink is resolved
/// inside the style closure, because the theme is only known when iced asks.
fn mark<'a>(markup: &'static [u8], size: f32, color: fn(&Theme) -> Color) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(size)
        .height(size)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(color(theme)),
        })
        .into()
}

/// The muted ink the loading labels and the section titles use.
fn muted(theme: &Theme) -> Color {
    Tokens::of(theme).muted_foreground
}

/// The foreground ink the heading text uses.
fn ink(theme: &Theme) -> Color {
    Tokens::of(theme).foreground
}

/// A band with its label laid over it, which is how a caller gets the text and
/// the highlight in the same place. The label keeps the foreground ink and the
/// band carries the motion.
fn band_line<'a>(
    label: &'a str,
    shimmer: Shimmer<Message>,
    size: f32,
    weight: iced::font::Weight,
    color: fn(&Theme) -> Color,
) -> Element<'a, Message> {
    stack![
        shimmer,
        container(
            text(label)
                .size(size)
                .font(iced::Font {
                    weight,
                    ..iced::Font::DEFAULT
                })
                .style(move |theme: &Theme| text::Style {
                    color: Some(color(theme)),
                }),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y(Length::Fill),
    ]
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

// --------------------------------------------------------------------- view

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic usage
    // The default sweep: two seconds, left to right, at the band's default
    // spread. The second label is muted and the third is one step larger and
    // medium, which the Slint example pairs with the same motion.
    let basic = column![
        band_line(
            "Thinking…",
            Shimmer::new(),
            theme::FONT_SIZE_SM,
            iced::font::Weight::Normal,
            ink,
        ),
        band_line(
            "Generating a response…",
            Shimmer::new(),
            theme::FONT_SIZE_SM,
            iced::font::Weight::Normal,
            muted,
        ),
        band_line(
            "Loading project data…",
            Shimmer::new(),
            theme::FONT_SIZE_MD,
            iced::font::Weight::Medium,
            ink,
        ),
    ]
    .spacing(24.0);

    // section: Duration
    // The Slint example's timing and spread variants: a three-second reverse
    // sweep in the accent colour, a narrow and a broad relative highlight, and
    // a fixed 48px half-width that stays the same physical size across labels.
    // They all share the page's `### Duration` heading because that is where
    // the page documents the sweep's timing.
    let configure = column![
        band_line(
            "Indexing files…",
            Shimmer::new()
                .duration(Duration::from_secs(3))
                .highlight(|tokens| tokens.primary)
                .reverse(true)
                .spread(0.45),
            theme::FONT_SIZE_SM,
            iced::font::Weight::Normal,
            muted,
        ),
        band_line(
            "Loading a narrow label…",
            Shimmer::new().spread(0.15),
            theme::FONT_SIZE_SM,
            iced::font::Weight::Normal,
            muted,
        ),
        band_line(
            "Loading a broad label…",
            Shimmer::new().spread(0.7),
            theme::FONT_SIZE_SM,
            iced::font::Weight::Normal,
            muted,
        ),
        band_line(
            "Fixed-width highlight…",
            Shimmer::new().spread_width(48.0),
            theme::FONT_SIZE_SM,
            iced::font::Weight::Normal,
            muted,
        ),
    ]
    .spacing(24.0);

    // section: Direction and play-once
    // One reverse sweep, then it stops at the end until the caller's replay
    // counter changes. `restart_on` is the caller's own generation: the widget
    // tree keeps the clock, and a new value starts it again.
    let play_once = column![
        band_line(
            "Finalizing…",
            Shimmer::new()
                .once(true)
                .reverse(true)
                .duration(Duration::from_millis(1500))
                .restart_on(app.replay),
            theme::FONT_SIZE_SM,
            iced::font::Weight::Normal,
            muted,
        ),
        Button::new("Replay")
            .variant(Variant::Default)
            .icon(mark(REFRESH_CW, 14.0, muted))
            .on_press(Message::Replay),
    ]
    .spacing(16.0);

    // section: Compose with Marker
    // The loading state where the reference uses it: an assistant label and an
    // attachment title. The page documents this composition under `## Compose
    // with Marker`, so the demo lands there even though the sample is the
    // `In context` group. Switching to finished replaces the shimmer with the
    // plain text, which is the rule the guide states — do not leave an animated
    // label running after the operation has ended.
    let assistant = if app.generating {
        band_line(
            "The assistant is thinking…",
            Shimmer::new(),
            theme::FONT_SIZE_SM,
            iced::font::Weight::Normal,
            muted,
        )
    } else {
        text("Here is a summary of the three open issues in the release milestone.")
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            })
            .into()
    };

    let file = container(
        row![
            mark(FILE_TEXT, 16.0, muted),
            column![
                if app.generating {
                    band_line(
                        "transcript.pdf",
                        Shimmer::new(),
                        theme::FONT_SIZE_SM,
                        iced::font::Weight::Medium,
                        ink,
                    )
                } else {
                    text("transcript.pdf")
                        .size(theme::FONT_SIZE_SM)
                        .font(iced::Font {
                            weight: iced::font::Weight::Medium,
                            ..iced::Font::DEFAULT
                        })
                        .style(|theme: &Theme| text::Style {
                            color: Some(Tokens::of(theme).foreground),
                        })
                        .into()
                },
                text(if app.generating {
                    "Processing document…"
                } else {
                    "PDF · 2.4 MB"
                })
                .size(theme::FONT_SIZE_XS)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
            ]
            .spacing(2.0),
        ]
        .spacing(12.0)
        .align_y(Alignment::Center),
    )
    .padding(Padding {
        top: 12.0,
        right: 12.0,
        bottom: 12.0,
        left: 12.0,
    })
    .width(Length::Fixed(260.0))
    .style(|theme: &Theme| container::Style {
        border: Border {
            color: Tokens::of(theme).border,
            width: 1.0,
            radius: theme::RADIUS_LG.into(),
        },
        ..container::Style::default()
    });

    let in_context = row![
        container(mark(BOT, 16.0, muted))
            .width(32.0)
            .height(32.0)
            .center_x(32.0)
            .center_y(32.0)
            .style(|theme: &Theme| container::Style {
                background: Some(Tokens::of(theme).muted.into()),
                border: iced::border::rounded(16.0),
                ..container::Style::default()
            }),
        container(assistant)
            .width(Length::Fill)
            .center_y(Length::Shrink),
        column![file, {
            let label = if app.generating {
                "Finish"
            } else {
                "Generate again"
            };
            Button::new(label).on_press(Message::ToggleGenerating)
        }]
        .spacing(12.0),
    ]
    .spacing(12.0)
    .align_y(Alignment::Center);

    // section: end
    let gallery = column![
        heading(app, 0, "Basic usage"),
        body_box(basic),
        heading(app, 1, "Duration"),
        body_box(configure),
        heading(app, 2, "Direction and play-once"),
        body_box(play_once),
        heading(app, 3, "Compose with Marker"),
        body_box(in_context),
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    iced::widget::scrollable(gallery)
        .height(Length::Fill)
        .into()
}

// The marks the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the
// two galleries draw the same glyphs.

const BOT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 8V4H8"/><rect width="16" height="12" x="4" y="8" rx="2"/><path d="M2 14h2"/><path d="M20 14h2"/><path d="M15 13v2"/><path d="M9 13v2"/></svg>"#;

const FILE_TEXT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M10 9H8"/><path d="M16 13H8"/><path d="M16 17H8"/></svg>"#;

const REFRESH_CW: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M8 16H3v5"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
