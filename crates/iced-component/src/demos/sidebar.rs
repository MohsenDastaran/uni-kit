//! `sidebar` — an application sidebar: a header, scrolling groups of entries and
//! a footer, with the collapsed rail and the application's own expand trigger.
//!
//! Run it with `cargo run --example sidebar`, or in the browser as `?story=sidebar`
//! in the gallery the website embeds.
//!
//! Each section carries the heading the website page gives it — **Basic Sidebar**,
//! **Collapsible Sidebar**, **Multiple Groups** and **With Badges and
//! Suffixes** — and the code icon that asks the embedding page to scroll to that
//! sample. Clicking either one posts the section's index and title through
//! [`crate::bridge`], the same bridge the Slint and GPUI galleries use.
//!
//! The `// section: <title>` comments below are the contract the website reads to
//! cut this file into the usage panels a component page shows, so the code on the
//! page is the code that runs here. Keep them, one per section, in order, and keep
//! the closing `// section: end` after the last one.
//!
//! The first three headings are the three the Slint example fills. The fourth is
//! the page's own **With Badges and Suffixes** sample, drawn as the workspace in
//! GPUI's sidebar story: a header with a swap chevron, section labels, a
//! disclosed Playground group whose children indent under it (one destructive
//! child with a switch), top-level rows with trailing chevrons, a dotted badge,
//! a muted Travel and the user footer. The disclosure is the component's own,
//! wrapped around `collapsible::reveal`, so its arrow swaps and its height
//! animates the same way the accordion's does.
//!
//! Each card is drawn with the rail against the leading edge and, while the rail
//! is collapsed, the 28px `panel-left` trigger the application owns. That trigger
//! is the Slint component's own arrangement: the rail has no room for an expand
//! control, so the application places one beside it.

