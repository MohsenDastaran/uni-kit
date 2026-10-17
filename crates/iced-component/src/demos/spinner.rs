//! `spinner` — a rotating loading arc that turns on the frame's own clock.
//!
//! Nothing here advances a value, drives a subscription or waits for a button:
//! the component samples each frame's instant and asks for the next frame, so
//! every spinner on the page moves as soon as it is mounted. That is the
//! difference from the model this demo used to show, where the caller owned a
//! `progress` and a rendered frame was identical to the one before it until
//! somebody pressed a button.
//!
//! Run it with `cargo run --example spinner`, or in the browser as
//! `?story=spinner` in the gallery the website embeds.
//!
//! The reference's samples are the four outlined `GroupBox`es of the Slint
//! example — Colors, Sizes, In context and Spinner with Custom Icon — and every
//! one is here. The page documents more samples than that, so the sections below
//! are the **twelve headings `website/component/spinner.md` documents a sample
//! under**, at the depth it documents them, in its order. No heading is invented
//! for the site.
//!
//! The `// section: <title>` comments are the contract the website reads to cut
//! this file into the usage panels a component page shows, so the code on the
//! page is the code that runs here. Keep them, one per section, in order, and
//! keep the closing `// section: end` after the last one. Each section carries a
//! heading the page has — the title, and the 28px code icon that asks the
//! embedding page to scroll to that sample; a click posts the section's index and
//! title through [`crate::bridge`].
//!
//! **The one place iced departs from the reference** is the glyph's name. Slint's
//! spinner forwards `icon` to `Icon` and spins whatever artwork that names; iced's
//! spinner draws the `loader-circle` ring itself, because iced has no icon set of
//! its own and the alternative is making an application ship a spinning asset.
//! So the custom-icon samples show the [`Icon`](crate::components::icon::Icon)
//! component's own turn — the same clock, the same 800ms — with the two glyphs
//! the Slint example names, and the module says so where it happens.
//!
//! The live preview under the last marker is not a sample: it is the control a
//! reader can drive — start/pause and a size choice — and it sits past
//! `// section: end` so nothing the page prints above it can move when it
//! changes.

