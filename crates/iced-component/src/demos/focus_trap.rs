//! `focus-trap` — the region a modal surface puts its controls in: the
//! reference's framed column, two of them side by side, and the region used as
//! the panel of a custom modal that opens and dismisses.
//!
//! Run it with `cargo run --example focus_trap`, or in the browser as
//! `?story=focus-trap` in the gallery the website embeds.
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
//! The page has seven `###` samples; the Slint example is one region, and three
//! of them can be drawn here without lying about what iced does. The other four
//! are named rather than approximated: **Focus Trap with Dialog** and **Focus
//! Trap with Sheet** would show a trap those components do not have (their own
//! module comments say so), **Nested Focus Traps** nests nothing when there is
//! nothing to nest, and **Conditional Focus Trap** would gate a scope that does
//! not exist. What the sections below can honestly show is the frame the
//! reference draws, and a custom modal built the way its page builds one.
//!
//! The gallery owns the open flag and the field values, because the component
//! has no state of its own. Every line that would move focus is the
//! application's: the modal is a layer in one [`Stack`](iced::widget::Stack),
//! the backdrop asks to close, and Escape is the subscription below — an in-flow
//! widget is handed no key of its own, as `components/focus_trap.rs` explains.

use iced::keyboard;
use iced::widget::{Space, Stack, column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Shadow, Subscription, Task, Theme, Vector};

use crate::components::button::{self, Button};
use crate::components::focus_trap::{FocusTrap, Style};
use crate::components::group_box::{self, GroupBox};
use crate::components::input::Input;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The gap between two regions sharing a row, and the margin a centered modal
/// keeps from the window edge.
const GAP: f32 = 16.0;

/// The custom modal's widest panel.
const MODAL_MAX_WIDTH: f32 = 360.0;

/// The section headings, in the order the component page's `###` examples have
/// them. The index is what the page is asked to scroll to.
const SECTIONS: [&str; 3] = [
    "Basic Focus Trap",
    "Multiple Focus Traps",
    "Custom Modal with Focus Trap",
];

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — focus-trap")
        .theme(theme)
        .subscription(subscription)
        .run()
}

