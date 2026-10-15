//! `chart` — the plot surface: a plot area, its axes and grid, five series
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
//! - **[`AreaChart`]** — the Slint `AreaChart`: the line, with the area under it
//!   filled at 35% of the series tone, the same joins and the same pinned axis.
//! - **[`PieChart`]** — the Slint `PieChart`: a pie or a donut whose slices are
//!   palette tones, with an optional gap between slices and optional labels at
//!   the ring's midpoint.
//! - **[`RadarChart`]** — the Slint `RadarChart`: one or two closed polygons on
//!   a ring grid, with optional dots and an outline-only mode.
//! - **[`CandlestickChart`]** — the Slint `CandlestickChart`: a wick and a body
//!   per OHLC datum, coloured by whether the close beat the open.
//! - **[`ChartLegend`]** — the swatch-and-name row, the Slint `ChartLegend`.
//! - **Axes and guides** — `y_ticks` labelled grid lines in a gutter that widens
//!   to fit them, and x labels under the plot with the Slint `tick_margin`
//!   stride.
//! - **Hover** — the crosshair and the dot follow the pointer, the bar under it
//!   is marked, and a popover-surface tooltip names the datum and its value.
//!   The state lives in the widget tree, the route `resizable` takes.
//! - **Appear** — every series draws in over one second on ease-out quart, the
//!   Slint `ChartMotion` and GPUI's `PLOT_APPEAR` alike: a line, an area, a
//!   candlestick and the sankey's would-be ribbons are revealed from the left,
//!   bars grow out of the zero line, a pie sweeps clockwise from its first
//!   slice, and a radar grows out of its centre. Axes, grid and labels are there
//!   from the first frame.
//! - **Tooltip transition** — the whole hover overlay fades in when the cursor
//!   lands on a datum and out after it leaves.
//!
//! **Named, not faked.** Slint's chart is seven kinds and forty cards; the two
//! the page documents first plus three more are here. Not ported:
//!
//! - `SankeyChart`. Its ribbons are cubic-bezier bands with a per-link gradient,
//!   which is a path this crate cannot draw without a redesign of the primitive;
//!   the candlestick, by contrast, is wicks and bodies and fell out of the same
//!   quads the bars use, so it is here.
//! - The **`curve`** interpolation. Slint's default is a natural cubic spline;
//!   iced exposes no path, so [`Interpolation`] offers `Linear` and `Step`, and
//!   the default is `Linear` where Slint's is `curve`.
//! - The **left/right/top bar alignments**, `gradient` fills, `stacked` series,
//!   `show-labels` colouring, and `minimum` stubs. Bars are vertical and grow
//!   from zero; areas are one series, not two stacked ones.
//! - **Elision**, as everywhere else in this catalog: a label wider than its
//!   column is clipped, not shortened with an ellipsis.
//!
//! **Where iced cannot match the reference exactly.** The appear starts on the
//! first frame the plot is asked to paint rather than when it scrolls into view:
//! iced gives a widget no "entered the viewport" signal, and Slint's
//! `absolute-position.y` test has no counterpart. It also cannot honour the
//! operating-system reduced-motion preference, which iced 0.14 does not expose,
//! so the appear runs regardless; the Slint source says "reduced motion skips the
//! appear". The Slint chart's tooltip itself has no transition — it shows and
//! hides on `has-hover` — so the fade here is GPUI's `PlotMotion`: 120 ms in on
//! `cubic-bezier(0.16, 1, 0.3, 1)` and 120 ms out on
//! `cubic-bezier(0.4, 0, 1, 1)`. The appear is Slint's exact curve,
//! `duration: 1s; easing: cubic-bezier(0.165, 0.84, 0.44, 1)`.
//!
//! **Drawn as quads and text, not through `canvas`.** A line is a run of
//! one-pixel-wide quads sampled along the segment, because iced's
//! `Transformation` has no rotation and a sloped stroke has no other quad
//! primitive; an area fill is one column of quads per pixel, a pie or donut
//! slice one row of quads per pixel, a radar polygon one run of quads per scan
//! line. This is also the path the brief asks for: the `Canvas` that once
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
use iced::time::{Duration, Instant};
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
/// The area fill's alpha, Slint's `fill.with-alpha(0.35)`.
const AREA_ALPHA: f32 = 0.35;
/// A radar polygon's fill alpha, Slint's `stroke.with-alpha(0.28)` and
/// `stroke-b.with-alpha(0.22)`.
const RADAR_ALPHA: f32 = 0.28;
/// A second radar polygon's fill alpha.
const RADAR_ALPHA_B: f32 = 0.22;
/// The radar's radius as a share of its plot box, Slint's `radius: 32` in a
/// 100-unit viewbox.
const RADAR_RADIUS: f32 = 0.32;
/// How far past the outer ring a radar label sits, Slint's `gx(i, 1.18)`.
const RADAR_LABEL_REACH: f32 = 1.18;
/// The default pie/donut box, Slint's `plot-size: 180px`.
const DEFAULT_PLOT_SIZE: f32 = 180.0;
/// The donut hole when `donut` is set and no radius is given, Slint's `28`.
const DEFAULT_DONUT_INNER: f32 = 28.0;
/// The outer radius in viewbox units, Slint's `outer-radius: 46`.
const DEFAULT_OUTER_RADIUS: f32 = 46.0;
/// A pie label's distance from the centre, Slint's `plot-size * 0.42`.
const PIE_LABEL_REACH: f32 = 0.42;
/// The pie hover dot's distance from the centre, Slint's `0.34`.
const PIE_MARK_REACH: f32 = 0.34;
/// How long a series takes to draw in the first time it is painted. Slint's
/// `animate progress { duration: 1s }` and GPUI's `PLOT_APPEAR`.
const APPEAR_DURATION: Duration = Duration::from_millis(1000);
/// How long the hover overlay takes to reach the cursor, GPUI's
/// `PlotMotion::enter` at the palette's `duration_fast`.
const TOOLTIP_ENTER: Duration = Duration::from_millis(120);
/// How long it takes to leave once the cursor is gone, GPUI's
/// `PlotMotion::exit`.
const TOOLTIP_EXIT: Duration = Duration::from_millis(120);

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

/// One radar datum: the dimension's name and the value each series reaches on
/// it. The Slint `RadarPoint`, whose two series are `alpha` and `beta`.
#[derive(Debug, Clone, PartialEq)]
pub struct RadarPoint {
    label: String,
    alpha: f32,
    beta: f32,
}

impl RadarPoint {
    /// A dimension scored by both series.
    pub fn new(label: impl Into<String>, alpha: f32, beta: f32) -> Self {
        Self {
            label: label.into(),
            alpha,
            beta,
        }
    }

    /// The dimension's name, drawn outside its spoke.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The first series' value on this dimension.
    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    /// The second series' value on this dimension.
    pub fn beta(&self) -> f32 {
        self.beta
    }
}

/// One candlestick: the session's open, high, low and close. The Slint
/// `CandlePoint`.
#[derive(Debug, Clone, PartialEq)]
pub struct Candle {
    label: String,
    open: f32,
    high: f32,
    low: f32,
    close: f32,
}

impl Candle {
    /// A session's four prices, in the order a reader says them.
    pub fn new(label: impl Into<String>, open: f32, high: f32, low: f32, close: f32) -> Self {
        Self {
            label: label.into(),
            open,
            high,
            low,
            close,
        }
    }

    /// The session's name on the band axis.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The opening price.
    pub fn open(&self) -> f32 {
        self.open
    }

    /// The session's high.
    pub fn high(&self) -> f32 {
        self.high
    }

