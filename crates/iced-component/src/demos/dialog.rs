//! `dialog` — the whole gallery the Slint example shows: a basic dialog, a form,
//! one with an icon, a scrolling body, the two options that change its
//! behaviour, a nested pair, custom styling and custom padding.
//!
//! Run it with `cargo run --example dialog`, or in the browser as
//! `?story=dialog` in the gallery the website embeds.
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
//! The gallery owns every piece of state a dialog draws from — the open flag of
//! each one, the two form strings — because the component has none: it reports a
//! request and the application answers with its own value. Each layer is placed
//! in one [`Stack`](iced::widget::Stack) over the page, which is what makes the
//! nested pair behave: a stack answers its topmost layer first and stops at the
//! one that captures, so a press never reaches the dialog below.
//!
//! The Slint example clips each dialog inside a 320px preview frame. Here the
//! layer fills the window, which is what a modal should do; a caller that wants
//! it inside a smaller frame puts the dialog in a stack over that frame.

use iced::keyboard;
use iced::widget::{Stack, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Subscription, Task, Theme};

use crate::components::button::{self, Button};
use crate::components::dialog::{Dialog, DialogFooter, Style};
use crate::components::group_box::{self, GroupBox};
use crate::components::input::Input;
use crate::components::scrollable::Scrollable;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's headings — have them. The index is what the page is asked to
/// scroll to.
const SECTIONS: [&str; 8] = [
    "Basic Dialog",
    "Form Dialog",
    "Dialog with Icon",
    "Scrollable Dialog",
    "Dialog Options",
    "Nested Dialogs",
    "Custom Styling",
    "Custom Padding",
];

/// Which dialog each piece of state belongs to. A section that shows more than
/// one dialog — Nested Dialogs — still gets one slot per dialog, so opening one
/// does not open its neighbour.
mod slot {
    pub const BASIC: usize = 0;
    pub const FORM: usize = 1;
    pub const ICON: usize = 2;
    pub const SCROLL: usize = 3;
    pub const OPTIONS: usize = 4;
    pub const NESTED_FIRST: usize = 5;
    pub const NESTED_SECOND: usize = 6;
    pub const STYLE: usize = 7;
    pub const PADDING: usize = 8;
    /// One more than the highest slot: the length of the state array.
    pub const COUNT: usize = 9;
}

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — dialog")
        .theme(theme)
        .subscription(subscription)
        .run()
}

#[derive(Debug, Clone)]
struct App {
    /// Whether each dialog's layer is showing.
    open: [bool; slot::COUNT],
    /// The form dialog's two fields.
    name: String,
    email: String,
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
    /// The form dialog's name field was edited.
    Name(String),
    /// The form dialog's email field was edited.
    Email(String),
    /// The nested pair's Done closes both layers, as the Slint example's does.
    NestedDone,
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
        name: String::new(),
        email: String::new(),
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
        Message::Name(name) => app.name = name,
        Message::Email(email) => app.email = email,
        Message::NestedDone => {
            app.open[slot::NESTED_SECOND] = false;
            app.open[slot::NESTED_FIRST] = false;
        }
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

    row![label, icon]
        .spacing(8.0)
        .align_y(Alignment::Center)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic Dialog
    // The whole of a basic dialog: a title, a description and a footer. `open` is
    // the caller's, `on_open_change` reports the close button and a press on the
    // backdrop, and `DialogFooter` is the action row.
    let basic = body_box(
        column![
            open_button("Open Dialog", slot::BASIC),
            muted_line(
                "A title, a description and two actions. The close button and a \
                 click on the backdrop both ask the caller to close it."
            ),
        ]
        .spacing(8.0),
    );
    let basic_dialog = Dialog::new()
        .open(app.open[slot::BASIC])
        .title("Welcome")
        .description("This is a basic dialog with a title, a description, and footer actions.")
        .footer(
            DialogFooter::new()
                .child(Button::new("Cancel").outline().on_press(Message::Opened {
                    slot: slot::BASIC,
                    open: false,
                }))
                .child(
                    Button::new("Continue")
                        .variant(button::Variant::Primary)
                        .on_press(Message::Opened {
                            slot: slot::BASIC,
                            open: false,
                        }),
                ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::BASIC,
            open,
        });

    // section: Form Dialog
    // The body is the caller's, so the fields are ordinary `Input`s holding the
    // application's own strings. A dialog does not own a value or a validation
    // rule any more than the form does.
    let form = body_box(
        column![
            open_button("Edit Profile", slot::FORM),
            muted_line("Two fields in the body; the values live in the application."),
        ]
        .spacing(8.0),
    );
    let form_dialog = Dialog::new()
        .open(app.open[slot::FORM])
        .title("User Information")
        .description("Please enter your details.")
        .content(
            column![
                muted_line("Name"),
                Input::new("dialog-name")
                    .value(app.name.clone())
                    .placeholder("Enter your name")
                    .on_input(Message::Name),
                muted_line("Email"),
                Input::new("dialog-email")
                    .value(app.email.clone())
                    .placeholder("Enter your email")
                    .on_input(Message::Email),
            ]
            .spacing(8.0),
        )
        .footer(
            DialogFooter::new()
                .child(Button::new("Cancel").outline().on_press(Message::Opened {
                    slot: slot::FORM,
                    open: false,
                }))
                .child(
                    Button::new("Submit")
                        .variant(button::Variant::Primary)
                        .on_press(Message::Opened {
                            slot: slot::FORM,
                            open: false,
                        }),
                ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::FORM,
            open,
        });

