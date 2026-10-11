//! A rendered document. Mirrors the Slint `TextHeading`, `TextParagraph`,
//! `TextQuote`, `TextCodeBlock`, `TextDivider` and `TextClamp`, and the block
//! spacing `TextView` stacks them with.
//!
//! **Markdown is rendered; HTML is not.** Slint's `text-view` renders Markdown
//! *and* HTML — its `TextParagraph` is documented as taking a `@markdown(..)`
//! literal — and the GPUI Component `TextView` it mirrors parses both. iced 0.14
//! ships a Markdown renderer and no HTML parser at all, so [`TextView::new`]
//! feeds its source to [`markdown::parse`](iced::widget::markdown::parse) and
//! nothing interprets tags: `<strong>bold</strong>` is drawn as those words and
//! their brackets, in the body colour. That is the documented gap; the closest
//! real equivalent is the parser that is actually in the dependency.
//!
//! **Two Slint properties have no exact iced counterpart**, and both resolve to
//! a real replacement rather than a stand-in:
//!
//! - `TextHeading` sets `font-weight: 650`, and Slint's `StyledText` has no
//!   weight property, which is why its headings carry no inline markup. iced's
//!   `markdown::Style` has no heading weight either, so a Markdown heading
//!   draws at the Slint size for its level and keeps whatever emphasis the
//!   source wrote. [`Heading`] is the hand-written heading, and it takes the
//!   weight — the nearest step the font weight scale offers is semibold.
//! - `TextCodeBlock` is a filled surface with a language label and a copy
//!   action. iced's default code block is a horizontal scroller with no header,
//!   so [`TextView`] draws the surface, the label and the copy button itself,
//!   out of the same [`Button`](super::button) the catalog uses for a ghost
//!   extra-small control. The copy action is real: it produces the block's
//!   source as a message for the caller.
//!
//! The remaining Slint properties map straight across: `compact` selects 8px or
//! 14px between blocks, the quote rule is 2px wide with the text 12px behind it,
//! the divider is 1px of the border colour, and `lines` on [`Clamp`] caps the
//! preview at the height Slint computes — `lines * font-size-sm * 1.45`.
//!
//! **Colours are read at render time.** Every plain block resolves its colour
//! inside a `style` closure through `Tokens::of(theme)`, so a document follows
//! the iced theme in force. The one value the Markdown parser needs while it
//! builds its runs — a link's colour — is supplied for both palettes and the
//! parser caches its runs per palette, so a theme switch re-derives the other
//! set.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use std::rc::Rc;

use iced::widget::markdown::{self, Bullet, HeadingLevel, Item, Text};
use iced::widget::text::Wrapping;
use iced::widget::{button, column, container, rich_text, row, rule, span, text};
use iced::{Border, Element, Font, Length, Padding, Pixels, Shadow, Theme, font};

use super::theme::{self, Tokens};

/// The line height Slint's `TextClamp` budgets a body line at, as the multiple
/// of the font size it multiplies by.
const CLAMP_LINE_HEIGHT: f32 = 1.45;

/// The block spacing the Slint `TextView` uses: 8px compact, 14px otherwise.
fn block_spacing(compact: bool) -> f32 {
    if compact {
        theme::SPACING_SM
    } else {
        14.0
    }
}

/// The Markdown text style for each palette, as a document parses against them.
fn palettes() -> (markdown::Style, markdown::Style) {
    (
        markdown::Style::from_palette(Theme::Light.palette()),
        markdown::Style::from_palette(Theme::Dark.palette()),
    )
}

/// The Markdown size table, in the units Slint's `TextHeading` selects between.
fn settings(compact: bool) -> markdown::Settings {
    markdown::Settings {
        text_size: Pixels(theme::FONT_SIZE_SM),
        h1_size: Pixels(theme::FONT_SIZE_XL),
        h2_size: Pixels(theme::FONT_SIZE_LG),
        h3_size: Pixels(theme::FONT_SIZE_MD),
        h4_size: Pixels(theme::FONT_SIZE_SM),
        h5_size: Pixels(theme::FONT_SIZE_SM),
        h6_size: Pixels(theme::FONT_SIZE_SM),
        code_size: Pixels(theme::FONT_SIZE_MONO),
        spacing: Pixels(block_spacing(compact)),
        // The parser colours a run while it builds it, so it is handed a
        // palette here; `palettes` gives the style closure the other one.
        style: markdown::Style::from_palette(Theme::Light.palette()),
    }
}

// ---------------------------------------------------------------- heading

