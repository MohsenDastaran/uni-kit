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
//! The page has seven `###` samples. Three are drawn here, and the trap itself
//! is now one of them: `Tab` and `Shift+Tab` are routed from the subscription
//! below to `focus_trap::tab`, which reads the focused id, steps through the
//! region that holds it and wraps at the ends. The modal opens on its first
//! field and returns focus to whatever held it before, when there was one. The
//! modal is still a layer in one [`Stack`](iced::widget::Stack) over the
//! backdrop the caller writes; the trap is the region inside it. The
//! other four samples are still named rather than approximated: **Focus Trap
//! with Dialog** and **Focus Trap with Sheet** would show a trap those
//! components do not have (their own module comments say so), **Nested Focus
//! Traps** would need the innermost region to win, which a flat list of regions
//! does not express, and **Conditional Focus Trap** gates a scope iced does not
//! have.
//!
//! The key is still the application's, because iced 0.14 binds no key to focus
//! movement: nothing but `combo_box` names `Tab`. So this file owns the open
//! flag and the field values, routes `Tab`/`Shift+Tab`/Escape from the
//! subscription, and remembers what held focus when the modal opened; the
//! region and its arithmetic live in `components/focus_trap.rs`.

use iced::keyboard;
use iced::widget::{
    Id, Space, Stack, column, container, mouse_area, operation, row, scrollable, svg, text,
};
use iced::{Alignment, Element, Length, Shadow, Subscription, Task, Theme, Vector};

use crate::components::button::{self, Button};
use crate::components::focus_trap::{self, FocusTrap, Style};
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

/// The fields each region traps, in reading order. `Id::new` is `const`, so the
/// view can name a field on its region and the `Tab` handler can walk the same
/// id, and the trap cannot drift from the fields it guards. A button is not
/// focusable in iced 0.14, so an action is never in one of these lists.
const BASIC_FIELDS: [Id; 2] = [Id::new("first"), Id::new("second")];

/// The one focusable of the first of the side-by-side regions.
const AREA_ONE_FIELDS: [Id; 1] = [Id::new("area-one")];

/// The one focusable of the second of the side-by-side regions.
const AREA_TWO_FIELDS: [Id; 1] = [Id::new("area-two")];

