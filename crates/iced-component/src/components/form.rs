//! A form: composition and validation over the controls beside it. Mirrors GPUI
//! Component's `Form` and `Field` and the Slint `Form`, `FormGrid`, `FormField`,
//! `FormRow`, `FormSection` and `FormSeparator` beside them.
//!
//! This file draws no control of its own. What it owns is the layout, the
//! labels, the error placement, and the state that ties a control to its
//! message — the same division GPUI's `Form` and Slint's `FormField` make. A
//! caller puts any [`Element`] in a [`Field`], so a form composes the input, the
//! textarea, the select, the switch and the rest without naming one of them.
//!
//! [`Form`] is a column of [`Field`]s by default. [`Form::columns`] turns it into
//! a grid, and a field's [`Field::col_span`] and [`Field::col_start`] place it in
//! that grid; `Form::columns(1)` is the single-column stack. [`LabelLayout`]
//! chooses where a label sits: [`LabelLayout::Above`] is Slint's `FormField` and
//! [`LabelLayout::Beside`] is Slint's `FormRow`. [`Field::section`] and
//! [`Field::separator`] are the two block rows, Slint's `FormSection` and
//! `FormSeparator`.
//!
//! **The grid is exact.** GPUI's `Form` is a CSS grid — `grid_cols(columns)`
//! with equal `1fr` columns and a gap — and Slint's `FormGrid` is a `GridLayout`
//! with the same equal cells and one `spacing` between them. A plain `row` of
//! `Length::FillPortion` cells cannot hold that once a row mixes spans: a field
//! spanning `n` columns must be `n` column widths plus the `n - 1` gaps *inside*
//! it, and a fill share has no way to add that fixed width. So each row is a
//! [`GridRow`], which reads the width its parent offers and lays every cell out
//! at the grid's own geometry — the measurement iced's own `Grid` makes for
//! uniform cells, with a per-cell span on top. Column edges therefore agree from
//! row to row, and the form's `gap` separates columns as well as rows, which is
//! Slint's single `FormGrid` `spacing` (GPUI keeps a wider `gap_x`).
//!
//! **Where a label sits is the shared choice [`LabelLayout`] makes.** `Above` is
//! Slint's `FormField` and GPUI's `Axis::Vertical`, so the label leads its own
//! cell; `Beside` is Slint's `FormRow` and GPUI's `Axis::Horizontal` with a fixed
//! `label_width`, so a single-column form's labels share one edge. A grid cell
//! carries its own field, label included — GPUI's grid does the same — so in a
//! multi-column form a label follows its column rather than a spine across the
//! whole form.
//!
//! **The column count is the caller's.** Neither reference derives it from the
//! viewport: GPUI's `Form::columns` and the page's Responsive Layout sample
//! choose it in the application, and Slint's `FormGrid` takes rows and columns
//! from its children. iced offers no viewport token, so this form follows the
//! caller's `columns` alone — the same limitation the description-list port
//! records.
//!
//! [`Field::error`] is what the reference keeps apart from a description: while
//! an error is set it takes the message line in the danger colour and the
//! description steps aside, exactly as Slint's `FormField` does with
//! `error` and `description`. The field does not reach into its child to paint
//! the control's edge — the child is an opaque [`Element`] — so a caller that
//! wants the rejected edge too passes `invalid(true)` to the control itself; the
//! gallery shows both together. The error is the application's state: the field
//! only places it.
//!
//! **Two reference behaviours have no iced 0.14 counterpart, and neither is
//! faked:**
//!
//! - **`label_fn` and `description_fn` are the caller's.** A GPUI field can hold
//!   a closure that builds its label or description per frame; an iced element is
//!   a value built from the caller's current state, so a dynamic description is
//!   the sentence the caller passes this frame, not a closure the field calls.
//! - **A hidden field leaves the grid.** GPUI's `visible(false)` keeps the cell
//!   and hides its contents; this port drops the field from the rows it packs, so
//!   the fields after it move up. Slint's `if` does the same as this port, which
//!   is why the choice is named here rather than left to look like GPUI's.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one variant
// gets a warning for each one it does not want.
#![allow(dead_code)]