/// The size step a heading draws at, 1 the largest. Slint's `level` with its
/// arms clamped the same way: there is no fifth size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Level {
    One,
    #[default]
    Two,
    Three,
    Four,
}

impl Level {
    /// The token size for the level: xl, lg, md, sm.
    pub fn font_size(self) -> f32 {
        match self {
            Self::One => theme::FONT_SIZE_XL,
            Self::Two => theme::FONT_SIZE_LG,
            Self::Three => theme::FONT_SIZE_MD,
            Self::Four => theme::FONT_SIZE_SM,
        }
    }

    /// The size a parsed Markdown heading of this level draws at, so a `#`
    /// heading and a hand-written [`Heading`] line up.
    pub fn of_markdown(level: &HeadingLevel) -> Self {
        match level {
            HeadingLevel::H1 => Self::One,
            HeadingLevel::H2 => Self::Two,
            HeadingLevel::H3 => Self::Three,
            _ => Self::Four,
        }
    }
}

/// A section heading: the level's size, at 650 weight, in the foreground
/// colour, wrapping at word boundaries.
///
/// This is the plain-text heading. A Markdown heading arrives through
/// [`TextView`], where the source's own emphasis survives and this weight does
/// not apply — see the module docs.
pub struct Heading<'a> {
    text: &'a str,
    level: Level,
}

impl<'a> Heading<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            level: Level::default(),
        }
    }

    /// Draws at this level rather than the default second.
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}

impl<'a, Message: 'a> From<Heading<'a>> for Element<'a, Message> {
    fn from(heading: Heading<'a>) -> Self {
        text(heading.text)
            .size(heading.level.font_size())
            .font(Font {
                weight: font::Weight::Semibold,
                ..Font::DEFAULT
            })
            .wrapping(Wrapping::Word)
            .style(|theme: &Theme| iced::widget::text::Style {
                color: Some(Tokens::of(theme).foreground),
            })
            .into()
    }
}

// ---------------------------------------------------------------- paragraph

/// Body text. Markdown arrives through [`TextView`]; this is the hand-written
/// half of the same block, so a paragraph can be placed on its own.
pub struct Paragraph<'a> {
    text: &'a str,
}

impl<'a> Paragraph<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl<'a, Message: 'a> From<Paragraph<'a>> for Element<'a, Message> {
    fn from(paragraph: Paragraph<'a>) -> Self {
        let runs: [iced::widget::text::Span<'_, ()>; 1] =
            [span(paragraph.text).color(theme::LIGHT.foreground)];

        rich_text(runs)
            .size(Pixels(theme::FONT_SIZE_SM))
            .wrapping(Wrapping::Word)
            .style(|theme: &Theme| iced::widget::text::Style {
                color: Some(Tokens::of(theme).foreground),
            })
            .into()
    }
}

// ---------------------------------------------------------------- quote

/// The rule a quoted passage is set behind: 2px of the border colour, the way
/// the Slint block's `Rectangle` draws it.
fn quote_rule<'a, Message: 'a>() -> Element<'a, Message> {
    rule::vertical(2)
        .style(|theme: &Theme| rule::Style {
            color: Tokens::of(theme).border,
            radius: 1.0.into(),
            ..rule::default(theme)
        })
        .into()
}

/// The rule and the passage 12px behind it, whatever draws the passage.
fn quoted<'a, Message: 'a>(contents: Element<'a, Message>) -> Element<'a, Message> {
    row![
        quote_rule(),
        container(contents).width(Length::Fill),
    ]
    .spacing(theme::SPACING_MD)
    .align_y(iced::Alignment::Start)
    .into()
}

/// A quoted passage: a rule down the left with the text set behind it, in the
/// muted colour.
pub struct Quote<'a> {
    text: &'a str,
}

impl<'a> Quote<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
}

impl<'a, Message: 'a> From<Quote<'a>> for Element<'a, Message> {
    fn from(quote: Quote<'a>) -> Self {
        let runs: [iced::widget::text::Span<'_, ()>; 1] =
            [span(quote.text).color(theme::LIGHT.muted_foreground)];

        quoted(
            rich_text(runs)
                .size(Pixels(theme::FONT_SIZE_SM))
                .wrapping(Wrapping::Word)
                .style(|theme: &Theme| iced::widget::text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                })
                .into(),
        )
    }
}

// ---------------------------------------------------------------- code block