/// The modal's fields: the workspace name to confirm, and the reason.
const MODAL_FIELDS: [Id; 2] = [Id::new("modal-name"), Id::new("modal-reason")];

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
    /// The custom modal's two fields.
    modal_name: String,
    modal_reason: String,
    /// Whether the custom modal's layer is showing.
    modal_open: bool,
    /// What held focus before the modal opened, so closing can return it. iced
    /// reports focus to no one, so this is read with `focus_trap::focused` at
    /// the moment the layer opens.
    previous_focus: Option<Id>,
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
    ModalName(String),
    ModalReason(String),
    Submitted,
    OpenModal,
    CloseModal,
    /// The focus report the modal asked for as it opened: what to restore when
    /// it closes. `None` when nothing was focused.
    OpenedFrom(Option<Id>),
    /// `Tab` or `Shift+Tab`: move focus inside the region that holds it.
    Tab { backwards: bool },
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
        modal_name: String::new(),
        modal_reason: String::new(),
        modal_open: false,
        previous_focus: None,
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
        Message::ModalName(value) => app.modal_name = value,
        Message::ModalReason(value) => app.modal_reason = value,
        Message::Submitted => {
            app.note = Some("Submit received. The region drew a press; Tab is what moves it.");
        }
        Message::OpenModal => {
            app.modal_open = true;
            app.note = None;
            // Ask what holds focus before the layer takes it, so closing can
            // return it. The answer arrives as `OpenedFrom`.
            return focus_trap::focused().map(Message::OpenedFrom);
        }
        Message::OpenedFrom(from) => {
            app.previous_focus = from;
            // Move focus into the trap, so the first Tab is already inside it.
            return focus_trap::enter(&MODAL_FIELDS);
        }
        Message::CloseModal => {
            app.modal_open = false;
            return restore(app);
        }
        // Tab and Shift+Tab are routed here by the subscription. While the
        // modal is open its region is the only one active, so focus is pulled
        // into it; otherwise the three regions on the page each hold their own.
        Message::Tab { backwards } => {
            return if app.modal_open {
                focus_trap::tab(&[&MODAL_FIELDS], backwards)
            } else {
                focus_trap::tab(
                    &[&BASIC_FIELDS, &AREA_ONE_FIELDS, &AREA_TWO_FIELDS],
                    backwards,
                )
            };
        }
        Message::StayOpen => {}
        // iced hands a widget no focus operation, so Escape is the
        // application's — the same binding `components/focus_trap.rs` names.
        Message::Escape => {
            if app.modal_open {
                app.modal_open = false;
                app.note = Some("Escape closed the modal; the application heard the key.");
                return restore(app);
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

/// Return focus to whatever held it before the modal opened — the move a trap
/// makes when it closes. iced reports no focus to the application, so the id
/// was read with [`focus_trap::focused`] as the layer opened; a modal opened
/// with a press usually has nothing to return to, because that press already
/// unfocused every field.
fn restore(app: &mut App) -> Task<Message> {
    match app.previous_focus.take() {
        Some(id) => operation::focus(id),
        None => Task::none(),
    }
}

/// The keys iced leaves unbound: `Tab`, `Shift+Tab` and Escape. iced moves
/// focus for no key at all, so the trap's driver is this subscription.
///
/// It listens through [`event::listen_with`](iced::event::listen_with) rather
/// than `keyboard::listen`, because a focused field consumes Escape to leave
/// the field and `keyboard::listen` only reports **ignored** events. The
/// subscription has to hear the key the field swallowed, or the modal would
/// never dismiss from the keyboard.
fn subscription(_app: &App) -> Subscription<Message> {
    iced::event::listen_with(|event, _status, _window| match event {
        iced::Event::Keyboard(keyboard::Event::KeyPressed {
            key, modifiers, ..
        }) => match key.as_ref() {
            keyboard::Key::Named(keyboard::key::Named::Escape) => Some(Message::Escape),
            keyboard::Key::Named(keyboard::key::Named::Tab) => Some(Message::Tab {
                backwards: modifiers.shift(),
            }),
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
    // reference's 12px gap inside its 16px inset and one-pixel edge. The region
    // names both fields, so `Tab` wraps second -> first inside it.
    let basic = body_box(
        column![
            FocusTrap::new()
                .focusable(BASIC_FIELDS[0].clone())
                .child(
                    Input::new(BASIC_FIELDS[0].clone())
                        .placeholder("First field")
                        .value(app.first.clone())
                        .on_input(Message::First),
                )
                .focusable(BASIC_FIELDS[1].clone())
                .child(
                    Input::new(BASIC_FIELDS[1].clone())
                        .placeholder("Second field")
                        .value(app.second.clone())
                        .on_input(Message::Second),
                )
                .child(Button::new("Submit").on_press(Message::Submitted)),
            muted_line(
                "The frame, the gap and the inset are the reference's. iced binds no key, so \
                 the subscription above drives Tab between these two fields and wraps it here.",
            ),
        ]
        .spacing(8.0),
    );

    // section: Multiple Focus Traps
    // Two regions sharing a row, each naming its own field. The region that
    // holds the focused id is the one `Tab` steps through, so the two traps
    // stay independent; a button is not focusable in iced, so each one holds
    // its single field.
    let multiple = body_box(
        column![
            row![
                FocusTrap::new()
                    .focusable(AREA_ONE_FIELDS[0].clone())
                    .child(
                        Input::new(AREA_ONE_FIELDS[0].clone())
                            .placeholder("Area 1")
                            .value(app.area_one.clone())
                            .on_input(Message::AreaOne),
                    )
                    .child(Button::new("Continue").on_press(Message::Submitted)),
                FocusTrap::new()
                    .focusable(AREA_TWO_FIELDS[0].clone())
                    .child(
                        Input::new(AREA_TWO_FIELDS[0].clone())
                            .placeholder("Area 2")
                            .value(app.area_two.clone())
                            .on_input(Message::AreaTwo),
                    )
                    .child(Button::new("Cancel").on_press(Message::Submitted)),
            ]
            .spacing(GAP),
            muted_line(
                "Two traps, side by side. Each holds its own field, so Tab stays in the region \
                 it started in; a button is not focusable in iced 0.14."
            ),
        ]
        .spacing(8.0),
    );

    // section: Custom Modal with Focus Trap
    // The page's own custom-modal shape: a dimmed full-size backdrop with the
    // region centered on it, here turned into a panel by the style closure. Its
    // two fields are what the trap holds, and `Tab` cycles them rather than
    // reaching the page behind.
    let custom_modal = body_box(
        column![
            Button::new("Open Modal").on_press(Message::OpenModal),
            muted_line(
                "A custom modal: the region is the panel, over a dimmed backdrop, and Tab \
                 cycles its two fields."
            ),
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
            text("Type the workspace name to confirm. Escape and the backdrop both dismiss it.")
                .size(theme::FONT_SIZE_SM)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        )
        .focusable(MODAL_FIELDS[0].clone())
        .child(
            Input::new(MODAL_FIELDS[0].clone())
                .placeholder("Workspace name")
                .value(app.modal_name.clone())
                .on_input(Message::ModalName),
        )
        .focusable(MODAL_FIELDS[1].clone())
        .child(
            Input::new(MODAL_FIELDS[1].clone())
                .placeholder("Reason")
                .value(app.modal_reason.clone())
                .on_input(Message::ModalReason),
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
            "The modal is open. Tab cycles its two fields; Escape, the backdrop and Cancel \
             dismiss it."
                .to_string()
        }
        (false, Some(note)) => note.to_string(),
        (false, None) => {
            "Tab is the subscription's and the trap's: it cycles the fields of whichever \
             region holds focus."
                .to_string()
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
