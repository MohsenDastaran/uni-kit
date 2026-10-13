//! `command` — the whole gallery the Slint example shows, in the sections the
//! component page has: the palette inline, without its search field, inside a
//! dialog, inside a scroll area, with variable-height rows and with the rich
//! stock rows, and the line that records the last choice.
//!
//! Run it with `cargo run --example command`, or in the browser as
//! `?story=command` in the gallery the website embeds.
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
//! The section titles are the page's own headings. The Slint example spells two
//! of them differently — "Dialog" and "Quick actions" — and the page calls them
//! "In a Dialog" and "Quick Actions Without Search", which is what a panel has to
//! be titled to land under the heading that already exists.
//!
//! The gallery owns every piece of state a palette draws from — one query and one
//! lit row per palette, one hover per palette kept apart from the row a key moved
//! to, which panel is open and what was last confirmed — because the component
//! has none: it reports a request and the application answers with its own value.
//!
//! The keyboard is the application's, as [`Command`]'s module comment says. iced
//! has no focus handle a palette can take, so the traversal keys act on the panel
//! that is open in a dialog; the inline palettes are driven by the pointer and
//! their search fields take the keys a text field needs. `Escape` closes the open
//! panel, which is the one dismissal iced's dialog cannot take for itself.
//!
//! The marks the hints band spells are the ones the wasm face can draw. `⌘` is
//! not in the embedded Fira Sans, so a shortcut that would paint a bare letter is
//! written `Cmd P` rather than `⌘P`; `⌫` is present and stays.

use iced::keyboard;
use iced::widget::{Id, Stack, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Border, Element, Length, Subscription, Task, Theme};

use crate::components::button::Button;
use crate::components::command::{self, Command, CommandItem};
use crate::components::dialog::Dialog;
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The dialog cards' height, the Slint example's `height: 420px` / `460px`.
const MENU_CARD: f32 = 420.0;
const STOCK_CARD: f32 = 460.0;

/// The section headings, in the order the component page has them. The index is
/// what the page is asked to scroll to.
const SECTIONS: [&str; 7] = [
    "Inline",
    "Quick Actions Without Search",
    "In a Dialog",
    "Scrollable",
    "Variable-height rows",
    "Search panel",
    "Last confirmed",
];

/// Which palette each piece of state belongs to. Two dialog sections hold their
/// own palette, and a section that shows more than one still gets one slot each,
/// so opening one panel does not open its neighbours.
mod slot {
    pub const INLINE: usize = 0;
    pub const QUICK: usize = 1;
    pub const DIALOG: usize = 2;
    pub const SCROLL: usize = 3;
    pub const ROWS: usize = 4;
    pub const SEARCH: usize = 5;
    /// One more than the highest slot: the length of the state arrays.
    pub const COUNT: usize = 6;
}

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — command")
        .theme(theme)
        .subscription(subscription)
        .run()
}

