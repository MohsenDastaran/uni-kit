//! `combobox` — the whole gallery the Slint example shows, in the sections the
//! component page has: a select-like trigger and its popup, with a search row
//! that filters the list as it is typed, one or many choices, disabled rows, a
//! footer that creates the item that was typed, sizes, the maximum the multiple
//! choice takes, the disabled state, the report a choice makes and the keyboard
//! the component leaves to its host.
//!
//! Run it with `cargo run --example combobox`, or in the browser as
//! `?story=combobox` in the gallery the website embeds.
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
//! The gallery owns every piece of state a combobox draws from — one open flag,
//! one chosen index, one picked set, the free text and one highlight per
//! combobox for each of the pointer's hover and the row a key moved to — because
//! the component has none: it reports a request and the application answers with
//! its own value. The two highlights are kept apart, as
//! `Combobox::on_highlight` requires, so the pointer's leave restores the key's
//! row rather than clearing it.
//!
//! The gallery follows the Slint component on the two decisions the component
//! deliberately leaves to its caller: a single choice clears the query and closes
//! the panel, and a multiple choice toggles the row and leaves both alone. A
//! dismissal — Escape or a click outside — keeps the query in either mode, which
//! is what the reference does and what makes the typed text stay in the field
//! when the panel comes back.
//!
//! The keyboard is the same arrangement as the select's. iced 0.14 has no focus
//! trap and no focus handle an overlay can take, so the arrows, Home/End,
//! Enter/Space and Escape are bound here and move the highlight the application
//! owns; Escape and a click outside also close the panel through the popover,
//! which owns those two dismissals for every overlay in this crate. Because iced
//! reports no focus to the application, the traversal keys act on the combobox
//! whose panel is open; the component page's "Up/Down opens the menu" is the one
//! line this gallery cannot honour, and the module comment of the component says
//! why.
//!
//! The sections are the Slint example's that this port can show. Groups, badges,
//! pinned rows, a custom check glyph and the chip/badges/overflow/count trigger
//! variants are named in the component's module comment rather than faked here,
//! because the menu row this composition draws has no column for them.

use iced::keyboard;
use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Subscription, Task, Theme};

use crate::components::combobox::Combobox;
use crate::components::group_box::{self, GroupBox};
use crate::components::menu::Item;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the component page has them. The index is
/// what the page is asked to scroll to.
const SECTIONS: [&str; 11] = [
    "Basic Single-Select",
    "Multi-Select",
    "Disabled Items",
    "Icons",
    "Footer Action",
    "Maximum selections",
    "Sizes",
    "Disabled",
    "Events",
    "Values",
    "Keyboard Shortcuts",
];

/// Which combobox each piece of state belongs to. A section that shows more than
/// one — Sizes — still gets one slot per combobox, so opening one panel does not
/// open its neighbours.
mod slot {
    pub const BASIC: usize = 0;
    pub const MULTI: usize = 1;
    pub const DISABLED_ITEMS: usize = 2;
    pub const ICONS: usize = 3;
    pub const FOOTER: usize = 4;
    pub const MAXIMUM: usize = 5;
    pub const SIZE_SMALL: usize = 6;
    pub const SIZE_MEDIUM: usize = 7;
    pub const SIZE_LARGE: usize = 8;
    pub const DISABLED: usize = 9;
    pub const EVENTS: usize = 10;
    pub const KEYBOARD: usize = 11;
    /// One more than the highest slot: the length of the state arrays.
    pub const COUNT: usize = 12;
}

/// The width the gallery fixes its triggers at, so the popup matches the trigger
/// — the one case iced can answer before layout.
const TRIGGER_WIDTH: f32 = 280.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — combobox")
        .theme(theme)
        .subscription(subscription)
        .run()
}

