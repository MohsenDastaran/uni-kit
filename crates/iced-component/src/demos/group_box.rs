//! `group-box` (module `group_box`) — a heading over content, in the plain,
//! filled and outlined shapes.
//!
//! Run it with `cargo run --example group_box`, or in the browser as
//! `?story=group_box` in the gallery the website embeds.
//!
//! The sections are the Slint example's `GroupBox`es, in its order, each titled
//! with the `###` heading of `website/component/group-box.md` that documents it.
//! The page has four more `###` headings than the Slint example has boxes —
//! *Custom ID*, *Complex Example*, *Form Section* and *Subscription
//! Management* — and all four are GPUI-only: the Slint example has no `GroupBox`
//! for them, so this port has no section for them and the page hides those four
//! under iced rather than showing an empty panel.
//!
//! Each section carries the heading the component page has — the title, and the
//! code icon that asks the embedding page to scroll to that sample. Clicking
//! either one posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one — without it the lines that assemble the
//! gallery would be shown as part of the last section.

use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::{Button, Variant};
use crate::components::checkbox::Checkbox;
use crate::components::group_box::{self, GroupBox};
use crate::components::radio::RadioGroup;
use crate::components::switch::Switch;
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's `###` headings — have them. The index is what the page is
/// asked to scroll to.
const SECTIONS: [&str; 8] = [
    "Account Settings",
    "Preferences",
    "GroupBox Variants",
    "With Title",
    "Footer outside the surface",
    "Custom Title Styling",
    "Custom Content Styling",
    "Without Title",
];

pub fn run(theme: Theme) -> iced::Result {
    // The palette in force at boot is the one the Custom Title Styling sample
    // names an explicit title colour from. Embedded, the gallery has already
    // parked the page's own palette, so this is the page's set; native, and
    // standalone in a browser, it is the built-in light or dark table.
    let tokens = Tokens::of(&theme);
    iced::application(move || boot(tokens), update, view)
        .title("uni-kit — group-box")
        .theme(theme)
        .run()
}

#[derive(Debug)]
struct App {
    private: bool,
    contributions: bool,
    all_activity: bool,
    product_updates: bool,
    account_activity: bool,
    appearance: Option<usize>,
    /// The palette the Custom Title Styling sample draws its title colour from.
    tokens: Tokens,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Private(bool),
    Contributions(bool),
    AllActivity(bool),
    ProductUpdates(bool),
    AccountActivity(bool),
    Appearance(usize),
    /// A button with nothing to report yet.
    Nothing,
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot(tokens: Tokens) -> App {
    App {
        private: true,
        contributions: false,
        all_activity: true,
        product_updates: true,
        account_activity: false,
        appearance: Some(2),
        tokens,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Private(checked) => app.private = checked,
        Message::Contributions(checked) => app.contributions = checked,
        Message::AllActivity(checked) => app.all_activity = checked,
        Message::ProductUpdates(checked) => app.product_updates = checked,
        Message::AccountActivity(checked) => app.account_activity = checked,
        Message::Appearance(index) => app.appearance = Some(index),
        Message::Nothing => {}
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

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A label that takes the room and the control that acts on it at the trailing
/// edge — the shape the Slint example gives most of its groups.
fn setting<'a>(
    label: &'static str,
    control: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    row![
        text(label).size(theme::FONT_SIZE_SM).width(Length::Fill),
        control.into(),
    ]
    .spacing(theme::SPACING_MD)
    .align_y(Alignment::Center)
    .into()
}

/// One of the three surfaces, side by side in the GroupBox Variants sample.
fn variant_panel<'a>(variant: group_box::Variant, copy: &'static str) -> Element<'a, Message> {
    GroupBox::new()
        .variant(variant)
        .body(text(copy).size(theme::FONT_SIZE_SM))
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
        .spacing(theme::SPACING_SM)
        .align_y(Alignment::Center)
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Account Settings
    // A filled group, its label taking the room and the switch at the trailing
    // edge, then the primary action: the Slint example's first `GroupBox`.
    let account = body_box(
        GroupBox::new()
            .title("Account Settings")
            .variant(group_box::Variant::Fill)
            .body(
                column![
                    setting(
                        "Make profile private and hide activity",
                        Switch::new(app.private).on_toggle(Message::Private),
                    ),
                    setting(
                        "Include private contributions on my profile",
                        Switch::new(app.contributions).on_toggle(Message::Contributions),
                    ),
                    Button::new("Save Changes")
                        .variant(Variant::Primary)
                        .on_press(Message::Nothing),
                ]
                .spacing(theme::SPACING_LG),
            ),
    );

