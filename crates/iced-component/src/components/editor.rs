//! A source-code editor. Mirrors the Slint `Editor` beside it: a header with the
//! open file and its language, a line-number gutter, a monospace field and a
//! status bar. It composes the landed [`Textarea`] for the field rather than
//! wrapping iced's `text_editor` a second time, which is where the frame, the
//! caret, the selection and the read-only behaviour already live.
//!
//! **What it draws and what it does not.** The header, the gutter and the status
//! bar are this component's; the value, the caret and the selection belong to the
//! [`Content`] the caller holds, exactly as they do in `Textarea`. An edit
//! arrives as an [`Action`] through [`on_action`](Editor::on_action), the caller
//! applies it with [`Content::perform`], and the status bar's line count follows
//! because it reads the same `Content` the field does.
//!
//! **Two things iced 0.14 cannot do, and neither is faked:**
//!
//! - **The gutter cannot share the field's scroll.** The Slint editor puts its
//!   gutter and its `TextInput` in one `Flickable`, so a number stays beside its
//!   line. iced's `text_editor` owns its own internal scroll and publishes no
//!   offset, so the gutter here is drawn statically against the field's top: it
//!   numbers the lines while they fit and simply stays put once the field
//!   scrolls, rather than pretending to follow it. It stops at 200 rows, which is
//!   the Slint component's own `max-rows`, so a long file does not draw a number
//!   for every line it has.
//! - **The frame cannot see the field's focus.** Slint draws the frame's edge in
//!   the ring colour while the field has focus. iced's editor reports focus in
//!   its own style, but a wrapper cannot ask, so the frame always draws the
//!   border colour and the field it composes keeps its edge switched off. The
//!   caret is the focus feedback, as it is in any plain text field.
//!
//! **The rest is named rather than approximated.** Syntax highlighting,
//! folding, search, decorations and multi-cursor editing are GPUI features with
//! no iced counterpart here: the field is plain text, as Slint's own field is,
//! and the language property is a label rather than a highlighter. `edited(text)`
//! becomes [`on_action`](Editor::on_action) because iced hands the caller an
//! `Action` to apply to its own `Content` rather than a finished string.
//!
//! An icon is the caller's, as it is for every component here: [`file_icon`]
//! and [`readonly_icon`](Editor::readonly_icon) take an element, and the caller
//! colours it — an SVG keeps its own black until a style closure says otherwise.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::Id;
use iced::widget::text::{LineHeight, Wrapping};
use iced::widget::text_editor::{Action, Content};
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Border, Color, Element, Length, Padding, Theme};

use super::textarea::Textarea;
use super::theme::{self, ControlSize, Tokens};

/// The line box the field draws at, as a multiple of the font size — the same
/// `1.45` `Textarea` gives its `text_editor`. The gutter numbers use it too, so
/// a number sits on its line while the field is at rest.
const LINE_HEIGHT: f32 = 1.45;

/// The header's height. The Slint editor fixes it at 36px.
const HEADER_HEIGHT: f32 = 36.0;

/// The status bar's height. The Slint editor fixes it at 28px.
const STATUS_HEIGHT: f32 = 28.0;

/// The rows the gutter numbers. Past this the field still scrolls and the gutter
/// stops rather than drawing a row it cannot keep in step — the Slint
/// component's `max-rows`.
const MAX_GUTTER_ROWS: usize = 200;

/// The field's size step, which owns its padding. Its font size is overridden by
/// [`Editor::font_size`], because the code size is the mono token rather than a
/// control step.
const FIELD: ControlSize = ControlSize::Small;

/// A source-code editor, built with the methods below.
pub struct Editor<'a, Message> {
    content: &'a Content,
    id: Option<Id>,
    filename: String,
    language: String,
    dirty: bool,
    indent_label: String,
    font_size: f32,
    enabled: bool,
    readonly: bool,
    show_header: bool,
    show_status: bool,
    show_gutter: bool,
    height: Length,
    file_icon: Option<Element<'a, Message>>,
    readonly_icon: Option<Element<'a, Message>>,
    on_action: Option<Box<dyn Fn(Action) -> Message + 'a>>,
}

