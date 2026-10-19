//! `chart` — ReUI-style chart cards with animated Canvas draws and a data
//! popover on tap/click.
import { Text, View } from "nsh";
import { Chart } from "../components/chart";
import { alpha, tokens } from "../components/theme";
import { Caption, ExampleSection, Page, Row } from "./ui";

const months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
const shortMonths = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
const revenue = [32, 58, 42, 68, 54, 82, 60, 94, 70, 88, 64, 44];
const marketA = [40, 72, 56, 44, 68, 80];
const marketB = [30, 62, 48, 32, 58, 66];
const sales = [28, 54, 38, 64, 48, 78, 56, 92, 66, 84, 60, 40];

export function Demo() {
  const dot = (color: string) => (
    <View style={{ width: 7, height: 7, borderRadius: 4, backgroundColor: color }} />
  );

  const legend = (items: { label: string; color: string }[]) => (
    <Row gap={10}>
      {items.map((item) => (
        <Row gap={4}>
          {dot(item.color)}
          <Text style={{ fontSize: 10, color: tokens.mutedForeground }}>{item.label}</Text>
        </Row>
      ))}
    </Row>
  );

  // A monochrome series palette keeps the cards in the ReUI grayscale.
  const mono = [
    tokens.foreground,
    alpha(tokens.foreground, 0.72),
    alpha(tokens.foreground, 0.5),
    alpha(tokens.foreground, 0.3),
  ];
  // section: Usage
  const usage = (
    <Chart
      colors={mono}
      type="bar"
      card
      title="Revenue Growth"
      description="Monthly revenue performance tracking"
      delta="+5.2%"
      data={revenue}
      labels={months}
      height={240}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Line
  const line = (
    <Chart
      colors={mono}
      type="line"
      card
      title="Website Traffic"
      description="Monthly unique visitor trends"
      delta="+24.5%"
      data={[28, 34, 30, 46, 42, 58]}
      labels={shortMonths}
      height={240}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Bar
  const bar = (
    <Chart
      colors={mono}
      type="bar"
      card
      title="Product Sales"
      description="Annual sales trend visualization"
      delta="+4.3%"
      data={sales}
      labels={months}
      height={240}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Multi-dataset bar
  const multi = (
    <Chart
      colors={mono}
      type="bar"
      card
      title="Market Share"
      description="Departmental performance comparison"
      delta="+12%"
      series={[marketA, marketB]}
      labels={shortMonths}
      height={240}
      footer={legend([
        { label: "This year", color: mono[0] },
        { label: "Last year", color: mono[1] },
      ])}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Diagonal stripe bar
  const diagonal = (
    <Chart
      colors={mono}
      type="bar"
      card
      barStyle="diagonal"
      title="Product Sales"
      description="Annual sales trend visualization"
      delta="+4.3%"
      data={sales}
      labels={months}
      height={240}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Dotted pattern bar
  const dotted = (
    <Chart
      colors={mono}
      type="bar"
      card
      barStyle="dots"
      title="Website Traffic"
      description="Monthly visitor behavior patterns"
      delta="+8%"
      data={revenue}
      labels={months}
      height={240}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Gradient bar
  const gradient = (
    <Chart
      colors={mono}
      type="bar"
      card
      barStyle="gradient"
      title="Engagement Metrics"
      description="User interaction and click-through rates"
      delta="+2.4%"
      data={[22, 48, 34, 56, 40, 68, 52, 76, 58, 72, 50, 36]}
      labels={months}
      height={240}
      defaultActiveIndex={6}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Horizontal bars
  const horizontal = (
    <Chart
      colors={mono}
      type="bar"
      card
      barStyle="horizontal"
      title="Inventory Levels"
      description="Stock availability across warehouses"
      delta="-3.2%"
      deltaTone="danger"
      data={[70, 92, 58, 76, 44, 82]}
      labels={shortMonths}
      height={220}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Area
  const area = (
    <Chart
      colors={mono}
      type="area"
      card
      title="Website Traffic"
      description="Monthly unique visitor trends"
      delta="+24.5%"
      data={[28, 34, 30, 46, 42, 58]}
      labels={shortMonths}
      height={240}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Stacked area
  const stacked = (
    <Chart
      colors={mono}
      type="area"
      areaStyle="stacked"
      card
      title="Traffic Sources"
      description="Visitor acquisition by channel"
      delta="+18.3%"
      series={[
        [20, 28, 24, 38, 34, 46],
        [14, 20, 18, 28, 26, 36],
        [10, 14, 12, 20, 18, 26],
      ]}
      labels={shortMonths}
      height={240}
      footer={legend([
        { label: "Direct", color: mono[0] },
        { label: "Search", color: mono[1] },
        { label: "Social", color: mono[2] },
      ])}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Step area
  const step = (
    <Chart
      colors={mono}
      type="area"
      areaStyle="step"
      card
      title="Active Connections"
      description="Server connection pool over time"
      delta="+257%"
      data={[18, 30, 24, 42, 38, 56, 52, 74]}
      labels={["Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep"]}
      height={240}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Donut
  const donut = (
    <Chart
      colors={mono}
      type="pie"
      donut
      card
      title="Traffic Sources"
      description="Where your visitors come from"
      data={[38, 28, 20, 14]}
      labels={["Direct", "Search", "Social", "Referral"]}
      centerValue="13.9k"
      centerLabel="Total Visits"
      height={240}
      footer={legend([
        { label: "Direct", color: mono[0] },
        { label: "Search", color: mono[1] },
        { label: "Social", color: mono[2] },
        { label: "Referral", color: tokens.chart4 },
      ])}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Radar
  const radar = (
    <Chart
      colors={mono}
      type="radar"
      card
      title="Skill Assessment"
      description="Team proficiency across key areas"
      data={[4.2, 3.2, 4.6, 4.0, 3.6, 3.0]}
      labels={["Frontend", "Backend", "Design", "Testing", "DevOps", "Security"]}
      height={260}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Candlestick
  const candle = (
    <Chart
      colors={mono}
      type="candlestick"
      card
      title="Request Volume"
      description="API and webhook traffic over 12 months"
      delta="+12.8%"
      data={[12, 18, 15, 24, 20, 30, 26, 38, 32, 44, 40, 52]}
      labels={months}
      height={240}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Sankey
  const sankey = (
    <Chart
      colors={mono}
      type="sankey"
      card
      title="Acquisition Flow"
      description="Where the first visit comes from"
      data={[42, 28, 18, 12]}
      labels={["Search", "Direct", "Social", "Email"]}
      height={220}
      style={{ maxWidth: 380 }}
    />
  );

  // section: Chart with popover
  const popover = (
    <Chart
      colors={mono}
      type="bar"
      card
      title="Engagement Metrics"
      description="Tap a column to inspect the data"
      delta="+2.4%"
      data={[22, 48, 34, 56, 40, 68, 52, 76, 58, 72, 50, 36]}
      labels={months}
      height={240}
      defaultActiveIndex={5}
      style={{ maxWidth: 380 }}
    />
  );
  // section: end

  return (
    <Page>
      <Caption>
        Charts animate from the baseline when they first appear. Tap or click a point to open
        its data popover; the cards keep the ReUI header, delta and legend.
      </Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Line">{line}</ExampleSection>
      <ExampleSection index={2} title="Bar">{bar}</ExampleSection>
      <ExampleSection index={3} title="Multi-dataset bar">{multi}</ExampleSection>
      <ExampleSection index={4} title="Diagonal stripe bar">{diagonal}</ExampleSection>
      <ExampleSection index={5} title="Dotted pattern bar">{dotted}</ExampleSection>
      <ExampleSection index={6} title="Gradient bar">{gradient}</ExampleSection>
      <ExampleSection index={7} title="Horizontal bars">{horizontal}</ExampleSection>
      <ExampleSection index={8} title="Area">{area}</ExampleSection>
      <ExampleSection index={9} title="Stacked area">{stacked}</ExampleSection>
      <ExampleSection index={10} title="Step area">{step}</ExampleSection>
      <ExampleSection index={11} title="Donut">{donut}</ExampleSection>
      <ExampleSection index={12} title="Radar">{radar}</ExampleSection>
      <ExampleSection index={13} title="Candlestick">{candle}</ExampleSection>
      <ExampleSection index={14} title="Sankey">{sankey}</ExampleSection>
      <ExampleSection index={15} title="Chart with popover">{popover}</ExampleSection>
    </Page>
  );
}
