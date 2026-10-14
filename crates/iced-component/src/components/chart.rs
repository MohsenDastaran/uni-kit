//! `chart` — the plot surface: a plot area, its axes and grid, two series
//! kinds, a legend and a hover tooltip.
//!
//! The Slint authority is `crates/slint-component/ui/chart.slint`, with the
//! example beside it in `ui/examples/chart.slint`; the GPUI original is
//! `crates/component/src/chart/`. This port follows the Slint rendering, because
//! that is the design the two galleries share. What it builds is the part a
//! chart *is* rather than the breadth of the page:
//!
//! - **[`Point`] and [`LineChart`]** — a labelled datum, and the line the
//!   Slint `LineChart` draws: one series, `linear` or `step` joins, optional
//!   dots, a pinned `max_value`, a tone from the theme's chart palette.
//! - **[`BarChart`]** — the Slint `BarChart` in its default orientation: bars
//!   grow out of the zero line, negative values extend the other way, and a
//!   `mixed` chart takes a palette tone per bar.
//! - **[`ChartLegend`]** — the swatch-and-name row, the Slint `ChartLegend`.
//! - **Axes and guides** — `y_ticks` labelled grid lines in a gutter that widens
//!   to fit them, and x labels under the plot with the Slint `tick_margin`
//!   stride.
//! - **Hover** — the crosshair and the dot follow the pointer, the bar under it
//!   is marked, and a popover-surface tooltip names the datum and its value.
//!   The state lives in the widget tree, the route `resizable` takes.
//!
//! **Named, not faked.** Slint's chart is seven kinds and forty cards; this is
//! the honest minimum of the two the page documents first. Not ported:
//!
//! - `AreaChart`, `PieChart`, `RadarChart`, `CandlestickChart` and
//!   `SankeyChart`. Only the line and the bar exist here, and a line is a
//!   single series — no second line or stack shares the plot, which is why the
//!   tooltip never has more than one row.
//! - The **`curve`** interpolation. Slint's default is a natural cubic spline;
//!   iced exposes no path, so [`Interpolation`] offers `Linear` and `Step`, and
//!   the default is `Linear` where Slint's is `curve`.
//! - The **left/right/top bar alignments**, `gradient` fills, `stacked` series,
//!   `show-labels` colouring, and `minimum` stubs. Bars are vertical and grow
//!   from zero.
//! - The **appear animation**. Slint draws a chart in over one second; iced
//!   0.14 has no animation of the kind the styled layer uses, and this
//!   component does not fake one.
//! - **Elision**, as everywhere else in this catalog: a label wider than its
//!   column is clipped, not shortened with an ellipsis.
//!
//! **Drawn as quads and text, not through `canvas`.** A line is a run of
//! one-pixel-wide quads sampled along the segment, because iced's
//! `Transformation` has no rotation and a sloped stroke has no other quad
//! primitive. This is also the path the brief asks for: the `Canvas` that once
//! silently drew nothing in a field's suffix (`speech`) is a trap this component
//! does not need to enter. Text is drawn with `renderer.fill_text`.
//!
//! **The tooltip box is measured by estimate.** iced 0.14 exposes no text
//! measurement to a custom widget — the shaper that owns it is private to
//! `iced_graphics::text` — so the box is sized from the character count at the
//! font size. The estimate is a few pixels either way on a proportional face,
//! which is why the box carries the same 8px of padding on every side.

// An application installs the components it uses, so the parts of this file it
// does not call are not dead code in the way the compiler means — the file is a
// library that arrived by copy. Without this, a project that wants one series
// kind gets a warning for the other.
#![allow(dead_code)]

use iced::advanced::Renderer as _;
use iced::advanced::layout::{self, Limits};
use iced::advanced::renderer::Quad;
use iced::advanced::text::{
    Alignment as TextAlignment, LineHeight, Renderer as _, Shaping, Text as CoreText, Wrapping,
};
use iced::advanced::widget::Tree;
use iced::advanced::widget::tree::{self, Tag};
use iced::advanced::{Clipboard, Layout, Shell, Widget, mouse};
use iced::widget::{Space, container, row, text};
use iced::{Color, Element, Event, Length, Pixels, Rectangle, Size, Theme};

use super::theme::{self, Tokens};