use iced::advanced::layout::{self, Limits};
use iced::advanced::widget::Tree;
use iced::advanced::{Clipboard, Layout, Shell, Widget, mouse, overlay, renderer};
use iced::widget::rule as iced_rule;
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Element, Event, Length, Rectangle, Size, Theme, Vector};

use super::label::Label;
use super::theme::{self, ControlSize, Tokens};

/// Where a field's label sits relative to its control.
///
/// [`Above`](Self::Above) is Slint's `FormField` and GPUI's
/// `Axis::Vertical`; [`Beside`](Self::Beside) is Slint's `FormRow` and GPUI's
/// `Axis::Horizontal`. iced owns no `Axis` type, so the choice is named for what
/// it does rather than for the axis it moves along.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelLayout {
    /// The label on its own line over the control.
    #[default]
    Above,
    /// The label in a fixed-width column before the control.
    Beside,
}

/// A form: a grid of [`Field`]s and an optional footer, built with the methods
/// below.
///
/// The form owns the shared geometry — the column count, the label layout and
/// width a field inherits when it does not set its own, the gap between rows and
/// the footer. It owns no value and no message: every field's control reports to
/// the application, which is the same ownership the rest of this crate uses.
pub struct Form<'a, Message> {
    fields: Vec<Field<'a, Message>>,
    footer: Option<Element<'a, Message>>,
    columns: usize,
    label_layout: LabelLayout,
    label_width: f32,
    size: ControlSize,
    gap: f32,
}

impl<'a, Message> Form<'a, Message> {
    /// A single-column form with labels above their controls, 16px between
    /// fields — Slint's `Form`.
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            footer: None,
            columns: 1,
            label_layout: LabelLayout::Above,
            label_width: 140.0,
            size: ControlSize::default(),
            gap: theme::SPACING_LG,
        }
    }

    /// A form whose labels sit beside their controls, Slint's `FormRow` for every
    /// field.
    pub fn horizontal() -> Self {
        Self::new().label_layout(LabelLayout::Beside)
    }

    /// A form whose labels sit above their controls, the default.
    pub fn vertical() -> Self {
        Self::new()
    }

    /// The number of columns the field grid has, 1 by default. The label layout
    /// is independent of it: a two-column form can still put each label above
    /// its control.
    pub fn columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(1);
        self
    }

    /// Where a field's label sits when the field does not choose for itself.
    pub fn label_layout(mut self, label_layout: LabelLayout) -> Self {
        self.label_layout = label_layout;
        self
    }

    /// The label column's width in [`LabelLayout::Beside`], 140px by default.
    pub fn label_width(mut self, label_width: f32) -> Self {
        self.label_width = label_width;
        self
    }

    /// The size step a field and its label inherit when the field does not choose
    /// one.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    /// The gap between rows, 16px by default.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Add a field.
    pub fn child(mut self, field: impl Into<Field<'a, Message>>) -> Self {
        self.fields.push(field.into());
        self
    }

    /// Add fields in order.
    pub fn children(mut self, fields: impl IntoIterator<Item = Field<'a, Message>>) -> Self {
        self.fields.extend(fields);
        self
    }

    /// Content under the field grid, spanning its width and aligned to the
    /// trailing edge — the commit row of the website examples. The form does not
    /// submit: the caller builds the button and owns what pressing it does.
    /// Calling this again replaces the footer.
    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Pack the fields into rows, flush a row when it fills or when a block row
    /// arrives, and stack the rows with the form's gap.
    fn into_element(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let Form {
            fields,
            footer,
            columns,
            label_layout,
            label_width,
            size,
            gap,
        } = self;

        let visible: Vec<Field<'a, Message>> =
            fields.into_iter().filter(|field| field.visible).collect();

        let mut rows: Vec<Element<'a, Message>> = Vec::new();
        let mut cells: Vec<(Element<'a, Message>, u16)> = Vec::new();
        let mut col = 0usize;

        for field in visible {
            let span = field.col_span.clamp(1, columns as u16);

            // A section or a separator owns its own row and spans the grid, which
            // is the row a `FormSection` or `FormSeparator` always takes in Slint.
            if field.full_width {
                close_row(&mut rows, &mut cells, &mut col, columns, gap);
                rows.push(field.render_element(label_layout, label_width, size));
                continue;
            }

            if let Some(start) = field.col_start {
                if start < col {
                    close_row(&mut rows, &mut cells, &mut col, columns, gap);
                }
                if start > col {
                    cells.push((filler(), (start - col) as u16));
                    col = start;
                }
            }

            if col + span as usize > columns {
                close_row(&mut rows, &mut cells, &mut col, columns, gap);
            }

            cells.push((field.render_element(label_layout, label_width, size), span));
            col += span as usize;
            if col >= columns {
                close_row(&mut rows, &mut cells, &mut col, columns, gap);
            }
        }

        close_row(&mut rows, &mut cells, &mut col, columns, gap);

        let mut grid = column(rows).spacing(gap).width(Length::Fill);

        if let Some(footer) = footer {
            grid = grid.push(
                container(footer)
                    .width(Length::Fill)
                    .align_x(Alignment::End),
            );
        }

        grid.into()
    }
}

