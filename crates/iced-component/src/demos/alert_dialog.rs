//! `alert-dialog` — the whole gallery the Slint example shows: the default
//! confirmation, a delete prompt with an icon, a centered permission prompt, a
//! destructive action, an alert with no title, a custom footer, custom content,
//! a notice, a plain confirm, and a dialog that refuses to close.
//!
//! Run it with `cargo run --example alert-dialog`, or in the browser as
//! `?story=alert-dialog` in the gallery the website embeds.
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
//! [`AlertDialog`] is the component this page is about; the sections the Slint
//! example draws with a plain `Dialog` — an icon, custom content, a footer of
//! its own — are drawn with the same [`Dialog`] here, because the alert dialog
//! deliberately has no slot for them. The gallery owns every open flag and the
//! sync note, because the components have none.
//!
//! Every dialog is a layer in one [`Stack`](iced::widget::Stack) over the page,
//! which is the mechanism the component module documents: a stack answers its
//! topmost layer first and stops at the one that captures.

use iced::keyboard;
use iced::widget::{Stack, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Subscription, Task, Theme};

use crate::components::alert_dialog::AlertDialog;
use crate::components::button::{self, Button};
use crate::components::dialog::{Dialog, DialogFooter};
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's headings — have them. The index is what the page is asked to
/// scroll to.
const SECTIONS: [&str; 10] = [
    "Default",
    "Delete file",
    "Icon",
    "Destructive",
    "Without title",
    "Custom footer",
    "Custom content",
    "Notice",
    "Confirm",
    "Prevent close",
];

/// Which dialog each piece of state belongs to.
mod slot {
    pub const DEFAULT: usize = 0;
    pub const DELETE: usize = 1;
    pub const ICON: usize = 2;
    pub const DESTRUCTIVE: usize = 3;
    pub const UNTITLED: usize = 4;
    pub const FOOTER: usize = 5;
    pub const CONTENT: usize = 6;
    pub const NOTICE: usize = 7;
    pub const CONFIRM: usize = 8;
    pub const SYNC: usize = 9;
    /// One more than the highest slot: the length of the state array.
    pub const COUNT: usize = 10;
}

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — alert-dialog")
        .theme(theme)
        .subscription(subscription)
        .run()
}

#[derive(Debug, Clone)]
struct App {
    /// Whether each dialog's layer is showing.
    open: [bool; slot::COUNT],
    /// The sync dialog's line, which its buttons write and its title does not.
    sync_note: String,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A dialog asked to be opened or closed; the slot says which one.
    Opened {
        slot: usize,
        open: bool,
    },
    /// The sync dialog's buttons write this instead of closing.
    Note(String),
    /// Escape was pressed: close every dialog.
    Escape,
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
        open: [false; slot::COUNT],
        sync_note: String::new(),
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Opened { slot, open } => {
            if let Some(flag) = app.open.get_mut(slot) {
                *flag = open;
            }
        }
        Message::Note(note) => app.sync_note = note,
        // An in-flow layer is handed no key of its own, so Escape is the
        // application's — see the module comment in `components/dialog.rs`.
        Message::Escape => app.open = [false; slot::COUNT],
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

/// The key the layer cannot take for itself. Escape closes every dialog, which
/// is the binding the component's module comment names.
fn subscription(_app: &App) -> Subscription<Message> {
    iced::keyboard::listen().filter_map(|event| match event {
        keyboard::Event::KeyPressed { key, .. } => match key.as_ref() {
            keyboard::Key::Named(keyboard::key::Named::Escape) => Some(Message::Escape),
            _ => None,
        },
        _ => None,
    })
}

/// The trigger a section shows: the ordinary outlined button the Slint example
/// centers in its preview frame, wired to open one slot.
fn open_button<'a>(label: &'static str, slot: usize) -> Element<'a, Message> {
    Button::new(label)
        .outline()
        .on_press(Message::Opened { slot, open: true })
        .into()
}

/// The button a section's own footer uses: it closes one slot.
fn close_button<'a>(label: &'static str, slot: usize) -> Button<'a, Message> {
    Button::new(label).on_press(Message::Opened { slot, open: false })
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

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box the dialog gallery draws.
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

