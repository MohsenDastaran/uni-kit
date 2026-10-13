//! `tooltip` — the whole gallery the Slint example shows: a label over a button,
//! three labels on three buttons in three placements, one carrying a keybinding,
//! a toolbar of icon-only controls, status glyphs, a card with rich content, and
//! a field with help text.
//!
//! Run it with `cargo run --example tooltip`, or in the browser as
//! `?story=tooltip` in the gallery the website embeds.
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
//! The gallery is one tall [`scrollable`](crate::components::scrollable), which
//! is also the point: a label drawn inside the layout that owns its trigger is
//! clipped at the edge of the list it describes, and every one of these is an
//! overlay instead. The first card's label is preferred above its button, so the
//! very first hover is also the flip test — there is no room above a trigger at
//! the top of the page.

use iced::widget::{column, container, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Padding, Task, Theme};

use crate::components::button::Button;
use crate::components::group_box::{self, GroupBox};
use crate::components::input::Input;
use crate::components::tooltip::{Arrow, Side, Tooltip};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the component page has them.
const SECTIONS: [&str; 8] = [
    "Hover the button",
    "Basic Tooltip with Text",
    "Tooltip with Action/Keybinding",
    "Toolbar with Tooltips",
    "Status Indicators with Tooltips",
    "Custom Element Tooltip",
    "Form Validation with Tooltips",
    "Arrow",
];

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — tooltip")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
    /// The field in the last card, so the label there is help over a live
    /// control rather than over a picture of one.
    email: String,
}

#[derive(Debug, Clone)]
enum Message {
    EmailChanged(String),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource { index: usize, title: &'static str },
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::EmailChanged(value) => app.email = value,
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

/// A muted line under an example, carrying what the demo has to say about it.
fn muted_line<'a>(line: impl Into<String>) -> Element<'a, Message> {
    text(line.into())
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

    let label = iced::widget::mouse_area(
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

    let icon = iced::widget::mouse_area(
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

/// The one line of label text every plain card shows.
fn label_text<'a>(line: &'static str) -> Element<'a, Message> {
    text(line).size(theme::FONT_SIZE_XS).into()
}

/// A tooltip and its label as one element, against the label's own message type.
///
/// The label is written before the trigger where a card reads better that way,
/// so this is the same `Tooltip::new(trigger).label(label)` spelled with the
/// label first — a free function is where the message type can be named, which
/// inference inside a `row!` cannot always do.
fn tip<'a>(
    label: impl Into<Element<'a, Message>>,
    trigger: impl Into<Element<'a, Message>>,
) -> Tooltip<'a, Message> {
    Tooltip::new(trigger).label(label)
}

/// A glyph from the set the Slint example names — `icon: "file"` there, the same
/// Lucide mark here — tinted in a colour read from the theme when it is drawn
/// rather than captured when the element is built.
fn glyph<'a>(markup: &'static [u8], size: f32, ink: fn(&Tokens) -> iced::Color) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(size)
        .height(size)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(ink(&Tokens::of(theme))),
        })
        .into()
}

/// An icon-only button carrying a label, which is what the toolbar section is:
/// the tooltip wraps the control rather than being a property of it.
fn icon_button<'a>(markup: &'static [u8], label: &'static str) -> Element<'a, Message> {
    tip(
        label_text(label),
        Button::new("")
            .icon(glyph(markup, 16.0, |tokens| tokens.foreground))
            .outline(),
    )
    .into()
}