/// The strip under the plot the x labels sit in.
const X_LABEL_HEIGHT: f32 = 18.0;
/// The gap kept above the plot. It leaves a full label line plus slack, because
/// the top tick's label is centred on the top grid line and the first few pixels
/// of the widget are clipped by its container.
const TOP_PAD: f32 = LABEL_SIZE * 1.3 / 2.0 + 10.0;
/// The gap kept to the right of the last band.
const RIGHT_PAD: f32 = 4.0;
/// The label size, Slint's `Theme.font-size-xs`.
const LABEL_SIZE: f32 = theme::FONT_SIZE_XS;
/// The line height of a label, so a label can be centred on a grid line.
const LABEL_LINE: f32 = LABEL_SIZE * 1.3;
/// The height of the plot Slint draws when the caller does not say, its
/// `plot-height: 140px`.
const DEFAULT_PLOT_HEIGHT: f32 = 140.0;
/// The guide line's alpha, Slint's `Theme.foreground.with-alpha(0.28)`.
const GUIDE_ALPHA: f32 = 0.28;
/// The grid line's alpha, Slint's `Theme.border.with-alpha(0.6)`.
const GRID_ALPHA: f32 = 0.6;
/// The line's thickness, Slint's `stroke-width: 2px`.
const LINE_WIDTH: f32 = 2.0;
/// How far the pointer may sit from the plot and still be asked about.
const HOVER_INSET: f32 = 6.0;

// ---------------------------------------------------------------- data

/// One datum: the label its axis shows and the value the plot draws. The Slint
/// `ChartPoint`.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    label: String,
    value: f32,
}

impl Point {
    /// A labelled value.
    pub fn new(label: impl Into<String>, value: f32) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }

    /// The text the x axis draws for this datum.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The number the plot places on the value axis.
    pub fn value(&self) -> f32 {
        self.value
    }
}

impl From<(&str, f32)> for Point {
    fn from((label, value): (&str, f32)) -> Self {
        Self::new(label, value)
    }
}

impl From<(String, f32)> for Point {
    fn from((label, value): (String, f32)) -> Self {
        Self::new(label, value)
    }
}

/// How a line joins its points. Slint's `LineInterpolation` without `curve`: a
/// natural cubic spline needs a path iced 0.14 does not expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Interpolation {
    /// A straight segment between neighbouring points.
    #[default]
    Linear,
    /// Hold the value until the next point, then step — Slint's `step after`.
    Step,
}

/// A palette tone, wrapping after the five the theme carries. `chart.1`
/// through `chart.5` in the theme file, `Theme.chart-tone` in Slint.
fn tone_color(tokens: &Tokens, tone: usize) -> Color {
    match tone % 5 {
        0 => tokens.chart_1,
        1 => tokens.chart_2,
        2 => tokens.chart_3,
        3 => tokens.chart_4,
        _ => tokens.chart_5,
    }
}

// ---------------------------------------------------------------- line

/// A line chart: the points joined in order, on the shared plot surface.
pub struct LineChart {
    points: Vec<Point>,
    interpolation: Interpolation,
    dots: bool,
    tone: usize,
    stroke: Option<Color>,
    tick_margin: usize,
    max_value: Option<f32>,
    plot_height: f32,
    y_ticks: usize,
    y_axis: bool,
    grid: bool,
}

impl LineChart {
    /// A line through `points`, in the order given.
    pub fn new(points: impl IntoIterator<Item = Point>) -> Self {
        Self {
            points: points.into_iter().collect(),
            interpolation: Interpolation::default(),
            dots: false,
            tone: 1,
            stroke: None,
            tick_margin: 1,
            max_value: None,
            plot_height: DEFAULT_PLOT_HEIGHT,
            y_ticks: 5,
            y_axis: true,
            grid: true,
        }
    }

    /// How the points join. Slint's `interpolation`.
    pub fn interpolation(mut self, interpolation: Interpolation) -> Self {
        self.interpolation = interpolation;
        self
    }

    /// Draw a dot on every point. Slint's `dots`.
    pub fn dots(mut self, dots: bool) -> Self {
        self.dots = dots;
        self
    }

    /// The series tone, wrapped to the five chart colours. Slint's `stroke`,
    /// which takes a colour because Slint resolves the theme itself.
    pub fn tone(mut self, tone: usize) -> Self {
        self.tone = tone;
        self.stroke = None;
        self
    }

    /// An explicit stroke. Setting it clears the tone.
    pub fn stroke(mut self, stroke: Color) -> Self {
        self.stroke = Some(stroke);
        self
    }

    /// Label every `tick_margin`-th point. Slint's `tick-margin`.
    pub fn tick_margin(mut self, tick_margin: usize) -> Self {
        self.tick_margin = tick_margin.max(1);
        self
    }

    /// Pin the top of the value axis, Slint's `max-value`, so a series that
    /// never nears zero is not pressed flat against the top.
    pub fn max_value(mut self, max_value: f32) -> Self {
        self.max_value = Some(max_value.max(1.0));
        self
    }