#[derive(Debug, Clone)]
struct App {
    /// The basic region's two fields.
    first: String,
    second: String,
    /// The two fields of the pair of regions.
    area_one: String,
    area_two: String,
    /// Whether the custom modal's layer is showing.
    modal_open: bool,
    /// The last thing the application did, drawn under the sections.
    note: Option<&'static str>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    First(String),
    Second(String),
    AreaOne(String),
    AreaTwo(String),
    Submitted,
    OpenModal,
    CloseModal,
    /// A press on the modal's own panel: it keeps the layer open, which is also
    /// what stops that press reaching the backdrop under it.
    StayOpen,
    /// Escape was pressed: dismiss the modal.
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
        first: String::new(),
        second: String::new(),
        area_one: String::new(),
        area_two: String::new(),
        modal_open: false,
        note: None,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::First(value) => app.first = value,
        Message::Second(value) => app.second = value,
        Message::AreaOne(value) => app.area_one = value,
        Message::AreaTwo(value) => app.area_two = value,
        Message::Submitted => {
            app.note = Some("Submit received. The region drew the press; it never held the focus.");
        }
        Message::OpenModal => {
            app.modal_open = true;
            app.note = None;
        }
        Message::CloseModal => app.modal_open = false,
        Message::StayOpen => {}
        // An in-flow layer is handed no key of its own, so Escape is the
        // application's — the same binding `components/focus_trap.rs` names.
        Message::Escape => {
            if app.modal_open {
                app.modal_open = false;
                app.note = Some("Escape closed the modal; the application heard the key.");
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

/// The key the layer cannot take for itself.
fn subscription(_app: &App) -> Subscription<Message> {
    iced::keyboard::listen().filter_map(|event| match event {
        keyboard::Event::KeyPressed { key, .. } => match key.as_ref() {
            keyboard::Key::Named(keyboard::key::Named::Escape) => Some(Message::Escape),
            _ => None,
        },
        _ => None,
    })
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

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic Focus Trap
    // The Slint example's own region: two fields and an action, stacked at the
    // reference's 12px gap inside its 16px inset and one-pixel edge. The frame
    // is all iced can draw of it; the trap is named in the module comment.
    let basic = body_box(
        column![
            FocusTrap::new()
                .child(
                    Input::new("first")
                        .placeholder("First field")
                        .value(app.first.clone())
                        .on_input(Message::First),
                )
                .child(
                    Input::new("second")
                        .placeholder("Second field")
                        .value(app.second.clone())
                        .on_input(Message::Second),
                )
                .child(Button::new("Submit").on_press(Message::Submitted)),
            muted_line(
                "The frame, the gap and the inset are the reference's. iced 0.14 draws no \
                 focus scope, so Tab is not held inside it.",
            ),
        ]
        .spacing(8.0),
    );

    // section: Multiple Focus Traps
    // Two regions sharing a row. The page says each trap is independent; in iced
    // neither holds focus, which is why this is two frames rather than two traps.
    let multiple = body_box(
        column![
            row![
                FocusTrap::new()
                    .child(
                        Input::new("area-one")
                            .placeholder("Area 1")
                            .value(app.area_one.clone())
                            .on_input(Message::AreaOne),
                    )
                    .child(Button::new("Continue").on_press(Message::Submitted)),
                FocusTrap::new()
                    .child(
                        Input::new("area-two")
                            .placeholder("Area 2")
                            .value(app.area_two.clone())
                            .on_input(Message::AreaTwo),
                    )
                    .child(Button::new("Cancel").on_press(Message::Submitted)),
            ]
            .spacing(GAP),
            muted_line(
                "Two frames, side by side. The page's two independent traps need a focus API."
            ),
        ]
        .spacing(8.0),
    );

    // section: Custom Modal with Focus Trap
    // The page's own custom-modal shape: a dimmed full-size backdrop with the
    // region centered on it, here turned into a panel by the style closure.
    let custom_modal = body_box(
        column![
            Button::new("Open Modal").on_press(Message::OpenModal),
            muted_line("A custom modal: the region is the panel, over a dimmed backdrop."),
        ]
        .spacing(8.0),
    );

    let modal_panel = FocusTrap::new()
        .style(|theme: &Theme| {
            let tokens = Tokens::of(theme);
            Style::resolve(tokens).fill(tokens.popover).shadow(Shadow {
                color: tokens.shadow,
                offset: Vector::new(0.0, 4.0),
                blur_radius: if tokens.dark { 8.0 } else { 4.0 },
            })
        })
        .child(
            text("Delete workspace?")
                .size(theme::FONT_SIZE_LG)
                .font(iced::Font {
                    weight: iced::font::Weight::Semibold,
                    ..iced::Font::DEFAULT
                })
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
        )
        .child(
            text("The region is the panel here. Escape and the backdrop both dismiss it.")
                .size(theme::FONT_SIZE_SM)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        )
        .child(
            row![
                Button::new("Cancel")
                    .outline()
                    .width(Length::Fill)
                    .on_press(Message::CloseModal),
                Button::new("Delete")
                    .variant(button::Variant::Danger)
                    .width(Length::Fill)
                    .on_press(Message::CloseModal),
            ]
            .spacing(8.0),
        );

    // The layer, or nothing at all while the modal is closed.
    let modal_layer: Element<'_, Message> = if app.modal_open {
        // The panel captures the press, so a click on its own body never reaches
        // the backdrop: iced captures only for a `mouse_area` that has a message.
        let panel = container(mouse_area(modal_panel).on_press(Message::StayOpen))
            .width(Length::Fill)
            .max_width(MODAL_MAX_WIDTH);
        let centered = container(panel)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding([0.0, PAGE])
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        let backdrop = mouse_area(
            container(Space::new().width(Length::Fill).height(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|theme: &Theme| container::Style {
                    background: Some(Tokens::of(theme).overlay.into()),
                    ..container::Style::default()
                }),
        )
        .on_press(Message::CloseModal);

        // A `Stack` sizes from its first child, so a covering layer needs the
        // explicit fill the module comment names.
        Stack::new()
            .push(backdrop)
            .push(centered)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    } else {
        Space::new().into()
    };

    // section: end
    let status = text(match (app.modal_open, app.note) {
        (true, _) => {
            "The modal is open. Escape, the backdrop and Cancel all dismiss it.".to_string()
        }
        (false, Some(note)) => note.to_string(),
        (false, None) => {
            "The regions draw; iced 0.14 has no focus scope, so Tab still leaves them.".to_string()
        }
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
        custom_modal,
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

    Stack::new().push(page).push(modal_layer).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
