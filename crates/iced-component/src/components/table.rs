//! A static table. Mirrors GPUI Component's `Table` and the Slint version beside
//! it: a bordered, rounded surface that stacks a [`TableHeader`], a
//! [`TableBody`], an optional [`TableFooter`] and an optional [`TableCaption`].
//! Rows are [`TableRow`]s of [`TableHead`] or [`TableCell`]; cells share the row
//! width equally unless given an explicit [`TableHead::width`], and they carry
//! their own alignment and size.
//!
//! It is deliberately not a data table: no virtual scrolling, sorting, resizing,
//! selection, or keyboard navigation. GPUI's `DataTable` is the component for
//! that, and the Slint side has the same split.
//!
//! **The body is a [`Scrollable`](super::scrollable::Scrollable).** The Slint
//! table stacks its sections and lets the page scroll; the roadmap puts this one
//! on the styled scroll area, so a [`TableBody::height`] bounds the rows and the
//! header and footer stay put while the body scrolls. With no height the body
//! sizes to its rows and draws no bar, which is the Slint component's behaviour.
//!
//! # What iced cannot draw, named rather than faked
//!
//! - **A column has no minimum or preferred width.** Slint asks for
//!   `min-width: 36px` and `preferred-width: 100px` and then stretches the
//!   column; iced's [`Length`] offers `Fill`/`FillPortion` or `Fixed`, so a cell
//!   without an explicit width takes its `col-span` as a fill share and the
//!   caller sets a fixed width where the reference sets `width`.
//! - **A cell does not wrap.** Slint elides overflowing cell text; iced 0.14 has
//!   neither elision nor a wrapping text widget. A long value is clipped by the
//!   cell, so the widths a caller gives are the ones that keep it readable.
//! - **A rounded surface does not clip its children.** iced clips to a
//!   rectangle, so the top and bottom bands carry the inner corner radius
//!   themselves — `outer radius − border` — which is the same concentric shape
//!   the Slint clip produces.
//! - **The row separator is drawn by [`TableRow::index`],** as in the Slint
//!   component: the first row has none and every row after it draws a one-pixel
//!   rule. A caller assembling rows without indexes gets a table without the
//!   rules between them.
//!
//! Everything is drawn from [`Tokens`], read inside a `style` closure while the
//! table renders, so a theme switch repaints it. Nothing here writes a colour of
//! its own, and there is no `Styled` trait to layer one on: a caller that wants
//! another surface wraps the table in a container of its own.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one part gets
// a warning for each one it does not want.
#![allow(dead_code)]

use iced::widget::container::Style as ContainerStyle;
use iced::widget::{column, container, row, text, Space};
use iced::{Alignment, Border, Color, Element, Font, Length, Theme};

use super::scrollable::{Axis, Scrollable};
use super::theme::{self, ControlSize, Tokens};

/// A header band's tint, `Theme.muted.with-alpha(0.5)` in the Slint component.
const BAND_TINT: f32 = 0.5;
/// A separator's tint, `Theme.border.with-alpha(0.7)` in the Slint component.
const RULE_TINT: f32 = 0.7;

/// Where a cell's content sits across its width, the Slint
/// `TextHorizontalAlignment`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

impl Align {
    fn iced(self) -> Alignment {
        match self {
            Self::Start => Alignment::Start,
            Self::Center => Alignment::Center,
            Self::End => Alignment::End,
        }
    }
}

/// The `Align` of a table family member is iced's `Alignment` as well: a cell of
/// [`DataTable`](super::data_table) and a cell of a [`Table`] line up by the same
/// value, so the two components share this enum rather than deriving a second.
impl From<Align> for Alignment {
    fn from(align: Align) -> Self {
        align.iced()
    }
}

/// A cell's horizontal inset at a size: 4 / 6 / 8 / 12, the Slint table's own
/// ladder. It is not the button's or the field's, so it lives here rather than
/// on [`ControlSize`].
fn pad_x(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 4.0,
        ControlSize::Small => 6.0,
        ControlSize::Medium => 8.0,
        ControlSize::Large => 12.0,
    }
}

/// A cell's vertical inset at a size: 2 / 3 / 4 / 8.
fn pad_y(size: ControlSize) -> f32 {
    match size {
        ControlSize::XSmall => 2.0,
        ControlSize::Small => 3.0,
        ControlSize::Medium => 4.0,
        ControlSize::Large => 8.0,
    }
}

/// A header cell. `text` is drawn in the muted header style; a caller with its
/// own content builds a [`TableCell`] instead.
pub struct TableHead {
    text: String,
    align: Align,
    size: ControlSize,
    span: u16,
    width: Option<f32>,
}

