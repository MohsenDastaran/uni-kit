//! `color-picker` — the whole gallery the Slint example shows: the compact
//! square trigger and the framed field one, the size ladder, a caller's own
//! featured row, the palette and the HSLA panel, and the hex field at the foot
//! of each.
//!
//! Run it with `cargo run --example color-picker`, or in the browser as
//! `?story=color-picker` in the gallery the website embeds.
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
//! The gallery owns every piece of state a colour picker draws from — the value,
//! the hex text, the open flag and the active tab — because the component has
//! none: it reports a request and the application answers with its own value.
//! The hex text is the caller's for a reason the component's docs give: a field
//! iced recomputed from the value every frame would erase a half-typed `#63`,
//! so the text is held here and parsed only when it names a colour. Escape and a
//! click outside close the panel through the popover, which owns those two
//! dismissals for every overlay in this crate; Escape is also bound here so the
//! key closes one from anywhere.
//!
//! Every picker starts on indigo, `#6366F1`, the colour the Slint example
//! starts on, so a screenshot and a hash compare are the same every run.

use iced::keyboard;
use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Border, Color, Element, Length, Subscription, Task, Theme};

use crate::components::color_picker::{self, ColorPicker};
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, ControlSize, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The colour every picker starts on, the Slint example's `#6366f1`.
const INDIGO: Color = Color::from_rgb8(0x63, 0x66, 0xf1);

/// The section headings, in the order the component page has them. The index is
/// what the page is asked to scroll to.
const SECTIONS: [&str; 6] = [
    "Usage",
    "Different Sizes",
    "With Custom Featured Colors",
    "Theme Color",
    "Color Select",
    "Color Theme Editor",
];

/// Which picker each piece of state belongs to. A section that shows more than
/// one picker — the size ladder, Color Select, the editor — still gets one slot
/// per picker, so opening one panel does not open its neighbours.
mod slot {
    pub const USAGE: usize = 0;
    pub const XSMALL: usize = 1;
    pub const SMALL: usize = 2;
    pub const MEDIUM: usize = 3;
    pub const LARGE: usize = 4;
    pub const FEATURED: usize = 5;
    pub const THEME: usize = 6;
    pub const SELECT_VALUE: usize = 7;
    pub const SELECT_EMPTY: usize = 8;
    pub const PRIMARY: usize = 9;
    pub const SECONDARY: usize = 10;
    pub const ACCENT: usize = 11;
    /// One more than the highest slot: the length of the state arrays.
    pub const COUNT: usize = 12;
}

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — color-picker")
        .theme(theme)
        .subscription(subscription)
        .run()
}

