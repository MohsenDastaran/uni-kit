//! `chart` — line, bar, area, pie, radar and candlestick charts on Canvas.
import { Chart } from "../components/chart";
import { tokens } from "../components/theme";
import { Caption, ExampleSection, Page } from "./ui";

const data = [12, 28, 18, 36, 24, 42, 30];
const labels = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

export function Demo() {
  // section: Line
  const line = <Chart type="line" data={data} labels={labels} height={240} />;
  // section: Bar
  const bar = <Chart type="bar" data={data} labels={labels} height={240} />;
  // section: Area
  const area = <Chart type="area" data={data} labels={labels} height={240} />;
  // section: Pie
  const pie = <Chart type="pie" data={[30, 22, 18, 15, 15]} height={240} />;
  // section: Radar
  const radar = <Chart type="radar" data={[3, 5, 4, 2, 5]} height={240} />;
  // section: Candlestick
  const candle = <Chart type="candlestick" data={[10, 14, 12, 18, 16, 22, 20]} labels={labels} height={240} />;
  // section: Sankey
  const sankey = <Chart type="sankey" data={[32, 24, 18, 14, 12]} labels={["Search", "Direct", "Social", "Email", "Referral"]} height={240} />;
  // section: end
  return (
    <Page>
      <Caption>Charts are Canvas drawings: the data and colours are the application's, the tooltip is not part of this port.</Caption>
      <ExampleSection index={0} title="Line">{line}</ExampleSection>
      <ExampleSection index={1} title="Bar">{bar}</ExampleSection>
      <ExampleSection index={2} title="Area">{area}</ExampleSection>
      <ExampleSection index={3} title="Pie">{pie}</ExampleSection>
      <ExampleSection index={4} title="Radar">{radar}</ExampleSection>
      <ExampleSection index={5} title="Candlestick">{candle}</ExampleSection>
      <ExampleSection index={6} title="Sankey">{sankey}</ExampleSection>
    </Page>
  );
}
