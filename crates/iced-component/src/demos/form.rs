//! `form` — layout and validation over the controls beside it, shown the way the
//! Slint example and the component page show it: one form composed by hand, the
//! basic, horizontal, multi-column, registration and settings forms, the
//! container and its label layouts, required fields with an error and a
//! description, submission and its action row, groups of related and conditional
//! fields, and the grid with its spans, positioning and responsive column
//! choice.
//!
//! Run it with `cargo run --example form`, or in the browser as `?story=form` in
//! the gallery the website embeds.
//!
//! Each section carries a heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one.
//!
//! The section titles are the component page's own `##` headings — the page's
//! vocabulary for the container, the layout, validation, submission and the grid
//! — rather than the Slint example's individual `GroupBox` titles. The Slint
//! example groups the same ground under twelve box headings; the page groups it
//! under seven, and the page is what the code icon has to land on.
//!
//! The form owns no value, so every field here reports to this application and
//! the next frame draws what it stored: a field's `error` is a string the
//! application sets when it validates and clears when the value is corrected.
//! The registration form is the one that runs the whole loop — press **Create
//! account**, the email and password fields take their error lines, and typing a
//! valid value clears the one beside it.

use iced::widget::text_editor::{Action, Content};
use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::{self, Button};
use crate::components::checkbox::Checkbox;
use crate::components::form::{Field, Form, LabelLayout};
use crate::components::group_box::{self, GroupBox};
use crate::components::input::Input;
use crate::components::menu::Item;
use crate::components::number_input::NumberInput;
use crate::components::select::Select;
use crate::components::slider::Slider;
use crate::components::switch::Switch;
use crate::components::textarea::Textarea;
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The registration form's rule: the address has to look like one, and the
/// password has to be long enough. iced validates nothing by itself, so this is
/// the application's check and the application's messages.
const MIN_PASSWORD: usize = 8;

/// Where the responsive sample stacks its two columns. The column count is the
/// application's choice in both references — GPUI's page calls the same idea
/// `is_mobile` — because iced offers no viewport token for the form to read.
const NARROW: f32 = 720.0;

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — form")
        .theme(theme)
        .subscription(subscription)
        .run()
}

/// Watch the window's width so the responsive sample can choose its columns,
/// which is the one thing on this page the component does not decide for the
/// caller.
fn subscription(_app: &App) -> iced::Subscription<Message> {
    iced::window::resize_events()
        .map(|(_id, size): (iced::window::Id, iced::Size)| Message::Resized(size.width < NARROW))
}

#[derive(Default)]
struct App {
    /// One value per field, because a `Form` and its `Field`s hold none: the
    /// caller passes the value in, takes the requested one back and stores it.
    name: String,
    email: String,
    bio: Content,

    /// The registration form, its errors and its result.
    reg_first: String,
    reg_last: String,
    reg_email: String,
    reg_password: String,
    reg_confirm: String,
    reg_terms: bool,
    reg_error_email: String,
    reg_error_password: String,
    reg_error_terms: String,
    reg_status: String,

    /// The container and layout examples.
    h_first: String,
    h_last: String,
    sizing_title: String,
    sizing_code: String,

    /// The validation section: one field the Validate button checks, and the
    /// state that decides whether the optional field is drawn.
    v_email: String,
    v_error: String,
    v_account: Option<usize>,
    v_company: String,
    v_tax_id: String,

    /// The submission section.
    s_name: String,
    s_email: String,
    s_status: String,

    /// The field-group examples.
    g_first: String,
    g_last: String,
    g_street: String,
    g_city: String,
    g_zip: String,
    g_notify: bool,
    g_volume: f32,
    g_quantity: f64,

    /// The grid examples.
    grid_a: String,
    grid_b: String,
    grid_c: String,
    grid_full: String,
    grid_positioned: String,

    /// The responsive sample: its two values, and the window width that decides
    /// whether it draws one column or two.
    resp_name: String,
    resp_email: String,
    narrow: bool,

    /// Which select's panel is open, which row is chosen, and the row under the
    /// pointer — the select owns none of them.
    select_open: bool,

    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Name(String),
    Email(String),
    Bio(Action),

    RegFirst(String),
    RegLast(String),
    RegEmail(String),
    RegPassword(String),
    RegConfirm(String),
    RegTerms(bool),
    RegCreate,

    HFirst(String),
    HLast(String),
    SizingTitle(String),
    SizingCode(String),