    // section: Preferences
    // Three checkboxes, each with the description the Slint example writes under
    // its label.
    let preferences = body_box(
        column![
            Checkbox::new(app.all_activity)
                .label("All activity")
                .description("Every mention, reply, and review.")
                .on_toggle(Message::AllActivity),
            Checkbox::new(app.product_updates)
                .label("Product updates")
                .description("Release notes and deprecations.")
                .on_toggle(Message::ProductUpdates),
            Checkbox::new(app.account_activity)
                .label("Account activity")
                .description("Sign-ins and billing changes.")
                .on_toggle(Message::AccountActivity),
        ]
        .spacing(theme::SPACING_LG),
    );

    // section: GroupBox Variants
    // The three surfaces side by side, each on its own rather than nested.
    let variants = body_box(
        row![
            variant_panel(
                group_box::Variant::Normal,
                "No visual container. The title alone groups the content.",
            ),
            variant_panel(
                group_box::Variant::Fill,
                "A muted background, for a recessed settings surface.",
            ),
            variant_panel(
                group_box::Variant::Outline,
                "A one-pixel border, for a card that must read as a unit.",
            ),
        ]
        .spacing(theme::SPACING_MD),
    );

    // section: With Title
    // The titled surface the group box is for: the title outside, the controls
    // and the action inside a filled body.
    let with_title = body_box(
        GroupBox::new()
            .title("Email notifications")
            .variant(group_box::Variant::Fill)
            .body(
                column![
                    Checkbox::new(app.all_activity)
                        .label("All activity")
                        .on_toggle(Message::AllActivity),
                    Checkbox::new(app.account_activity)
                        .label("Account activity")
                        .on_toggle(Message::AccountActivity),
                    Button::new("Save preferences")
                        .variant(Variant::Primary)
                        .on_press(Message::Nothing),
                ]
                .spacing(theme::SPACING_LG),
            ),
    );

    // section: Footer outside the surface
    // The footer hangs under the filled surface as small muted text, 8px away,
    // sharing the title's leading edge.
    let footer = body_box(
        GroupBox::new()
            .title("Contributions & activity")
            .variant(group_box::Variant::Fill)
            .footer("Private contributions never reveal repository names.")
            .body(setting(
                "Make profile private and hide activity",
                Switch::new(app.private).on_toggle(Message::Private),
            )),
    );

    // section: Custom Title Styling
    // `title_color` takes any colour; the sample names the page's foreground,
    // which is the colour the Slint example gives the title.
    let title_styling = body_box(
        GroupBox::new()
            .title("Appearance")
            .variant(group_box::Variant::Outline)
            .title_color(app.tokens.foreground)
            .body(
                RadioGroup::new()
                    .options(["Light", "Dark", "System"])
                    .selected(app.appearance)
                    .on_change(Message::Appearance),
            ),
    );

    // section: Custom Content Styling
    // A 24px inset on a 12px radius: the two knobs a caller reaches for most.
    let content_styling = body_box(
        GroupBox::new()
            .title("Roomy surface")
            .variant(group_box::Variant::Outline)
            .content_padding(24.0)
            .corner_radius(theme::RADIUS_XL)
            .body(text("A 24px inset on a 12px radius.").size(theme::FONT_SIZE_SM)),
    );

    // section: Without Title
    // The title is optional: an outline surface holding one setting.
    let without_title = body_box(GroupBox::new().variant(group_box::Variant::Outline).body(
        setting(
            "Make profile private and hide activity",
            Switch::new(app.private).on_toggle(Message::Private),
        ),
    ));

    // section: end
    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        account,
        heading(app, 1, SECTIONS[1]),
        preferences,
        heading(app, 2, SECTIONS[2]),
        variants,
        heading(app, 3, SECTIONS[3]),
        with_title,
        heading(app, 4, SECTIONS[4]),
        footer,
        heading(app, 5, SECTIONS[5]),
        title_styling,
        heading(app, 6, SECTIONS[6]),
        content_styling,
        heading(app, 7, SECTIONS[7]),
        without_title,
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    // The page itself is a control, so a press outside a panel lands on a widget
    // rather than on the bare window.
    container(scrollable(gallery).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).background.into()),
            ..container::Style::default()
        })
        .into()
}

// The Lucide mark the code icon uses, from the icon set this repository already
// ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three galleries
// draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
