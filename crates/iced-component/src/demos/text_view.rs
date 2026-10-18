//! `text-view` — a rendered Markdown document, its richer blocks, and the
//! actions a code block carries.
//!
//! Run it with `cargo run --example text_view`, or in the browser as
//! `?story=text-view` in the gallery the website embeds.
//!
//! The Slint example's five outlined boxes are the five here, in its order,
//! each under the heading `website/component/text-view.md` documents that
//! sample at: `Markdown`, `HTML`, `Clamp to a number of lines`,
//! `Link Click Handling` and `Code Block Actions`. The page writes the first
//! three as `###` under `## Usage` and the last two as `##`, and the samples
//! keep the page's heading depth; the outline nests all five under Usage,
//! because the whole region they sit in is the page's usage region.
//!
//! The page documents more headings than the Slint example draws — `Fade in
//! streamed text`, `Highlight ranges`, `Scroll to a range`, `Touch Selection`,
//! `Images`, `Markdown Plugins`, `MarkdownNode`, `Block Plugins` and `YAML
//! Frontmatter`. They describe GPUI's `TextViewState`, plugins and range
//! painting, which this component does not carry, and the Slint example has no
//! box for them either: they get no iced panel rather than a panel showing
//! something else.
//!
//! **HTML is the one sample that draws the component's own gap.** Slint parses
//! `<u>` and `<font color>`; iced 0.14's Markdown parser parses no HTML, so the
//! tags come out as the words and brackets they are, which is what the
//! component's module doc records. The panel shows exactly that, because a panel
//! showing an underline would not be the code that runs.
//!
//! **The Markdown sample stays off `*emphasis*`.** The browser module embeds
//! Fira Sans Regular and no italic face, and cosmic-text finds no font for an
//! italic span and aborts the frame with `no default font found`. The sample
//! therefore draws the inline spans the gallery can — inline code, strong and a
//! link. An application that embeds an italic face can still use one; this
//! demo's module cannot, and an italic here would blank the whole gallery.
//!
//! Each heading carries the title and the code icon that asks the embedding page
//! to scroll to that sample. Both are the button, as in Slint's `SourceHeading`,
//! and a click posts the section's index and title through [`crate::bridge`], the
//! same bridge the Slint and GPUI galleries use. The `// section: <title>`
//! comments below are the contract the website reads to cut this file into the
//! usage panels a component page shows, so the code on the page is the code that
//! runs here. Keep them, one per section, in order, and keep the closing
//! `// section: end` after the last one. The status line that reports the live
//! controls sits after it, and is the only part of the page that is not a sample.

use crate::components::group_box::{self, GroupBox};
use crate::components::text_view::{Clamp, CodeBlock, Paragraph, TextView};
use crate::components::theme::{self, Tokens};
use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment, Element, Length, Task, Theme};

/// The page padding and the gap between sections, as the Slint example sets them.
const PAGE: f32 = 24.0;

/// The section headings, in the order the Slint example's `GroupBox`es — and the
/// component page's headings — have them. The first three are `###` under Usage;
/// the last two are `##`. The index is what the page is asked to scroll to.
const SECTIONS: [&str; 5] = [
    "Markdown",
    "HTML",
    "Clamp to a number of lines",
    "Link Click Handling",
    "Code Block Actions",
];

pub fn run(theme: Theme) -> iced::Result {
    // The parsed runs are built before iced hands the view its theme, so the
    // palette the application booted in is carried into the view.
    let tokens = Tokens::of(&theme);

    iced::application(move || boot(tokens), update, view)
        .title("uni-kit — text-view")
        .theme(theme)
        .run()
}

/// Which section heading the pointer is over, and which was pressed, so the code
/// icon can draw the two states the Slint heading animates to, and the last thing
/// the document reported.
#[derive(Debug, Clone)]
struct App {
    tokens: Tokens,
    event: Option<Event>,
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Event {
    Opened(String),
    Copied(String),
}

#[derive(Debug, Clone)]
enum Message {
    /// A link in a document was clicked.
    Opened(String),
    /// A code block's copy control was pressed.
    Copied(String),
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
        tokens,
        event: None,
        hovered: None,
        pressed_source: None,
    }
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Opened(url) => app.event = Some(Event::Opened(url)),
        Message::Copied(code) => app.event = Some(Event::Copied(code)),
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

/// The box under a heading: the same `GroupBox` with the outline variant the
/// Slint example wraps its sections in, untitled because the heading above it
/// carries the title.
fn panel<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

/// A titled section: the heading row, then the sample it names.
fn section<'a>(
    app: &App,
    index: usize,
    title: &'static str,
    body: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    column![heading(app, index, title), body.into()]
        .spacing(8)
        .into()
}

