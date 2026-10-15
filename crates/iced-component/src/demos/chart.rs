//! `chart` — the plot surface, shown the way the Slint example shows it: a
//! line, a bar chart, an area, a pie, a radar, a candlestick, the shared axes,
//! and the hover tooltip with its fade.
//!
//! Run it with `cargo run --example chart`, or in the browser as
//! `?story=chart` in the gallery the website embeds.
//!
//! The section titles below are the page's own headings — `LineChart`,
//! `Axes and Guides`, `BarChart`, `AreaChart`, `PieChart`, `RadarChart`,
//! `CandlestickChart` and `Hover and Tooltips` — which are the headings
//! `website/component/chart.md` documents these samples under. The plugin's own
//! `icedSections` reads these markers, so the code on the page is the code that
//! runs here, and each panel lands under the heading that names it.
//!
//! The Slint example wraps forty-odd cards in `ChartCard`s; this is the surface
//! the component behind them actually owns — six kinds — and the module docs in
//! [`crate::components::chart`] name what was not ported.

use crate::components::chart::{
    AreaChart, BarChart, Candle, CandlestickChart, ChartLegend, Interpolation, LineChart, PieChart,
    Point, RadarChart, RadarPoint,
};
use crate::components::group_box::{self, GroupBox};
use crate::components::theme::{self, Tokens};
use iced::widget::{column, container, mouse_area, row, scrollable, svg, text};
use iced::{Alignment as Axis, Element, Length, Task, Theme};

/// The page padding and the gap between sections, which the Slint example sets
/// to 24px.
const PAGE: f32 = theme::SPACING_XL;

pub fn run(theme: Theme) -> iced::Result {
    iced::application(boot, update, view)
        .title("uni-kit — chart")
        .theme(theme)
        .run()
}

/// The section heading the pointer is over, and the one that was pressed, so
/// the code icon draws the two states the Slint heading animates to.
#[derive(Debug, Default)]
struct App {
    hovered: Option<&'static str>,
    pressed_source: Option<&'static str>,
}

#[derive(Debug, Clone)]
enum Message {
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
        Message::HoverSource(title) => app.hovered = title,
        Message::PressSource(title) => app.pressed_source = title,
        Message::ShowSource { index, title } => {
            app.pressed_source = None;
            crate::bridge::show(index, title);
        }
    }
    Task::none()
}

/// A muted line, for the note under the sections.
fn owned_muted<'a>(value: String, size: f32) -> Element<'a, Message> {
    text(value)
        .size(size)
        .style(|theme: &Theme| iced::widget::text::Style {
            color: Some(Tokens::of(theme).muted_foreground),
        })
        .into()
}

/// A section heading: the title, and the code icon that asks the page for this
/// sample. Either one is the button, as in Slint's `SourceHeading`.
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
            let (fill, edge) = if pressed {
                (0.34, 0.7)
            } else if hovered {
                (0.22, 0.55)
            } else {
                (0.12, 0.35)
            };
            container::Style {
                background: Some(primary.scale_alpha(fill).into()),
                border: iced::border::rounded(theme::RADIUS_LG)
                    .color(primary.scale_alpha(edge))
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
        .spacing(theme::SPACING_SM)
        .align_y(Axis::Center)
        .into()
}

/// The box under a heading: the `GroupBox` with the outline variant the Slint
/// example wraps its sections in, untitled because the heading carries the name.
fn body_box<'a>(body: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    GroupBox::new()
        .variant(group_box::Variant::Outline)
        .body(body)
        .into()
}

// The data the Slint example carries as properties, one function per series.
// `Point` is the labelled value the component draws.

fn months() -> Vec<Point> {
    const VALUES: [f32; 12] = [
        42.0, 51.0, 47.0, 63.0, 58.0, 71.0, 66.0, 74.0, 69.0, 81.0, 76.0, 88.0,
    ];
    const LABELS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    LABELS
        .iter()
        .zip(VALUES)
        .map(|(label, value)| Point::new(*label, value))
        .collect()
}

/// The step series: the same shape as a conversion rate, in percent.
fn conversion() -> Vec<Point> {
    const VALUES: [f32; 12] = [2.1, 2.4, 2.2, 2.8, 2.6, 3.1, 2.9, 3.3, 3.0, 3.5, 3.2, 3.6];
    [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ]
    .iter()
    .zip(VALUES)
    .map(|(label, value)| Point::new(*label, value))
    .collect()
}

