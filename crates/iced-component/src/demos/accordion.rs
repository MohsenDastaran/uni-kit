//! `accordion` — a vertical stack of collapsible sections.
//!
//! Run it with `cargo run --example accordion`, or in the browser as
//! `?story=accordion` in the gallery the website embeds.
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
//! The markers are the component page's own `###` headings, in its order:
//! Single, Multiple, Icons and custom content, Sizes, Borderless and disabled,
//! With Borders, Handle Toggle Events, Nested Accordions. The page has no
//! heading for a caller-supplied disclosure, so that example sits under
//! **Icons and custom content**, the closest heading the page documents, and
//! its marker repeats that title. A section here toggles a panel that is
//! [`crate::components::collapsible::reveal`] underneath, so every one of them
//! animates — the accordion inherits the reveal rather than drawing its own,
//! which is the whole point of the composition. The header is still the
//! caller's element, and the open set is the caller's state.

use std::collections::BTreeSet;

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Color, Element, Length, Task, Theme};

use crate::components::accordion::{self, Accordion, Section};
use crate::components::button::{Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

// One id per accordion in the demo, so one `BTreeSet<(group, index)>` can hold
// every section's open state without a field per widget.
const SINGLE: u8 = 0;
const MULTIPLE: u8 = 1;
const ICONS: u8 = 2;
const XSMALL: u8 = 3;
const SMALL: u8 = 4;
const LARGE: u8 = 5;
const BORDERLESS: u8 = 6;
const BORDERED: u8 = 7;
const EVENTS: u8 = 8;
const OUTER: u8 = 9;
const INNER: u8 = 10;
const CUSTOM_ICON: u8 = 11;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — accordion")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    /// Which sections are open, as `(group, index)`. Slint's `open-index` is one
    /// section in single mode; the caller's set is what an iced application
    /// keeps instead.
    open: BTreeSet<(u8, u8)>,
    /// The unread count the "Icons and custom content" panel's controls move.
    notifications: u32,
    /// The last title toggled, so the status line reports the event the caller
    /// received.
    notice: Option<String>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle {
        group: u8,
        index: u8,
        title: &'static str,
        open: bool,
        exclusive: bool,
    },
    MarkAllRead,
    SimulateNew,
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
        open: [
            (SINGLE, 0),
            (MULTIPLE, 0),
            (MULTIPLE, 1),
            (ICONS, 0),
            (XSMALL, 0),
            (SMALL, 0),
            (LARGE, 0),
            (BORDERLESS, 0),
            (BORDERED, 0),
            (EVENTS, 0),
            (OUTER, 0),
            (INNER, 0),
            (CUSTOM_ICON, 0),
        ]
        .into_iter()
        .collect(),
        notifications: 3,
        notice: None,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Toggle {
            group,
            index,
            title,
            open,
            exclusive,
        } => {
            // Single mode: opening one section closes the others in its group.
            // An accordion the caller leaves alone is "multiple" by construction
            // — the state is a set, so nothing else closes on its own.
            if exclusive && open {
                app.open.retain(|(slot, _)| *slot != group);
            }
            if open {
                app.open.insert((group, index));
            } else {
                app.open.remove(&(group, index));
            }
            app.notice = Some(format!(
                "{title} {}.",
                if open { "opened" } else { "closed" }
            ));
        }
        Message::MarkAllRead => {
            app.notifications = 0;
            app.notice = Some("Marked all as read.".to_string());
        }
        Message::SimulateNew => {
            app.notifications += 1;
            app.notice = Some("A new notification arrived.".to_string());
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

// ------------------------------------------------------------------- pieces

/// The title size at each control size, the same ladder `theme` gives the
/// component: text-xs, text-sm, text-md.
fn label_size(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => theme::FONT_SIZE_XS,
        ControlSize::Large => theme::FONT_SIZE_MD,
        _ => theme::FONT_SIZE_SM,
    }
}

fn is_open(app: &App, group: u8, index: u8) -> bool {
    app.open.contains(&(group, index))
}

/// The toggle a section's header reports: the caller flips its own state in
/// `update`, so the message carries the value to write.
fn toggle(app: &App, group: u8, index: u8, title: &'static str, exclusive: bool) -> Message {
    Message::Toggle {
        group,
        index,
        title,
        open: !is_open(app, group, index),
        exclusive,
    }
}

/// A mark from the icon set this repository ships (`crates/slint-component/ui/icons`,
/// Lucide, ISC), so the two galleries draw the same glyphs. The ink is resolved
/// inside the style closure, because the theme is only known when iced asks —
/// an unresolved colour is the trap that once drew a dark icon as black.
fn mark<'a>(markup: &'static [u8], size: f32, color: fn(&Theme) -> Color) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(markup))
        .width(size)
        .height(size)
        .style(move |theme: &Theme, _status| svg::Style {
            color: Some(color(theme)),
        })
        .into()
}

