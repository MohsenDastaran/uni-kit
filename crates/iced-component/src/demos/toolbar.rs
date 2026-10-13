//! `toolbar` — the whole gallery the Slint example shows: a row of commands, the
//! size ladder, labels and a field beside the commands, a custom surface, and
//! the groups that name a run of controls.
//!
//! Run it with `cargo run --example toolbar`, or in the browser as
//! `?story=toolbar` in the gallery the website embeds.
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
//! The toolbar is transparent, so every section below wraps it in the surface a
//! real header would supply: the Slint example's Rectangle, the `sidebar`
//! fill with the `border` token as its edge. The bar's own size sets its height
//! and its slots; iced has no `Sizable` seam for the bar to resize a control it
//! did not build, so the demo sizes each button to match — the caller's half of
//! the contract the component's module comment states.
//!
//! The overflow at the end of the first bar is the application's composition,
//! not the component's: a ghost icon button as a [`Popover`]'s trigger and a
//! [`Menu`] as its panel. The open flag is the gallery's, exactly as a popover's
//! is, so the button draws itself selected while the menu is showing.
//!
//! Slint's example has no disabled command; one is added here because the bar's
//! job is to make a disabled item read as disabled, and a screenshot is how that
//! is checked. It is drawn through the shared pressable handle rather than a
//! [`Button`], whose label carries its own ink and so cannot take the muted
//! disabled colour — the reason is written on `disabled_command` below.

use iced::widget::{column, container, mouse_area, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::button::{self, Button, Variant};
use crate::components::group_box::{self, GroupBox};
use crate::components::input::Input;
use crate::components::menu::{Item, Menu};
use crate::components::popover::{Placement, Popover};
use crate::components::theme::{self, ControlSize, Tokens};
use crate::components::toolbar::{Toolbar, ToolbarGroup, ToolbarSeparator, ToolbarSpacer};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The toolbar's inset inside its surface, Slint's `inset: 6px`.
const INSET: f32 = 6.0;

/// The filter field's width, Slint's `width: 200px`.
const FILTER_WIDTH: f32 = 200.0;

/// The overflow panel's width. The menu's own default is 280px; a command
/// overflow is a short list, and Slint's `menu-width: 0px` leaves it to the
/// control, which in iced is the popover's fixed width.
const OVERFLOW_WIDTH: f32 = 240.0;

/// The section headings, in the order the Slint example and the component page
/// have them. The index is what the page is asked to scroll to.
const SECTIONS: [&str; 5] = [
    "Commands",
    "Sizes",
    "Labels and custom elements",
    "Custom styling",
    "Groups",
];

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — toolbar")
        .theme(theme)
        .run()
}