/// A title and a line beside a leading glyph, the Slint example's
/// `HorizontalLayout` inside a panel body.
fn icon_row<'a>(
    glyph: &'static [u8],
    color: theme::Status,
    size: f32,
    title: &'static str,
    description: &'static str,
) -> Element<'a, Message> {
    row![
        svg(svg::Handle::from_memory(glyph))
            .width(size)
            .height(size)
            .style(move |theme: &Theme, _status| svg::Style {
                color: Some(Tokens::of(theme).status_color(color)),
            }),
        column![
            text(title)
                .size(theme::FONT_SIZE_LG)
                .font(iced::Font {
                    weight: iced::font::Weight::Semibold,
                    ..iced::Font::DEFAULT
                })
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
            text(description)
                .size(theme::FONT_SIZE_SM)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        ]
        .spacing(4.0)
        .width(Length::Fill),
    ]
    .spacing(12.0)
    .align_y(Alignment::Start)
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Default
    // The default confirmation: a title, a description and a centered footer
    // whose cancel action comes first and whose destructive one comes last.
    let default = body_box(
        column![
            open_button("Discard Draft", slot::DEFAULT),
            muted_line("A centered footer: Cancel first, then the destructive action."),
        ]
        .spacing(8.0),
    );
    let default_dialog = Dialog::new()
        .open(app.open[slot::DEFAULT])
        .close_button(false)
        .title("Discard unsaved changes?")
        .description("Your edits since the last save will be permanently lost.")
        .footer(
            DialogFooter::new()
                .centered(true)
                .child(close_button("Cancel", slot::DEFAULT))
                .child(
                    Button::new("Discard")
                        .variant(button::Variant::Danger)
                        .on_press(Message::Opened {
                            slot: slot::DEFAULT,
                            open: false,
                        }),
                ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::DEFAULT,
            open,
        });

    // section: Delete file
    // The panel's body is the caller's, so a leading danger glyph and the two
    // lines beside it are an ordinary row placed in `content`.
    let delete = body_box(
        column![
            open_button("Delete File", slot::DELETE),
            muted_line("A danger glyph leads the title and the line beside it."),
        ]
        .spacing(8.0),
    );
    let delete_dialog = Dialog::new()
        .open(app.open[slot::DELETE])
        .close_button(false)
        .content(icon_row(
            INFO_XML,
            theme::Status::Danger,
            20.0,
            "Delete File",
            "Are you sure you want to delete this file? This action cannot be undone.",
        ))
        .footer(
            DialogFooter::new()
                .centered(true)
                .child(close_button("Cancel", slot::DELETE))
                .child(
                    Button::new("Delete")
                        .variant(button::Variant::Danger)
                        .on_press(Message::Opened {
                            slot: slot::DELETE,
                            open: false,
                        }),
                ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::DELETE,
            open,
        });

    // section: Icon
    // The centered prompt: the glyph, the title and the line stacked, and the
    // two full-width actions stacked under them. A `footer` accepts any element,
    // so it is a column here rather than the usual row.
    let icon = body_box(
        column![
            open_button("Request Permission", slot::ICON),
            muted_line("Everything is centered, and the actions are stacked full width."),
        ]
        .spacing(8.0),
    );
    let icon_dialog =
        Dialog::new()
            .open(app.open[slot::ICON])
            .close_button(false)
            .content(
                column![
                svg(svg::Handle::from_memory(TRIANGLE_ALERT))
                    .width(40.0)
                    .height(40.0)
                    .style(|theme: &Theme, _status| svg::Style {
                        color: Some(Tokens::of(theme).warning),
                    }),
                container(
                    text("Network Permission Required")
                        .size(theme::FONT_SIZE_LG)
                        .font(iced::Font {
                            weight: iced::font::Weight::Semibold,
                            ..iced::Font::DEFAULT
                        })
                )
                .width(Length::Fill)
                .center_x(Length::Fill),
                container(
                    text("We need your permission to access the network. Allow access to continue.")
                        .size(theme::FONT_SIZE_SM)
                        .style(|theme: &Theme| text::Style {
                            color: Some(Tokens::of(theme).muted_foreground),
                        })
                )
                .width(Length::Fill)
                .center_x(Length::Fill),
            ]
                .spacing(8.0),
            )
            .footer(
                column![
                    Button::new("Allow")
                        .variant(button::Variant::Primary)
                        .width(Length::Fill)
                        .on_press(Message::Opened {
                            slot: slot::ICON,
                            open: false,
                        }),
                    Button::new("Deny")
                        .outline()
                        .width(Length::Fill)
                        .on_press(Message::Opened {
                            slot: slot::ICON,
                            open: false,
                        }),
                ]
                .spacing(8.0),
            )
            .on_open_change(|open| Message::Opened {
                slot: slot::ICON,
                open,
            });

    // section: Destructive
    // Both actions are outlined and share the row, which is the Slint example's
    // `horizontal-stretch` on each button.
    let destructive = body_box(
        column![
            open_button("Delete Account", slot::DESTRUCTIVE),
            muted_line("Two outlined actions share the row."),
        ]
        .spacing(8.0),
    );
    let destructive_dialog = Dialog::new()
        .open(app.open[slot::DESTRUCTIVE])
        .close_button(false)
        .title("Delete Account")
        .description(
            "This will permanently delete your account and all associated data. \
             This action cannot be undone.",
        )
        .footer(
            DialogFooter::new()
                .centered(true)
                .child(
                    Button::new("Cancel")
                        .outline()
                        .width(Length::Fill)
                        .on_press(Message::Opened {
                            slot: slot::DESTRUCTIVE,
                            open: false,
                        }),
                )
                .child(
                    Button::new("Delete")
                        .outline()
                        .variant(button::Variant::Danger)
                        .width(Length::Fill)
                        .on_press(Message::Opened {
                            slot: slot::DESTRUCTIVE,
                            open: false,
                        }),
                ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::DESTRUCTIVE,
            open,
        });

    // section: Without title
    // The component itself: `AlertDialog` builds the centered footer from its
    // two labels, and an empty title simply draws no title.
    let untitled = body_box(
        column![
            open_button("Continue without Title", slot::UNTITLED),
            muted_line("`AlertDialog` with a description and no title."),
        ]
        .spacing(8.0),
    );
    let untitled_dialog = AlertDialog::new()
        .open(app.open[slot::UNTITLED])
        .description("Continue with this action?")
        .confirm_text("Continue")
        .on_confirm(Message::Opened {
            slot: slot::UNTITLED,
            open: false,
        })
        .on_open_change(|open| Message::Opened {
            slot: slot::UNTITLED,
            open,
        });

    // section: Custom footer
    // A single full-width action where the footer would be: the panel's body is
    // the caller's, so the "footer" is just the last child.
    let footer = body_box(
        column![
            open_button("Show Session Expiry", slot::FOOTER),
            muted_line("A single full-width action stands in for the footer."),
        ]
        .spacing(8.0),
    );
    let footer_dialog = Dialog::new()
        .open(app.open[slot::FOOTER])
        .close_button(false)
        .title("Session Expired")
        .description("Your session has expired due to inactivity. Please log in again to continue.")
        .content(
            Button::new("Sign in")
                .variant(button::Variant::Primary)
                .width(Length::Fill)
                .on_press(Message::Opened {
                    slot: slot::FOOTER,
                    open: false,
                }),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::FOOTER,
            open,
        });

    // section: Custom content
    // A muted trough holding the two actions, which is the Slint example's
    // `Rectangle { background: Theme.muted }` around a row.
    let content = body_box(
        column![
            open_button("Install Update", slot::CONTENT),
            muted_line("A muted trough holds the two actions."),
        ]
        .spacing(8.0),
    );
    let content_dialog = Dialog::new()
        .open(app.open[slot::CONTENT])
        .close_button(false)
        .title("Update Available")
        .description(
            "A new version (v2.0.0) is available. This update includes new features \
             and bug fixes.",
        )
        .content(
            container(
                row![
                    Button::new("Later")
                        .outline()
                        .width(Length::Fill)
                        .on_press(Message::Opened {
                            slot: slot::CONTENT,
                            open: false,
                        }),
                    Button::new("Update")
                        .variant(button::Variant::Primary)
                        .width(Length::Fill)
                        .on_press(Message::Opened {
                            slot: slot::CONTENT,
                            open: false,
                        }),
                ]
                .spacing(8.0),
            )
            .width(Length::Fill)
            .padding(12.0)
            .style(|theme: &Theme| container::Style {
                background: Some(Tokens::of(theme).muted.into()),
                border: iced::border::rounded(theme::RADIUS_MD),
                ..container::Style::default()
            }),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::CONTENT,
            open,
        });

    // section: Notice
    // An acknowledgement: one full-width action in the centered footer.
    let notice = body_box(
        column![
            open_button("Review Notice", slot::NOTICE),
            muted_line("An acknowledgement with one action to dismiss it."),
        ]
        .spacing(8.0),
    );
    let notice_dialog = Dialog::new()
        .open(app.open[slot::NOTICE])
        .close_button(false)
        .title("Important Notice")
        .description("Please read this important notice carefully before proceeding.")
        .footer(
            DialogFooter::new().centered(true).child(
                Button::new("Got It")
                    .variant(button::Variant::Primary)
                    .on_press(Message::Opened {
                        slot: slot::NOTICE,
                        open: false,
                    }),
            ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::NOTICE,
            open,
        });

    // section: Confirm
    // `AlertDialog` again, this time with the title and the primary confirm
    // label the caller chose.
    let confirm = body_box(
        column![
            open_button("Open Confirmation", slot::CONFIRM),
            muted_line("`AlertDialog` with a title and a confirm label."),
        ]
        .spacing(8.0),
    );
    let confirm_dialog = AlertDialog::new()
        .open(app.open[slot::CONFIRM])
        .title("Are you sure?")
        .description("Continue with this action?")
        .confirm_text("OK")
        .on_confirm(Message::Opened {
            slot: slot::CONFIRM,
            open: false,
        })
        .on_open_change(|open| Message::Opened {
            slot: slot::CONFIRM,
            open,
        });

    // section: Prevent close
    // The footer buttons write the dialog's line instead of closing it; only the
    // close button and the backdrop ask the caller to close, which is what
    // `on_open_change` reports.
    let sync = body_box(
        column![
            open_button("Close During Sync", slot::SYNC),
            muted_line("Nothing in the footer closes it; its buttons write the line."),
        ]
        .spacing(8.0),
    );
    let sync_note = if app.sync_note.is_empty() {
        "Your changes are still syncing. The dialog stays open until syncing finishes.".to_string()
    } else {
        app.sync_note.clone()
    };
    let sync_dialog = Dialog::new()
        .open(app.open[slot::SYNC])
        .title("Sync in progress")
        .description(sync_note)
        .footer(
            DialogFooter::new()
                .centered(true)
                .child(
                    Button::new("Wait")
                        .on_press(Message::Note("Waiting for sync to finish.".to_string())),
                )
                .child(
                    Button::new("Close")
                        .variant(button::Variant::Primary)
                        .on_press(Message::Note("Cannot close while syncing.".to_string())),
                ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::SYNC,
            open,
        });

    // section: end
    let open_count = app.open.iter().filter(|open| **open).count();
    let status = text(match open_count {
        0 => "Open a dialog. Escape or a click on the backdrop closes it.".to_string(),
        1 => "One dialog is open. The layer fills the window and captures the press.".to_string(),
        stacked => format!("{stacked} dialogs are stacked. The topmost answers the press."),
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        default,
        heading(app, 1, SECTIONS[1]),
        delete,
        heading(app, 2, SECTIONS[2]),
        icon,
        heading(app, 3, SECTIONS[3]),
        destructive,
        heading(app, 4, SECTIONS[4]),
        untitled,
        heading(app, 5, SECTIONS[5]),
        footer,
        heading(app, 6, SECTIONS[6]),
        content,
        heading(app, 7, SECTIONS[7]),
        notice,
        heading(app, 8, SECTIONS[8]),
        confirm,
        heading(app, 9, SECTIONS[9]),
        sync,
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

    // Every dialog is a layer in one stack over the page. The order is the order
    // they are pushed, so the topmost answers first.
    Stack::new()
        .push(page)
        .push(default_dialog)
        .push(delete_dialog)
        .push(icon_dialog)
        .push(destructive_dialog)
        .push(untitled_dialog)
        .push(footer_dialog)
        .push(content_dialog)
        .push(notice_dialog)
        .push(confirm_dialog)
        .push(sync_dialog)
        .into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;

const TRIANGLE_ALERT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>"#;

const INFO_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"#;
