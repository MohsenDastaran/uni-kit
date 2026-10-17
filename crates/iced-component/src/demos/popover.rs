//! `popover` — the whole gallery the Slint example shows: the plain case, the
//! click dismissal, all four placements, arbitrary content, an action row, a
//! dismiss from inside, a styled surface, an external control driving the same
//! open state, and a panel that starts open.
//!
//! Run it with `cargo run --example popover`, or in the browser as
//! `?story=popover` in the gallery the website embeds.
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
//! The gallery owns the state, because the popover does not: every section has
//! its own flag below, and each popover renders from that flag and reports back
//! through `on_open_change`. That is the contract a `select` or a `combobox`
//! composes against, and it is the reason a section can be closed by a control
//! outside the panel as easily as by a click inside it.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::{self, Button};
use crate::components::group_box::{self, GroupBox};
use crate::components::popover::{PanelStyle, Placement, Popover};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example and the component page
/// have them. The index is what the page is asked to scroll to.
const SECTIONS: [&str; 10] = [
    "Usage",
    "Click",
    "Basic Popover",
    "Popover with Custom Positioning",
    "View in Popover",
    "Add content by `content` method",
    "Dismiss Popover manually",
    "Styling Popover",
    "Control Open State",
    "Default Open",
];

/// The flags a gallery of these sections needs: one per panel, and the
/// custom-positioning card is four panels on its own.
const SLOTS: usize = 12;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — popover")
        .theme(theme)
        .run()
}