#[derive(Debug, Clone)]
struct App {
    /// The colour each picker holds. `None` is the Slint `empty` state.
    value: [Option<Color>; slot::COUNT],
    /// The hex text each picker's field shows. The caller owns it — see the
    /// module comment.
    hex: [String; slot::COUNT],
    /// Whether each picker's panel is showing.
    open: [bool; slot::COUNT],
    /// Which panel tab each picker shows: 0 Palette, 1 HSLA.
    tab: [usize; slot::COUNT],
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// A picker asked to be opened or closed; the slot says which one.
    Opened { slot: usize, open: bool },
    /// A palette cell or a slider asked for a colour.
    Changed { slot: usize, value: Option<Color> },
    /// The hex field was edited, as the text it now holds.
    HexInput { slot: usize, text: String },
    /// Enter was pressed in a hex field.
    HexSubmit { slot: usize },
    /// A tab button was pressed.
    Tab { slot: usize, tab: usize },
    /// Escape was pressed: close whichever panel is open.
    Escape,
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource { index: usize, title: &'static str },
}

fn boot() -> App {
    let mut value = [None; slot::COUNT];
    value[slot::USAGE] = Some(INDIGO);
    value[slot::XSMALL] = Some(INDIGO);
    value[slot::SMALL] = Some(INDIGO);
    value[slot::MEDIUM] = Some(INDIGO);
    value[slot::LARGE] = Some(INDIGO);
    value[slot::FEATURED] = Some(INDIGO);
    value[slot::THEME] = Some(INDIGO);
    value[slot::SELECT_VALUE] = Some(INDIGO);
    value[slot::PRIMARY] = Some(INDIGO);
    value[slot::SECONDARY] = Some(Color::from_rgb8(0x06, 0xb6, 0xd4));
    value[slot::ACCENT] = Some(Color::from_rgb8(0xec, 0x48, 0x99));

    let mut hex: [String; slot::COUNT] = std::array::from_fn(|_| String::new());
    for (index, color) in value.iter().enumerate() {
        if let Some(color) = color {
            hex[index] = color_picker::to_hex(*color);
        }
    }

    App {
        value,
        hex,
        open: [false; slot::COUNT],
        tab: [0; slot::COUNT],
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
        Message::Changed { slot, value } => {
            app.value[slot] = value;
            // The text the field shows follows the value, which is what makes a
            // palette press and a slider drag visible in the hex field.
            app.hex[slot] = value.map(color_picker::to_hex).unwrap_or_default();
        }
        Message::HexInput { slot, text } => {
            app.hex[slot] = text.clone();
            // A text that names a colour previews it at once; one that does not
            // leaves the value alone, so a half-typed `#63` does not flicker the
            // preview to nothing.
            if let Some(color) = color_picker::parse_hex(&text) {
                app.value[slot] = Some(color);
            }
        }
        Message::HexSubmit { slot } => match color_picker::parse_hex(&app.hex[slot]) {
            Some(color) => {
                app.value[slot] = Some(color);
                app.hex[slot] = color_picker::to_hex(color);
            }
            // Enter on something that is not a colour restores the value's own
            // text rather than leaving the field lying about it.
            None => {
                app.hex[slot] = app.value[slot].map(color_picker::to_hex).unwrap_or_default();
            }
        },
        Message::Tab { slot, tab } => {
            app.tab[slot] = tab;
        }
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

/// The keyboard the picker leaves to its host. The popover already answers
/// Escape, and a click outside closes every overlay in this crate; the binding
/// here makes the key close a panel the application owns even when the event
/// never reaches the overlay, which is cheap insurance for one key.
fn subscription(_app: &App) -> Subscription<Message> {
    iced::keyboard::listen().filter_map(|event| match event {
        keyboard::Event::KeyPressed { key, .. } => match key.as_ref() {
            keyboard::Key::Named(keyboard::key::Named::Escape) => Some(Message::Escape),
            _ => None,
        },
        _ => None,
    })
}

/// One colour picker in the gallery, wired to the slot its value, hex text, open
/// flag and tab live in.
///
/// A section sample spells the builder out rather than calling this, so the code
/// the component page shows is the code a reader would write; this is the body
/// those samples share where there is more than one picker in a card.
fn picker<'a>(
    app: &App,
    slot: usize,
    field: bool,
    size: ControlSize,
) -> ColorPicker<'a, Message> {
    ColorPicker::new()
        .value(app.value[slot])
        .hex(app.hex[slot].clone())
        .open(app.open[slot])
        .tab(app.tab[slot])
        .field(field)
        .size(size)
        .id(format!("color-picker-{slot}"))
        .on_open_change(move |open| Message::Opened { slot, open })
        .on_change(move |value| Message::Changed { slot, value })
        .on_hex_input(move |text| Message::HexInput { slot, text })
        .on_hex_submit(Message::HexSubmit { slot })
        .on_tab(move |tab| Message::Tab { slot, tab })
}

/// The selected colour, drawn full width and 96px high, the Slint example's
/// preview panel. A colour with alpha shows the panel through it.
fn preview<'a>(color: Option<Color>) -> Element<'a, Message> {
    container(iced::widget::space().width(Length::Fill).height(Length::Fill))
        .width(Length::Fill)
        .height(Length::Fixed(96.0))
        .style(move |theme: &Theme| container::Style {
            background: Some(color.unwrap_or(Color::TRANSPARENT).into()),
            border: Border {
                color: Tokens::of(theme).border,
                width: 1.0,
                radius: theme::RADIUS_LG.into(),
            },
            ..container::Style::default()
        })
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

/// One labelled row of the editor: the name and a field-shaped picker beside it.
fn editor_row<'a>(app: &App, slot: usize, label: &'static str) -> Element<'a, Message> {
    row![
        text(label)
            .size(theme::FONT_SIZE_SM)
            .width(Length::Fixed(110.0))
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
        picker(app, slot, true, ControlSize::Medium),
    ]
    .spacing(theme::SPACING_MD)
    .align_y(Alignment::Center)
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Usage
    // The whole of a colour picker: the value, the hex text, the open flag and
    // the reports. `new` starts on nothing; `value` is the chosen colour and
    // `None` is the empty state, `hex` is the text the field shows, and the four
    // callbacks are what the trigger, a palette press or a slider drag, the hex
    // field and the two tabs ask for. The preview under it is the caller's, made
    // from the value the picker reported.
    let usage = body_box(
        column![
            ColorPicker::new()
                .value(app.value[slot::USAGE])
                .hex(app.hex[slot::USAGE].clone())
                .open(app.open[slot::USAGE])
                .tab(app.tab[slot::USAGE])
                .id("color-picker-0")
                .on_open_change(|open| Message::Opened {
                    slot: slot::USAGE,
                    open,
                })
                .on_change(|value| Message::Changed {
                    slot: slot::USAGE,
                    value,
                })
                .on_hex_input(|text| Message::HexInput {
                    slot: slot::USAGE,
                    text,
                })
                .on_hex_submit(Message::HexSubmit { slot: slot::USAGE })
                .on_tab(|tab| Message::Tab {
                    slot: slot::USAGE,
                    tab,
                }),
            preview(app.value[slot::USAGE]),
            muted_line(
                "The square trigger opens the palette; the HSLA tab and the hex \
                 field at the foot of the panel edit the same value.",
            ),
        ]
        .spacing(12.0),
    );

