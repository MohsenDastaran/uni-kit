//! `text-view` — a rendered Markdown document, in the blocks, states and
//! spacings the catalog shares with GPUI and Slint.
//!
//! Run it with `cargo run --example text_view`, or in the browser as
//! `?story=text_view` in the gallery the website embeds.

use iced::widget::{column, container, scrollable, text};
use iced::{Element, Length, Task};

use crate::components::text_view::{
    Clamp, CodeBlock, Divider, Heading, Level, Paragraph, Quote, TextView,
};

pub fn run(theme: iced::Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — text-view")
        .theme(theme)
        .run()
}

/// The document the page is about. It carries every block the component draws,
/// so a reader can see them side by side.
const DOCUMENT: &str = "\
# Heading level one

The block above is a **level one** heading; the level below is a *level two*.
Headings keep whatever inline emphasis the source writes, because iced's
Markdown style has no heading weight to overwrite it.

## Heading level two

A paragraph wraps at word boundaries and carries `inline code`, **strong**,
*emphasised* text and a [link](https://example.com) in the link colour.

### Heading level three

> A quoted passage is set behind a two-pixel rule, in the muted foreground
> colour. It can hold more than one block.

1. An ordered list item
2. A second, so the numbering is visible

- An unordered item
- Another one

```rust
let document = TextView::new(source)
    .compact(true)
    .on_link(Message::Opened)
    .on_copy(Message::Copied);
```

---

The rule above is a divider. `<strong>HTML is not parsed</strong>` and is drawn
as those words, which is the one thing this component does not do that the
Slint original does.";

#[derive(Debug, Clone, Default)]
struct App {
    /// The last thing the document reported: a link that was clicked, or a code
    /// block whose copy action was pressed.
    event: Option<Event>,
}

#[derive(Debug, Clone)]
enum Event {
    Opened(String),
    Copied(String),
}

#[derive(Debug, Clone)]
enum Message {
    Opened(String),
    Copied(String),
}

fn boot() -> App {
    App::default()
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Opened(url) => app.event = Some(Event::Opened(url)),
        Message::Copied(code) => app.event = Some(Event::Copied(code)),
    }
    Task::none()
}

/// A label above a block, so the example says what each one is.
fn label<'a>(title: &'a str) -> Element<'a, Message> {
    text(title).size(12).into()
}

fn view(app: &App) -> Element<'_, Message> {
    // The single blocks below carry no message of their own, so they are lifted
    // into this example's message type here.
    let blocks = column![
        // The blocks the Slint file exports, each drawn on its own first.
        label("TextHeading — levels one through four"),
        Heading::new("Level one").level(Level::One),
        Heading::new("Level two").level(Level::Two),
        Heading::new("Level three").level(Level::Three),
        Heading::new("Level four").level(Level::Four),
        Divider::new(),
        label("TextParagraph"),
        Paragraph::new(
            "Body text, at text-size-sm, in the foreground colour, wrapping at word \
             boundaries."
        ),
        Divider::new(),
        label("TextQuote — a rule and the muted text behind it"),
        Quote::new("A quoted passage, set behind its own rule."),
        Divider::new(),
        label("TextCodeBlock — language, copy action, mono face"),
        CodeBlock::new("let x = 1;", Message::Copied("let x = 1;".to_owned()))
            .language("rust"),
        label("TextCodeBlock without a language draws no label row"),
        CodeBlock::new("plain", Message::Copied("plain".to_owned())),
        Divider::new(),
        label("TextDivider — one pixel of the border colour"),
        Divider::new(),
        label("TextClamp — a bounded preview, two lines by default"),
        Clamp::new(
            "This paragraph is longer than the two lines the box holds, so it stops at \
             the box's edge. The Slint component elides the last line; iced 0.14 has no \
             multi-line elision, so the clip is the edge instead."
        ),
        Clamp::new("A three-line preview, for a taller box.").lines(3),
        Divider::new(),
        // And the whole document, which is what the component is for.
        label("TextView — a parsed document, block spacing and all"),
        TextView::new(DOCUMENT)
            .on_link(Message::Opened)
            .on_copy(|code| Message::Copied(code.to_owned())),
        label("TextView::compact — the same blocks, 8px apart"),
        TextView::new(DOCUMENT)
            .compact(true)
            .on_link(Message::Opened)
            .on_copy(|code| Message::Copied(code.to_owned())),
        // The state the two click closures report.
        text(match &app.event {
            Some(Event::Opened(url)) => format!("opened {url}"),
            Some(Event::Copied(code)) => format!("copied {}", code.lines().next().unwrap_or("")),
            None => "click a link or a copy action".to_owned(),
        }),
    ]
    .spacing(12)
    .padding(16);

    let blocks: Element<'_, Message> = blocks.into();

    container(scrollable(blocks))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