/// The muted ink most leading icons draw in.
fn muted(theme: &Theme) -> Color {
    Tokens::of(theme).muted_foreground
}

/// A line of text in one of the semantic inks. It is the `Text` itself rather
/// than an element, so a caller can give it `Length::Fill` in a row.
fn line<'a>(
    value: &'a str,
    size: f32,
    color: fn(&Theme) -> Color,
) -> iced::widget::Text<'a, Theme, iced::Renderer> {
    text(value)
        .size(size)
        .style(move |theme: &Theme| text::Style {
            color: Some(color(theme)),
        })
}

/// A small muted caption above one of the size examples.
fn caption<'a>(value: &'a str) -> Element<'a, Message> {
    text(value)
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// An accordion whose sections are a title and one line of body text. This is
/// the shape the Single, Multiple, Sizes, Borderless, With Borders and Handle
/// Toggle Events samples share; they differ only in size, border and whether
/// opening one section closes the others.
///
/// The header reports through [`Section::on_toggle`] rather than carrying the
/// press itself, because the header row owns the whole target: `accordion::
/// text_header` draws the title, the component draws the chevron lane, and
/// [`accordion::header_target`] makes both of them toggle — which is the Slint
/// `TouchArea` over the row, and is what the one mark that says "this opens"
/// needs to be worth clicking.
fn panel<'a>(
    app: &App,
    group: u8,
    exclusive: bool,
    size: ControlSize,
    bordered: bool,
    rows: &[(&'static str, &'static str)],
) -> Element<'a, Message> {
    let size_label = label_size(size);
    let sections: Vec<Section<'a, Message>> = rows
        .iter()
        .enumerate()
        .map(|(index, (title, body))| {
            let index = index as u8;
            let open = is_open(app, group, index);
            Section::new(header(title, size_label, |theme| {
                Tokens::of(theme).foreground
            }))
            .on_toggle(toggle(app, group, index, title, exclusive))
            .open(open)
            .content(line(body, size_label, muted))
        })
        .collect();

    Accordion::new(sections)
        .size(size)
        .bordered(bordered)
        .into()
}

/// The header *content* of a section: the title alone. The chrome around it —
/// the row's padding, the chevron lane and the hit area that covers both — is
/// the component's, mounted by `Section::on_toggle`; this is the piece the
/// click's message is built from.
fn header<'a>(title: &'a str, label_size: f32, color: fn(&Theme) -> Color) -> Element<'a, Message> {
    line(title, label_size, color)
        .width(Length::Fill)
        .font(iced::Font {
            weight: iced::font::Weight::Medium,
            ..iced::Font::DEFAULT
        })
        .into()
}

/// The same, with a leading icon beside the title — the two-piece header the
/// Slint `AccordionItem` draws when its `icon` is set.
fn labeled_header<'a>(
    icon: impl Into<Element<'a, Message>>,
    title: &'a str,
    label_size: f32,
    gap: f32,
) -> Element<'a, Message> {
    row![
        icon.into(),
        header(title, label_size, |theme| { Tokens::of(theme).foreground })
    ]
    .spacing(gap)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()
}

/// A section whose disclosure is the caller's own mark rather than the
/// component's default chevron. `Section::disclosure` takes any element, so
/// the icon set stays the caller's: this swaps Lucide's `arrow-right` for
/// `arrow-down` as the section opens, where the component would have drawn its
/// `chevron-down`/`chevron-up`.
fn arrow_section<'a>(
    app: &App,
    group: u8,
    index: u8,
    title: &'static str,
    body: &'static str,
) -> Section<'a, Message> {
    let open = is_open(app, group, index);
    Section::new(header(title, label_size(ControlSize::Medium), |theme| {
        Tokens::of(theme).foreground
    }))
    .on_toggle(toggle(app, group, index, title, false))
    .open(open)
    .disclosure(mark(
        if open { ARROW_DOWN } else { ARROW_RIGHT },
        12.0,
        muted,
    ))
    .content(line(body, theme::FONT_SIZE_SM, muted))
}