#[derive(Debug)]
struct App {
    /// Whether each combobox's panel is showing. iced has no combobox state to
    /// own it, so the application holds the flag and answers every request.
    open: [bool; slot::COUNT],
    /// The single choice of each combobox. Ignored in multiple-select.
    selected: [Option<usize>; slot::COUNT],
    /// The multiple choice of each combobox, as indices into its items.
    picked: [Vec<usize>; slot::COUNT],
    /// The free text each combobox's search field holds.
    query: [String; slot::COUNT],
    /// The last row each combobox reported, so a choice stays visible.
    last: [Option<usize>; slot::COUNT],
    /// The footer example's own list: the footer action appends the item that
    /// was typed, which is the one place the gallery's data changes.
    schools: Vec<Item>,
    /// The row a key moved to, and which combobox holds it. The keyboard's, and
    /// it is not touched by the pointer's reports.
    highlighted: (Option<usize>, Option<usize>),
    /// The row the pointer is over, and which combobox holds it. This is the
    /// transient hover `Combobox::on_highlight` reports and `Combobox::on_leave`
    /// clears; it is kept apart from `highlighted` so a key's row survives the
    /// pointer crossing the panel.
    hover: (Option<usize>, Option<usize>),
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl App {
    /// The highlight a combobox should draw: the pointer's hover when it is over
    /// this combobox, otherwise the row a key moved to in it. The pointer wins
    /// while it is there; the key's row comes back once it leaves.
    fn highlighted_in(&self, slot: usize) -> Option<usize> {
        if self.hover.1 == Some(slot) {
            self.hover.0
        } else if self.highlighted.1 == Some(slot) {
            self.highlighted.0
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    /// A combobox asked to be opened or closed; the slot says which one.
    Opened {
        slot: usize,
        open: bool,
    },
    /// A combobox's row was chosen, by slot and by index.
    Chosen {
        slot: usize,
        index: usize,
    },
    /// A combobox's free text changed, by slot and by the value the field holds.
    Queried {
        slot: usize,
        query: String,
    },
    /// The footer action was pressed: add the typed item to the list.
    Created {
        slot: usize,
    },
    /// The highlight moved, to a combobox's row. This is the pointer's transient
    /// hover; the keyboard sets `highlighted` instead.
    Highlighted {
        slot: usize,
        index: usize,
    },
    /// The pointer left a combobox's row, or left its panel. `row` is the row it
    /// left, or `None` for the panel as a whole.
    Left {
        slot: usize,
        row: Option<usize>,
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
    selected[slot::FOOTER] = Some(0);
    selected[slot::SIZE_SMALL] = Some(0);
    selected[slot::SIZE_MEDIUM] = Some(1);
    selected[slot::SIZE_LARGE] = Some(2);
    let mut picked: [Vec<usize>; slot::COUNT] = std::array::from_fn(|_| Vec::new());
    picked[slot::MULTI] = vec![0, 2];
    picked[slot::MAXIMUM] = vec![0];
    App {
        open: [false; slot::COUNT],
        selected,
        picked,
        query: std::array::from_fn(|_| String::new()),
        last: [None; slot::COUNT],
        schools: vec![
            Item::new("Harvard University"),
            Item::new("MIT"),
            Item::new("Stanford"),
            Item::new("Cambridge"),
        ],
        highlighted: (None, None),
        hover: (None, None),
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Opened { slot, open } => {
            if let Some(flag) = app.open.get_mut(slot) {
                *flag = open;
                // A closed combobox owns no highlight: the row it lit belongs to
                // the panel, not to the trigger.
                if !open {
                    if app.highlighted.1 == Some(slot) {
                        app.highlighted = (None, None);
                    }
                    if app.hover.1 == Some(slot) {
                        app.hover = (None, None);
                    }
                }
            }
        }
        Message::Chosen { slot, index } => choose(app, slot, index),
        Message::Queried { slot, query } => {
            if let Some(text) = app.query.get_mut(slot) {
                *text = query;
            }
        }
        Message::Created { slot } => {
            // The typed name becomes a row. The Slint footer does the same and
            // leaves the panel open so the new row is visible; it does not change
            // the choice.
            let label = app.query[slot].trim().to_string();
            if !label.is_empty() {
                if slot == slot::FOOTER {
                    app.schools.push(Item::new(label));
                    app.last[slot] = Some(app.schools.len() - 1);
                }
                app.query[slot].clear();
            }
        }
        Message::Highlighted { slot, index } => app.hover = (Some(index), Some(slot)),
        // The row travels with the report: iced walks the rows top-down, so an
        // upper row's entry is published before a lower row's leave. Only the
        // row the pointer was on may be cleared, or entering the upper row is
        // undone on the same move. `None` is the panel, and clears the hover
        // whatever row it sat on.
        Message::Left { slot, row } => {
            if app.hover.1 == Some(slot) && (row.is_none() || app.hover.0 == row) {
                app.hover = (None, None);
            }
        }
        // The keys act on the combobox that is open and holds the highlight. This
        // gallery can have one open at a time, and the subscription cannot see
        // which, so `update` reads it from the state.
        Message::Key(key) => {
            if let Some(slot) = app.open.iter().position(|open| *open) {
                // A key takes the highlight over from the pointer, as it does in
                // the menu gallery.
                app.hover = (None, None);
                let from = app.highlighted_in(slot);
                match key {
                    Key::Choose => {
                        // Enter chooses the highlighted row, or the first
                        // actionable one when the key is the first thing after
                        // opening the combobox.
                        if let Some(index) = from.or_else(|| step_to(app, slot, None, Key::Down))
                            && !blocked(app, slot, index)
                        {
                            choose(app, slot, index);
                        }
                    }
                    Key::Dismiss => {
                        app.open[slot] = false;
                        app.highlighted = (None, None);
                    }
                    step => app.highlighted = (step_to(app, slot, from, step), Some(slot)),
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

/// Answer a chosen row: set the single choice and close, or toggle the multiple
/// one and stay open — the Slint component's own `toggle`.
fn choose(app: &mut App, slot: usize, index: usize) {
    app.last[slot] = Some(index);
    if multiple(slot) {
        if let Some(position) = app.picked[slot].iter().position(|picked| *picked == index) {
            app.picked[slot].remove(position);
        } else {
            let limit = max_selected(slot);
            if limit == 0 || app.picked[slot].len() < limit {
                app.picked[slot].push(index);
            }
        }
    } else {
        app.selected[slot] = Some(index);
        // A single choice clears the query and closes the panel, which is what
        // the Slint `toggle` does.
        app.query[slot].clear();
        app.open[slot] = false;
    }
    app.highlighted = (Some(index), Some(slot));
    app.hover = (None, None);
}

/// A key the combobox answers, mapped from the keyboard by the subscription and
/// acted on by `update`, which is where the open panel and its highlight are.
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

/// The keyboard the combobox component leaves to its host.
///
/// `Combobox` reports the row the pointer entered and draws the row the caller
/// says is highlighted, but iced 0.14 has no focus handle an overlay can take, so
/// the keys are the application's. `↑`/`↓` step, `Home`/`End` jump, `Enter` and
/// `Space` choose the highlighted row and `Escape` closes the panel — the keys
/// the component page documents. Because the subscription cannot see which
/// combobox has focus, the keys answer the open panel; see the module comment.
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

/// Whether a combobox toggles a set instead of setting one choice.
fn multiple(slot: usize) -> bool {
    matches!(slot, slot::MULTI | slot::MAXIMUM)
}

/// The most rows a multiple combobox may hold, `0` for no limit.
fn max_selected(slot: usize) -> usize {
    if slot == slot::MAXIMUM { 2 } else { 0 }
}

/// The options a combobox holds, in the Slint example's order.
fn items(slot: usize) -> Vec<Item> {
    match slot {
        slot::MULTI | slot::MAXIMUM => vec![
            Item::new("React"),
            Item::new("Nextjs"),
            Item::new("Angular"),
            Item::new("VueJS"),
            Item::new("Django"),
            Item::new("Astro"),
            Item::new("Remix"),
            Item::new("Svelte"),
            Item::new("SolidJS"),
            Item::new("Qwik"),
        ],
        slot::DISABLED_ITEMS => vec![
            Item::new("Apples"),
            Item::new("Bananas").disabled(true),
            Item::new("Cherries"),
            Item::new("Carrots"),
            Item::new("Broccoli").disabled(true),
        ],
        slot::ICONS => vec![
            Item::new("Information Technology").icon(CPU),
            Item::new("Healthcare").icon(HEART),
            Item::new("Finance").icon(GLOBE),
            Item::new("Education").icon(BOOK_OPEN),
            Item::new("Entertainment").icon(STAR),
        ],
        _ => vec![
            Item::new("Next.js"),
            Item::new("SvelteKit"),
            Item::new("Nuxt.js"),
            Item::new("Remix"),
            Item::new("Astro"),
        ],
    }
}

/// The items a combobox draws, including the footer example's own growing list.
fn items_of(app: &App, slot: usize) -> Vec<Item> {
    if slot == slot::FOOTER {
        app.schools.clone()
    } else {
        items(slot)
    }
}

/// The rows the query leaves visible, in the caller's indices. The component
/// filters the same way; the gallery repeats the reading so its keyboard steps
/// over exactly the rows the panel draws.
fn visible_indices(app: &App, slot: usize) -> Vec<usize> {
    let query = app.query[slot].to_lowercase();
    items_of(app, slot)
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            !item.is_separator()
                && (query.is_empty() || item.label().to_lowercase().contains(&query))
        })
        .map(|(index, _)| index)
        .collect()
}

/// Whether a row answers nothing: a separator, a disabled item, or a row the
/// maximum has put out of reach. The component draws the same three.
fn blocked(app: &App, slot: usize, index: usize) -> bool {
    let at_capacity = multiple(slot)
        && max_selected(slot) > 0
        && app.picked[slot].len() >= max_selected(slot)
        && !app.picked[slot].contains(&index);
    if at_capacity {
        return true;
    }
    items_of(app, slot)
        .get(index)
        .is_none_or(|item| item.is_separator() || !item.is_actionable())
}

/// The row a step lands on, over the rows the query leaves visible, skipping the
/// ones that answer nothing and stopping at the ends rather than wrapping — the
/// same stepping rule the selected gallery writes where its keys are handled.
fn step_to(app: &App, slot: usize, from: Option<usize>, step: Key) -> Option<usize> {
    let rows: Vec<usize> = visible_indices(app, slot)
        .into_iter()
        .filter(|index| !blocked(app, slot, *index))
        .collect();
    match step {
        Key::First => rows.first().copied(),
        Key::Last => rows.last().copied(),
        Key::Down | Key::Up => {
            let delta: isize = if matches!(step, Key::Down) { 1 } else { -1 };
            match from.and_then(|index| rows.iter().position(|row| *row == index)) {
                Some(position) => {
                    let next = position as isize + delta;
                    if next < 0 || next >= rows.len() as isize {
                        Some(rows[position])
                    } else {
                        Some(rows[next as usize])
                    }
                }
                None => {
                    if delta > 0 {
                        rows.first().copied()
                    } else {
                        rows.last().copied()
                    }
                }
            }
        }
        // `Choose` and `Dismiss` are not moves; `update` answers them itself.
        Key::Choose | Key::Dismiss => from,
    }
}

/// The label a report refers to, so a status line can name the choice rather
/// than its index.
fn label_of(app: &App, slot: usize, index: Option<usize>) -> Option<String> {
    let items = items_of(app, slot);
    index
        .and_then(|index| items.get(index))
        .map(|item| item.label().to_string())
}

/// The label the trigger itself shows: the joined multiple choice, the single
/// choice, or nothing while the placeholder stands in.
fn current_label(app: &App, slot: usize) -> Option<String> {
    let items = items_of(app, slot);
    if multiple(slot) {
        let labels: Vec<String> = app.picked[slot]
            .iter()
            .filter_map(|index| items.get(*index))
            .map(|item| item.label().to_string())
            .collect();
        (!labels.is_empty()).then(|| labels.join(", "))
    } else {
        app.selected[slot]
            .and_then(|index| items.get(index))
            .map(|item| item.label().to_string())
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
/// is the same 28px box the select gallery draws.
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

/// One combobox in the gallery, wired to the slot its open flag, choice, free
/// text and highlight live in.
///
/// The section samples below spell the builder out rather than calling this, so
/// the code the component page shows is the code a reader would write. This is
/// the body those samples share: a `Combobox` over the slot's items, its state
/// read back, and the five reports the application answers.
fn combo<'a>(app: &App, slot: usize) -> Combobox<'a, Message> {
    Combobox::new(items_of(app, slot))
        .selected(app.selected[slot])
        .picked(app.picked[slot].clone())
        .multiple(multiple(slot))
        .max_selected(max_selected(slot))
        .query(app.query[slot].clone())
        .open(app.open[slot])
        .highlighted(app.highlighted_in(slot))
        .width(Length::Fixed(TRIGGER_WIDTH))
        .on_open_change(move |open| Message::Opened { slot, open })
        .on_chosen(move |index| Message::Chosen { slot, index })
        .on_query(move |query| Message::Queried { slot, query })
        .on_highlight(move |index| Message::Highlighted { slot, index })
        .on_leave(move |row| Message::Left { slot, row })
}

/// The footer's content: the Slint footer's own plus glyph and label, in a band
/// the caller owns and the band's click adds the typed item.
fn footer_action<'a>(label: &str) -> Element<'a, Message> {
    mouse_area(
        container(
            row![
                svg(svg::Handle::from_memory(PLUS))
                    .width(14.0)
                    .height(14.0)
                    .style(|theme: &Theme, _status| svg::Style {
                        color: Some(Tokens::of(theme).foreground),
                    }),
                text(label.to_string())
                    .size(theme::FONT_SIZE_SM)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).foreground),
                    }),
            ]
            .spacing(theme::SPACING_SM)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .padding(theme::SPACING_SM),
    )
    .on_press(Message::Created { slot: slot::FOOTER })
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic Single-Select
    // The whole of a combobox: the items, the caller's single choice, open flag
    // and free text, and the five reports. The trigger is the field's frame — the
    // same height, inset and edge as `Input` — and the panel is the popover's one
    // surface around a search row and the menu's rows. `Item` is the shape the
    // menu, the dropdown button and the select already share.
    let basic = body_box(
        Combobox::new(items(slot::BASIC))
            .selected(app.selected[slot::BASIC])
            .query(app.query[slot::BASIC].clone())
            .open(app.open[slot::BASIC])
            .highlighted(app.highlighted_in(slot::BASIC))
            .placeholder("Select framework...")
            .search_placeholder("Search…")
            .width(Length::Fixed(TRIGGER_WIDTH))
            .on_open_change(|open| Message::Opened {
                slot: slot::BASIC,
                open,
            })
            .on_chosen(|index| Message::Chosen {
                slot: slot::BASIC,
                index,
            })
            .on_query(|query| Message::Queried {
                slot: slot::BASIC,
                query,
            })
            .on_highlight(|index| Message::Highlighted {
                slot: slot::BASIC,
                index,
            })
            .on_leave(|row| Message::Left {
                slot: slot::BASIC,
                row,
            }),
    );

    // section: Multi-Select
    // `multiple(true)` makes a click toggle the row into `picked` instead of
    // setting `selected`; the chosen rows carry the menu's check mark and the
    // panel stays open until Escape or a click outside closes it.
    let multiple = body_box(
        combo(app, slot::MULTI)
            .multiple(true)
            .placeholder("Select frameworks..."),
    );

    // section: Disabled Items
    // A row built with `disabled(true)` is drawn muted and answers no pointer or
    // key, which is the Slint `dimmed` state.
    let disabled_items = body_box(combo(app, slot::DISABLED_ITEMS).placeholder("Select item..."));

    // section: Icons
    // An `Item` carries the SVG the row draws in its leading slot; the bytes are
    // the application's, as every icon in this crate is.
    let icons = body_box(combo(app, slot::ICONS).placeholder("Select industry category"));

    // section: Footer Action
    // The footer band is the caller's element: here the plus glyph and a label,
    // wrapped in the click that appends the typed name to the list. The component
    // draws the band's one-pixel top edge and nothing else.
    let footer = body_box(
        column![
            combo(app, slot::FOOTER)
                .placeholder("Select university")
                .footer(footer_action("New university")),
            muted_line("Type a name, then choose New university to add it to the list."),
        ]
        .spacing(8.0),
    );

    // section: Maximum selections
    // `max_selected(2)` draws the rows that would exceed it muted and stops the
    // click, the Slint `at-capacity` block.
    let maximum = body_box(combo(app, slot::MAXIMUM).placeholder("Select up to 2 frameworks"));

    // section: Sizes
    // The size picks the height, the inset, the type and the chevron together,
    // the way the Slint component's `size` picks `Theme.input-height`,
    // `input-padding-x`, `input-font-size` and `icon-size`.
    let sizes = body_box(
        column![
            combo(app, slot::SIZE_SMALL)
                .size(theme::ControlSize::Small)
                .placeholder("Small"),
            combo(app, slot::SIZE_MEDIUM).placeholder("Medium"),
            combo(app, slot::SIZE_LARGE)
                .size(theme::ControlSize::Large)
                .placeholder("Large"),
        ]
        .spacing(12.0),
    );

    // section: Disabled
    // A disabled combobox answers nothing: the trigger is drawn muted and the
    // popover is given no open state, so the panel cannot be opened at all.
    let disabled = body_box(
        combo(app, slot::DISABLED)
            .placeholder("Select framework...")
            .disabled(true),
    );

    // section: Events
    // `on_chosen` reports the row by index; the caller owns the items, asks
    // itself what index 2 means, and decides whether to close by answering
    // `open`. The line under the combobox is the report made visible.
    let events = body_box(
        column![
            combo(app, slot::EVENTS).placeholder("Select framework..."),
            muted_line(format!(
                "Last choice: {}",
                label_of(app, slot::EVENTS, app.last[slot::EVENTS])
                    .unwrap_or_else(|| "nothing yet".to_string())
            )),
        ]
        .spacing(8.0),
    );

    // section: Values
    // The choice read back the way the trigger reads it: the single label, or
    // the multiple one joined with commas — the Slint `current-label()`.
    let values = body_box(
        column![
            muted_line(format!(
                "basic: {}",
                current_label(app, slot::BASIC).unwrap_or_else(|| "none".to_string())
            )),
            muted_line(format!(
                "multiple: {}",
                current_label(app, slot::MULTI).unwrap_or_else(|| "none".to_string())
            )),
            muted_line(format!(
                "maximum: {}",
                current_label(app, slot::MAXIMUM).unwrap_or_else(|| "none".to_string())
            )),
        ]
        .spacing(4.0),
    );

    // section: Keyboard Shortcuts
    // `↑`/`↓` step to the previous and next actionable row the query leaves
    // visible, `Home`/`End` jump to the first and last, and `Enter` and `Space`
    // choose the one highlighted. Escape is the popover's, and closes the panel
    // from anywhere in it.
    let keyboard = body_box(
        column![
            combo(app, slot::KEYBOARD).placeholder("Open with the keyboard"),
            muted_line("↑ ↓ Home End move the highlight; Enter or Space chooses; Escape closes."),
        ]
        .spacing(8.0),
    );

    // section: end
    // The status names whichever highlight is live: the pointer's hover while it
    // is over a row, otherwise the row a key moved to.
    let lit = if app.hover.0.is_some() {
        app.hover
    } else {
        app.highlighted
    };
    let status = text(match (
        app.last
            .iter()
            .enumerate()
            .find_map(|(slot, last)| last.map(|index| (slot, index))),
        lit,
    ) {
        (Some((slot, index)), _) => format!(
            "Combobox {} chose {}. A single choice closes the panel and clears the query; Escape or a click outside closes one and keeps it.",
            slot + 1,
            label_of(app, slot, Some(index)).unwrap_or_default()
        ),
        (None, (Some(index), Some(slot))) => {
            format!("The highlight is on row {} of combobox {}.", index + 1, slot + 1)
        }
        _ => "Click a trigger to open its options; type to filter, or use ↑ ↓ Home End Enter and Escape.".to_string(),
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        basic,
        heading(app, 1, SECTIONS[1]),
        multiple,
        heading(app, 2, SECTIONS[2]),
        disabled_items,
        heading(app, 3, SECTIONS[3]),
        icons,
        heading(app, 4, SECTIONS[4]),
        footer,
        heading(app, 5, SECTIONS[5]),
        maximum,
        heading(app, 6, SECTIONS[6]),
        sizes,
        heading(app, 7, SECTIONS[7]),
        disabled,
        heading(app, 8, SECTIONS[8]),
        events,
        heading(app, 9, SECTIONS[9]),
        values,
        heading(app, 10, SECTIONS[10]),
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

/// The plus glyph the footer band draws, from
/// `crates/slint-component/ui/icons/plus.svg` (Lucide, ISC), the same file the
/// Slint footer names.
const PLUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>"#;

const CPU: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20v2"/><path d="M12 2v2"/><path d="M17 20v2"/><path d="M17 2v2"/><path d="M2 12h2"/><path d="M2 17h2"/><path d="M2 7h2"/><path d="M20 12h2"/><path d="M20 17h2"/><path d="M20 7h2"/><path d="M7 20v2"/><path d="M7 2v2"/><rect x="4" y="4" width="16" height="16" rx="2"/><rect x="8" y="8" width="8" height="8" rx="1"/></svg>"#;

const HEART: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5"/></svg>"#;

const GLOBE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>"#;

const BOOK_OPEN: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v16"/><path d="M20.001 19A2 2 0 0022 17V5a2 2 0 00-1.999-2L16 3.002A5 5 0 0012 5a5 5 0 00-4-2H4a2 2 0 00-2 2v12a2 2 0 001.999 2H8a5 5 0 014 2 5 5 0 014-2z"/></svg>"#;

const STAR: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/></svg>"#;
