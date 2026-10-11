//! `tree` — a disclosure tree.
//!
//! Run it with `cargo run --example tree`, or in the browser as
//! `?story=tree` in the gallery the website embeds. The application owns the open set;
//! the component only reports the requests.

use std::collections::HashSet;

use iced::widget::{column, text};
use iced::{Element, Task};

use crate::components::tree::{Node, Tree};

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — tree")
        .theme(theme)
        .run()
}

/// Stable ids: a `&'static str` is what a real file tree would call a path.
const SRC: &str = "src";
const COMPONENTS: &str = "src/components";
const TREE: &str = "src/components/tree.rs";
const LIB: &str = "src/lib.rs";
const CARGO: &str = "Cargo.toml";
const README: &str = "README.md";

struct App {
    /// Which folders are open. This is the state the Slint model keeps in
    /// `expanded`, and the one a caller must own here.
    open: HashSet<&'static str>,
    selected: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(&'static str),
    Select(&'static str),
}

fn boot() -> App {
    App {
        open: HashSet::from([SRC, COMPONENTS]),
        selected: Some(TREE),
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Toggle(id) => {
            if !app.open.remove(id) {
                app.open.insert(id);
            }
        }
        Message::Select(id) => app.selected = Some(id),
    }
    Task::none()
}

fn folder(id: &'static str, label: &str, children: Vec<Node<'static, &'static str, Message>>) -> Node<'static, &'static str, Message> {
    Node::new(id, label).icon(text("\u{1F4C1}")).extend(children)
}

fn file(id: &'static str, label: &str) -> Node<'static, &'static str, Message> {
    Node::new(id, label).icon(text("\u{1F4C4}"))
}

fn view(app: &App) -> Element<'_, Message> {
    let nodes = vec![
        folder(
            SRC,
            "src",
            vec![
                folder(COMPONENTS, "components", vec![
                    file(TREE, "tree.rs"),
                    file("src/components/calendar.rs", "calendar.rs"),
                ]),
                file(LIB, "lib.rs"),
            ],
        ),
        file(CARGO, "Cargo.toml"),
        file(README, "README.md"),
    ];

    let selected = app
        .selected
        .map(|id| format!("selected: {id}"))
        .unwrap_or_else(|| "selected: none".into());

    let open = app.open.len();

    column![
        Tree::new(nodes)
            .selected(app.selected)
            .is_open(move |id: &&'static str| app.open.contains(*id))
            .on_toggle(Message::Toggle)
            .on_select(Message::Select),
        text(selected).size(13),
        text(format!("{open} folder(s) open — click an arrow to toggle, a row to select"))
            .size(13),
    ]
    .spacing(12)
    .padding(16)
    .into()
}