impl TableHead {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            align: Align::default(),
            size: ControlSize::default(),
            span: 1,
            width: None,
        }
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// How many columns the head takes. iced expresses a span as a fill share,
    /// so a span of two takes two shares of the row.
    pub fn span(mut self, span: u16) -> Self {
        self.span = span.max(1);
        self
    }

    /// A fixed column width. Without one the column takes its span as a share of
    /// the free width, which is the Slint `horizontal-stretch`.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(0.0));
        self
    }

    fn into_element<'a, Message: 'a>(self) -> Element<'a, Message> {
        let label = text(self.text)
            .size(theme::FONT_SIZE_SM)
            .font(Font {
                weight: iced::font::Weight::Medium,
                ..Font::DEFAULT
            })
            .style(|theme: &Theme| text::Style {
                color: Some(Tokens::of(theme).muted_foreground),
            });

        cell(label.into(), self.align, self.size, self.span, self.width)
    }
}

impl From<&str> for TableHead {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl From<String> for TableHead {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

/// A data cell: `text` in the row foreground, or caller content for a badge, an
/// icon or a button. `text` is ignored when `content` is set.
pub struct TableCell<'a, Message> {
    text: String,
    content: Option<Element<'a, Message>>,
    align: Align,
    size: ControlSize,
    span: u16,
    width: Option<f32>,
    text_color: Option<Color>,
    mono: bool,
}

impl<'a, Message> TableCell<'a, Message> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            content: None,
            align: Align::default(),
            size: ControlSize::default(),
            span: 1,
            width: None,
            text_color: None,
            mono: false,
        }
    }

    /// Custom content in place of the text: the Slint component's `@children`.
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub fn span(mut self, span: u16) -> Self {
        self.span = span.max(1);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width.max(0.0));
        self
    }

    /// The ink the text is drawn in. The default is the row foreground.
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Draw the text in the monospace face at its own size, the Slint `mono`
    /// flag. A caller keeping one token's colour must read it inside a style
    /// closure; this component does for the default foreground.
    pub fn mono(mut self, mono: bool) -> Self {
        self.mono = mono;
        self
    }

    fn into_element(self) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let size = if self.mono {
            theme::FONT_SIZE_MONO
        } else {
            theme::FONT_SIZE_SM
        };
        let mono = self.mono;
        let color = self.text_color;

        let content: Element<'a, Message> = match self.content {
            Some(content) => content,
            None => text(self.text)
                .size(size)
                .font(if mono { theme::MONO } else { Font::DEFAULT })
                .style(move |theme: &Theme| text::Style {
                    color: Some(color.unwrap_or(Tokens::of(theme).foreground)),
                })
                .into(),
        };

        cell(content, self.align, self.size, self.span, self.width)
    }
}

impl<'a, Message> From<&str> for TableCell<'a, Message> {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl<'a, Message> From<String> for TableCell<'a, Message> {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

/// One cell: the content inset and aligned, at its share of the row.
///
/// The outer container is what carries the width, so a fill share and a fixed
/// width behave the same to the row beside them. It keeps its content height
/// rather than filling the row: a row whose every child asks to fill has no
/// intrinsic height of its own and would collapse to nothing.
fn cell<'a, Message: 'a>(
    content: Element<'a, Message>,
    align: Align,
    size: ControlSize,
    span: u16,
    width: Option<f32>,
) -> Element<'a, Message> {
    let body = container(content)
        .width(Length::Fill)
        .align_x(align.iced())
        .align_y(Alignment::Center)
        .padding([pad_y(size), pad_x(size)]);

    let length = match width {
        Some(width) => Length::Fixed(width),
        None => Length::FillPortion(span.max(1)),
    };

    container(body).width(length).into()
}

/// A one-pixel rule across the surface, the Slint separators drawn from the
/// border token at 0.7.
fn rule<'a, Message: 'a>() -> Element<'a, Message> {
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(1.0))
        .style(|theme: &Theme| ContainerStyle {
            background: Some(theme::alpha(Tokens::of(theme).border, RULE_TINT).into()),
            ..ContainerStyle::default()
        })
        .into()
}

/// A row of a table, built with the methods below.
///
/// `index` is what makes the row a body row: the first row draws no separator
/// and every row after it draws one, and `striped` tints the odd rows. Both are
/// the Slint `TableRow`'s own properties.
pub struct TableRow<'a, Message> {
    index: usize,
    striped: bool,
    cells: Vec<Element<'a, Message>>,
}

