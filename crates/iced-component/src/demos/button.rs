//! `button` — the whole gallery the Slint version shows: the variants, the
//! outline set, the four sizes, icons and states, button groups, progress,
//! dropdowns, a custom size, a custom colour, icon-only buttons and a live
//! loading state.
//!
//! Run it with `cargo run --example button`, or in the browser as
//! `?story=button` in the gallery the website embeds.
//!
//! Each section carries the heading the Slint example gives it — the title, and
//! the code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use.
//!
//! The `// section: <title>` comments below are not decoration: the website reads
//! them to cut this file into the usage panels a component page shows, so the code
//! on the page is the code that runs here rather than a second copy of it. Keep
//! them, one per section, in the order the sections appear, and keep the closing
//! `// section: end` after the last one — without it the lines that assemble the
//! gallery, the status line among them, would be shown as part of the last
//! section.
//!
//! Two things Slint's `Button` does that this one cannot, both visible below:
//! `suffix-icon` draws a glyph at the trailing edge, and `ButtonGroup` squares the
//! corners where two buttons meet. iced's button carries one icon and draws it
//! before the label, and every button rounds its own corners, so the dropdown
//! chevrons lead and a group is buttons set against one another rather than joined
//! into one control. Both are Slint behaviour this port does not have; neither is
//! faked here.

use iced::time::Instant;
use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Subscription, Task, Theme};
use std::time::Duration;

use crate::components::button::{self, Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::progress::ProgressCircle;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// How long the `Sync` example stays in its loading state.
const SYNC_DURATION: Duration = Duration::from_millis(1400);

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — button")
        .theme(theme)
        .subscription(subscription)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// What was pressed last, so a click is visible. Every button in the gallery
    /// carries a message, because a button without one is how iced draws a
    /// disabled control and these are the enabled ones; the single disabled button
    /// below is the one that has none.
    pressed: Option<&'static str>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
    /// The `Sync` example is working: it refuses input and carries the loading
    /// mark, the way the Slint `Button`'s `loading` does.
    syncing: bool,
    /// When the sync started, so the frame clock knows when to end it.
    sync_started: Option<Instant>,
    /// The loading arc's position, advanced one frame at a time.
    sync_progress: f32,
}

#[derive(Debug, Clone)]
enum Message {
    Pressed(&'static str),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A redraw happened: the only clock this crate can subscribe to.
    Frame(Instant),
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
        Message::Pressed(label) => {
            // The `Sync` control stands in for an operation an application would
            // run: the press starts the wait, and the frame clock below ends it.
            if label == "Sync" {
                app.syncing = true;
                app.sync_started = Some(Instant::now());
                app.sync_progress = 0.0;
            }
            app.pressed = Some(label);
        }
        Message::Frame(now) => {
            if app.syncing {
                if let Some(started) = app.sync_started {
                    if now.duration_since(started) >= SYNC_DURATION {
                        app.syncing = false;
                        app.sync_started = None;
                        app.pressed = Some("Sync done");
                        return Task::none();
                    }
                }
                app.sync_progress = (app.sync_progress + 0.06).fract();
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

/// The frames the sync example is timed on. iced's timer needs a runtime feature
/// this crate does not enable, so the state advances and ends on its own frame
/// clock — the same clock the speech demo runs its script on. It is subscribed
/// only while the wait runs, so an idle gallery is not redrawn every frame.
fn subscription(app: &App) -> Subscription<Message> {
    if app.syncing {
        iced::window::frames().map(Message::Frame)
    } else {
        Subscription::none()
    }
}

/// A live control, labelled and ready to report a press.
fn control(label: &'static str) -> Button<'static, Message> {
    Button::new(label).on_press(Message::Pressed(label))
}

/// A control with no label: the icon is the whole control, which is what Slint's
/// `icon`-without-`text` draws.
fn icon_control(
    name: &'static str,
    markup: &'static [u8],
    variant: Variant,
    outline: bool,
) -> Button<'static, Message> {
    let control = Button::new("")
        .icon(glyph(markup, variant, outline))
        .variant(variant)
        .on_press(Message::Pressed(name));
    if outline {
        control.outline()
    } else {
        control
    }
}

/// A glyph from the set the Slint example names — `icon: "search"` there, the same
/// Lucide mark here. iced ships no icon set, so the artwork is the caller's, and
/// it is tinted like the label beside it: the ink of the button's own variant,
/// resolved from the theme when the glyph is drawn rather than captured when the
/// element was built.
fn glyph<'a>(markup: &'static [u8], variant: Variant, outline: bool) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(16.0)
        .height(16.0)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(button::ink(Tokens::of(theme), variant, outline)),
        })
        .into()
}