struct App {
    /// One flag per panel in the gallery, because iced has no popover state to
    /// own them: the caller holds whether a panel is showing and decides what a
    /// request to change it means.
    open: [bool; SLOTS],
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A popover asked to be opened or closed; the slot is which one.
    Opened { slot: usize, open: bool },
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
        open: [false; SLOTS],
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Opened { slot, open } => {
            if let Some(flag) = app.open.get_mut(slot) {
                *flag = open;
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

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// The trigger every plain section opens: the Slint example's outlined button.
fn trigger<'a>(label: &str) -> Button<'a, Message> {
    Button::new(label.to_string()).outline()
}

/// The panel content the Slint example's `@children` slots hold: a title in the
/// content colour and a muted second line, stacked `spacing` apart.
///
/// A long second line wraps rather than widening the panel — iced's `Text` wraps
/// to the width its container gives it, and the panel is a fixed width — which is
/// what the Slint example asks for with `wrap: word-wrap`.
fn panel_text<'a>(
    title: impl Into<String>,
    detail: impl Into<String>,
    spacing: f32,
) -> Element<'a, Message> {
    column![
        text(title.into())
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
        text(detail.into())
            .size(theme::FONT_SIZE_XS)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
    ]
    .spacing(spacing)
    .width(Length::Fill)
    .into()
}

/// One row of the activity view: a status glyph, a title and a detail.
fn activity_row<'a>(
    title: &'static str,
    detail: &'static str,
    failed: bool,
) -> Element<'a, Message> {
    let mark = svg(svg::Handle::from_memory(if failed {
        CIRCLE_X
    } else {
        CIRCLE_CHECK
    }))
    .width(14.0)
    .height(14.0)
    .style(move |theme: &Theme, _status| {
        let tokens = Tokens::of(theme);
        svg::Style {
            color: Some(if failed {
                tokens.danger
            } else {
                tokens.success
            }),
        }
    });

    row![
        mark,
        column![
            text(title)
                .size(theme::FONT_SIZE_SM)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
            text(detail)
                .size(theme::FONT_SIZE_XS)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        ]
        .spacing(1.0)
    ]
    .spacing(8.0)
    .align_y(Alignment::Center)
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // The panels, in the order the sections below show them. Each one renders
    // from a flag and reports every request to change it back through the one
    // message the application already handles.
    //
    // The popover draws the surface (`filled(true)`) and lays its own 12px
    // inset inside it, which is the Slint panel's `padding: 12px` and GPUI's
    // `popover_style(cx).p_3()`. A panel that wrapped its content in
    // `panel_container` instead put that surface *inside* the inset, so the
    // text met the visible edge — the only place the two surfaces can be told
    // apart, and why nothing here draws its own.
    let usage = Popover::new(trigger("Click me"))
        .open(app.open[0])
        .filled(true)
        .content(panel_text(
            "Hello, this is a popover!",
            "It appears when you click the button.",
            4.0,
        ))
        .panel_width(260.0)
        .on_open_change(|open| Message::Opened { slot: 0, open });

    let click = Popover::new(trigger("Open popover"))
        .open(app.open[1])
        .panel_width(280.0)
        .filled(true)
        .content(panel_text(
            "Click outside to dismiss.",
            "The panel is a popup, so it is not clipped by this card.",
            6.0,
        ))
        .on_open_change(|open| Message::Opened { slot: 1, open });

    // The other half of the choice: bare content, and the panel draws its own
    // surface. A list or a calendar — content that already fills the panel —
    // asks for this rather than wrapping itself in a container.
    let basic = Popover::new(trigger("Click me"))
        .open(app.open[2])
        .panel_width(260.0)
        .filled(true)
        .content(panel_text(
            "Hello, this is a popover!",
            "It appears when you click the button.",
            4.0,
        ))
        .on_open_change(|open| Message::Opened { slot: 2, open });

    let below = Popover::new(trigger("Below"))
        .open(app.open[3])
        .placement(Placement::Bottom)
        .panel_width(220.0)
        .filled(true)
        .content(panel_text("Placement::Bottom", "Under the trigger.", 0.0))
        .on_open_change(|open| Message::Opened { slot: 3, open });

    let right = Popover::new(trigger("Right"))
        .open(app.open[4])
        .placement(Placement::Right)
        .panel_width(220.0)
        .filled(true)
        .content(panel_text("Placement::Right", "Right of the trigger.", 0.0))
        .on_open_change(|open| Message::Opened { slot: 4, open });

    let above = Popover::new(trigger("Above"))
        .open(app.open[5])
        .placement(Placement::Top)
        .panel_width(220.0)
        .filled(true)
        .content(panel_text("Placement::Top", "Above the trigger.", 0.0))
        .on_open_change(|open| Message::Opened { slot: 5, open });

    let left = Popover::new(trigger("Left"))
        .open(app.open[6])
        .placement(Placement::Left)
        .panel_width(220.0)
        .filled(true)
        .content(panel_text("Placement::Left", "Left of the trigger.", 0.0))
        .on_open_change(|open| Message::Opened { slot: 6, open });

    let activity = Popover::new(trigger("Open activity"))
        .open(app.open[7])
        .panel_width(280.0)
        .filled(true)
        .content(
            column![
                muted_line("Recent activity"),
                activity_row("Build finished", "2 minutes ago", false),
                activity_row("New comment", "9 minutes ago", false),
                activity_row("Deploy failed", "1 hour ago", true),
            ]
            .spacing(10.0)
            .width(Length::Fill),
        )
        .on_open_change(|open| Message::Opened { slot: 7, open });

    let complex = Popover::new(trigger("Complex content"))
        .open(app.open[8])
        .panel_width(280.0)
        .filled(true)
        .content(
            column![
                panel_text(
                    "This popover has complex content.",
                    "The buttons inside are ordinary controls.",
                    2.0,
                ),
                row![
                    Button::new("Perform action").outline(),
                    Button::new("Cancel").variant(button::Variant::Ghost),
                ]
                .spacing(8.0),
            ]
            .spacing(10.0)
            .width(Length::Fill),
        )
        .on_open_change(|open| Message::Opened { slot: 8, open });

    let dismissable = Popover::new(trigger("Dismiss popover"))
        .open(app.open[9])
        .panel_width(300.0)
        .filled(true)
        .content(
            column![
                panel_text(
                    "Click the button below to dismiss this popover.",
                    "The button is inside the panel and the state is outside it.",
                    2.0,
                ),
                Button::new("Close popover")
                    .outline()
                    .on_press(Message::Opened {
                        slot: 9,
                        open: false,
                    }),
            ]
            .spacing(10.0)
            .width(Length::Fill),
        )
        .on_open_change(|open| Message::Opened { slot: 9, open });

    // The host's own surface: `primary` for the fill, no edge, `radius-xl`, and
    // the panel draws it here rather than around the content, which is what
    // Slint's `panel-background`, `panel-border: false` and `panel-radius` do.
    let styled = Popover::new(trigger("Custom style"))
        .open(app.open[10])
        .panel_width(300.0)
        .panel(|theme| {
            let tokens = Tokens::of(theme);
            PanelStyle::resolve(tokens)
                .fill(tokens.primary)
                .border(tokens.primary, 0.0)
                .radius(theme::RADIUS_XL)
                .text_color(tokens.primary_foreground)
        })
        .filled(true)
        .content(
            column![
                text("Fully custom styled popover")
                    .size(theme::FONT_SIZE_SM)
                    .font(iced::Font {
                        weight: iced::font::Weight::Semibold,
                        ..iced::Font::DEFAULT
                    })
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).primary_foreground),
                    }),
                text("A custom surface, radius, and no border.")
                    .size(theme::FONT_SIZE_XS)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).primary_foreground),
                    }),
            ]
            .spacing(4.0)
            .width(Length::Fill),
        )
        .on_open_change(|open| Message::Opened { slot: 10, open });

    // The controlled flag is read here, before the two builders that use it, so
    // neither borrows it from inside a message the panel will send back.
    let controlled_open = app.open[11];

    let controlled = Popover::new(trigger("Controlled popover"))
        .open(controlled_open)
        .panel_width(280.0)
        .filled(true)
        .content(panel_text(
            "This popover's open state is controlled programmatically.",
            "The button beside it drives the same flag the panel reports to.",
            0.0,
        ))
        .on_open_change(|open| Message::Opened { slot: 11, open });

    // The panel is open from the first frame, because the flag says so: iced has
    // no separate "default open" state to set, and the caller's initial value is
    // already the default.
    let open_from_the_start = Popover::new(trigger("Default Open"))
        .open(app.open[11])
        .panel_width(280.0)
        .filled(true)
        .content(panel_text(
            "This popover is open by default when first rendered.",
            "iced has no separate default state; this is the caller's own value.",
            0.0,
        ))
        .on_open_change(|open| Message::Opened { slot: 11, open });

    // section: Usage
    // The whole of a popover: a trigger, the content, the caller's flag, and the
    // one message that moves it. A trigger click, an outside click and Escape all
    // report through `on_open_change`.
    let usage_section = column![
        usage,
        muted_line(
            "`open` is the caller's, and `on_open_change` is the only thing the popover reports."
        ),
    ]
    .spacing(8.0);

    // section: Click
    // A click outside the panel dismisses it; so does Escape. The press is
    // consumed by the overlay, so it does not also reach whatever is under it.
    let click_section = column![
        click,
        muted_line("Click outside the panel, or press Escape, to dismiss it."),
    ]
    .spacing(8.0);

    // section: Basic Popover
    let basic_section = body_box(basic);

    // section: Popover with Custom Positioning
    // Every side is named, and each panel resolves its own: the preferred side
    // when the window has room, the opposite side when it does not, and a clamp
    // into the window when neither fits. The arrow GPUI's popover draws is not
    // part of the Slint component, so there is none to place.
    let positioning = row![below, right, above, left]
        .spacing(24.0)
        .align_y(Alignment::Center);

    // section: View in Popover
    // Any element can be the panel's content, not just text.
    let view_section = body_box(activity);

    // section: Add content by `content` method
    // A panel can hold interactive children. A control inside it sends an
    // ordinary message, exactly as it would anywhere else.
    let complex_section = body_box(complex);

    // section: Dismiss Popover manually
    // Closing from inside needs a handle on the popover, and the host has one:
    // the button sends the same message an outside click would.
    let dismiss_section = body_box(dismissable);

    // section: Styling Popover
    // A host that wants its own look states it through `panel` rather than
    // reaching into the surface, which it cannot see. This is Slint's
    // `panel-background: Theme.primary`, `panel-radius: Theme.radius-xl` and
    // `panel-border: false`.
    let style_section = body_box(styled);

    // section: Control Open State
    // The host owns the flag, so a control outside the panel drives the same
    // state the panel reports to — the way the Slint example wires an external
    // button to `open`.
    let control_section = body_box(
        row![
            Button::new(if controlled_open { "Close" } else { "Open" })
                .outline()
                .on_press(Message::Opened {
                    slot: 11,
                    open: !controlled_open,
                }),
            controlled,
        ]
        .spacing(12.0),
    );

    // section: Default Open
    // `open(true)` from the first frame, with no trigger press at all.
    let default_section = body_box(open_from_the_start);

    // section: end
    // section: end
    let status = text(if app.open.iter().any(|open| *open) {
        "A panel is open: click outside it, press Escape, or use a control inside it."
    } else {
        "Click a trigger to open a panel — the gallery is live."
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        body_box(usage_section),
        heading(app, 1, SECTIONS[1]),
        body_box(click_section),
        heading(app, 2, SECTIONS[2]),
        basic_section,
        heading(app, 3, SECTIONS[3]),
        body_box(positioning),
        heading(app, 4, SECTIONS[4]),
        view_section,
        heading(app, 5, SECTIONS[5]),
        complex_section,
        heading(app, 6, SECTIONS[6]),
        dismiss_section,
        heading(app, 7, SECTIONS[7]),
        style_section,
        heading(app, 8, SECTIONS[8]),
        control_section,
        heading(app, 9, SECTIONS[9]),
        default_section,
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The page itself is a control, so a press outside a panel lands on a widget
    // rather than on the bare window.
    let page = container(scrollable(gallery).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            ..container::Style::default()
        });
    page.into()
}

// The marks the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the
// two galleries draw the same glyphs rather than two lookalikes.

const CIRCLE_CHECK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/></svg>"#;

const CIRCLE_X: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