#[derive(Debug)]
struct App {
    /// The free text each palette's search field holds.
    query: [String; slot::COUNT],
    /// The row each palette draws on the accent surface when the pointer is not
    /// over one — the row a key moved to.
    active: [Option<usize>; slot::COUNT],
    /// The row the pointer is over, per palette. Kept apart from `active` so a
    /// leave clears the hover without taking the key's row with it.
    hover: [Option<usize>; slot::COUNT],
    /// The line the "Last confirmed" section shows, the Slint `last`.
    last: String,
    /// Whether each dialog section's panel is open.
    menu_open: bool,
    stocks_open: bool,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

impl App {
    /// The row a palette should draw lit: the pointer's hover while it is over
    /// one, otherwise the row a key moved to.
    fn active_in(&self, slot: usize) -> Option<usize> {
        self.hover[slot].or(self.active[slot])
    }
}

#[derive(Debug, Clone)]
enum Message {
    /// A palette's free text changed, by slot and by the value the field holds.
    Queried {
        slot: usize,
        query: String,
    },
    /// A palette's row was chosen, by slot and by index into its items.
    Chosen {
        slot: usize,
        index: usize,
    },
    /// The highlight moved to a palette's row. This is the pointer's transient
    /// hover; the keyboard sets `active` instead.
    Highlighted {
        slot: usize,
        index: usize,
    },
    /// The pointer left a palette's row, or left its list. `row` is the row it
    /// left, or `None` for the list as a whole.
    Left {
        slot: usize,
        row: Option<usize>,
    },
    /// A traversal key was pressed: `↑`/`↓`/`Home`/`End` move the highlight,
    /// `Enter` chooses, `Escape` dismisses.
    Key(Key),
    /// A dialog section was opened or dismissed.
    MenuOpen(bool),
    StocksOpen(bool),
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
        query: std::array::from_fn(|_| String::new()),
        active: std::array::from_fn(|_| Some(0)),
        hover: [None; slot::COUNT],
        last: String::from("none"),
        menu_open: false,
        stocks_open: false,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        // A query change hands the highlight back to the first row it leaves
        // actionable, which is the Slint `changed query`. The component filters
        // from the value the caller passes back, so the two agree.
        Message::Queried { slot, query } => {
            if let Some(text) = app.query.get_mut(slot) {
                *text = query;
            }
            app.hover[slot] = None;
            let items = items_of(slot);
            app.active[slot] =
                command::visible_indices(&items, &app.query[slot], preview_count(slot))
                    .into_iter()
                    .find(|index| items[*index].is_actionable());
        }
        Message::Chosen { slot, index } => choose(app, slot, index),
        Message::Highlighted { slot, index } => app.hover[slot] = Some(index),
        // The row travels with the report: iced walks the rows top-down, so an
        // upper row's entry is published before a lower row's leave. Only the row
        // the pointer was on may be cleared, or entering the upper row is undone
        // on the same move. `None` is the list, and clears the hover whatever row
        // it sat on.
        Message::Left { slot, row } => {
            if row.is_none() || app.hover[slot] == row {
                app.hover[slot] = None;
            }
        }
        Message::Key(key) => {
            if let Some(slot) = open_slot(app) {
                let items = items_of(slot);
                let preview = preview_count(slot);
                match key {
                    Key::Dismiss => close(app, slot),
                    Key::Choose => {
                        let first = command::visible_indices(&items, &app.query[slot], preview)
                            .into_iter()
                            .find(|index| items[*index].is_actionable());
                        if let Some(index) = app.active[slot].or(first) {
                            choose(app, slot, index);
                        }
                    }
                    step => {
                        let rows: Vec<usize> =
                            command::visible_indices(&items, &app.query[slot], preview)
                                .into_iter()
                                .filter(|index| items[*index].is_actionable())
                                .collect();
                        app.active[slot] = step_row(&rows, app.active[slot], step);
                        app.hover[slot] = None;
                    }
                }
            }
        }
        Message::MenuOpen(open) => app.menu_open = open,
        Message::StocksOpen(open) => app.stocks_open = open,
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

/// The dialog section whose panel is open, or nothing while both are closed.
/// The traversal keys act on it, because iced reports no focus to the component.
fn open_slot(app: &App) -> Option<usize> {
    if app.menu_open {
        Some(slot::DIALOG)
    } else if app.stocks_open {
        Some(slot::SEARCH)
    } else {
        None
    }
}

/// Close the dialog a slot belongs to.
fn close(app: &mut App, slot: usize) {
    if slot == slot::SEARCH {
        app.stocks_open = false;
    } else {
        app.menu_open = false;
    }
    app.hover[slot] = None;
}

/// Answer a chosen row: light it, record it the way the Slint `remember` does,
/// and close the dialog if the palette was in one.
fn choose(app: &mut App, slot: usize, index: usize) {
    app.active[slot] = Some(index);
    app.hover[slot] = None;
    app.last = format!("row {index} — {}", section_name(slot));
    if slot == slot::DIALOG || slot == slot::SEARCH {
        close(app, slot);
    }
}

/// The name the "Last confirmed" line records, which is the section the choice
/// was made in — the Slint `remember(index, label)` call's second argument.
fn section_name(slot: usize) -> &'static str {
    match slot {
        slot::INLINE => "inline",
        slot::QUICK => "quick",
        slot::DIALOG => "dialog",
        slot::SCROLL => "scroll",
        slot::ROWS => "rows",
        slot::SEARCH => "stock",
        _ => "command",
    }
}

/// A key the palette answers, mapped from the keyboard by the subscription and
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

/// The keyboard a palette leaves to its host.
///
/// `Command` reports the row the pointer entered and draws the row the caller
/// says is lit, but iced 0.14 has no focus handle a palette can take, so the
/// keys are the application's. `↑`/`↓` step, `Home`/`End` jump, `Enter` chooses
/// the lit row and `Escape` closes the open panel — the keys the hints band
/// names.
fn subscription(_app: &App) -> Subscription<Message> {
    iced::keyboard::listen().filter_map(|event| match event {
        keyboard::Event::KeyPressed { key, .. } => match key.as_ref() {
            keyboard::Key::Named(keyboard::key::Named::ArrowDown) => Some(Message::Key(Key::Down)),
            keyboard::Key::Named(keyboard::key::Named::ArrowUp) => Some(Message::Key(Key::Up)),
            keyboard::Key::Named(keyboard::key::Named::Home) => Some(Message::Key(Key::First)),
            keyboard::Key::Named(keyboard::key::Named::End) => Some(Message::Key(Key::Last)),
            keyboard::Key::Named(keyboard::key::Named::Enter) => Some(Message::Key(Key::Choose)),
            keyboard::Key::Named(keyboard::key::Named::Escape) => Some(Message::Key(Key::Dismiss)),
            _ => None,
        },
        _ => None,
    })
}

/// The row a step lands on, over the rows the query leaves actionable, stopping
/// at the ends rather than wrapping. The same stepping rule the select and
/// combobox galleries write where their keys are handled.
fn step_row(rows: &[usize], from: Option<usize>, step: Key) -> Option<usize> {
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

/// The name iced keeps a palette's search field under. It has to be stable
/// across frames and distinct per palette, so the gallery gives each slot its
/// own; a page with one palette can leave the component's default.
fn search_id(slot: usize) -> Id {
    Id::new(match slot {
        slot::INLINE => "command-inline",
        slot::QUICK => "command-quick",
        slot::DIALOG => "command-dialog",
        slot::SCROLL => "command-scroll",
        slot::ROWS => "command-rows",
        slot::SEARCH => "command-search",
        _ => "command",
    })
}

/// Show only this many rows for a palette while its query is empty, `0` for no
/// preview. Only the stock panel sets one.
fn preview_count(slot: usize) -> usize {
    if slot == slot::SEARCH { 5 } else { 0 }
}

/// The items a palette holds, in the Slint example's order. The two dialog
/// palettes show the same list as the inline one.
fn items_of(slot: usize) -> Vec<CommandItem> {
    match slot {
        slot::INLINE | slot::DIALOG => vec![
            CommandItem::new("Calendar")
                .icon(CALENDAR)
                .group("Suggestions"),
            CommandItem::new("Search Emoji")
                .icon(SEARCH_ICON_XML)
                .group("Suggestions")
                .keywords("smile icon")
                .checked(true),
            CommandItem::new("Calculator")
                .icon(FRAME)
                .group("Suggestions")
                .disabled(true),
            CommandItem::separator(),
            CommandItem::new("Profile")
                .icon(USER)
                .shortcut("Cmd P")
                .group("Settings"),
            CommandItem::new("Billing")
                .icon(CIRCLE_USER)
                .shortcut("Cmd B")
                .group("Settings"),
            CommandItem::new("Settings")
                .icon(SETTINGS)
                .shortcut("Cmd S")
                .group("Settings"),
        ],
        slot::QUICK => vec![
            CommandItem::new("New File").icon(PLUS),
            CommandItem::new("Duplicate").icon(COPY),
            CommandItem::new("Move to Trash").icon(TRASH),
        ],
        slot::SCROLL => vec![
            CommandItem::new("Home")
                .icon(LAYOUT_DASHBOARD)
                .shortcut("Cmd H")
                .group("Navigation"),
            CommandItem::new("Inbox")
                .icon(INBOX)
                .shortcut("Cmd I")
                .group("Navigation"),
            CommandItem::new("Documents")
                .icon(FILE)
                .shortcut("Cmd D")
                .group("Navigation"),
            CommandItem::new("Folders")
                .icon(FOLDER)
                .shortcut("Cmd F")
                .group("Navigation"),
            CommandItem::separator(),
            CommandItem::new("New File")
                .icon(PLUS)
                .shortcut("Cmd N")
                .group("Actions"),
            CommandItem::new("Copy")
                .icon(COPY)
                .shortcut("Cmd C")
                .group("Actions"),
            CommandItem::new("Delete")
                .icon(TRASH)
                .shortcut("⌫")
                .group("Actions"),
            CommandItem::separator(),
            CommandItem::new("Profile").icon(USER).group("Account"),
            CommandItem::new("Notifications")
                .icon(BELL)
                .group("Account"),
            CommandItem::new("Help & Support")
                .icon(INFO)
                .group("Account"),
            CommandItem::separator(),
            CommandItem::new("Palette").icon(PALETTE).group("Tools"),
            CommandItem::new("Terminal")
                .icon(SQUARE_TERMINAL)
                .group("Tools"),
            CommandItem::new("Globe").icon(GLOBE).group("Tools"),
        ],
        slot::ROWS => vec![
            CommandItem::new("Compact custom row"),
            CommandItem::new("Expanded custom row")
                .detail("Its extra detail gives this row a different height."),
        ],
        slot::SEARCH => vec![
            stock("AAPL.US", "Apple Inc.", "228.52", "+1.24%", true, true),
            stock(
                "NVDA.US",
                "NVIDIA Corporation",
                "134.81",
                "-0.62%",
                false,
                true,
            ),
            stock("TSLA.US", "Tesla, Inc.", "251.44", "+3.18%", true, true),
            stock(
                "MSFT.US",
                "Microsoft Corporation",
                "428.02",
                "+0.41%",
                true,
                true,
            ),
            stock(
                "AMZN.US",
                "Amazon.com, Inc.",
                "186.33",
                "-1.07%",
                false,
                true,
            ),
            stock(
                "700.HK",
                "Tencent Holdings Ltd.",
                "412.60",
                "+0.87%",
                true,
                false,
            ),
            stock(
                "9988.HK",
                "Alibaba Group Holding Ltd.",
                "82.15",
                "-2.31%",
                false,
                false,
            ),
            stock("3690.HK", "Meituan", "128.90", "+1.66%", true, false),
            stock(
                "600519.SH",
                "Kweichow Moutai Co., Ltd.",
                "1482.00",
                "-0.34%",
                false,
                false,
            ),
            stock(
                "000858.SZ",
                "Wuliangye Yibin Co., Ltd.",
                "142.77",
                "+0.19%",
                true,
                false,
            ),
        ],
        _ => Vec::new(),
    }
}

/// One rich stock row, the Slint `stock()` helper: the symbol is what the query
/// matches besides the label, the price is the meta and the change its colour.
fn stock(
    symbol: &str,
    name: &str,
    price: &str,
    change: &str,
    up: bool,
    popular: bool,
) -> CommandItem {
    let item = CommandItem::new(name)
        .keywords(symbol)
        .meta(price)
        .change(change)
        .up(up);
    if popular { item.group("Popular") } else { item }
}

/// One palette, wired to the slot its query, highlight and hover live in. The
/// sections below add what makes each one different — no search, a scrolling
/// list, custom rows — so the reader sees that beside the call.
fn palette<'a>(app: &App, slot: usize) -> Command<'a, Message> {
    Command::new(items_of(slot))
        .search_id(search_id(slot))
        .query(app.query[slot].clone())
        .active(app.active_in(slot))
        .on_chosen(move |index| Message::Chosen { slot, index })
        .on_query(move |query| Message::Queried { slot, query })
        .on_highlight(move |index| Message::Highlighted { slot, index })
        .on_leave(move |row| Message::Left { slot, row })
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box the other galleries draw.
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
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// The framed card a dialog section is demonstrated in — the Slint example's
/// `Rectangle { height: 420px; border-width: 1px; border-radius: lg; clip }`.
/// The stack the caller passes is the card's body, so the dialog it holds fills
/// the card rather than the window, which is what the reference does.
fn card<'a>(body: impl Into<Element<'a, Message>>, height: f32) -> Element<'a, Message> {
    container(body)
        .width(Length::Fill)
        .height(Length::Fixed(height))
        .padding(theme::SPACING_LG)
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: Tokens::of(theme).border,
                width: 1.0,
                radius: theme::RADIUS_LG.into(),
            },
            ..container::Style::default()
        })
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Inline
    // The whole of a palette: the items, the caller's query and lit row, and the
    // four reports. `Item` is the row shape the menu, the select and the combobox
    // already share, extended with the group a heading is drawn from and the
    // keywords the query matches besides the label.
    let inline = body_box(
        Command::new(items_of(slot::INLINE))
            .search_id(search_id(slot::INLINE))
            .query(app.query[slot::INLINE].clone())
            .active(app.active_in(slot::INLINE))
            .on_chosen(|index| Message::Chosen {
                slot: slot::INLINE,
                index,
            })
            .on_query(|query| Message::Queried {
                slot: slot::INLINE,
                query,
            })
            .on_highlight(|index| Message::Highlighted {
                slot: slot::INLINE,
                index,
            })
            .on_leave(|row| Message::Left {
                slot: slot::INLINE,
                row,
            }),
    );

    // section: Quick Actions Without Search
    // `searchable(false)` drops the query field and the filtering with it; the
    // list stays whole, which is the Slint "Quick actions" card.
    let quick = body_box(palette(app, slot::QUICK).searchable(false));

    // section: In a Dialog
    // The palette is the same one; what changes is its frame. `Dialog` draws the
    // dimmed backdrop and the centred panel — the popover's own surface — and the
    // command is asked for `bordered(false)`, so exactly one surface is drawn and
    // the heading and the hints band belong to the command inside it.
    let dialog = body_box(card(
        Stack::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Button::new("Open Menu")
                    .outline()
                    .on_press(Message::MenuOpen(true)),
            )
            .push(
                Dialog::new()
                    .open(app.menu_open)
                    .close_button(false)
                    .content_padding(0.0)
                    .max_width(380.0)
                    .content(
                        Command::new(items_of(slot::DIALOG))
                            .search_id(search_id(slot::DIALOG))
                            .query(app.query[slot::DIALOG].clone())
                            .active(app.active_in(slot::DIALOG))
                            .bordered(false)
                            .heading("Commands")
                            .hints(true)
                            .on_chosen(|index| Message::Chosen {
                                slot: slot::DIALOG,
                                index,
                            })
                            .on_query(|query| Message::Queried {
                                slot: slot::DIALOG,
                                query,
                            })
                            .on_highlight(|index| Message::Highlighted {
                                slot: slot::DIALOG,
                                index,
                            })
                            .on_leave(|row| Message::Left {
                                slot: slot::DIALOG,
                                row,
                            }),
                    )
                    .on_open_change(Message::MenuOpen),
            ),
        MENU_CARD,
    ));

    // section: Scrollable
    // `max_list_height` puts the rows in the shared scroll area at that height,
    // which is the Slint `max-list-height` and its `Scrollable`.
    let scroll = body_box(palette(app, slot::SCROLL).max_list_height(220.0));

    // section: Variable-height rows
    // `custom_rows(true)` draws each row as its label and its detail, so a row
    // with a detail is taller than one without — the Slint variable-height card.
    let rows = body_box(palette(app, slot::ROWS).custom_rows(true));

    // section: Search panel
    // A second dialog, this time over an asynchronous-looking result list.
    // `preview_count(5)` shows the first five rows while the query is empty; a
    // typed query searches every row, and a rich row draws its symbol, name,
    // price and change in the two columns the Slint `stock()` helper fills.
    let search = body_box(card(
        Stack::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Button::new("Search Stocks")
                    .outline()
                    .on_press(Message::StocksOpen(true)),
            )
            .push(
                Dialog::new()
                    .open(app.stocks_open)
                    .close_button(false)
                    .content_padding(0.0)
                    .max_width(380.0)
                    .content(
                        Command::new(items_of(slot::SEARCH))
                            .search_id(search_id(slot::SEARCH))
                            .query(app.query[slot::SEARCH].clone())
                            .active(app.active_in(slot::SEARCH))
                            .bordered(false)
                            .heading("Stocks")
                            .placeholder("Search symbols or companies...")
                            .preview_count(5)
                            .on_chosen(|index| Message::Chosen {
                                slot: slot::SEARCH,
                                index,
                            })
                            .on_query(|query| Message::Queried {
                                slot: slot::SEARCH,
                                query,
                            })
                            .on_highlight(|index| Message::Highlighted {
                                slot: slot::SEARCH,
                                index,
                            })
                            .on_leave(|row| Message::Left {
                                slot: slot::SEARCH,
                                row,
                            }),
                    )
                    .on_open_change(Message::StocksOpen),
            ),
        STOCK_CARD,
    ));

    // section: Last confirmed
    // The report `on_chosen` makes, made visible the way the Slint `remember`
    // records it: the row's index and the section the choice was made in.
    let last = body_box(
        text(app.last.clone())
            .size(theme::FONT_SIZE_SM)
            .font(theme::MONO)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
    );

    // section: end
    let status = text(
        "Click a palette's search field and type to filter; the lit row is chosen \
         with a click, or with Enter while a dialog is open. Escape closes the open \
         panel.",
    )
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        inline,
        heading(app, 1, SECTIONS[1]),
        quick,
        heading(app, 2, SECTIONS[2]),
        dialog,
        heading(app, 3, SECTIONS[3]),
        scroll,
        heading(app, 4, SECTIONS[4]),
        rows,
        heading(app, 5, SECTIONS[5]),
        search,
        heading(app, 6, SECTIONS[6]),
        last,
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

