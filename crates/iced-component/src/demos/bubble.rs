//! `bubble` — the gallery the Slint example shows: the variants, the two
//! alignments, the rich and long content, the groups, the reactions and
//! interactive content, and a caller-owned surface colour.
//!
//! Run it with `cargo run --example bubble`, or in the browser as
//! `?story=bubble` in the gallery the website embeds.
//!
//! The section titles below are the page's own headings, not the Slint
//! `GroupBox` titles: `website/component/bubble.md` documents each behaviour as
//! a section, and the five Slint boxes that share a page section —
//! `Reactions`, `Links and buttons`, `Tooltip`, `Popover`, and the two rich
//! content boxes — are folded into the two sections that cover them. That is the
//! same mapping `remark-slint-source.js` records in its `SHARED.bubble` table,
//! so the iced panel lands beside the Slint one rather than after the page.
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
//! Two things the Slint example draws have no iced counterpart here and are
//! named rather than faked: the `👍`/`✨` reaction glyphs, which need an emoji
//! face the wasm build does not embed, so the reactions are labelled controls;
//! and the `Tooltip`/`Popover` reactions, which the GPUI page itself defers to
//! the application — the demo shows the same information as plain text inside
//! the pill.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment as Axis, Color, Element, Length, Task, Theme};

use crate::components::bubble::{
    Alignment, Bubble, Content, Group, ReactionSide, Reactions, Variant,
};
use crate::components::button::{Button, Variant as ButtonVariant};
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, ControlSize, Tokens};

/// The gap between the sections, which the Slint example sets to 24px.
const PAGE: f32 = theme::SPACING_XL;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — bubble")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// The collapsible bubble is the example's one piece of state.
    details_open: bool,
    report_open: bool,
    last_reaction: Option<&'static str>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleDetails,
    OpenReport,
    React(&'static str),
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
        Message::ToggleDetails => app.details_open = !app.details_open,
        Message::OpenReport => app.report_open = !app.report_open,
        Message::React(label) => app.last_reaction = Some(label),
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

/// A line of text whose colour is resolved from the active tokens when iced
/// renders it, so a caller-owned child reads correctly in either theme. An
/// arbitrary child of a bubble does not inherit the surface's ink, which is why
/// the example spells this out where the Slint `Text` inherits its parent.
fn line<'a>(
    value: &'a str,
    size: f32,
    tone: fn(&Tokens) -> Color,
) -> iced::widget::Text<'a, Theme, iced::Renderer> {
    text(value)
        .size(size)
        .style(move |theme: &Theme| iced::widget::text::Style {
            color: Some(tone(&Tokens::of(theme))),
        })
}

/// A reaction pill holding one labelled control, the spacing and inset the
/// Slint `BubbleReactions`/`BubbleReactionRow` pair uses. The Slint example
/// fills it with an emoji face the wasm build cannot draw, so the control is a
/// ghost button whose label carries the same information.
fn reaction_pill<'a>(
    alignment: Alignment,
    label: &'static str,
    key: &'static str,
) -> Reactions<'a, Message> {
    Reactions::new().alignment(alignment).child(
        Button::new(label)
            .variant(ButtonVariant::Ghost)
            .size(ControlSize::XSmall)
            .on_press(Message::React(key)),
    )
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
        .spacing(theme::SPACING_SM)
        .align_y(Axis::Center)
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

