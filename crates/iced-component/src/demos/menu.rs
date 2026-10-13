//! `menu` — the whole gallery the Slint example shows: a dropdown menu of file
//! actions, the icon set, the check state, the disabled row, the destructive
//! row, and the keyboard the menu answers.
//!
//! Run it with `cargo run --example menu`, or in the browser as
//! `?story=menu` in the gallery the website embeds.
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
//! The gallery owns the state, as it does for the popover: one flag per menu
//! below, and the row each menu last reported. A menu never moves its own
//! highlight out of the tree the popover rebuilds — that is the port's — but
//! whether the panel is showing is the application's, and so is what a choice
//! means.
//!
//! The icons are named in the Slint example by string and resolved by its build
//! script. There is no such resolver here: an iced application owns its assets,
//! so the bytes are in this file, taken from `crates/slint-component/ui/icons`
//! (Lucide, ISC) so the two galleries draw the same glyphs.

use iced::widget::{column, container, mouse_area, scrollable, svg, text};
use iced::keyboard;
use iced::{Alignment, Element, Length, Subscription, Task, Theme};

use crate::components::button::Button;
use crate::components::group_box::{self, GroupBox};
use crate::components::menu::{Item, Menu, PANEL_WIDTH};
use crate::components::popover::Popover;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example and the component page
/// have them. The index is what the page is asked to scroll to.
const SECTIONS: [&str; 6] = [
    "Usage",
    "Icons",
    "Check state",
    "Disabled State",
    "Destructive",
    "Keyboard Shortcuts",
];