use iced::widget::{button, column, container, mouse_area, row, svg, text};
use iced::{Alignment, Color, Element, Length, Padding, Shadow, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::scrollable::Scrollable;
use crate::components::sidebar::{Sidebar, SidebarEntry, SidebarGroup};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;
/// The first two cards' height, `height: 320px` in the Slint example.
const CARD: f32 = 320.0;
/// The multiple-groups card, `height: 340px` in the Slint example.
const TALL_CARD: f32 = 340.0;
/// The workspace card: tall enough for the header, both groups, the disclosed
/// Playground children and the footer without the body's own scroll moving.
const WORKSPACE_CARD: f32 = 560.0;
/// The collapsed rail's expand trigger, `width: 28px; height: 28px`.
const TRIGGER: f32 = 28.0;

// The icon set is the application's, taken here from the same Lucide files the
// Slint icons folder carries (`crates/slint-component/ui/icons/`).

/// `box`, the brand mark the Slint example's header draws.
const BOX: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"/><path d="m3.3 7 8.7 5 8.7-5"/><path d="M12 22V12"/></svg>"#;
/// `panel-left`, the trigger the application owns while the rail is collapsed.
const PANEL_LEFT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/></svg>"#;
/// `house`.
const HOUSE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"/><path d="M3 10a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/></svg>"#;
/// `inbox`.
const INBOX: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/></svg>"#;
/// `settings`.
const SETTINGS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>"#;
/// `layout-dashboard`.
const LAYOUT_DASHBOARD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/></svg>"#;
/// `chart-column`.
const CHART_COLUMN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M18 17V9"/><path d="M13 17V5"/><path d="M8 17v-3"/></svg>"#;
/// `users`.
const USERS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><path d="M16 3.128a4 4 0 0 1 0 7.744"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><circle cx="9" cy="7" r="4"/></svg>"#;
/// `database`.
const DATABASE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5V19A9 3 0 0 0 21 19V5"/><path d="M3 12A9 3 0 0 0 21 12"/></svg>"#;
/// `message-circle`.
const MESSAGE_CIRCLE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/></svg>"#;
/// `layers`, the workspace mark the fourth card's header draws.
const LAYERS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z"/><path d="M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 12"/><path d="M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 17"/></svg>"#;
/// `square-terminal`, the Playground row.
const SQUARE_TERMINAL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m7 11 2-2-2-2"/><path d="M11 13h4"/><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/></svg>"#;
/// `bot`, the Models row.
const BOT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 8V4H8"/><rect width="16" height="12" x="4" y="8" rx="2"/><path d="M2 14h2"/><path d="M20 14h2"/><path d="M15 13v2"/><path d="M9 13v2"/></svg>"#;
/// `book-open`, the Documentation row.
const BOOK_OPEN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v16"/><path d="M20.001 19A2 2 0 0022 17V5a2 2 0 00-1.999-2L16 3.002A5 5 0 0012 5a5 5 0 00-4-2H4a2 2 0 00-2 2v12a2 2 0 001.999 2H8a5 5 0 014 2 5 5 0 014-2z"/></svg>"#;
/// `frame`, the Design Engineering row.
const FRAME: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="22" x2="2" y1="6" y2="6"/><line x1="22" x2="2" y1="18" y2="18"/><line x1="6" x2="6" y1="2" y2="22"/><line x1="18" x2="18" y1="2" y2="22"/></svg>"#;
/// `globe`, the muted Travel row.
const GLOBE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>"#;
/// `bell`, the badge's glyph on Design Engineering.
const BELL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"/></svg>"#;
/// `chevron-right`, the collapsed top-level rows' trailing mark.
const CHEVRON_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#;
/// `chevrons-up-down`, the header's workspace swap.
const CHEVRONS_UP_DOWN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/></svg>"#;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — sidebar")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone)]
struct App {
    /// The current row of each of the four sidebars, as `(group, entry)`.
    selected: [(usize, usize); 4],
    /// Whether each rail is collapsed. The collapsible card starts on the rail,
    /// as the Slint example sets `collapsed: true`.
    collapsed: [bool; 4],
    /// The workspace card's disclosed group and its child toggle. Both are the
    /// caller's state, which is the sidebar's controlled shape.
    playground_open: bool,
    history_on: bool,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            selected: [(0, 0), (0, 1), (1, 0), (0, 0)],
            collapsed: [false, true, false, false],
            playground_open: true,
            history_on: true,
            hovered: None,
            pressed_source: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Select {
        section: usize,
        group: usize,
        entry: usize,
    },
    /// The collapse control, or the application's own trigger, asked the rail to
    /// take this value.
    Toggle {
        section: usize,
        collapsed: bool,
    },
    /// The workspace card's Playground row was clicked: disclose or close its
    /// children.
    TogglePlayground,
    /// The Playground child's own toggle, with the value it wants.
    SetHistory(bool),
    /// The header's swap chevron. GPUI opens a workspace menu here; the demo
    /// has one workspace, so the mark is the shape and the press is inert.
    SwapWorkspace,
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
        Message::Select {
            section,
            group,
            entry,
        } => app.selected[section] = (group, entry),
        Message::Toggle { section, collapsed } => app.collapsed[section] = collapsed,
        Message::TogglePlayground => app.playground_open = !app.playground_open,
        Message::SetHistory(checked) => app.history_on = checked,
        Message::SwapWorkspace => {}
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

/// The glyph the gallery's own chrome draws, coloured from the tokens in force
/// while it renders. An SVG keeps its own black unless a closure colours it,
/// which is the trap that vanished icons in the dark theme.
fn glyph<'a>(bytes: &'static [u8], size: f32) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// The application's own expand trigger, drawn while a rail is collapsed — the
/// 28px bordered `panel-left` box the Slint example's screen puts beside the
/// rail. The rail cannot carry it: at 56px there is no room, and a control
/// outside the rail's bounds is painted under the application's content.
fn expand_trigger<'a>(section: usize) -> Element<'a, Message> {
    button(
        container(glyph(PANEL_LEFT, 16.0))
            .center_x(TRIGGER)
            .center_y(TRIGGER),
    )
    .width(Length::Fixed(TRIGGER))
    .height(Length::Fixed(TRIGGER))
    .padding(0)
    .style(|theme: &Theme, status| {
        let tokens = Tokens::of(theme);
        let fill = if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            tokens.muted
        } else {
            Color::TRANSPARENT
        };
        button::Style {
            background: (fill.a > 0.0).then_some(fill.into()),
            text_color: tokens.muted_foreground,
            border: iced::border::rounded(theme::RADIUS_SM)
                .color(tokens.border)
                .width(1.0),
            shadow: Shadow::default(),
            snap: false,
        }
    })
    .on_press(Message::Toggle {
        section,
        collapsed: false,
    })
    .into()
}