#[derive(Debug, Default)]
struct App {
    /// Whether the overflow menu is showing. iced has no menu state to own it,
    /// so the gallery holds the flag and answers every request to change it.
    open: bool,
    /// The overflow row last chosen.
    chosen: Option<usize>,
    /// The row the pointer is over in the overflow menu; the transient highlight
    /// the menu reports and clears.
    highlighted: Option<usize>,
    /// The filter field's value, because an iced field holds no text of its own.
    filter: String,
    /// What the gallery last showed, so a press has a visible result.
    last: Option<String>,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    /// The overflow asked to be opened or closed.
    Opened(bool),
    /// An overflow row was chosen, by index.
    Chosen(usize),
    /// The pointer entered an overflow row.
    Highlighted(usize),
    /// The pointer left a row, or left the panel.
    Left(Option<usize>),
    /// A command was pressed. The label is the command's own.
    Command(&'static str),
    /// The filter field reported an edit.
    Filtered(String),
    HoverSource(Option<&'static str>),
    PressSource(Option<&'static str>),
    /// A section heading was released: ask the page for that sample.
    ShowSource {
        index: usize,
        title: &'static str,
    },
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Opened(open) => app.open = open,
        Message::Chosen(index) => {
            app.chosen = Some(index);
            // A menu reports the choice; whether the panel stays open is the
            // caller's, because the open flag is the caller's. A command
            // overflow closes when its command runs.
            app.open = false;
            app.highlighted = None;
            app.last = Some(format!("The overflow chose row {}.", index + 1));
        }
        Message::Highlighted(index) => app.highlighted = Some(index),
        // A row's exit and the panel's exit both clear the pointer's row. The
        // index is carried so the report matches the menu's own contract, and
        // it is read here rather than discarded.
        Message::Left(row) => {
            if row.is_none() || app.highlighted == row {
                app.highlighted = None;
            }
        }
        Message::Command(label) => app.last = Some(format!("Pressed {label}.")),
        Message::Filtered(value) => app.filter = value,
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
/// is the same 28px box the popover gallery draws.
fn heading<'a>(app: &App, index: usize, title: &'static str) -> Element<'a, Message> {
    let hovered = app.hovered == Some(title);
    let pressed = app.pressed_source == Some(title);

    let label = mouse_area(
        container(
            text(title)
                .size(theme::FONT_SIZE_SM)
                .font(iced::Font {
                    weight: iced::font::Weight::Medium,
                    ..iced::Font::DEFAULT
                }),
        )
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

/// The surface the toolbar sits on: the Slint example's Rectangle — the
/// `sidebar` fill with the `border` token as a one-pixel edge. A tinted surface
/// stands in for the example's custom-styling section, which washes the fill
/// with the primary colour at 8% in the light theme and 18% in the dark one.
///
/// Every token is resolved inside the closure, so the surface follows the theme
/// in force when it is drawn rather than the one at build time.
fn surface<'a>(bar: impl Into<Element<'a, Message>>, tinted: bool) -> Element<'a, Message> {
    container(bar)
        .width(Length::Fill)
        .style(move |theme: &Theme| {
            let tokens = Tokens::of(theme);
            let background = if tinted {
                theme::alpha(tokens.primary, if tokens.dark { 0.18 } else { 0.08 })
            } else {
                tokens.sidebar
            };
            container::Style {
                background: Some(background.into()),
                border: iced::border::rounded(theme::RADIUS_MD)
                    .color(tokens.border)
                    .width(1.0),
                ..container::Style::default()
            }
        })
        .into()
}

/// A ghost command with a leading glyph, at the size the bar is drawn at.
fn command<'a>(label: &'static str, size: ControlSize, markup: &'static [u8]) -> Element<'a, Message> {
    Button::new(label)
        .variant(Variant::Ghost)
        .size(size)
        .icon(icon(markup))
        .on_press(Message::Command(label))
        .into()
}

/// A ghost command that is only its glyph. Slint draws the same button with an
/// `icon` and no `text`; iced has no accessible name for it, which the component
/// module comment names.
fn icon_command<'a>(label: &'static str, size: ControlSize, markup: &'static [u8]) -> Element<'a, Message> {
    Button::new("")
        .variant(Variant::Ghost)
        .size(size)
        .icon(icon(markup))
        .on_press(Message::Command(label))
        .into()
}

/// A command the bar draws but does not answer.
///
/// A [`Button`] carries its label's ink itself, so its disabled state is the
/// fill's and not the word's — a ghost command with no message still paints its
/// label in the enabled ink. This draws the muted state through the shared
/// pressable handle instead, with every token resolved inside the closure, so a
/// disabled command is legible *as* disabled in both themes.
fn disabled_command<'a>(
    label: &'static str,
    size: ControlSize,
    markup: &'static [u8],
) -> Element<'a, Message> {
    let content = iced::widget::row![
        svg(svg::Handle::from_memory(markup))
            .width(14.0)
            .height(14.0)
            .style(|theme: &Theme, _status| svg::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            }),
        text(label)
            .size(size.control_font_size())
            .font(iced::Font {
                weight: iced::font::Weight::Medium,
                ..iced::Font::DEFAULT
            }),
    ]
    .spacing(4.0)
    .align_y(Alignment::Center);

    button::handle(content, None::<Message>, |theme: &Theme, _status| {
        let tokens = Tokens::of(theme);
        iced::widget::button::Style {
            background: None,
            text_color: tokens.muted_foreground,
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
            snap: false,
        }
    })
}

/// The overflow menu: the commands a bar of this width cannot show, over the
/// shared menu panel.
fn overflow_menu<'a>(app: &App) -> Element<'a, Message> {
    Menu::new([
        Item::new("Duplicate").icon(COPY).shortcut("Cmd D"),
        Item::new("Export as CSV").icon(DOWNLOAD),
        Item::separator(),
        Item::new("Delete").icon(TRASH).danger(true),
    ])
    .selected(app.chosen)
    .highlighted(app.highlighted)
    .on_chosen(Message::Chosen)
    .on_highlight(Message::Highlighted)
    .on_leave(Message::Left)
    .into_element()
}