/// Today's sessions, still arriving: six points against a pinned axis.
fn intraday() -> Vec<Point> {
    const VALUES: [f32; 6] = [102.0, 104.0, 101.0, 108.0, 112.0, 110.0];
    ["9:30", "10:30", "11:30", "12:30", "13:30", "14:30"]
        .iter()
        .zip(VALUES)
        .map(|(label, value)| Point::new(*label, value))
        .collect()
}

/// Share of traffic by region, which the mixed bar chart colours one tone each.
fn regions() -> Vec<Point> {
    [
        ("N. America", 48.0),
        ("Europe", 29.0),
        ("APAC", 16.0),
        ("LatAm", 7.0),
    ]
    .into_iter()
    .map(Point::from)
    .collect()
}

/// Monthly net cash: a bar chart that has to cross its zero line.
fn cash() -> Vec<Point> {
    const VALUES: [f32; 12] = [
        12.0, -4.0, 8.0, 15.0, -6.0, 11.0, 9.0, -2.0, 14.0, 7.0, -5.0, 18.0,
    ];
    [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ]
    .iter()
    .zip(VALUES)
    .map(|(label, value)| Point::new(*label, value))
    .collect()
}

/// Visitors by device: eight days of a trend that is still moving.
fn visitors() -> Vec<Point> {
    const VALUES: [f32; 8] = [42.0, 48.0, 51.0, 46.0, 55.0, 61.0, 58.0, 64.0];
    [
        "Apr 1", "Apr 8", "Apr 15", "Apr 22", "May 1", "May 15", "Jun 1", "Jun 15",
    ]
    .iter()
    .zip(VALUES)
    .map(|(label, value)| Point::new(*label, value))
    .collect()
}

/// Share of traffic by channel, which the pie colours one tone per slice.
fn traffic() -> Vec<Point> {
    [
        ("Direct", 38.0),
        ("Organic Search", 27.0),
        ("Social", 18.0),
        ("Referral", 11.0),
        ("Email", 6.0),
    ]
    .into_iter()
    .map(Point::from)
    .collect()
}

/// Six review dimensions, scored out of 100, the Slint example's `scores`.
fn scores() -> Vec<RadarPoint> {
    [
        ("Usability", 88.0, 72.0),
        ("Reliability", 74.0, 91.0),
        ("Performance", 81.0, 77.0),
        ("Support", 69.0, 86.0),
        ("Design", 92.0, 70.0),
        ("Value", 78.0, 80.0),
    ]
    .into_iter()
    .map(|(label, alpha, beta)| RadarPoint::new(label, alpha, beta))
    .collect()
}

/// Twelve sessions of open, high, low and close, the Slint example's `candles`.
fn candles() -> Vec<Candle> {
    [
        ("Mar 3", 102.0, 108.0, 100.0, 106.0),
        ("Mar 4", 106.0, 110.0, 103.0, 104.0),
        ("Mar 5", 104.0, 107.0, 98.0, 99.0),
        ("Mar 6", 99.0, 105.0, 97.0, 103.0),
        ("Mar 7", 103.0, 112.0, 102.0, 111.0),
        ("Mar 10", 111.0, 114.0, 107.0, 109.0),
        ("Mar 11", 109.0, 118.0, 108.0, 116.0),
        ("Mar 12", 116.0, 119.0, 111.0, 113.0),
        ("Mar 13", 113.0, 117.0, 109.0, 110.0),
        ("Mar 14", 110.0, 121.0, 108.0, 120.0),
        ("Mar 17", 120.0, 124.0, 115.0, 118.0),
        ("Mar 18", 118.0, 126.0, 117.0, 125.0),
    ]
    .into_iter()
    .map(|(label, open, high, low, close)| Candle::new(label, open, high, low, close))
    .collect()
}

