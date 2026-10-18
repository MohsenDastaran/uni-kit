//! `marker` — plain text and a highlight band at text-sm, and the conversation
//! rows the Slint example composes from them.
//!
//! Run it with `cargo run --example marker`, or in the browser as
//! `?story=marker` in the gallery the website embeds.
//!
//! The Slint example's six outlined sections are the six here, each under the
//! heading `website/component/marker.md` documents that sample at. Two of the
//! Slint boxes — `With icon` and `Links and buttons` — have no heading of their
//! own on that page, whose single heading for both is `Icons, content, and
//! interactive children`; they share it, and the website's own plugin gives the
//! second its own scroll target by index, the way `accordion`'s repeated
//! `Icons and custom content` does. The remaining Slint box, `Loading styles`, is
//! the page's own heading.
//!
//! **The iced marker is the highlight primitive, not the status row.** Slint's
//! `Marker`/`MarkerHighlight` and this component are the same two shapes — text
//! at text-sm, and a warning-tinted band behind it — and the page's
//! `Marker::loading(...)`, `MarkerIcon` and `MarkerContent` slots have no iced
//! counterpart. So a sample that composes those slots here composes them from
//! the catalog's own `Icon` and `Button`, exactly as the Slint example composes
//! its separator rules from `Rectangle`s. The page's `Border` variant and its
//! `Shimmer configuration` have no iced panel for the same reason: neither can
//! be drawn by this component, and a panel showing something else under those
//! headings would not be the code that runs.
//!
//! Each heading carries the title and the code icon that asks the embedding page
//! to scroll to that sample. Both are the button, as in Slint's `SourceHeading`,
//! and a click posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.

use iced::widget::{Space, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Color, Element, Length, Task, Theme};

use crate::components::button::{Button, Variant as ButtonVariant};
use crate::components::group_box::{self, GroupBox};
use crate::components::icon::Icon;
use crate::components::marker::Marker;
use crate::components::theme::{self, DARK, LIGHT, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the component page documents the samples —
/// which is the Slint example's order, with its two heading-less boxes sharing
/// the page's one heading for them. The index is what the page is asked to
/// scroll to.
const SECTIONS: [&str; 6] = [
    "Plain",
    "Separator",
    "Alignment",
    "Loading styles",
    "Icons, content, and interactive children",
    "Icons, content, and interactive children",
];

/// The tokens the mode on screen is using, for the colours the caller passes in
/// explicitly — an `Icon`'s ink, which has no style closure to read the theme in.
fn tokens(app: &App) -> Tokens {
    if app.dark { DARK } else { LIGHT }
}

pub fn run(theme: Theme) -> iced::Result {
    // The passed theme decides the boot appearance, so the demo's own flag and
    // the window's tokens are the same set from the first frame. The demo owns no
    // theme switch: embedded, the page passes the theme it is shown in, and a
    // change reloads the frame, so both are rebuilt together.
    let dark = theme.extended_palette().is_dark;
    iced::application(move || boot(dark), update, view)
        .title("uni-kit — marker")
        .theme(|app: &App| Some(theme::iced_theme(app.dark)))
        .run()
}

/// Which section heading the pointer is over, and which was pressed, so the code
/// icon can draw the two states the Slint heading animates to, and the last
/// control the demo reported.
#[derive(Debug, Clone, Default)]
struct App {
    dark: bool,
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
    pressed: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Pressed(&'static str),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot(dark: bool) -> App {
    App {
        dark,
        ..App::default()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
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

/// A plain marker: text at text-sm in the foreground colour.
fn marker<'a>(label: impl Into<String>) -> Element<'a, Message> {
    Marker::<Message>::new(label).into()
}

/// The highlighted marker: the same text on the warning band.
fn highlight<'a>(label: impl Into<String>) -> Element<'a, Message> {
    Marker::<Message>::new(label).highlight().into()
}

/// A one-pixel rule, the divider the Slint example draws with a `Rectangle`. It
/// is the `border` token, which the page also uses for a divider between two
/// surfaces.
fn rule<'a>() -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fill)
        .height(1.0)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).border.into()),
            ..container::Style::default()
        })
        .into()
}

/// Muted text, the colour a separator or status label takes.
fn muted_label<'a>(label: impl Into<String>) -> Element<'a, Message> {
    text(label.into())
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
fn panel<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A titled section: the heading row, then the sample it names.
fn section<'a>(
    app: &App,
    index: usize,
    title: &'static str,
    body: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![heading(app, index, title), body.into()]
        .spacing(8)
        .into()
}