/// Close the open row into `rows`, padding it to the column count so the cells
/// of one row line up with the cells of the next.
fn close_row<'a, Message: Clone + 'a>(
    rows: &mut Vec<Element<'a, Message>>,
    cells: &mut Vec<(Element<'a, Message>, u16)>,
    col: &mut usize,
    columns: usize,
    gap: f32,
) {
    if cells.is_empty() {
        return;
    }
    if *col < columns {
        cells.push((filler(), (columns - *col) as u16));
    }
    rows.push(row_of(std::mem::take(cells), columns, gap));
    *col = 0;
}

/// An empty cell that takes the share the row still owes the grid.
fn filler<'a, Message: 'a>() -> Element<'a, Message> {
    Space::new().width(Length::Fill).into()
}

/// One row of the grid: cells with a column span, laid out at the grid's own
/// geometry. See [`GridRow`].
fn row_of<'a, Message: Clone + 'a>(
    cells: Vec<(Element<'a, Message>, u16)>,
    columns: usize,
    gap: f32,
) -> Element<'a, Message> {
    GridRow::new(cells, columns, gap).into()
}

/// A row of the form's grid, with a span on every cell.
///
/// The form's rows have to place their cells at the same x positions in every
/// row, and a `row` of `Length::FillPortion` cells cannot: iced gives a fill cell
/// a share of the width left *after* the row's own `spacing`, so a row mixing a
/// one-column and a two-column cell gets widths that do not add the fixed gap
/// inside the span. This row instead reads the width its parent offers and lays
/// each cell out at `span` column widths plus the `span - 1` gaps inside it —
/// the same arithmetic iced's own `Grid` does for uniform cells, with a span on
/// top. That is what makes a spanning field exactly as wide as the columns it
/// covers, so the edges line up across rows.
struct GridRow<'a, Message> {
    cells: Vec<Element<'a, Message>>,
    /// The column span of each cell, parallel to `cells`.
    spans: Vec<u16>,
    columns: u16,
    gap: f32,
}