impl<'a, Message: 'a> TableRow<'a, Message> {
    pub fn new() -> Self {
        Self {
            index: 0,
            striped: false,
            cells: Vec::new(),
        }
    }

    /// The row's position in the body, which selects the separator and the
    /// stripe.
    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    /// Tint the odd rows, `striped` in the Slint component.
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// A header cell. A `&str` is a [`TableHead`] with the defaults.
    pub fn head(mut self, head: impl Into<TableHead>) -> Self {
        self.cells.push(head.into().into_element());
        self
    }

    /// A data cell. A `&str` is a [`TableCell`] with the defaults.
    pub fn cell(mut self, cell: impl Into<TableCell<'a, Message>>) -> Self {
        self.cells.push(cell.into().into_element());
        self
    }

    /// The row's own element. `bottom_radius` is the inner corner the row draws
    /// when it is the last painted band of a bordered table.
    fn into_element(self, bottom_radius: Option<f32>) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let striped = self.striped && self.index % 2 == 1;

        let mut stack = column![].spacing(0.0).width(Length::Fill);
        if self.index > 0 {
            stack = stack.push(rule());
        }
        stack = stack.push(
            row(self.cells)
                .spacing(0.0)
                .width(Length::Fill)
                .align_y(Alignment::Center),
        );

        container(stack)
            .width(Length::Fill)
            .style(move |theme: &Theme| {
                let radius = match bottom_radius {
                    Some(radius) => iced::border::bottom(radius),
                    None => iced::border::Radius::default(),
                };
                ContainerStyle {
                    background: Some(if striped {
                        theme::alpha(Tokens::of(theme).muted, BAND_TINT).into()
                    } else {
                        Color::TRANSPARENT.into()
                    }),
                    border: Border {
                        radius,
                        ..Border::default()
                    },
                    ..ContainerStyle::default()
                }
            })
            .into()
    }
}

impl<'a, Message: 'a> Default for TableRow<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The header band: a muted surface with a bottom rule, wrapping header rows.
pub struct TableHeader<'a, Message> {
    rows: Vec<TableRow<'a, Message>>,
}

impl<'a, Message> TableHeader<'a, Message> {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn row(mut self, row: TableRow<'a, Message>) -> Self {
        self.rows.push(row);
        self
    }

    pub fn rows(mut self, rows: impl IntoIterator<Item = TableRow<'a, Message>>) -> Self {
        self.rows.extend(rows);
        self
    }

    fn into_element(self, radius: f32) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let mut stack = column![].spacing(0.0).width(Length::Fill);
        for row in self.rows {
            stack = stack.push(row.into_element(None));
        }
        // The band's own rule sits under the rows, which is the Slint
        // `padding-bottom: 1px` and the rectangle it leaves room for.
        stack = stack.push(rule());

        container(stack)
            .width(Length::Fill)
            .style(move |theme: &Theme| ContainerStyle {
                background: Some(theme::alpha(Tokens::of(theme).muted, BAND_TINT).into()),
                border: Border {
                    radius: iced::border::top(radius),
                    ..Border::default()
                },
                ..ContainerStyle::default()
            })
            .into()
    }
}

impl<'a, Message> Default for TableHeader<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// The body: the rows between the bands, in a [`Scrollable`] so a bounded height
/// scrolls them while the header and footer stay.
pub struct TableBody<'a, Message> {
    rows: Vec<TableRow<'a, Message>>,
    height: Length,
    axis: Axis,
}

impl<'a, Message> TableBody<'a, Message> {
    pub fn new(rows: impl IntoIterator<Item = TableRow<'a, Message>>) -> Self {
        Self {
            rows: rows.into_iter().collect(),
            height: Length::Shrink,
            axis: Axis::Vertical,
        }
    }

    pub fn row(mut self, row: TableRow<'a, Message>) -> Self {
        self.rows.push(row);
        self
    }

    /// Bound the body's height, so it scrolls its rows and draws a bar. Without
    /// it the body sizes to its rows, which is the Slint component.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Which way the body may overflow. Vertical by default; a body wider than
    /// its frame wants [`Axis::Both`] so the columns scroll too.
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    fn into_element(self, round_bottom: bool, radius: f32) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let last = self.rows.len().saturating_sub(1);
        let mut stack = column![].spacing(0.0).width(Length::Fill);
        for (index, row) in self.rows.into_iter().enumerate() {
            let bottom = if round_bottom && index == last {
                Some(radius)
            } else {
                None
            };
            stack = stack.push(row.into_element(bottom));
        }

        Scrollable::new(stack)
            .axis(self.axis)
            .height(self.height)
            .into()
    }
}

/// The footer band: a muted surface with a top rule, wrapping footer rows.
pub struct TableFooter<'a, Message> {
    rows: Vec<TableRow<'a, Message>>,
}