    /// The session's low.
    pub fn low(&self) -> f32 {
        self.low
    }

    /// The closing price.
    pub fn close(&self) -> f32 {
        self.close
    }

    /// Whether the session closed at or above its open, which colours it.
    fn rose(&self) -> bool {
        self.close >= self.open
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
        Element::new(Chart {
            kind: Kind::Plot(chart.into_plot()),
        })
    }
}

// ---------------------------------------------------------------- area

/// An area chart: the line, with the area between it and the baseline filled.
pub struct AreaChart {
    points: Vec<Point>,
    interpolation: Interpolation,
    tone: usize,
    stroke: Option<Color>,
    tick_margin: usize,
    max_value: Option<f32>,
    plot_height: f32,
    y_ticks: usize,
    y_axis: bool,
    grid: bool,
}

impl AreaChart {
    /// An area under the line through `points`, in the order given.
    pub fn new(points: impl IntoIterator<Item = Point>) -> Self {
        Self {
            points: points.into_iter().collect(),
            interpolation: Interpolation::default(),
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

    /// How the upper line joins its points. Slint's `interpolation`.
    pub fn interpolation(mut self, interpolation: Interpolation) -> Self {
        self.interpolation = interpolation;
        self
    }

    /// The series tone, wrapped to the five chart colours. Slint's `fill`.
    pub fn tone(mut self, tone: usize) -> Self {
        self.tone = tone;
        self.stroke = None;
        self
    }

    /// An explicit series colour, used for the stroke and, at 35%, the fill.
    pub fn stroke(mut self, stroke: Color) -> Self {
        self.stroke = Some(stroke);
        self
    }

    /// Label every `tick_margin`-th point. Slint's `tick-margin`.
    pub fn tick_margin(mut self, tick_margin: usize) -> Self {
        self.tick_margin = tick_margin.max(1);
        self
    }

    /// Pin the top of the value axis. Slint's `max-value`, which the Slint
    /// example uses for a series that is still in progress.
    pub fn max_value(mut self, max_value: f32) -> Self {
        self.max_value = Some(max_value.max(1.0));
        self
    }

    /// The plot's height in pixels, Slint's `plot-height`.
    pub fn plot_height(mut self, plot_height: f32) -> Self {
        self.plot_height = plot_height.max(1.0);
        self
    }

    /// How many labelled grid lines the value axis carries. Slint's default.
    pub fn y_ticks(mut self, y_ticks: usize) -> Self {
        self.y_ticks = y_ticks.max(2);
        self
    }

    /// Draw the value axis gutter.
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
            series: Series::Area {
                interpolation: self.interpolation,
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
            bar_width: 0.55,
        }
    }
}

impl<'a, Message> From<AreaChart> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(chart: AreaChart) -> Self {
        Element::new(Chart {
            kind: Kind::Plot(chart.into_plot()),
        })
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
        Element::new(Chart {
            kind: Kind::Plot(Plot {
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
            }),
        })
    }
}

// ---------------------------------------------------------------- pie

/// A pie or donut chart: one slice per point, sweeping clockwise from the top.
pub struct PieChart {
    points: Vec<Point>,
    donut: bool,
    pad_angle: f32,
    show_labels: bool,
    inner_radius: f32,
    outer_radius: f32,
    plot_size: f32,
}

impl PieChart {
    /// A slice per point, in the order given.
    pub fn new(points: impl IntoIterator<Item = Point>) -> Self {
        Self {
            points: points.into_iter().collect(),
            donut: false,
            pad_angle: 0.0,
            show_labels: false,
            inner_radius: 0.0,
            outer_radius: DEFAULT_OUTER_RADIUS,
            plot_size: DEFAULT_PLOT_SIZE,
        }
    }

    /// Leave a hole in the middle. Slint's `donut`.
    pub fn donut(mut self, donut: bool) -> Self {
        self.donut = donut;
        self
    }

    /// The gap between slices as a share of the full turn, Slint's
    /// `pad-angle: 0.02`.
    pub fn pad_angle(mut self, pad_angle: f32) -> Self {
        self.pad_angle = pad_angle.max(0.0);
        self
    }

    /// Draw each slice's name at the ring's midpoint. Slint's `show-labels`.
    pub fn show_labels(mut self, show_labels: bool) -> Self {
        self.show_labels = show_labels;
        self
    }

    /// The hole's radius in viewbox units, 100 across the box. Slint's
    /// `inner-radius`, which also turns a pie into a donut.
    pub fn inner_radius(mut self, inner_radius: f32) -> Self {
        self.inner_radius = inner_radius.max(0.0);
        self
    }

    /// The ring's outer radius in viewbox units. Slint's `outer-radius: 46`.
    pub fn outer_radius(mut self, outer_radius: f32) -> Self {
        self.outer_radius = outer_radius.clamp(1.0, 50.0);
        self
    }

    /// The square the ring is drawn in, Slint's `plot-size: 180px`.
    pub fn plot_size(mut self, plot_size: f32) -> Self {
        self.plot_size = plot_size.max(1.0);
        self
    }

    /// The hole's radius, Slint's `hole`: `donut` asks for the default 28 when
    /// an explicit inner radius is not given.
    fn hole(&self) -> f32 {
        if self.inner_radius > 0.0 {
            self.inner_radius
        } else if self.donut {
            DEFAULT_DONUT_INNER
        } else {
            0.0
        }
    }

    /// The sum the slices share, never zero.
    fn total(&self) -> f32 {
        self.points
            .iter()
            .map(|point| point.value.max(0.0))
            .sum::<f32>()
            .max(0.001)
    }
}

impl<'a, Message> From<PieChart> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(chart: PieChart) -> Self {
        let hole = chart.hole();
        Element::new(Chart {
            kind: Kind::Pie(Pie {
                points: chart.points,
                hole,
                pad_angle: chart.pad_angle,
                show_labels: chart.show_labels,
                outer_radius: chart.outer_radius,
                plot_size: chart.plot_size,
            }),
        })
    }
}

// ---------------------------------------------------------------- radar

/// A radar chart: one or two closed polygons around a centre, one vertex per
/// dimension.
pub struct RadarChart {
    points: Vec<RadarPoint>,
    multiple: bool,
    dots: bool,
    lines_only: bool,
    grid_levels: usize,
    tone: usize,
    tone_b: usize,
    max_value: f32,
    plot_size: f32,
    series_a: String,
    series_b: String,
}

impl RadarChart {
    /// A polygon through `points`, in the order given.
    pub fn new(points: impl IntoIterator<Item = RadarPoint>) -> Self {
        Self {
            points: points.into_iter().collect(),
            multiple: false,
            dots: false,
            lines_only: false,
            grid_levels: 4,
            tone: 0,
            tone_b: 1,
            max_value: 100.0,
            plot_size: 200.0,
            series_a: "Alpha".to_string(),
            series_b: "Beta".to_string(),
        }
    }

    /// Draw the second series as well. Slint's `multiple`.
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Draw a dot on every vertex of the first series. Slint's `dots`.
    pub fn dots(mut self, dots: bool) -> Self {
        self.dots = dots;
        self
    }

    /// Outline only, no fill. Slint's `lines-only`.
    pub fn lines_only(mut self, lines_only: bool) -> Self {
        self.lines_only = lines_only;
        self
    }

    /// How many rings the grid carries. Slint's `grid-levels: 4`.
    pub fn grid_levels(mut self, grid_levels: usize) -> Self {
        self.grid_levels = grid_levels.clamp(1, 10);
        self
    }