    /// The plot's height in pixels, Slint's `plot-height`.
    pub fn plot_height(mut self, plot_height: f32) -> Self {
        self.plot_height = plot_height.max(1.0);
        self
    }

    /// How many labelled grid lines the value axis carries, both ends
    /// included. Slint's `y-tick-count`.
    pub fn y_ticks(mut self, y_ticks: usize) -> Self {
        self.y_ticks = y_ticks.max(2);
        self
    }

    /// Draw the value axis gutter. Slint's `y-axis`.
    pub fn y_axis(mut self, y_axis: bool) -> Self {
        self.y_axis = y_axis;
        self
    }

    /// Draw the horizontal grid lines. Slint's `grid`.
    pub fn grid(mut self, grid: bool) -> Self {
        self.grid = grid;
        self
    }

    fn into_plot(self) -> Plot {
        Plot {
            series: Series::Line {
                interpolation: self.interpolation,
                dots: self.dots,
                tone: self.tone,
                stroke: self.stroke,
            },
            points: self.points,
            tick_margin: self.tick_margin,
            max_value: self.max_value,
            plot_height: self.plot_height,
            y_ticks: self.y_ticks,
            y_axis: self.y_axis,
            grid: self.grid,
            // A line has no bars; the width is the bar kind's own default.
            bar_width: 0.55,
        }
    }
}

impl<'a, Message> From<LineChart> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(chart: LineChart) -> Self {
        Element::new(chart.into_plot())
    }
}

// ---------------------------------------------------------------- bar

/// A bar chart: one bar per point, growing out of the zero line.
pub struct BarChart {
    points: Vec<Point>,
    rounded: bool,
    mixed: bool,
    show_labels: bool,
    tone: usize,
    tick_margin: usize,
    plot_height: f32,
    y_ticks: usize,
    y_axis: bool,
    grid: bool,
    bar_width: f32,
}

impl BarChart {
    /// A bar per point, in the order given.
    pub fn new(points: impl IntoIterator<Item = Point>) -> Self {
        Self {
            points: points.into_iter().collect(),
            rounded: false,
            mixed: false,
            show_labels: false,
            tone: 1,
            tick_margin: 1,
            plot_height: DEFAULT_PLOT_HEIGHT,
            y_ticks: 5,
            y_axis: true,
            grid: true,
            bar_width: 0.55,
        }
    }

    /// Round the bars' corners. Slint's `rounded`.
    pub fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }

    /// One palette tone per bar rather than one for the series. Slint's
    /// `mixed`, which the legend then names.
    pub fn mixed(mut self, mixed: bool) -> Self {
        self.mixed = mixed;
        self
    }

    /// Draw each bar's value over it. Slint's `show-labels`.
    pub fn show_labels(mut self, show_labels: bool) -> Self {
        self.show_labels = show_labels;
        self
    }

    /// The series tone, wrapped to the five chart colours. Slint's `fill`.
    pub fn tone(mut self, tone: usize) -> Self {
        self.tone = tone;
        self
    }

    /// Label every `tick_margin`-th band. Slint's `tick-margin`.
    pub fn tick_margin(mut self, tick_margin: usize) -> Self {
        self.tick_margin = tick_margin.max(1);
        self
    }

    /// The plot's height in pixels, Slint's `plot-height`.
    pub fn plot_height(mut self, plot_height: f32) -> Self {
        self.plot_height = plot_height.max(1.0);
        self
    }

    /// How many labelled grid lines the value axis carries, both ends
    /// included. Slint's `value-tick-count`.
    pub fn y_ticks(mut self, y_ticks: usize) -> Self {
        self.y_ticks = y_ticks.max(2);
        self
    }

    /// Draw the value axis gutter. Slint's `value-axis`.
    pub fn y_axis(mut self, y_axis: bool) -> Self {
        self.y_axis = y_axis;
        self
    }

    /// Draw the horizontal grid lines. Slint's `grid`.
    pub fn grid(mut self, grid: bool) -> Self {
        self.grid = grid;
        self
    }

    /// Each bar's share of its band, 0.55 by default.
    pub fn bar_width(mut self, bar_width: f32) -> Self {
        self.bar_width = bar_width.clamp(0.05, 1.0);
        self
    }
}

impl<'a, Message> From<BarChart> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(chart: BarChart) -> Self {
        Element::new(Plot {
            series: Series::Bar {
                rounded: chart.rounded,
                mixed: chart.mixed,
                show_labels: chart.show_labels,
                tone: chart.tone,
            },
            points: chart.points,
            tick_margin: chart.tick_margin,
            max_value: None,
            plot_height: chart.plot_height,
            y_ticks: chart.y_ticks,
            y_axis: chart.y_axis,
            grid: chart.grid,
            bar_width: chart.bar_width,
        })
    }
}