/// The surface, the label row and the code, laid out the way both a
/// hand-written block and a parsed fence draw.
///
/// The copy control is drawn whenever the caller has a message for it; without
/// one it is not drawn at all, because a copy action with nothing to send is not
/// a copy action.
fn code_surface<'a, Message: Clone + 'a>(
    code: &'a str,
    language: &'a str,
    on_copy: Option<Message>,
) -> Element<'a, Message> {
    let mut body = column![].spacing(theme::SPACING_SM);

    if !language.is_empty() {
        let mut label = row![
            text(language)
                .size(theme::FONT_SIZE_XS)
                .width(Length::Fill)
                .align_y(iced::Alignment::Center)
                .style(|theme: &Theme| iced::widget::text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        ]
        .spacing(theme::SPACING_SM)
        .align_y(iced::Alignment::Center);

        if let Some(on_copy) = on_copy {
            label = label.push(copy_button(on_copy));
        }

        body = body.push(label);
    }

    body = body.push(
        text(code)
            .size(theme::FONT_SIZE_MONO)
            .font(theme::MONO)
            .wrapping(Wrapping::Word)
            .style(|theme: &Theme| iced::widget::text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
    );

    container(body)
        .padding(Pixels(theme::SPACING_MD))
        .width(Length::Fill)
        .style(|theme: &Theme| {
            let tokens = Tokens::of(theme);
            container::Style {
                background: Some(tokens.muted.into()),
                border: Border {
                    color: tokens.border,
                    width: 1.0,
                    radius: theme::RADIUS_MD.into(),
                },
                text_color: Some(tokens.foreground),
                ..container::Style::default()
            }
        })
        .into()
}

/// The copy control on a code block's label row: a ghost extra-small button, the
/// look [`Button`](super::button) gives `Variant::Ghost` at `ControlSize::XSmall`
/// — no fill of its own, xsmall-foreground ink, a 6px pill.
fn copy_button<'a, Message: Clone + 'a>(on_copy: Message) -> Element<'a, Message> {
    button(text("Copy").size(theme::FONT_SIZE_XS))
        .padding(Padding {
            top: 2.0,
            right: 6.0,
            bottom: 2.0,
            left: 6.0,
        })
        .on_press(on_copy)
        .style(|theme: &Theme, _status| {
            let tokens = Tokens::of(theme);
            button::Style {
                text_color: tokens.secondary_foreground,
                background: None,
                border: Border::default(),
                shadow: Shadow::default(),
                snap: false,
            }
        })
        .into()
}

/// A fenced code block: the muted surface, the language it is written in, the
/// copy action a reader expects on one, and the code in the mono face at the
/// mono size.
pub struct CodeBlock<'a, Message> {
    code: &'a str,
    language: &'a str,
    on_copy: Message,
}

impl<'a, Message> CodeBlock<'a, Message> {
    /// A block of code, and the message its copy control sends.
    pub fn new(code: &'a str, on_copy: Message) -> Self {
        Self {
            code,
            language: "",
            on_copy,
        }
    }

    /// The language the block is written in. Empty draws no label row, the way
    /// the Slint block's `if language != ""` does.
    pub fn language(mut self, language: &'a str) -> Self {
        self.language = language;
        self
    }
}

impl<'a, Message: 'a> From<CodeBlock<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(block: CodeBlock<'a, Message>) -> Self {
        code_surface(block.code, block.language, Some(block.on_copy))
    }
}

// ---------------------------------------------------------------- divider

/// A rule between sections: one pixel of the border colour.
pub struct Divider;

impl Divider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: 'a> From<Divider> for Element<'a, Message> {
    fn from(_divider: Divider) -> Self {
        rule::horizontal(1)
            .style(|theme: &Theme| rule::Style {
                color: Tokens::of(theme).border,
                ..rule::default(theme)
            })
            .into()
    }
}

// ---------------------------------------------------------------- clamp

/// A bounded preview of body text: the box holds `lines` lines and stops.
///
/// Slint sets `overflow: elide` on a plain `Text`; iced 0.14 elides unwrapped
/// single-line text only, and a wrapped paragraph has no multi-line elision, so
/// the box takes the height Slint computes and clips at its edge. A line that
/// would straddle the bottom is not drawn past it, and no ellipsis is drawn in
/// its place.
pub struct Clamp<'a> {
    text: &'a str,
    lines: usize,
}

impl<'a> Clamp<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, lines: 2 }
    }

    /// The number of body lines the preview holds.
    pub fn lines(mut self, lines: usize) -> Self {
        self.lines = lines.max(1);
        self
    }
}