impl<'a, Message> Editor<'a, Message> {
    /// An editor over the content the caller holds. The header reads `main.rs`
    /// in `Rust`, the gutter and the status bar are drawn, and the field is
    /// editable once [`on_action`](Self::on_action) has a message.
    pub fn new(content: &'a Content) -> Self {
        Self {
            content,
            id: None,
            filename: "main.rs".to_owned(),
            language: "Rust".to_owned(),
            dirty: false,
            indent_label: "Spaces: 4".to_owned(),
            font_size: theme::FONT_SIZE_MONO,
            enabled: true,
            readonly: false,
            show_header: true,
            show_status: true,
            show_gutter: true,
            height: Length::Fixed(260.0),
            file_icon: None,
            readonly_icon: None,
            on_action: None,
        }
    }

    /// The `Id` the window keeps the field's focus under, if the application
    /// needs to focus it from code or ask whether it is focused. The text and
    /// the caret live in the [`Content`], not here.
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// The file the header names, `main.rs` by default.
    pub fn filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = filename.into();
        self
    }

    /// The language the header and the status bar report, `Rust` by default. It
    /// is a label: iced draws no syntax colouring here, as the Slint editor
    /// leaves highlighting to the host.
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    /// Shows the unsaved-changes dot in the header, off by default.
    pub fn dirty(mut self, dirty: bool) -> Self {
        self.dirty = dirty;
        self
    }

    /// The status bar's indentation note, `Spaces: 4` by default.
    pub fn indent_label(mut self, indent_label: impl Into<String>) -> Self {
        self.indent_label = indent_label.into();
        self
    }

    /// The code size. The gutter's row height follows it, so the numbers stay
    /// beside their lines. The default is the theme's mono size, 13px.
    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    /// Refuses input and draws the field muted, the way Slint's `enabled: false`
    /// does. A field with no [`on_action`](Self::on_action) is disabled without
    /// this.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Rejects edits and keeps the frame, the text, the caret and the selection.
    /// The status bar says `Read only` instead of `Editable`.
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Draws the header. `false` leaves the field and the gutter alone, the way
    /// the Slint component's `show-header` does.
    pub fn show_header(mut self, show_header: bool) -> Self {
        self.show_header = show_header;
        self
    }

    /// Draws the status bar. `false` drops it, the way the Slint component's
    /// `show-status` does.
    pub fn show_status(mut self, show_status: bool) -> Self {
        self.show_status = show_status;
        self
    }

    /// Draws the line-number gutter. `false` gives the whole width to the field.
    pub fn show_gutter(mut self, show_gutter: bool) -> Self {
        self.show_gutter = show_gutter;
        self
    }

    /// The editor's height. The Slint component's preferred height is 260px and
    /// that is the default; a demo that stacks several editors asks for less.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// The glyph the header leads with — a file mark the caller embeds and
    /// colours, since the icon set belongs to the application.
    pub fn file_icon(mut self, icon: Element<'a, Message>) -> Self {
        self.file_icon = Some(icon);
        self
    }

    /// The glyph the status bar puts beside `Read only`.
    pub fn readonly_icon(mut self, icon: Element<'a, Message>) -> Self {
        self.readonly_icon = Some(icon);
        self
    }

    /// Every interaction, as the action the field performed. Apply it to the
    /// content — `content.perform(action)` — or the field will not change. A
    /// disabled editor publishes nothing, which is how this crate spells a
    /// control that cannot be used.
    pub fn on_action(mut self, on_action: impl Fn(Action) -> Message + 'a) -> Self {
        self.on_action = Some(Box::new(on_action));
        self
    }
}