fn view(app: &App) -> Element<'_, Message> {
    // section: Anatomy and basic usage
    // The shortest form is `text(...)`; a caller that needs its own layout uses
    // `content(...)` and colours the children, because an arbitrary child does
    // not inherit the surface's ink.
    let anatomy = column![
        Bubble::new().text("Can you review this draft?"),
        Bubble::new().variant(Variant::Outline).content(
            Content::new().child(
                row![
                    line(
                        "Can you review this draft?",
                        theme::FONT_SIZE_SM,
                        |tokens| tokens.foreground,
                    ),
                    line("PDF · 2.4 MB", theme::FONT_SIZE_XS, |tokens| {
                        tokens.muted_foreground
                    }),
                ]
                .spacing(theme::SPACING_SM)
                .align_y(Axis::Center),
            ),
        ),
    ]
    .spacing(theme::SPACING_MD);

    // section: Alignment
    // `start` is the received turn and the default; `end` is the sent one.
    let alignment = column![
        Bubble::new()
            .variant(Variant::Secondary)
            .text("Incoming message"),
        Bubble::new()
            .alignment(Alignment::End)
            .text("Outgoing message"),
    ]
    .spacing(theme::SPACING_MD);

    // section: Variants
    let variants = column![
        Bubble::new().text("A strong primary bubble."),
        Bubble::new()
            .variant(Variant::Secondary)
            .text("The neutral secondary bubble."),
        Bubble::new()
            .variant(Variant::Muted)
            .text("A lower-emphasis muted bubble."),
        Bubble::new()
            .variant(Variant::Tinted)
            .text("A softly tinted primary bubble."),
        Bubble::new()
            .variant(Variant::Outline)
            .text("A bordered bubble for rich content."),
        Bubble::new()
            .variant(Variant::Destructive)
            .text("A failed action with its reason in text."),
        Bubble::new()
            .variant(Variant::Ghost)
            .text("Ghost content is unframed and can use the full row width."),
    ]
    .spacing(theme::SPACING_LG);

    // section: Rich content and long messages
    // A bubble has no hidden-text state: the application owns the collapsed
    // flag and the trigger, and renders the same bubble in either state.
    let mut details = column![].spacing(theme::SPACING_SM);
    if app.details_open {
        details = details.push(line(
            "Dialog and sheet controls already keep their focus rings visible.",
            theme::FONT_SIZE_SM,
            |tokens| tokens.foreground,
        ));
        details = details.push(line(
            "Update the menu focus token separately from its pointer hover state.",
            theme::FONT_SIZE_SM,
            |tokens| tokens.foreground,
        ));
    }
    details = details.push(
        Button::new(if app.details_open {
            "Show less"
        } else {
            "Show more"
        })
        .variant(ButtonVariant::Ghost)
        .size(ControlSize::Small)
        .on_press(Message::ToggleDetails),
    );

    let rich = column![
        Bubble::new()
            .variant(Variant::Secondary)
            .text("A longer message wraps naturally within the bubble's available width."),
        Bubble::new()
            .variant(Variant::Muted)
            .text("The accessibility review found two focus states that need more contrast.",)
            .child(details),
        Bubble::new().text("design-notes.pdf\n2.4 MB · PDF"),
    ]
    .spacing(theme::SPACING_MD);

    // section: Groups
    // `Group` infers no sender identity; the application says which consecutive
    // bubbles belong to one sender and which edge the stack sits on.
    let groups = column![
        Group::new()
            .child(
                Bubble::new()
                    .variant(Variant::Secondary)
                    .text("Can you tell me what changed?"),
            )
            .child(
                Bubble::new()
                    .variant(Variant::Secondary)
                    .text("The registry route was stale."),
            ),
        Group::new()
            .align(Alignment::End)
            .child(
                Bubble::new()
                    .alignment(Alignment::End)
                    .text("I removed the stale route."),
            )
            .child(
                Bubble::new()
                    .alignment(Alignment::End)
                    .text("The updated registry is ready to review."),
            ),
    ]
    .spacing(theme::SPACING_XL);

    // section: Reactions and interactive content
    // A reaction is a pill of plain elements below or above the surface; the
    // action inside it is an ordinary `Button` the application owns. A link is
    // content, not a command, so it is drawn in the primary ink.
    let reactions = column![
        Bubble::new()
            .variant(Variant::Outline)
            .text("This bubble has reaction feedback.")
            .reactions(reaction_pill(Alignment::Start, "Like · 2", "Like")),
        Bubble::new()
            .alignment(Alignment::End)
            .text("Reactions can attach to any edge.")
            .reactions(reaction_pill(Alignment::End, "Thanks · 1", "Thanks")),
        Bubble::new()
            .variant(Variant::Outline)
            .text("The implementation guide is available online.")
            .child(line(
                "Open the component documentation",
                theme::FONT_SIZE_SM,
                |tokens| tokens.primary,
            )),
        Bubble::new().variant(Variant::Muted).content(
            Content::new().child(
                row![
                    line(
                        "The generated report is ready.",
                        theme::FONT_SIZE_SM,
                        |tokens| tokens.foreground,
                    ),
                    Button::new(if app.report_open {
                        "Close report"
                    } else {
                        "Open report"
                    })
                    .variant(ButtonVariant::Ghost)
                    .size(ControlSize::Small)
                    .on_press(Message::OpenReport),
                ]
                .spacing(theme::SPACING_MD)
                .align_y(Axis::Center),
            )
        ),
        Bubble::new()
            .variant(Variant::Destructive)
            .text("The build command could not finish.")
            .reactions(
                Reactions::new()
                    .side(ReactionSide::Top)
                    .alignment(Alignment::Start)
                    .child(line("Build failed", theme::FONT_SIZE_XS, |tokens| {
                        tokens.foreground
                    })),
            ),
    ]
    .spacing(theme::SPACING_XL);

    // section: Custom styling and theme tokens
    // Success is the semantic green; the fill is a tint and the label and border
    // use the same role, so the contrast stays paired. Every token is resolved
    // from the active theme when the surface draws.
    let custom = Bubble::new().text("Custom semantic color").content(
        Content::new()
            .fill(|tokens| theme::alpha(tokens.success, 0.15))
            .text_color(|tokens| tokens.success)
            .border_color(|tokens| theme::alpha(tokens.success, 0.35))
            .border_width(1.0)
            .radius(theme::RADIUS_LG)
            .padding_x(16.0)
            .padding_y(12.0),
    );

    // section: end
    let status = text(format!(
        "{} · {} · {}",
        if app.details_open {
            "Details open"
        } else {
            "Details closed"
        },
        if app.report_open {
            "Report open"
        } else {
            "Report closed"
        },
        match app.last_reaction {
            Some(label) => format!("Last reaction: {label}"),
            None => "No reaction yet".to_string(),
        },
    ));

    let gallery = column![
        heading(app, 0, "Anatomy and basic usage"),
        body_box(anatomy),
        heading(app, 1, "Alignment"),
        body_box(alignment),
        heading(app, 2, "Variants"),
        body_box(variants),
        heading(app, 3, "Rich content and long messages"),
        body_box(rich),
        heading(app, 4, "Groups"),
        body_box(groups),
        heading(app, 5, "Reactions and interactive content"),
        body_box(reactions),
        heading(app, 6, "Custom styling and theme tokens"),
        body_box(custom),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The Lucide mark the code icon uses, from the icon set this repository already
// ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three galleries
// draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
