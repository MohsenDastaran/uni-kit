//! `select` — the whole gallery the Slint example shows: the field-shaped
//! trigger and its popup of options, the placeholder, the sizes, the report a
//! choice makes, the trigger's own surface, and the disabled state, plus the
//! keyboard the component leaves to its host.
//!
//! Run it with `cargo run --example select`, or in the browser as
//! `?story=select` in the gallery the website embeds.
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
//! The gallery owns every piece of state a select draws from — one open flag, one
//! chosen index and one highlight per select — because the component has none: it
//! reports a request and the application answers with its own value. The keyboard
//! is the same arrangement. iced 0.14 has no focus trap and no focus handle an
//! overlay can take, so the arrows, Home/End, Enter/Space and Escape are bound
//! here and move the highlight the application owns; Escape and a click outside
//! also close the panel through the popover, which owns those two dismissals for
//! every overlay in this crate.
//!
//! The selections start where the Slint example starts them, and the panel is a
//! fixed width rather than the trigger's: Slint's `menu-width: 0` means the
//! trigger's own width, which a `Length::Fill` trigger cannot answer before
//! layout, so the gallery fixes the trigger's width where it wants the two to
//! agree and lets the rest fall back to the menu's 280px. The component's module
//! comment names that gap.

use iced::keyboard;
use iced::widget::{column, container, mouse_area, scrollable, svg, text};
use iced::{Alignment, Element, Length, Subscription, Task, Theme};

use crate::components::group_box::{self, GroupBox};
use crate::components::menu::Item;
use crate::components::select::Select;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example and the component page
/// have them. The index is what the page is asked to scroll to.
const SECTIONS: [&str; 7] = [
    "Framework",
    "Placeholder",
    "Sizes",
    "Events",
    "Custom Appearance",
    "Disabled State",
    "Keyboard Shortcuts",
];

/// Which select each piece of state belongs to. A section that shows more than
/// one select — Sizes, Custom Appearance — still gets one slot per select, so
/// opening one panel does not open its neighbours.
mod slot {
    pub const FRAMEWORK: usize = 0;
    pub const PLACEHOLDER: usize = 1;
    pub const SIZE_SMALL: usize = 2;
    pub const SIZE_MEDIUM: usize = 3;
    pub const SIZE_LARGE: usize = 4;
    pub const EVENTS: usize = 5;
    pub const COUNTRY: usize = 6;
    pub const BUILD: usize = 7;
    pub const PLAIN: usize = 8;
    pub const DISABLED: usize = 9;
    pub const KEYBOARD: usize = 10;
    /// One more than the highest slot: the length of the state arrays.
    pub const COUNT: usize = 11;
}

/// The width the gallery fixes its triggers at. The Slint example stretches a
/// select to its column; the gallery names a width instead so the popup matches
/// the trigger, which is the one case iced can answer — see the module comment.
const TRIGGER_WIDTH: f32 = 240.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — select")
        .theme(theme)
        .subscription(subscription)
        .run()
}