const CALENDAR: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M8 2v3" /> <path d="M16 2v3" /> <rect x="3" y="3" width="18" height="18" rx="2" /> <path d="M3 9h18" /> </svg>"#;

const SEARCH_ICON_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m21 21-4.34-4.34" /> <circle cx="11" cy="11" r="8" /> </svg>"#;

const FRAME: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <line x1="22" x2="2" y1="6" y2="6" /> <line x1="22" x2="2" y1="18" y2="18" /> <line x1="6" x2="6" y1="2" y2="22" /> <line x1="18" x2="18" y1="2" y2="22" /> </svg>"#;

const USER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" /> <circle cx="12" cy="7" r="4" /> </svg>"#;

const CIRCLE_USER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <circle cx="12" cy="12" r="10" /> <circle cx="12" cy="10" r="3" /> <path d="M7 20.662V19a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v1.662" /> </svg>"#;

const SETTINGS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915" /> <circle cx="12" cy="12" r="3" /> </svg>"#;

const PLUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M5 12h14" /> <path d="M12 5v14" /> </svg>"#;

const COPY: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <rect width="14" height="14" x="8" y="8" rx="2" ry="2" /> <path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" /> </svg>"#;

const TRASH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10 11v6" /> <path d="M14 11v6" /> <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" /> <path d="M3 6h18" /> <path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" /> </svg>"#;