    // section: Dialog with Icon
    // A dialog with no title property: the icon, the heading and the line are one
    // row of `content`, which is the Slint example's `HorizontalLayout` inside the
    // panel.
    let icon_body = row![
        svg(svg::Handle::from_memory(TRIANGLE_ALERT))
            .width(20.0)
            .height(20.0)
            .style(|theme: &Theme, _status| svg::Style {
                color: Some(Tokens::of(theme).warning),
            }),
        column![
            text("Unsaved Changes")
                .size(theme::FONT_SIZE_LG)
                .font(iced::Font {
                    weight: iced::font::Weight::Semibold,
                    ..iced::Font::DEFAULT
                })
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
            text("This action cannot be undone.")
                .size(theme::FONT_SIZE_SM)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        ]
        .spacing(4.0),
    ]
    .spacing(12.0)
    .align_y(Alignment::Start);

    let icon = body_box(
        column![
            open_button("Discard Draft", slot::ICON),
            muted_line("The panel's content is a row: a warning glyph and the text."),
        ]
        .spacing(8.0),
    );
    let icon_dialog = Dialog::new()
        .open(app.open[slot::ICON])
        .content(icon_body)
        .footer(
            DialogFooter::new()
                .child(Button::new("Cancel").outline().on_press(Message::Opened {
                    slot: slot::ICON,
                    open: false,
                }))
                .child(
                    Button::new("Discard")
                        .variant(button::Variant::Danger)
                        .on_press(Message::Opened {
                            slot: slot::ICON,
                            open: false,
                        }),
                ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::ICON,
            open,
        });

    // section: Scrollable Dialog
    // A bounded `Scrollable` is the Slint example's fixed-height body. The panel
    // never grows past the frame, so the title and the action row stay put while
    // the body scrolls.
    let long_body = Scrollable::new(
        column![
            text(
                "A dialog never grows past the frame. When the body is taller than \
                 the space between the header and the footer, it scrolls instead of \
                 pushing the actions off screen."
            )
            .size(theme::FONT_SIZE_SM),
            text(
                "Keep the title and the action row visible while the rest of the \
                 content moves. This is how long documents, tables, and settings \
                 panels stay usable in a small dialog."
            )
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
            text(
                "Set an explicit height on the scrolling region so iced bounds the \
                 viewport rather than copying the content height onto it."
            )
            .size(theme::FONT_SIZE_SM),
            text(
                "Everything below the fold is reachable with the wheel, the trackpad, \
                 or the scrollbar."
            )
            .size(theme::FONT_SIZE_SM)
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
        ]
        .spacing(8.0),
    )
    .height(Length::Fixed(140.0));

    let scrolling = body_box(
        column![
            open_button("Long Content", slot::SCROLL),
            muted_line("A 140px scroll area holds the body between the header and the footer."),
        ]
        .spacing(8.0),
    );
    let scroll_dialog = Dialog::new()
        .open(app.open[slot::SCROLL])
        .title("Long Content")
        .description("The body scrolls inside a fixed height.")
        .content(long_body)
        .footer(
            DialogFooter::new()
                .child(Button::new("Cancel").outline().on_press(Message::Opened {
                    slot: slot::SCROLL,
                    open: false,
                }))
                .child(
                    Button::new("Confirm")
                        .variant(button::Variant::Primary)
                        .on_press(Message::Opened {
                            slot: slot::SCROLL,
                            open: false,
                        }),
                ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::SCROLL,
            open,
        });

    // section: Dialog Options
    // `close_button(false)` removes the header's close control and
    // `overlay_closable(false)` makes a press on the backdrop ask to stay open;
    // the backdrop still captures it, so nothing behind the dialog is reachable.
    let options = body_box(
        column![
            open_button("Open Options Dialog", slot::OPTIONS),
            muted_line("No close button, and the backdrop ignores a click."),
        ]
        .spacing(8.0),
    );
    let options_dialog = Dialog::new()
        .open(app.open[slot::OPTIONS])
        .close_button(false)
        .overlay_closable(false)
        .title("Dialog Options")
        .description("This dialog hides the close button and ignores clicks on the overlay.")
        .footer(
            DialogFooter::new().child(
                Button::new("Got It")
                    .variant(button::Variant::Primary)
                    .on_press(Message::Opened {
                        slot: slot::OPTIONS,
                        open: false,
                    }),
            ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::OPTIONS,
            open,
        });

