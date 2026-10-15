//! `dropdown_button` — the whole gallery the Slint example shows: the basic
//! split, the variants, the inner button's own options, and the menu width.
//!
//! Run it with `cargo run --example dropdown_button`, or in the browser as
//! `?story=dropdown-button` in the gallery the website embeds.
//!
//! Each section carries the heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! The open flag is the gallery's, exactly as a popover's is: a dropdown button
//! has no state of its own, so each control below renders from its slot in
//! `open` and reports every request to move it through `on_open_change`.
//!
//! The shortcuts below are the Slint example's, with `⌘` and `⌥` spelled `Cmd`
//! and `Alt`: on wasm the monospace family resolves to the embedded Fira Sans,
//! which has no glyph for either mark, so `⌘E` painted as `E`. The marks the face
//! does have (`⇧`, `⌫`) are kept, because they render.
//!
//! The caret is the component's own Lucide `chevron-down`, the mark the Slint
//! trigger hardcodes, so no control below supplies one. The two halves square the
//! corners that meet and share one outline: a primary split joins into the single
//! button the reference shows, and a bordered one keeps the edge between them.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::Variant;
use crate::components::dropdown_button::{DropdownAction, DropdownButton};
use crate::components::group_box::{self, GroupBox};
use crate::components::menu::Item;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example and the component page
/// have them. The index is what the page is asked to scroll to.
const SECTIONS: [&str; 5] = [
    "Usage",
    "Basic split",
    "Variants",
    "Inner button options",
    "With custom anchor",
];

/// One open flag per control in the gallery, in the order it is built:
/// 1 + 1 + 5 + 3 + 2.
const SLOTS: usize = 12;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — dropdown button")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// Whether each control's menu is showing.
    open: [bool; SLOTS],
    /// What each control last reported: its row, and which half reported.
    chosen: [Option<(usize, DropdownAction)>; SLOTS],
    /// The row a menu has under its pointer, which the control draws.
    highlighted: Option<usize>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A control asked to be opened or closed; the slot says which one.
    Opened { slot: usize, open: bool },
    /// A control reported a choice: its row, and which half.
    Chosen {
        index: usize,
        action: DropdownAction,
        slot: usize,
    },
    /// A control's menu reported the row under its pointer.
    Highlighted(usize),
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
        Message::Opened { slot, open } => {
            if let Some(flag) = app.open.get_mut(slot) {
                *flag = open;
            }
        }
        Message::Chosen {
            index,
            action,
            slot,
        } => {
            if let Some(chosen) = app.chosen.get_mut(slot) {
                *chosen = Some((index, action));
            }
        }
        Message::Highlighted(index) => app.highlighted = Some(index),
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
/// is the same 28px box the popover gallery draws.
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

/// A row of controls, narrowed to the element the gallery stores. `Row`'s
/// `Into<Element>` cannot be inferred through `let x: Element<_> = row![..]`,
/// because a row has more than one element type it can become, so the
/// conversion is written out.
fn layout_row<'a, Message: 'a>(
    row: iced::widget::Row<'a, Message, Theme, iced::Renderer>,
) -> Element<'a, Message> {
    Element::from(row)
}