/// The rule the toolbar puts between groups of controls.
fn divider<'a>() -> Element<'a, Message> {
    container(iced::widget::Space::new().width(1.0).height(20.0))
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).border.into()),
            ..container::Style::default()
        })
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // The label every plain card shows: one extra-small line in the popover's
    // own ink. It is an element rather than a string because an iced element is
    // built where its state is; the tooltip does not guess at the type.
    let line = |line: &'static str| label_text(line);

    // section: Hover the button
    // The label waits out Slint's `delay` (400ms) and cancels if the pointer
    // leaves first. Its preferred side is the top, which the first card of the
    // page has no room for, so the flip is what this one actually shows.
    let hover: Element<'_, Message> = tip(
        line("Create a new document"),
        Button::new("New file")
            .icon(glyph(FILE, 16.0, |tokens| tokens.foreground))
            .outline(),
    )
    .into();
    let hover_section = column![
        hover,
        muted_line("The pointer rests 400ms; leaving first cancels the label."),
    ]
    .spacing(8.0);

    // section: Basic Tooltip with Text
    // One label per button, and the third asks for the side under its trigger.
    let basic = row![
        tip(line("Bold (⌘B)"), Button::new("Bold").outline()),
        tip(line("Italic (⌘I)"), Button::new("Italic").outline()),
        tip(line("Underline (⌘U)"), Button::new("Underline").outline())
            .placement(Side::Bottom),
    ]
    .spacing(16.0)
    .align_y(Alignment::Center);
    let basic_section = body_box(basic);

    // section: Tooltip with Action/Keybinding
    // GPUI's `Tooltip` can resolve the keybinding of a named action. iced has no
    // action registry a component can ask, so the shortcut is part of the label
    // — which is what the Slint component does with the same card.
    let keybinding: Element<'_, Message> = tip(
        line("Save the workspace — ⌘S"),
        Button::new("Save")
            .icon(glyph(DOWNLOAD, 16.0, |tokens| tokens.foreground))
            .outline(),
    )
    .into();
    let keybinding_section = body_box(keybinding);

    // section: Toolbar with Tooltips
    // A tooltip is a label, not a menu: it never takes the click, so the pointer
    // passes straight through it to the control it describes.
    let toolbar = container(
        row![
            icon_button(ARROW_LEFT, "Undo"),
            icon_button(ARROW_RIGHT, "Redo"),
            divider(),
            icon_button(PLUS, "Zoom in"),
            icon_button(MINUS, "Zoom out"),
        ]
        .spacing(4.0)
        .align_y(Alignment::Center),
    )
    .padding(6.0)
    .width(Length::Fill)
    .style(|theme: &Theme| {
        let tokens = Tokens::of(theme);
        container::Style {
            background: Some(tokens.sidebar.into()),
            border: iced::border::rounded(theme::RADIUS_MD)
                .color(tokens.border)
                .width(1.0),
            ..container::Style::default()
        }
    });
    let toolbar_section = body_box(toolbar);

    // section: Status Indicators with Tooltips
    // Any hoverable element can be the host, including a bare glyph: the trigger
    // is whatever element the tooltip was built around.
    let status = row![
        tip(
            line("Build passed 4 minutes ago"),
            glyph(CIRCLE_CHECK, 18.0, |tokens| tokens.success)
        ),
        tip(
            line("2 warnings in the last build"),
            glyph(TRIANGLE_ALERT, 18.0, |tokens| tokens.warning)
        ),
        tip(line("Upload failed"), glyph(CIRCLE_X, 18.0, |tokens| tokens.danger)),
    ]
    .spacing(20.0)
    .align_y(Alignment::Center);
    let status_section = body_box(status);

    // section: Custom Element Tooltip
    // The label is an element, so it is not limited to one line: this one is a
    // small column with a glyph, two muted lines and the `Kbd`-shaped shortcut
    // the page describes, built from the same tokens as the rest of the card.
    let rich_label = column![
        row![
            glyph(FILE, 14.0, |tokens| tokens.muted_foreground),
            text("document.txt")
                .size(theme::FONT_SIZE_SM)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..iced::Font::DEFAULT
                }),
        ]
        .spacing(theme::SPACING_XS)
        .align_y(Alignment::Center),
        muted_line("Size: 2.4 KB"),
        muted_line("Modified: 2 hours ago"),
        row![
            container(
                text("Enter")
                    .size(theme::FONT_SIZE_XS)
                    .font(theme::MONO)
            )
            .padding(Padding::from([1.0, 6.0]))
            .style(|theme: &Theme| container::Style {
                background: Some(Tokens::of(theme).muted.into()),
                border: iced::border::rounded(theme::RADIUS_SM)
                    .color(Tokens::of(theme).border)
                    .width(1.0),
                ..container::Style::default()
            }),
            muted_line("to open"),
        ]
        .spacing(theme::SPACING_XS)
        .align_y(Alignment::Center),
    ]
    .spacing(4.0);
    let rich = tip(
        rich_label,
        container(text("File: document.txt").size(theme::FONT_SIZE_SM))
            .padding(Padding::from([8.0, 12.0]))
            .style(|theme: &Theme| container::Style {
                border: iced::border::rounded(theme::RADIUS_MD)
                    .color(Tokens::of(theme).border)
                    .width(1.0),
                ..container::Style::default()
            }),
    )
    .arrow(Arrow::Center);
    let rich_section = body_box(rich);

    // section: Form Validation with Tooltips
    // A field with help over it. GPUI's page swaps the label for the validation
    // error when there is one; an iced label is an element, so the same swap is
    // a different element in the same place.
    let invalid = app.email.contains(' ') || app.email.is_empty();
    let field = Input::new("email")
        .placeholder("Email address")
        .value(app.email.clone())
        .invalid(invalid)
        .on_input(Message::EmailChanged);
    let help = tip(
        if invalid {
            line("Enter an address with no spaces.")
        } else {
            line("We'll never share your email address.")
        },
        field,
    );
    let form_section = body_box(
        column![help, muted_line("Type, then hover the field to read the help.")].spacing(8.0),
    );

    // section: Arrow
    // The Slint component has no arrow property and its example never shows
    // one, so the plain cards above draw none. This is the port's own addition:
    // the same surface, a triangle in its own fill between it and the trigger.
    let arrows = row![
        tip(line("Arrow::Start"), Button::new("Start").outline()).arrow(Arrow::Start),
        tip(line("Arrow::Center"), Button::new("Center").outline()).arrow(Arrow::Center),
        tip(line("Arrow::End"), Button::new("End").outline()).arrow(Arrow::End),
    ]
    .spacing(24.0)
    .align_y(Alignment::Center);
    let arrow_section = body_box(arrows);

    // section: end
    let status_line = muted_line(
        "Hover a control for 400ms: an overlay draws the label, so no card or list edge can clip it.",
    );

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        body_box(hover_section),
        heading(app, 1, SECTIONS[1]),
        basic_section,
        heading(app, 2, SECTIONS[2]),
        keybinding_section,
        heading(app, 3, SECTIONS[3]),
        toolbar_section,
        heading(app, 4, SECTIONS[4]),
        status_section,
        heading(app, 5, SECTIONS[5]),
        rich_section,
        heading(app, 6, SECTIONS[6]),
        form_section,
        heading(app, 7, SECTIONS[7]),
        arrow_section,
        status_line,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The page itself is a control, so a press anywhere lands on a widget rather
    // than on the bare window.
    container(scrollable(gallery).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            ..container::Style::default()
        })
        .into()
}

// The marks the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so
// the two galleries draw the same glyphs rather than two lookalikes.

const FILE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/></svg>"#;

const DOWNLOAD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 15V3"/><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/></svg>"#;

const ARROW_LEFT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>"#;

const ARROW_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>"#;

const PLUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>"#;

const MINUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/></svg>"#;

const CIRCLE_CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m16 9-5.5 5.5L8 12"/></svg>"#;

const TRIANGLE_ALERT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>"#;

const CIRCLE_X: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