// ---------------------------------------------------------------- legend

/// The swatch-and-name row under a chart. The Slint `ChartLegend`.
#[derive(Debug, Clone, Default)]
pub struct ChartLegend {
    items: Vec<(String, usize)>,
}

impl ChartLegend {
    /// An empty legend.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add one swatch and its name, at the palette tone it names.
    pub fn item(mut self, label: impl Into<String>, tone: usize) -> Self {
        self.items.push((label.into(), tone));
        self
    }

    /// Add one item per name, in order.
    pub fn items<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for (index, label) in items.into_iter().enumerate() {
            self.items.push((label.into(), index));
        }
        self
    }
}

impl<'a, Message> From<ChartLegend> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(legend: ChartLegend) -> Self {
        let mut items = row![].spacing(theme::SPACING_MD);
        for (label, tone) in legend.items {
            let swatch =
                container(Space::new().width(8.0).height(8.0)).style(move |theme: &Theme| {
                    container::Style {
                        background: Some(tone_color(&Tokens::of(theme), tone).into()),
                        border: iced::border::rounded(2.0),
                        ..container::Style::default()
                    }
                });
            items = items.push(
                row![
                    swatch,
                    text(label)
                        .size(theme::FONT_SIZE_XS)
                        .style(|theme: &Theme| text::Style {
                            color: Some(Tokens::of(theme).muted_foreground),
                        }),
                ]
                .spacing(theme::SPACING_XS)
                .align_y(iced::Alignment::Center),
            );
        }
        items.wrap().into()
    }
}

// ---------------------------------------------------------------- plot

/// Which series kind a plot draws.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Series {
    Line {
        interpolation: Interpolation,
        dots: bool,
        tone: usize,
        stroke: Option<Color>,
    },
    Bar {
        rounded: bool,
        mixed: bool,
        show_labels: bool,
        tone: usize,
    },
}

/// The hover the widget tree holds: the datum under the pointer, if any.
#[derive(Debug, Default)]
struct Hover {
    index: Option<usize>,
}

/// The plot the two builders reduce to: the shared surface, so the axes, the
/// grid and the hover are written once.
struct Plot {
    series: Series,
    points: Vec<Point>,
    tick_margin: usize,
    max_value: Option<f32>,
    plot_height: f32,
    y_ticks: usize,
    y_axis: bool,
    grid: bool,
    bar_width: f32,
}

impl Plot {
    /// The top and bottom of the value axis, before rounding: a line fits the
    /// data from zero, Slint's `py`; a bar always includes the zero line.
    fn scale(&self) -> (f32, f32) {
        let mut highest = 0.0f32;
        let mut lowest = 0.0f32;
        for point in &self.points {
            highest = highest.max(point.value);
            lowest = lowest.min(point.value);
        }

        match self.series {
            Series::Line { .. } => {
                let top = self.max_value.unwrap_or(highest).max(1.0);
                (0.0, top)
            }
            Series::Bar { show_labels, .. } => {
                let mut top = if highest > 0.0 { highest } else { 0.0 };
                let mut bottom = if lowest < 0.0 { lowest } else { 0.0 };
                if show_labels && (top - bottom).abs() > f32::EPSILON {
                    // Slint keeps "a line of text clear above the tallest bar",
                    // so a value label is drawn inside the chart. One label line
                    // of headroom in value space, rounded up to a readable tick.
                    let pad = (top - bottom) * (LABEL_LINE + 2.0) / self.plot_height;
                    if top > 0.0 {
                        top = nice_ceiling(top + pad);
                    }
                    if bottom < 0.0 {
                        bottom = -nice_ceiling(-bottom + pad);
                    }
                }
                if (top - bottom).abs() < f32::EPSILON {
                    (0.0, 1.0)
                } else {
                    (bottom, top)
                }
            }
        }
    }

    /// The values the grid lines and their labels mark, bottom first, both ends
    /// included — Slint's `y-tick-count`.
    fn tick_values(&self, bottom: f32, top: f32) -> Vec<f32> {
        let count = self.y_ticks.max(2);
        (0..count)
            .map(|index| bottom + (top - bottom) * index as f32 / (count - 1) as f32)
            .collect()
    }

    /// How wide the value-axis gutter has to be for its widest label.
    fn gutter(&self, ticks: &[f32]) -> f32 {
        if !self.y_axis {
            return 0.0;
        }
        let widest = ticks
            .iter()
            .map(|value| estimate(&format_value(*value), LABEL_SIZE))
            .fold(0.0f32, f32::max);
        widest + theme::SPACING_SM
    }

    /// The total height: the top gap, the plot and the label strip.
    fn height(&self) -> f32 {
        TOP_PAD + self.plot_height + X_LABEL_HEIGHT
    }