    VEmail(String),
    VValidate,
    VAccount(usize),
    VCompany(String),
    VTaxId(String),

    SName(String),
    SEmail(String),
    SSubmit,

    GFirst(String),
    GLast(String),
    GStreet(String),
    GCity(String),
    GZip(String),
    GNotify(bool),
    GVolume(f32),
    GQuantity(f64),

    GridA(String),
    GridB(String),
    GridC(String),
    GridFull(String),
    GridPositioned(String),

    RespName(String),
    RespEmail(String),
    /// The window was resized: is it narrower than the responsive breakpoint?
    Resized(bool),

    ComposeSave,
    ComposeCancel,

    SelectOpen(bool),

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
        name: String::from("Ada Lovelace"),
        reg_first: String::from("Ada"),
        reg_last: String::from("Lovelace"),
        g_notify: true,
        g_volume: 60.0,
        g_quantity: 1.0,
        v_account: Some(0),
        ..App::default()
    }
}

/// Whether an address looks like one, for the registration form's rule.
fn looks_like_email(value: &str) -> bool {
    let mut parts = value.split('@');
    let local = parts.next().unwrap_or_default();
    let domain = parts.next().unwrap_or_default();
    !local.is_empty() && domain.contains('.') && parts.next().is_none()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Name(value) => app.name = value,
        Message::Email(value) => app.email = value,
        Message::Bio(action) => {
            app.bio.perform(action);
        }
        Message::RegFirst(value) => app.reg_first = value,
        Message::RegLast(value) => app.reg_last = value,
        Message::RegEmail(value) => {
            // Correcting the value clears the message that named it, which is the
            // other half of the validation loop.
            if looks_like_email(&value) {
                app.reg_error_email.clear();
            }
            app.reg_email = value;
        }
        Message::RegPassword(value) => {
            if value.chars().count() >= MIN_PASSWORD {
                app.reg_error_password.clear();
            }
            app.reg_password = value;
        }
        Message::RegConfirm(value) => app.reg_confirm = value,
        Message::RegTerms(value) => {
            if value {
                app.reg_error_terms.clear();
            }
            app.reg_terms = value;
        }
        Message::RegCreate => {
            app.reg_error_email.clear();
            app.reg_error_password.clear();
            app.reg_error_terms.clear();
            if !looks_like_email(&app.reg_email) {
                app.reg_error_email = String::from("Enter a valid email address.");
            }
            if app.reg_password.chars().count() < MIN_PASSWORD {
                app.reg_error_password =
                    format!("Use at least {MIN_PASSWORD} characters, so the account stays yours.");
            }
            if !app.reg_terms {
                app.reg_error_terms = String::from("Accept the terms to continue.");
            }
            app.reg_status = if app.reg_error_email.is_empty()
                && app.reg_error_password.is_empty()
                && app.reg_error_terms.is_empty()
            {
                String::from("Account created.")
            } else {
                String::new()
            };
        }

        Message::HFirst(value) => app.h_first = value,
        Message::HLast(value) => app.h_last = value,
        Message::SizingTitle(value) => app.sizing_title = value,
        Message::SizingCode(value) => app.sizing_code = value,

        Message::VEmail(value) => {
            if looks_like_email(&value) {
                app.v_error.clear();
            }
            app.v_email = value;
        }
        Message::VValidate => {
            app.v_error = if looks_like_email(&app.v_email) {
                String::new()
            } else {
                String::from("Enter a valid email address.")
            };
        }
        Message::VAccount(index) => app.v_account = Some(index),
        Message::VCompany(value) => app.v_company = value,
        Message::VTaxId(value) => app.v_tax_id = value,

        Message::SName(value) => app.s_name = value,
        Message::SEmail(value) => app.s_email = value,
        Message::SSubmit => {
            app.s_status = if app.s_name.is_empty() || app.s_email.is_empty() {
                String::from("Name and email are both required.")
            } else {
                String::from("Ready to submit.")
            };
        }

        Message::GFirst(value) => app.g_first = value,
        Message::GLast(value) => app.g_last = value,
        Message::GStreet(value) => app.g_street = value,
        Message::GCity(value) => app.g_city = value,
        Message::GZip(value) => app.g_zip = value,
        Message::GNotify(value) => app.g_notify = value,
        Message::GVolume(value) => app.g_volume = value,
        Message::GQuantity(value) => app.g_quantity = value,

        Message::GridA(value) => app.grid_a = value,
        Message::GridB(value) => app.grid_b = value,
        Message::GridC(value) => app.grid_c = value,
        Message::GridFull(value) => app.grid_full = value,
        Message::GridPositioned(value) => app.grid_positioned = value,

        Message::RespName(value) => app.resp_name = value,
        Message::RespEmail(value) => app.resp_email = value,
        Message::Resized(narrow) => app.narrow = narrow,

        Message::SelectOpen(open) => app.select_open = open,
        Message::ComposeSave => {
            app.s_status = String::from("Saved.");
        }
        Message::ComposeCancel => {
            app.s_status = String::from("Cancelled.");
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

/// A muted line under an example, carrying what the demo has to say about it.
fn muted_line<'a>(line: &str) -> Element<'a, Message> {
    text(line.to_string())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon is
/// the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and 0.55
/// on hover, 0.34 and 0.7 while pressed.
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

/// The box under a heading: the same `GroupBox` with the outline variant the Slint
/// example wraps its sections in, untitled because the heading above it carries the
/// title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Predictable composition
    // One form built the way the page builds it: the label beside the control,
    // two field columns, and the commit row in `footer`.
    let composed = Form::new()
        .label_layout(LabelLayout::Beside)
        .label_width(96.0)
        .columns(2)
        .gap(12.0)
        .child(
            Field::new().label("Name").child(
                Input::new("pc-name")
                    .value(app.name.clone())
                    .placeholder("Ada Lovelace")
                    .on_input(Message::Name),
            ),
        )
        .child(
            Field::new().label("Email").required(true).child(
                Input::new("pc-email")
                    .value(app.email.clone())
                    .placeholder("ada@example.com")
                    .on_input(Message::Email),
            ),
        )
        .footer(
            row![
                Button::new("Cancel")
                    .outline()
                    .on_press(Message::ComposeCancel),
                Button::new("Save")
                    .variant(button::Variant::Primary)
                    .on_press(Message::ComposeSave),
            ]
            .spacing(8.0),
        );

    // section: Usage
    // The basic form: one column, labels above their controls.
    let basic = Form::vertical()
        .child(
            Field::new().label("Name").child(
                Input::new("use-name")
                    .value(app.name.clone())
                    .on_input(Message::Name),
            ),
        )
        .child(
            Field::new().label("Email").required(true).child(
                Input::new("use-email")
                    .value(app.email.clone())
                    .on_input(Message::Email),
            ),
        )
        .child(
            Field::new().label("Bio").child(
                Textarea::new(&app.bio)
                    .placeholder("Tell us about yourself")
                    .id("use-bio")
                    .on_action(Message::Bio),
            ),
        );

    // The horizontal form: the same fields with the labels beside them.
    let horizontal = Form::horizontal()
        .label_width(120.0)
        .child(
            Field::new().label("First Name").child(
                Input::new("h-first")
                    .value(app.h_first.clone())
                    .placeholder("Ada")
                    .on_input(Message::HFirst),
            ),
        )
        .child(
            Field::new().label("Last Name").child(
                Input::new("h-last")
                    .value(app.h_last.clone())
                    .placeholder("Lovelace")
                    .on_input(Message::HLast),
            ),
        );

    // The multi-column form: two field columns, and the Bio field spanning both.
    let multi = Form::vertical()
        .columns(2)
        .child(
            Field::new().label("First Name").child(
                Input::new("m-first")
                    .value(app.h_first.clone())
                    .placeholder("Ada")
                    .on_input(Message::HFirst),
            ),
        )
        .child(
            Field::new().label("Last Name").child(
                Input::new("m-last")
                    .value(app.h_last.clone())
                    .placeholder("Lovelace")
                    .on_input(Message::HLast),
            ),
        )
        .child(
            Field::new().label("Bio").col_span(2).child(
                Textarea::new(&app.bio)
                    .rows(3)
                    .placeholder("Tell us about yourself")
                    .id("m-bio")
                    .on_action(Message::Bio),
            ),
        );

    // The registration form: the one example that runs validation end to end.
    // **Create account** checks the email and the password and writes the strings
    // their fields show; typing a valid value clears the one it corrects.
    let registration = Form::vertical()
        .child(
            Field::new().label("Personal Information").child(
                row![
                    Input::new("reg-first")
                        .value(app.reg_first.clone())
                        .placeholder("First name")
                        .on_input(Message::RegFirst),
                    Input::new("reg-last")
                        .value(app.reg_last.clone())
                        .placeholder("Last name")
                        .on_input(Message::RegLast),
                ]
                .spacing(8.0),
            ),
        )
        .child(
            Field::new()
                .label("Email")
                .required(true)
                .error(app.reg_error_email.clone())
                .child(
                    Input::new("reg-email")
                        .value(app.reg_email.clone())
                        .invalid(!app.reg_error_email.is_empty())
                        .placeholder("you@example.com")
                        .on_input(Message::RegEmail),
                ),
        )
        .child(
            Field::new()
                .label("Password")
                .required(true)
                .description("Must be at least 8 characters")
                .error(app.reg_error_password.clone())
                .child(
                    Input::new("reg-password")
                        .value(app.reg_password.clone())
                        .secure(true)
                        .invalid(!app.reg_error_password.is_empty())
                        .placeholder("Password")
                        .on_input(Message::RegPassword),
                ),
        )
        .child(
            Field::new().label("Confirm Password").required(true).child(
                Input::new("reg-confirm")
                    .value(app.reg_confirm.clone())
                    .secure(true)
                    .placeholder("Confirm password")
                    .on_input(Message::RegConfirm),
            ),
        )
        .child(
            Field::new()
                .label_indent(false)
                .error(app.reg_error_terms.clone())
                .child(
                    Checkbox::new(app.reg_terms)
                        .label("I agree to the Terms of Service")
                        .on_toggle(Message::RegTerms),
                ),
        )
        .child(
            Field::new().label_indent(false).child(
                Button::new("Create account")
                    .variant(button::Variant::Primary)
                    .on_press(Message::RegCreate),
            ),
        );

    // The settings form: `Field::section` is the heading over a group and
    // `Field::separator` the rule under it, which is Slint's `FormSection` and
    // `FormSeparator`.
    let settings = Form::vertical()
        .child(Field::section("Profile").description("How you appear to others."))
        .child(Field::separator())
        .child(
            Field::new().label("Display Name").child(
                Input::new("set-display")
                    .value(app.name.clone())
                    .placeholder("Ada Lovelace")
                    .on_input(Message::Name),
            ),
        )
        .child(
            Field::new().label("Email").child(
                Input::new("set-email")
                    .value(app.email.clone())
                    .placeholder("ada@example.com")
                    .on_input(Message::Email),
            ),
        )
        .child(Field::section("Preferences").description("Choose how the app behaves."))
        .child(Field::separator())
        .child(
            Field::new().label("Notifications").child(
                Switch::new(app.g_notify)
                    .label("Enable notifications")
                    .on_toggle(Message::GNotify),
            ),
        );

    let usage = column![
        basic,
        Field::separator(),
        horizontal,
        Field::separator(),
        multi,
        Field::separator(),
        registration,
        muted_line(app.reg_status.as_str()),
        settings,
    ]
    .spacing(16.0);

    // section: Form Container and Layout
    // The vertical default, the horizontal label layout, and the size ladder.
    let vertical = Form::vertical()
        .gap(12.0)
        .child(
            Field::new().label("Name").child(
                Input::new("vl-name")
                    .value(app.name.clone())
                    .on_input(Message::Name),
            ),
        )
        .child(
            Field::new().label("Email").child(
                Input::new("vl-email")
                    .value(app.email.clone())
                    .on_input(Message::Email),
            ),
        );

    let layout_horizontal = Form::horizontal()
        .label_width(100.0)
        .child(
            Field::new().label("Name").child(
                Input::new("lh-name")
                    .value(app.name.clone())
                    .on_input(Message::Name),
            ),
        )
        .child(
            Field::new().label("Email").child(
                Input::new("lh-email")
                    .value(app.email.clone())
                    .on_input(Message::Email),
            ),
        );

    let large = Form::vertical().size(ControlSize::Large).child(
        Field::new().label("Title").child(
            Input::new("sz-large")
                .size(ControlSize::Large)
                .value(app.sizing_title.clone())
                .placeholder("Large field")
                .on_input(Message::SizingTitle),
        ),
    );

    let small = Form::vertical().size(ControlSize::Small).child(
        Field::new().label("Code").child(
            Input::new("sz-small")
                .size(ControlSize::Small)
                .value(app.sizing_code.clone())
                .placeholder("Small field")
                .on_input(Message::SizingCode),
        ),
    );

    let container_layout = column![vertical, layout_horizontal, large, small].spacing(20.0);

    // section: Form Validation
    // `required` draws the asterisk; `error` places the message, and the
    // description steps aside while one is set. Both are the application's: the
    // Validate button writes the string the field shows.
    let required = Form::vertical()
        .child(
            Field::new()
                .label("Email")
                .required(true)
                .error(app.v_error.clone())
                .child(
                    Input::new("val-email")
                        .value(app.v_email.clone())
                        .invalid(!app.v_error.is_empty())
                        .placeholder("you@example.com")
                        .on_input(Message::VEmail),
                ),
        )
        .child(
            Field::new()
                .label("Password")
                .required(true)
                .description("Must be at least 8 characters long")
                .child(
                    Input::new("val-password")
                        .secure(true)
                        .placeholder("Password"),
                ),
        )
        .footer(
            Button::new("Validate")
                .variant(button::Variant::Primary)
                .on_press(Message::VValidate),
        );

    let dynamic = Form::vertical().child(
        Field::new()
            .label("Bio")
            .description(if app.bio.text().chars().count() > 40 {
                "That is plenty — shorter reads better."
            } else {
                "Use at most 100 words to describe yourself."
            })
            .child(
                Textarea::new(&app.bio)
                    .rows(3)
                    .id("val-bio")
                    .on_action(Message::Bio),
            ),
    );

    let conditional = Form::vertical()
        .child(
            Field::new().label("Account Type").child(
                Select::new([Item::new("Personal"), Item::new("Business")])
                    .selected(app.v_account)
                    .open(app.select_open)
                    .width(Length::Fill)
                    .on_open_change(Message::SelectOpen)
                    .on_chosen(Message::VAccount),
            ),
        )
        .child(
            Field::new()
                .label("Company Name")
                .visible(app.v_account == Some(1))
                .child(
                    Input::new("val-company")
                        .value(app.v_company.clone())
                        .placeholder("Acme Inc.")
                        .on_input(Message::VCompany),
                ),
        )
        .child(
            Field::new()
                .label("Tax ID")
                .required(true)
                .visible(app.v_account == Some(1))
                .child(
                    Input::new("val-tax")
                        .value(app.v_tax_id.clone())
                        .placeholder("US-0000000")
                        .on_input(Message::VTaxId),
                ),
        );

    let validation = column![required, dynamic, conditional].spacing(20.0);

    // section: Submit Handling
    // The form does not submit; the button reports, the application validates and
    // the status line under the form is what changed.
    let submit = Form::vertical()
        .child(
            Field::new().label("Name").child(
                Input::new("sub-name")
                    .value(app.s_name.clone())
                    .on_input(Message::SName),
            ),
        )
        .child(
            Field::new().label("Email").child(
                Input::new("sub-email")
                    .value(app.s_email.clone())
                    .on_input(Message::SEmail),
            ),
        )
        .child(
            Field::new().label_indent(false).child(
                Button::new("Submit")
                    .variant(button::Variant::Primary)
                    .on_press(Message::SSubmit),
            ),
        );

    let actions = Form::vertical()
        .child(
            Field::new()
                .label("Title")
                .child(Input::new("act-title").placeholder("Release notes")),
        )
        .child(
            Field::new()
                .label("Content")
                .child(Input::new("act-content").placeholder("What changed?")),
        )
        .footer(
            row![
                Button::new("Save").variant(button::Variant::Primary),
                Button::new("Cancel").outline(),
                Button::new("Preview").variant(button::Variant::Ghost),
            ]
            .spacing(8.0),
        );

    let submission = column![submit, muted_line(app.s_status.as_str()), actions].spacing(20.0);

    // section: Field Groups
    // One label over a row of two controls; one label over a stack; and the
    // controls that are not text fields at all.
    let related = Form::vertical()
        .child(
            Field::new().label("Name").child(
                row![
                    Input::new("grp-first")
                        .value(app.g_first.clone())
                        .placeholder("First name")
                        .on_input(Message::GFirst),
                    Input::new("grp-last")
                        .value(app.g_last.clone())
                        .placeholder("Last name")
                        .on_input(Message::GLast),
                ]
                .spacing(8.0),
            ),
        )
        .child(
            Field::new().label("Address").child(
                column![
                    Input::new("grp-street")
                        .value(app.g_street.clone())
                        .placeholder("Street address")
                        .on_input(Message::GStreet),
                    row![
                        Input::new("grp-city")
                            .value(app.g_city.clone())
                            .placeholder("City")
                            .on_input(Message::GCity),
                        Input::new("grp-zip")
                            .value(app.g_zip.clone())
                            .placeholder("ZIP")
                            .width(120.0)
                            .on_input(Message::GZip),
                    ]
                    .spacing(8.0),
                ]
                .spacing(8.0),
            ),
        );

    let custom = Form::vertical()
        .child(
            Field::new().label("Notifications").child(
                Switch::new(app.g_notify)
                    .label("Email notifications")
                    .on_toggle(Message::GNotify),
            ),
        )
        .child(
            Field::new().label("Volume").child(
                Slider::new(app.g_volume)
                    .minimum(0.0)
                    .maximum(100.0)
                    .on_change(Message::GVolume),
            ),
        )
        .child(
            Field::new().label("Quantity").child(
                NumberInput::new("grp-qty")
                    .value(app.g_quantity)
                    .minimum(1.0)
                    .maximum(20.0)
                    .on_change(Message::GQuantity),
            ),
        );

    let groups = column![related, custom].spacing(20.0);

    // section: Grid Layout and Positioning
    // Three columns, a field spanning all three, and a field placed at a column
    // of its own.
    let spanning = Form::vertical()
        .columns(3)
        .child(
            Field::new().label("First").child(
                Input::new("g-a")
                    .value(app.grid_a.clone())
                    .placeholder("One")
                    .on_input(Message::GridA),
            ),
        )
        .child(
            Field::new().label("Second").child(
                Input::new("g-b")
                    .value(app.grid_b.clone())
                    .placeholder("Two")
                    .on_input(Message::GridB),
            ),
        )
        .child(
            Field::new().label("Third").child(
                Input::new("g-c")
                    .value(app.grid_c.clone())
                    .placeholder("Three")
                    .on_input(Message::GridC),
            ),
        )
        .child(
            Field::new().label("Full Width").col_span(3).child(
                Input::new("g-full")
                    .value(app.grid_full.clone())
                    .placeholder("Spans all three columns")
                    .on_input(Message::GridFull),
            ),
        );

    let positioning = Form::vertical()
        .columns(4)
        .child(
            Field::new()
                .label("A")
                .child(Input::new("g-p1").placeholder("First")),
        )
        .child(
            Field::new()
                .label("B")
                .child(Input::new("g-p2").placeholder("Second")),
        )
        .child(
            Field::new()
                .label("Positioned")
                .col_start(1)
                .col_span(2)
                .child(
                    Input::new("g-pos")
                        .value(app.grid_positioned.clone())
                        .placeholder("Columns two and three")
                        .on_input(Message::GridPositioned),
                ),
        );

    // The page's Responsive Layout sample: the application chooses the column
    // count from the window, and the component clamps the Bio field's span to
    // whatever the count is, exactly as the page's `.when(!is_mobile, …)` does.
    let responsive = Form::vertical()
        .columns(if app.narrow { 1 } else { 2 })
        .child(
            Field::new().label("Name").child(
                Input::new("resp-name")
                    .value(app.resp_name.clone())
                    .placeholder("Ada Lovelace")
                    .on_input(Message::RespName),
            ),
        )
        .child(
            Field::new().label("Email").child(
                Input::new("resp-email")
                    .value(app.resp_email.clone())
                    .placeholder("ada@example.com")
                    .on_input(Message::RespEmail),
            ),
        )
        .child(
            Field::new().label("Bio").col_span(2).child(
                Textarea::new(&app.bio)
                    .rows(3)
                    .placeholder("Tell us about yourself")
                    .id("resp-bio")
                    .on_action(Message::Bio),
            ),
        );

    let grid = column![spanning, positioning, responsive].spacing(20.0);

    // section: end
    let status =
        text("Click a control, or a heading and its code icon — the page scrolls to that sample.");

    let gallery = column![
        heading(app, 0, "Predictable composition"),
        body_box(composed),
        heading(app, 1, "Usage"),
        body_box(usage),
        heading(app, 2, "Form Container and Layout"),
        body_box(container_layout),
        heading(app, 3, "Form Validation"),
        body_box(validation),
        heading(app, 4, "Submit Handling"),
        body_box(submission),
        heading(app, 5, "Field Groups"),
        body_box(groups),
        heading(app, 6, "Grid Layout and Positioning"),
        body_box(grid),
        status,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