impl<'a, Message: 'a> From<Clamp<'a>> for Element<'a, Message> {
    fn from(clamp: Clamp<'a>) -> Self {
        let height = clamp.lines as f32 * theme::FONT_SIZE_SM * CLAMP_LINE_HEIGHT;

        container(
            text(clamp.text)
                .size(theme::FONT_SIZE_SM)
                .wrapping(Wrapping::Word)
                .style(|theme: &Theme| iced::widget::text::Style {
                    color: Some(Tokens::of(theme).foreground),
                }),
        )
        .height(Length::Fixed(height))
        .width(Length::Fill)
        .clip(true)
        .into()
    }
}

// ---------------------------------------------------------------- document

/// A run of Markdown text at the size its block draws it, in the theme's colours
/// and with the document's link message attached.
///
/// Every run the parser builds carries its own colour — a link takes the
/// palette's link colour, inline code takes the palette's inline-code colour and
/// the rest take the body colour — so the run is parsed against the light
/// palette and the style closure picks out the colour the same run took in the
/// theme's own palette. The parser caches its runs per palette, so a theme
/// switch re-derives the other set.
fn run<'a, Message: Clone + 'a>(
    contents: &Text,
    size: f32,
    on_link: &Option<Rc<dyn Fn(markdown::Uri) -> Message + 'a>>,
) -> Element<'a, Message> {
    let (light, dark) = palettes();
    let runs = contents.spans(light);
    let on_link = on_link.clone();

    let mut body = rich_text(runs)
        .size(Pixels(size))
        .wrapping(Wrapping::Word)
        .style(move |theme: &Theme| {
            let style = if Tokens::of(theme).dark { dark } else { light };
            iced::widget::text::Style {
                color: Some(style.link_color),
            }
        });

    if let Some(on_link) = on_link {
        body = body.on_link_click(move |uri| on_link(uri));
    }

    body.into()
}

/// The Markdown block renderer: one parsed [`Item`] into the element the Slint
/// component beside it draws.
struct Viewer<'a, Message> {
    compact: bool,
    on_link: Option<Rc<dyn Fn(markdown::Uri) -> Message + 'a>>,
    on_copy: Option<Rc<dyn Fn(&str) -> Message + 'a>>,
}

impl<'a, Message> Viewer<'a, Message> {
    /// The block spacing this viewer lays out with.
    fn spacing(&self) -> f32 {
        block_spacing(self.compact)
    }
}

impl<'a, Message> markdown::Viewer<'a, Message, Theme, iced::Renderer> for Viewer<'a, Message>
where
    Message: Clone + 'a,
{
    fn on_link_click(_url: markdown::Uri) -> Message {
        unreachable!("a document's links are delivered through its own closure, not this one")
    }

    fn heading(
        &self,
        settings: markdown::Settings,
        level: &'a HeadingLevel,
        contents: &'a Text,
        index: usize,
    ) -> Element<'a, Message, Theme, iced::Renderer> {
        let top = if index > 0 {
            settings.text_size.0 / 2.0
        } else {
            0.0
        };

        container(run(
            contents,
            Level::of_markdown(level).font_size(),
            &self.on_link,
        ))
        .padding(Padding {
            top,
            ..Padding::ZERO
        })
        .into()
    }

    fn paragraph(
        &self,
        settings: markdown::Settings,
        contents: &Text,
    ) -> Element<'a, Message, Theme, iced::Renderer> {
        run(contents, settings.text_size.0, &self.on_link)
    }

    fn code_block(
        &self,
        _settings: markdown::Settings,
        language: Option<&'a str>,
        code: &'a str,
        _lines: &'a [Text],
    ) -> Element<'a, Message, Theme, iced::Renderer> {
        let on_copy = self.on_copy.as_ref().map(|on_copy| on_copy(code));
        code_surface(code, language.unwrap_or_default(), on_copy)
    }

    fn quote(
        &self,
        settings: markdown::Settings,
        contents: &'a [Item],
    ) -> Element<'a, Message, Theme, iced::Renderer> {
        let runs = contents
            .iter()
            .enumerate()
            .map(|(index, item)| markdown::item(self, settings, item, index));

        quoted(
            container(column(runs).spacing(settings.spacing.0))
                .width(Length::Fill)
                .style(|theme: &Theme| container::Style {
                    text_color: Some(Tokens::of(theme).muted_foreground),
                    ..container::Style::default()
                })
                .into(),
        )
    }

    fn rule(
        &self,
        _settings: markdown::Settings,
    ) -> Element<'a, Message, Theme, iced::Renderer> {
        rule::horizontal(1)
            .style(|theme: &Theme| rule::Style {
                color: Tokens::of(theme).border,
                ..rule::default(theme)
            })
            .into()
    }

    fn unordered_list(
        &self,
        settings: markdown::Settings,
        bullets: &'a [Bullet],
    ) -> Element<'a, Message, Theme, iced::Renderer> {
        let spacing = self.spacing();
        let runs = bullets.iter().map(|bullet| {
            row![
                text(marker(bullet))
                    .size(settings.text_size)
                    .style(|theme: &Theme| iced::widget::text::Style {
                        color: Some(Tokens::of(theme).foreground),
                    }),
                markdown::view_with(bullet_contents(bullet), settings, self),
            ]
            .spacing(spacing)
            .into()
        });

        column(runs).spacing(spacing * 0.75).into()
    }

    fn ordered_list(
        &self,
        settings: markdown::Settings,
        start: u64,
        bullets: &'a [Bullet],
    ) -> Element<'a, Message, Theme, iced::Renderer> {
        let spacing = self.spacing();
        let runs = bullets.iter().enumerate().map(|(offset, bullet)| {
            row![
                text(numbered_marker(start, offset, bullet))
                    .size(settings.text_size)
                    .style(|theme: &Theme| iced::widget::text::Style {
                        color: Some(Tokens::of(theme).foreground),
                    }),
                markdown::view_with(bullet_contents(bullet), settings, self),
            ]
            .spacing(spacing)
            .into()
        });

        column(runs).spacing(spacing * 0.75).into()
    }
}