const LAYOUT_DASHBOARD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <rect width="7" height="9" x="3" y="3" rx="1" /> <rect width="7" height="5" x="14" y="3" rx="1" /> <rect width="7" height="9" x="14" y="12" rx="1" /> <rect width="7" height="5" x="3" y="16" rx="1" /> </svg>"#;

const INBOX: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <polyline points="22 12 16 12 14 15 10 15 8 12 2 12" /> <path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" /> </svg>"#;

const FILE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z" /> <path d="M14 2v5a1 1 0 0 0 1 1h5" /> </svg>"#;

const FOLDER: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z" /> </svg>"#;

const BELL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M10.268 21a2 2 0 0 0 3.464 0" /> <path d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326" /> </svg>"#;

const INFO: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <circle cx="12" cy="12" r="10" /> <path d="M12 16v-4" /> <path d="M12 8h.01" /> </svg>"#;

const PALETTE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="M12 22a1 1 0 0 1 0-20 10 9 0 0 1 10 9 5 5 0 0 1-5 5h-2.25a1.75 1.75 0 0 0-1.4 2.8l.3.4a1.75 1.75 0 0 1-1.4 2.8z" /> <circle cx="13.5" cy="6.5" r=".5" fill="currentColor" /> <circle cx="17.5" cy="10.5" r=".5" fill="currentColor" /> <circle cx="6.5" cy="12.5" r=".5" fill="currentColor" /> <circle cx="8.5" cy="7.5" r=".5" fill="currentColor" /> </svg>"#;

const SQUARE_TERMINAL: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <path d="m7 11 2-2-2-2" /> <path d="M11 13h4" /> <rect width="18" height="18" x="3" y="3" rx="2" ry="2" /> </svg>"#;

const GLOBE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" > <circle cx="12" cy="12" r="10" /> <path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" /> <path d="M2 12h20" /> </svg>"#;