/// The loading example's control: a `Sync` button that carries a `refresh-cw`
/// glyph at rest and the progress arc while it works. The arc is a `ProgressCircle`
/// rather than a `Spinner` because the circle draws quads whereas the spinner is a
/// `Canvas`, and a `Canvas` inside a control this small is the position the
/// catalogue has twice watched draw nothing.
fn sync_control<'a>(app: &App) -> Button<'a, Message> {
    let loading = app.syncing;
    let icon: Element<'a, Message> = if loading {
        ProgressCircle::new(app.sync_progress).diameter(16.0).into()
    } else {
        svg(svg::Handle::from_memory(REFRESH))
            .width(16.0)
            .height(16.0)
            .style(|theme: &Theme, _status| svg::Style {
                color: Some(Tokens::of(theme).primary_foreground),
            })
            .into()
    };
    Button::new(if loading { "Syncing" } else { "Sync" })
        .variant(Variant::Primary)
        .icon(icon)
        .loading(loading)
        .on_press(Message::Pressed("Sync"))
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
    // section: Variants
    let variants = column![
        row![
            control("Default"),
            control("Primary").variant(Variant::Primary),
            control("Secondary").variant(Variant::Secondary),
            control("Danger").variant(Variant::Danger),
            control("Warning").variant(Variant::Warning),
            control("Success").variant(Variant::Success),
            control("Info").variant(Variant::Info),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
        row![
            control("Ghost").variant(Variant::Ghost),
            control("Link").variant(Variant::Link),
            control("Text").variant(Variant::Text),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    ]
    .spacing(8.0);

    // section: Outline
    let outline = row![
        control("Primary").variant(Variant::Primary).outline(),
        control("Secondary").variant(Variant::Secondary).outline(),
        control("Danger").variant(Variant::Danger).outline(),
        control("Warning").variant(Variant::Warning).outline(),
        control("Success").variant(Variant::Success).outline(),
        control("Info").variant(Variant::Info).outline(),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    // section: Sizes
    // 20, 24, 32 and 40 pixels tall, centred on one another so the row reads as a
    // ladder rather than as four controls on a baseline.
    let sizes = row![
        control("Extra small").size(ControlSize::XSmall),
        control("Small").size(ControlSize::Small),
        control("Medium").size(ControlSize::Medium),
        control("Large").size(ControlSize::Large),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    // section: Icons and states
    let icons_and_states = row![
        control("Search").icon(glyph(SEARCH, Variant::Default, false)),
        // Slint puts this one on the trailing edge; see the module note.
        control("Options").icon(glyph(CHEVRON_DOWN, Variant::Default, false)),
        icon_control("Settings", SETTINGS, Variant::Ghost, false),
        icon_control("Add", PLUS, Variant::Primary, false),
        control("Saving").loading(true),
        control("Selected").selected(true),
        // No message, so iced draws it disabled. This is the only control in the
        // gallery that is inert on purpose.
        Button::new("Disabled"),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    // section: Button group
    // Touching, not joined: each button keeps its own radius, where Slint's
    // `group-start`/`group-end` square the shared corners.
    let groups = row![
        row![
            control("One").outline(),
            control("Two").outline(),
            control("Three").outline(),
        ]
        .spacing(0.0)
        .align_y(Alignment::Center),
        column![
            control("One").outline(),
            control("Two").outline(),
            control("Three").outline(),
        ]
        .spacing(0.0),
    ]
    .spacing(16.0)
    .align_y(Alignment::Center);

    // section: Progress
    let progress = row![
        control("Installing").variant(Variant::Primary).loading(true),
        control("Installing").loading(true),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    // section: Dropdown
    let dropdown = row![
        control("Primary")
            .variant(Variant::Primary)
            .icon(glyph(CHEVRON_DOWN, Variant::Primary, false)),
        control("Default").icon(glyph(CHEVRON_DOWN, Variant::Default, false)),
        control("Ghost")
            .variant(Variant::Ghost)
            .icon(glyph(CHEVRON_DOWN, Variant::Ghost, false)),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    // section: Custom size
    let custom_size = row![icon_control("Favourite", HEART, Variant::Ghost, false)
        .size(ControlSize::XSmall)]
    .align_y(Alignment::Center);

    // section: Custom color
    let custom_color = row![
        control("Custom Button").variant(Variant::Success),
        control("Outline Button").variant(Variant::Success).outline(),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    // section: Icon only
    let icon_only = row![
        icon_control("Search", SEARCH, Variant::Primary, false),
        icon_control("Info", INFO, Variant::Default, false),
        icon_control("Close", X, Variant::Danger, false),
        icon_control("Search outlined", SEARCH, Variant::Default, true),
        icon_control("Back", ARROW_LEFT, Variant::Ghost, false),
    ]
    .spacing(8.0)
    .align_y(Alignment::Center);

    // section: Loading State with Icons
    // A button that actually waits: the press starts the sync and the frame clock
    // ends it, so the control is seen idle, working and done rather than only
    // working. iced's button draws its loading form as the disabled treatment and
    // does not swap the icon itself, so the demo supplies the mark the reference
    // swaps in — a progress arc in the icon slot, advanced by the same clock.
    let loading_state = row![
        sync_control(app),
        text(if app.syncing {
            "Syncing…"
        } else if app.pressed == Some("Sync done") {
            "Synced"
        } else {
            "Idle"
        })
        .size(14.0),
    ]
    .spacing(12.0)
    .align_y(Alignment::Center);
    // section: end
    let status = text(match app.pressed {
        Some(label) => format!("Last pressed: {label}"),
        None => "Press any control but the disabled one — the gallery is live.".to_string(),
    });

    let gallery = column![
        heading(app, 0, "Variants"),
        body_box(variants),
        heading(app, 1, "Outline"),
        body_box(outline),
        heading(app, 2, "Sizes"),
        body_box(sizes),
        heading(app, 3, "Icons and states"),
        body_box(icons_and_states),
        heading(app, 4, "Button group"),
        body_box(groups),
        heading(app, 5, "Progress"),
        body_box(progress),
        heading(app, 6, "Dropdown"),
        body_box(dropdown),
        heading(app, 7, "Custom size"),
        body_box(custom_size),
        heading(app, 8, "Custom color"),
        body_box(custom_color),
        heading(app, 9, "Icon only"),
        body_box(icon_only),
        heading(app, 10, "Loading State with Icons"),
        body_box(loading_state),
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

const SETTINGS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"/><circle cx="12" cy="12" r="3"/></svg>"#;

const PLUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>"#;

const X: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>"#;

const INFO: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#;

const CHEVRON_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#;

const ARROW_LEFT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>"#;

const HEART: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5"/></svg>"#;

/// The set's `refresh-cw`, the mark the loading example rests on.
const REFRESH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M8 16H3v5"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