/// The contents of a list item. iced keeps `Bullet::items` to itself, so the
/// variant is opened here instead.
fn bullet_contents(bullet: &Bullet) -> &[Item] {
    match bullet {
        Bullet::Point { items } | Bullet::Task { items, .. } => items,
    }
}

/// The mark a bullet list item leads with.
fn marker(bullet: &Bullet) -> String {
    match bullet {
        Bullet::Task { done, .. } => {
            if *done {
                "\u{2611}".to_owned()
            } else {
                "\u{2610}".to_owned()
            }
        }
        Bullet::Point { .. } => "\u{2022}".to_owned(),
    }
}

/// The mark an ordered list item leads with: its number from the list's start.
fn numbered_marker(start: u64, offset: usize, bullet: &Bullet) -> String {
    match bullet {
        Bullet::Task { .. } | Bullet::Point { .. } => format!("{}.", start + offset as u64),
    }
}

// The whole-document widget. iced's `markdown::view_with` is an ordinary
// function that turns a slice of parsed items into an element, and the element
// it produces borrows those items, so it cannot be built and returned from the
// same call. A widget can: it owns the parse, and the element it builds lives
// only inside the trait methods.

/// A parsed document and the two closures its clicks answer to.
struct Render<'a, Message> {
    compact: bool,
    on_link: Option<Rc<dyn Fn(markdown::Uri) -> Message + 'a>>,
    on_copy: Option<Rc<dyn Fn(&str) -> Message + 'a>>,
}

impl<'a, Message> Render<'a, Message> {
    /// The document's size table.
    fn settings(&self) -> markdown::Settings {
        settings(self.compact)
    }
}

/// A parsed Markdown document, as a widget: it keeps the parse and lays it out
/// with [`markdown::view_with`](iced::widget::markdown::view_with).
struct Document<'a, Message> {
    items: Vec<Item>,
    render: Render<'a, Message>,
}

impl<'a, Message> Document<'a, Message> {
    /// The parsed blocks as one element, borrowed from `self` for as long as the
    /// element is used.
    fn view(&self) -> Element<'_, Message, Theme, iced::Renderer>
    where
        Message: Clone + 'a,
    {
        let viewer = Viewer {
            compact: self.render.compact,
            on_link: self.render.on_link.clone(),
            on_copy: self.render.on_copy.clone(),
        };

        markdown::view_with(self.items.iter(), self.render.settings(), &viewer)
    }
}

impl<'a, Message: 'a> From<Document<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(document: Document<'a, Message>) -> Self {
        Element::new(document)
    }
}

impl<'a, Message> iced_core::Widget<Message, Theme, iced::Renderer>
    for Document<'a, Message>