/// The overflow control: a ghost icon button as the popover's trigger, and the
/// menu as its panel. The button is `selected` while the menu is open, so the
/// trigger stays visibly pressed until the panel closes.
fn overflow<'a>(app: &App, size: ControlSize) -> Element<'a, Message> {
    Popover::new(
        Button::new("")
            .variant(Variant::Ghost)
            .size(size)
            .icon(icon(ELLIPSIS))
            .selected(app.open),
    )
    .open(app.open)
    .placement(Placement::Bottom)
    .offset(4.0)
    .panel_width(OVERFLOW_WIDTH)
    // One surface, and the popover owns it: the menu hands back its body and
    // draws none of its own. `padding(0.0)` keeps that surface on the body,
    // whose 4px inset is the panel's own.
    .filled(true)
    .padding(0.0)
    .content(overflow_menu(app))
    .on_open_change(Message::Opened)
    .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Commands
    // A toolbar is transparent, so this frame stands in for the panel header
    // that would supply the surface. The bar's spacer keeps the leading
    // commands together and still lets the overflow reach the trailing edge.
    let commands = body_box(surface(
        Toolbar::new()
            .size(ControlSize::Small)
            .inset(INSET)
            .child(command("New", ControlSize::Small, PLUS))
            .content(ToolbarSeparator::new())
            .child(icon_command("Undo", ControlSize::Small, ARROW_LEFT))
            .child(icon_command("Redo", ControlSize::Small, ARROW_RIGHT))
            // A command with no handler: the bar must keep it legible as
            // disabled in both themes, which is the state a screenshot checks.
            .child(disabled_command("Publish", ControlSize::Small, UPLOAD))
            .content(ToolbarSpacer::new())
            .child(overflow(app, ControlSize::Small)),
        false,
    ));

    // section: Sizes
    // The size is the whole bar's: it sets the height and the command slots,
    // and the caller sizes the controls inside it to match.
    let sized = |size: ControlSize, label: &'static str| {
        surface(
            Toolbar::new()
                .size(size)
                .inset(INSET)
                .child(
                    Button::new(label)
                        .variant(Variant::Ghost)
                        .size(size)
                        .on_press(Message::Command(label)),
                )
                .child(
                    Button::new(label)
                        .variant(Variant::Ghost)
                        .size(size)
                        .on_press(Message::Command(label)),
                )
                .content(ToolbarSpacer::new()),
            false,
        )
    };
    let sizes = column![
        sized(
            ControlSize::XSmall,
            // The Slint example's own labels: the same word twice at each step.
            "Small",
        ),
        sized(ControlSize::Medium, "Medium"),
        sized(ControlSize::Large, "Large"),
    ]
    .spacing(12.0);

    // section: Labels and custom elements
    // A label and a field are content, not commands, so they keep their own
    // dimensions and go through `content`; only `Review` is a command.
    let labels = body_box(surface(
        Toolbar::new()
            .size(ControlSize::Small)
            .inset(INSET)
            .spacing(theme::SPACING_SM)
            .content(muted_line("3 files changed"))
            .content(ToolbarSeparator::new())
            .content(
                Input::new("toolbar-filter")
                    .size(ControlSize::Small)
                    .width(FILTER_WIDTH)
                    .placeholder("Filter files…")
                    .prefix(glyph_muted(SEARCH))
                    .value(app.filter.clone())
                    .on_input(Message::Filtered),
            )
            .content(ToolbarSpacer::new())
            .child(
                Button::new("Review")
                    .variant(Variant::Primary)
                    .size(ControlSize::Small)
                    .on_press(Message::Command("Review")),
            ),
        false,
    ));

    // section: Custom styling
    // The surrounding surface owns the background and the border, so the bar
    // drops into a header without fighting it; a tinted surface is the one the
    // Slint example washes with the primary colour.
    let custom = body_box(surface(
        Toolbar::new()
            .size(ControlSize::Small)
            .inset(INSET)
            .spacing(6.0)
            .content(
                text("Editor")
                    .size(theme::FONT_SIZE_XS)
                    .font(iced::Font {
                        weight: iced::font::Weight::Medium,
                        ..iced::Font::DEFAULT
                    })
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).foreground),
                    }),
            )
            .content(ToolbarSpacer::new())
            .child(icon_command("Toggle left panel", ControlSize::Small, PANEL_LEFT))
            .child(icon_command(
                "Toggle right panel",
                ControlSize::Small,
                PANEL_RIGHT,
            )),
        true,
    ));

    // section: Groups
    // A group holds a run together with a tighter gap than the bar's, so the
    // three runs read as three units. The separator divides them; the spacer
    // pushes the last run to the trailing edge.
    let groups = body_box(surface(
        Toolbar::new()
            .size(ControlSize::Small)
            .inset(INSET)
            .child(
                ToolbarGroup::new()
                    .size(ControlSize::Small)
                    .child(icon_command("Undo", ControlSize::Small, ARROW_LEFT))
                    .child(icon_command("Redo", ControlSize::Small, ARROW_RIGHT)),
            )
            .content(ToolbarSeparator::new())
            .child(
                ToolbarGroup::new()
                    .size(ControlSize::Small)
                    .child(icon_command("Bold", ControlSize::Small, BOLD))
                    .child(icon_command("Italic", ControlSize::Small, ITALIC))
                    .child(icon_command("Underline", ControlSize::Small, UNDERLINE)),
            )
            .content(ToolbarSpacer::new())
            .child(
                ToolbarGroup::new()
                    .size(ControlSize::Small)
                    .child(
                        Button::new("Share")
                            .variant(Variant::Ghost)
                            .size(ControlSize::Small)
                            .on_press(Message::Command("Share")),
                    )
                    .child(icon_command("More", ControlSize::Small, ELLIPSIS)),
            ),
        false,
    ));

    // section: end
    let status = text(match &app.last {
        Some(line) => line.clone(),
        None => "Press a command, or open the overflow menu.".to_string(),
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        heading(app, 0, SECTIONS[0]),
        commands,
        heading(app, 1, SECTIONS[1]),
        sizes,
        heading(app, 2, SECTIONS[2]),
        labels,
        heading(app, 3, SECTIONS[3]),
        custom,
        heading(app, 4, SECTIONS[4]),
        groups,
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

/// An icon the components draw themselves: a 14px SVG in the ink a ghost
/// command uses. The glyphs are the caller's, because an iced application owns
/// its assets — the Slint example names them and its build script resolves the
/// folder — but the colour is not: an SVG with no colour of its own keeps the
/// one in its bytes, which is black in both themes and vanishes on the dark
/// surface.
fn icon(bytes: &'static [u8]) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(14.0)
        .height(14.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(button::ink(Tokens::of(theme), Variant::Ghost, false)),
        })
        .into()
}

/// A muted 16px glyph for a field's prefix slot, which draws its own colour.
fn glyph_muted(bytes: &'static [u8]) -> Element<'static, Message> {
    svg(svg::Handle::from_memory(bytes))
        .width(16.0)
        .height(16.0)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

// The marks the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so
// the two galleries draw the same glyphs rather than two lookalikes.

const PLUS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="M12 5v14"/></svg>"#;

const ARROW_LEFT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m12 19-7-7 7-7"/><path d="M19 12H5"/></svg>"#;

const ARROW_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12h14"/><path d="m12 5 7 7-7 7"/></svg>"#;

const ELLIPSIS: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>"#;

const SEARCH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21 21-4.34-4.34"/><circle cx="11" cy="11" r="8"/></svg>"#;

const PANEL_LEFT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/></svg>"#;

const PANEL_RIGHT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M15 3v18"/></svg>"#;

const BOLD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8"/></svg>"#;

const ITALIC: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="19" x2="10" y1="4" y2="4"/><line x1="14" x2="5" y1="20" y2="20"/><line x1="15" x2="9" y1="4" y2="20"/></svg>"#;

const UNDERLINE: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 4v6a6 6 0 0 0 12 0V4"/><line x1="4" x2="20" y1="20" y2="20"/></svg>"#;

const UPLOAD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m17 8-5-5-5 5"/><path d="M12 3v12"/></svg>"#;

const COPY: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/></svg>"#;

const DOWNLOAD: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/><path d="M12 15V3"/></svg>"#;

const TRASH: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