    /// Which datum the pointer is asking about, or none when it is away. The
    /// line asks for the nearest point, the bar for the band — Slint's
    /// `ChartFormat.nearest` and `ChartFormat.band`.
    fn hovered(&self, geometry: Geometry, position: iced::Point) -> Option<usize> {
        let plot = geometry.plot;
        let count = self.points.len();
        if count == 0 || plot.width <= 0.0 {
            return None;
        }
        if position.x < plot.x - HOVER_INSET
            || position.x > plot.x + plot.width + HOVER_INSET
            || position.y < plot.y - HOVER_INSET
            || position.y > plot.y + plot.height + HOVER_INSET
        {
            return None;
        }
        let fraction = ((position.x - plot.x) / plot.width).clamp(0.0, 1.0);
        let index = match self.series {
            Series::Line { .. } => {
                if count <= 1 {
                    0
                } else {
                    (fraction * (count - 1) as f32).round() as usize
                }
            }
            Series::Bar { .. } => ((fraction * count as f32).floor() as usize).min(count - 1),
        };
        Some(index)
    }

    /// The tone a series or a bar draws in. `index` is the bar's position, which
    /// a `mixed` chart colours one tone per bar.
    fn plot_tone(&self, tokens: &Tokens, index: usize) -> Color {
        match self.series {
            Series::Line { tone, stroke, .. } => stroke.unwrap_or_else(|| tone_color(tokens, tone)),
            Series::Bar { mixed, tone, .. } => {
                if mixed {
                    tone_color(tokens, index)
                } else {
                    tone_color(tokens, tone)
                }
            }
        }
    }
}

/// The plot's geometry inside the widget's bounds.
#[derive(Debug, Clone, Copy)]
struct Geometry {
    plot: Rectangle,
}

impl Geometry {
    fn new(bounds: Rectangle, gutter: f32, plot_height: f32) -> Self {
        Self {
            plot: Rectangle {
                x: bounds.x + gutter,
                y: bounds.y + TOP_PAD,
                width: (bounds.width - gutter - RIGHT_PAD).max(1.0),
                height: plot_height.max(1.0),
            },
        }
    }

    /// The y a value sits at, the axis running bottom to top.
    fn y(&self, value: f32, bottom: f32, top: f32) -> f32 {
        let span = (top - bottom).max(f32::EPSILON);
        self.plot.y + self.plot.height * (1.0 - (value - bottom) / span)
    }

    /// The x at the centre of a band, for bars and their labels.
    fn band(&self, index: usize, count: usize) -> f32 {
        self.plot.x + self.plot.width * (index as f32 + 0.5) / count.max(1) as f32
    }

    /// The x of a point, for a line and its labels.
    fn point_x(&self, index: usize, count: usize) -> f32 {
        if count <= 1 {
            self.plot.x + self.plot.width / 2.0
        } else {
            self.plot.x + self.plot.width * index as f32 / (count - 1) as f32
        }
    }

