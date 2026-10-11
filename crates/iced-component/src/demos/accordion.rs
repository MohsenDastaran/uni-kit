//! `accordion` — a vertical stack of collapsible sections.
//!
//! Run it with `cargo run --example accordion`, or in the browser as
//! `?story=accordion` in the gallery the website embeds.

use iced::widget::{column, text};
use iced::{Element, Task};

use crate::components::accordion::{self, Accordion, Section};
use crate::components::theme::{ControlSize, Tokens};

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — accordion")
        .theme(theme)
        .run()
}

#[derive(Debug, Clone, Default)]
struct App {
    /// Which sections are open. Slint's `open-index` is one section in single
    /// mode; the caller's set is what an iced application keeps instead.
    open: Vec<bool>,
}

#[derive(Debug, Clone)]
enum Message {
    Toggled { index: usize, open: bool },
}

fn boot() -> App {
    App {
        open: vec![true, false, false],
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Toggled { index, open } => {
            // Single mode: opening one section closes the others. An accordion
            // built with `.multiple(true)` would leave them alone.
            let _ = open;
            for (i, slot) in app.open.iter_mut().enumerate() {
                *slot = i == index && open;
            }
        }
    }
    Task::none()
}

fn view(app: &App) -> Element<'_, Message> {
    let accordion = Accordion::new([
        Section::new(accordion::text_header(
            "Shipping address",
            14.0,
            Message::Toggled {
                index: 0,
                open: !app.open[0],
            },
        ))
        .open(app.open[0])
        .content(
            text("1 Infinite Loop, Cupertino, CA")
                .size(14.0)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
        ),
        Section::new(accordion::text_header(
            "Payment method",
            14.0,
            Message::Toggled {
                index: 1,
                open: !app.open[1],
            },
        ))
        .open(app.open[1])
        .content(
            text("Visa ending in 4242")
                .size(14.0)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        ),
        Section::new(accordion::text_header(
            "Notification preferences",
            14.0,
            Message::Toggled {
                index: 2,
                open: !app.open[2],
            },
        ))
        .open(app.open[2])
        .content(
            text("Weekly summaries, and anything that mentions you.")
                .size(14.0)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        ),
    ])
    .size(ControlSize::Medium);

    column![
        accordion,
        column![
            text("Small, unbordered, several open at once").size(12.0).style(
                |theme: &iced::Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }
            ),
            Accordion::new([
                Section::new(accordion::text_header(
                    "Alpha",
                    12.0,
                    Message::Toggled {
                        index: 0,
                        open: true,
                    },
                ))
                .open(true)
                .content(text("First").size(12.0)),
                Section::new(accordion::text_header(
                    "Beta",
                    12.0,
                    Message::Toggled {
                        index: 1,
                        open: true,
                    },
                ))
                .open(true)
                .content(text("Second").size(12.0)),
            ])
            .size(ControlSize::XSmall)
            .bordered(false),
        ]
        .spacing(8.0),
    ]
    .spacing(16.0)
    .padding(16.0)
    .into()
}