/// A control's gallery wires: the flag the gallery owns, and the two reports.
/// The slot is what makes the message say which control spoke.
macro_rules! wired {
    ($app:expr, $slot:expr, $control:expr) => {
        $control
            .open($app.open[$slot])
            .highlighted($app.highlighted)
            .on_chosen(move |index, action| Message::Chosen {
                index,
                action,
                slot: $slot,
            })
            .on_highlight(Message::Highlighted)
            .on_open_change(move |open| Message::Opened { slot: $slot, open })
    };
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Usage
    // The whole of a dropdown button: the label on the leading half, the menu on
    // the trailing one, and the message that says which half reported. A menu row
    // reports `DropdownAction::Menu` with its own index; the leading half reports
    // `DropdownAction::Main`.
    let usage: Element<Message> = Element::from(wired!(
        app,
        0,
        DropdownButton::new("Export")
            .variant(Variant::Primary)
            .items([
                Item::new("Export all rows (.csv)")
                    .icon(DOWNLOAD)
                    .shortcut("Cmd E"),
                Item::new("Download report (.pdf)").icon(FILE_TEXT),
                Item::separator(),
                Item::new("Copy as Markdown").icon(COPY),
            ])
    ));

    let usage_section = column![
        usage,
        muted_line("`on_chosen` says which half reported, and which row when it was the menu."),
    ]
    .spacing(8.0);

    // section: Basic split
    // The leading half runs its own action; the trailing half opens the menu, so
    // the label is never a way to open the menu by accident. The primary variant
    // is the reference's ordinary shape: the two fills join into one button with
    // a trailing chevron.
    let basic: Element<Message> = Element::from(wired!(
        app,
        1,
        DropdownButton::new("Export")
            .variant(Variant::Primary)
            .items([
                Item::new("Export all rows (.csv)").shortcut("Cmd E"),
                Item::new("Download report (.pdf)"),
            ])
    ));

    let basic_section = column![
        basic,
        muted_line("Press the label for the main action, the caret for the menu."),
    ]
    .spacing(8.0);

    // section: Variants
    // The variant and the size belong to the whole control: one value styles
    // both halves, which is Slint's `variant` and GPUI's effective variant.
    let variants: Element<Message> = layout_row(row![
        wired!(
            app,
            2,
            DropdownButton::new("Default").items([
                Item::new("Rename").icon(PENCIL),
                Item::new("Duplicate").icon(COPY),
            ])
        ),
        wired!(
            app,
            3,
            DropdownButton::new("Primary")
                .variant(Variant::Primary)
                .items([
                    Item::new("Publish now").icon(UPLOAD),
                    Item::new("Schedule…").icon(CLOCK),
                ])
        ),
        wired!(
            app,
            4,
            DropdownButton::new("Outline")
                .variant(Variant::Primary)
                .outline(true)
                .selected(true)
                .items([
                    Item::new("Export all rows (.csv)")
                        .icon(DOWNLOAD)
                        .checked(true),
                    Item::new("Export selection").icon(FILE_TEXT),
                ])
        ),
        wired!(
            app,
            5,
            DropdownButton::new("Small")
                .variant(Variant::Secondary)
                .size(ControlSize::Small)
                .items([
                    Item::new("Compact view").icon(LIST),
                    Item::new("Comfortable view").icon(TABLE),
                ])
        ),
        wired!(
            app,
            6,
            DropdownButton::new("Ghost")
                .variant(Variant::Ghost)
                .items([
                    Item::new("Duplicate").icon(COPY),
                    Item::new("Delete").icon(TRASH),
                ])
        ),
    ]
    .spacing(12.0)
    .align_y(Alignment::Center));

    let variants_section = body_box(variants);

    // section: Inner button options
    // The leading half owns its icon, its loading state and whether it answers at
    // all; the whole control owns the variant and the size. The muted `Save` is
    // the reference's own split: its border draws the edge between the halves,
    // where the primary `Export` above joins into one button.
    let inner: Element<Message> = layout_row(row![
        wired!(
            app,
            7,
            DropdownButton::new("Save")
                .variant(Variant::Default)
                .icon(|| icon(DOWNLOAD))
                .items([
                    Item::new("Save as new file…").icon(FILE).shortcut("Cmd S"),
                    Item::new("Save as template…").icon(COPY),
                ])
        ),
        wired!(
            app,
            8,
            DropdownButton::new("Saving…")
                .variant(Variant::Primary)
                .loading(true)
                .items([Item::new("Save as new file…").icon(FILE)])
        ),
        wired!(
            app,
            9,
            DropdownButton::new("Unavailable")
                .variant(Variant::Primary)
                .disabled(true)
                .items([Item::new("Export all rows (.csv)").icon(DOWNLOAD)])
        ),
    ]
    .spacing(12.0)
    .align_y(Alignment::Center));

    let inner_section = body_box(inner);

    // section: With custom anchor
    // A control whose label is only an icon, with the caret the component draws
    // itself, and a menu wider than its button. The menu hangs from the trailing
    // edge, which is where the popover's bottom placement and the control's own
    // width put it.
    let anchors: Element<Message> = layout_row(row![
        wired!(
            app,
            10,
            DropdownButton::new("")
                .icon(|| icon(PLAY))
                .variant(Variant::Secondary)
                .size(ControlSize::Small)
                .menu_width(240.0)
                .items([
                    Item::new("Run with release profile")
                        .icon(ZAP)
                        .shortcut("⇧Cmd R"),
                    Item::new("Run with debugger").icon(BOT),
                    Item::separator(),
                    Item::new("Stop").icon(SQUARE).disabled(true),
                ])
        ),
        wired!(
            app,
            11,
            DropdownButton::new("Disabled")
                .variant(Variant::Secondary)
                .disabled(true)
                .items([Item::new("Open recent").icon(CLOCK)])
        ),
    ]
    .spacing(12.0)
    .align_y(Alignment::Center));

    let anchors_section = body_box(anchors);

    // section: end
    let status = text(
        match app
            .chosen
            .iter()
            .enumerate()
            .find_map(|(slot, chosen)| chosen.map(|(index, action)| (slot, index, action)))
        {
            Some((slot, _, DropdownAction::Main)) => {
                format!("Control {}: the leading half ran its action.", slot + 1)
            }
            Some((slot, index, DropdownAction::Menu)) => {
                format!("Control {} chose menu row {}.", slot + 1, index + 1)
            }
            None => "Press a label for its action, or a caret to open the menu.".to_string(),
        },
    )
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        body_box(usage_section),
        heading(app, 1, SECTIONS[1]),
        basic_section,
        heading(app, 2, SECTIONS[2]),
        variants_section,
        heading(app, 3, SECTIONS[3]),
        inner_section,
        heading(app, 4, SECTIONS[4]),
        anchors_section,
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

/// An icon the components draw themselves: a 14px SVG in whatever ink the
/// control gives it. The glyphs are the caller's, because an iced application
/// owns its assets — the Slint example names them and its build script resolves
/// the folder.
fn icon(bytes: &'static [u8]) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(14.0)
        .height(14.0)
        .into()
}

// The marks the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so
// the two galleries draw the same glyphs rather than two lookalikes. The
// caret is not among them: the component draws its own `chevron-down`.

const DOWNLOAD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/><path d="M12 15V3"/></svg>"#;

const FILE_TEXT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/><path d="M10 9H8"/><path d="M16 13H8"/><path d="M16 17H8"/></svg>"#;

const FILE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/></svg>"#;

const COPY: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>"#;

const PENCIL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/><path d="m15 5 4 4"/></svg>"#;

const TRASH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>"#;

const UPLOAD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m17 8-5-5-5 5"/><path d="M12 3v12"/></svg>"#;

const CLOCK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#;

const LIST: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12h.01"/><path d="M3 18h.01"/><path d="M3 6h.01"/><path d="M8 12h13"/><path d="M8 18h13"/><path d="M8 6h13"/></svg>"#;

const TABLE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3v18"/><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="M3 15h18"/></svg>"#;

const PLAY: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="6 3 20 12 6 21 6 3"/></svg>"#;

const ZAP: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z"/></svg>"#;

const BOT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 8V4H8"/><rect width="16" height="12" x="4" y="8" rx="2"/><path d="M2 14h2"/><path d="M20 14h2"/><path d="M15 13v2"/><path d="M9 13v2"/></svg>"#;

const SQUARE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