    /// The width of one band.
    fn band_width(&self, count: usize) -> f32 {
        self.plot.width / count.max(1) as f32
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Plot {
    fn tag(&self) -> Tag {
        Tag::of::<Hover>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Hover::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(self.height()))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, Length::Fixed(self.height()))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let hover = tree.state.downcast_mut::<Hover>();

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                // The geometry depends on the gutter, which depends on the
                // ticks, so it is rebuilt here exactly as `draw` builds it.
                let next = cursor
                    .position()
                    .and_then(|position| self.hovered(self.geometry(layout.bounds()), position));
                if next != hover.index {
                    hover.index = next;
                    // iced repaints on request, so a change asks for the frame
                    // that draws the guide and the tooltip.
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorLeft) => {
                if hover.index.is_some() {
                    hover.index = None;
                    shell.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.width < 2.0 || bounds.height < 2.0 {
            return;
        }

        // Every token is read when iced draws, so a theme switch moves the
        // chart with the page.
        let tokens = Tokens::of(theme);
        let (bottom, top) = self.scale();
        let ticks = self.tick_values(bottom, top);
        let geometry = self.geometry(bounds);
        let plot = geometry.plot;
        let count = self.points.len();
        let grid_color = tokens.border.scale_alpha(GRID_ALPHA);

        // The grid sits behind the series, as Slint's `ChartGrid` does.
        if self.grid {
            for value in &ticks {
                let y = geometry.y(*value, bottom, top);
                fill(
                    renderer,
                    Rectangle {
                        x: plot.x,
                        y,
                        width: plot.width,
                        height: 1.0,
                    },
                    grid_color,
                    0.0,
                );
            }
        }

        // The value axis: a label ending just left of every tick line. The
        // right edge is computed rather than asked for, because iced's horizontal
        // alignment is a no-op for a single line (see `draw_text`).
        if self.y_axis {
            for value in &ticks {
                let y = geometry.y(*value, bottom, top);
                let label = format_value(*value);
                let width = estimate(&label, LABEL_SIZE);
                draw_text(
                    renderer,
                    &label,
                    iced::Point::new(plot.x - theme::SPACING_SM - width, y - LABEL_LINE / 2.0),
                    Size::new(width + 2.0, LABEL_LINE),
                    LABEL_SIZE,
                    iced::Font::DEFAULT,
                    iced::alignment::Vertical::Center,
                    tokens.muted_foreground,
                    bounds,
                );
            }
        }

        // The baseline, drawn under the series so a bar does not float.
        let zero = geometry.y(0.0f32.clamp(bottom, top), bottom, top);
        fill(
            renderer,
            Rectangle {
                x: plot.x,
                y: zero,
                width: plot.width,
                height: 1.0,
            },
            tokens.border,
            0.0,
        );

        // The x labels, one band each with Slint's stride between them. Both
        // kinds label the band's centre, which is Slint's `space-around`
        // `PlotLabels`: a label centred on the first or last *point* would hang
        // half a band outside the plot and be clipped.
        for index in 0..count {
            if index % self.tick_margin != 0 {
                continue;
            }
            let label = self.points[index].label.as_str();
            let width = estimate(label, LABEL_SIZE);
            draw_text(
                renderer,
                label,
                iced::Point::new(
                    geometry.band(index, count) - width / 2.0,
                    plot.y + plot.height + 3.0,
                ),
                Size::new(width + 2.0, LABEL_LINE),
                LABEL_SIZE,
                iced::Font::DEFAULT,
                iced::alignment::Vertical::Top,
                tokens.muted_foreground,
                bounds,
            );
        }

        // The series itself.
        match self.series {
            Series::Line {
                interpolation,
                dots,
                ..
            } => {
                draw_line_series(
                    renderer,
                    &self.points,
                    interpolation,
                    dots,
                    self.plot_tone(&tokens, 0),
                    &geometry,
                    bottom,
                    top,
                );
            }
            Series::Bar {
                rounded,
                show_labels,
                ..
            } => {
                for index in 0..count {
                    let band = geometry.band_width(count);
                    let width = (band * self.bar_width).max(1.0);
                    let x = geometry.band(index, count) - width / 2.0;
                    let value = self.points[index].value;
                    let y = geometry.y(value, bottom, top);
                    let quad = Rectangle {
                        x,
                        y: y.min(zero),
                        width,
                        height: (zero - y).abs().max(1.0),
                    };
                    let radius = if rounded { (width / 2.0).min(6.0) } else { 3.0 };
                    fill(renderer, quad, self.plot_tone(&tokens, index), radius);

                    if show_labels {
                        // Above a bar that grows up, below one that grows down,
                        // so the label is never inside the plot's neighbour.
                        let above = value >= 0.0;
                        let label_y = if above {
                            quad.y - LABEL_LINE - 1.0
                        } else {
                            quad.y + quad.height + 1.0
                        };
                        let label = format_value(value);
                        let width = estimate(&label, LABEL_SIZE);
                        draw_text(
                            renderer,
                            &label,
                            iced::Point::new(geometry.band(index, count) - width / 2.0, label_y),
                            Size::new(width + 2.0, LABEL_LINE),
                            LABEL_SIZE,
                            iced::Font::DEFAULT,
                            iced::alignment::Vertical::Top,
                            tokens.foreground,
                            bounds,
                        );
                    }
                }
            }
        }

        // The hover: the guide, the mark, and the tooltip.
        let hover = tree.state.downcast_ref::<Hover>();
        if let (Some(index), Some(position)) = (hover.index, cursor.position()) {
            if index < count {
                let mark_x = match self.series {
                    Series::Line { .. } => geometry.point_x(index, count),
                    Series::Bar { .. } => geometry.band(index, count),
                };

                // A bar's mark is the whole band, the way Slint's `ChartHover`
                // highlights the hovered bar; a faint foreground tint reads on
                // the page and on the bar in either theme.
                if matches!(self.series, Series::Bar { .. }) {
                    let band = geometry.band_width(count);
                    fill(
                        renderer,
                        Rectangle {
                            x: geometry.band(index, count) - band / 2.0,
                            y: plot.y,
                            width: band,
                            height: plot.height,
                        },
                        tokens.foreground.scale_alpha(0.06),
                        0.0,
                    );
                }

                // Slint's `ChartHover`: a 1px guide in the foreground at 0.28.
                fill(
                    renderer,
                    Rectangle {
                        x: mark_x,
                        y: plot.y,
                        width: 1.0,
                        height: plot.height,
                    },
                    tokens.foreground.scale_alpha(GUIDE_ALPHA),
                    0.0,
                );

                if matches!(self.series, Series::Line { .. }) {
                    // A dot on the line at the hovered point, ringed in the page
                    // so it reads off the line it sits on.
                    let y = geometry.y(self.points[index].value, bottom, top);
                    fill(renderer, dot(mark_x, y, 5.0), tokens.background, 5.0);
                    fill(
                        renderer,
                        dot(mark_x, y, 3.0),
                        self.plot_tone(&tokens, 0),
                        3.0,
                    );
                }

                draw_tooltip(
                    renderer,
                    &tokens,
                    bounds,
                    position,
                    &self.points[index].label,
                    &format_value(self.points[index].value),
                );
            }
        }
    }
}

impl Plot {
    /// The geometry the widget draws with, from its bounds.
    fn geometry(&self, bounds: Rectangle) -> Geometry {
        let (bottom, top) = self.scale();
        let ticks = self.tick_values(bottom, top);
        Geometry::new(bounds, self.gutter(&ticks), self.plot_height)
    }
}

/// A filled quad with a corner radius.
fn fill(renderer: &mut iced::Renderer, bounds: Rectangle, color: Color, radius: f32) {
    renderer.fill_quad(
        Quad {
            bounds,
            border: iced::border::rounded(radius),
            ..Quad::default()
        },
        color,
    );
}

/// A circle, as a square rounded to half its side.
fn dot(x: f32, y: f32, radius: f32) -> Rectangle {
    Rectangle {
        x: x - radius,
        y: y - radius,
        width: radius * 2.0,
        height: radius * 2.0,
    }
}

/// A line series: the segments, then the dots.
fn draw_line_series(
    renderer: &mut iced::Renderer,
    points: &[Point],
    interpolation: Interpolation,
    dots: bool,
    color: Color,
    geometry: &Geometry,
    bottom: f32,
    top: f32,
) {
    let count = points.len();
    if count == 0 {
        return;
    }

    for index in 1..count {
        let from = iced::Point::new(
            geometry.point_x(index - 1, count),
            geometry.y(points[index - 1].value, bottom, top),
        );
        let to = iced::Point::new(
            geometry.point_x(index, count),
            geometry.y(points[index].value, bottom, top),
        );
        match interpolation {
            Interpolation::Linear => segment(renderer, from, to, color),
            Interpolation::Step => {
                // Hold the old value across the band, then step: Slint's
                // `step after` join.
                segment(renderer, from, iced::Point::new(to.x, from.y), color);
                segment(renderer, iced::Point::new(to.x, from.y), to, color);
            }
        }
    }

    if dots {
        for index in 0..count {
            let centre = iced::Point::new(
                geometry.point_x(index, count),
                geometry.y(points[index].value, bottom, top),
            );
            fill(renderer, dot(centre.x, centre.y, 3.0), color, 3.0);
        }
    }
}

/// One straight segment, as a run of one-pixel quads sampled along it. iced's
/// `Transformation` has no rotation, so a sloped stroke has no other primitive;
/// the quads overlap slightly so the join does not thin out.
fn segment(renderer: &mut iced::Renderer, from: iced::Point, to: iced::Point, color: Color) {
    let dx = to.x - from.x;
    let dy = to.y - from.y;

    if dx.abs() < 0.01 {
        fill(
            renderer,
            Rectangle {
                x: from.x - LINE_WIDTH / 2.0,
                y: from.y.min(to.y),
                width: LINE_WIDTH,
                height: dy.abs().max(LINE_WIDTH),
            },
            color,
            0.0,
        );
        return;
    }

    let step = 0.5;
    let steps = (dx.abs() / step).ceil().max(1.0) as usize;
    let pitch = dx.abs() / steps as f32;
    // The taller the slope, the taller each sample has to be to close the gap
    // to its neighbour.
    let height = LINE_WIDTH + dy.abs() / steps as f32;

    for index in 0..=steps {
        let t = index as f32 / steps as f32;
        let x = from.x + dx * t;
        let y = from.y + dy * t;
        fill(
            renderer,
            Rectangle {
                x,
                y: y - height / 2.0,
                width: pitch + 0.5,
                height,
            },
            color,
            0.0,
        );
    }
}

/// The tooltip: the popover surface beside the pointer, flipping toward the
/// centre of the plot near an edge, with the datum's name and value.
fn draw_tooltip(
    renderer: &mut iced::Renderer,
    tokens: &Tokens,
    bounds: Rectangle,
    cursor: iced::Point,
    title: &str,
    detail: &str,
) {
    // iced 0.14 exposes no text measurement to a custom widget, so the box is
    // sized from the character count. It is honest about being an estimate,
    // which is why the padding is generous on every side.
    let width =
        estimate(title, LABEL_SIZE).max(estimate(detail, LABEL_SIZE)) + theme::SPACING_MD * 2.0;
    let height = LABEL_LINE * 2.0 + 1.0 + theme::SPACING_SM + 2.0;

    let raw_x = if cursor.x + 14.0 + width > bounds.x + bounds.width - 4.0 {
        cursor.x - width - 12.0
    } else {
        cursor.x + 14.0
    };
    let raw_y = if cursor.y + 16.0 + height > bounds.y + bounds.height - 4.0 {
        cursor.y - height - 12.0
    } else {
        cursor.y + 16.0
    };
    let x = raw_x.clamp(
        bounds.x + 4.0,
        (bounds.x + bounds.width - width - 4.0).max(bounds.x + 4.0),
    );
    let y = raw_y.clamp(
        bounds.y + 4.0,
        (bounds.y + bounds.height - height - 4.0).max(bounds.y + 4.0),
    );

    fill(
        renderer,
        Rectangle {
            x,
            y,
            width,
            height,
        },
        tokens.popover,
        theme::RADIUS_MD,
    );
    renderer.fill_quad(
        Quad {
            bounds: Rectangle {
                x,
                y,
                width,
                height,
            },
            border: iced::border::rounded(theme::RADIUS_MD)
                .color(tokens.border)
                .width(1.0),
            shadow: iced::Shadow {
                color: tokens.shadow,
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Quad::default()
        },
        Color::TRANSPARENT,
    );

    let text_x = x + theme::SPACING_MD;
    let text_width = (width - theme::SPACING_MD * 2.0).max(1.0);
    draw_text(
        renderer,
        title,
        iced::Point::new(text_x, y + theme::SPACING_XS),
        Size::new(text_width, LABEL_LINE),
        LABEL_SIZE,
        iced::Font {
            weight: iced::font::Weight::Semibold,
            ..iced::Font::DEFAULT
        },
        iced::alignment::Vertical::Top,
        tokens.foreground,
        bounds,
    );
    draw_text(
        renderer,
        detail,
        iced::Point::new(text_x, y + theme::SPACING_XS + LABEL_LINE + 1.0),
        Size::new(text_width, LABEL_LINE),
        LABEL_SIZE,
        iced::Font::DEFAULT,
        iced::alignment::Vertical::Top,
        tokens.muted_foreground,
        bounds,
    );
}

/// Draw one line of text, with every attribute the flat `Text` carries.
#[allow(clippy::too_many_arguments)]
fn draw_text(
    renderer: &mut iced::Renderer,
    content: &str,
    position: iced::Point,
    bounds: Size,
    size: f32,
    font: iced::Font,
    align_y: iced::alignment::Vertical,
    color: Color,
    clip: Rectangle,
) {
    renderer.fill_text(
        CoreText {
            content: content.to_string(),
            bounds,
            size: Pixels(size),
            line_height: LineHeight::Relative(1.3),
            font,
            // iced only applies `align_x` to a paragraph with more than one
            // line (`iced_graphics::text::align`), so a single line is always
            // left-aligned at `position`. Every caller computes its own x.
            align_x: TextAlignment::Left,
            align_y,
            shaping: Shaping::default(),
            wrapping: Wrapping::None,
        },
        position,
        color,
        clip,
    );
}

/// A number as a tick or a value label: whole when it is whole, one decimal
/// otherwise, which is the Slint `ChartFormat.num` with a fractional case.
fn format_value(value: f32) -> String {
    if (value - value.round()).abs() < 0.05 {
        format!("{}", value.round() as i64)
    } else {
        format!("{value:.1}")
    }
}

/// The smallest readable tick value at or above `value`: a 1/1.5/2/2.5/3/4/5/6/8
/// step of the value's own magnitude, so a value axis that grows for a bar label
/// still reads as round numbers.
fn nice_ceiling(value: f32) -> f32 {
    if !(value > 0.0) {
        return 1.0;
    }
    let magnitude = 10f32.powf(value.log10().floor());
    for step in [1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0] {
        let candidate = magnitude * step;
        if candidate >= value - 1e-4 {
            return candidate;
        }
    }
    magnitude * 10.0
}

/// How wide a line of text is, near enough to place a quad behind it. iced
/// 0.14 keeps the shaper that would answer this exactly.
fn estimate(value: &str, size: f32) -> f32 {
    value.chars().count() as f32 * size * 0.62
}