where
    Message: Clone + 'a,
{
    fn tag(&self) -> iced_core::widget::tree::Tag {
        iced_core::widget::tree::Tag::of::<()>()
    }

    fn state(&self) -> iced_core::widget::tree::State {
        iced_core::widget::tree::State::new(())
    }

    fn children(&self) -> Vec<iced_core::widget::Tree> {
        let element = self.view();
        let widget: &dyn iced_core::Widget<Message, Theme, iced::Renderer> =
            element.as_widget();
        widget.children()
    }

    fn diff(&self, tree: &mut iced_core::widget::Tree) {
        let element = self.view();
        let widget: &dyn iced_core::Widget<Message, Theme, iced::Renderer> =
            element.as_widget();
        *tree = iced_core::widget::Tree {
            tag: iced_core::widget::tree::Tag::of::<()>(),
            state: iced_core::widget::tree::State::new(()),
            children: widget.children(),
        };
    }

    fn size(&self) -> iced_core::Size<Length> {
        self.view().as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        renderer: &iced::Renderer,
        limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        let mut element = self.view();

        element
            .as_widget_mut()
            .layout(tree, renderer, limits)
    }

    fn draw(
        &self,
        tree: &iced_core::widget::Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &iced_core::renderer::Style,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
    ) {
        self.view().as_widget().draw(
            tree, renderer, theme, style, layout, cursor, viewport,
        );
    }

    fn operate(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        layout: iced_core::Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced_core::widget::Operation,
    ) {
        let mut element = self.view();

        element
            .as_widget_mut()
            .operate(tree, layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut iced_core::widget::Tree,
        event: &iced_core::Event,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn iced_core::Clipboard,
        shell: &mut iced_core::Shell<'_, Message>,
        viewport: &iced_core::Rectangle,
    ) {
        let mut element = self.view();

        element.as_widget_mut().update(
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &iced_core::widget::Tree,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        viewport: &iced_core::Rectangle,
        renderer: &iced::Renderer,
    ) -> iced_core::mouse::Interaction {
        self.view().as_widget().mouse_interaction(
            tree, layout, cursor, viewport, renderer,
        )
    }

    // The element is built inside each call above, so there is no stored child
    // to hand an overlay out of; Markdown draws no overlays of its own.
    fn overlay<'b>(
        &'b mut self,
        _tree: &'b mut iced_core::widget::Tree,
        _layout: iced_core::Layout<'b>,
        _renderer: &iced::Renderer,
        _viewport: &iced_core::Rectangle,
        _translation: iced_core::Vector,
    ) -> Option<iced_core::overlay::Element<'b, Message, Theme, iced::Renderer>> {
        None
    }
}

/// A rendered Markdown document, built with the methods below.
///
/// The spacing between blocks is the Slint `TextView`'s: 14px for a page, 8px
/// when [`compact`](Self::compact) asks for a preview. The message type follows
/// the click closures: without [`on_link`](Self::on_link) a link draws in the
/// link colour and produces its own URL, and without [`on_copy`](Self::on_copy)
/// a code block's copy control is not drawn.
///
/// ```no_run
/// use iced_component::components::text_view::TextView;
///
/// enum Message {
///     Link(String),
///     Copy(String),
/// }
///
/// fn view(source: &str) -> iced::Element<'_, Message> {
///     TextView::new(source)
///         .compact(true)
///         .on_link(Message::Link)
///         .on_copy(Message::Copy)
///         .into()
/// }
/// ```
pub struct TextView<'a, Message = markdown::Uri> {
    source: &'a str,
    compact: bool,
    on_link: Option<Rc<dyn Fn(markdown::Uri) -> Message + 'a>>,
    on_copy: Option<Rc<dyn Fn(&str) -> Message + 'a>>,
}

impl<'a, Message> TextView<'a, Message> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            compact: false,
            on_link: None,
            on_copy: None,
        }
    }

    /// Tighter spacing, for a preview rather than a page.
    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    /// The message a clicked link produces. The URL is the Markdown
    /// destination, unopened: what a link does belongs to the application.
    pub fn on_link(mut self, on_link: impl Fn(markdown::Uri) -> Message + 'a) -> Self {
        self.on_link = Some(Rc::new(on_link));
        self
    }

    /// The message the copy action on a code block produces, carrying the
    /// block's source.
    pub fn on_copy(mut self, on_copy: impl Fn(&str) -> Message + 'a) -> Self {
        self.on_copy = Some(Rc::new(on_copy));
        self
    }
}

impl<'a, Message: 'a> From<TextView<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(view: TextView<'a, Message>) -> Self {
        Document {
            items: markdown::parse(view.source).collect(),
            render: Render {
                compact: view.compact,
                on_link: view.on_link,
                on_copy: view.on_copy,
            },
        }
        .into()
    }
}
