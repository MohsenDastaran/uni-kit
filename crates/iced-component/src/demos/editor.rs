//! `editor` — the code editor's gallery the way the Slint example shows it: the
//! basic editable buffer, a read-only file, and a larger code size. The three
//! sections are the Slint example's visible ones, in its order; its fourth is a
//! finished screen the gallery only shows on its own, so it is the page rather
//! than the component and is not ported here, exactly as `table` and `settings`
//! left theirs out.
//!
//! Run it with `cargo run --example editor`, or in the browser as
//! `?story=editor` in the gallery the website embeds.
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
//! **The gutter cannot follow the field's scroll, and the first card says so.**
//! iced's `text_editor` owns its internal scroll and publishes no offset, so the
//! line numbers are drawn against the field's top and stay there while the field
//! scrolls. The Slint editor shares one `Flickable` between the two, which iced
//! does not offer; the component's module docs name the same gap.

use iced::widget::text_editor::{Action, Content};
use iced::widget::{column, container, mouse_area, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

use crate::components::editor::Editor;
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, Tokens};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — editor")
        .theme(theme)
        .run()
}

#[derive(Default)]
struct App {
    /// One `Content` per example, because iced keeps the text, the caret and the
    /// selection in it and the caller owns it: every edit comes back through
    /// `on_action` and is applied with `perform`.
    basic: Content,
    options: Content,
    font: Content,
    /// Which section heading the pointer is over, and which was pressed, so the
    /// code icon can draw the two states the Slint heading animates to.
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
    Basic(Action),
    Options(Action),
    Font(Action),
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
        basic: Content::with_text(
            "fn main() {\n    let name = \"GPUI Kit\";\n    println!(\"Hello, {name}!\");\n}\n",
        ),
        options: Content::with_text(
            "{\n  \"theme\": \"dark\",\n  \"fontSize\": 14,\n  \"lineNumbers\": true,\n  \"autoSave\": \"afterDelay\"\n}\n",
        ),
        font: Content::with_text(
            "# Release notes\n\n## 0.6.4\n\n- Add the iced editor\n- Sharpen the gutter and the status bar\n- Measure the line count from the text\n",
        ),
        ..App::default()
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Basic(action) => app.basic.perform(action),
        // Read-only: the field never publishes an edit, so whatever arrives is a
        // motion or a selection and applying it is safe.
        Message::Options(action) => app.options.perform(action),
        Message::Font(action) => app.font.perform(action),
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

/// The file glyph the editor's header leads with, and the lock it puts beside
/// `Read only`. Both are the Slint example's own marks, taken from the icon set
/// this repository already ships (`crates/slint-component/ui/icons`, Lucide,
/// ISC), and both are coloured by a style closure — an SVG keeps its own black
/// until one does.
fn icon<'a>(art: &'static [u8], pixels: f32) -> Element<'a, Message> {
    svg(svg::Handle::from_memory(art))
        .width(pixels)
        .height(pixels)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
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
/// sample. Either one is the button, as in Slint's `SourceHeading`, and the icon
/// is the same 28px box — `primary` at 0.12 with a 0.35 border at rest, 0.22 and
/// 0.55 on hover, 0.34 and 0.7 while pressed.
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

fn view(app: &App) -> Element<'_, Message> {
    // section: Basic usage
    // The field is editable and the header, gutter and status bar follow it. The
    // gutter is drawn against the field's top: iced's `text_editor` owns its own
    // scroll and publishes no offset, so the numbers stay put once the field
    // scrolls rather than pretending to follow it.
    let basic = Editor::new(&app.basic)
        .id("basic")
        .filename("main.rs")
        .language("Rust")
        .file_icon(icon(FILE_TEXT, 14.0))
        .height(Length::Fixed(220.0))
        .on_action(Message::Basic);

    // section: Editor options
    // `readonly` keeps the frame, the text, the caret and the selection and only
    // refuses the edits; the status bar reads `Read only` and takes the lock.
    let options = Editor::new(&app.options)
        .filename("settings.json")
        .language("JSON")
        .indent_label("Spaces: 2")
        .readonly(true)
        .readonly_icon(icon(LOCK, 11.0))
        .height(Length::Fixed(220.0))
        .on_action(Message::Options);

    // section: Font
    // The code size, which the gutter's row height follows. The default is the
    // theme's mono size, 13px.
    let font = Editor::new(&app.font)
        .filename("CHANGELOG.md")
        .language("Markdown")
        .font_size(16.0)
        .height(Length::Fixed(220.0))
        .on_action(Message::Font);

    // section: end
    let gallery = column![
        heading(app, 0, "Basic usage"),
        body_box(column![basic, muted_line("Type into the field: the caret, the gutter and the status count follow it.")].spacing(8.0)),
        heading(app, 1, "Editor options"),
        body_box(options),
        heading(app, 2, "Font"),
        body_box(font),
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The marks the Slint example asks for by name, taken from the icon set this
// repository already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the
// two galleries draw the same glyphs rather than two lookalikes.

const FILE_TEXT: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M10 9H8"/><path d="M16 13H8"/><path d="M16 17H8"/></svg>"#;

const LOCK: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>"#;

const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