    /// The first series' tone. Slint's `stroke`, which defaults to `chart-1`.
    pub fn tone(mut self, tone: usize) -> Self {
        self.tone = tone;
        self
    }

    /// The second series' tone. Slint's `stroke-b`, `chart-2` by default.
    pub fn tone_b(mut self, tone_b: usize) -> Self {
        self.tone_b = tone_b;
        self
    }

    /// The value the outer ring stands for. Slint's `max-value: 100`.
    pub fn max_value(mut self, max_value: f32) -> Self {
        self.max_value = max_value.max(f32::EPSILON);
        self
    }

    /// The square the radar is drawn in, Slint's `plot-size: 200px`.
    pub fn plot_size(mut self, plot_size: f32) -> Self {
        self.plot_size = plot_size.max(1.0);
        self
    }

    /// The names the two series carry in the tooltip. Slint's `series-a` and
    /// `series-b`.
    pub fn series_names(
        mut self,
        series_a: impl Into<String>,
        series_b: impl Into<String>,
    ) -> Self {
        self.series_a = series_a.into();
        self.series_b = series_b.into();
        self
    }
}

impl<'a, Message> From<RadarChart> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(chart: RadarChart) -> Self {
        Element::new(Chart {
            kind: Kind::Radar(Radar {
                points: chart.points,
                multiple: chart.multiple,
                dots: chart.dots,
                lines_only: chart.lines_only,
                grid_levels: chart.grid_levels,
                tone: chart.tone,
                tone_b: chart.tone_b,
                max_value: chart.max_value,
                plot_size: chart.plot_size,
                series_a: chart.series_a,
                series_b: chart.series_b,
            }),
        })
    }
}

// ---------------------------------------------------------------- candlestick

/// A candlestick chart: one wick and body per OHLC datum.
pub struct CandlestickChart {
    candles: Vec<Candle>,
    body_width: f32,
    tick_margin: usize,
    plot_height: f32,
    min_price: Option<f32>,
    max_price: Option<f32>,
    y_ticks: usize,
    y_axis: bool,
    grid: bool,
    bullish: Option<Color>,
    bearish: Option<Color>,
}

impl CandlestickChart {
    /// A candle per datum, in the order given.
    pub fn new(candles: impl IntoIterator<Item = Candle>) -> Self {
        Self {
            candles: candles.into_iter().collect(),
            body_width: 0.8,
            tick_margin: 5,
            plot_height: DEFAULT_PLOT_HEIGHT,
            min_price: None,
            max_price: None,
            y_ticks: 5,
            y_axis: true,
            grid: true,
            bullish: None,
            bearish: None,
        }
    }

    /// Each body's share of its band, 0.8 by default. Slint's `body-width`.
    pub fn body_width(mut self, body_width: f32) -> Self {
        self.body_width = body_width.clamp(0.05, 1.0);
        self
    }

    /// Label every `tick_margin`-th session. Slint's `tick-margin`.
    pub fn tick_margin(mut self, tick_margin: usize) -> Self {
        self.tick_margin = tick_margin.max(1);
        self
    }

    /// The plot's height in pixels, Slint's `plot-height`.
    pub fn plot_height(mut self, plot_height: f32) -> Self {
        self.plot_height = plot_height.max(1.0);
        self
    }

    /// Pin the bottom of the value axis. Slint's `min-price`.
    pub fn min_price(mut self, min_price: f32) -> Self {
        self.min_price = Some(min_price);
        self
    }

    /// Pin the top of the value axis. Slint's `max-price`.
    pub fn max_price(mut self, max_price: f32) -> Self {
        self.max_price = Some(max_price);
        self
    }

    /// How many labelled grid lines the value axis carries.
    pub fn y_ticks(mut self, y_ticks: usize) -> Self {
        self.y_ticks = y_ticks.max(2);
        self
    }

    /// Draw the value axis gutter.
    pub fn y_axis(mut self, y_axis: bool) -> Self {
        self.y_axis = y_axis;
        self
    }

    /// Draw the horizontal grid lines. Slint's `grid`.
    pub fn grid(mut self, grid: bool) -> Self {
        self.grid = grid;
        self
    }

    /// The colour of a candle that closed at or above its open. Slint's
    /// `bullish`, the theme's `success`.
    pub fn bullish(mut self, bullish: Color) -> Self {
        self.bullish = Some(bullish);
        self
    }

    /// The colour of a candle that closed below its open. Slint's `bearish`,
    /// the theme's `danger`.
    pub fn bearish(mut self, bearish: Color) -> Self {
        self.bearish = Some(bearish);
        self
    }
}