/// One open flag per menu in the gallery, in the order the sections show them.
const SLOTS: usize = 6;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — menu")
        .theme(theme)
        .subscription(subscription)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// Whether each menu is showing. iced has no menu state to own them, so the
    /// application holds the flag and answers every request to change it.
    open: [bool; SLOTS],
    /// The last row each menu reported, so a choice is visible.
    chosen: [Option<usize>; SLOTS],
    /// The row the pointer or a key highlighted last, and which menu holds it.
    /// The highlight is the application's — see the component's type comment —
    /// so a gallery of six menus has one row lit and one menu that owns it.
    highlighted: (Option<usize>, Option<usize>),
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A menu asked to be opened or closed; the slot says which one.
    Opened { slot: usize, open: bool },
    /// A menu row was chosen, by slot and by index.
    Chosen { slot: usize, index: usize },
    /// The highlight moved, to a menu's row.
    Highlighted { slot: usize, index: usize },
    /// A traversal key was pressed: `↑`/`↓`/`Home`/`End`, or `Enter` to choose,
    /// or `Escape` to dismiss.
    Key(Key),
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
        Message::Opened { slot, open } => {
            if let Some(flag) = app.open.get_mut(slot) {
                *flag = open;
            }
        }
        Message::Chosen { slot, index } => {
            if let Some(chosen) = app.chosen.get_mut(slot) {
                *chosen = Some(index);
            }
        }
        Message::Highlighted { slot, index } => app.highlighted = (Some(index), Some(slot)),
        // The keys act on the menu that is open and holds the highlight. This
        // gallery can have one open at a time, and the subscription cannot see
        // which, so `update` reads it from the state.
        Message::Key(key) => {
            if let Some(slot) = app.open.iter().position(|open| *open) {
                let (index, owner) = app.highlighted;
                // The highlight is one row of one menu. When it belongs to
                // another menu, a step starts from this menu's edge rather than
                // from the neighbour's row.
                let from = if owner == Some(slot) { index } else { None };
                match key {
                    Key::Choose => {
                        // Enter chooses the highlighted row, or the first
                        // actionable one when the key is the first thing after
                        // opening the menu.
                        let row = from.or_else(|| step_to(slot, None, Key::Down));
                        if let Some(index) = row
                            && menu_items(slot).get(index).is_some_and(Item::is_actionable)
                        {
                            app.chosen[slot] = Some(index);
                            app.highlighted = (Some(index), Some(slot));
                            // The Slint example leaves the menu open on a
                            // choice; the key does too, so a second Enter
                            // chooses again.
                        }
                    }
                    Key::Dismiss => {
                        app.open[slot] = false;
                        app.highlighted = (None, None);
                    }
                    step => app.highlighted = (step_to(slot, from, step), Some(slot)),
                }
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

/// A key the menu answers, mapped from the keyboard by the subscription and
/// acted on by `update`, which is where the open menu and its highlight are.
///
/// The subscription is `filter_map`ped, and iced requires that closure to be
/// non-capturing, so nothing about the application's state can be read there.
/// It maps the key; `update` decides what the key means.
#[derive(Debug, Clone, Copy)]
enum Key {
    Up,
    Down,
    First,
    Last,
    Choose,
    Dismiss,
}

/// The keyboard the menu component leaves to its host.
///
/// `Menu` reports the row the pointer entered and draws the row the caller says
/// is highlighted, but iced 0.14 has no focus handle an overlay can take, so the
/// keys are the application's. `↑`/`↓` step, `Home`/`End` jump, `Enter` and
/// `Space` choose the highlighted row and `Escape` closes the menu — the keys the
/// Slint example shows and the ones the component page documents.
///
/// iced requires the `filter_map` closure to be non-capturing, so this reads no
/// state: it maps the key, and `update` — which sees the open menu and its
/// highlight — decides what the key means.
fn subscription(_app: &App) -> Subscription<Message> {
    iced::keyboard::listen().filter_map(|event| match event {
        keyboard::Event::KeyPressed { key, .. } => match key.as_ref() {
            keyboard::Key::Named(keyboard::key::Named::ArrowDown) => Some(Message::Key(Key::Down)),
            keyboard::Key::Named(keyboard::key::Named::ArrowUp) => Some(Message::Key(Key::Up)),
            keyboard::Key::Named(keyboard::key::Named::Home) => Some(Message::Key(Key::First)),
            keyboard::Key::Named(keyboard::key::Named::End) => Some(Message::Key(Key::Last)),
            keyboard::Key::Named(keyboard::key::Named::Enter) => Some(Message::Key(Key::Choose)),
            keyboard::Key::Named(keyboard::key::Named::Space) => Some(Message::Key(Key::Choose)),
            keyboard::Key::Named(keyboard::key::Named::Escape) => Some(Message::Key(Key::Dismiss)),
            _ => None,
        },
        _ => None,
    })
}

/// The rows a section's menu holds, in the Slint example's order.
///
/// The shortcuts are the Slint example's, with the two modifier marks the wasm
/// face cannot draw spelled out. On wasm the monospace family resolves to the
/// embedded Fira Sans, which has `⇧` and `⌫` and no `⌘` or `⌥`; `⌘N` therefore
/// painted as `N`, a shortcut that reads wrong. `Cmd N` and `Alt Z` render in
/// the 48px column and mean the same thing on every platform. The marks the face
/// does have are kept, so `⇧Cmd Z` is a shift and a command and not three words.
fn menu_items(slot: usize) -> Vec<Item> {
    match slot {
        0 => vec![
            Item::new("New file").shortcut("Cmd N"),
            Item::new("Open…").shortcut("Cmd O"),
            Item::new("Save").shortcut("Cmd S"),
            Item::separator(),
            Item::new("Export as CSV"),
        ],
        1 => vec![
            Item::new("Rename").icon(PENCIL).shortcut("F2"),
            Item::new("Duplicate").icon(COPY).shortcut("Cmd D"),
            Item::new("Move to…").icon(FOLDER),
            Item::new("Delete").icon(TRASH).shortcut("⌫"),
        ],
        2 => vec![
            Item::new("Show minimap").checked(true),
            Item::new("Wrap lines").shortcut("Alt Z").checked(true),
            Item::new("Show whitespace"),
        ],
        3 => vec![
            Item::new("Cut").shortcut("Cmd X"),
            Item::new("Copy").shortcut("Cmd C"),
            Item::new("Paste").shortcut("Cmd V").disabled(true),
        ],
        4 => vec![
            Item::new("Archive").icon(ARCHIVE),
            Item::separator(),
            Item::new("Delete project").icon(TRASH).danger(true),
        ],
        _ => vec![
            Item::new("Undo").shortcut("Cmd Z"),
            Item::new("Redo").shortcut("⇧Cmd Z"),
            Item::separator(),
            Item::new("Find…").shortcut("Cmd F"),
            Item::new("Replace…").shortcut("Cmd H").disabled(true),
        ],
    }
}

/// The row a step lands on, skipping separators and disabled rows and stopping
/// at the ends rather than wrapping — the component's own stepping rule, written
/// where the key is handled.
fn step_to(slot: usize, from: Option<usize>, step: Key) -> Option<usize> {
    let items = menu_items(slot);
    let last = items.len().checked_sub(1);
    let actionable = |index: usize| items.get(index).is_some_and(Item::is_actionable);
    match step {
        Key::First => (0..items.len()).find(|index| actionable(*index)),
        Key::Last => (0..items.len()).rev().find(|index| actionable(*index)),
        Key::Down | Key::Up => {
            let last = last?;
            let delta: isize = if matches!(step, Key::Down) { 1 } else { -1 };
            let start = match from {
                Some(index) if index <= last && actionable(index) => index,
                Some(index) => index.min(last),
                None => {
                    return if delta > 0 {
                        (0..=last).find(|index| actionable(*index))
                    } else {
                        (0..=last).rev().find(|index| actionable(*index))
                    };
                }
            };
            let mut index = start;
            loop {
                let next = index as isize + delta;
                if next < 0 || next > last as isize {
                    return Some(start);
                }
                index = next as usize;
                if actionable(index) {
                    return Some(index);
                }
            }
        }
        // `Choose` and `Dismiss` are not moves; `update` answers them itself.
        Key::Choose | Key::Dismiss => from,
    }
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

    iced::widget::row![label, icon]
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

/// The trigger every menu below opens: the outlined button the Slint example
/// uses, so the panel and not the trigger is what the section is about.
///
/// The button carries no action of its own. Opening and both dismissals — Escape
/// and a click outside — belong to the [`Popover`] the menu sits in, exactly as
/// they do for the popover demo's own triggers; the button only draws.
fn trigger<'a>(label: &str) -> Button<'a, Message> {
    Button::new(label.to_string()).outline()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Usage
    // The whole of a menu: the items, the trigger, the caller's flag, and the
    // one message that moves it. `on_chosen` is the contract `select` composes.
    // `filled(true)` is the panel's one surface — `Menu::into_element` hands back
    // the rows and their inset and draws none — and `padding(0.0)` keeps that
    // surface on the body rather than a few pixels outside it.
    let usage = Popover::new(trigger("Open menu"))
        .open(app.open[0])
        .panel_width(PANEL_WIDTH)
        .filled(true)
        .padding(0.0)
        .content(
            Menu::new(menu_items(0))
                .selected(app.chosen[0])
                .highlighted(app.highlighted.0)
                .on_chosen(|index| Message::Chosen { slot: 0, index })
                .on_highlight(move |index| Message::Highlighted { slot: 0, index })
                .into_element(),
        )
        .on_open_change(|open| Message::Opened { slot: 0, open });

    let usage_section = column![
        usage,
        muted_line(
            "`on_chosen` reports the row by index; the caller owns the items and the open flag."
        ),
    ]
    .spacing(8.0);

    // section: Icons
    // Every row keeps the same four columns — a leading icon slot, the label,
    // the shortcut, and a trailing check slot — so a row without an icon still
    // leaves the slot empty rather than shifting its label left.
    let icons = Popover::new(trigger("Open icons"))
        .open(app.open[1])
        .panel_width(PANEL_WIDTH)
        .filled(true)
        .padding(0.0)
        .content(
            Menu::new(menu_items(1))
                .selected(app.chosen[1])
                .highlighted(app.highlighted.0)
                .on_chosen(|index| Message::Chosen { slot: 1, index })
                .on_highlight(move |index| Message::Highlighted { slot: 1, index })
                .into_element(),
        )
        .on_open_change(|open| Message::Opened { slot: 1, open });

    let icons_section = body_box(icons);

    // section: Check state
    // `checked` draws the check mark in the trailing slot; `selected` draws the
    // whole row on the accent surface, which is what a menu of toggles wants.
    let checks = Popover::new(trigger("Open checks"))
        .open(app.open[2])
        .panel_width(PANEL_WIDTH)
        .filled(true)
        .padding(0.0)
        .content(
            Menu::new(menu_items(2))
                .selected(app.chosen[2])
                .highlighted(app.highlighted.0)
                .on_chosen(|index| Message::Chosen { slot: 2, index })
                .on_highlight(move |index| Message::Highlighted { slot: 2, index })
                .into_element(),
        )
        .on_open_change(|open| Message::Opened { slot: 2, open });

    let checks_section = body_box(checks);

    // section: Disabled State
    // A disabled row is never highlighted and never chosen: the arrow keys step
    // over it, and the row is drawn at `muted-foreground`.
    let disabled = Popover::new(trigger("Open disabled"))
        .open(app.open[3])
        .panel_width(PANEL_WIDTH)
        .filled(true)
        .padding(0.0)
        .content(
            Menu::new(menu_items(3))
                .selected(app.chosen[3])
                .highlighted(app.highlighted.0)
                .on_chosen(|index| Message::Chosen { slot: 3, index })
                .on_highlight(move |index| Message::Highlighted { slot: 3, index })
                .into_element(),
        )
        .on_open_change(|open| Message::Opened { slot: 3, open });

    let disabled_section = body_box(disabled);

    // section: Destructive
    // `danger` draws the row in `destructive`, which is the one column GPUI's
    // `PopupMenuItem` has and the Slint `MenuEntry` does not.
    let destructive = Popover::new(trigger("Open destructive"))
        .open(app.open[4])
        .panel_width(PANEL_WIDTH)
        .filled(true)
        .padding(0.0)
        .content(
            Menu::new(menu_items(4))
                .selected(app.chosen[4])
                .highlighted(app.highlighted.0)
                .on_chosen(|index| Message::Chosen { slot: 4, index })
                .on_highlight(move |index| Message::Highlighted { slot: 4, index })
                .into_element(),
        )
        .on_open_change(|open| Message::Opened { slot: 4, open });

    let destructive_section = body_box(destructive);

    // section: Keyboard Shortcuts
    // `↑`/`↓` step to the previous and next actionable row, `Home`/`End` jump to
    // the first and last, and `Enter` and `Space` choose the one highlighted.
    // Escape is the popover's, and closes the panel from anywhere in it.
    let keyboard = Popover::new(trigger("Open with the keyboard"))
        .open(app.open[5])
        .panel_width(PANEL_WIDTH)
        .filled(true)
        .padding(0.0)
        .content(
            Menu::new(menu_items(5))
                .selected(app.chosen[5])
                .highlighted(app.highlighted.0)
                .on_chosen(|index| Message::Chosen { slot: 5, index })
                .on_highlight(move |index| Message::Highlighted { slot: 5, index })
                .into_element(),
        )
        .on_open_change(|open| Message::Opened { slot: 5, open });

    let keyboard_section = body_box(keyboard);

    // section: end
    let status = text(match (app.chosen.iter().enumerate().find_map(|(slot, chosen)| {
        chosen.map(|index| (slot, index))
    }), app.highlighted) {
        (Some((slot, index)), _) => format!(
            "Menu {} chose row {}. A menu stays open on a choice; Escape or a click outside closes it.",
            slot + 1,
            index + 1
        ),
        (None, (Some(index), Some(slot))) => {
            format!("The highlight is on row {} of menu {}.", index + 1, slot + 1)
        }
        _ => {
            "Click a trigger to open a menu; hover a row, or use ↑ ↓ Home End Enter and Escape.".to_string()
        }
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        body_box(usage_section),
        heading(app, 1, SECTIONS[1]),
        icons_section,
        heading(app, 2, SECTIONS[2]),
        checks_section,
        heading(app, 3, SECTIONS[3]),
        disabled_section,
        heading(app, 4, SECTIONS[4]),
        destructive_section,
        heading(app, 5, SECTIONS[5]),
        keyboard_section,
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
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so
// the two galleries draw the same glyphs rather than two lookalikes.

const PENCIL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/><path d="m15 5 4 4"/></svg>"#;

const COPY: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>"#;

const FOLDER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/></svg>"#;

const TRASH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>"#;

const ARCHIVE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="5" x="2" y="3" rx="1"/><path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"/><path d="M10 12h4"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