/// Small supporting text, the muted label the Slint clamp box writes under a
/// block.
fn muted_label<'a>(label: impl Into<String>) -> Element<'a, Message> {
    text(label.into())
        .size(theme::FONT_SIZE_XS)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: Markdown
    // The Slint example's first box: a whole document, with a heading scale,
    // inline spans, a fence, lists, a quote and a rule. `TextView` parses it, and
    // the two closures give the link and the fence's copy control their messages.
    const DOCUMENT: &str = r#"# Getting started

Copy a component into your project and import it from its `.rs` file. Every
component reads the theme, so it follows whichever appearance the app sets.

## Heading level two

A paragraph wraps at word boundaries and carries `inline code`, **strong**
text and a [link](https://gpui-kit.com/components) in the link colour.

### Install

```shell
bunx @dastaran/uni-kit@latest add iced button
```

- `button.rs` — a themed button
- `input.rs` — a single-line field
- `dialog.rs` — a modal panel

1. Run the command
2. Import the file
3. Use the component

> Components read the theme and nothing else, so a palette change reaches every
> one of them at once.

---

Read the [component catalog](https://gpui-kit.com/components) for the full list."#;

    let markdown = panel(
        TextView::new(DOCUMENT)
            .tokens(app.tokens)
            .on_link(Message::Opened)
            .on_copy(|code| Message::Copied(code.to_owned())),
    );

    // section: HTML
    // Slint draws an underlined run and two coloured ones. iced parses Markdown
    // and no HTML, so the tags are drawn as the words and brackets they are —
    // the documented gap, not a simplified stand-in for the Slint panel.
    let html = panel(
        TextView::new(
            "An <u>underlined</u> run, a <font color=\"#f97316\">coloured</font> one, \
             and an <u><font color=\"#22c55e\">both at once</font></u>.",
        )
        .tokens(app.tokens),
    );

    // section: Clamp to a number of lines
    // The box holds two lines and clips at its edge, and the same paragraph
    // below it is unclamped, so the difference is visible. Slint elides the last
    // line; iced 0.14 has no multi-line elision, which the component records.
    const DOCK: &str = "Dock builds application workspaces from draggable tab groups, \
        nested splits, and collapsible left, right, and bottom docks. It is the layout \
        foundation Longbridge uses in production rather than an isolated UI demo, and it \
        reconciles a pure-data layout tree into live panels on every frame.";

    let clamp = panel(
        column![
            muted_label("TextClamp — two lines"),
            Clamp::new(DOCK).lines(2),
            muted_label("The same paragraph, unclamped"),
            Paragraph::new(DOCK),
        ]
        .spacing(theme::SPACING_SM)
        .align_x(Alignment::Start),
    );

    // section: Link Click Handling
    // A link in a parsed document reports the URL it was clicked with, and the
    // line under the last section shows the last one. `on_link` is what turns the
    // links live: without it they keep the link colour and do nothing.
    let link = panel(
        TextView::new(
            "Open the [component catalog](https://gpui-kit.com/components) or read \
             the [guides](https://gpui-kit.com/docs).",
        )
        .tokens(app.tokens)
        .on_link(Message::Opened),
    );

    // section: Code Block Actions
    // A code block's copy control reports the block's source, which is where a
    // host writes the clipboard; without the message the control is not drawn.
    const SNIPPET: &str = "let document = TextView::new(source).compact(true);";

    let code = panel(CodeBlock::new(SNIPPET, Message::Copied(SNIPPET.to_owned())).language("rust"));

    // section: end
    // The one live control outside a sample: the line records the last message
    // the link or the copy action posted.
    let status_line = text(match &app.event {
        Some(Event::Opened(url)) => format!("Last opened: {url}"),
        Some(Event::Copied(code)) => {
            format!("Last copied: {}", code.lines().next().unwrap_or(""))
        }
        None => "Click a link or a copy control — the gallery is live.".to_owned(),
    })
    .size(theme::FONT_SIZE_XS)
    .style(|theme: &Theme| text::Style {
        color: Some(Tokens::of(theme).muted_foreground),
    });

    let gallery = column![
        section(app, 0, SECTIONS[0], markdown),
        section(app, 1, SECTIONS[1], html),
        section(app, 2, SECTIONS[2], clamp),
        section(app, 3, SECTIONS[3], link),
        section(app, 4, SECTIONS[4], code),
        status_line,
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

// The Lucide `</>` mark the code icon uses, from the icon set this repository
// already ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three
// galleries draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