/// The width the gutter needs for a file of `line_count` rows, which is what the
/// Slint component computes: the digit column plus an 8px inset, and another
/// column once the numbers reach two digits and then three.
fn gutter_width(font_size: f32, line_count: usize) -> f32 {
    font_size * 1.2
        + 8.0
        + if line_count > 9 { font_size * 0.62 } else { 0.0 }
        + if line_count > 99 {
            font_size * 0.62
        } else {
            0.0
        }
}

/// A filled dot: the unsaved-changes mark and the language mark. The colour is
/// chosen from the tokens inside the style closure, never read while the element
/// is built.
fn dot<'a, Message: 'a>(tone: fn(Tokens) -> Color, size: f32) -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .style(move |theme: &Theme| container::Style {
            background: Some(tone(Tokens::of(theme)).into()),
            border: Border {
                radius: (size / 2.0).into(),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .into()
}

/// A status-bar label: the small muted text the bar puts on both sides.
fn muted<'a, Message: 'a>(label: impl Into<String>) -> Element<'a, Message> {
    text(label.into())
        .size(11.0)
        .style(|theme: &Theme| text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// The header: the file glyph, the name, the unsaved dot, and the language on
/// the trailing edge.
fn header<'a, Message: 'a>(
    filename: &str,
    language: &str,
    dirty: bool,
    icon: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut bar = row![]
        .spacing(theme::SPACING_SM)
        .align_y(Alignment::Center)
        .padding([0.0, theme::SPACING_MD]);

    if let Some(icon) = icon {
        bar = bar.push(icon);
    }

    bar = bar.push(
        text(filename.to_owned())
            .size(theme::FONT_SIZE_XS)
            .font(iced::Font {
                weight: iced::font::Weight::Semibold,
                ..iced::Font::DEFAULT
            })
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).foreground),
            }),
    );

    if dirty {
        bar = bar.push(dot(|tokens| tokens.warning, 7.0));
    }

    bar = bar.push(Space::new().width(Length::Fill));

    if !language.is_empty() {
        bar = bar.push(dot(|tokens| tokens.success, 6.0));
        bar = bar.push(muted(language.to_owned()));
    }

    // The header sits inside the frame's rounded edge, so its own fill carries
    // the top corners rather than poking a square corner through them.
    container(bar)
        .width(Length::Fill)
        .height(Length::Fixed(HEADER_HEIGHT))
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).sidebar.into()),
            border: Border {
                radius: iced::border::top(theme::RADIUS_LG),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .into()
}

/// The line-number gutter: one right-aligned number per row, on the muted
/// surface, with the row height the field draws at. The numbers start at the
/// field's own top padding so the first one sits on the first line.
fn gutter<'a, Message: 'a>(line_count: usize, font_size: f32, padding: f32) -> Element<'a, Message> {
    let numbers = (0..line_count.min(MAX_GUTTER_ROWS)).map(|index| {
        text(format!("{}", index + 1))
            .size(font_size)
            .font(theme::MONO)
            .line_height(LineHeight::Relative(LINE_HEIGHT))
            .width(Length::Fill)
            .align_x(Alignment::End)
            .style(|theme: &Theme| text::Style {
                color: Some(theme::alpha(Tokens::of(theme).muted_foreground, 0.8)),
            })
            .into()
    });

    container(
        column(numbers)
            .spacing(0.0)
            .padding(Padding {
                top: padding,
                right: theme::SPACING_SM,
                ..Padding::ZERO
            })
            .width(Length::Fill),
    )
    .width(Length::Fixed(gutter_width(font_size, line_count)))
    .height(Length::Fill)
    .style(|theme: &Theme| container::Style {
        background: Some(theme::alpha(Tokens::of(theme).muted, 0.4).into()),
        ..container::Style::default()
    })
    .into()
}

/// The one-pixel rule between the gutter and the field.
fn separator<'a, Message: 'a>() -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fixed(1.0))
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).border.into()),
            ..container::Style::default()
        })
        .into()
}