/// A section's card: the rail against the leading edge of a bordered box, with
/// the application's trigger beside it while the rail is collapsed. The trigger
/// is inset to the rail's own inner top, so the two read as one row.
fn card<'a>(
    section: usize,
    sidebar: Sidebar<'a, Message>,
    collapsed: bool,
    height: f32,
) -> Element<'a, Message> {
    let mut line = row![sidebar]
        .spacing(theme::SPACING_MD)
        .align_y(Alignment::Start)
        .height(Length::Fill);
    if collapsed {
        line = line.push(container(expand_trigger(section)).padding(Padding {
            top: theme::SPACING_MD,
            ..Padding::default()
        }));
    }

    container(line)
        .width(Length::Fill)
        .height(Length::Fixed(height))
        .style(|theme: &Theme| container::Style {
            border: iced::border::rounded(theme::RADIUS_LG)
                .color(Tokens::of(theme).border)
                .width(1.0),
            ..container::Style::default()
        })
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box the list gallery draws.
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
/// example wraps its sections in, untitled because the heading above it carries
/// the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// One row of the basic and collapsible cards: the same three destinations, so
/// the two sections differ only in the state the rail is drawn in.
fn platform_entries() -> [SidebarEntry; 3] {
    [
        SidebarEntry::new("Home").icon(HOUSE),
        SidebarEntry::new("Inbox").icon(INBOX),
        SidebarEntry::new("Settings").icon(SETTINGS),
    ]
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic Sidebar
    // One untitled group, the first row current. The brand mark is `box` and the
    // footer keeps the component's defaults, which is the Slint example's own
    // binding.
    let basic = card(
        0,
        Sidebar::new()
            .header_icon(BOX)
            .group(SidebarGroup::new("").entries(platform_entries()))
            .selected(app.selected[0].0, app.selected[0].1)
            .collapsed(app.collapsed[0])
            .on_select(|group, entry| Message::Select {
                section: 0,
                group,
                entry,
            })
            .on_toggle(|collapsed| Message::Toggle {
                section: 0,
                collapsed,
            }),
        app.collapsed[0],
        CARD,
    );

    // section: Collapsible Sidebar
    // The same group, drawn as the 56px rail. The rails' headings, labels, suffix
    // and footer name leave; the icons and the brand mark stay. The application's
    // `panel-left` trigger sits beside the rail, because the rail has no room for
    // its own.
    let collapsible = card(
        1,
        Sidebar::new()
            .header_icon(BOX)
            .group(SidebarGroup::new("Platform").entries(platform_entries()))
            .selected(app.selected[1].0, app.selected[1].1)
            .collapsed(app.collapsed[1])
            .on_select(|group, entry| Message::Select {
                section: 1,
                group,
                entry,
            })
            .on_toggle(|collapsed| Message::Toggle {
                section: 1,
                collapsed,
            }),
        app.collapsed[1],
        CARD,
    );

    // section: Multiple Groups
    // Groups are what carry the headings; an entry is the same shape whether it
    // sits in a titled group or an untitled one. The heading names the workspace
    // and the second group's first row is current, which is the Slint example's
    // binding.
    let multiple = card(
        2,
        Sidebar::new()
            .header_icon(BOX)
            .header_title("Acme")
            .group(SidebarGroup::new("Overview").entries([
                SidebarEntry::new("Dashboard").icon(LAYOUT_DASHBOARD),
                SidebarEntry::new("Reports").icon(CHART_COLUMN),
            ]))
            .group(SidebarGroup::new("Management").entries([
                SidebarEntry::new("Customers").icon(USERS).suffix("1,234"),
                SidebarEntry::new("Billing").icon(DATABASE),
                SidebarEntry::new("Support").icon(MESSAGE_CIRCLE),
            ]))
            .selected(app.selected[2].0, app.selected[2].1)
            .collapsed(app.collapsed[2])
            .on_select(|group, entry| Message::Select {
                section: 2,
                group,
                entry,
            })
            .on_toggle(|collapsed| Message::Toggle {
                section: 2,
                collapsed,
            }),
        app.collapsed[2],
        TALL_CARD,
    );

    // section: With Badges and Suffixes
    // The workspace from GPUI's sidebar story: the same rail, with a header
    // swap, two section labels, a disclosed Playground group, trailing chevrons
    // on the top-level rows, a dotted badge, a muted Travel and the user footer.
    // The disclosure is the component's own `collapsible::reveal`, so the arrow
    // swaps and the children move over the accordion's 150ms line.
    let workspace = card(
        3,
        Sidebar::new()
            .header_icon(LAYERS)
            .header_title("Company Name")
            .header_subtitle("Enterprise")
            .header_action(CHEVRONS_UP_DOWN, Message::SwapWorkspace)
            .group(
                SidebarGroup::new("Platform").entries([
                    SidebarEntry::new("Playground")
                        .icon(SQUARE_TERMINAL)
                        .children([
                            SidebarEntry::new("History")
                                .destructive(true)
                                .switch(app.history_on),
                            SidebarEntry::new("Starred"),
                            SidebarEntry::new("Settings"),
                        ])
                        .open(app.playground_open),
                    SidebarEntry::new("Models")
                        .icon(BOT)
                        .trailing_icon(CHEVRON_RIGHT),
                    SidebarEntry::new("Documentation")
                        .icon(BOOK_OPEN)
                        .trailing_icon(CHEVRON_RIGHT),
                    SidebarEntry::new("Settings")
                        .icon(SETTINGS)
                        .trailing_icon(CHEVRON_RIGHT),
                ]),
            )
            .group(
                SidebarGroup::new("Projects").entries([
                    SidebarEntry::new("Design Engineering")
                        .icon(FRAME)
                        .trailing_icon(BELL)
                        .badge(true),
                    SidebarEntry::new("Sales and Marketing")
                        .icon(CHART_COLUMN)
                        .trailing_icon(SETTINGS),
                    SidebarEntry::new("Travel").icon(GLOBE).muted(true),
                ]),
            )
            .selected(app.selected[3].0, app.selected[3].1)
            .collapsed(app.collapsed[3])
            .footer_initials("JL")
            .footer_name("Jason Lee")
            .on_select(|group, entry| match (group, entry) {
                // The row with children reports the same press a leaf does; the
                // caller is the one that decides it means "disclose".
                (0, 0) => Message::TogglePlayground,
                _ => Message::Select {
                    section: 3,
                    group,
                    entry,
                },
            })
            .on_entry_switch(|_, _, _, checked| Message::SetHistory(checked)),
        app.collapsed[3],
        WORKSPACE_CARD,
    );

    // section: end
    let selected = app.selected;
    let status = text(format!(
        "Row {} of group {} is current. Click a row to move the accent; the panel-left control, or the trigger beside a collapsed rail, folds it.",
        selected[0].1 + 1,
        selected[0].0 + 1
    ))
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, "Basic Sidebar"),
        body_box(basic),
        heading(app, 1, "Collapsible Sidebar"),
        body_box(collapsible),
        heading(app, 2, "Multiple Groups"),
        body_box(multiple),
        heading(app, 3, "With Badges and Suffixes"),
        body_box(workspace),
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