/// The header of a section that is not interactive: no button, no hover, and
/// the muted ink the component's `ink` helper gives a disabled title. The Slint
/// accordion carries an `enabled` flag; iced's caller renders the disabled row
/// instead of wiring one.
fn disabled_header<'a>(title: &'a str, size: f32) -> Element<'a, Message> {
    container(
        line(title, size, |theme| {
            accordion::ink(Tokens::of(theme), false)
        })
        .width(Length::Fill),
    )
    .width(Length::Fill)
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
            let (fill, edge) = if pressed {
                (0.34, 0.7)
            } else if hovered {
                (0.22, 0.55)
            } else {
                (0.12, 0.35)
            };
            container::Style {
                background: Some(primary.scale_alpha(fill).into()),
                border: iced::border::rounded(theme::RADIUS_LG)
                    .color(primary.scale_alpha(edge))
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

// --------------------------------------------------------------------- view

fn view(app: &App) -> Element<'_, Message> {
    // section: Single
    // Opening one section closes the others: single mode is the caller's own
    // rule over its open set, which is why the last of these three is not open.
    let single = panel(
        app,
        SINGLE,
        true,
        ControlSize::Medium,
        true,
        &[
            (
                "What is Iced?",
                "A Rust desktop application framework built on iced, with styled \
                 components inspired by macOS and Windows controls.",
            ),
            (
                "Which platforms are supported?",
                "macOS on Apple silicon and Intel, Linux on x86_64, and Windows on \
                 x86_64.",
            ),
            (
                "Can I restyle the components?",
                "Yes. Every component reads semantic theme tokens, so switching the \
                 palette restyles the whole application.",
            ),
        ],
    );

    // section: Multiple
    // The open set holds every open section, so an accordion the caller does not
    // close on a toggle simply allows several — two are open on boot.
    let multiple = panel(
        app,
        MULTIPLE,
        false,
        ControlSize::Medium,
        true,
        &[
            (
                "Shipping",
                "Orders ship within two business days from our warehouse.",
            ),
            (
                "Returns",
                "Return any unused item within 30 days for a full refund.",
            ),
            (
                "Warranty",
                "Hardware is covered for two years against manufacturing defects.",
            ),
        ],
    );

    // section: Icons and custom content
    // The header is the caller's element, so a leading icon is just another
    // child; the panel is the caller's too, so the notification controls below
    // live inside a section.
    let unread = if app.notifications == 0 {
        "You're all caught up.".to_string()
    } else {
        format!("{} unread notifications.", app.notifications)
    };
    let icons = Accordion::new([
        Section::new(labeled_header(
            mark(BELL, 14.0, muted),
            "Notifications",
            theme::FONT_SIZE_SM,
            8.0,
        ))
        .on_toggle(toggle(app, ICONS, 0, "Notifications", false))
        .open(is_open(app, ICONS, 0))
        .content(column![
            text(unread)
                .size(theme::FONT_SIZE_SM)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
            row![
                Button::new("Mark all as read")
                    .size(ControlSize::XSmall)
                    .on_press(Message::MarkAllRead),
                Button::new("Simulate new")
                    .variant(Variant::Ghost)
                    .size(ControlSize::XSmall)
                    .icon(mark(PLUS, 14.0, |theme| {
                        Tokens::of(theme).muted_foreground
                    }))
                    .on_press(Message::SimulateNew),
            ]
            .spacing(8.0),
        ]),
        Section::new(labeled_header(
            mark(SETTINGS, 14.0, muted),
            "Settings",
            theme::FONT_SIZE_SM,
            8.0,
        ))
        .on_toggle(toggle(app, ICONS, 1, "Settings", false))
        .open(is_open(app, ICONS, 1))
        .content(line(
            "Preferences for appearance, keyboard shortcuts, and updates.",
            theme::FONT_SIZE_SM,
            muted,
        )),
    ])
    .size(ControlSize::Medium);

    // section: Icons and custom content
    // The marker repeats the heading above because the page has no heading of
    // its own for a caller-supplied disclosure: this example sits under the
    // closest one the page documents. `Section::disclosure` hands the trailing
    // lane the caller's element — here Lucide's arrow in place of the
    // component's default chevron — so the icon set stays the caller's.
    let custom_icon = Accordion::new([
        arrow_section(
            app,
            CUSTOM_ICON,
            0,
            "Custom arrow",
            "The disclosure in the trailing lane is the caller's arrow.",
        ),
        arrow_section(
            app,
            CUSTOM_ICON,
            1,
            "Second section",
            "Each section draws the mark its own open state wants.",
        ),
    ])
    .size(ControlSize::Medium);

    // section: Sizes
    // The size ladder: compact rows at xsmall, sidebar rows at small, roomy rows
    // at large. iced has no wrapping grid, so the three stack.
    let sizes = column![
        caption("Extra small"),
        panel(
            app,
            XSMALL,
            false,
            ControlSize::XSmall,
            true,
            &[
                ("Extra small", "Compact rows for dense panels."),
                ("Second section", "Content."),
            ],
        ),
        caption("Small"),
        panel(
            app,
            SMALL,
            false,
            ControlSize::Small,
            true,
            &[
                ("Small", "Rows for sidebars and inspectors."),
                ("Second section", "Content."),
            ],
        ),
        caption("Large"),
        panel(
            app,
            LARGE,
            false,
            ControlSize::Large,
            true,
            &[
                ("Large", "Roomy rows for settings pages."),
                ("Second section", "Content."),
            ],
        ),
    ]
    .spacing(8.0);

    // section: Borderless and disabled
    // Borderless drops the card but keeps the separating rule; disabled is the
    // caller not wiring a header at all, drawn in the muted ink the component's
    // `ink` helper gives a disabled title.
    let borderless_and_disabled = column![
        panel(
            app,
            BORDERLESS,
            false,
            ControlSize::Medium,
            false,
            &[
                ("Borderless", "Items are still separated by a line."),
                ("Second section", "Content."),
            ],
        ),
        Accordion::new([
            Section::new(disabled_header(
                "Disabled section",
                label_size(ControlSize::Medium)
            ))
            .open(false)
            .content(line("Content.", theme::FONT_SIZE_SM, muted)),
            Section::new(disabled_header(
                "Another disabled section",
                label_size(ControlSize::Medium),
            )),
        ])
        .size(ControlSize::Medium),
    ]
    .spacing(16.0);

    // section: With Borders
    // `bordered(true)` is the default and draws the single rounded card the
    // sections sit in, with the padding that keeps the border off the rows.
    let with_borders = panel(
        app,
        BORDERED,
        false,
        ControlSize::Medium,
        true,
        &[("Section 1", "Content 1"), ("Section 2", "Content 2")],
    );

    // section: Handle Toggle Events
    // Every header reports the caller's message with the section's new state, so
    // the caller writes the open set and can show what it received.
    let events = panel(
        app,
        EVENTS,
        true,
        ControlSize::Medium,
        true,
        &[
            ("Section 1", "The first section."),
            ("Section 2", "The second section."),
            ("Section 3", "The third section."),
        ],
    );
    let events = column![
        events,
        line(
            app.notice
                .as_deref()
                .unwrap_or("Click a section title to toggle it."),
            theme::FONT_SIZE_XS,
            muted,
        ),
    ]
    .spacing(8.0);

    // section: Nested Accordions
    // A section's content is the caller's, so it can be another accordion; the
    // inner one reveals inside the outer one's reveal.
    let inner = Accordion::new([
        Section::new(header(
            "Child section 1",
            label_size(ControlSize::Small),
            |theme| Tokens::of(theme).foreground,
        ))
        .on_toggle(toggle(app, INNER, 0, "Child section 1", true))
        .open(is_open(app, INNER, 0))
        .content(line(
            "Content of the first child section.",
            theme::FONT_SIZE_SM,
            muted,
        )),
        Section::new(header(
            "Child section 2",
            label_size(ControlSize::Small),
            |theme| Tokens::of(theme).foreground,
        ))
        .on_toggle(toggle(app, INNER, 1, "Child section 2", true))
        .open(is_open(app, INNER, 1))
        .content(line(
            "Content of the second child section.",
            theme::FONT_SIZE_SM,
            muted,
        )),
    ])
    .size(ControlSize::Small);

    let nested = Accordion::new([Section::new(header(
        "Parent section",
        label_size(ControlSize::Medium),
        |theme| Tokens::of(theme).foreground,
    ))
    .on_toggle(toggle(app, OUTER, 0, "Parent section", false))
    .open(is_open(app, OUTER, 0))
    .content(inner)])
    .size(ControlSize::Medium);

    // section: end
    let status = text(match &app.notice {
        Some(notice) => notice.clone(),
        None => "Click a section title: each accordion is the caller's own open set.".to_string(),
    })
    .size(theme::FONT_SIZE_SM)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, "Single"),
        body_box(single),
        heading(app, 1, "Multiple"),
        body_box(multiple),
        heading(app, 2, "Icons and custom content"),
        body_box(icons),
        heading(app, 3, "Icons and custom content"),
        body_box(custom_icon),
        heading(app, 4, "Sizes"),
        body_box(sizes),
        heading(app, 5, "Borderless and disabled"),
        body_box(borderless_and_disabled),
        heading(app, 6, "With Borders"),
        body_box(with_borders),
        heading(app, 7, "Handle Toggle Events"),
        body_box(events),
        heading(app, 8, "Nested Accordions"),
        body_box(nested),
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

const BELL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"/></svg>"#;

const SETTINGS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>"#;

const PLUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>"#;

/// Lucide's `arrow-right`, the custom disclosure while a section is closed.
const ARROW_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>"#;

/// Lucide's `arrow-down`, the same disclosure while a section is open.
const ARROW_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14"/><path d="m19 12-7 7-7-7"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