    // section: Different Sizes
    // The trigger's height, inset and type come from one size step — the same
    // ladder `input` and `select` use — so a picker sits beside the fields in a
    // form. The panel does not change with it.
    let sizes = body_box(
        column![
            row![
                picker(app, slot::XSMALL, false, ControlSize::XSmall),
                picker(app, slot::SMALL, false, ControlSize::Small),
                picker(app, slot::MEDIUM, false, ControlSize::Medium),
                picker(app, slot::LARGE, false, ControlSize::Large),
            ]
            .spacing(12.0)
            .align_y(Alignment::Center),
            muted_line("Extra small, small, medium and large square triggers."),
        ]
        .spacing(12.0),
    );

    // section: With Custom Featured Colors
    // `featured` replaces the top row — the Slint `swatches` property, GPUI's
    // `featured_colors`. The nine family ramps below it are the component's own
    // and do not change.
    let featured = body_box(
        column![
            picker(app, slot::FEATURED, true, ControlSize::Medium).featured(vec![
                Color::from_rgb8(0xff, 0x6b, 0x6b),
                Color::from_rgb8(0x4e, 0xcd, 0xc4),
                Color::from_rgb8(0x45, 0xb7, 0xd1),
                Color::from_rgb8(0x96, 0xce, 0xb4),
                Color::from_rgb8(0xff, 0xea, 0xa7),
            ]),
            muted_line(
                "A caller's own five: open the panel and the featured row is the \
                 brand set rather than the component's default eleven.",
            ),
        ]
        .spacing(12.0),
    );

    // section: Theme Color
    // The field trigger: a swatch, the hex value and a caret, sharing the height
    // and frame of the inputs around it. `field(true)` is GPUI's `ColorSelect`
    // and the Slint `field` property; the preview under it is the application's.
    let accent = app.value[slot::THEME];
    let theme_color = body_box(
        column![
            picker(app, slot::THEME, true, ControlSize::Medium),
            preview(accent),
            row![
                text("Selected color")
                    .size(theme::FONT_SIZE_SM)
                    .width(Length::Fill)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    }),
                text(app.hex[slot::THEME].clone())
                    .size(theme::FONT_SIZE_SM)
                    .font(theme::MONO)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).foreground),
                    }),
            ]
            .spacing(8.0)
            .align_y(Alignment::Center),
        ]
        .spacing(16.0),
    );

    // section: Color Select
    // The same control in a form: the framed field, and the placeholder a field
    // shows while nothing is chosen.
    let color_select = body_box(
        column![
            picker(app, slot::SELECT_VALUE, true, ControlSize::Medium),
            ColorPicker::new()
                .value(app.value[slot::SELECT_EMPTY])
                .hex(app.hex[slot::SELECT_EMPTY].clone())
                .open(app.open[slot::SELECT_EMPTY])
                .field(true)
                .placeholder("Pick a color")
                .id("color-picker-8")
                .on_open_change(|open| Message::Opened {
                    slot: slot::SELECT_EMPTY,
                    open,
                })
                .on_change(|value| Message::Changed {
                    slot: slot::SELECT_EMPTY,
                    value,
                })
                .on_hex_input(|text| Message::HexInput {
                    slot: slot::SELECT_EMPTY,
                    text,
                })
                .on_hex_submit(Message::HexSubmit {
                    slot: slot::SELECT_EMPTY,
                })
                .on_tab(|tab| Message::Tab {
                    slot: slot::SELECT_EMPTY,
                    tab,
                }),
            muted_line(
                "The first field holds a value; the second is empty and shows its \
                 placeholder until a colour is picked.",
            ),
        ]
        .spacing(12.0),
    );

    // section: Color Theme Editor
    // Three field pickers, one per theme role. Each is the same control and its
    // own caller-owned value — GPUI's editor, which the Slint example shows as
    // the two pickers of its Color Select card.
    let editor = body_box(
        column![
            editor_row(app, slot::PRIMARY, "Primary"),
            editor_row(app, slot::SECONDARY, "Secondary"),
            editor_row(app, slot::ACCENT, "Accent"),
        ]
        .spacing(12.0),
    );

    // section: end
    let status = text(match app.open.iter().position(|open| *open) {
        Some(slot) => format!(
            "Picker {} is open. A palette cell or a slider reports a colour; \
             Escape or a click outside closes the panel.",
            slot + 1
        ),
        None => "Open a trigger and pick a colour; the hex field takes a typed \
                 value and Enter commits it."
            .to_string(),
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        usage,
        heading(app, 1, SECTIONS[1]),
        sizes,
        heading(app, 2, SECTIONS[2]),
        featured,
        heading(app, 3, SECTIONS[3]),
        theme_color,
        heading(app, 4, SECTIONS[4]),
        color_select,
        heading(app, 5, SECTIONS[5]),
        editor,
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