impl<'a, Message> From<CandlestickChart> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(chart: CandlestickChart) -> Self {
        Element::new(Chart {
            kind: Kind::Candles(Candles {
                candles: chart.candles,
                body_width: chart.body_width,
                tick_margin: chart.tick_margin,
                plot_height: chart.plot_height,
                min_price: chart.min_price,
                max_price: chart.max_price,
                y_ticks: chart.y_ticks,
                y_axis: chart.y_axis,
                grid: chart.grid,
                bullish: chart.bullish,
                bearish: chart.bearish,
            }),
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

/// Which series kind a cartesian plot draws.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Series {
    Line {
        interpolation: Interpolation,
        dots: bool,
        tone: usize,
        stroke: Option<Color>,
    },
    Area {
        interpolation: Interpolation,
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

/// The plot a line, area or bar builder reduces to: the shared surface, so the
/// axes, the grid and the hover are written once.
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
    /// The top and bottom of the value axis, before rounding: a line and an area
    /// fit the data from zero, Slint's `py`; a bar always includes the zero line.
    fn scale(&self) -> (f32, f32) {
        let mut highest = 0.0f32;
        let mut lowest = 0.0f32;
        for point in &self.points {
            highest = highest.max(point.value);
            lowest = lowest.min(point.value);
        }

        match self.series {
            Series::Line { .. } | Series::Area { .. } => {
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
    fn ticks(&self, bottom: f32, top: f32) -> Vec<f32> {
        tick_values(bottom, top, self.y_ticks)
    }

    /// How wide the value-axis gutter has to be for its widest label.
    fn gutter(&self, ticks: &[f32]) -> f32 {
        gutter_width(self.y_axis, ticks)
    }

    /// The total height: the top gap, the plot and the label strip.
    fn height(&self) -> f32 {
        TOP_PAD + self.plot_height + X_LABEL_HEIGHT
    }

    /// Which datum the pointer is asking about, or none when it is away. The
    /// line and the area ask for the nearest point, the bar for the band —
    /// Slint's `ChartFormat.nearest` and `ChartFormat.band`.
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
            Series::Line { .. } | Series::Area { .. } => {
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
            Series::Line { tone, stroke, .. } | Series::Area { tone, stroke, .. } => {
                stroke.unwrap_or_else(|| tone_color(tokens, tone))
            }
            Series::Bar { mixed, tone, .. } => {
                if mixed {
                    tone_color(tokens, index)
                } else {
                    tone_color(tokens, tone)
                }
            }
        }
    }

    /// The geometry the widget draws with, from its bounds.
    fn geometry(&self, bounds: Rectangle) -> Geometry {
        let (bottom, top) = self.scale();
        let ticks = self.ticks(bottom, top);
        Geometry::new(bounds, self.gutter(&ticks), self.plot_height)
    }

    /// The hovered datum's tooltip title and detail rows.
    fn tooltip(&self, index: usize) -> (String, Vec<String>) {
        (
            self.points[index].label.clone(),
            vec![format_value(self.points[index].value)],
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        tokens: &Tokens,
        bounds: Rectangle,
        hover: Option<usize>,
        position: Option<iced::Point>,
        appear: f32,
        fade: f32,
    ) {
        let (bottom, top) = self.scale();
        let ticks = self.ticks(bottom, top);
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

        // The series itself. A line, an area and the sankey's ribbons draw in
        // from the left behind a moving clip, Slint's `ChartReveal`; bars grow
        // out of the zero line instead, so their own length carries the appear.
        let series = |renderer: &mut iced::Renderer| {
            self.draw_series(renderer, tokens, &geometry, bottom, top, zero, appear);
        };
        match self.series {
            Series::Line { .. } | Series::Area { .. } => {
                if appear >= 1.0 {
                    series(renderer);
                } else if appear > 0.0 {
                    renderer.with_layer(
                        Rectangle {
                            x: plot.x,
                            y: bounds.y,
                            width: plot.width * appear,
                            height: bounds.height,
                        },
                        series,
                    );
                }
            }
            Series::Bar { .. } => series(renderer),
        }

        // The hover: the guide, the mark, and the tooltip, all at the overlay's
        // current opacity so it fades in and out as one surface.
        if fade > 0.0 {
            if let Some(index) = hover {
                if index < count {
                    self.draw_hover(
                        renderer, tokens, bounds, &geometry, bottom, top, index, position, fade,
                    );
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_series(
        &self,
        renderer: &mut iced::Renderer,
        tokens: &Tokens,
        geometry: &Geometry,
        bottom: f32,
        top: f32,
        zero: f32,
        appear: f32,
    ) {
        let count = self.points.len();
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
                    self.plot_tone(tokens, 0),
                    geometry,
                    bottom,
                    top,
                );
            }
            Series::Area { interpolation, .. } => {
                let color = self.plot_tone(tokens, 0);
                fill_area(
                    renderer,
                    &self.points,
                    interpolation,
                    color.scale_alpha(AREA_ALPHA),
                    geometry,
                    bottom,
                    top,
                );
                draw_line_series(
                    renderer,
                    &self.points,
                    interpolation,
                    false,
                    color,
                    geometry,
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
                    // The bar grows out of the zero line, so its length carries
                    // the appear and its head moves with it.
                    let full = (zero - y).abs().max(1.0);
                    let height = (full * appear).max(0.0);
                    let quad = if value >= 0.0 {
                        Rectangle {
                            x,
                            y: zero - height,
                            width,
                            height,
                        }
                    } else {
                        Rectangle {
                            x,
                            y: zero,
                            width,
                            height,
                        }
                    };
                    if height <= 0.0 {
                        continue;
                    }
                    let radius = if rounded { (width / 2.0).min(6.0) } else { 3.0 };
                    fill(renderer, quad, self.plot_tone(tokens, index), radius);

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
                            geometry.plot,
                        );
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_hover(
        &self,
        renderer: &mut iced::Renderer,
        tokens: &Tokens,
        bounds: Rectangle,
        geometry: &Geometry,
        bottom: f32,
        top: f32,
        index: usize,
        position: Option<iced::Point>,
        fade: f32,
    ) {
        let plot = geometry.plot;
        let count = self.points.len();
        let mark_x = match self.series {
            Series::Line { .. } | Series::Area { .. } => geometry.point_x(index, count),
            Series::Bar { .. } => geometry.band(index, count),
        };

        // A bar's mark is the whole band, the way Slint's `ChartHover`
        // highlights the hovered bar; a faint foreground tint reads on the page
        // and on the bar in either theme.
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
                tokens.foreground.scale_alpha(0.06 * fade),
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
            tokens.foreground.scale_alpha(GUIDE_ALPHA * fade),
            0.0,
        );

        if matches!(self.series, Series::Line { .. } | Series::Area { .. }) {
            // A dot on the line at the hovered point, ringed in the page so it
            // reads off the line it sits on.
            let y = geometry.y(self.points[index].value, bottom, top);
            fill(
                renderer,
                dot(mark_x, y, 5.0),
                tokens.background.scale_alpha(fade),
                5.0,
            );
            fill(
                renderer,
                dot(mark_x, y, 3.0),
                self.plot_tone(tokens, 0).scale_alpha(fade),
                3.0,
            );
        }

        let (title, lines) = self.tooltip(index);
        draw_tooltip(
            renderer,
            tokens,
            bounds,
            position.unwrap_or(iced::Point::new(mark_x, plot.y)),
            &title,
            &lines,
            fade,
        );
    }
}

// ---------------------------------------------------------------- pie plot

/// The pie or donut the builder reduces to.
struct Pie {
    points: Vec<Point>,
    hole: f32,
    pad_angle: f32,
    show_labels: bool,
    outer_radius: f32,
    plot_size: f32,
}

impl Pie {
    fn total(&self) -> f32 {
        self.points
            .iter()
            .map(|point| point.value.max(0.0))
            .sum::<f32>()
            .max(0.001)
    }

    /// `outer` and `hole` in pixels for the box the slice is drawn in.
    fn radii(&self, bounds: Rectangle) -> (f32, f32, iced::Point) {
        let outer = self.plot_size * self.outer_radius / 100.0;
        let hole = self.plot_size * self.hole / 100.0;
        let centre = iced::Point::new(
            bounds.x + bounds.width / 2.0,
            bounds.y + self.plot_size / 2.0,
        );
        (hole, outer, centre)
    }

    fn height(&self) -> f32 {
        self.plot_size + if self.show_labels { 8.0 } else { 0.0 }
    }

    /// The slice whose angle the pointer falls in, Slint's `pie-holds`, with the
    /// reach test that keeps the pointer on the ring.
    fn hovered(&self, bounds: Rectangle, position: iced::Point) -> Option<usize> {
        if self.points.is_empty() {
            return None;
        }
        let (hole, outer, centre) = self.radii(bounds);
        let dx = position.x - centre.x;
        let dy = position.y - centre.y;
        let reach = (dx * dx + dy * dy).sqrt();
        if reach > outer + 6.0 || reach + 4.0 < hole {
            return None;
        }
        let turn = dy.atan2(dx).to_degrees() + 90.0;
        let turn = if turn < 0.0 { turn + 360.0 } else { turn };
        let fraction = turn / 360.0;
        let total = self.total();
        let mut prefix = 0.0;
        for (index, point) in self.points.iter().enumerate() {
            let next = prefix + point.value.max(0.0) / total;
            if fraction >= prefix && fraction < next {
                return Some(index);
            }
            prefix = next;
        }
        Some(self.points.len() - 1)
    }

    #[allow(clippy::too_many_arguments)]
    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        tokens: &Tokens,
        bounds: Rectangle,
        hover: Option<usize>,
        position: Option<iced::Point>,
        appear: f32,
        fade: f32,
    ) {
        let (hole, outer, centre) = self.radii(bounds);
        let total = self.total();
        let pad = self.pad_angle * 360.0 * appear;
        let mut prefix = 0.0f32;

        for (index, point) in self.points.iter().enumerate() {
            let fraction = point.value.max(0.0) / total;
            let start = prefix * 360.0 * appear + pad / 2.0 - 90.0;
            let sweep = fraction * 360.0 * appear - pad;
            if sweep > 0.5 {
                fill_sector(
                    renderer,
                    centre,
                    hole,
                    outer,
                    start,
                    start + sweep,
                    tone_color(tokens, index),
                );
            }
            prefix += fraction;
        }

        if self.show_labels {
            let mut prefix = 0.0f32;
            for point in &self.points {
                let fraction = point.value.max(0.0) / total;
                let mid = (prefix + fraction / 2.0) * 360.0 - 90.0;
                let label = point.label.as_str();
                let width = estimate(label, LABEL_SIZE);
                let reach = self.plot_size * PIE_LABEL_REACH;
                let x = centre.x + reach * mid.to_radians().cos() - width / 2.0;
                let y = centre.y + reach * mid.to_radians().sin() - LABEL_LINE / 2.0;
                draw_text(
                    renderer,
                    label,
                    iced::Point::new(x, y),
                    Size::new(width + 2.0, LABEL_LINE),
                    LABEL_SIZE,
                    iced::Font {
                        weight: iced::font::Weight::Semibold,
                        ..iced::Font::DEFAULT
                    },
                    iced::alignment::Vertical::Top,
                    tokens.foreground,
                    bounds,
                );
                prefix += fraction;
            }
        }

        if fade > 0.0 {
            if let Some(index) = hover {
                if index < self.points.len() {
                    let point = &self.points[index];
                    let fraction = point.value.max(0.0) / total;
                    let prefix: f32 = self.points[..index]
                        .iter()
                        .map(|item| item.value.max(0.0) / total)
                        .sum();
                    let mid = (prefix + fraction / 2.0) * 360.0 - 90.0;
                    let reach = self.plot_size * PIE_MARK_REACH;
                    let mark = iced::Point::new(
                        centre.x + reach * mid.to_radians().cos(),
                        centre.y + reach * mid.to_radians().sin(),
                    );
                    fill(
                        renderer,
                        dot(mark.x, mark.y, 5.0),
                        tokens.background.scale_alpha(fade),
                        5.0,
                    );
                    fill(
                        renderer,
                        dot(mark.x, mark.y, 3.0),
                        tone_color(tokens, index).scale_alpha(fade),
                        3.0,
                    );

                    draw_tooltip(
                        renderer,
                        tokens,
                        bounds,
                        position.unwrap_or(mark),
                        &point.label,
                        &[format_value(point.value)],
                        fade,
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------- radar plot

/// The radar the builder reduces to.
struct Radar {
    points: Vec<RadarPoint>,
    multiple: bool,
    dots: bool,
    lines_only: bool,
    grid_levels: usize,
    tone: usize,
    tone_b: usize,
    max_value: f32,
    plot_size: f32,
    series_a: String,
    series_b: String,
}

impl Radar {
    /// The spokes the polygon is drawn for, Slint's `n`.
    fn spokes(&self) -> usize {
        self.points.len().clamp(3, 8)
    }

    /// The centre and radius of the ring, in pixels.
    fn geometry(&self, bounds: Rectangle) -> (iced::Point, f32) {
        (
            iced::Point::new(
                bounds.x + bounds.width / 2.0,
                bounds.y + self.plot_size / 2.0,
            ),
            self.plot_size * RADAR_RADIUS,
        )
    }

    /// The pixel position of spoke `index` at a share `t` of the radius.
    fn vertex(&self, centre: iced::Point, radius: f32, index: usize, t: f32) -> iced::Point {
        let n = self.spokes();
        let angle = (-90.0 + 360.0 * index as f32 / n as f32).to_radians();
        iced::Point::new(
            centre.x + radius * t * angle.cos(),
            centre.y + radius * t * angle.sin(),
        )
    }

    fn height(&self) -> f32 {
        self.plot_size + 8.0
    }

    /// The spoke nearest the pointer's angle, Slint's `radar-at`: any position
    /// in the box answers, the ring is not hit-tested.
    fn hovered(&self, bounds: Rectangle, position: iced::Point) -> Option<usize> {
        if self.points.len() < 3 {
            return None;
        }
        let (centre, _) = self.geometry(bounds);
        let turn = (position.y - centre.y)
            .atan2(position.x - centre.x)
            .to_degrees()
            + 90.0;
        let turn = if turn < 0.0 { turn + 360.0 } else { turn };
        let n = self.spokes();
        let index = (turn / 360.0 * n as f32).round() as i64;
        Some(index.rem_euclid(n as i64) as usize)
    }

    #[allow(clippy::too_many_arguments)]
    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        tokens: &Tokens,
        bounds: Rectangle,
        hover: Option<usize>,
        position: Option<iced::Point>,
        appear: f32,
        fade: f32,
    ) {
        if self.points.len() < 3 {
            return;
        }
        let (centre, radius) = self.geometry(bounds);
        let n = self.spokes();
        let border = tokens.border;

        // The rings, outside in or inside out but always under the series.
        for level in 1..=self.grid_levels {
            let t = level as f32 / self.grid_levels as f32;
            for index in 0..n {
                segment(
                    renderer,
                    self.vertex(centre, radius, index, t),
                    self.vertex(centre, radius, (index + 1) % n, t),
                    border,
                );
            }
        }

        // The first series, growing out of the centre on its own appear.
        let alpha: Vec<iced::Point> = (0..n)
            .map(|index| {
                let t = self.points[index].alpha / self.max_value * appear;
                self.vertex(centre, radius, index, t)
            })
            .collect();
        let tone = tone_color(tokens, self.tone);
        if !self.lines_only {
            fill_polygon(renderer, &alpha, tone.scale_alpha(RADAR_ALPHA));
        }
        for index in 0..n {
            segment(renderer, alpha[index], alpha[(index + 1) % n], tone);
        }

        // The second series, when there is one.
        if self.multiple {
            let beta: Vec<iced::Point> = (0..n)
                .map(|index| {
                    let t = self.points[index].beta / self.max_value * appear;
                    self.vertex(centre, radius, index, t)
                })
                .collect();
            let tone_b = tone_color(tokens, self.tone_b);
            fill_polygon(renderer, &beta, tone_b.scale_alpha(RADAR_ALPHA_B));
            for index in 0..n {
                segment(renderer, beta[index], beta[(index + 1) % n], tone_b);
            }
        }

        // Slint draws dots on the first series only.
        if self.dots {
            for vertex in &alpha {
                fill(renderer, dot(vertex.x, vertex.y, 3.0), tone, 3.0);
            }
        }

        // The dimension labels, pushed radially past the outer ring.
        for (index, point) in self.points.iter().enumerate() {
            let at = self.vertex(centre, radius, index, RADAR_LABEL_REACH);
            let width = estimate(&point.label, LABEL_SIZE);
            draw_text(
                renderer,
                &point.label,
                iced::Point::new(at.x - width / 2.0, at.y - LABEL_LINE / 2.0),
                Size::new(width + 2.0, LABEL_LINE),
                LABEL_SIZE,
                iced::Font::DEFAULT,
                iced::alignment::Vertical::Top,
                tokens.muted_foreground,
                bounds,
            );
        }

        if fade > 0.0 {
            if let Some(index) = hover {
                if index < n {
                    // A dot on the first series at the hovered spoke, then the
                    // tooltip.
                    let vertex = alpha[index];
                    fill(
                        renderer,
                        dot(vertex.x, vertex.y, 5.0),
                        tokens.background.scale_alpha(fade),
                        5.0,
                    );
                    fill(
                        renderer,
                        dot(vertex.x, vertex.y, 3.0),
                        tone.scale_alpha(fade),
                        3.0,
                    );

                    let point = &self.points[index];
                    let lines = if self.multiple {
                        vec![
                            format!("{}  {}", self.series_a, format_value(point.alpha)),
                            format!("{}  {}", self.series_b, format_value(point.beta)),
                        ]
                    } else {
                        vec![format_value(point.alpha)]
                    };
                    draw_tooltip(
                        renderer,
                        tokens,
                        bounds,
                        position.unwrap_or(vertex),
                        &point.label,
                        &lines,
                        fade,
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------- candles

/// The candlestick plot the builder reduces to.
struct Candles {
    candles: Vec<Candle>,
    body_width: f32,
    tick_margin: usize,
    plot_height: f32,
    min_price: Option<f32>,
    max_price: Option<f32>,
    y_ticks: usize,
    y_axis: bool,
    grid: bool,
    bullish: Option<Color>,
    bearish: Option<Color>,
}

impl Candles {
    /// The price range the candles span: whatever the caller pinned, otherwise
    /// the data's own lows and highs.
    fn scale(&self) -> (f32, f32) {
        let mut lowest = f32::INFINITY;
        let mut highest = f32::NEG_INFINITY;
        for candle in &self.candles {
            lowest = lowest.min(candle.low);
            highest = highest.max(candle.high);
        }
        if !lowest.is_finite() || !highest.is_finite() {
            return (0.0, 1.0);
        }
        let bottom = self.min_price.unwrap_or(lowest);
        let top = self.max_price.unwrap_or(highest);
        if (top - bottom).abs() < f32::EPSILON {
            (bottom, bottom + 1.0)
        } else {
            (bottom.min(top), bottom.max(top))
        }
    }

    fn ticks(&self, bottom: f32, top: f32) -> Vec<f32> {
        tick_values(bottom, top, self.y_ticks)
    }

    fn geometry(&self, bounds: Rectangle) -> (Geometry, f32, f32) {
        let (bottom, top) = self.scale();
        let ticks = self.ticks(bottom, top);
        let gutter = gutter_width(self.y_axis, &ticks);
        (Geometry::new(bounds, gutter, self.plot_height), bottom, top)
    }

    fn height(&self) -> f32 {
        TOP_PAD + self.plot_height + X_LABEL_HEIGHT
    }

    fn hovered(&self, geometry: Geometry, position: iced::Point) -> Option<usize> {
        let plot = geometry.plot;
        let count = self.candles.len();
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
        Some(((fraction * count as f32).floor() as usize).min(count - 1))
    }

    fn colour(&self, tokens: &Tokens, candle: &Candle) -> Color {
        if candle.rose() {
            self.bullish.unwrap_or(tokens.success)
        } else {
            self.bearish.unwrap_or(tokens.danger)
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        tokens: &Tokens,
        bounds: Rectangle,
        hover: Option<usize>,
        position: Option<iced::Point>,
        appear: f32,
        fade: f32,
    ) {
        let (geometry, bottom, top) = self.geometry(bounds);
        let plot = geometry.plot;
        let count = self.candles.len();
        let grid_color = tokens.border.scale_alpha(GRID_ALPHA);
        let ticks = self.ticks(bottom, top);

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
        for index in 0..count {
            if index % self.tick_margin != 0 {
                continue;
            }
            let label = self.candles[index].label.as_str();
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

        let candles = |renderer: &mut iced::Renderer| {
            let band = geometry.band_width(count);
            for (index, candle) in self.candles.iter().enumerate() {
                let centre = geometry.band(index, count);
                let colour = self.colour(tokens, candle);
                let high = geometry.y(candle.high, bottom, top);
                let low = geometry.y(candle.low, bottom, top);
                fill(
                    renderer,
                    Rectangle {
                        x: centre - 0.5,
                        y: high,
                        width: 1.0,
                        height: (low - high).max(1.0),
                    },
                    colour,
                    0.0,
                );

                let top_price = candle.open.max(candle.close);
                let bottom_price = candle.open.min(candle.close);
                let body_top = geometry.y(top_price, bottom, top);
                let body_bottom = geometry.y(bottom_price, bottom, top);
                let width = (band * self.body_width).max(2.0);
                fill(
                    renderer,
                    Rectangle {
                        x: centre - width / 2.0,
                        y: body_top,
                        width,
                        height: (body_bottom - body_top).max(2.0),
                    },
                    colour,
                    1.0,
                );
            }
        };
        if appear >= 1.0 {
            candles(renderer);
        } else if appear > 0.0 {
            renderer.with_layer(
                Rectangle {
                    x: plot.x,
                    y: bounds.y,
                    width: plot.width * appear,
                    height: bounds.height,
                },
                candles,
            );
        }

        if fade > 0.0 {
            if let Some(index) = hover {
                if index < count {
                    let candle = &self.candles[index];
                    let centre = geometry.band(index, count);
                    let colour = self.colour(tokens, candle);
                    fill(
                        renderer,
                        Rectangle {
                            x: centre,
                            y: plot.y,
                            width: 1.0,
                            height: plot.height,
                        },
                        tokens.foreground.scale_alpha(GUIDE_ALPHA * fade),
                        0.0,
                    );
                    let close_y = geometry.y(candle.close, bottom, top);
                    fill(
                        renderer,
                        dot(centre, close_y, 5.0),
                        tokens.background.scale_alpha(fade),
                        5.0,
                    );
                    fill(
                        renderer,
                        dot(centre, close_y, 3.0),
                        colour.scale_alpha(fade),
                        3.0,
                    );

                    let lines = vec![
                        format!("Open  {}", format_value(candle.open)),
                        format!("High  {}", format_value(candle.high)),
                        format!("Low  {}", format_value(candle.low)),
                        format!("Close  {}", format_value(candle.close)),
                    ];
                    draw_tooltip(
                        renderer,
                        tokens,
                        bounds,
                        position.unwrap_or(iced::Point::new(centre, close_y)),
                        &candle.label,
                        &lines,
                        fade,
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------- chart widget

/// What a chart is: a cartesian plot, a pie, a radar or a candlestick. They
/// share one widget so the appear, the hover fade and the tree state are
/// written once.
enum Kind {
    Plot(Plot),
    Pie(Pie),
    Radar(Radar),
    Candles(Candles),
}

impl Kind {
    fn height(&self) -> f32 {
        match self {
            Kind::Plot(plot) => plot.height(),
            Kind::Pie(pie) => pie.height(),
            Kind::Radar(radar) => radar.height(),
            Kind::Candles(candles) => candles.height(),
        }
    }

    fn hovered(&self, bounds: Rectangle, position: iced::Point) -> Option<usize> {
        match self {
            Kind::Plot(plot) => plot.hovered(plot.geometry(bounds), position),
            Kind::Pie(pie) => pie.hovered(bounds, position),
            Kind::Radar(radar) => radar.hovered(bounds, position),
            Kind::Candles(candles) => {
                let (geometry, _, _) = candles.geometry(bounds);
                candles.hovered(geometry, position)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        tokens: &Tokens,
        bounds: Rectangle,
        hover: Option<usize>,
        position: Option<iced::Point>,
        appear: f32,
        fade: f32,
    ) {
        match self {
            Kind::Plot(plot) => plot.draw(renderer, tokens, bounds, hover, position, appear, fade),
            Kind::Pie(pie) => pie.draw(renderer, tokens, bounds, hover, position, appear, fade),
            Kind::Radar(radar) => {
                radar.draw(renderer, tokens, bounds, hover, position, appear, fade)
            }
            Kind::Candles(candles) => {
                candles.draw(renderer, tokens, bounds, hover, position, appear, fade)
            }
        }
    }
}

/// The chart widget: a kind and the frame state the tree holds.
struct Chart {
    kind: Kind,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Chart {
    fn tag(&self) -> Tag {
        Tag::of::<Frame>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Frame::new())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(self.kind.height()))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, Length::Fixed(self.kind.height()))
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
        // The redraw request carries the frame's instant, so the appear and the
        // fade are sampled at it rather than at whatever `Instant::now()` says
        // this pass — two passes over one frame then agree.
        let frame = match event {
            Event::Window(iced::window::Event::RedrawRequested(at)) => Some(*at),
            _ => None,
        };
        let now = frame.unwrap_or_else(Instant::now);
        let state = tree.state.downcast_mut::<Frame>();

        // The appear starts on the first frame the plot is asked to paint, the
        // closest thing iced gives to Slint's "enters the window".
        if let Some(at) = frame {
            if !state.started {
                state.started = true;
                state.appear.retarget(1.0, at);
            }
        }

        let mut redraw = false;
        if let Event::Mouse(mouse::Event::CursorMoved { .. }) = event {
            let next = cursor
                .position()
                .and_then(|p| self.kind.hovered(layout.bounds(), p));
            if let Some(position) = cursor.position() {
                state.position = Some(position);
            }
            if next != state.hover {
                state.hover = next;
                state.fade.retarget(
                    if next.is_some() && state.appear.value >= 1.0 {
                        1.0
                    } else {
                        0.0
                    },
                    now,
                );
                redraw = true;
            }
        } else if let Event::Mouse(mouse::Event::CursorLeft) = event {
            if state.hover.is_some() {
                state.hover = None;
                state.fade.retarget(0.0, now);
                redraw = true;
            }
        }

        let appear_moving = state.appear.sample(now, APPEAR_DURATION, |t| {
            cubic_bezier(0.165, 0.84, 0.44, 1.0, t)
        });

        // The tooltip waits until the data is whole, the way GPUI holds it
        // until the appear ends.
        let target = if state.hover.is_some() && state.appear.value >= 1.0 {
            1.0
        } else {
            0.0
        };
        state.fade.retarget(target, now);
        let fade_moving = if state.fade.to >= 0.5 {
            state
                .fade
                .sample(now, TOOLTIP_ENTER, |t| cubic_bezier(0.16, 1.0, 0.3, 1.0, t))
        } else {
            state
                .fade
                .sample(now, TOOLTIP_EXIT, |t| cubic_bezier(0.4, 0.0, 1.0, 1.0, t))
        };

        if redraw || appear_moving || fade_moving {
            // iced has no interval subscription under this runtime, so the frame
            // that just sampled asks for the next one — the route the
            // collapsible's reveal takes.
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        if bounds.width < 2.0 || bounds.height < 2.0 {
            return;
        }

        // Every token is read when iced draws, so a theme switch moves the
        // chart with the page.
        let tokens = Tokens::of(theme);
        let state = tree.state.downcast_ref::<Frame>();
        self.kind.draw(
            renderer,
            &tokens,
            bounds,
            state.hover,
            state.position,
            state.appear.value.clamp(0.0, 1.0),
            state.fade.value.clamp(0.0, 1.0),
        );
    }
}

// ---------------------------------------------------------------- motion

/// A value moving from where it is to a target, the route the collapsible's
/// `Motion` takes: the line is stored, not a tick, so any frame can ask where
/// it should be and an interruption reverses from the value on screen.
#[derive(Debug)]
struct Motion {
    from: f32,
    to: f32,
    started: Instant,
    value: f32,
}

impl Motion {
    /// A value sitting at rest.
    fn new(value: f32) -> Self {
        Self {
            from: value,
            to: value,
            started: Instant::now(),
            value,
        }
    }

    /// Points the line at `to`, from wherever the value is now.
    fn retarget(&mut self, to: f32, now: Instant) {
        if (self.to - to).abs() < f32::EPSILON {
            return;
        }
        self.from = self.value;
        self.to = to;
        self.started = now;
    }

    /// Samples the line at `now`, and answers whether there is more to come.
    fn sample(&mut self, now: Instant, duration: Duration, easing: impl Fn(f32) -> f32) -> bool {
        let elapsed = now.duration_since(self.started);
        if duration.is_zero() || elapsed >= duration {
            self.value = self.to;
            return false;
        }
        let t = elapsed.as_secs_f32() / duration.as_secs_f32();
        self.value = self.from + (self.to - self.from) * easing(t.clamp(0.0, 1.0));
        true
    }
}

/// What a chart remembers between frames, held in the widget tree instead of
/// the caller's state.
#[derive(Debug)]
struct Frame {
    /// The datum under the pointer, if any.
    hover: Option<usize>,
    /// Where the tooltip is anchored. Kept after the pointer leaves so the
    /// overlay can fade out where the cursor last was.
    position: Option<iced::Point>,
    /// How far the data has drawn in, 0..=1.
    appear: Motion,
    /// How far the hover overlay has faded in, 0..=1.
    fade: Motion,
    /// Whether the appear clock has been started by the first frame.
    started: bool,
}

impl Frame {
    fn new() -> Self {
        Self {
            hover: None,
            position: None,
            appear: Motion::new(0.0),
            fade: Motion::new(0.0),
            started: false,
        }
    }
}

/// The y at `t` on the cubic Bézier through `(0,0)`, `(x1,y1)`, `(x2,y2)` and
/// `(1,1)`, solved for x by bisection. Both Slint and GPUI spell their motion
/// as a CSS curve, so this is how the reference's easing is reproduced exactly.
fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32, x: f32) -> f32 {
    let x_at = |t: f32| {
        let u = 1.0 - t;
        3.0 * u * u * t * x1 + 3.0 * u * t * t * x2 + t * t * t
    };
    let y_at = |t: f32| {
        let u = 1.0 - t;
        3.0 * u * u * t * y1 + 3.0 * u * t * t * y2 + t * t * t
    };
    let (mut low, mut high) = (0.0f32, 1.0f32);
    for _ in 0..24 {
        let mid = (low + high) / 2.0;
        if x_at(mid) < x {
            low = mid;
        } else {
            high = mid;
        }
    }
    y_at((low + high) / 2.0)
}

// ---------------------------------------------------------------- geometry

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

/// The values a value axis marks, bottom first, both ends included.
fn tick_values(bottom: f32, top: f32, count: usize) -> Vec<f32> {
    let count = count.max(2);
    (0..count)
        .map(|index| bottom + (top - bottom) * index as f32 / (count - 1) as f32)
        .collect()
}

/// How wide a value-axis gutter has to be for its widest label.
fn gutter_width(y_axis: bool, ticks: &[f32]) -> f32 {
    if !y_axis {
        return 0.0;
    }
    let widest = ticks
        .iter()
        .map(|value| estimate(&format_value(*value), LABEL_SIZE))
        .fold(0.0f32, f32::max);
    widest + theme::SPACING_SM
}

// ---------------------------------------------------------------- drawing

/// A filled quad with a corner radius.
fn fill(renderer: &mut iced::Renderer, bounds: Rectangle, color: Color, radius: f32) {
    if color.a <= 0.0 || bounds.width <= 0.0 || bounds.height <= 0.0 {
        return;
    }
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

/// The area under a line, as one vertical quad per pixel column. The value is
/// interpolated the same way the stroke is, so the fill and the line agree.
fn fill_area(
    renderer: &mut iced::Renderer,
    points: &[Point],
    interpolation: Interpolation,
    color: Color,
    geometry: &Geometry,
    bottom: f32,
    top: f32,
) {
    let count = points.len();
    if count == 0 || geometry.plot.width <= 0.0 {
        return;
    }
    let plot = geometry.plot;
    let baseline = plot.y + plot.height;
    let columns = plot.width.ceil() as usize;
    for column in 0..columns {
        let fraction = (column as f32 + 0.5) / plot.width;
        let value = value_at(points, interpolation, fraction);
        let y = geometry.y(value, bottom, top);
        let height = (baseline - y).max(0.0);
        if height <= 0.0 {
            continue;
        }
        fill(
            renderer,
            Rectangle {
                x: plot.x + column as f32,
                y,
                width: 1.5,
                height: height + 0.5,
            },
            color,
            0.0,
        );
    }
}

/// The value a line has at `fraction` across the plot, the same interpolation
/// the stroke uses.
fn value_at(points: &[Point], interpolation: Interpolation, fraction: f32) -> f32 {
    let count = points.len();
    if count == 0 {
        return 0.0;
    }
    if count == 1 {
        return points[0].value;
    }
    let t = fraction.clamp(0.0, 1.0) * (count - 1) as f32;
    let index = (t.floor() as usize).min(count - 2);
    let local = t - index as f32;
    match interpolation {
        Interpolation::Linear => {
            points[index].value + (points[index + 1].value - points[index].value) * local
        }
        Interpolation::Step => {
            if local >= 1.0 {
                points[index + 1].value
            } else {
                points[index].value
            }
        }
    }
}

/// A filled pie or donut slice, as one horizontal quad row per pixel.
///
/// A sector has no quad primitive — the renderer has no rotation — so each row
/// intersects the slice's angular interval with the row's half-circle and fills
/// the x range between them, then subtracts the donut hole.
fn fill_sector(
    renderer: &mut iced::Renderer,
    centre: iced::Point,
    inner: f32,
    outer: f32,
    start: f32,
    end: f32,
    color: Color,
) {
    let sweep = end - start;
    if outer <= 0.0 || sweep <= 0.0 {
        return;
    }
    let rows = (outer * 2.0).ceil() as usize;
    for row in 0..rows {
        let y = centre.y - outer + row as f32 + 0.5;
        let dy = y - centre.y;
        if dy.abs() > outer {
            continue;
        }
        let half_out = (outer * outer - dy * dy).max(0.0).sqrt();
        let half_in = if inner > 0.0 && dy.abs() < inner {
            (inner * inner - dy * dy).max(0.0).sqrt()
        } else {
            0.0
        };

        let mut bounds: Vec<(f32, f32)> = Vec::new();
        if sweep >= 359.9 {
            bounds.push((centre.x - half_out, centre.x + half_out));
        } else {
            // The interval may cross the ±180° branch cut; a shift by a full
            // turn catches the copy that meets this row's half-circle.
            let half = if dy > 0.0 {
                (0.0, 180.0)
            } else {
                (-180.0, 0.0)
            };
            for shift in [-360.0f32, 0.0, 360.0] {
                let low = (start + shift).max(half.0);
                let high = (end + shift).min(half.1);
                if low >= high {
                    continue;
                }
                let (ax, bx) = (angle_x(centre.x, dy, low), angle_x(centre.x, dy, high));
                bounds.push((
                    ax.min(bx).max(centre.x - half_out),
                    ax.max(bx).min(centre.x + half_out),
                ));
            }
        }

        for (low, high) in bounds {
            // Subtract the hole's chord, which leaves at most two runs.
            let runs: [(f32, f32); 2] = if half_in <= 0.0 {
                [(low, high), (0.0, 0.0)]
            } else {
                let hole_low = centre.x - half_in;
                let hole_high = centre.x + half_in;
                [(low, hole_low.min(high)), (hole_high.max(low), high)]
            };
            for (from, to) in runs {
                if to - from < 0.5 {
                    continue;
                }
                fill(
                    renderer,
                    Rectangle {
                        x: from,
                        y: y - 0.1,
                        width: to - from,
                        height: 1.2,
                    },
                    color,
                    0.0,
                );
            }
        }
    }
}

/// The x where a ray at math angle `phi` (degrees) meets the horizontal line at
/// `dy` below the centre. The result is ±∞ on the axis, which the caller clamps
/// to the circle.
fn angle_x(cx: f32, dy: f32, phi: f32) -> f32 {
    if phi.abs() < 1e-4 {
        return f32::INFINITY;
    }
    if (phi.abs() - 180.0).abs() < 1e-4 {
        return f32::NEG_INFINITY;
    }
    let tan = phi.to_radians().tan();
    if tan.abs() < 1e-9 {
        return cx;
    }
    cx + dy / tan
}

/// A filled polygon, as one horizontal quad run per scan line — the even-odd
/// rule, so a concave radar series still fills correctly.
fn fill_polygon(renderer: &mut iced::Renderer, vertices: &[iced::Point], color: Color) {
    if vertices.len() < 3 {
        return;
    }
    let min_y = vertices
        .iter()
        .map(|point| point.y)
        .fold(f32::INFINITY, f32::min)
        .floor();
    let max_y = vertices
        .iter()
        .map(|point| point.y)
        .fold(f32::NEG_INFINITY, f32::max)
        .ceil();
    let rows = (max_y - min_y).max(0.0) as usize;
    let mut crossings: Vec<f32> = Vec::with_capacity(vertices.len());
    for row in 0..rows {
        let y = min_y + row as f32 + 0.5;
        crossings.clear();
        for index in 0..vertices.len() {
            let a = vertices[index];
            let b = vertices[(index + 1) % vertices.len()];
            if (a.y <= y && b.y > y) || (b.y <= y && a.y > y) {
                let t = (y - a.y) / (b.y - a.y);
                crossings.push(a.x + (b.x - a.x) * t);
            }
        }
        crossings.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        for pair in crossings.chunks_exact(2) {
            let width = pair[1] - pair[0];
            if width < 0.5 {
                continue;
            }
            fill(
                renderer,
                Rectangle {
                    x: pair[0],
                    y: y - 0.1,
                    width,
                    height: 1.2,
                },
                color,
                0.0,
            );
        }
    }
}

/// The tooltip: the popover surface beside the pointer, flipping toward the
/// centre of the plot near an edge, with the datum's name and its detail rows.
#[allow(clippy::too_many_arguments)]
fn draw_tooltip(
    renderer: &mut iced::Renderer,
    tokens: &Tokens,
    bounds: Rectangle,
    cursor: iced::Point,
    title: &str,
    lines: &[String],
    alpha: f32,
) {
    // iced 0.14 exposes no text measurement to a custom widget, so the box is
    // sized from the character count. It is honest about being an estimate,
    // which is why the padding is generous on every side.
    let mut widest = estimate(title, LABEL_SIZE);
    for line in lines {
        widest = widest.max(estimate(line, LABEL_SIZE));
    }
    let width = widest + theme::SPACING_MD * 2.0;
    let height = theme::SPACING_XS * 2.0 + LABEL_LINE * (1 + lines.len()) as f32;

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
        tokens.popover.scale_alpha(alpha),
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
                .color(tokens.border.scale_alpha(alpha))
                .width(1.0),
            shadow: iced::Shadow {
                color: tokens.shadow.scale_alpha(alpha),
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
        tokens.foreground.scale_alpha(alpha),
        bounds,
    );
    for (index, line) in lines.iter().enumerate() {
        draw_text(
            renderer,
            line,
            iced::Point::new(
                text_x,
                y + theme::SPACING_XS + LABEL_LINE * (index + 1) as f32,
            ),
            Size::new(text_width, LABEL_LINE),
            LABEL_SIZE,
            iced::Font::DEFAULT,
            iced::alignment::Vertical::Top,
            tokens.muted_foreground.scale_alpha(alpha),
            bounds,
        );
    }
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
    if color.a <= 0.0 {
        return;
    }
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