/// A marker led by a glyph in the given ink, the shape the Slint example's
/// `With icon` box and the page's `MarkerIcon` slot both draw.
fn status_row<'a>(
    xml: &'static [u8],
    ink: Color,
    label: impl Into<String>,
) -> Element<'a, Message> {
    row![
        Icon::<Message>::new(svg::Handle::from_memory(xml))
            .size(14.0)
            .color(ink),
        marker(label),
    ]
    .spacing(6.0)
    .align_y(Alignment::Center)
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // The status inks, named at view-build time because an `Icon` carries its
    // colour rather than resolving it from a style closure.
    let ink = tokens(app);

    // section: Plain
    // The Slint example's first box: the default text run, and the highlight
    // band inline in the middle of it. The band has its own two-pixel inset, so
    // the runs butt together at zero spacing.
    let plain = panel(
        column![
            row![
                marker("Drag a file here, or "),
                highlight("browse"),
                marker(" to upload."),
            ]
            .spacing(0.0)
            .align_y(Alignment::Center),
            row![
                marker("Messages are "),
                highlight("end-to-end encrypted"),
                marker("."),
            ]
            .spacing(0.0)
            .align_y(Alignment::Center),
            marker("Conversation archived"),
        ]
        .spacing(8.0)
        .align_x(Alignment::Start),
    );

    // section: Separator
    // Slint draws its separator from two `Rectangle`s rather than from the
    // marker, and this is the same construction: the stacked form the page's
    // `MarkerVariant::Separator` names, and the inline form beside it.
    let separator = panel(
        column![
            column![
                rule(),
                container(muted_label("Today"))
                    .width(Length::Fill)
                    .center_x(Length::Fill),
                rule(),
            ]
            .spacing(6.0)
            .width(Length::Fill),
            row![
                rule(),
                muted_label("The answer stopped before it finished."),
                rule(),
            ]
            .spacing(12.0)
            .align_y(Alignment::Center)
            .width(Length::Fill),
        ]
        .spacing(12.0),
    );

    // section: Alignment
    // A marker spans the row; the alignment decides where its children sit in
    // it. Start, centre and end, which is the page's own three values.
    let alignment = panel(
        column![
            container(marker("Delivered")).width(Length::Fill),
            container(marker("The answer stopped before it finished."))
                .width(Length::Fill)
                .center_x(Length::Fill),
            container(marker("Read at 14:02"))
                .width(Length::Fill)
                .align_x(Alignment::End),
        ]
        .spacing(8.0),
    );

    // section: Loading styles
    // iced's marker has no `loading` flag, so the loading row composes the
    // catalog's own spinning `Icon` ahead of the label, which is what the Slint
    // example does with `Icon { spinning: true }`.
    let loading = panel(
        column![
            row![
                Icon::<Message>::new(svg::Handle::from_memory(LOADER))
                    .size(12.0)
                    .color(ink.muted_foreground)
                    .spinning(true),
                marker("Loading messages…"),
            ]
            .spacing(8.0)
            .align_y(Alignment::Center),
            row![
                Icon::<Message>::new(svg::Handle::from_memory(LOADER))
                    .size(12.0)
                    .color(ink.muted_foreground)
                    .spinning(true),
                marker("Thinking…"),
            ]
            .spacing(8.0)
            .align_y(Alignment::Center),
        ]
        .spacing(8.0)
        .align_x(Alignment::Start),
    );

    // section: Icons, content, and interactive children
    // The Slint example's `With icon` box: a marker led by a status glyph. The
    // page's `MarkerIcon` slot is what this stands in for; its first sample is
    // the info tone and a plain marker.
    let with_icon = panel(
        column![
            status_row(
                INFO,
                ink.muted_foreground,
                "Messages are end-to-end encrypted"
            ),
            status_row(CIRCLE_CHECK, ink.success, "Delivered"),
            status_row(
                TRIANGLE_ALERT,
                ink.warning,
                "The answer stopped before it finished."
            ),
        ]
        .spacing(8.0)
        .align_x(Alignment::Start),
    );

    // section: Icons, content, and interactive children
    // The Slint example's `Links and buttons` box, and the page's interactive
    // child: a marker beside the control, with the action left to the caller.
    let links = panel(
        row![
            marker("Stopped before it finished."),
            Button::new("Retry")
                .variant(ButtonVariant::Link)
                .on_press(Message::Pressed("Retry")),
        ]
        .spacing(8.0)
        .align_y(Alignment::Center),
    );

    // section: end
    // The one live control the demo still owns: the link button reports its own
    // press, and the line under the sections records it.
    let status_line = text(match app.pressed {
        Some(label) => format!("Last pressed: {label}"),
        None => "Select Retry in the last section — the gallery is live.".to_string(),
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        section(app, 0, SECTIONS[0], plain),
        section(app, 1, SECTIONS[1], separator),
        section(app, 2, SECTIONS[2], alignment),
        section(app, 3, SECTIONS[3], loading),
        section(app, 4, SECTIONS[4], with_icon),
        section(app, 5, SECTIONS[5], links),
        status_line,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The page itself is a control, so a press outside a panel lands on a widget
    // rather than on the bare window.
    container(scrollable(gallery).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            ..container::Style::default()
        })
        .into()
}

// The Lucide `</>` mark the code icon uses, from the icon set this repository
// already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three
// galleries draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;

/// A loader ring, the Lucide `loader-circle` glyph the Slint loading section
/// spins.
const LOADER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"#;

/// The Lucide `info` glyph the first icon row leads with.
const INFO: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#;

/// The Lucide `circle-check` glyph the delivered row leads with.
const CIRCLE_CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m16 9-5.5 5.5L8 12"/></svg>"#;

/// The Lucide `triangle-alert` glyph the stopped-answer row leads with.
const TRIANGLE_ALERT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>"#;