fn view(app: &App) -> Element<'_, Message> {
    // section: LineChart
    // A line joins its points in the order given. `Linear` is the default and
    // `Step` holds each value until the next point, Slint's `step after`. The
    // tone is one of the theme's five chart colours, resolved when iced draws
    // so the series moves with the theme rather than being frozen at build time.
    let lines = column![
        LineChart::new(months()).dots(true).tone(1),
        LineChart::new(conversion())
            .interpolation(Interpolation::Step)
            .tone(2),
    ]
    .spacing(theme::SPACING_MD);

    // section: Axes and Guides
    // The value axis draws `y_ticks` labelled grid lines, both ends included,
    // in a gutter that widens to fit the widest label. The x axis labels every
    // `tick_margin`-th point, `max_value` pins the top for a series that never
    // nears zero, and `grid(false)` leaves the plot bare.
    let axes = row![
        LineChart::new(months()).tone(1).y_ticks(5),
        LineChart::new(intraday())
            .tone(2)
            .y_ticks(3)
            .max_value(130.0)
            .grid(false),
    ]
    .spacing(theme::SPACING_LG);

    // section: BarChart
    // Bars grow out of the zero line, so a negative value extends below it
    // rather than being clamped. `mixed` takes one tone per bar, `rounded`
    // rounds the corners, and `show_labels` writes each value over its bar. The
    // legend names the tones under the plot.
    let bars = column![
        BarChart::new(regions())
            .mixed(true)
            .rounded(true)
            .show_labels(true),
        ChartLegend::new().items(["N. America", "Europe", "APAC", "LatAm"]),
        BarChart::new(cash()).rounded(true).tone(1),
    ]
    .spacing(theme::SPACING_SM);

    // section: AreaChart
    // An area fills the region between its line and the baseline at 35% of the
    // series tone. It takes the same joins and the same pinned axis as a line,
    // so a series still in progress holds room for the points still to come.
    let areas = column![
        AreaChart::new(visitors()).tone(1),
        AreaChart::new(intraday())
            .interpolation(Interpolation::Step)
            .tone(3)
            .max_value(130.0),
    ]
    .spacing(theme::SPACING_MD);

    // section: PieChart
    // A pie sweeps clockwise from the top, one palette tone per slice. `donut`
    // leaves a hole, `pad_angle` opens a gap between slices, and `show_labels`
    // names each slice at the ring's midpoint.
    let pies = row![
        PieChart::new(traffic()).donut(true).pad_angle(0.02),
        PieChart::new(regions())
            .show_labels(true)
            .outer_radius(38.0),
    ]
    .spacing(theme::SPACING_LG);

    // section: RadarChart
    // A radar closes a polygon around one vertex per dimension. `multiple`
    // draws the second series, `dots` marks the first series' vertices, and
    // `lines_only` drops the fill so only the ring grid and the outline remain.
    let radars = row![
        RadarChart::new(scores()).multiple(true).dots(true),
        RadarChart::new(scores())
            .lines_only(true)
            .grid_levels(5)
            .tone(3),
    ]
    .spacing(theme::SPACING_LG);

    // section: CandlestickChart
    // Each candle is a wick from the session's low to its high and a body from
    // its open to its close, coloured by whether the close beat the open, over
    // a price range the caller can pin with `min_price` and `max_price`.
    let candle_chart = CandlestickChart::new(candles())
        .min_price(90.0)
        .max_price(130.0)
        .tick_margin(2);

    // section: Hover and Tooltips
    // Nothing opts in: a chart hit-tests the pointer, draws the guide and a mark
    // on the datum under it, and shows that datum's name and value in a tooltip
    // that flips toward the centre near an edge. The tooltip fades in and out,
    // and every series draws in over one second on the first paint. The state
    // lives in the widget tree, so the element itself stays stateless.
    let hover = column![
        LineChart::new(visitors()).dots(true).tone(0),
        BarChart::new(regions()).mixed(true).rounded(true),
    ]
    .spacing(theme::SPACING_MD);

    // section: end
    let gallery = column![
        heading(app, 0, "LineChart"),
        body_box(lines),
        heading(app, 1, "Axes and Guides"),
        body_box(axes),
        heading(app, 2, "BarChart"),
        body_box(bars),
        heading(app, 3, "AreaChart"),
        body_box(areas),
        heading(app, 4, "PieChart"),
        body_box(pies),
        heading(app, 5, "RadarChart"),
        body_box(radars),
        heading(app, 6, "CandlestickChart"),
        body_box(candle_chart),
        heading(app, 7, "Hover and Tooltips"),
        body_box(hover),
        owned_muted(
            "Hover a plot to see the guide and the tooltip.".to_string(),
            theme::FONT_SIZE_SM,
        ),
    ]
    .spacing(PAGE)
    .padding(PAGE)
    .width(Length::Fill);

    scrollable(gallery).height(Length::Fill).into()
}

// The Lucide mark the code icon uses, from the icon set this repository already
// ships (`crates/slint-component/ui/icons`, Lucide, ISC), so the three galleries
// draw the same glyph.
const CODE_XML: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/></svg>"#;