/// The status bar: the language and the line count on the leading edge, the
/// indentation, the encoding and the editing state on the trailing one.
fn status<'a, Message: 'a>(
    language: &str,
    line_count: usize,
    indent_label: &str,
    enabled: bool,
    readonly: bool,
    readonly_icon: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let lines = format!(
        "{} line{}",
        line_count,
        if line_count == 1 { "" } else { "s" }
    );

    let mut bar = row![]
        .spacing(14.0)
        .align_y(Alignment::Center)
        .padding([0.0, theme::SPACING_MD]);

    if !language.is_empty() {
        bar = bar.push(muted(language.to_owned()));
    }
    bar = bar.push(muted(lines));
    bar = bar.push(Space::new().width(Length::Fill));
    bar = bar.push(muted(indent_label.to_owned()));
    bar = bar.push(muted("UTF-8"));

    if readonly {
        let mut mark = row![].spacing(theme::SPACING_XS).align_y(Alignment::Center);
        if let Some(icon) = readonly_icon {
            mark = mark.push(icon);
        }
        mark = mark.push(muted("Read only"));
        bar = bar.push(mark);
    } else {
        bar = bar.push(muted(if enabled { "Editable" } else { "Disabled" }));
    }

    container(bar)
        .width(Length::Fill)
        .height(Length::Fixed(STATUS_HEIGHT))
        .style(|theme: &Theme| container::Style {
            background: Some(Tokens::of(theme).sidebar.into()),
            border: Border {
                radius: iced::border::bottom(theme::RADIUS_LG),
                ..Border::default()
            },
            ..container::Style::default()
        })
        .into()
}

impl<'a, Message> From<Editor<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(control: Editor<'a, Message>) -> Self {
        let line_count = control.content.line_count();
        let font_size = control.font_size;
        let padding = FIELD.input_padding_x();
        let enabled = control.enabled;
        let readonly = control.readonly;
        let show_header = control.show_header;
        let show_status = control.show_status;
        let show_gutter = control.show_gutter;
        let filename = control.filename.clone();
        let language = control.language.clone();
        let dirty = control.dirty;
        let indent_label = control.indent_label.clone();
        let height = control.height;
        let file_icon = control.file_icon;
        let readonly_icon = control.readonly_icon;

        // The field composes the landed multi-line control: the frame is the
        // editor's, so the field's own edge, fill and radius are switched off,
        // and it takes the height the frame gives it.
        let mut field = Textarea::new(control.content)
            .appearance(false)
            .size(FIELD)
            .font_size(font_size)
            // A code line scrolls rather than breaking at a word, which is the
            // Slint field's `wrap: no-wrap`.
            .wrapping(Wrapping::None)
            .height(Length::Fill);

        if let Some(id) = control.id {
            field = field.id(id);
        }
        // A disabled editor carries no message: iced reads a missing `on_action`
        // as the disabled state, and one it cannot report would drop the edit.
        if enabled {
            if let Some(on_action) = control.on_action {
                field = field.on_action(on_action);
            }
        }
        if readonly {
            field = field.readonly(true);
        }

        let mut body = row![].spacing(0.0).height(Length::Fill);
        if show_gutter {
            body = body.push(gutter(line_count, font_size, padding));
            body = body.push(separator());
        }
        body = body.push(field);

        let mut sections = column![].spacing(0.0);
        if show_header {
            sections = sections.push(header(&filename, &language, dirty, file_icon));
        }
        sections = sections.push(body);
        if show_status {
            sections = sections.push(status(
                &language,
                line_count,
                &indent_label,
                enabled,
                readonly,
                readonly_icon,
            ));
        }

        container(sections)
            .width(Length::Fill)
            .height(height)
            .clip(true)
            .style(|theme: &Theme| container::Style {
                background: Some(Tokens::of(theme).background.into()),
                text_color: Some(Tokens::of(theme).foreground),
                border: Border {
                    color: Tokens::of(theme).border,
                    width: 1.0,
                    radius: theme::RADIUS_LG.into(),
                },
                ..container::Style::default()
            })
            .into()
    }
}