    // section: Nested Dialogs
    // Two dialogs in the same stack. The second is pushed above the first, and a
    // stack answers its topmost layer only, so the first stays open underneath
    // until Done closes both — which is what the Slint example's second dialog
    // does.
    let nested = body_box(
        column![
            open_button("Open First Dialog", slot::NESTED_FIRST),
            muted_line("The first opens a second on top of it."),
        ]
        .spacing(8.0),
    );
    let nested_first = Dialog::new()
        .open(app.open[slot::NESTED_FIRST])
        .title("First Dialog")
        .description("This dialog opens another dialog on top of it.")
        .footer(
            DialogFooter::new()
                .child(Button::new("Close").outline().on_press(Message::Opened {
                    slot: slot::NESTED_FIRST,
                    open: false,
                }))
                .child(
                    Button::new("Open Another")
                        .variant(button::Variant::Primary)
                        .on_press(Message::Opened {
                            slot: slot::NESTED_SECOND,
                            open: true,
                        }),
                ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::NESTED_FIRST,
            open,
        });
    let nested_second = Dialog::new()
        .open(app.open[slot::NESTED_SECOND])
        .title("Second Dialog")
        .description("This is the nested dialog.")
        .footer(
            DialogFooter::new().child(
                Button::new("Done")
                    .variant(button::Variant::Primary)
                    .on_press(Message::NestedDone),
            ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::NESTED_SECOND,
            open,
        });

    // section: Custom Styling
    // `style` takes a function of the theme, so the panel's fill, edge and text
    // colours can all come from the tokens the Slint example's properties name.
    let styled = body_box(
        column![
            open_button("Custom Style", slot::STYLE),
            muted_line("The panel fill, edge and text colours come from `Style`."),
        ]
        .spacing(8.0),
    );
    let styled_dialog = Dialog::new()
        .open(app.open[slot::STYLE])
        .title("Custom Dialog")
        .description("Background, border, and text colors all come from properties.")
        .style(|theme: &Theme| {
            let tokens = Tokens::of(theme);
            Style::resolve(tokens)
                .fill(tokens.primary)
                .border(tokens.primary, 1.0)
                .title_color(tokens.primary_foreground)
                .description_color(theme::alpha(tokens.primary_foreground, 0.8))
                .text_color(tokens.primary_foreground)
        })
        .footer(
            DialogFooter::new()
                .child(Button::new("Cancel").outline().on_press(Message::Opened {
                    slot: slot::STYLE,
                    open: false,
                }))
                .child(
                    Button::new("Confirm")
                        .variant(button::Variant::Secondary)
                        .on_press(Message::Opened {
                            slot: slot::STYLE,
                            open: false,
                        }),
                ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::STYLE,
            open,
        });

    // section: Custom Padding
    // `content_padding` is the Slint `content-padding`: one value for the panel's
    // inset instead of a fixed default.
    let padding = body_box(
        column![
            open_button("Custom Padding", slot::PADDING),
            muted_line("A 12px inset replaces the default 20px."),
        ]
        .spacing(8.0),
    );
    let padding_dialog = Dialog::new()
        .open(app.open[slot::PADDING])
        .content_padding(12.0)
        .title("Custom Padding")
        .description("The content inset is controlled by content-padding.")
        .footer(
            DialogFooter::new().child(
                Button::new("Close")
                    .variant(button::Variant::Primary)
                    .on_press(Message::Opened {
                        slot: slot::PADDING,
                        open: false,
                    }),
            ),
        )
        .on_open_change(|open| Message::Opened {
            slot: slot::PADDING,
            open,
        });

    // section: end
    let status = text(match app.open.iter().filter(|open| **open).count() {
        0 => "Open a dialog. Escape or a click on the backdrop closes the topmost one.".to_string(),
        1 => "One dialog is open. The layer fills the window and captures the press.".to_string(),
        stacked => format!(
            "{stacked} dialogs are stacked. The topmost answers; a press never reaches \
             the one below."
        ),
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        basic,
        heading(app, 1, SECTIONS[1]),
        form,
        heading(app, 2, SECTIONS[2]),
        icon,
        heading(app, 3, SECTIONS[3]),
        scrolling,
        heading(app, 4, SECTIONS[4]),
        options,
        heading(app, 5, SECTIONS[5]),
        nested,
        heading(app, 6, SECTIONS[6]),
        styled,
        heading(app, 7, SECTIONS[7]),
        padding,
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
    // they are pushed, so the nested second answers before the first.
    Stack::new()
        .push(page)
        .push(basic_dialog)
        .push(form_dialog)
        .push(icon_dialog)
        .push(scroll_dialog)
        .push(options_dialog)
        .push(nested_first)
        .push(nested_second)
        .push(styled_dialog)
        .push(padding_dialog)
        .into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;

const TRIANGLE_ALERT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>"#;