#[derive(Debug)]
struct App {
    /// Whether each select's panel is showing. iced has no select state to own
    /// them, so the application holds the flag and answers every request.
    open: [bool; slot::COUNT],
    /// The chosen row of each select, starting where the Slint example starts
    /// them.
    selected: [Option<usize>; slot::COUNT],
    /// The last row each select reported, so a choice stays visible.
    last: [Option<usize>; slot::COUNT],
    /// The row the pointer or a key highlighted last, and which select holds it.
    /// The highlight is the application's — an element cannot keep it — so the
    /// gallery has one row lit and one select that owns it.
    highlighted: (Option<usize>, Option<usize>),
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl App {
    /// The highlight a select should draw: the row lit only when this select owns
    /// it, so a step taken in one panel does not light a row in another.
    fn highlighted_in(&self, slot: usize) -> Option<usize> {
        if self.highlighted.1 == Some(slot) {
            self.highlighted.0
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    /// A select asked to be opened or closed; the slot says which one.
    Opened {
        slot: usize,
        open: bool,
    },
    /// A select's row was chosen, by slot and by index.
    Chosen {
        slot: usize,
        index: usize,
    },
    /// The highlight moved, to a select's row.
    Highlighted {
        slot: usize,
        index: usize,
    },
    /// A traversal key was pressed: `↑`/`↓`/`Home`/`End`, `Enter` or `Space` to
    /// choose, `Escape` to dismiss.
    Key(Key),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot() -> App {
    let mut selected = [None; slot::COUNT];
    selected[slot::FRAMEWORK] = Some(0);
    selected[slot::SIZE_SMALL] = Some(0);
    selected[slot::SIZE_MEDIUM] = Some(1);
    selected[slot::SIZE_LARGE] = Some(0);
    selected[slot::EVENTS] = Some(0);
    selected[slot::COUNTRY] = Some(1);
    selected[slot::BUILD] = Some(1);
    selected[slot::PLAIN] = Some(0);
    selected[slot::DISABLED] = Some(0);
    selected[slot::KEYBOARD] = Some(0);
    App {
        open: [false; slot::COUNT],
        selected,
        last: [None; slot::COUNT],
        highlighted: (None, None),
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Opened { slot, open } => {
            if let Some(flag) = app.open.get_mut(slot) {
                *flag = open;
                // A closed select owns no highlight: the row it lit belongs to
                // the panel, not to the trigger.
                if !open && app.highlighted.1 == Some(slot) {
                    app.highlighted = (None, None);
                }
            }
        }
        Message::Chosen { slot, index } => {
            if let Some(chosen) = app.selected.get_mut(slot) {
                *chosen = Some(index);
            }
            if let Some(last) = app.last.get_mut(slot) {
                *last = Some(index);
            }
            // A choice closes the panel, which is what the Slint example's row
            // does after it reports the index.
            if let Some(flag) = app.open.get_mut(slot) {
                *flag = false;
            }
            app.highlighted = (Some(index), Some(slot));
        }
        Message::Highlighted { slot, index } => app.highlighted = (Some(index), Some(slot)),
        // The keys act on the select that is open and holds the highlight. This
        // gallery can have one open at a time, and the subscription cannot see
        // which, so `update` reads it from the state.
        Message::Key(key) => {
            if let Some(slot) = app.open.iter().position(|open| *open) {
                let from = app.highlighted_in(slot);
                match key {
                    Key::Choose => {
                        // Enter chooses the highlighted row, or the first
                        // actionable one when the key is the first thing after
                        // opening the select.
                        let row = from.or_else(|| step_to(slot, None, Key::Down));
                        if let Some(index) = row
                            && items(slot).get(index).is_some_and(Item::is_actionable)
                        {
                            app.selected[slot] = Some(index);
                            app.last[slot] = Some(index);
                            app.highlighted = (Some(index), Some(slot));
                            app.open[slot] = false;
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

/// A key the select answers, mapped from the keyboard by the subscription and
/// acted on by `update`, which is where the open select and its highlight are.
///
/// The subscription is `filter_map`ped, and iced requires that closure to be
/// non-capturing, so nothing about the application's state can be read there. It
/// maps the key; `update` decides what the key means.
#[derive(Debug, Clone, Copy)]
enum Key {
    Up,
    Down,
    First,
    Last,
    Choose,
    Dismiss,
}

/// The keyboard the select component leaves to its host.
///
/// `Select` reports the row the pointer entered and draws the row the caller says
/// is highlighted, but iced 0.14 has no focus handle an overlay can take, so the
/// keys are the application's. `↑`/`↓` step, `Home`/`End` jump, `Enter` and
/// `Space` choose the highlighted row and `Escape` closes the panel — the keys
/// the component page documents.
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

/// The options a select holds, in the Slint example's order.
fn items(slot: usize) -> Vec<Item> {
    match slot {
        slot::FRAMEWORK => vec![Item::new("GPUI"), Item::new("Slint"), Item::new("egui")],
        slot::COUNTRY => vec![
            Item::new("Portugal"),
            Item::new("Netherlands"),
            Item::new("New Zealand"),
        ],
        slot::BUILD => vec![Item::new("Debug"), Item::new("Release")],
        _ => vec![Item::new("GPUI"), Item::new("Slint")],
    }
}

/// The label a report refers to, so a status line can name the choice rather
/// than its index.
fn label_of(slot: usize, index: Option<usize>) -> Option<String> {
    index.and_then(|index| items(slot).get(index).map(|item| item.label().to_string()))
}

/// The row a step lands on, skipping separators and disabled rows and stopping at
/// the ends rather than wrapping — the same stepping rule the menu gallery writes
/// where its keys are handled.
fn step_to(slot: usize, from: Option<usize>, step: Key) -> Option<usize> {
    let items = items(slot);
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
/// is the same 28px box the menu gallery draws.
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

/// One select in the gallery, wired to the slot its open flag, chosen index and
/// highlight live in.
///
/// The section samples below spell the builder out rather than calling this, so
/// the code the component page shows is the code a reader would write. This is
/// the body those samples share: a `Select` over `items(slot)`, its three pieces
/// of state read back, and the three reports the application answers.
fn select<'a>(app: &App, slot: usize) -> Select<'a, Message> {
    Select::new(items(slot))
        .selected(app.selected[slot])
        .open(app.open[slot])
        .highlighted(app.highlighted_in(slot))
        .width(Length::Fixed(TRIGGER_WIDTH))
        .on_open_change(move |open| Message::Opened { slot, open })
        .on_chosen(move |index| Message::Chosen { slot, index })
        .on_highlight(move |index| Message::Highlighted { slot, index })
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Framework
    // The whole of a select: the items, the caller's chosen index and open flag,
    // and the two reports. The trigger is the field's frame — the same height,
    // inset and edge as `Input` — and the panel is the popover's one surface
    // around the menu's rows. `Item` is the shape the menu and the dropdown
    // button already share, so an option can carry an icon or a shortcut.
    let framework = body_box(
        Select::new(vec![
            Item::new("GPUI"),
            Item::new("Slint"),
            Item::new("egui"),
        ])
        .selected(app.selected[slot::FRAMEWORK])
        .open(app.open[slot::FRAMEWORK])
        .highlighted(app.highlighted_in(slot::FRAMEWORK))
        .placeholder("Choose a framework")
        .width(Length::Fixed(TRIGGER_WIDTH))
        .on_open_change(|open| Message::Opened {
            slot: slot::FRAMEWORK,
            open,
        })
        .on_chosen(|index| Message::Chosen {
            slot: slot::FRAMEWORK,
            index,
        })
        .on_highlight(|index| Message::Highlighted {
            slot: slot::FRAMEWORK,
            index,
        }),
    );

    // section: Placeholder
    // Nothing is selected, so the trigger shows the placeholder; the second
    // select is disabled, which is the placeholder card the Slint example shows
    // beside the first.
    let placeholder = body_box(
        column![
            select(app, slot::PLACEHOLDER).placeholder("Choose a framework"),
            Select::new(items(slot::PLACEHOLDER))
                .placeholder("No region matches this filter")
                .width(Length::Fixed(TRIGGER_WIDTH))
                .disabled(true),
        ]
        .spacing(12.0),
    );

    // section: Sizes
    // The size picks the height, the inset, the type and the chevron together,
    // the way the Slint component's `size` picks `Theme.input-height`,
    // `input-padding-x`, `input-font-size` and `icon-size`.
    let sizes = body_box(
        column![
            select(app, slot::SIZE_SMALL)
                .size(theme::ControlSize::Small)
                .placeholder("Small"),
            select(app, slot::SIZE_MEDIUM).placeholder("Medium"),
            select(app, slot::SIZE_LARGE)
                .size(theme::ControlSize::Large)
                .placeholder("Large"),
        ]
        .spacing(12.0),
    );

    // section: Events
    // `on_chosen` reports the row by index; the caller owns the items, asks
    // itself what index 2 means, and closes the panel by answering `open`. The
    // line under the select is the report made visible.
    let events = body_box(
        column![
            select(app, slot::EVENTS).placeholder("Choose a framework"),
            muted_line(format!(
                "Last choice: {}",
                label_of(slot::EVENTS, app.last[slot::EVENTS])
                    .unwrap_or_else(|| "nothing yet".to_string())
            )),
        ]
        .spacing(8.0),
    );

    // section: Custom Appearance
    // The trigger's own surface: a tinted fill, a matching edge and a softer
    // radius, a wider panel than the trigger, and a prefix on the value. The last
    // one asks for `appearance: false`, for a host that supplies the surface.
    let custom = body_box(
        column![
            select(app, slot::COUNTRY)
                .title_prefix("Country: ")
                .width(Length::Fixed(220.0))
                .menu_width(360.0)
                .background(|theme| Tokens::of(theme).primary.scale_alpha(0.08))
                .border_color(|theme| Tokens::of(theme).primary.scale_alpha(0.45))
                .radius(theme::RADIUS_XL),
            select(app, slot::BUILD)
                .title_prefix("Build: ")
                .background(|theme| Tokens::of(theme).muted)
                .border_color(|theme| Tokens::of(theme).border)
                .radius(theme::RADIUS_SM),
            select(app, slot::PLAIN)
                .placeholder("Choose a framework")
                .appearance(false),
        ]
        .spacing(12.0),
    );

    // section: Disabled State
    // A disabled select answers nothing: the trigger is drawn muted and the
    // popover is given no open state, so the panel cannot be opened at all.
    let disabled = body_box(
        Select::new(items(slot::DISABLED))
            .selected(app.selected[slot::DISABLED])
            .placeholder("Choose a framework")
            .width(Length::Fixed(TRIGGER_WIDTH))
            .disabled(true),
    );

    // section: Keyboard Shortcuts
    // `↑`/`↓` step to the previous and next actionable row, `Home`/`End` jump to
    // the first and last, and `Enter` and `Space` choose the one highlighted.
    // Escape is the popover's, and closes the panel from anywhere in it.
    let keyboard = body_box(
        column![
            select(app, slot::KEYBOARD).placeholder("Open with the keyboard"),
            muted_line("↑ ↓ Home End move the highlight; Enter or Space chooses; Escape closes."),
        ]
        .spacing(8.0),
    );

    // section: end
    let status = text(match (
        app.last
            .iter()
            .enumerate()
            .find_map(|(slot, last)| last.map(|index| (slot, index))),
        app.highlighted,
    ) {
        (Some((slot, index)), _) => format!(
            "Select {} chose row {}. A choice closes the panel; Escape or a click outside closes one without choosing.",
            slot + 1,
            index + 1
        ),
        (None, (Some(index), Some(slot))) => {
            format!("The highlight is on row {} of select {}.", index + 1, slot + 1)
        }
        _ => "Click a trigger to open its options; hover a row, or use ↑ ↓ Home End Enter and Escape."
            .to_string(),
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        framework,
        heading(app, 1, SECTIONS[1]),
        placeholder,
        heading(app, 2, SECTIONS[2]),
        sizes,
        heading(app, 3, SECTIONS[3]),
        events,
        heading(app, 4, SECTIONS[4]),
        custom,
        heading(app, 5, SECTIONS[5]),
        disabled,
        heading(app, 6, SECTIONS[6]),
        keyboard,
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

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