impl<'a, Message> TableFooter<'a, Message> {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn row(mut self, row: TableRow<'a, Message>) -> Self {
        self.rows.push(row);
        self
    }

    pub fn rows(mut self, rows: impl IntoIterator<Item = TableRow<'a, Message>>) -> Self {
        self.rows.extend(rows);
        self
    }

    fn into_element(self, radius: f32) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let mut stack = column![].spacing(0.0).width(Length::Fill);
        stack = stack.push(rule());
        for row in self.rows {
            stack = stack.push(row.into_element(None));
        }

        container(stack)
            .width(Length::Fill)
            .style(move |theme: &Theme| ContainerStyle {
                background: Some(theme::alpha(Tokens::of(theme).muted, BAND_TINT).into()),
                border: Border {
                    radius: iced::border::bottom(radius),
                    ..Border::default()
                },
                ..ContainerStyle::default()
            })
            .into()
    }
}

impl<'a, Message> Default for TableFooter<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// A muted caption centered under the footer, inside the surface.
pub struct TableCaption {
    text: String,
    size: ControlSize,
}

impl TableCaption {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: ControlSize::default(),
        }
    }

    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    fn into_element<'a, Message: 'a>(self) -> Element<'a, Message> {
        container(
            text(self.text)
                .size(theme::FONT_SIZE_SM)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .style(|theme: &Theme| text::Style {
                    color: Some(Tokens::of(theme).muted_foreground),
                }),
        )
        .width(Length::Fill)
        .padding([pad_y(self.size), pad_x(self.size)])
        .into()
    }
}

impl From<&str> for TableCaption {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

/// A table, built with the methods below.
///
/// The sections are optional, and they stack in the order they are named here:
/// a header, a body, a footer, and a caption. A table with no body draws only
/// what it was given, so an empty table is an empty surface rather than nothing.
pub struct Table<'a, Message> {
    header: Option<TableHeader<'a, Message>>,
    body: Option<TableBody<'a, Message>>,
    footer: Option<TableFooter<'a, Message>>,
    caption: Option<TableCaption>,
    bordered: bool,
    width: Length,
}

impl<'a, Message> Table<'a, Message> {
    pub fn new() -> Self {
        Self {
            header: None,
            body: None,
            footer: None,
            caption: None,
            bordered: true,
            width: Length::Fill,
        }
    }

    /// Draws the outer border and the rounded corners. `false` is the Slint
    /// `bordered: false` and GPUI's `.border_0().rounded_none()`: the surface
    /// keeps its fill and loses its edge.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn header(mut self, header: TableHeader<'a, Message>) -> Self {
        self.header = Some(header);
        self
    }

    pub fn body(mut self, body: TableBody<'a, Message>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn footer(mut self, footer: TableFooter<'a, Message>) -> Self {
        self.footer = Some(footer);
        self
    }

    pub fn caption(mut self, caption: impl Into<TableCaption>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message> Default for Table<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Table<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(control: Table<'a, Message>) -> Self {
        let bordered = control.bordered;
        // The border and its radius are the one switch: without the edge the
        // surface is square and the inner corner has no band to round.
        let border_width = if bordered { 1.0 } else { 0.0 };
        let radius = if bordered { theme::RADIUS_MD } else { 0.0 };
        // The inner corner the header and footer bands draw: the outer radius
        // less the border, so the two curves share a centre.
        let inner = (radius - border_width).max(0.0);

        let has_footer = control.footer.is_some();
        let has_caption = control.caption.is_some();

        let mut sections = column![].spacing(0.0).width(Length::Fill);

        if let Some(header) = control.header {
            sections = sections.push(header.into_element(inner));
        }

        if let Some(body) = control.body {
            // Only a body that is the last painted surface rounds its bottom
            // row; under a footer or a caption the corner is interior.
            let round_bottom = !has_footer && !has_caption;
            sections = sections.push(body.into_element(round_bottom, inner));
        }

        if let Some(footer) = control.footer {
            sections = sections.push(footer.into_element(inner));
        }

        if let Some(caption) = control.caption {
            sections = sections.push(caption.into_element());
        }

        container(sections)
            .width(control.width)
            .padding(border_width)
            .clip(true)
            .style(move |theme: &Theme| {
                // Resolved here, inside the closure, and never at build time: a
                // surface read from the light tokens before the table is drawn
                // would paint a white table under the dark theme.
                let tokens = Tokens::of(theme);
                ContainerStyle {
                    background: Some(tokens.background.into()),
                    border: Border {
                        color: if bordered {
                            tokens.border
                        } else {
                            Color::TRANSPARENT
                        },
                        width: border_width,
                        radius: radius.into(),
                    },
                    ..ContainerStyle::default()
                }
            })
            .into()
    }
}