impl<'a, Message> GridRow<'a, Message> {
    fn new(cells: Vec<(Element<'a, Message>, u16)>, columns: usize, gap: f32) -> Self {
        let (cells, spans) = cells.into_iter().unzip();
        Self {
            cells,
            spans,
            columns: columns.max(1) as u16,
            gap,
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for GridRow<'_, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        self.cells.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.cells);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &Limits,
    ) -> layout::Node {
        let size = self.size();
        let limits = limits.width(size.width).height(size.height);
        let available = limits.max();
        let columns = self.columns.max(1) as f32;
        let gap = self.gap.max(0.0);

        // A grid needs the width its parent offers, which a `Fill` child inside a
        // growing row has. An unbounded parent (a horizontal scrollable) is the
        // one case that cannot answer, so there the cells fall back to their own
        // intrinsic widths rather than every column collapsing to zero.
        if !available.width.is_finite() {
            let mut nodes = Vec::with_capacity(self.cells.len());
            let mut x: f32 = 0.0;
            let mut height: f32 = 0.0;
            for (cell, state) in self.cells.iter_mut().zip(&mut tree.children) {
                let node = cell
                    .as_widget_mut()
                    .layout(state, renderer, &limits)
                    .move_to((x, 0.0));
                x += node.size().width + gap;
                height = height.max(node.size().height);
                nodes.push(node);
            }
            let width = (x - gap).max(0.0);
            return layout::Node::with_children(Size::new(width, height), nodes);
        }

        // Equal columns: the whole width minus the gaps the columns leave, split
        // evenly. A cell then takes its span's worth of columns and the gaps
        // between them, which is the only way a spanning cell can be exactly as
        // wide as the columns it covers.
        let column = ((available.width - gap * (columns - 1.0)) / columns).max(0.0);

        let mut nodes = Vec::with_capacity(self.cells.len());
        let mut x: f32 = 0.0;
        let mut height: f32 = 0.0;
        for ((cell, span), state) in self
            .cells
            .iter_mut()
            .zip(&self.spans)
            .zip(&mut tree.children)
        {
            let span = (*span).max(1) as f32;
            let cell_width = (span * column + (span - 1.0) * gap).max(0.0);
            let cell_limits = Limits::new(
                Size::new(cell_width, 0.0),
                Size::new(cell_width, available.height),
            );
            let node = cell
                .as_widget_mut()
                .layout(state, renderer, &cell_limits)
                .move_to((x, 0.0));
            height = height.max(node.size().height);
            x += cell_width + gap;
            nodes.push(node);
        }

        layout::Node::with_children(Size::new(available.width, height), nodes)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        for ((cell, state), child_layout) in self
            .cells
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            cell.as_widget_mut().update(
                state,
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.cells
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((cell, state), child_layout)| {
                cell.as_widget()
                    .mouse_interaction(state, child_layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if let Some(viewport) = layout.bounds().intersection(viewport) {
            for ((cell, state), child_layout) in self
                .cells
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
                .filter(|(_, child_layout)| child_layout.bounds().intersects(&viewport))
            {
                cell.as_widget().draw(
                    state,
                    renderer,
                    theme,
                    style,
                    child_layout,
                    cursor,
                    &viewport,
                );
            }
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            for ((cell, state), child_layout) in self
                .cells
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                cell.as_widget_mut()
                    .operate(state, child_layout, renderer, operation);
            }
        });
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        overlay::from_children(
            &mut self.cells,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: Clone + 'a> From<GridRow<'a, Message>> for Element<'a, Message> {
    fn from(grid_row: GridRow<'a, Message>) -> Self {
        Element::new(grid_row)
    }
}

impl<'a, Message> Default for Form<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Form<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(form: Form<'a, Message>) -> Self {
        form.into_element()
    }
}

/// One labelled field of a [`Form`], built with the methods below.
///
/// It places a label, the caller's control, and one message line; sets how much
/// of the grid it takes; and can be hidden. It paints no control and owns no
/// value.
pub struct Field<'a, Message> {
    label: String,
    description: String,
    error: String,
    required: bool,
    visible: bool,
    full_width: bool,
    content: Vec<Element<'a, Message>>,
    size: Option<ControlSize>,
    label_layout: Option<LabelLayout>,
    label_width: Option<f32>,
    label_indent: bool,
    col_span: u16,
    col_start: Option<usize>,
}

impl<'a, Message: 'a> Field<'a, Message> {
    /// An empty field.
    pub fn new() -> Self {
        Self {
            label: String::new(),
            description: String::new(),
            error: String::new(),
            required: false,
            visible: true,
            full_width: false,
            content: Vec::new(),
            size: None,
            label_layout: None,
            label_width: None,
            label_indent: true,
            col_span: 1,
            col_start: None,
        }
    }

    /// A block heading inside a form, Slint's `FormSection`: the title over a
    /// muted description, spanning the grid. Its title is the field's
    /// [`label`](Self::label) and its description is the field's
    /// [`description`](Self::description).
    pub fn section(title: impl Into<String>) -> Self {
        Self {
            full_width: true,
            ..Self::new().label(title).label_indent(false)
        }
    }

    /// A one-pixel rule between groups of fields, Slint's `FormSeparator`,
    /// spanning the grid.
    pub fn separator() -> Self {
        Self {
            full_width: true,
            ..Self::new().child(separator_rule())
        }
    }

    /// The text over or beside the control.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// The supporting line under the control, in the muted colour. An
    /// [`error`](Self::error) takes its place while one is set.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// The rejected value's message, in the danger colour under the control.
    /// While it is set the description steps aside, the way Slint's `FormField`
    /// hides its `description` behind its `error`. The control's own rejected
    /// edge is the caller's: pass `invalid(true)` to the child as well.
    pub fn error(mut self, error: impl Into<String>) -> Self {
        self.error = error.into();
        self
    }

    /// Draws the danger asterisk after the label, `required: true` in Slint.
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Whether the field is laid out at all, `true` by default. A hidden field
    /// leaves the grid rather than keeping an empty cell; the module comment
    /// names that against GPUI's `visible(false)`.
    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    /// The size step this field's label uses, when it does not want the form's.
    pub fn size(mut self, size: ControlSize) -> Self {
        self.size = Some(size);
        self
    }

    /// Where this field's label sits, when it does not want the form's layout.
    pub fn label_layout(mut self, label_layout: LabelLayout) -> Self {
        self.label_layout = Some(label_layout);
        self
    }

    /// The label column's width, when this field does not want the form's.
    pub fn label_width(mut self, label_width: f32) -> Self {
        self.label_width = Some(label_width);
        self
    }

    /// Reserves the label column in [`LabelLayout::Beside`] even when there is no
    /// label, so a label-less row's control still lines up with its neighbours.
    /// `true` by default; GPUI's `label_indent`.
    pub fn label_indent(mut self, label_indent: bool) -> Self {
        self.label_indent = label_indent;
        self
    }

    /// How many of the form's columns the field takes, 1 by default. See the
    /// module comment for why the width is proportional rather than an exact
    /// number of cells.
    pub fn col_span(mut self, col_span: u16) -> Self {
        self.col_span = col_span.max(1);
        self
    }

    /// The zero-based column the field starts at, when it should not simply
    /// follow the field before it. Columns before it on the row are left empty.
    pub fn col_start(mut self, col_start: usize) -> Self {
        self.col_start = Some(col_start);
        self
    }

    /// The control, or a stack of them — a field may hold a row of two inputs, a
    /// textarea, or a switch beside a note.
    pub fn child(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.content.push(child.into());
        self
    }

    /// Several children, stacked with the field's own gap.
    pub fn children(mut self, children: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        self.content.extend(children);
        self
    }

    /// The size the field resolves to: its own, or the form's.
    fn resolve_size(&self, form_size: ControlSize) -> ControlSize {
        self.size.unwrap_or(form_size)
    }

    /// Draw the label, the children and the message line, in the resolved layout.
    fn render_element(
        self,
        form_layout: LabelLayout,
        form_label_width: f32,
        form_size: ControlSize,
    ) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let layout = self.label_layout.unwrap_or(form_layout);
        let label_width = self.label_width.unwrap_or(form_label_width);
        let size = self.resolve_size(form_size);

        let label: Option<Element<'a, Message>> = if self.label.is_empty() {
            None
        } else {
            Some(
                Label::new(self.label)
                    .required(self.required)
                    .size(size)
                    .into(),
            )
        };