use iced::font::Weight;
use iced::widget::{column, container, mouse_area, row, scrollable, stack, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::{Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::icon::Icon;
use crate::components::spinner::Spinner;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The sizes the live preview offers, in pixels. The first four are the catalog
/// steps; the last two are the `width` the Slint example lets a caller set.
const PREVIEW_SIZES: [f32; 4] = [24.0, 32.0, 48.0, 64.0];

/// The height of the loading card in the reference's "In context" sample.
const CARD_HEIGHT: f32 = 140.0;

pub fn run(theme: Theme) -> iced::Result {
    // The palette in force at boot is the one the view names explicit colours
    // from. Embedded, the gallery has already parked the page's own palette, so
    // this is the page's set; native, it is the built-in light or dark table.
    let tokens = Tokens::of(&theme);
    iced::application(move || boot(tokens), update, view)
        .title("uni-kit — spinner")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone)]
struct App {
    /// The colours the samples name explicitly, such as the status spinners.
    tokens: Tokens,
    /// The live preview's clock: whether the arc is turning.
    running: bool,
    /// The live preview's diameter, in pixels.
    preview: f32,
    /// Whether the "Conditional Loading" sample's flag is on.
    loading: bool,
    /// The last in-context control a reader pressed.
    pressed: Option<&'static str>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// Start or stop the preview's turn.
    ToggleRunning,
    /// Resize the preview.
    PreviewSize(f32),
    /// Flip the flag the "Conditional Loading" sample is drawn from.
    ToggleLoading,
    /// One of the in-context buttons was pressed.
    Pressed(&'static str),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot(tokens: Tokens) -> App {
    App {
        tokens,
        running: true,
        preview: 48.0,
        loading: true,
        pressed: None,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::ToggleRunning => app.running = !app.running,
        Message::PreviewSize(pixels) => app.preview = pixels,
        Message::ToggleLoading => app.loading = !app.loading,
        Message::Pressed(label) => app.pressed = Some(label),
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

/// The muted ink the page's supporting text uses, read when iced draws so a
/// theme switch recolours it with the page.
fn muted(theme: &Theme) -> text::Style {
    text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    }
}

/// One spinner with a label under it, so a size can be read off the screen.
fn sized<'a>(label: &'a str, spinner: Spinner<Message>) -> Element<'a, Message> {
    column![
        container(Element::<Message>::from(spinner))
            .width(Length::Fixed(56.0))
            .height(Length::Fixed(56.0))
            .center_x(Length::Fixed(56.0))
            .center_y(Length::Fixed(56.0)),
        text(label).size(theme::FONT_SIZE_XS).style(muted),
    ]
    .spacing(4.0)
    .align_x(Alignment::Center)
    .into()
}

/// The reference's "In context" card: a titled card of its own with a large
/// primary spinner centred over a translucent veil.
fn loading_panel<'a>(
    tokens: Tokens,
    title: &'static str,
    description: &'static str,
) -> Element<'a, Message> {
    let body = column![
        text(title).size(theme::FONT_SIZE_SM).font(iced::Font {
            weight: Weight::Semibold,
            ..iced::Font::DEFAULT
        }),
        text(description)
            .size(theme::FONT_SIZE_SM)
            .style(move |_| text::Style {
                color: Some(tokens.muted_foreground),
            }),
    ]
    .spacing(8.0);

    let veil = container(
        column![
            Spinner::<Message>::new()
                .size(ControlSize::Large)
                .color(tokens.primary),
            text("Loading…")
                .size(theme::FONT_SIZE_SM)
                .style(move |_| text::Style {
                    color: Some(tokens.muted_foreground),
                }),
        ]
        .spacing(12.0)
        .align_x(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(move |_| container::Style {
        background: Some(theme::alpha(tokens.background, 0.8).into()),
        ..container::Style::default()
    });

    let card = container(body)
        .width(Length::Fill)
        .height(Length::Fixed(CARD_HEIGHT))
        .padding(16.0)
        .style(move |_| container::Style {
            border: iced::border::rounded(theme::RADIUS_MD)
                .color(tokens.border)
                .width(1.0),
            ..container::Style::default()
        });

    stack![card, veil]
        .width(Length::Fill)
        .height(Length::Fixed(CARD_HEIGHT))
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
        container(text(title).size(theme::FONT_SIZE_SM).font(iced::Font {
            weight: Weight::Medium,
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

/// The box under a heading: the outlined `GroupBox` the Slint example wraps each
/// sample in, untitled because the heading above it carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A titled section: the heading the page has, then the sample it names.
fn section<'a>(
    app: &App,
    index: usize,
    title: &'static str,
    body: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![heading(app, index, title), body_box(body)]
        .spacing(8.0)
        .into()
}

// --------------------------------------------------------------------- view

fn view(app: &App) -> Element<'_, Message> {
    let tokens = app.tokens;

    // section: Colors
    // The Slint example's six, in its order: the foreground default, then the
    // muted, info, success, warning and danger tokens.
    let colors = row![
        Spinner::<Message>::new(),
        Spinner::new().color(tokens.muted_foreground),
        Spinner::new().color(tokens.info),
        Spinner::new().color(tokens.success),
        Spinner::new().color(tokens.warning),
        Spinner::new().color(tokens.danger),
    ]
    .spacing(24.0)
    .align_y(Alignment::Center);

    // section: Sizes
    // The four steps the catalog shares — 12, 14, 16 and 24 — and the custom
    // 32px spinner the Slint example sets in the primary colour.
    let sizes = row![
        Spinner::new().size(ControlSize::XSmall),
        Spinner::new().size(ControlSize::Small),
        Spinner::new(),
        Spinner::new().size(ControlSize::Large),
        Spinner::new().diameter(32.0).color(tokens.primary),
    ]
    .spacing(24.0)
    .align_y(Alignment::Center);

    // section: In context
    // The Refresh button carries the spinner in its icon slot, the loading row
    // is the reference's "Loading data…", and the card is its report card with
    // a large primary spinner over a translucent veil.
    let context = column![
        row![
            Button::new("Refresh")
                .variant(Variant::Primary)
                .icon(
                    Spinner::new()
                        .size(ControlSize::Small)
                        .color(tokens.primary_foreground)
                )
                .on_press(Message::Pressed("Refresh")),
            row![
                Spinner::new()
                    .size(ControlSize::Small)
                    .color(tokens.muted_foreground),
                text("Loading data…").size(theme::FONT_SIZE_SM).style(muted),
            ]
            .spacing(8.0)
            .align_y(Alignment::Center),
        ]
        .spacing(16.0)
        .align_y(Alignment::Center),
        loading_panel(
            tokens,
            "Quarterly report",
            "Revenue, active users and retention for the last three months."
        ),
    ]
    .spacing(16.0);

    // section: Spinner with Custom Icon
    // iced's spinner draws the `loader-circle` ring itself and takes no glyph
    // name, so a custom mark is the `Icon` component's own turn — the same
    // clock and the same 800ms — with the glyphs the Slint example spins.
    let custom_icons = row![
        Icon::<Message>::new(svg::Handle::from_memory(LOADER))
            .size(ControlSize::Large)
            .color(tokens.primary)
            .spinning(true),
        Icon::new(svg::Handle::from_memory(REFRESH))
            .size(ControlSize::Large)
            .color(tokens.success)
            .spinning(true),
        Icon::new(svg::Handle::from_memory(REFRESH))
            .size(ControlSize::Large)
            .color(tokens.warning)
            .spinning(true),
    ]
    .spacing(16.0)
    .align_y(Alignment::Center);

    // section: Loading States
    // The page's three: a plain spinner, one in a chosen colour, and a large one
    // in the primary colour.
    let states = row![
        Spinner::new(),
        Spinner::new().color(tokens.info),
        Spinner::new()
            .size(ControlSize::Large)
            .color(tokens.primary),
    ]
    .spacing(24.0)
    .align_y(Alignment::Center);

    // section: Different Loading Icons
    // The loader ring, then the `refresh-cw` mark in two colours — the glyphs
    // the page names, all turning on the same clock.
    let different = row![
        Spinner::new().color(tokens.muted_foreground),
        Icon::new(svg::Handle::from_memory(LOADER))
            .size(ControlSize::Large)
            .color(tokens.info)
            .spinning(true),
        Icon::new(svg::Handle::from_memory(REFRESH))
            .size(ControlSize::Large)
            .color(tokens.success)
            .spinning(true),
    ]
    .spacing(24.0)
    .align_y(Alignment::Center);

    // section: Status Spinners
    // The three states the page draws: resting, processing and succeeded, each
    // still turning.
    let status = row![
        Spinner::new()
            .size(ControlSize::Small)
            .color(tokens.muted_foreground),
        Spinner::new().size(ControlSize::Small).color(tokens.info),
        Spinner::new()
            .size(ControlSize::Small)
            .color(tokens.success),
    ]
    .spacing(24.0)
    .align_y(Alignment::Center);

    // section: Size Variations
    // The same ladder with its numbers under it, which is how the page's table
    // reads: 12 for inline text, 14 in buttons, 16 by default, 24 for a
    // prominent state, 32 when the caller wants its own.
    let variations = row![
        sized(
            "12 px",
            Spinner::new()
                .size(ControlSize::XSmall)
                .color(tokens.muted_foreground)
        ),
        sized(
            "14 px",
            Spinner::new()
                .size(ControlSize::Small)
                .color(tokens.primary)
        ),
        sized("16 px", Spinner::new().color(tokens.primary)),
        sized(
            "24 px",
            Spinner::new().size(ControlSize::Large).color(tokens.info)
        ),
        sized("32 px", Spinner::new().diameter(32.0).color(tokens.success)),
    ]
    .spacing(16.0);

    // section: In UI Components
    // The page's three uses: inside a button, beside a header's text, and
    // centred in a page of its own.
    let in_components = column![
        Button::new("Submit")
            .variant(Variant::Primary)
            .icon(
                Spinner::new()
                    .size(ControlSize::Small)
                    .color(tokens.primary_foreground)
            )
            .on_press(Message::Pressed("Submit")),
        row![
            text("Processing…").size(theme::FONT_SIZE_SM),
            Spinner::new()
                .size(ControlSize::Small)
                .color(tokens.muted_foreground),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
        container(
            Spinner::new()
                .size(ControlSize::Large)
                .color(tokens.primary)
        )
        .width(Length::Fill)
        .height(Length::Fixed(88.0))
        .center_x(Length::Fill)
        .center_y(Length::Fixed(88.0)),
    ]
    .spacing(16.0);

    // section: Conditional Loading
    // The page draws the spinner only while its flag is set; the button under
    // the row is that flag, so the sample can be driven rather than described.
    let conditional: Element<Message> = if app.loading {
        row![
            Spinner::new()
                .size(ControlSize::Small)
                .color(tokens.primary),
            text("Loading data…").size(theme::FONT_SIZE_SM),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center)
        .into()
    } else {
        text("Nothing is loading.")
            .size(theme::FONT_SIZE_SM)
            .style(muted)
            .into()
    };
    let conditional_sample = column![
        container(conditional)
            .width(Length::Fill)
            .center_y(Length::Fixed(24.0)),
        Button::new(if app.loading {
            "Finish loading"
        } else {
            "Start loading"
        })
        .on_press(Message::ToggleLoading),
    ]
    .spacing(12.0);

    // section: Loading with Text
    let with_text = row![
        Spinner::new()
            .size(ControlSize::Small)
            .color(tokens.primary),
        text("Loading data…").size(theme::FONT_SIZE_SM).style(muted),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    // section: Overlay Loading
    // The page's full-area overlay, sized to the gallery: the content dimmed by
    // a translucent card and a large primary spinner over it.
    let overlay = loading_panel(
        tokens,
        "Weekly digest",
        "Twenty-four new items since your last visit.",
    );

    // section: end
    let status_line = text(match app.pressed {
        Some(label) => format!("Last pressed: {label}"),
        None => "The turn runs from the frame's own instant: 800ms per turn, \
                 linear, and no subscription."
            .to_string(),
    })
    .size(theme::FONT_SIZE_SM)
    .style(muted);

    // Not a sample: the live preview sits past the last marker, so the page's
    // twelve panels are untouched by it. Start/pause drives the component's own
    // clock, and the size choice is a diameter the caller sets.
    let size_button = |pixels: f32| {
        let button = Button::new(format!("{pixels:.0} px")).on_press(Message::PreviewSize(pixels));
        if app.preview == pixels {
            button.variant(Variant::Primary)
        } else {
            button.variant(Variant::Ghost)
        }
    };
    let preview_status = if app.running {
        format!(
            "{:.0} px · running: the arc samples the frame's instant, so it asks \
             for a frame every frame.",
            app.preview
        )
    } else {
        format!(
            "{:.0} px · paused: the arc holds the angle on screen and asks for no \
             frames. Start continues from that angle.",
            app.preview
        )
    };
    let preview = GroupBox::new()
        .title("Preview")
        .variant(group_box::Variant::Outline)
        .footer("The preview is live: start or pause the turn, and choose its size.")
        .body(
            column![
                container(
                    Spinner::<Message>::new()
                        .diameter(app.preview)
                        .color(tokens.primary)
                        .running(app.running)
                )
                .width(Length::Fill)
                .height(Length::Fixed(120.0))
                .center_x(Length::Fill)
                .center_y(Length::Fixed(120.0)),
                row![
                    Button::new(if app.running { "Pause" } else { "Start" })
                        .variant(Variant::Primary)
                        .on_press(Message::ToggleRunning),
                    text("Size").size(theme::FONT_SIZE_SM).style(muted),
                    row(PREVIEW_SIZES.map(|pixels| Element::<Message>::from(size_button(pixels))))
                        .spacing(8.0)
                        .align_y(Alignment::Center),
                ]
                .spacing(12.0)
                .align_y(Alignment::Center),
                text(preview_status).size(theme::FONT_SIZE_XS).style(muted),
            ]
            .spacing(16.0),
        );

    let gallery = column![
        section(app, 0, "Colors", colors),
        section(app, 1, "Sizes", sizes),
        section(app, 2, "In context", context),
        section(app, 3, "Spinner with Custom Icon", custom_icons),
        section(app, 4, "Loading States", states),
        section(app, 5, "Different Loading Icons", different),
        section(app, 6, "Status Spinners", status),
        section(app, 7, "Size Variations", variations),
        section(app, 8, "In UI Components", in_components),
        section(app, 9, "Conditional Loading", conditional_sample),
        section(app, 10, "Loading with Text", with_text),
        section(app, 11, "Overlay Loading", overlay),
        preview,
        status_line,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The `</>` mark, taken from the icon set this repository already ships
// (`crates/slint-component/ui/icons`, Lucide, ISC), so the two galleries draw
// the same glyph rather than two lookalikes.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;

/// A loader ring, the Lucide `loader-circle` glyph the Slint spinner names.
const LOADER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"#;

/// A refresh arrow, the Lucide `refresh-cw` glyph the Slint loading samples spin.
const REFRESH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M8 16H3v5"/></svg>"#;