        // The error takes the message line from the description, which is what
        // Slint's `description != "" && error == ""` decides.
        let message: Option<Element<'a, Message>> = if !self.error.is_empty() {
            Some(
                text(self.error)
                    .size(theme::FONT_SIZE_XS)
                    .width(Length::Fill)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).danger),
                    })
                    .into(),
            )
        } else if !self.description.is_empty() {
            Some(
                text(self.description)
                    .size(theme::FONT_SIZE_XS)
                    .width(Length::Fill)
                    .style(|theme: &Theme| text::Style {
                        color: Some(Tokens::of(theme).muted_foreground),
                    })
                    .into(),
            )
        } else {
            None
        };

        let has_control = !self.content.is_empty();
        let mut control = column![].spacing(theme::SPACING_XS).width(Length::Fill);
        for child in self.content {
            control = control.push(child);
        }

        let message_gap = theme::SPACING_XS;

        match layout {
            LabelLayout::Above => {
                let mut stack = column![].spacing(message_gap).width(Length::Fill);
                if let Some(label) = label {
                    stack = stack.push(label);
                }
                // A section heading is a label and a description with no control;
                // an empty control column would only add its two gaps.
                if has_control {
                    stack = stack.push(control);
                }
                if let Some(message) = message {
                    stack = stack.push(message);
                }
                stack.into()
            }
            LabelLayout::Beside => {
                // The label sits at the top of the row with Slint's `FormRow`
                // `padding-top: 8px`, so it lines up with a single-line control's
                // text instead of drifting when the control is a textarea.
                let label_slot: Element<'a, Message> = match label {
                    Some(label) => container(label)
                        .width(Length::Fixed(label_width))
                        .padding([theme::SPACING_SM, 0.0])
                        .into(),
                    None if self.label_indent => {
                        Space::new().width(Length::Fixed(label_width)).into()
                    }
                    None => Space::new().width(Length::Fixed(0.0)).into(),
                };

                let mut right = column![].spacing(message_gap).width(Length::Fill);
                if has_control {
                    right = right.push(control);
                }
                if let Some(message) = message {
                    right = right.push(message);
                }

                row![label_slot, right]
                    .spacing(theme::SPACING_LG)
                    .width(Length::Fill)
                    .align_y(Alignment::Start)
                    .into()
            }
        }
    }
}

impl<'a, Message: 'a> Default for Field<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<Field<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(field: Field<'a, Message>) -> Self {
        field.render_element(LabelLayout::default(), 140.0, ControlSize::default())
    }
}

/// A one-pixel rule in the border colour, the body of [`Field::separator`].
fn separator_rule<'a, Message: 'a>() -> Element<'a, Message> {
    iced_rule::horizontal(1)
        .style(|theme: &Theme| iced_rule::Style {
            color: Tokens::of(theme).border,
            radius: 0.0.into(),
            fill_mode: iced_rule::FillMode::Full,
            snap: true,
        })
        .into()
}

/// A single-column form with labels above their controls — [`Form::vertical`].
pub fn v_form<'a, Message>() -> Form<'a, Message> {
    Form::vertical()
}

/// A form with labels beside their controls — [`Form::horizontal`].
pub fn h_form<'a, Message>() -> Form<'a, Message> {
    Form::horizontal()
}

/// A [`Field`].
pub fn field<'a, Message: 'a>() -> Field<'a, Message> {
    Field::new()
}
